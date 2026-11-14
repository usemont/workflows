# Product finish launch

Original 12-second, 16:9 studio reveal of AERA, a fictional desktop audio controller.
Three procedural products show teal matte, orange lacquer and pale brushed-metal finishes.
A continuous camera pullback and subtle product spread reveal the collection above a reflective floor.
No borrowed models, images or product branding.

## Create and render

Requires a current Mont CLI source build including native `sourceMont` span validation; preview.16 does not support this ingress path.

```sh
python3 create.py ./output/product-finish --inputs inputs.example.json --output ./output/product-finish.mp4
```

Use `--mont /path/to/mont` to select a current source build. Omit `--output` to create only the editable project.
Default project size is 1920 × 1080 at 30 fps.

| Input | Default | Meaning |
| --- | --- | --- |
| `productName` | `AERA` | Short product wordmark, rendered using bundled Geist |
| `accent` | `#EF713D` | Hero product sRGB hex colour |
| `roughness` | `0.38` | Hero surface, clearcoat and metal-dial finish, 0–1 |

`scene.mont` is the canonical native spatial source, mirrored in the SDK showcase fixture.
`create.py` only calls Mont CLI initialization and the existing programmatic-span command with
`sourceMont` and `variables`. All geometry, materials, absolute-time motion, shadows and reflections
are evaluated by the shared Rust SDK; no Python, browser or client-side renderer is involved.
The underlying spatial publication contract is also available to JSX and procedural authors.

Review preview only: not added to the public catalog until approval.
