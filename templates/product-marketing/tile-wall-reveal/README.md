# Tile wall reveal

Preview with `mont templates preview tile-wall-reveal`, or create an editable workspace with `mont templates create tile-wall-reveal ./tile-wall`.

A close view of an original video-editing panel pulls back to four mini-tools, then lands in an isometric wall that keeps panning along its own vertical axis. No headline overlay. Square 960 × 960, 6 seconds, silent.

## Editing

- `tokens.tiles`: one contiguous 7 × 11 grid, 420 × 300 panels, 18 px gutters. Guard rows cover the moving viewport. `column` and `row` determine placement; `kind` selects Edit, Captions, Record or Motion. Native text fields hold each tool name and project caption.
- `tokens.editArt`, `captionArt`, `recordArt`, `motionArt`: original inline SVG illustrations, not screenshots or borrowed product assets. Header labels, captions, timer and motion copy remain editable native `Text` primitives.
- `holdMs`, `pullbackMs`: initial hold and camera pullback. Quintic easing has zero velocity and acceleration at both ends; the pullback never rebounds.
- `tiltMs`: isometric landing duration, starting together with the pullback after the initial hold. A shared −45° leftward rotation followed by vertical compression to 1/√3 gives equal axis foreshortening and 30° diagonals, without perspective shrinkage.
- `panSpeed`: wall-local vertical travel in pixels per second, clamped to 0–120. It eases into constant motion during the landing; the pan inherits the wall projection rather than moving vertically across the screen.
- `background`, `width`, `height`: surrounding surface and uniform square-stage fit. `speed` scales the clock, clamped to 0.25–2; change the exported span duration when changing speed.

All tiles inherit one transform, keeping gutters and relative positions continuous. The finite guard grid covers the default six-second scene; extend it for longer travel. There is no per-frame randomness, external media, human likeness or dependency on reference assets. The mini-tools are illustrative, not claims about exact editor UI.

## Verify / render

```sh
python3 verify.py /absolute/path/to/new-output.mp4
```

Requires `mont`, Node, FFmpeg and FFprobe. The check exercises camera easing, isometric projection and pan continuity, renders all explicit inputs through the native SDK, and verifies square frame count, the closeup-to-wall visual change and continued motion after landing. It uses an isolated temporary workspace and refuses to overwrite an existing render.

Visual inspiration: the tiled-surface reveal in [Simon Høiberg’s reference](https://x.com/SimonHoiberg/status/2103916672080097521), around 6.65–8.0 seconds. Artwork, copy and scene implementation are original Mont work.
