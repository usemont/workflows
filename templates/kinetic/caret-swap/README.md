# Caret swap

An original Mont type study: **Make Space. → Move Ideas.** Cream Archivo 900 on midnight blue, interrupted by a chartreuse caret. A smooth opaque sweep covers the phrase and contracts into a caret, then the replacement types on crisp, discrete beats. A short readable hold leads into a reverse wipe and smooth source reveal before the loop.

Inspired by the motion vocabulary of [Hyperframes Caret Swap](https://hyperframes.heygen.com/catalog/components/caret-swap.md). This is an independently authored native scene, not a DOM/GSAP port or a text morph. The native cover is deliberately **solid**, not a simulated gradient mask.

## Use

Preview with `npx usemont templates preview caret-swap`, or load `scene.tsx` as a `programmatic.scene` capability with the matching frame size. Set both `width` and `height` variables when changing aspect ratio. The default cycle is **3,600 ms**, with an edit preview at 2,300 ms; longer placements loop automatically.

| Variables | Purpose |
| --- | --- |
| `sourceText`, `replacementText` | Editable nonempty single-line phrases. |
| `background`, `ink`, `caretColor` | Midnight, cream, and chartreuse defaults. |
| `fontSize` | Maximum headline size; both phrases share a measured responsive fit. |
| `sourceHoldMs`, `wipeMs`, `collapseMs`, `pauseMs` | Opening hold, cover sweep, smooth collapse, and short pretyping beat. |
| `typingMs`, `finalHoldMs`, `returnMs` | Complete discrete type-in, readable final hold, and smooth source reveal including its final hold. The reverse cover reuses `wipeMs` and its easing. |
| `blinkMs` | Duration of each caret on/off interval during the final hold, measured from typing completion. |

### Default rhythm

| Phase | Duration | Cycle time |
| --- | ---: | ---: |
| Source hold | 500 ms | 0–500 ms |
| Cover sweep | 350 ms | 500–850 ms |
| Collapse to caret | 250 ms | 850–1,100 ms |
| Pretyping beat | 100 ms | 1,100–1,200 ms |
| Discrete typing | 900 ms | 1,200–2,100 ms |
| Replacement hold | 800 ms | 2,100–2,900 ms |
| Reverse cover | 350 ms | 2,900–3,250 ms |
| Source reveal | 350 ms | 3,250–3,600 ms |

Both cover sweeps, collapse, and source reveal use cubic ease-in-out: acceleration and deceleration without overshoot, bounce, or reversal. The final 20% of `returnMs` is a settled hold, preventing a loop-boundary jump. Typing stays discrete and evenly paced (about 82 ms per shaped cluster for the default replacement), never a continuous text wipe. The 300 ms hold blink restarts when typing completes; the caret stays solid for the last 120 ms before the reverse sweep and through the opening source hold, preserving continuity into each wipe and across the loop.

Timing minimums are 200 ms for the opening hold, 150 ms each for sweeps/collapse/source reveal, 50 ms for the pretyping beat, 300 ms for typing, 400 ms for the replacement hold, and 100 ms for a blink interval. The cycle duration sums the clamped phases, including two `wipeMs` phases. Update the host placement duration when a changed timing should occupy exactly one cycle.

## Native behavior

Only `Scene`, `Group`, `Rect`, `Text`, static `Repeat`, typed variables/computeds, and native `text.glyphs` are used. There are no assets, browser APIs, runtime JavaScript, shaders, or new built-in elements. The caret is 0.08 em wide, 1.1 em tall, with a 0.06 em gap after each measured prefix. Each typed caret uses the shaped record's `offset + advance`, never a character-count width estimate. The line reserves the larger phrase's measured width, so its left edge remains stable while typing; both phrases fit 960×540 and 540×960 using the same scale.

The bundled Archivo 900 face must be available in the native runtime. Text must satisfy `text.glyphs`: nonempty, no control characters/newlines, and independently renderable shaped clusters. A font ligature or combined sequence can be one cluster; contextual or reordered scripts unsupported by that API are rejected rather than silently approximated.

## Behavioral check

With installed `mont` and `ffmpeg`:

```sh
npx --yes --package=usemont -- python3 templates/kinetic/caret-swap/verify.py
```

The check creates temporary native projects, renders landscape/title-case defaults and portrait/edited mixed-width copy, and samples their actual pixels. It checks bounded monotonic coverage and collapse without rebound or text leaks, neighboring-frame discrete typing, caret movement proportional to narrow/wide glyphs, phase-anchored hold blinking, a stable readable hold, reverse coverage/reveal without overshoot, framing, and matching source/caret silhouettes immediately before and at the loop boundary. No source-text assertions.
