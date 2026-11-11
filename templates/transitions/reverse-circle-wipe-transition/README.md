# Reverse Circle Wipe

Reusable reverse circular wipe transition that keeps the outgoing frame inside a shrinking circle while revealing the incoming frame behind it.

**Canvas:** 540 × 540 (square). **Transition:** 1 seconds. **License:** MIT.

```sh
npx usemont templates preview reverse-circle-wipe-transition
npx usemont templates create reverse-circle-wipe-transition ./reverse-circle-wipe-transition
```

The CLI entry, `preview.tsx`, demonstrates the transition with two animated scene captures.

Use `scene.tsx` to transition between timeline clips: bind `clipA` to the outgoing span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup).

Originally published by Mont in the native transition catalog; republished here with its original license.
