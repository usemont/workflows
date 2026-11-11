# Page Turn

Reusable page-curl transition that rolls the outgoing clip away like a turning page, revealing the incoming clip underneath with curvature and shadow controls.

**Canvas:** 1080 × 1080 (square). **Transition:** 1 seconds. **License:** MIT.

```sh
npx usemont templates preview page-turn-transition
npx usemont templates create page-turn-transition ./page-turn-transition
```

The CLI entry, `preview.tsx`, demonstrates the transition with two animated scene captures.

Use `scene.tsx` to transition between timeline clips: bind `clipA` to the outgoing span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup).

Originally published by Mont in the native transition catalog; republished here with its original license.
