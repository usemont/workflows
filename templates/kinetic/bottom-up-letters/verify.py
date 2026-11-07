"""Run with python3 verify.py; requires mont and ffmpeg on PATH."""
import json
from pathlib import Path
import subprocess
import tempfile
import uuid

SOURCE = Path(__file__).with_name("scene.tsx").read_text()


def run(*args):
    return subprocess.check_output(args, stderr=subprocess.PIPE)


def verify(root, width, height, variables):
    workspace = root / f"{width}x{height}"
    run("mont", "init", str(workspace), "--json")

    def dispatch(command):
        envelope = root / "command.json"
        envelope.write_text(json.dumps({"commandId": str(uuid.uuid4()), "command": command}))
        run("mont", "dispatch", "-C", str(workspace), str(envelope), "--json")

    for key, value in [("sceneWidth", width), ("sceneHeight", height)]:
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "bottom-up-letters",
        "placement": {"startMs": 0, "durationMs": 4200, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "sourceTsx": SOURCE, "variables": {"width": width, "height": height, **variables},
            "frame": {"x": 0, "y": 0, "width": width, "height": height}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    video = root / f"{width}x{height}.mp4"
    run("mont", "render", "-C", str(workspace), "--output", str(video), "--fps", "30", "--json")

    def ink(time):
        pixels = run("ffmpeg", "-v", "error", "-ss", str(time), "-i", str(video),
                     "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "rgb24", "pipe:1")
        return [(i // 3 % width, i // 3 // width)
                for i in range(0, len(pixels), 3) if min(pixels[i:i + 3]) < 140]

    assert not ink(0), "Letters must be hidden before their entry"
    early = ink(.43)
    assert early and max(x for x, y in early) < width / 2, "Letters must enter left to right"
    held = ink(2)
    xs, ys = zip(*held)
    assert min(xs) > width * .04 and max(xs) < width * .96, "Copy must fit horizontally"
    assert min(xs) < width * .2 and max(xs) > width * .8, "Copy must use the available width"
    assert max(ys) - min(ys) < height * .30, "Copy must remain on one line"
    assert not ink(4.1), "Exit must finish before the loop seam"
    print(f"PASS {width}x{height}: staggered entry, single-line fit, clean loop seam")


if __name__ == "__main__":
    with tempfile.TemporaryDirectory(prefix="mont-bottom-up-") as temporary:
        root = Path(temporary)
        verify(root, 960, 540, {})
        verify(root, 540, 960, {"text": "One line, always."})
