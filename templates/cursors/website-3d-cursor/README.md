# Editor 3D assembly

Square, nine-second Mont scene featuring **Forma**, an original fictional video editor. Editable native scene primitives build its media library, preview, properties panel, and timeline; the only raster asset is the embedded Mont arrow cursor. This is an illustrative animation, not a live editor or a website capture.

## Edit

- `scene.tsx`: named native UI layers, original copy, palette variables, staggered assembly, independent camera guide splines, and eased cursor arcs.
- Warm-white panels, dark plum text, and muted lilac accents coordinate the window, artwork, timeline, and surrounding background. Change the named color variables together to retheme it.
- The authored desktop is 1440 × 1000; the output is 960 × 960. Window chrome and panel wells exist from the start. Header, library, preview, inspector, and timeline content land in sequence.
- Landing motion starts 560 scene units above the window along its local Z axis, with a 900 ms quartic-out settle; opacity and blur resolve in 400 ms. Elements stay parallel to the window. `landingScale/X/Y` analytically project this depth using the shared 1800-unit perspective, pitch, yaw and camera focus; scale and displacement come from perspective, not an independent screen-space slide. No unsupported group Z property or engine extension is required.
- `pointerX/Y` visit playback, a video clip, then the opacity slider. The animated slider thumb stays aligned with the pointer during the final drag.
- `guideX/Y` define composition independently. After the opening, `focusX/Y` blend 75% delayed cursor tracking with 25% guide framing. Three trailing samples (160–520 ms) let the pointer lead and the camera settle behind it.
- Camera travel over the preceding 220 ms drives restrained whole-window pitch, yaw and roll. The window, rim, shadows, content and cursor share that transform; cursor pauses do not freeze the camera.
- A 12-pixel pale rim, outer highlight, inner seam, lavender glow, ambient shadow and contact shadow frame the window. The 50-pixel outer radius matches its 38-pixel inner radius plus the rim inset.
- `editorName`, `projectName`, `posterLine1/2`, palette, output dimensions and playback speed are scene variables. Native text and geometry remain directly editable.

## Verify and render

Requires Node.js, Python 3, and Mont CLI with programmatic spans and native rendering:

```sh
python3 templates/cursors/website-3d-cursor/verify.py
python3 templates/cursors/website-3d-cursor/verify.py /absolute/path/to/new-preview.mp4
```

Use a new output filename: Mont does not overwrite existing renders. The script materializes the embedded cursor as a workspace resource and renders at 960 × 960, 30 fps. Motion checks cover camera continuity, delayed following and settling, camera/rotation response to a changed cursor route, cursor target alignment, restrained face-up angles, close framing, invisible entrances, monotonic arrivals, and exact landing positions. An independent full 3D projection verifies each landing against surface-normal translation throughout its entrance.

Preview from the published catalog with `mont templates preview website-3d-cursor`.

Standalone create/preview requires the embedded-image template materialization support described in the repository README; CLI `0.1.0-preview.16` lacks that support. The verification script handles image materialization explicitly.
