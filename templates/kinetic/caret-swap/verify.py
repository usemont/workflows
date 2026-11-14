"""Render native Mont scenes; check bounded covers, discrete typing, caret, and loop pixels."""
import json
import math
from pathlib import Path
import subprocess
import tempfile
import uuid

SOURCE = Path(__file__).with_name("scene.tsx").read_text()
FPS = 30
SOURCE_HOLD, WIPE, COLLAPSE, PAUSE = 0.5, 0.35, 0.25, 0.1
TYPING, FINAL_HOLD, RETURN = 0.9, 0.8, 0.35
WIPE_END = SOURCE_HOLD + WIPE
COLLAPSE_END = WIPE_END + COLLAPSE
TYPE_START = COLLAPSE_END + PAUSE
TYPE_END = TYPE_START + TYPING
RETURN_START = TYPE_END + FINAL_HOLD
RETURN_COVERED = RETURN_START + WIPE
CYCLE = RETURN_COVERED + RETURN


def run(*args):
    return subprocess.check_output(args, stderr=subprocess.PIPE)


def verify(root, width, height, variables, replacement):
    workspace = root / f"{width}x{height}"
    run("mont", "init", str(workspace), "--json")

    def dispatch(command):
        envelope = root / "command.json"
        envelope.write_text(json.dumps({"commandId": str(uuid.uuid4()), "command": command}))
        run("mont", "dispatch", "-C", str(workspace), str(envelope), "--json")

    for key, value in [("sceneWidth", width), ("sceneHeight", height)]:
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "caret-swap",
        "placement": {"startMs": 0, "durationMs": round((CYCLE + 0.3) * 1000), "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "source": SOURCE, "variables": {"width": width, "height": height, **variables},
            "frame": {"x": 0, "y": 0, "width": width, "height": height}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    video = root / f"{width}x{height}.mp4"
    run("mont", "render", "-C", str(workspace), "--output", str(video), "--fps", str(FPS), "--json")

    def masks(seconds):
        # Seek just before the requested output frame, avoiding floating-point
        # rounding that could accidentally select the next typing step.
        frame = math.ceil(seconds * FPS - 1e-6)
        rgb = run("ffmpeg", "-v", "error", "-ss", str(max(0, frame / FPS - 0.0001)), "-i", str(video),
                  "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "rgb24", "pipe:1")
        assert len(rgb) == width * height * 3, "native render dimensions differ"
        ink, accent = set(), set()
        for i, (r, g, b) in enumerate(zip(rgb[::3], rgb[1::3], rgb[2::3])):
            if r > 170 and g > 170 and b > 150 and r >= g:
                ink.add(i)
            elif g > 200 and g > r * 1.15 and b < 150:
                accent.add(i)
        return ink, accent

    def bounds(mask):
        assert mask, "expected visible native pixels"
        xs = [i % width for i in mask]
        ys = [i // width for i in mask]
        return min(xs), max(xs), min(ys), max(ys)

    def span(mask):
        left, right, _, _ = bounds(mask)
        return right - left + 1

    def same_foreground(first, second, message):
        # H.264 changes antialiased edges across GOPs. Require the same silhouette
        # with a one-pixel edge allowance, not exact thresholded pixel identity.
        assert max(abs(a - b) for a, b in zip(bounds(first), bounds(second))) <= 2, message
        for a, b in ((first, second), (second, first)):
            expanded = {i + dy * width + dx for i in b for dy in (-1, 0, 1) for dx in (-1, 0, 1)}
            assert len(a - expanded) < max(20, len(a) * 0.005), message

    source, source_caret = masks(0.1)
    before_wipe, _ = masks(SOURCE_HOLD - 1 / FPS)
    same_foreground(source, before_wipe, "source changes during its short opening hold")
    early_ink, early_cover = masks(SOURCE_HOLD + WIPE * 0.2)
    wiped_ink, mid_cover = masks(SOURCE_HOLD + WIPE * 0.65)
    covered_ink, full_cover = masks(WIPE_END)
    collapsed_ink, small_caret = masks(COLLAPSE_END + PAUSE / 2)
    assert len(early_ink) > len(wiped_ink), "cover sweep does not progress smoothly"
    assert span(mid_cover) > span(early_cover) * 1.1, "cover sweep does not expand across the phrase"
    wipe_edges = [bounds(masks(SOURCE_HOLD + WIPE * fraction / 8)[1]) for fraction in range(9)]
    assert all(b[0] <= a[0] + 2 and b[1] - b[0] >= a[1] - a[0] - 2
               for a, b in zip(wipe_edges, wipe_edges[1:])), "wipe reverses direction"
    assert all(edge[0] >= bounds(full_cover)[0] - 2 and
               edge[1] - edge[0] <= span(full_cover) + 2 for edge in wipe_edges), "wipe overshoots"
    assert len(covered_ink) < len(source) * 0.01, "source remains visible through the opaque cover"
    collapsing = [masks(WIPE_END + COLLAPSE * fraction / 8) for fraction in range(9)]
    assert all(not ink for ink, _ in collapsing), "text leaks during cover collapse"
    widths = [span(cover) for _, cover in collapsing]
    assert all(b <= a + 2 for a, b in zip(widths, widths[1:])), "collapsing cover rebounds"
    assert not collapsed_ink, "replacement appears before typing begins"
    assert span(small_caret) < span(full_cover) * 0.08, "cover does not collapse to a caret"

    positions, prefixes = [], []
    # ASCII fixtures have one shaped cluster per character. Compare neighboring
    # rendered frames inside each beat: reveals must be discrete, not a text wipe.
    for count in range(1, len(replacement) + 1):
        if count < len(replacement):
            step_start = TYPE_START + TYPING * count / len(replacement)
            step_end = TYPE_START + TYPING * (count + 1) / len(replacement)
            first_frame = math.floor(step_start * FPS + 1e-6) + 1
            at = (first_frame + 1) / FPS
            assert at < step_end, "fixture needs two native frames per typing beat"
            stable, _ = masks(first_frame / FPS)
        else:
            at = TYPE_END + 2 / FPS
            stable, _ = masks(at - 1 / FPS)
        ink, caret = masks(at)
        positions.append(bounds(caret)[0])
        prefixes.append(ink)
        if not replacement[count - 1].isspace():
            gap = bounds(caret)[0] - bounds(ink)[1]
            assert 0 <= gap < height * 0.06, "caret does not follow the measured visible prefix"
        assert len(ink ^ stable) < max(20, len(ink) * 0.04), "typing is not a discrete cluster reveal"
    assert all(b > a for a, b in zip(positions, positions[1:])), "caret does not advance with each cluster"
    assert len({len(mask) // 20 for mask in prefixes}) >= len(replacement) - 2, "replacement is not typed progressively"
    if replacement == "Wi.M!":
        steps = [b - a for a, b in zip(positions, positions[1:])]
        assert max(steps) > min(steps) * 1.7, "caret uses uniform character widths instead of shaped advances"

    held, off_caret = masks(TYPE_END + 0.4)
    _, on_caret = masks(TYPE_END + 0.1)
    ready, ready_caret = masks(RETURN_START - 1 / FPS)
    same_foreground(held, prefixes[-1], "replacement moves during its readable hold")
    same_foreground(held, ready, "replacement changes before the reverse cover")
    assert on_caret and not off_caret, "hold blink is not anchored to completed typing"
    assert ready_caret, "caret disappears immediately before the reverse cover"
    reverse_ink, reverse_cover = masks(RETURN_START + WIPE * 0.65)
    assert len(reverse_ink) < len(held) * 0.8, "reverse cover does not wipe the replacement"
    assert span(reverse_cover) > span(ready_caret) * 4, "reverse cover never expands"
    reverse_covered, _ = masks((math.floor(RETURN_COVERED * FPS - 1e-6)) / FPS)
    assert len(reverse_covered) < len(held) * 0.01, "replacement leaks through the full reverse cover"
    revealing, _ = masks(RETURN_COVERED + RETURN * 0.2)
    assert 0 < len(revealing) < len(source) * 0.95, "source does not reveal behind the departing cover"
    before_loop, end_caret = masks(CYCLE - 1 / FPS)
    reveal_edges = [bounds(masks(RETURN_COVERED + RETURN * fraction / 8)[1]) for fraction in range(8)]
    assert all(b[0] >= a[0] - 2 and b[1] - b[0] <= a[1] - a[0] + 2
               for a, b in zip(reveal_edges, reveal_edges[1:])), "return reveal rebounds"
    assert all(edge[0] <= bounds(end_caret)[0] + 2 for edge in reveal_edges), "return caret overshoots"
    loop_start, loop_caret = masks(CYCLE)
    returned, _ = masks(CYCLE + 0.1)
    same_foreground(source, before_loop, "source is not restored before the loop boundary")
    same_foreground(source, loop_start, "loop boundary shifts the source")
    same_foreground(source, returned, "reverse wipe does not restore source glyphs")
    same_foreground(source_caret, end_caret, "return caret does not settle at the source's measured end")
    same_foreground(source_caret, loop_caret, "caret pops at the loop boundary")
    for visible in (source, held, full_cover):
        left, right, top, bottom = bounds(visible)
        assert width * 0.04 < left < right < width * 0.96, "single-line typography clips horizontally"
        assert height * 0.3 < top < bottom < height * 0.7, "typography is not vertically centered"
    print(f"PASS {width}x{height}: monotonic bounded wipe/collapse/reveal, measured discrete typing, settled return loop")


if __name__ == "__main__":
    with tempfile.TemporaryDirectory(prefix="mont-caret-swap-") as temporary:
        root = Path(temporary)
        verify(root, 960, 540, {}, "Move Ideas.")
        verify(root, 540, 960, {"sourceText": "Make room.", "replacementText": "Wi.M!", "ink": "#F6E6CE"}, "Wi.M!")
