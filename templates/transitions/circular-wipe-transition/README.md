# Circular Wipe

Reusable timeline transition template that reveals the incoming clip through an expanding circular mask over outgoing and incoming host-frame streams.

**Canvas:** 1080 × 1080 (square). **Transition:** 3 seconds. **License:** Apache-2.0.

```sh
npx usemont templates preview circular-wipe-transition
npx usemont templates create circular-wipe-transition ./circular-wipe-transition
```

The CLI entry, `preview.tsx`, is a self-contained A/B demonstration with animated scene-texture captures. The gallery GIF shows the approved live-frame timeline demonstration.

Use `scene.tsx` for the reusable transition: bind `clipA` to the outgoing timeline span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup). This source retains live host-frame sampling; it is not a baked video.

Originally published by Mont in the native transition catalog; republished here with its original license. Circular mask geometry was adapted to the approved square canvas.
