# Agent cursor

Native Mont TSX adaptation of [HeyGen’s Agent Cursor](https://hyperframes.heygen.com/catalog/components/agent-cursor.md): an AI label types its intent before each curved target move, performs two meaningful clicks, then confirms and leaves. Original Mont teaser setup copy and UI; no HeyGen visual assets.

**1920 × 1080 · 4.8 seconds · render at 30 fps.** Light neutral surfaces, lavender accent, embedded first-party Mont arrow, no browser or network dependency. The offscreen image prewarms the cursor resource. Keep the data URI registered as a resource when dispatching directly through the CLI.

## Beats

| Time | Action |
| --- | --- |
| 0–0.65 s | Broad quadratic entry from the right; “Reading setup” types into the agent label. |
| 0.85–1.21 s | “Set vertical format” types while the cursor rests. |
| 1.25–1.80 s | Minimum-jerk quadratic approach to the Vertical card, followed by a stationary click dwell. |
| 2.00 s | First press/ring: select 9:16, reshape the preview and update its resolution. |
| 2.15–2.44 s | “Turn on captions” types before movement. |
| 2.31–2.55 s | If needed for the next target, the label eases left along a shallow lower detour, clear of the arrow. |
| 2.55–3.10 s | Gentle quadratic approach to the captions toggle, followed by a stationary click dwell. |
| 3.30 s | Second press/ring: enable captions, reveal the preview subtitle and update the setup summary. |
| 3.65–4.30 s | White check appears; “Ready to create” types, then holds. |
| 4.30–4.80 s | Quadratic exit down/right; label fades by 4.52 s; completed setup stays readable. |

## Geometry / review assertions

- Cursor hotspot and ring centers must coincide at **(1166, 532)** at 2.00 s and **(1630, 670)** at 3.30 s. Both are inside their actual UI controls.
- The 56 px-high card hugs the **complete current intent**, not its typed prefix. Bundled Inter `text.glyphs(...)[0].totalAdvance` measures each status at 400/20 px, the agent name at 600/20 px, and the separator at 400/23 px. The icon keeps its original 46 px reservation; separator gaps are 10 px each, with 16 px right padding.
- Width changes use 180 ms minimum-jerk interpolation at 0.85, 2.15 and 3.65 s. Text origins remain fixed within the card. Once on the left, its left edge stays anchored to the wider of the two later labels, so the confirmation resize does not slide the text.
- Position clamps use the **current measured width**, leaving 24 px frame margins. The card sits at least 54 px below the hotspot; its conditional 240 ms side-change adds up to 20 px of clearance rather than crossing the arrow. The side change is skipped if the later labels fit to the target's right.
- The arrow is 55.652173913 px square, offset **(−9.275362319, −6.956521739)** from the hotspot. Press scaling uses a hotspot-anchored group and bottoms at 0.94; the image has an explicit transparent zero-width border.
- The 9:16 preview lands at **(412, 411), 216 × 384 px**, exactly 9:16. Its graphic and caption remain inside the frame.
- The preview's concentric artwork shares one center at **(1/φ, 1/φ³)** of the animated viewport: the 61.8% golden-ratio column and upper 23.6% secondary row. Outer/inner diameters are **φ:1**. The outer diameter is the settled viewport's shorter side divided by **φ²**; interpolate the two endpoint sizes during resizing so the circles shrink smoothly without bulging or drifting apart.
- Intent strings finish before their target moves: default first intent finishes by 1.192 s (move starts 1.25 s), second by 2.438 s (move starts 2.55 s).
- All four cursor legs are monotone quadratic arcs driven by quintic minimum-jerk progress: no sine hooks, overshoot, or bounce. Velocity and acceleration settle to zero at every wait. The cursor is stationary at the first target from 1.80–2.55 s and the second from 3.10–4.30 s.
- Suggested inspection frames: **0.35, 0.70, 1.20, 1.53, 2.10, 2.43, 2.83, 3.40, 3.90, 4.767 s**. Check full-phrase card fit, width transitions, curved travel, both rings/UI responses, the lower label detour, final check and departure.

Editable inputs: agent name, project name, the four short status lines, accent, background and cursor image. Label strings must be nonempty, single-line text; keep replacements short enough for the 1920 px composition. Width/height preserve the composition with a centered uniform scale.

Run `node templates/cursors/agent-cursor/verify.mjs` from the repository root. The regression reproduces the former center drift, then checks concentricity, golden-ratio placement, diameter proportions, circularity, containment and monotone sizing throughout the format transition, alongside the existing click/motion assertions. These checks do not replace native font shaping or visual review. Verified a native 1920×1080, 30 fps MP4 export (144 frames), inspecting 1.90, 2.03, 2.10, 2.17, 2.30 and 3.90 s for the landscape-to-portrait reflow, shared circle centers, spacing and completed setup.

## Attribution

Reference concept: Agent Cursor, Copyright 2026 HeyGen, Inc., Apache-2.0. See [license](../../subscribe/LICENSE). Cursor imagery is the supplied first-party Mont arrow; all application copy, preview artwork and UI are original.
