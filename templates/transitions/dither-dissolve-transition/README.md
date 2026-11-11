# Dither Dissolve

Reusable shader transition that dissolves the outgoing clip into the incoming clip with an ordered Bayer dither pattern and subtle diagonal grain.

**Canvas:** 1080 × 1080 (square). **Transition:** 1 seconds. **License:** MIT.

```sh
npx usemont templates preview dither-dissolve-transition
npx usemont templates create dither-dissolve-transition ./dither-dissolve-transition
```

The CLI entry, `preview.tsx`, demonstrates the transition with two animated scene captures.

Use `scene.tsx` to transition between timeline clips: bind `clipA` to the outgoing span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup).

Originally published by Mont in the native transition catalog; republished here with its original license.
