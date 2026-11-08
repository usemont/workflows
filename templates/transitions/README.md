# Transitions

Eight square transitions from Mont's native catalog, with the approved animated A/B previews.

| Template | Canvas | Transition duration | License |
| --- | --- | --- | --- |
| [Cube Flip](cube-flip-transition) | 1080 × 1080 | 1 s | MIT |
| [Circular Wipe](circular-wipe-transition) | 1080 × 1080 | 3 s | Apache-2.0 |
| [Reverse Circle Wipe](reverse-circle-wipe-transition) | 540 × 540 | 1 s | MIT |
| [Dither Dissolve](dither-dissolve-transition) | 1080 × 1080 | 1 s | MIT |
| [Liquid Metal Wipe](liquid-metal-wipe-transition) | 1080 × 1080 | 1 s | MIT |
| [Page Turn](page-turn-transition) | 1080 × 1080 | 1 s | MIT |
| [Object Flyby Wipe](object-flyby-wipe-transition) | 1080 × 1080 | 1 s | MIT |
| [Noise Field Dissolve](noise-field-dissolve-transition) | 1080 × 1080 | 1 s | MIT |

## Preview or create

```sh
mont templates preview cube-flip-transition
mont templates create cube-flip-transition ./cube-flip-transition
```

Each bundle contains two editable sources:

- **`preview.tsx`** is the CLI entry: a standalone demonstration using two animated scene-texture captures. It needs no external footage or timeline bindings.
- **`scene.tsx`** is the reusable transition: it samples the outgoing and incoming timeline frames through `hostFrameInputs`. It is not a baked video.

Gallery GIFs show the approved live-frame timeline demonstration, including short holds before and after each transition. The standalone entry runs the transition itself.

## Timeline setup

Use `scene.tsx` in a programmatic transition span, with the two input bindings on its capability:

```json
{
  "kind": "programmatic.scene",
  "version": 1,
  "sourceBindings": {
    "clipA": "outgoing-span-id",
    "clipB": "incoming-span-id"
  },
  "state": {
    "sourceTsx": "<contents of scene.tsx>"
  }
}
```

Replace the example IDs with the actual source span IDs. Put the transition above both sources. For a seam at time `T` and transition duration `D`, place it at `T - D / 2` for duration `D`, with the outgoing clip ending and incoming clip starting at `T`.

Keep the source's `hostFrameInputs` declarations and `host-frame:clipA` / `host-frame:clipB` references: these are live timeline inputs, not asset paths. The CLI's single-span template creation does not supply those bindings, which is why its entry uses scene captures instead.

The authored geometry is square. Changing only project dimensions does not re-layout the shader, mask, or cube geometry; adapt the source when targeting another aspect ratio.

## Attribution and licenses

Originally published by Mont in its native transition catalog. Each template retains its original license in its own `LICENSE` file. Cube Flip and Circular Wipe are adapted for square presentation; the other six retain their catalog source geometry.
