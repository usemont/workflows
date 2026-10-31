# Spiral type

A centered, full-frame spiral of **DM Serif Display** lettering in ice blue on midnight navy. Every glyph follows the same circular spiral with constant turn spacing and a uniform type size; the text drifts along the path at a measured pace.

```sh
mont templates preview spiral-type
```

Save editable inputs as `settings.json`:

```json
{"text":"FOLLOW YOUR CURIOSITY","width":1080,"height":1920,"speed":0.8}
```

```sh
mont templates preview spiral-type --inputs settings.json
```

Defaults: **960 × 540**, **24 fps**, **9 seconds**; `text: "FOLLOW A DIFFERENT GRAVITY"`, `background: "#101A30"`, `ink: "#DDEAFF"`, `speed: 0.85`, `innerRadius: 62`, `pitch: 46`. Radius and pitch are reference pixels at a 540-pixel short edge and scale with the canvas. Negative `speed` reverses motion; zero freezes it.

Use 1920 × 1080 for 16:9, 1080 × 1920 for 9:16, or 1080 × 1080 for square. The spiral fills the frame, including the corners. Text repeats by measured advance, including spaces and Unicode clusters. DM Serif Display 400 is bundled; replacement glyphs may need additional catalog fonts.

Native scene only. Rotation follows the mathematical tangent, with soft entry/exit along the finite spiral. Review **0, 2200, 4500, and 7600 ms** for uniform shape and continuous travel. The 9-second composition is deterministic under arbitrary seeking, not a seamless loop. [scene.tsx](scene.tsx) is the sole source for inputs, defaults, and animation.
