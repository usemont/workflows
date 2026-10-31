# Letter inflate swell

Staggered letters inflate into rounded, over-canvas shapes.

![Letter inflate swell — native Mont render](../../../assets/previews/letter-inflate-swell-square.gif)

## Make it yours

From the repository root, save this as `settings.json`:

```json
{"text":"BLOOM","width":1080,"height":1920}
```

```sh
mkdir -p out/letter-inflate-swell-custom
mont -C "$PWD/out/letter-inflate-swell-custom" workflow run \
  "$PWD/templates/kinetic/letter-inflate-swell/template.mont" \
  --inputs "$PWD/settings.json" --run-id custom
```

| Format | Width × height |
| --- | --- |
| Landscape · 16:9 | 1920 × 1080 |
| Portrait · 9:16 | 1080 × 1920 |
| Square · 1:1 | 1080 × 1080 |

`background`, `pink`, and `glow` set the palette. Uses Titan One; letterforms differ from the reference’s custom font. See [template.mont](template.mont) for all inputs; edit [scene.tsx](scene.tsx) to change the animation.

Requires [Mont CLI](https://github.com/usemont/mont-cli#install) 0.1.0-preview.5+. No inference key. Outputs `preview.mp4` and editable `project/`. First use of a non-embedded font needs network access. Use a new output directory and run ID for each new generation.
