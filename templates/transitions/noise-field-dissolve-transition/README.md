# Noise Field Dissolve

Reusable procedural noise-field dissolve transition with soft threshold motion, subtle turbulent texture, and clean host-frame endpoints.

**Canvas:** 1080 × 1080 (square). **Transition:** 1 seconds. **License:** MIT.

```sh
npx usemont templates preview noise-field-dissolve-transition
npx usemont templates create noise-field-dissolve-transition ./noise-field-dissolve-transition
```

The CLI entry, `preview.tsx`, demonstrates the transition with two animated scene captures.

Use `scene.tsx` to transition between timeline clips: bind `clipA` to the outgoing span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup).

Originally published by Mont in the native transition catalog; republished here with its original license.
