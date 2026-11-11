# Object Flyby Wipe

Reusable object flyby wipe transition that hides the moving A/B edge behind a close foreground object crossing the camera.

**Canvas:** 1080 × 1080 (square). **Transition:** 1 seconds. **License:** MIT.

```sh
npx usemont templates preview object-flyby-wipe-transition
npx usemont templates create object-flyby-wipe-transition ./object-flyby-wipe-transition
```

The CLI entry, `preview.tsx`, demonstrates the transition with two animated scene captures.

Use `scene.tsx` to transition between timeline clips: bind `clipA` to the outgoing span and `clipB` to the incoming span. See the [A/B setup](../README.md#timeline-setup).

Originally published by Mont in the native transition catalog; republished here with its original license.
