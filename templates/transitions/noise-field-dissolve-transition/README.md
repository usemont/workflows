# Noise Field Dissolve

Reusable procedural noise-field dissolve transition with soft threshold motion, subtle turbulent texture, and clean host-frame endpoints.

**Canvas:** 1080 × 1080 (square). **Transition:** 1 seconds. **License:** MIT.

```sh
npx usemont templates preview noise-field-dissolve-transition
npx usemont templates create noise-field-dissolve-transition ./noise-field-dissolve-transition
```

The CLI entry, `preview.tsx`, is a self-contained A/B demonstration with animated scene-texture captures. The gallery GIF shows the approved live-frame timeline demonstration.

Use `scene.tsx` for the reusable transition: bind `clipA` to the outgoing timeline span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup). This source retains live host-frame sampling; it is not a baked video.

Originally published by Mont in the native transition catalog; republished here with its original license. Transition source is unchanged.
