# Creator Follow Card

Native Mont programmatic scene, **1920 × 1080**, **4.5 seconds**, 30 fps preview. Only the final profile/Follow interaction from Blue Sweater Intro Video, beginning at source **9.76 s**. The 800 × 430 warm card enters with an original generated abstract hero and generated ringed portrait. The hero has rounded top corners and square lower corners. Mont’s canonical black-and-white arrow clicks Follow; the button presses, recovers, reveals Following/check and emits a restrained native ring/nine-dot burst. The unrelated intro/chat narrative is excluded.

Fictional demo identity: **Elias Reed · @eliasframes**, consistent with the X follow card. The portrait and abstract hero are model-generated for these templates, not stock photography or a claim about a real person. The original card geometry, text sizes/tracking and entrance are retained; the check remains native editable geometry. The cursor embeds the exact Mont editor arrow asset, not an upstream cursor or a redrawn approximation, and follows an authored cubic Bézier with quintic easing instead of separate straight-line legs. Source **Helvetica Neue 620/680 → native Inter 600/700**, and **Helvetica Neue 400 → native Inter 400**; exact Helvetica Neue is unavailable in the installed native font set. All text remains editable; no scene/text rasterization, DOM, CSS, GSAP, audio or engine changes.

## Edit

Use `creator`, `handle`, `avatarSrc`, `heroSrc`, `bio`, action/confirmed labels, colors, `entryAtMs`, `pressAtMs`, `cursorSrc`, and `cursorVisible` (0 hides the pointer). `width`/`height` scale uniformly with centered letterboxing. Keep labels and bio in the supplied fixed boxes. Timing values are milliseconds within the fixed duration; retain room for the curved approach, press/release and hold. The 80 × 80 cursor keeps the pack’s large-arrow hotspot (8, 6 in its 48 × 48 logical canvas) aligned with the motion anchor. This is an illustrative animation, not a live Follow action.

## Timing

0.10–0.70 s card entrance; 0.20–1.02 s continuous curved pointer approach; 1.02–1.12 s hover on Follow; 1.12–1.20 s press to 95%; 1.20–1.36 s recovery/state reveal without repeated wobble; 1.20–1.48 s expanding ring; 1.20–1.68 s deterministic nine-dot burst. After recovery, the pointer follows a small curved settle at 1.36–1.82 s. Both movements use quintic easing with zero endpoint velocity and acceleration; no abrupt corner, rebound or mid-approach stop. `cursorX`/`cursorY` declare the Bézier control points; approach timing follows `entryAtMs`/`pressAtMs`, and settling follows `pressAtMs`. Fixed particle directions/speed offsets and arbitrary seeking remain deterministic. The standalone extends the readable final hold.

The card and pointer never freeze after settling. Independent slow arcs fade in smoothly during recovery: the card drifts by at most 5 px horizontally / 4 px vertically, the cursor by 6 px / 4.5 px. `cardDrift*` and `cursorDrift*` control these deterministic residual movements without changing the click timing.

## Preview and checks

From this repository, run `npx --yes --package=usemont -- python3 templates/subscribe/verify.py --scene creator-follow-card --output-root /absolute/path/to/native-repo/.tmp/template-previews/creator-follow-card`. It creates a resource-bound native workspace, renders the MP4, checks dimensions/duration, button press/state/particles and bounded late-frame card/cursor drift, and saves sRGB-managed frames and a contact sheet. Requires an embedded-image-capable `mont` build, `ffmpeg`, `ffprobe` and Pillow. The output directory must not already exist.

To create an editable workspace without rendering, add `--create-only` to that command, then use `npx usemont render -C /absolute/path/to/native-repo/.tmp/template-previews/creator-follow-card/creator-follow-card/workspace --output /absolute/path/to/creator-follow-card.mp4 --fps 30 --json`. The helper explicitly registers each embedded image as `project.resources[src] = {uri: src}`; register replacement `avatarSrc`, `heroSrc` or `cursorSrc` likewise. Off-canvas native images preload the portrait and hero before the initially invisible entrance. Fonts use native Inter; check and particles remain native shapes.

Keep `borderWidth={0} borderColor="transparent"` on native `Image` elements to avoid the installed renderer’s default rectangular image border.

## Source and license

Adapted from [Blue Sweater Intro Video](https://hyperframes.heygen.com/catalog/blocks/blue-sweater-intro-video); [upstream source](https://github.com/heygen-com/hyperframes/tree/main/registry/blocks/blue-sweater-intro-video). Hyperframes copyright 2026 HeyGen, Inc.; Apache-2.0 (copy in [`../LICENSE`](../LICENSE)). The upstream credits Joe Sai; his name/likeness are not used. Derivative native layout/motion retains this attribution. No upstream image, logo, portrait, cursor PNG, CSS-gradient PNG or audio is redistributed.

Portrait provenance: `assets/template-media/people/elias-reed.png`, generated for Mont’s fictional demo cast, circularly cropped/resized to 216 × 216 for embedding. The same fictional Elias portrait appears in the X follow card. Hero provenance: `assets/template-media/backgrounds/creator-abstract-hero.png`, generated using Gemini 3.1 Flash Image, center-cropped to 1596 × 300 and embedded with transparent 60 px top corners only (30 scene units); both lower corners remain square and fully opaque. Preserve this top-only alpha crop when replacing `heroSrc`. The check and deterministic particles are independently authored native primitives. Cursor provenance: `slides-recorder-monorepo/projects/mont-sdk/flutter/assets/mont-cursors/png/large-2x/cursor-arrow.png` (96 × 96), embedded without modification; SHA-256 `b6fb12e393a1550b8f22ffb7c9f7ced90cfcb66f024b493a09fef424e0409239`.

Asset origin/model/prompt/hash records: [`provenance.json`](provenance.json). Native verification also writes a per-scene `asset-inventory.json` for every embedded image variable.
