"""Render the square review, or check an existing render with --video."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import uuid

HERE = Path(__file__).resolve().parent


def run(*args):
    result = subprocess.run(args, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode:
        raise RuntimeError(result.stderr.decode() + result.stdout.decode(errors="replace"))
    return result.stdout


def render(output):
    with tempfile.TemporaryDirectory(prefix="mont-subscription-review-") as directory:
        workspace = Path(directory) / "workspace"
        envelope = Path(directory) / "command.json"
        run("mont", "init", str(workspace), "--json")

        def dispatch(command):
            envelope.write_text(json.dumps({"commandId": str(uuid.uuid4()), "command": command}))
            run("mont", "dispatch", "-C", str(workspace), str(envelope), "--json")

        for key in ("sceneWidth", "sceneHeight"):
            dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": 960})
        dispatch({"type": "user_intent:programmatic_span.create", "request": {
            "requestedId": "subscription-pile-up",
            "placement": {"startMs": 0, "durationMs": 5500, "trackId": "template", "layer": 0},
            "capability": {"kind": "programmatic.scene", "version": 1, "state": {
                "sourceTsx": (HERE / "scene.tsx").read_text(), "variables": {},
                "frame": {"x": 0, "y": 0, "width": 960, "height": 960}}},
            "interaction": {"selectable": True},
            "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
        run("mont", "render", "-C", str(workspace), "--output", str(output), "--fps", "30", "--json")


def verify(video):
    info = json.loads(run("ffprobe", "-v", "error", "-select_streams", "v:0",
                          "-show_entries", "stream=width,height,nb_frames", "-of", "json", str(video)))
    stream = info["streams"][0]
    assert (stream["width"], stream["height"], int(stream["nb_frames"])) == (960, 960, 165)

    def pixels(seconds, crop):
        return run("ffmpeg", "-v", "error", "-ss", str(seconds), "-i", str(video),
                   "-frames:v", "1", "-vf", "crop=" + crop,
                   "-f", "rawvideo", "-pix_fmt", "gray", "pipe:1")

    def bright(seconds, crop):
        return sum(value > 90 for value in pixels(seconds, crop))

    # Each card must be absent before its cue and readable after settling.
    assert bright(0.2, "400:150:120:115") < 10
    assert bright(0.95, "400:150:120:115") > 1000
    assert bright(2.9, "325:135:500:40") < 10
    assert bright(3.8, "325:135:500:40") > 1000

    # Before the second cue, the first card grows around its own fixed center
    # and overshoots its resting size rather than sliding in from an edge.
    def bounds(seconds):
        left, top, right, bottom = 960, 960, 0, 0
        for index, value in enumerate(pixels(seconds, "960:960:0:0")):
            if value > 23:
                x, y = index % 960, index // 960
                left, top = min(left, x), min(top, y)
                right, bottom = max(right, x), max(bottom, y)
        assert right > left and bottom > top
        return ((left + right) / 2, (top + bottom) / 2, right - left)

    small, overshoot, settled = (bounds(t) for t in (0.367, 0.5, 0.633))
    for pose in (small, overshoot, settled):
        assert abs(pose[0] - 315) < 7 and abs(pose[1] - 187) < 7
    assert overshoot[2] > settled[2] * 1.10
    assert overshoot[2] > small[2] * 1.20

    # A settled card must keep moving, not freeze after its entrance.
    a = pixels(4.2, "400:180:120:100")
    b = pixels(5.3, "400:180:120:100")
    assert sum(abs(x - y) for x, y in zip(a, b)) / len(a) > 3

    # Grow rendered silhouettes by 12 source pixels on each side. They must
    # remain separate, including entrance overshoot and the entire late wobble.
    masks = run("ffmpeg", "-v", "error", "-i", str(video), "-vf",
                "scale=240:240:flags=area,format=gray,"
                "lut=y='if(gt(val,18),255,0)',dilation,dilation,dilation",
                "-f", "rawvideo", "-pix_fmt", "gray", "pipe:1")
    cues = (0.3, 0.65, 1, 1.4, 1.8, 2.2, 2.6, 3)
    for frame in range(165):
        seconds = frame / 30
        # An almost-transparent entrance has no reliable body silhouette yet.
        if any(0 <= seconds - cue < 0.12 for cue in cues):
            continue
        mask = bytearray(masks[frame * 57600:(frame + 1) * 57600])
        components = 0
        for start in range(len(mask)):
            if not mask[start]:
                continue
            mask[start] = 0
            stack, size = [start], 0
            while stack:
                pixel = stack.pop()
                size += 1
                x = pixel % 240
                for neighbor in (pixel - 240 if pixel >= 240 else -1,
                                 pixel + 240 if pixel < 57360 else -1,
                                 pixel - 1 if x else -1,
                                 pixel + 1 if x < 239 else -1):
                    if neighbor >= 0 and mask[neighbor]:
                        mask[neighbor] = 0
                        stack.append(neighbor)
            components += size > 12  # Ignore isolated compression speckles.
        assert components == sum(seconds > cue for cue in cues), (
            f"Card clearance failed at {seconds:.3f}s: {components} silhouettes")
    print("PASS: square 165-frame render; staggered arrivals; centered bounce; "
          "continuous late wobble; padded card silhouettes stay separate")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--video", type=Path, help="Verify an existing default render")
    parser.add_argument("--output", type=Path, default=Path("subscription-pile-up.mp4"))
    args = parser.parse_args()
    video = (args.video or args.output).resolve()
    if not args.video:
        render(video)
    verify(video)
