# Circular Wipe

Reusable timeline transition template that reveals the incoming clip through an expanding circular mask over outgoing and incoming host-frame streams.

**Canvas:** 1080 × 1080 (square). **Transition:** 3 seconds. **License:** Apache-2.0.

```sh
npx usemont templates preview circular-wipe-transition
npx usemont templates create circular-wipe-transition ./circular-wipe-transition
```

The CLI entry, `preview.tsx`, demonstrates the transition with two animated scene captures.

Use `scene.tsx` to transition between timeline clips: bind `clipA` to the outgoing span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup).

Originally published by Mont in the native transition catalog; republished here with its original license.
