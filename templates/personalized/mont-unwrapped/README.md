# Mont Unwrapped

A 19-second, 1920×1080 personalized editing-room recap. One continuous camera move connects a contact sheet, film reel and timeline, and the recipient’s favourite recording layout, then pulls back into a recap poster.

## Render a row

Requires the Mont CLI, Python 3, and Node.js for the small regression check.

```sh
python3 check.py
python3 create.py /tmp/unwrapped-ada --row 0 --output /tmp/unwrapped-ada.mp4
python3 create.py /tmp/unwrapped-mateo --row 1 --output /tmp/unwrapped-mateo.mp4
python3 create.py /tmp/unwrapped-lin --row 2 --output /tmp/unwrapped-lin.mp4
```

Run these commands from this directory. Each workspace path must be new. The output is a silent, 30 fps MP4 and an editable native Mont workspace, not a flattened imported video.

`inputs.example.json` contains three fictional demo rows. Use `--inputs your-inputs.json` with a single object or an array; `--row` is zero-based. No statistics are fetched from user accounts.

| Input | Meaning |
| --- | --- |
| `name`, `company`, `product` | Personalized greeting and poster attribution |
| `year` | Recap year, integer 0–9999 |
| `videos` | Animated count, integer 0–9999 |
| `minutes` | Animated count, integer 0–999999 |
| `favouriteLayout` | `Side by side`, `Corner bubble`, or `Presenter` |
| `link` | End-card destination, shown as text |
| `accent` | Optional `#RRGGBB`; defaults to Mont orange |

Use `--logo path/to/logo.png` to replace the bundled official Mont logo. The creator embeds it in the workspace, keeping renders independent of external asset URLs. Company names and input strings are rendered as text, never injected into TSX.

Display labels and greeting size are resolved from the input row by `create.py`; rerun it into a new workspace after editing input data. Native counters use digit glyph selection rather than runtime string concatenation. Camera movement, counter timing, card arrivals, and layout changes remain native programmatic-span expressions in `scene.tsx`.

## Creative direction

Inspired by the personalized recap format of GitHub Unwrapped. The editing-room set, copy, artwork, layout, and motion are original; no reference footage or space-themed assets are included. The recording compositions are miniature illustrations, not captured UI or human footage.

Published in the [Personalized gallery](../../../README.md#personalized), with three [MP4 and GIF previews](../../../assets/previews/).
