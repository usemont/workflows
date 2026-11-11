# Text-select cursor

Native Mont TSX, **1920 × 1080, 5.6 seconds, 30 fps**.

A light document receives the first-party Mont arrow, switches to Mont’s I-beam, selects a phrase character by character, raises a floating formatting toolbar, clicks **Bold**, and holds the genuinely bold result while the pointer moves aside.

Preview: `npx usemont templates preview text-select-cursor`

## Editable inputs

`title`, `beforeText`, `phrase`, `afterText`, `bodyLine1`, `bodyLine2`, `meta`, `footer`, `background`, and `accent` are scene variables. Keep `phrase` nonempty and the complete selected line (`beforeText + phrase + afterText`) within the document’s 1106 px text column; surrounding paragraphs are deliberately editable individual lines, not an automatic text-wrap system. `width` and `height` letterbox the 1920×1080 design proportionally.

Native `text.glyphs` supplies proportional shaped advances for the prefix, phrase, and individual selection cells. The phrase stays a complete `Text` run to retain native shaping. At 3.15 s, its font weight changes from 400 to 700; the suffix and selection width use the separately measured bold advance.

The embedded PNGs are Mont’s `large-2x/cursor-arrow.png` and `cursor-text.png`, prewarmed offscreen. Both render at 55.652173913 px; arrow source hotspot (16,12) maps to offsets (-9.275362319,-6.956521739), text source hotspot (48,48) to (-27.826086957,-27.826086957). Arrival, toolbar approach, and move-aside follow curved minimum-jerk paths.

## Timing / review points

| Time | Beat |
| --- | --- |
| 0–0.62 s | Curved arrow arrival; switches to I-beam at the phrase start. |
| 0.62–0.84 s | Small 8 px preparatory pullback, then return to the selection anchor. |
| 0.84–0.9 s | Press feedback before dragging. |
| 0.9–2.02 s | Minimum-jerk drag; selection follows measured glyph boundaries. |
| 2.07–2.16 s | Release; the complete phrase remains selected. |
| 2.16–2.46 s | Toolbar rises 18 px with a roughly 3 px settling overshoot. |
| 2.5–3.03 s | Arrow follows an upward arc to Bold, then pauses on target. |
| 3.08–3.32 s | Press, commit at 3.15 s, release; tiny button recoil and quiet click ring. |
| 3.38–3.94 s | Curved departure to the document’s lower-right whitespace. |
| 3.94–5.6 s | Readable final hold with less than 2 px of cursor idle drift. |

## Animation principles

| Principle | Application |
| --- | --- |
| Squash and stretch | Subtle reciprocal cursor strain on press; text and document stay rigid. |
| Anticipation | Preparatory pullback and a short deliberate pre-drag press. |
| Staging | One action at a time; toolbar appears only after selection release. |
| Straight-ahead / pose-to-pose | Authored contact, click and departure poses; deterministic interpolation between them. |
| Follow-through / overlap | Toolbar settles after arrival; button releases before departure; idle drift continues. |
| Slow in / slow out | Minimum-jerk travel and drag, with zero endpoint velocity. |
| Arcs | Independently authored arrival, toolbar approach and departure curves. |
| Secondary action | Quiet hover, compression and low-opacity click ring support the real format change. |
| Timing | Separate selection, release, recognition, click and final-reading beats. |
| Exaggeration | Readable press deformation and small toolbar overshoot, not a bouncing interface. |
| Solid drawing | Native glyph metrics, fixed UI geometry, and hotspot-anchored pointer transforms. |
| Appeal | Restrained lavender feedback and uncluttered original document artwork. |

Run `node templates/cursors/text-select-cursor/verify.mjs` from the repository root. It checks monotonic selection/frontier contact, continuity at travel boundaries, the exact Bold dwell, press/release and deformation limits for several text widths.

## Attribution

Motion concept adapted from [HeyGen Text Select Cursor](https://hyperframes.heygen.com/catalog/components/cursor-text-select.md), Copyright 2026 HeyGen, Inc., Apache-2.0; see [LICENSE](../../subscribe/LICENSE). Document copy and native Mont implementation are original. No HeyGen imagery or cursor artwork is included.
