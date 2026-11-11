# X Follower Card

Native Mont scene, **1920 × 1080**, **5 seconds**, 30 fps preview. Editable 768 × 512 dark X-style profile card with a generated photographic cover, overlapping avatar, bio/count, six follower portraits, and a white Follow button. Includes a back-eased count, staggered follower reveal, and button press/state/ring/particle interaction.

Fictional demo profile: **Elias Reed · @eliasframes · 18.6K followers** (illustrative metrics). The avatar and follower portraits are model-generated fictional people. **Inter 400/700** text remains editable.

## Edit

Use `creator`, `handle`, `stat`, `bio`, `avatarSrc`, `heroSrc`, follower-image variables, action/confirmed labels, colors, `revealAtMs` and `pressAtMs`. `width`/`height` scale uniformly with centered letterboxing. Keep copy inside the supplied text boxes. Timing values are milliseconds in the fixed duration; `pressAtMs` synchronizes every button effect.

## Timing

0.50–0.96 s follower count reveal; 0.72–1.25 s staggered follower portraits; 0.35 s reading beat after the final portrait settles. Button: 1.60–1.68 s press to 95%; 1.68–1.84 s recovery/state reveal without repeated wobble; 1.68–1.96 s expanding ring; 1.68–2.16 s nine-dot burst; final hold through 5.00 s. Fixed angles/speed offsets and time-derived positions/alpha make arbitrary seeking deterministic.

## Preview and checks

From this repository, run `npx --yes --package=usemont -- python3 templates/subscribe/verify.py --output-root /absolute/path/to/native-repo/.tmp/template-previews/subscribe`. It creates resource-bound native workspaces, renders all five MP4s, checks dimensions/durations, Mont branding, photographic X cover/avatar, press/state/particle/hold changes and count/follower reveals, and saves sRGB-managed peak-burst/settled PNGs and contact sheets. Requires `mont`, `ffmpeg`, `ffprobe` and Pillow. The output directory must not already exist.

To create editable workspaces without rendering, use `npx --yes --package=usemont -- python3 templates/subscribe/verify.py --create-only --output-root /absolute/path/to/native-repo/.tmp/subscribe-workspaces`, then `npx usemont render -C /absolute/path/to/native-repo/.tmp/subscribe-workspaces/NAME/workspace --output /absolute/path/to/NAME.mp4 --fps 30 --json`. The helper explicitly registers every embedded image as `project.resources[src] = {uri: src}`; register replacement image variables likewise. Off-canvas native images preload portraits and the cover. Keep `borderWidth={0} borderColor="transparent"` on each native image; typography and all interaction shapes stay native.

## Source and license

Adapted from [X Follower Card](https://hyperframes.heygen.com/catalog/components/x-follow-card); [upstream source](https://github.com/heygen-com/hyperframes/tree/main/registry/components/x-follow-card). Hyperframes copyright 2026 HeyGen, Inc.; Apache-2.0 (copy in [`../LICENSE`](../LICENSE)). Changes: original fictional profile/copy, native editable photographic layout, retained back-eased count/staggered reveal, and independently authored deterministic button interaction. No upstream logo/photos/icon payloads remain. Platform names describe reference UI, not endorsement.

Generated images: `assets/template-media/backgrounds/x-profile-hero.png` and `assets/template-media/people/{elias-reed,maya-chen,theo-park,nia-brooks,sora-kim,jules-rivera,ava-patel}.png`. Images are cropped/downscaled for embedding. Elias is shared with the creator follow card.

Canonical asset origin/model/prompt/hash records: [`assets/template-media/provenance.json`](../../../assets/template-media/provenance.json). Native verification also writes a per-scene `asset-inventory.json` for every embedded image variable.
