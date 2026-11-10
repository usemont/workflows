# Subscription pile-up

Original square Mont scene: eight video-production tool subscriptions pop from their own centers into an asymmetric constellation, then keep gently wobbling. Covers screen recording, video editing, captions, motion graphics, audio cleanup, AI voiceover, color grading, and export. Prices are illustrative, not Mont pricing. Cards only—no heading or total.

```sh
mont templates preview subscription-pile-up
```

## Verification

```sh
python3 templates/product-marketing/subscription-pile-up/verify.py --output /tmp/subscription-pile-up.mp4
```

Requires `mont`, Python 3, `ffmpeg`, and `ffprobe`. The command checks staggered arrivals, centered spring overshoot, continued motion after settling, and clearance between padded card silhouettes throughout the rendered animation (excluding the first 120 ms of each fade-in). To check an existing default render, pass `--video /path/to/video.mp4` instead.

## Editing

- `variables`: palette and playback speed (0.25–2).
- `tokens.cards`: original names, badge letters, monthly prices, arrival times, center positions, rotations, scales, wobble phases, and periods in milliseconds.
- Centers were repacked offline against rotated footprints sampled at 60 Hz over the default animation, including overshoot and wobble, with a 44 px minimum clearance and 28 px canvas inset. The scene keeps literal poses: no per-frame collision solver. Re-run verification after changing poses or motion.
- `pop`: underdamped spring; `8` controls decay and `13` controls bounce frequency. Card opacity enters over 180 ms.
- `sway` / `floatIn`: continuous translation and rotation, gently introduced after each cue. Position amplitudes are 5/7 px; rotation amplitude is 2.2°.
- `tokens.cardLighting`: inline SVG diagonal light-to-dark wash and edge highlight over `variables.surface`. Badges have a localized colored glow. No downloaded reference assets.

Default composition: 960 × 960, 5.5 seconds, eight cards. Geometry scales together inside other canvas dimensions. The recorded seed phrase is `subscription-pile-up / loose constellation / v2`; variations are emitted as stable literals, never sampled per frame. No external images, fonts, logos, reference audio, custom renderer elements, or plugins are bundled.
