# Morph text

Five bold phrases melt into one another on warm paper. Native text layers blur and overlap; a generic WGSL shader thresholds their combined alpha into crisp, merging letterforms—not a simple crossfade. No browser runtime or template-specific renderer element is required.

```sh
mont templates preview morph-text
```

Save editable inputs as `settings.json`:

```json
{
  "phrase1": "Make something.",
  "phrase2": "Worth sharing.",
  "phrase3": "Find your pace.",
  "phrase4": "Keep it simple.",
  "phrase5": "Start today.",
  "background": "#152B38",
  "ink": "#E9DBB9",
  "fontSize": 104,
  "holdMs": 1500,
  "morphMs": 1000,
  "width": 1080,
  "height": 1080
}
```

```sh
mont templates preview morph-text --inputs settings.json
```

Defaults: **960 × 540**, **12.5 seconds**. Each phrase holds for **1500 ms**, then morphs for **1000 ms**; the fifth morph returns to the first phrase. `holdMs` is clamped to zero and `morphMs` to at least 200 ms. Changing timing changes the loop period, not the scene duration; adjust the span duration in the editor to include more cycles.

`fontSize` is relative to the default canvas. Measured Space Grotesk 700 text fits within 88% of the width, including portrait output; longer phrases shrink rather than clip. The gallery GIF uses a square canvas. Keep phrases short for the clearest morphs; fallback glyphs may need additional catalog fonts.

Review **750 ms** (first phrase), **1750–2250 ms** (morph), **2750 ms** (second phrase), and **11750–12250 ms** (return morph). [scene.tsx](scene.tsx) contains all editable copy, layout, timing, and shader code.

Visual inspiration: [Hyperframes Morph Text](https://hyperframes.heygen.com/catalog/components/morph-text). This version is authored in Mont scene syntax with original copy and styling.
