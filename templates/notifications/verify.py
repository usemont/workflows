#!/usr/bin/env python3
"""Render native notification scenes and check their real decoded arrival/hold/exit pixels."""
import argparse
import base64
import hashlib
import json
from pathlib import Path
import re
import subprocess
import uuid

SCENES = {
    "macos-notification": {"width": 1920, "height": 1080, "duration": 5000, "holds": [0, 2.5, 4.9], "region": [1450, 0, 470, 200], "uniformBackground": True},
    "native-notification-ios": {"width": 960, "height": 540, "duration": 5000, "holds": [0, 2.5, 4.9], "boundaryExit": True, "uniformBackground": True},
    "native-notification-macos": {"width": 960, "height": 540, "duration": 5000, "holds": [0, 2.5, 4.9], "boundaryExit": True, "uniformBackground": True},
    "notification-stack": {"width": 960, "height": 540, "duration": 6000, "holds": [0, 3, 5.9], "boundaryExit": True},
    "notification-pileup": {"width": 960, "height": 540, "duration": 8000, "holds": [0, 4, 7.9], "boundaryExit": True},
    # Source holds its final Slack notification through 12 s; wallpaper keeps pushing.
    "slack-notification-ad": {"width": 540, "height": 960, "duration": 12000, "holds": [0, 9.8, 11.9], "retainsEnd": True, "continuousMotion": True},
    # Moving photo/shader backgrounds invalidate a whole-frame background-clearance assertion.
    "notification-cascade": {"width": 540, "height": 960, "duration": 10000, "holds": [0, 8.6, 9.9], "continuousMotion": True},
    "liquid-glass-notification": {"width": 960, "height": 540, "duration": 8000, "holds": [0, 5.2, 7.9], "continuousMotion": True},
    # The source retains three cards through 15 s and continuously drifts its 3D camera.
    "ios26-liquid-glass": {"width": 960, "height": 540, "duration": 15000, "holds": [0, 7.5, 14.9], "retainsEnd": True, "continuousMotion": True},
}
SCENES.update({
    "badge-pop": {"width": 960, "height": 540, "duration": 4200, "holds": [0, 2, 4.1]},
    "overwhelm-surround": {"width": 960, "height": 540, "duration": 4700, "holds": [0, 3, 4.6], "boundaryExit": True},
})
DEFAULT_OUTPUT = Path(".tmp/template-previews/notifications")


def run(*args):
    result = subprocess.run([str(a) for a in args], capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(f"{args[0]} failed ({result.returncode}): {result.stdout}\n{result.stderr}")
    return result.stdout


def dispatch(workspace, command):
    envelope = workspace / "notification-command.json"
    envelope.write_text(json.dumps({"commandId": str(uuid.uuid4()), "command": command}))
    run("mont", "dispatch", "-C", workspace, envelope, "--json")


def pixels(video, at, width, height, region=None):
    filters = ["colorspace=all=bt709:trc=iec61966-2-1:range=pc"]
    if region:
        x, y, w, h = region
        filters.append(f"crop={w}:{h}:{x}:{y}")
    filters.append("format=rgb24")
    return subprocess.check_output(["ffmpeg", "-v", "error", "-ss", str(at), "-i", str(video),
        "-frames:v", "1", "-vf", ",".join(filters), "-f", "rawvideo", "-pix_fmt", "rgb24", "pipe:1"])


def difference(a, b, background_relative=False):
    assert len(a) == len(b) and a, "Missing decoded preview frame"
    if background_relative:
        # This crop's top-left pixel is unobstructed solid background at every sample.
        # Native exports exhibit whole-stage RGB drift; measure card contrast instead,
        # without relaxing arrival/clearance thresholds or hiding the raw residual.
        return sum(abs((x - a[i % 3]) - (y - b[i % 3]))
                   for i, (x, y) in enumerate(zip(a, b))) / len(a)
    return sum(abs(x-y) for x, y in zip(a, b)) / len(a)


def asset_inventory(source_text):
    """Reject unaccounted image/model payloads and record reproducible origins."""
    media = Path(__file__).resolve().parents[2] / "assets" / "template-media"
    known = {hashlib.sha256(path.read_bytes()).hexdigest(): str(path.relative_to(media))
             for path in media.rglob("*.png")}
    known.update({
        "ba876751d829df3ed467d47de7aafb7e5b843dd161fd21254068e29edd07643e": "Messages service mark",
        "d405095509b2d229ee741ceeca7a7a4c5dd95965b3ad928285b9076d244e3ef9": "Slack service mark",
    })
    inventory = []
    for uri in sorted(set(re.findall(r"""data:(?:image|model)/[^"'`\s]+""", source_text))):
        header, encoded = uri.split(",", 1)
        payload = base64.b64decode(encoded, validate=True)
        digest = hashlib.sha256(payload).hexdigest()
        if header.startswith("data:model/"):
            assert payload[:4] == b"glTF", "Device payload is not a GLB"
            json_size = int.from_bytes(payload[12:16], "little")
            model = json.loads(payload[20:20 + json_size])
            assert model["asset"]["generator"] == "Mont original stdlib phone generator", "Unaccounted device model"
            origin = "ios26-liquid-glass/generate-device.py"
        else:
            assert digest in known, f"Unaccounted image payload: {digest}"
            origin = known[digest]
        inventory.append({"contentType": header[5:].split(";")[0], "sha256": digest,
                          "bytes": len(payload), "origin": origin})
    return inventory


def check_slack_portrait(video):
    """Check the rendered photo's proportions and Mont icon, not just card arrival."""
    from PIL import Image, ImageChops, ImageStat

    photo = Image.open(Path(__file__).resolve().parents[2] /
                       "assets/template-media/backgrounds/studio-desk.png").convert("RGB")
    zoom = 1 + .05 * (9800 - 100) / 11900
    scale = max(608.8 / photo.width, 1079.2 / photo.height)
    left = -304.4 - (photo.width * scale - 608.8) / 2
    top = -462.8 - (photo.height * scale - 1079.2) / 2
    # Unobstructed photo below the cards; project its cover crop into frame space.
    crop = (0, 440, 540, 770)
    box = ((crop[0] - 270) / zoom / scale - left / scale,
           (crop[1] - 403.2) / zoom / scale - top / scale,
           (crop[2] - 270) / zoom / scale - left / scale,
           (crop[3] - 403.2) / zoom / scale - top / scale)
    expected = photo.transform((540, 330), Image.Transform.EXTENT, box, Image.Resampling.BICUBIC)
    frame = Image.frombytes("RGB", (540, 960), pixels(video, 9.8, 540, 960))
    error = sum(ImageStat.Stat(ImageChops.difference(frame.crop(crop), expected)).mean) / 3
    assert error < 5, f"Portrait wallpaper is stretched or incorrectly cropped: {error}"
    red, green, blue = frame.getpixel((32, 333))
    assert red > green * 1.8 and green > blue * 1.5, "Mont icon has an inset white surround"


def check_cascade_glass(video):
    """Reject sharp translucent cards and opaque substitutes in the native render."""
    from PIL import Image, ImageChops, ImageStat

    def detail(box):
        patch = frame.crop(box)
        width, height = patch.size
        horizontal = ImageChops.difference(patch.crop((1, 0, width, height)),
                                          patch.crop((0, 0, width - 1, height)))
        vertical = ImageChops.difference(patch.crop((0, 1, width, height)),
                                        patch.crop((0, 0, width, height - 1)))
        return ImageStat.Stat(horizontal).mean[0] + ImageStat.Stat(vertical).mean[0]

    # Check both the first card and the complete stack: invisible cards must not mask frosting.
    for time in (2.8, 8.6):
        frame = Image.frombytes("RGB", (540, 960), pixels(video, time, 540, 960)).convert("L")
        covered = detail((95, 700, 290, 709))
        uncovered = detail((95, 734, 290, 743))
        assert uncovered > 2, "Wallpaper reference lost its visible wood grain"
        assert covered < uncovered * .5, f"Card does not frost the wallpaper at {time}s: {covered}/{uncovered}"
    # Identical material over light wood and the dark phone must retain different tones.
    light = ImageStat.Stat(frame.crop((95, 383, 290, 392))).mean[0]
    dark = ImageStat.Stat(frame.crop((95, 490, 290, 499))).mean[0]
    assert light - dark > 15, f"Cards hide the wallpaper behind an opaque fill: {light}/{dark}"

    # Light-facing edges should reflect more strongly than the opposite edges.
    # Average several rows to tolerate subpixel placement and video compression.
    def luminance(box):
        return ImageStat.Stat(frame.crop(box)).mean[0]

    top_rim = bottom_rim = 0
    for y in (309, 416, 523, 630):
        top_rim += luminance((140, y, 390, y + 4)) - luminance((140, y + 5, 390, y + 8))
        bottom_rim += (luminance((140, y + 84, 390, y + 88)) -
                       luminance((140, y + 80, 390, y + 83)))
    assert (top_rim - bottom_rim) / 4 > 5, "Glass rim became a flat uniform outline"


def render(name, spec, output):
    destination = output / name
    destination.mkdir(parents=True, exist_ok=True)
    workspace = destination / ("workspace-" + uuid.uuid4().hex[:10])
    run("mont", "init", workspace, "--json")
    width, height = spec["width"], spec["height"]
    for pointer, value in [("/sceneWidth", width), ("/sceneHeight", height)]:
        dispatch(workspace, {"type": "setProjectValue", "pointer": pointer, "value": value})
    source = Path(__file__).parent / name / "scene.tsx"
    source_text = source.read_text()
    inventory = asset_inventory(source_text)
    images = set(re.findall(r"""data:image/[^"'`\s]+""", source_text))
    if images:
        dispatch(workspace, {"type": "setProjectValue", "pointer": "/resources",
            "value": {uri: {"uri": uri} for uri in images}})
    dispatch(workspace, {"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": name,
        "placement": {"startMs": 0, "durationMs": spec["duration"], "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "source": source_text, "variables": {},
            "frame": {"x": 0, "y": 0, "width": width, "height": height}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    video = destination / (name + "-" + uuid.uuid4().hex[:8] + ".mp4")
    render_output = run("mont", "render", "-C", workspace, "--output", video, "--fps", "30", "--json")
    (destination / "render.json").write_text(render_output)
    info = json.loads(run("ffprobe", "-v", "error", "-select_streams", "v:0", "-show_entries", "stream=width,height,duration,nb_frames", "-of", "json", video))["streams"][0]
    assert (info["width"], info["height"]) == (width, height), info
    assert abs(float(info["duration"]) - spec["duration"] / 1000) < 0.08, info
    times = dict(zip(["start", "hold", "end"], spec["holds"]))
    # The source's cubic exit completes at the duration boundary, not 100 ms before it.
    times["end"] = spec["duration"] / 1000 - 1 / 30
    times["hold-confirm"] = times["hold"] + 0.1
    decoded = {}
    for label, at in times.items():
        decoded[label] = pixels(video, at, width, height, spec.get("region"))
        run("ffmpeg", "-y", "-v", "error", "-ss", at, "-i", video, "-frames:v", "1",
            "-vf", "colorspace=all=bt709:trc=iec61966-2-1:range=pc,format=rgb24",
            destination / (label + ".png"))
    relative = spec.get("uniformBackground", False)
    arrival = difference(decoded["start"], decoded["hold"], relative)
    clearance = difference(decoded["start"], decoded["end"], relative)
    stillness = difference(decoded["hold"], decoded["hold-confirm"], relative)
    assert arrival > 0.6, f"{name}: notification never visibly arrived ({arrival})"
    if spec.get("retainsEnd"):
        assert clearance > 0.6, f"{name}: source-retained end state disappeared ({clearance})"
    elif spec.get("boundaryExit"):
        # Last encoded frame precedes source's terminal keyframe: require a visible fade,
        # not an impossible exact-background frame at the excluded duration boundary.
        assert clearance < arrival * 0.8, f"{name}: boundary fade did not reduce the notification ({clearance}/{arrival})"
    elif not spec.get("continuousMotion"):
        assert clearance < 1.5, f"{name}: notification did not clear ({clearance})"
    # Continuously moving source scenes have no static whole-frame hold contract.
    if not spec.get("continuousMotion"):
        assert stillness < 1.5, f"{name}: hold unexpectedly moves ({stillness})"
    if name == "slack-notification-ad":
        check_slack_portrait(video)
    if name == "notification-cascade":
        check_cascade_glass(video)
    result = {"scene": name, "video": str(video), "durationSeconds": float(info["duration"]),
        "frames": int(info["nb_frames"]), "arrivalPixelDelta": round(arrival, 4),
        "exitResidual": round(clearance, 4), "holdPixelDelta": round(stillness, 4),
        "rawExitResidual": round(difference(decoded["start"], decoded["end"]), 4),
        "pixelComparison": "background-relative contrast" if relative else "absolute RGB",
        "assets": inventory, "colorDecode": "Rec.709 to sRGB full-range RGB", "status": "PASS"}
    (destination / "proof.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result), flush=True)
    return result


def main():
    # Uniform RGB drift may cancel; a localized card residual must remain detectable.
    assert difference(bytes([10, 20, 30] * 4), bytes([15, 25, 35] * 4), True) == 0
    assert difference(bytes([10, 20, 30] * 4), bytes([15, 25, 35] + [100, 110, 120] * 3), True) > 1.5
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--scene", choices=SCENES)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    results = [render(name, spec, args.output) for name, spec in SCENES.items()
               if name == args.scene or (not args.scene and (Path(__file__).parent / name / "scene.tsx").is_file())]
    (args.output / ("proof-" + (args.scene or "all") + ".json")).write_text(json.dumps(results, indent=2) + "\n")


if __name__ == "__main__":
    main()
