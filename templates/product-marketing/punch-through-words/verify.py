#!/usr/bin/env python3
"""Render the source and check word beats, stepped reveals, and progressive defocus.

Requires mont, ffmpeg, ffprobe. Usage: python3 verify.py /tmp/punch-through-words.mp4
"""
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile
import uuid

PALETTE = dict(background="#101521", surface="#202A3B", border="#39465A",
               ink="#F3F4F8", muted="#A6B1C5", accent="#F5ACD1")


def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True, check=True)
    return result.stdout


def render(output):
    source = Path(__file__).with_name("scene.tsx").read_text()
    with tempfile.TemporaryDirectory(prefix="mont-punch-") as folder:
        workspace = str(Path(folder) / "workspace")
        run("mont", "init", workspace, "--json")

        def dispatch(command):
            run("mont", "dispatch", "-C", workspace, "-", "--json", stdin=json.dumps({
                "commandId": str(uuid.uuid4()), "command": command,
            }).encode())

        for key in ("sceneWidth", "sceneHeight"):
            dispatch(dict(type="setProjectValue", pointer="/" + key, value=960))
        dispatch({"type": "user_intent:programmatic_span.create", "request": {
            "requestedId": "punch-through-words",
            "placement": dict(startMs=0, durationMs=3400, trackId="template", layer=0),
            "capability": {"kind": "programmatic.scene", "version": 1, "state": {
                "source": source, "variables": PALETTE,
                "frame": dict(x=0, y=0, width=960, height=960),
            }},
            "interaction": dict(selectable=True),
            "producer": dict(kind="programmatic", id="mont-templates", version="1"),
        }})
        run("mont", "render", "-C", workspace, "--output", str(output), "--fps", "30", "--json")


def check_video(video):
    meta = json.loads(run("ffprobe", "-v", "error", "-select_streams", "v:0",
                          "-show_entries", "stream=width,height,nb_frames", "-of", "json", str(video)))["streams"][0]
    assert (meta["width"], meta["height"], int(meta["nb_frames"])) == (960, 960, 102), meta

    def frame(seconds):
        return run("ffmpeg", "-v", "error", "-ss", str(seconds), "-i", str(video),
                   "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "rgb24", "-")

    def pixels(data, box):
        x0, y0, x1, y1 = box
        for y in range(y0, y1):
            for x in range(x0, x1):
                offset = (y * 960 + x) * 3
                yield x, y, data[offset], data[offset + 1], data[offset + 2]

    def badge(data, cx, cy):
        # Track the original recorder's lavender badge, not text or source structure.
        samples = [(x, y, max(0, r + b - 2 * g - 6))
                   for x, y, r, g, b in pixels(data, (cx - 38, cy - 38, cx + 39, cy + 39))]
        total = sum(w for _, _, w in samples)
        assert total > 500, "Recorder badge disappeared"
        mx = sum(x * w for x, _, w in samples) / total
        my = sum(y * w for _, y, w in samples) / total
        spread = sum(((x - mx) ** 2 + (y - my) ** 2) * w for x, y, w in samples) / total
        return mx, my, spread

    opening, first, second, final = [frame(t) for t in (0, 1.05, 1.7, 2.35)]
    tracks = [badge(f, x, y) for f, x, y in zip(
        (opening, first, second, final), (185, 234, 282, 324), (187, 237, 284, 325))]
    assert all(25 < tracks[i + 1][0] - tracks[i][0] < 65 for i in range(3)), tracks
    for settled, later, cx, cy in ((tracks[1], frame(1.23), 234, 237), (tracks[2], frame(1.88), 282, 284)):
        position = badge(later, cx, cy)
        assert math.hypot(position[0] - settled[0], position[1] - settled[1]) < 5, "Camera must hold between impacts"
    assert tracks[3][2] / 0.53 ** 2 > tracks[0][2] * 1.5, "Cards must progressively defocus, not merely shrink"

    for cue, cx, cy in ((0.7, 185, 187), (1.35, 234, 237), (2.0, 282, 284)):
        before = badge(frame(cue - 0.075), cx, cy)
        approaching = badge(frame(cue - 0.04), cx, cy)
        assert math.hypot(approaching[0] - before[0], approaching[1] - before[1]) < 3, "Background must wait for stamp contact"
        struck = badge(frame(cue + 0.1), cx + 30, cy + 30)
        assert struck[0] - before[0] > 20, "Each stamp must immediately push the cards into the background"

    bridge_pixels = sum(max(r, g, b) > 35 and max(r, g, b) - min(r, g, b) > 10
                        for _, _, r, g, b in pixels(final, (550, 645, 715, 735)))
    assert bridge_pixels > 250, "The new field must fill the gap beside the original lower cards"

    def music_badge_area(seconds):
        # A newly arriving peach badge grows from its center, overshoots, then settles.
        return sum(r > 55 and r > g * 1.08 and g > b * 1.08
                   for _, _, r, g, b in pixels(frame(seconds), (740, 220, 950, 345)))

    unborn, entering, overshoot, settled = [music_badge_area(t) for t in (0.733, 0.8, 0.967, 1.23)]
    assert unborn < 10, "New cards must not already exist before their entrance"
    assert entering < settled * 0.3, "Cards must grow in, not simply slide into view"
    assert overshoot > settled * 1.25, "Card entrances must visibly overshoot before settling"

    def word_shape(seconds):
        columns = {}
        peak = 0
        for x, _, r, g, b in pixels(frame(seconds), (0, 300, 960, 650)):
            if min(r, g, b) > 65 and max(r, g, b) - min(r, g, b) < 25:
                columns[x] = columns.get(x, 0) + 1
                peak = max(peak, r)
        # Ignore isolated lettering on background cards.
        body = [x for x, count in columns.items() if count > 12]
        return (max(body) - min(body) + 1 if body else 0), peak

    arriving_width, arriving_ink = word_shape(1.933)
    stamped_width, stamped_ink = word_shape(2.35)
    contact_width, contact_ink = word_shape(2.033)
    assert abs(contact_width - stamped_width) < stamped_width * 0.03 and contact_ink > stamped_ink * 0.9, "Word must be sharp, solid and landed when the camera starts receding"
    assert arriving_width > stamped_width * 1.15, "Words must stamp down from a larger state, not pop up from below"
    assert arriving_ink < stamped_ink * 0.8, "The approaching word must start translucent before becoming solid"
    counts = []
    for data in (opening, first, second, final):
        counts.append(sum(min(r, g, b) > 175 for _, _, r, g, b in pixels(data, (100, 330, 860, 620))))
    assert counts[0] < 3000, "Opening must contain only cards"
    assert counts[1] > 20000 and counts[2] > counts[1] * 1.3, counts
    assert counts[3] > 30000, "The final word must use the same white ink as the preceding words"
    print("PASS: square render; three synchronized stamp impacts; stepped camera holds; continuous reveals; progressive defocus; spring entrances; translucent word approaches")


if __name__ == "__main__":
    output = Path(sys.argv[1]).resolve()
    render(output)
    check_video(output)
