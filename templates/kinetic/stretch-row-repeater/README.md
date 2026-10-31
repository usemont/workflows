# Stretch row repeater

Rows stretch independently and travel in opposite directions.

![Stretch row repeater — native Mont render](../../../assets/previews/stretch-row-repeater-square.gif)

## Make it yours

From the repository root, save this as `settings.json`:

```json
{"text":"MAKE SPACE","width":1080,"height":1920}
```

```sh
mkdir -p out/stretch-row-repeater-custom
mont -C "$PWD/out/stretch-row-repeater-custom" workflow run \
  "$PWD/templates/kinetic/stretch-row-repeater/template.mont" \
  --inputs "$PWD/settings.json" --run-id custom
```

| Format | Width × height |
| --- | --- |
| Landscape · 16:9 | 1920 × 1080 |
| Portrait · 9:16 | 1080 × 1920 |
| Square · 1:1 | 1080 × 1080 |

`alternateText` changes the alternating rows; `crimson` and `cream` set the palette. See [template.mont](template.mont) for all inputs; edit [scene.tsx](scene.tsx) to change the animation.

Requires [Mont CLI](https://github.com/usemont/mont-cli#install) 0.1.0-preview.5+. No inference key. Outputs `preview.mp4` and editable `project/`. First use of a non-embedded font needs network access. Use a new output directory and run ID for each new generation.
