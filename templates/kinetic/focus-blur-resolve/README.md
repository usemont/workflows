# Focus blur resolve

Three editable words pull into focus one after another, hold as a crisp single-line headline, then blur away in the same order. Uses native Mont text, Gaussian blur, and groups; no custom renderer, Rust changes, or runtime JavaScript.

| Setting | Purpose |
| --- | --- |
| `firstWord`, `secondWord`, `thirdWord` | Three non-empty words; defaults to `FOCUS`, `BLUR`, `RESOLVE` |
| `background`, `ink` | Canvas and text colors |
| `fontSize` | Preferred type size; measured text shrinks to fit the canvas |
| `blurPx` | Defocus strength in the headline's local coordinates; 0 disables blur |
| `focusMs` | Focus-pull duration; minimum 150 ms |
| `staggerMs` | Delay between consecutive words, on both entry and exit; minimum 0 ms |
| `holdMs` | Fully sharp, stationary hold; minimum 0 ms |
| `exitMs` | Soft blur-out duration; minimum 150 ms |
| `width`, `height` | Canvas dimensions |

Default loop: 3.6 seconds, with a 200 ms stagger. Each cycle has a 200 ms blank lead-in, staggered focus pulls, a shared sharp hold, staggered exits, and a 200 ms blank tail. After changing timing, set the span duration to `400 + max(150, focusMs) + 4 * max(0, staggerMs) + max(0, holdMs) + max(150, exitMs)` milliseconds, or an integer multiple for repeated cycles.

## Verify

Requires `mont`, `ffmpeg`, and Python 3:

```sh
python3 templates/kinetic/focus-blur-resolve/verify.py
```

Renders landscape and portrait variants with different words and timing. Checks that the first word appears while the last remains hidden, actual optical blur independently of opacity, a stationary sharp hold, centered width-fitting text, and blank loop boundaries. Temporary workspaces and videos are removed automatically.
