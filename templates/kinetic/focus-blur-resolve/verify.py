"""Render the real template and check focus, framing, hold, and loop boundaries."""
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
        "requestedId": "focus-blur-resolve",
        "placement": {"startMs": 0, "durationMs": 3600, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "sourceTsx": SOURCE, "variables": {"width": width, "height": height, **variables},
            "frame": {"x": 0, "y": 0, "width": width, "height": height}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    video = root / f"{width}x{height}.mp4"
    run("mont", "render", "-C", str(workspace), "--output", str(video), "--fps", "30", "--json")

    def pixels(time):
        # Exclude the canvas's antialiased outer edge, not any headline pixels.
        return run("ffmpeg", "-v", "error", "-ss", str(time), "-i", str(video),
                   "-frames:v", "1", "-vf", "crop=iw-8:ih-8:4:4",
                   "-f", "rawvideo", "-pix_fmt", "gray", "pipe:1")

    def sharpness(frame):
        # Normalize contrast: a faded sharp image must not pass as optical defocus.
        contrast = max(frame) - min(frame)
        assert contrast > 20, "headline is not visible"
        return max(abs(frame[i] - frame[i - 1]) for i in range(1, len(frame)) if i % (width - 8)) / contrast

    start, entry, held, exit, end = [pixels(t) for t in (0, 0.4, 1.6, 3.1, 3.5)]
    def band(frame, left, right):
        return [frame[y * (width - 8) + x]
                for y in range(int(height * 0.3), int(height * 0.7))
                for x in range(int(width * left), int(width * right))]

    assert max(band(entry, 0.08, 0.33)) - min(band(entry, 0.08, 0.33)) > 20, "first word has not appeared"
    assert max(band(entry, 0.62, 0.94)) - min(band(entry, 0.62, 0.94)) < 6, "last word appears before its turn"
    assert max(start) - min(start) < 6, "loop starts with visible text"
    assert max(end) - min(end) < 6, "loop ends with visible text"
    assert sharpness(entry) < sharpness(held) * 0.7, "entry blur is missing"
    assert sharpness(exit) < sharpness(held) * 0.7, "exit blur is missing"
    # Allow sparse inter-frame codec noise, not continued scaling or refocusing.
    assert sum(abs(a - b) for a, b in zip(held, pixels(2))) / len(held) < 0.05, "headline moves during its hold"
    ink = [(i % (width - 8) + 4, i // (width - 8) + 4) for i, value in enumerate(held) if value < 140]
    xs, ys = zip(*ink)
    assert min(xs) > width * 0.04 and max(xs) < width * 0.96, "headline clips horizontally"
    assert min(ys) > height * 0.3 and max(ys) < height * 0.7, "headline is not vertically centered"
    assert max(xs) - min(xs) > width * 0.65, "headline does not use available width"
    print(f"PASS {width}x{height}: word-by-word focus, real entry/exit blur, stable hold, centered text, clean loop")


if __name__ == "__main__":
    with tempfile.TemporaryDirectory(prefix="mont-focus-blur-") as temporary:
        root = Path(temporary)
        verify(root, 960, 540, {})
        verify(root, 540, 960, {"firstWord": "Find", "secondWord": "your", "thirdWord": "focus.", "focusMs": 600, "holdMs": 1200})
