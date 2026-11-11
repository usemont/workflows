# Card vortex

The video-editing tool cards from **Punch-through words** drain through a clockwise vortex, then resolve into a light point and expanding pulse. Inner cards overtake outer cards; each card banks and follows its own streamline.

**960 × 960 · 3 seconds · 30 fps.** Editable Mont primitives and inline SVG lighting.

## Edit

- `tokens.cards`: original labels, prices, center offsets, sizes and colors. `delay` staggers release, `arc` varies circulation, and `pitch`/`yaw` vary the banking flutter. Traits are fixed literals sampled from the recorded seed, never resampled during playback.
- `holdMs`, `collapseMs`, `staggerMs`: opening hold, field-drain duration and release spread. Defaults: 550, 1700 and 110 ms. The light handoff follows their sum.
- `circulation`: clockwise flow strength, default 300 px²/ms. Outside the core, tangential speed rises as `1/r` and angular speed as `1/r²`; `coreRadius` (70 px) caps angular speed and softens the final capture.
- `flowRampMs` (150 ms) eases the flow in. `alignLagMs` (55 ms) lets card headings trail their velocity; banking increases toward the core. Local perspective is authored at 900 px; edit its literal in the card group rather than binding a runtime input, which native retained lowering does not support.
- `cardBlur`, `defocus`: opening softness and additional collapse defocus. `shutterMs` controls directional blur from each card's actual curved velocity.
- `vignette`, `flash`: edge darkening and light-pulse strength. The settled light point remains for the next scene's handoff.
- `background`, `surface`, `border`, `ink`, `muted`, `accent`: inherited card palette.
- `width`, `height`: uniformly fitted composition. `speed` scales time, clamped to 0.25–2; adjust the exported span duration accordingly.

Card origins stay centered. Cards retain most of their size until entering the core. Far-to-near ordering is stable as cards bank; all float and flutter are deterministic. The shared sink uses the authored field's squared reach in `computed.drain`; update that reach when enlarging the card layout.

The logarithm in the spiral integral uses a portable polynomial because source-only lowering lacks `Math.log`. Relative error stays below 0.1% for the authored radius/core ratios (1–950).

Check differential angular speed, inward acceleration, capped core speed and heading alignment, then render and check the visible convergence, cleared edges and light pulse. Requires Python 3, Node.js, `mont`, `ffmpeg` and `ffprobe`:

```sh
npx --yes --package=usemont -- python3 templates/product-marketing/card-vortex/verify.py /tmp/card-vortex.mp4
```
