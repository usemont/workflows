# Bottom-up letters

A single editable line rises letter by letter, settles with a small overshoot, holds, and exits upward.

```sh
npx usemont templates preview bottom-up-letters
```

`scene.tsx` exposes:

| Setting | Purpose |
| --- | --- |
| `text` | Single-line, non-empty copy; defaults to `BOTTOM UP LETTERS` |
| `background`, `ink` | Background and text colors |
| `riseMs` | Per-letter rise and settle duration; minimum 150 ms |
| `staggerMs` | Delay between letters; minimum 0 ms |
| `cycleMs` | Loop length; minimum 3,000 ms |
| `width`, `height` | Canvas size; measured text scales down to fit |

Default loop: 4.2 seconds. Entry begins at 250 ms; the final 650 ms contains the upward exit and a blank seam. For longer copy or slower timing, increase `cycleMs` and the span duration so all letters can settle before the exit.

## Verify

Requires `mont`, `ffmpeg`, and Python 3:

```sh
npx --yes --package=usemont -- python3 templates/kinetic/bottom-up-letters/verify.py
```

Renders landscape and portrait variants, checks staggered entry, single-line horizontal fit, and hidden text at the loop seam. Temporary workspaces and videos are removed automatically.

