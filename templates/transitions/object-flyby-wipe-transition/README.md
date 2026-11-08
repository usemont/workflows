# Object Flyby Wipe

Reusable object flyby wipe transition that hides the moving A/B edge behind a close foreground object crossing the camera.

**Canvas:** 1080 × 1080 (square). **Transition:** 1 seconds. **License:** MIT.

```sh
mont templates preview object-flyby-wipe-transition
mont templates create object-flyby-wipe-transition ./object-flyby-wipe-transition
```

The CLI entry, `preview.tsx`, is a self-contained A/B demonstration with animated scene-texture captures. The gallery GIF shows the approved live-frame timeline demonstration.

Use `scene.tsx` for the reusable transition: bind `clipA` to the outgoing timeline span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup). This source retains live host-frame sampling; it is not a baked video.

Originally published by Mont in the native transition catalog; republished here with its original license. Transition source is unchanged.
