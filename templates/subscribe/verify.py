#!/usr/bin/env python3
"""Render native subscribe templates and retain asset/interaction-boundary evidence."""
import argparse
import base64
import hashlib
import io
import json
from pathlib import Path
import re
import subprocess
import uuid

from PIL import Image, ImageChops, ImageStat, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parent
CASES = {
    "tiktok-follow": (1080, 1920, 4500, (608, 1631, 858, 1711)),
    "instagram-follow": (1080, 1920, 4500, (628, 1631, 878, 1711)),
    "youtube-subscribe": (1920, 1080, 4500, (1067, 898, 1287, 966)),
    "x-follow-card": (1920, 1080, 5000, (1110, 465, 1286, 517)),
    "creator-follow-card": (1920, 1080, 4500, (1139, 415, 1313, 469)),
}


def run(*args):
    result = subprocess.run([str(arg) for arg in args], stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=900)
    if result.returncode:
        raise RuntimeError(result.stderr.decode() + result.stdout.decode())
    return result.stdout


def delta(a, b, noise_floor=0):
    difference = ImageChops.difference(a, b)
    if noise_floor:
        difference = difference.point(lambda channel: max(0, channel - noise_floor))
    return sum(ImageStat.Stat(difference).mean) / 3


def verify(output, name, case, create_only=False):
    width, height, duration, button_box = case
    folder = output / name
    folder.mkdir(parents=True, exist_ok=False)
    workspace = folder / "workspace"
    run("mont", "init", workspace, "--json")

    def dispatch(command):
        envelope = folder / "command.json"
        envelope.write_text(json.dumps({"commandId": str(uuid.uuid4()), "command": command}))
        run("mont", "dispatch", "-C", workspace, envelope, "--json")

    for key, value in (("sceneWidth", width), ("sceneHeight", height)):
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    source = (ROOT / name / "scene.tsx").read_text()
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": name,
        "placement": {"startMs": 0, "durationMs": duration, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "sourceTsx": source, "variables": {},
            "frame": {"x": 0, "y": 0, "width": width, "height": height}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    # The canonical renderer requires explicit image bindings, including data URIs.
    images = re.findall(r'"(data:[^"\r\n]+)"', source)
    inventory = []
    for variable, src in re.findall(r'(\w+): stringVar\("(data:[^"\r\n]+)"\)', source):
        payload = base64.b64decode(src.split(",", 1)[1])
        image = Image.open(io.BytesIO(payload))
        inventory.append({"variable": variable, "sha256": hashlib.sha256(payload).hexdigest(),
                          "bytes": len(payload), "dimensions": image.size, "format": image.format})
    (folder / "asset-inventory.json").write_text(json.dumps(
        sorted(inventory, key=lambda item: item["variable"]), indent=2) + "\n")
    if images:
        dispatch({"type": "setProjectValue", "pointer": "/resources",
                  "value": {src: {"uri": src} for src in images}})
    if create_only:
        print(json.dumps({"template": name, "workspace": str(workspace)}), flush=True)
        return
    video = folder / (name + ".mp4")
    render_result = run("mont", "render", "-C", workspace, "--output", video, "--fps", 30, "--json")
    (folder / "render.json").write_bytes(render_result)
    metadata = json.loads(run("ffprobe", "-v", "error", "-show_streams", "-show_format", "-of", "json", video))
    stream = next(s for s in metadata["streams"] if s["codec_type"] == "video")
    assert (stream["width"], stream["height"]) == (width, height), metadata
    assert abs(float(metadata["format"]["duration"]) - duration / 1000) < 0.08, metadata
    moments = {"initial": 0, "entry": 0.3, "before": 0.9, "press": 1.066667,
               "burst": 1.3, "after": 1.9, "hold": 3.5, "end": 4.2}
    if name == "creator-follow-card":
        moments["press"] = 1.2
    frames = {}
    for label, seconds in moments.items():
        path = folder / (label + ".png")
        run("ffmpeg", "-v", "error", "-ss", seconds, "-i", video,
            "-vf", "colorspace=all=bt709:trc=iec61966-2-1:range=pc,format=rgb24",
            "-frames:v", 1, path)
        frames[label] = Image.open(path).convert("RGB")
    if name in ("tiktok-follow", "instagram-follow", "youtube-subscribe"):
        avatar_box = {
            "tiktok-follow": (210, 1611, 330, 1731),
            "instagram-follow": (190, 1611, 310, 1731),
            "youtube-subscribe": (621, 884, 717, 980),
        }[name]
        avatar = frames["after"].crop(avatar_box)
        pixels = list(avatar.getdata())
        orange = sum(r > 190 and 45 < g < 160 and b < 90 for r, g, b in pixels) / len(pixels)
        ink = sum(r < 90 and g < 65 and b < 65 for r, g, b in pixels) / len(pixels)
        assert orange > 0.35 and ink > 0.06, (name, "Mont mark missing", orange, ink)
    if name == "x-follow-card":
        hero = frames["after"].crop((580, 288, 1340, 440))
        texture = ImageStat.Stat(hero.convert("L").filter(ImageFilter.FIND_EDGES)).mean[0]
        avatar_detail = ImageStat.Stat(frames["after"].crop((614, 402, 702, 490))).stddev
        assert texture > 4 and min(avatar_detail) > 20, ("X photographic cover/avatar missing", texture, avatar_detail)
        stat_box = (610, 635, 1310, 686)
        avatars_box = (600, 706, 862, 758)
        stat_delta = delta(frames["entry"].crop(stat_box), frames["after"].crop(stat_box))
        avatar_delta = delta(frames["entry"].crop(avatars_box), frames["after"].crop(avatars_box))
        assert stat_delta > 8 and avatar_delta > 8, (stat_delta, avatar_delta)
    checks = {}
    if button_box:
        checks["button_state_delta"] = delta(frames["before"].crop(button_box), frames["after"].crop(button_box))
        checks["press_delta"] = delta(frames["before"].crop(button_box), frames["press"].crop(button_box))
        assert checks["button_state_delta"] > 2, checks
        assert checks["press_delta"] > 1, checks
        burst_box = (button_box[0] - 60, button_box[1] - 45,
                     button_box[2] + 60, button_box[3] + 45)
        burst_region = frames["burst"].crop(burst_box)
        settled_region = frames["after"].crop(burst_box)
        # Mask the button itself: the surrounding particle burst must visibly change.
        inner = (60, 45, 60 + button_box[2] - button_box[0], 45 + button_box[3] - button_box[1])
        ImageDraw.Draw(burst_region).rectangle(inner, fill=0)
        ImageDraw.Draw(settled_region).rectangle(inner, fill=0)
        checks["particle_burst_delta"] = delta(burst_region, settled_region)
        assert checks["particle_burst_delta"] > 0.05, checks
        if name == "creator-follow-card":
            # The completed interaction stays readable while card and pointer keep drifting.
            for target, box, limit in (
                ("card", (1100, 390, 1330, 475), 8),
                ("cursor", (1270, 475, 1420, 590), 14),
            ):
                bounds = [frames[label].crop(box).convert("L").point(lambda value: 255 if value < 75 else 0).getbbox()
                          for label in ("after", "hold", "end")]
                assert all(bound is not None for bound in bounds), (target, "missing interaction", bounds)
                movement = max(max(bound[axis] for bound in bounds) - min(bound[axis] for bound in bounds)
                               for axis in (0, 1))
                checks[target + "_drift_px"] = movement
                assert 2 <= movement <= limit, (target, "residual drift missing or excessive", movement)
        else:
            checks["hold_delta"] = delta(frames["after"].crop(button_box), frames["hold"].crop(button_box))
            assert checks["hold_delta"] < 1, checks
        if name not in ("creator-follow-card", "x-follow-card"):
            checks["entry_delta"] = delta(frames["initial"], frames["before"])
            checks["exit_delta_raw"] = delta(frames["initial"], frames["end"])
            # Canonical H.264 varies dark background channels by a few code values.
            checks["exit_delta"] = delta(frames["initial"], frames["end"], noise_floor=8)
            assert checks["entry_delta"] > 1, checks
            assert checks["exit_delta"] < 1, checks
        elif name == "creator-follow-card":
            checks["entry_delta"] = delta(frames["initial"], frames["after"])
            assert checks["entry_delta"] > 2, checks
    tile_width = 260 if height > width else 480
    tile_height = round(height * tile_width / width)
    sheet = Image.new("RGB", (tile_width * 4, (tile_height + 28) * 2), "#24262B")
    draw = ImageDraw.Draw(sheet)
    for i, (label, frame) in enumerate(frames.items()):
        x, y = (i % 4) * tile_width, (i // 4) * (tile_height + 28)
        sheet.paste(frame.resize((tile_width, tile_height)), (x, y + 28))
        draw.text((x + 8, y + 7), f"{label}: {moments[label]:.3f}s", fill="white")
    sheet.save(folder / "contact-sheet.png")
    (folder / "checks.json").write_text(json.dumps(checks, indent=2) + "\n")
    print(json.dumps({"template": name, "video": str(video), "checks": checks}), flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-root", type=Path, required=True, help="New evidence directory outside the public repository")
    parser.add_argument("--create-only", action="store_true",
                        help="Create resource-bound editable native workspaces without rendering")
    parser.add_argument("--scene", choices=CASES, help="Render one template")
    args = parser.parse_args()
    output = args.output_root.resolve()
    assert not output.is_relative_to(ROOT.parents[1]), "Keep preview artifacts outside the public repository"
    output.mkdir(parents=True, exist_ok=False)
    cases = {name: case for name, case in CASES.items()
             if name == args.scene or (args.scene is None and (ROOT / name / "scene.tsx").is_file())}
    for name, case in cases.items():
        verify(output, name, case, args.create_only)
    if not args.create_only:
        print(f"PASS: {len(cases)} native MP4(s); dimensions/durations, button states, particle bursts and holds")


if __name__ == "__main__":
    main()
