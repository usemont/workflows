# Stretch row repeater

Rows stretch independently and travel in opposite directions.

![Stretch row repeater — native Mont render](../../../assets/previews/stretch-row-repeater-square.gif)

## Try it

```sh
# install (macOS)
brew install usemont/tap/mont
# generate preview
mont stretch-row-repeater
```

On Linux, use the [signed CLI installation guide](https://github.com/usemont/mont-cli/blob/main/docs/install.md#linux).

Outputs: `out/stretch-row-repeater/preview.mp4` and editable `out/stretch-row-repeater/project/`. Reruns create `out/stretch-row-repeater-2/`, then `-3/`, without overwriting previous output.

## Make it yours

Save this as `settings.json` in your current directory:

```json
{"text":"MAKE SPACE","width":1080,"height":1920}
```

```sh
mont stretch-row-repeater --inputs settings.json
```

| Format | Width × height |
| --- | --- |
| Landscape · 16:9 | 1920 × 1080 |
| Portrait · 9:16 | 1080 × 1920 |
| Square · 1:1 | 1080 × 1080 |

`alternateText` changes the alternating rows; `crimson` and `cream` set the palette. See [template.mont](template.mont) for all inputs; edit [scene.tsx](scene.tsx) to change the animation.

No inference API key needed. First use of a non-embedded font needs network access.
