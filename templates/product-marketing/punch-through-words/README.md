# Punch-through words

Square, 3.4-second continuation of **Subscription pile-up**. Three editable white words stamp onto a widening constellation of video-tool subscriptions. Prices are illustrative, not Mont pricing.

```sh
npx usemont templates preview punch-through-words
```

Three word impacts pull the camera back in discrete steps. New cards spring out from their own centers in staggered waves, overshoot, and retain independent wobble; they are not an already-present offscreen field. Camera velocity drives directional motion blur; separate Gaussian defocus increases on each beat. Background shake grows on successive impacts (1×, 1.45×, 1.9×), while the words stay stable. An edge vignette and soft dark shading sit beneath the words.

The expanded field continues the original staggered spacing, with bridging cards between the opening cluster and the viewport edges rather than a separate outer ring. Each new card enters on its reveal beat.

Words approach for 160 ms from 145% scale, slight defocus and 32% opacity, landing sharp and solid exactly on the camera, card-reveal and shake cue. The camera recedes over 180 ms with a stronger, fast-decaying impact. Each entrance starts after the previous word clears; there is no rebound. Cards retain their original sizes and progressively blur with the camera pullbacks.

Edit `firstWord`, `secondWord`, and `thirdWord` for copy; `firstImpactMs` (the landing time, minimum 160 ms) and `beatMs` for timing. `cardBlur`, `shutterMs`, `shakePx`, `vignette`, and `wordShade` control the effects. Card copy, centers, tilts, colors, and float phases live in `tokens.cards`. The first eight cards preserve the approved starting layout; new cards use `enterBeat` (1–3) and `delayMs` to time their spring entrances relative to the word impacts. All artwork is inline SVG and native scene primitives; no external assets or custom renderer code.

Render and verify with the installed Mont CLI, FFmpeg, and Python 3:

```sh
npx --yes --package=usemont -- python3 templates/product-marketing/punch-through-words/verify.py /tmp/punch-through-words.mp4
```

The check binds explicit palette values and verifies square output, sequential word beats, sharp stamp contact synchronized with background recession, three camera pullbacks with stationary intervals, coverage between the original and expanded field, progressive defocus, and a newly revealed card entrance that grows, overshoots, and settles.
