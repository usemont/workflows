# Letter inflate swell

Staggered letters inflate into rounded, over-canvas shapes.

![Letter inflate swell — native Mont render](../../../assets/previews/letter-inflate-swell-square.gif)

## Try it

```sh
# install (macOS)
brew install usemont/tap/mont
# generate preview
mont letter-inflate-swell
```

On Linux, use the [signed CLI installation guide](https://github.com/usemont/mont-cli/blob/main/docs/install.md#linux).

Outputs: `out/letter-inflate-swell/preview.mp4` and editable `out/letter-inflate-swell/project/`. Reruns create `out/letter-inflate-swell-2/`, then `-3/`, without overwriting previous output.

## Make it yours

Save this as `settings.json` in your current directory:

```json
{"text":"BLOOM","width":1080,"height":1920}
```

```sh
mont letter-inflate-swell --inputs settings.json
```

| Format | Width × height |
| --- | --- |
| Landscape · 16:9 | 1920 × 1080 |
| Portrait · 9:16 | 1080 × 1920 |
| Square · 1:1 | 1080 × 1080 |

`background`, `pink`, and `glow` set the palette. Uses Titan One. See [template.mont](template.mont) for all inputs; edit [scene.tsx](scene.tsx) to change the animation.

No inference API key needed. First use of a non-embedded font needs network access.
