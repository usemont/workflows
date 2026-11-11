# Annotate cursor

Native Mont TSX, **1920 × 1080, 30 fps, 3.6 seconds**. A vector marker circles an illustrative first-pass approval metric in a Mont production note.

Preview: `npx usemont templates preview annotate-cursor`

## Timing

- **0–530 ms:** curved minimum-jerk entrance, nib hovering 24 px above contact.
- **530–700 ms:** touch down at **(836, 514)**.
- **700–2080 ms:** draw a smooth, imperfect ellipse beyond a full turn, ending near **(856, 477)** outside the initial arc without touching it.
- **2080–2480 ms:** lift and curve aside to **(1238, 742)**.
- **2480–3200 ms:** unobstructed final hold.
- **3200–3600 ms:** curved exit; the completed mark stays on the page.

Edit `title`, `subtitle`, `metricValue`, `metricLabel`, `ink`, and `background`. The default metrics are illustrative copy. `width` and `height` uniformly letterbox the scene.

## Geometry / review

The loop sweeps **2π + 0.3 radians** (about **377°**) with small deterministic asymmetry, sampled into **320 short chords** through `computed.points` / `path.fromPoints`. Its radius grows by `1 + 0.18 × progress⁶`, opening outward late in the gesture so the finishing tail overshoots the starting angle without connecting or crossing. The lift starts from the path endpoint.

The ink is a **filled `Path` ribbon**, offset along `path.normalAt` on either side of the centerline. Smoothstep ramps occupy the first **2.5%** and final **3.5%** of path progress; the body stays at **0.8 ± 0.035** pressure. The width formula `6.5 × (0.04 + 0.96 × pressure)` gives an approximately **5.0–5.5 px** body and **0.26 px** terminals.

The nib and fractional ink frontier share `path.pointAt(loopPath, drawEase)`; the two ribbon edges extend along its normal, keeping the center of the contact patch under the nib without smoothing or time-reveal lag. The marker's physical nib remains its rotation origin **(0, 0)**. Document bounds: **(420, 152)–(1500, 934)**.

Run `node templates/cursors/annotate-cursor/geometry-check.mjs` from the repository root to check the sweep, arc clearance, finishing crossing, chord error, and pressure tapers.

Adapted from [HeyGen Annotate Cursor](https://hyperframes.heygen.com/catalog/components/cursor-annotate.md), copyright 2026 HeyGen, Inc., Apache-2.0. See [LICENSE](../../subscribe/LICENSE). Original Mont copy and native vector artwork; no HeyGen assets bundled.
