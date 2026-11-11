# Cube Flip

Reusable 3D cube transition that maps outgoing and incoming timeline frames onto rotating model faces.

**Canvas:** 1080 × 1080 (square). **Transition:** 1 seconds. **License:** MIT.

```sh
npx usemont templates preview cube-flip-transition
npx usemont templates create cube-flip-transition ./cube-flip-transition
```

The CLI entry, `preview.tsx`, demonstrates the transition with two animated scene captures.

Use `scene.tsx` to transition between timeline clips: bind `clipA` to the outgoing span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup).

Originally published by Mont in the native transition catalog; republished here with its original license.
