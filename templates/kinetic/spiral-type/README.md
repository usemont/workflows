# Spiral type

An off-axis luminous text orbit hangs in midnight blue. Large **DM Serif Display** glyphs follow a finite, tilted Archimedean spiral; a compact **Press Start 2P** monospaced inner orbit turns against it. The sparse composition leaves deliberate negative space rather than covering every corner with a repeating pattern. Both faces are bundled.

```sh
mont templates preview spiral-type
```

Save editable inputs as `settings.json`:

```json
{"text":"FOLLOW YOUR CURIOSITY","orbitText":"TAKE THE LONG WAY","width":1080,"height":1920,"speed":0.8}
```

```sh
mont templates preview spiral-type --inputs settings.json
```

Defaults: **960 × 540**, **24 fps**, **9 seconds**; `text: "FOLLOW A DIFFERENT GRAVITY"`, `orbitText: "CHANGE YOUR ORBIT"`, `background: "#101A30"`, `ink: "#DDEAFF"`, `light: "#B1F6CF"`, `speed: 1`, `innerRadius: 62`, `pitch: 80`. Radius and pitch are reference pixels at a 540-pixel short edge and scale with the canvas. Negative `speed` reverses motion; zero freezes it.

Use 1920 × 1080 for 16:9, 1080 × 1920 for 9:16, or 1080 × 1080 for square. Main text repeats by measured advance, including spaces and Unicode clusters; the smaller orbit fits its copy around one revolution. Replacement glyphs outside the bundled faces may need additional catalog fonts.

Native scene only. Rotation follows the mathematical tangent, with soft entry/exit along the finite spiral. Review **0, 2200, 4500, and 7600 ms** for drift, changing tilt, and the counter-rotating inner text. The 9-second composition is deterministic under arbitrary seeking, not a seamless loop. See [template.mont](template.mont) and [scene.tsx](scene.tsx).
