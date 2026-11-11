# Drag and Drop

Native Mont TSX scene, 1920 × 1080, 3.6 seconds. A pointer lifts a video-production task, carries it into the next column, releases it, and moves away. The remaining tasks close the gap and column counts update. All cards and text remain editable native primitives; the pointer is Mont’s canonical arrow.

The board uses Linear-inspired neutral surfaces, fine borders, compact 10 px card corners, muted tags and monochrome initials, with a violet status/drop accent. The lifted shadow is 10% opacity, 24 px blur and 10 px offset; card lift is 2.5%, with at most 2° tilt.

The original demo copy follows a teaser-production board: “Ideas” → “In motion”, with tasks “Map the opening beat”, “Shape the title reveal”, “Capture a cursor gesture” and “Ship the teaser cut”. Tags are Outline, Type, Screen and Launch; beat numbers replace weekday labels, and avatar initials are fictional.

## Edit

`scene.tsx` exposes task labels, column headings, `accent`, `background`, `tone` (`light` or `dark`), `cursorSrc`, and canvas dimensions. The `clock`, `carryX`, `carryY`, lift and gap-close computations define the animation. Card motion and pointer motion share the same grip anchor throughout the drag.

Arrival (0–650 ms), carry (900–2000 ms), lift and release use minimum-jerk quintic easing: zero endpoint velocity and acceleration. After the readable hold, the pointer leaves on one continuous cubic Bézier (2600–3600 ms). Landing settles without rebound.

## Render and verify

From the workflows repository, with an embedded-image-capable `mont`, `ffmpeg`, `ffprobe`, and Python Pillow available:

```sh
npx --yes --package=usemont -- python3 templates/cursors/drag-and-drop/verify.py /absolute/path/to/new-review-directory
```

The output directory must not already exist. The command creates an editable native workspace, binds the cursor resource, renders `drag-and-drop.mp4` at 30 fps, and checks the carry trajectory, pointer grip, bounded frame-to-frame drag speed/acceleration, landing, list gap closure, counts, and stationary card after release. It also saves color-managed reference frames.

## Source and license

Native adaptation of [Drag and Drop Cursor](https://hyperframes.heygen.com/catalog/components/cursor-drag-drop), copyright 2026 HeyGen, Inc., [Apache-2.0](../../subscribe/LICENSE). Changes: Mont declarative primitives and expressions in place of HTML/CSS/JavaScript, original video-production tasks, fictional initials, Linear-inspired styling, lighter shadows, minimum-jerk easing, a continuous cursor departure, and Mont’s own cursor. No upstream media or runtime is included.

Cursor source: `slides-recorder-monorepo/projects/mont-sdk/flutter/assets/mont-cursors/png/large-2x/cursor-arrow.png`, embedded unchanged (96 × 96; SHA-256 `b6fb12e393a1550b8f22ffb7c9f7ced90cfcb66f024b493a09fef424e0409239`). Its logical hotspot is preserved while the card lifts and tilts.
