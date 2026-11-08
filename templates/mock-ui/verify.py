"""Native render check: progressive typing, send/clear, streamed reply, readable hold.
Run: python3 templates/mock-ui/verify.py (requires mont and ffmpeg).
"""
import json
from pathlib import Path
import subprocess
import tempfile
import uuid

ROOT = Path(__file__).parent
CASES = {
    "chatgpt-exchange": {"full": 3.9, "draft": (172, 300, 645, 326),
        "cleared": (172, 456, 645, 482), "bubble": (140, 148, 840, 178),
        "reply": (142, 226, 820, 350)},
    "claude-exchange": {"full": 3.3, "draft": (168, 232, 775, 259),
        "cleared": (168, 390, 775, 417), "bubble": (180, 79, 808, 105),
        "reply": (190, 159, 790, 300)},
}


def run(*args):
    result = subprocess.run(args, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode:
        raise RuntimeError(result.stderr.decode() + result.stdout.decode(errors="replace"))
    return result.stdout


def verify(root, name, case):
    workspace = root / name
    run("mont", "init", str(workspace), "--json")

    def dispatch(command):
        envelope = root / "command.json"
        envelope.write_text(json.dumps({"commandId": str(uuid.uuid4()), "command": command}))
        run("mont", "dispatch", "-C", str(workspace), str(envelope), "--json")

    for key, value in [("sceneWidth", 960), ("sceneHeight", 540)]:
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": name,
        "placement": {"startMs": 0, "durationMs": 11500, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "sourceTsx": (ROOT / name / "scene.tsx").read_text(), "variables": {},
            "frame": {"x": 0, "y": 0, "width": 960, "height": 540}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    video = root / (name + ".mp4")
    run("mont", "render", "-C", str(workspace), "--output", str(video), "--fps", "30", "--json")

    def ink(seconds, region):
        x, y, right, bottom = region
        width, height = right - x, bottom - y
        rgb = run("ffmpeg", "-v", "error", "-ss", str(seconds), "-i", str(video),
                  "-frames:v", "1", "-vf", f"format=rgb24,crop={width}:{height}:{x}:{y}",
                  "-f", "rawvideo", "-pix_fmt", "rgb24", "pipe:1")
        assert len(rgb) == width * height * 3, "wrong native output dimensions"
        return {(i % width, i // width) for i, (r, g, b) in
                enumerate(zip(rgb[::3], rgb[1::3], rgb[2::3])) if min(r, g, b) > 100}

    def width(mask):
        return max(x for x, _ in mask) - min(x for x, _ in mask) + 1

    prefix = ink(2.0, case["draft"])
    full = ink(case["full"], case["draft"])
    assert 20 < len(prefix) < len(full) * 0.9, "prompt does not type progressively"
    assert width(prefix) < width(full) * 0.9, "draft is not a growing prefix"
    bubble = ink(4.6, case["bubble"])
    assert abs(width(bubble) - width(full)) <= 5, "sent prompt loses or clips text"
    placeholder = ink(0.3, case["draft"])
    cleared = ink(4.6, case["cleared"])
    assert len(cleared ^ placeholder) < max(20, len(placeholder) * 0.15), "input does not return to its placeholder after send"
    partial = ink(6.2, case["reply"])
    finished = ink(10.0, case["reply"])
    held = ink(11.0, case["reply"])
    assert 100 < len(partial) < len(finished) * 0.9, "assistant reply does not stream"
    assert abs(len(held) - len(finished)) < len(finished) * 0.03, "reply changes during reading hold"
    assert max(y for _, y in finished) - min(y for _, y in finished) > 70, "reply lines overlap or disappear"
    print(f"PASS {name}: typed prefix, complete sent prompt, cleared input, streamed reply, stable hold")


if __name__ == "__main__":
    with tempfile.TemporaryDirectory(prefix="mont-mock-ui-check-") as temporary:
        for source in sorted(ROOT.glob("*/scene.tsx")):
            name = source.parent.name
            verify(Path(temporary), name, CASES[name])
