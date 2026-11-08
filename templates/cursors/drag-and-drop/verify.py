#!/usr/bin/env python3
"""Render the native scene; check curved carry, pointer grip, landing and list closure."""
import argparse
import json
import math
from pathlib import Path
import re
import subprocess
import uuid

from PIL import Image, ImageChops, ImageStat


def run(*args):
    result = subprocess.run([str(arg) for arg in args], capture_output=True, timeout=900)
    if result.returncode:
        raise RuntimeError(result.stderr.decode() + result.stdout.decode())
    return result.stdout


def carry(seconds):
    progress = max(0, min(1, (seconds - 0.9) / 1.1))
    eased = progress**3 * (10 + progress * (-15 + 6 * progress))
    return (606.88 * (1 - eased) * eased + 640 * eased**2,
            -291.6 * (1 - eased) * eased - 138 * eased**2, progress, eased)


def centroid(image, color):
    pixels = image.load()
    points = {(x, y) for y in range(140, 960) for x in range(290, 1630)
              if color(*pixels[x, y])}
    largest = []
    # Ignore compression-colored fringes around other avatars; track the connected chip.
    while points:
        component = [points.pop()]
        for x, y in component:
            for neighbor in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
                if neighbor in points:
                    points.remove(neighbor)
                    component.append(neighbor)
        if len(component) > len(largest):
            largest = component
    assert len(largest) > 700, "Task chip missing or clipped"
    return tuple((min(point[axis] for point in largest) + max(point[axis] for point in largest)) / 2
                 for axis in (0, 1))


def verify(output):
    output.mkdir(parents=True, exist_ok=False)
    workspace = output / "workspace"
    run("mont", "init", workspace, "--json")

    def dispatch(command):
        envelope = output / "command.json"
        envelope.write_text(json.dumps({"commandId": str(uuid.uuid4()), "command": command}))
        run("mont", "dispatch", "-C", workspace, envelope, "--json")

    for key, value in (("sceneWidth", 1920), ("sceneHeight", 1080)):
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    source = Path(__file__).with_name("scene.tsx").read_text()
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "drag-and-drop",
        "placement": {"startMs": 0, "durationMs": 3600, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "sourceTsx": source, "variables": {},
            "frame": {"x": 0, "y": 0, "width": 1920, "height": 1080}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    images = re.findall(r'"(data:[^"\r\n]+)"', source)
    dispatch({"type": "setProjectValue", "pointer": "/resources",
              "value": {src: {"uri": src} for src in images}})
    video = output / "drag-and-drop.mp4"
    (output / "render.json").write_bytes(run(
        "mont", "render", "-C", workspace, "--output", video, "--fps", 30, "--json"))
    metadata = json.loads(run("ffprobe", "-v", "error", "-show_streams", "-show_format", "-of", "json", video))
    stream = next(s for s in metadata["streams"] if s["codec_type"] == "video")
    assert (stream["width"], stream["height"]) == (1920, 1080), metadata
    assert abs(float(metadata["format"]["duration"]) - 3.6) < 0.04, metadata
    frames = {}
    for label, seconds in (("initial", 0), ("lift", 25 / 30), ("carry", 42 / 30),
                           ("approach", 52 / 30), ("landed", 2.4), ("hold", 3.1)):
        path = output / (label + ".png")
        run("ffmpeg", "-v", "error", "-ss", seconds, "-i", video,
            "-vf", "colorspace=all=bt709:trc=iec61966-2-1:range=pc,format=rgb24",
            "-frames:v", 1, path)
        frame = Image.open(path).convert("RGB")
        frames[label] = frame
        dx, dy, progress, eased = carry(seconds)
        lift = max(0, min(1, (seconds - 0.7) / 0.2))
        scale = 1 + 0.025 * lift**3 * (10 + lift * (-15 + 6 * lift)) if seconds < 2 else 1
        speed = 30 * progress**2 * (1 - progress)**2 / 1.1
        angle = math.radians(min(2, (606.88 + 66.24 * eased) * speed / 1200 * 2))
        expected = (-144 + 1.15 * (730 + dx + scale * (-264 * math.cos(angle) + 28 * math.sin(angle))),
                    -81.75 + 1.15 * (500 + dy + scale * (-264 * math.sin(angle) - 28 * math.cos(angle))))
        actual = centroid(frame, lambda r, g, b: r > 235 and 210 < g < 240 and b > g + 4 and r > b + 8)
        assert math.dist(actual, expected) < 9, (label, "Card detached from curved grip", actual, expected)
        if label in ("lift", "carry", "approach"):
            gx, gy = round(-144 + 1.15 * (730 + dx)), round(-81.75 + 1.15 * (500 + dy))
            body = frame.crop((gx + 5, gy + 8, gx + 17, gy + 24))
            assert sum(max(body.getpixel((x, y))) < 90 for y in range(body.height) for x in range(body.width)) > 55, (label, "Mont cursor detached from card")
    green = lambda r, g, b: g > 228 and 210 < r < 235 and 215 < b < 239 and g > r + 8 and g > b + 7
    initial = centroid(frames["initial"], green)
    landed = centroid(frames["landed"], green)
    assert abs(initial[0] - landed[0]) < 2 and abs(initial[1] - landed[1] - 158.7) < 3, "List did not close the vacated row"
    for box in ((466, 185, 511, 224), (1284, 185, 1329, 224)):
        difference = ImageChops.difference(frames["initial"].crop(box), frames["landed"].crop(box))
        assert sum(ImageStat.Stat(difference).mean) / 3 > 1, "Column count did not change"
    # Sample the rendered card rather than source formulas: avoid the old fast mid-drag lurch.
    run("ffmpeg", "-v", "error", "-i", video, "-vf",
        "select='between(t,0.9,2.0)',scale=960:540,colorspace=all=bt709:trc=iec61966-2-1:range=pc,format=rgb24",
        "-fps_mode", "vfr", output / "motion-%03d.png")
    positions = [centroid(Image.open(path).convert("RGB").resize((1920, 1080)),
                          lambda r, g, b: r > 235 and 210 < g < 240 and b > g + 4 and r > b + 8)
                 for path in sorted(output.glob("motion-*.png"))]
    steps = [math.dist(a, b) for a, b in zip(positions, positions[1:])]
    assert max(steps) < 55, ("Mid-drag lurch", max(steps))
    assert max(abs(a - b) for a, b in zip(steps, steps[1:])) < 9, "Abrupt drag acceleration"
    # Exclude the held pointer; it deliberately moves aside after the card lands.
    difference = ImageChops.difference(frames["landed"].crop((1050, 263, 1390, 402)), frames["hold"].crop((1050, 263, 1390, 402)))
    assert sum(ImageStat.Stat(difference).mean) / 3 < 1, "Dropped card drifts after release"
    print(json.dumps({"video": str(video), "checks": ["curved carry", "locked Mont cursor", "smooth drag speed", "drop landing", "list gap closure", "column counts", "stable hold"]}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, help="New directory for workspace, MP4 and inspection frames")
    verify(parser.parse_args().output)
