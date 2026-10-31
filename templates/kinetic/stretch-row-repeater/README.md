# Stretch row repeater

Three asymmetric editorial ribbons cross a charcoal field. Space Grotesk lettering moves in opposing directions while a measured, continuous wave compresses and releases individual glyph cells. The cut bands, changing angles, and open negative space are part of the composition—not a full-frame text wallpaper.

```sh
mont templates preview stretch-row-repeater
```

Save editable inputs as `settings.json`:

```json
{"text":"MAKE ROOM","alternateText":"FOR THE UNEXPECTED","width":1080,"height":1920,"stretch":0.48}
```

```sh
mont templates preview stretch-row-repeater --inputs settings.json
```

Defaults: **960 × 540**, **24 fps**, **12 seconds**; `text: "MAKE ROOM"`, `alternateText: "FOR THE UNEXPECTED"`, `background: "#171B22"`, `cobalt: "#3857FF"`, `lemon: "#E7F478"`, `stretch: 0.48`, `cycleMs: 12000`, `phase: 0` (radians). `stretch: 0` disables the compression wave without stopping the ribbon travel.

Use 1920 × 1080 for 16:9, 1080 × 1920 for 9:16, or 1080 × 1080 for square. Ribbon dimensions respond to the canvas; measured advances preserve spaces and Unicode clusters with both short and long copy. Space Grotesk 700 is bundled; fallback glyphs may need additional catalog fonts.

Native scene only: no HTML or browser animation runtime. Review motion at **0, 1800, 4200, and 7800 ms**; the default cycle restarts at **12000 ms**. All animation is a deterministic function of scene time. Inputs and bindings live in [template.mont](template.mont); composition lives in [scene.tsx](scene.tsx).
