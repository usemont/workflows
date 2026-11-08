"""Render native frames; assert dense packing, concurrent reveals and outward-only framing."""
import json
from pathlib import Path
import subprocess
import tempfile
import uuid

SOURCE = Path(__file__).with_name("scene.tsx").read_text()
COLORS = ["#FF4040", "#40FF40", "#4040FF", "#FFFF40", "#40FFFF", "#FF40FF"]
CODES = {4: 0, 2: 1, 1: 2, 6: 3, 3: 4, 5: 5}
STARTS = (0.52, 1.04, 1.56, 2.08, 2.60)
LANDINGS = (0.40, 0.96, 1.48, 2.00, 2.52, 3.04)


def run(*args):
    return subprocess.check_output(args, stderr=subprocess.PIPE)


def verify(root, width, height, words=None):
    workspace = root / f"{width}x{height}"
    run("mont", "init", str(workspace), "--json")

    def dispatch(command):
        envelope = root / "command.json"
        envelope.write_text(json.dumps({"commandId": str(uuid.uuid4()), "command": command}))
        run("mont", "dispatch", "-C", str(workspace), str(envelope), "--json")

    for key, value in [("sceneWidth", width), ("sceneHeight", height)]:
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    variables = {"width": width, "height": height, "background": "#101820"}
    variables.update({f"ink{i + 1}": color for i, color in enumerate(COLORS)})
    if words:
        variables.update({f"word{i + 1}": word for i, word in enumerate(words)})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "camera-follow-captions",
        "placement": {"startMs": 0, "durationMs": 5000, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "sourceTsx": SOURCE, "variables": variables,
            "frame": {"x": 0, "y": 0, "width": width, "height": height}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    video = root / f"{width}x{height}.mp4"
    run("mont", "render", "-C", str(workspace), "--output", str(video), "--fps", "30", "--json")

    def frame(at):
        pixels = run("ffmpeg", "-v", "error", "-ss", str(at), "-i", str(video),
                     "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "rgb24", "pipe:1")
        assert len(pixels) == width * height * 3, "native render returned an incomplete frame"
        return pixels

    def bounds(pixels):
        boxes = [[width, height, -1, -1, 0] for _ in COLORS]
        masks = [set() for _ in COLORS]
        channels = iter(pixels)
        for position, (r, g, b) in enumerate(zip(channels, channels, channels)):
            # Include antialiased strokes after zoom-out without treating codec
            # chroma bleed as a second word.
            peak = max(r, g, b)
            if peak < 70 or any(peak * 0.45 < c < peak * 0.65 for c in (r, g, b)):
                continue
            index = CODES.get((r > peak * 0.65) * 4 + (g > peak * 0.65) * 2 + (b > peak * 0.65))
            if index is None:
                continue
            masks[index].add((position % width, position // width))
        for box, mask in zip(boxes, masks):
            while mask:
                pending = [mask.pop()]
                component = []
                while pending:
                    x, y = pending.pop()
                    component.append((x, y))
                    for neighbor in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
                        if neighbor in mask:
                            mask.remove(neighbor)
                            pending.append(neighbor)
                # H.264 chroma bleed produces isolated wrong-color pixels near
                # other words. They are not rendered glyphs or usable bounds.
                if len(component) < 6:
                    continue
                for x, y in component:
                    box[0], box[1] = min(box[0], x), min(box[1], y)
                    box[2], box[3] = max(box[2], x), max(box[3], y)
                    box[4] += 1
        return boxes

    def fitted(boxes, visible, future_hidden=True):
        for i, (left, top, right, bottom, count) in enumerate(boxes[:visible]):
            assert count > 12, f"accumulated word {i} disappeared"
            assert width * 0.025 < left < right < width * 0.975, f"word {i} clips horizontally"
            assert height * 0.025 < top < bottom < height * 0.975, f"word {i} clips vertically"
        if future_hidden:
            assert all(box[4] <= 12 for box in boxes[visible:]), "future words appear before their cue"

    def scale_ratio(before, after):
        # Measure the widest persistent word; small letters alias at overview scale.
        a, b = max(zip(before, after), key=lambda pair: pair[0][2] - pair[0][0])
        return (b[2] - b[0]) / (a[2] - a[0])

    fitted(bounds(frame(0)), 1)
    assert all(box[4] == 0 for box in bounds(frame(4.9))), "end-of-cycle reset is not blank"

    landings = [bounds(frame(at)) for at in LANDINGS]
    for i, boxes in enumerate(landings):
        fitted(boxes, i + 1)
        if i:
            assert scale_ratio(landings[i - 1][:i], boxes[:i]) < 0.98, "a step does not move outward"

    # New ink appears while the camera is still moving, never after landing.
    # Existing words remain fully framed throughout every pull-back.
    for i, start in enumerate(STARTS, 1):
        before = bounds(frame(round(start - 0.06, 3)))
        fitted(before, i)
        during = bounds(frame(round(start + 0.20, 3)))
        assert during[i][4] > 12, "new word waits for the camera to land"
        assert scale_ratio(during[:i], landings[i][:i]) < 0.995, "camera lands before the word reveal"
        previous = before
        for step in range(1, 5):
            current = bounds(frame(round(start + step / 10, 3)))
            fitted(current, i, future_hidden=False)
            assert scale_ratio(previous[:i], current[:i]) <= 1.03, "camera zooms inward during a transition"
            assert scale_ratio(before[:i], current[:i]) <= 1.03, "camera scale exceeds its previous step"
            previous = current
        assert scale_ratio(before[:i], previous[:i]) < 0.98, "transition contains no genuine zoom-out"

    hold = frame(3.5)
    overview = bounds(hold)
    fitted(overview, 6)
    assert sum(abs(a - b) for a, b in zip(hold, frame(4.1))) / len(hold) < 0.12, "final composition does not hold still"

    # Each addition touches the accumulated block's left edge or baseline.
    # Pixel checks catch oversized invisible line boxes and accidental loose rows.
    for i, box in enumerate(overview[1:], 1):
        previous = overview[:i]
        left = min(b[0] for b in previous)
        right = max(b[2] for b in previous)
        bottom = max(b[3] for b in previous)
        ink_height = box[3] - box[1]
        beside = abs(box[3] - bottom) < max(3, ink_height * 0.12) and -2 <= box[0] - right < ink_height * 0.30
        below = abs(box[0] - left) < max(3, ink_height * 0.12) and -2 <= box[1] - bottom < ink_height * 0.30
        assert beside or below, f"word {i} is not packed against the accumulated grid: {box}, {previous}"

    if words:
        # H's cap height survives H.264 chroma subsampling more reliably than
        # its thin outer stems when the diagnostic colors differ.
        for i in (2, 3):
            small = overview[i][3] - overview[i][1]
            large = overview[i + 2][3] - overview[i + 2][1]
            assert abs(large - small * 2.61803398875) < 5, "world font sizes do not follow the golden ratio"
    print(f"PASS {width}x{height}: packed grid, concurrent word/camera cues, golden-ratio type, outward-only camera")


if __name__ == "__main__":
    with tempfile.TemporaryDirectory(prefix="mont-camera-follow-") as temporary:
        root = Path(temporary)
        verify(root, 960, 540)
        verify(root, 540, 960, ["H"] * 6)
