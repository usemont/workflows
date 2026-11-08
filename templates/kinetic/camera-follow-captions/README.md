# Camera Follow Captions

**MAKE EVERY FRAME FEEL MORE ALIVE.** builds a tightly packed typographic grid. Bold **Space Grotesk 700** follows the reference's sans-serif direction; gold lettering on midnight navy uses two colors total. No lines, dots, crosses, or corner labels.

## Use

Preview with `mont templates preview camera-follow-captions`, or load `scene.tsx` as a native `programmatic.scene` span. Set its frame and `width` / `height` variables to the canvas dimensions. Duration: **5000 ms**; edit preview: **3600 ms**.

- `word1`…`word6`: editable short uppercase words.
- `background`, `ink1`…`ink6`: editable colors. All six inks default to the same gold.
- Requires the native bundled **Space Grotesk 700** face; no intentional font fallback.

## Packing and camera

Font sizes grow by the golden ratio: **f₀ = 100; fₙ = fₙ₋₁ × 1.61803398875**. Native shaped advances measure each word. Compact 0.78em layout heights and 0.07em gaps replace oversized padded line boxes. Each addition goes right, baseline-aligned with the accumulated block, or below, left-aligned, according to block/canvas aspect ratios. Landscape and portrait therefore pack differently.

Words stay fixed in the world. One zero-pivot camera fits the accumulated bounds, including additions that extend above the old block. Each target scale is capped at 96% of its predecessor: the camera only pulls outward. Direct interpolation of fitted screen translations and scale keeps previously visible words framed throughout each move.

Each word is one complete Text node. Static native glyph records provide widths; fixed per-word Groups set relative sizes. There is no per-frame text reshaping or custom renderer code.

## Timing

The first word is visible at frame zero. Further cues begin at **520 / 1040 / 1560 / 2080 / 2600 ms**. Each cue starts the **160 ms word fade and 400 ms camera pull-back together**, matching the reference's ordering: the new word becomes solid while the camera is still moving, not after it lands. Both use cubic ease-out.

The final composition holds from **3000–4350 ms**, fades until **4650 ms**, then stays blank until the 5000 ms reset. The final landing is already the overview; there is no zoom back in.

## Check

With native `mont` and `ffmpeg` on PATH:

```sh
python3 templates/kinetic/camera-follow-captions/verify.py
```

The check renders disposable landscape and portrait projects. Diagnostic ink overrides identify actual word bounds. Assertions cover packed baseline/left-edge placement, accumulated framing, unrevealed future words, word visibility before camera landing, outward-only moves, a stable final hold, the blank reset interval, and golden-ratio cap heights. Portrait uses repeated H glyphs for comparable raster measurements.

Short uppercase copy is the layout contract, not arbitrary paragraphs or multilingual text. Review both aspect ratios with the intended words before publishing.
