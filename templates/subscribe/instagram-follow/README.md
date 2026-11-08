# Instagram Follow

Native Mont programmatic scene, **1080 × 1920**, **4.5 seconds**, 30 fps preview. Source-sized dark pill, actual Mont icon and blue Follow button. No invented verification badge. Button-local press/recover, editable success label/check or chevron, restrained nine-particle burst and expanding ring; the card exits downward.

Demonstration channel: **Mont · @usemont · 17.3K followers** (illustrative metrics). The avatar embeds the genuine `assets/template-media/mont-icon.png`, derived from Mont’s own app icon, not a third-party logo. Native **DM Sans 400/700** text uses explicit SHA-256-addressed Mont font assets; initial rendering needs `fonts.usemont.com` or its local cache. Interaction marks are native editable paths, not upstream image bytes. No text rasterization, DOM, CSS, GSAP, audio or engine changes.

## Edit

Use `creator`, `handle`, `count`, `avatarSrc`, `profileWidth`, action/confirmed labels, colors, `entryAtMs`, `pressAtMs` and `exitAtMs`. `profileWidth` retains the corrected original pill/button geometry; update it only when intentionally reflowing the profile. `pressAtMs` drives the press, label swap, ring and particles together. Keep labels inside their supplied boxes. `backgroundOpacity = 0` removes the neutral preview background; MP4 previews use it opaque. `width`/`height` scale uniformly with centered letterboxing. Timing values are milliseconds inside the fixed duration. This is an illustrative animation, not a live platform action.

## Timing

0.10–0.60 s card entrance; 1.00–1.08 s press to 95%; 1.08–1.24 s smooth recovery/state reveal (no repeated wobble); 1.08–1.36 s expanding ring; 1.08–1.56 s deterministic nine-dot burst; 3.80–4.05 s exit. Particle angles/speed offsets are fixed, so seeking renders the same frame.

## Preview and checks

From this repository, run `python3 templates/subscribe/verify.py --output-root /absolute/path/to/native-repo/.tmp/template-previews/subscribe`. It creates resource-bound native workspaces, renders all five MP4s, checks dimensions/durations, visible Mont branding, press/state/particle/hold/exit changes, and saves sRGB-managed peak-burst and settled PNGs and contact sheets. Requires `mont`, `ffmpeg`, `ffprobe` and Pillow. The output directory must not already exist.

To create editable workspaces without rendering, use `python3 templates/subscribe/verify.py --create-only --output-root /absolute/path/to/native-repo/.tmp/subscribe-workspaces`, then `mont render -C /absolute/path/to/native-repo/.tmp/subscribe-workspaces/NAME/workspace --output /absolute/path/to/NAME.mp4 --fps 30 --json`. The helper explicitly registers each embedded image as `project.resources[src] = {uri: src}`; the catalog single-source loader does not register child images. Register any replacement `avatarSrc` likewise. An off-canvas native image preloads the Mont avatar before the initially invisible entrance. Fonts use their explicit `renderConfig.fontAsset` descriptors, not image resource entries.

Keep `borderWidth={0} borderColor="transparent"` on native `Image` elements. The installed renderer otherwise draws a thin rectangular edge from its default black border color even at zero border width; `stroke`/`strokeWidth` are not mapped for images.

## Source and license

Adapted from [Instagram Follow](https://hyperframes.heygen.com/catalog/blocks/instagram-follow); [upstream source](https://github.com/heygen-com/hyperframes/tree/main/registry/blocks/instagram-follow). Hyperframes copyright 2026 HeyGen, Inc.; Apache-2.0 (copy in [`../LICENSE`](../LICENSE)). Changes: original Mont identity/copy/assets, native editable layout/text/interaction paths, retained corrected proportions/font assets/depth/entrance/exit, and an independently authored deterministic button microinteraction. No upstream logo, portrait or icon image payload is redistributed. Platform names describe reference UI only; no endorsement or real account ownership is implied.

Canonical asset origin/model/prompt/hash records: [`assets/template-media/provenance.json`](../../../assets/template-media/provenance.json). Native verification also writes a per-scene `asset-inventory.json` for every embedded image variable.
