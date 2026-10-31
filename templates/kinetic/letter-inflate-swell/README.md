# Letter inflate swell

Pale-mint lettering becomes an airy sculpture above a quiet copper glow on ink blue. A rounded bulge travels through each letter's contours. It uses **one face throughout**—not a font crossfade, weight swap, or uniform word-scale animation. There are no corner labels or brand furniture.

```sh
mont templates preview letter-inflate-swell
```

Save editable inputs as `settings.json`:

```json
{"text":"breathe","width":1080,"height":1920,"bulge":0.85,"cycleMs":6000}
```

```sh
mont templates preview letter-inflate-swell --inputs settings.json
```

Defaults: **960 × 540**, **24 fps**, **12 seconds**; `text: "aerate"`, `background: "#142A3A"`, `mint: "#DAF3DE"`, `copper: "#CA8968"`, `bulge: 0.85`, `cycleMs: 6000`, `phase: 0` (cycle fraction). `bulge: 0` gives undeformed lettering.

Native clipped text bands deform each glyph around its own center. Measured advances keep letter order and word spacing while the lettering inflates. Bands follow output resolution; no browser runtime or unsupported warp prop is used.

Use 1920 × 1080 for 16:9, 1080 × 1920 for 9:16, or 1080 × 1080 for square. Short and long copy fit within the canvas; very long copy necessarily renders smaller. Space Grotesk 700 is bundled; fallback glyphs may need additional catalog fonts.

Review contours at **0 ms** (rest), **1500 ms** (traveling crest), **3000 ms** (maximum breath), and **4500 ms** (release). The default rest/reset is **6000 ms**. [scene.tsx](scene.tsx) is the sole source for inputs, defaults, and native deformation.
