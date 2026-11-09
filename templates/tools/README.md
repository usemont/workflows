# Recorder tools

Nine editable synthetic examples demonstrate Mont's recording layouts, themes, and cursor styles. These are SDK-composed recorder projects, not pre-rendered video templates. Camera-only is intentionally not published.

```sh
mont tools list
mont tools show corner-bubble
mont tools preview corner-bubble
```

Use the names in the [Tools gallery](../../README.md#tools) with `mont tools preview`. Requires a CLI build with the `tools` commands.

Each example includes a pinned `recipe.json` and self-contained `asset-catalog.json`. Recipes select the layout, theme, cursor, source geometry, and 1.2× cursor-follow zoom with 250 ms framing smoothing. Landscape examples use 16:9, Shorts stack uses 9:16, and Square post uses 1:1. Layouts do not animate independently of the authored interaction.

The SDK resolves recorder definitions from [usemont/programmatic-tools](https://github.com/usemont/programmatic-tools), pinned by commit and SHA-256 in the root catalog. Shared source media and rendered MP4/GIF previews are also SHA-256 pinned. Previewing materializes an editable project without requesting screen, microphone, or camera access.

## Synthetic sources

- [`recorder-demo/desktop.html`](recorder-demo/desktop.html): original fictional desktop, authored click targets, and UI events.
- [`recorder-demo/storyboard.json`](recorder-demo/storyboard.json): eight seconds of timed cursor samples, synchronized with those targets. Screen/cursor coordinates use a 1280×800 source canvas.
- [`desktop.mp4`](../../assets/template-media/recorders/desktop.mp4): that desktop rendered at 24 fps.
- [`presenter.mp4`](../../assets/template-media/recorders/presenter.mp4): original AI-generated fictional adult presenter, shared across camera layouts; [generation provenance](../../assets/template-media/recorders/provenance.json).
- [`silence.m4a`](../../assets/template-media/recorders/silence.m4a): silent microphone input for reproducible previews.

All examples are eight seconds at 24 fps. The presenter is generated, not a real person's recording or endorsement. These examples demonstrate composition, not live-capture acceptance.

[Apache-2.0](../../LICENSE)
