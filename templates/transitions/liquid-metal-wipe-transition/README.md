# Liquid Metal Wipe

Reusable shader transition that reveals the incoming clip through a reflective liquid-metal wipe band with distortion, contour highlights, and host-frame texture refraction.

**Canvas:** 1080 × 1080 (square). **Transition:** 1 seconds. **License:** MIT.

```sh
npx usemont templates preview liquid-metal-wipe-transition
npx usemont templates create liquid-metal-wipe-transition ./liquid-metal-wipe-transition
```

The CLI entry, `preview.tsx`, demonstrates the transition with two animated scene captures.

Use `scene.tsx` to transition between timeline clips: bind `clipA` to the outgoing span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup).

Originally published by Mont in the native transition catalog; republished here with its original license.
