# Mobile keyboard

Reusable native `AssetScene`, 540 × 326 logical pixels. It has no built-in typing timeline: the host passes the currently pressed key, case, symbol page and palette. Position or scale it with the parent `Group`.

| Input | Meaning |
| --- | --- |
| `pressed` | Active character, `" "`, `"Enter"`, `"Backspace"`, or `""` to release |
| `uppercase` | Uppercase labels and active Shift |
| `symbols` | Number/punctuation page instead of letters |
| `surface`, `foreground`, `muted`, `background`, `secondary`, `accent` | Key faces, labels, shadows, base, modifier keys and active key |

Supported pages are explicitly authored in `scene.tsx`: QWERTY letters and `1234567890-/:;()$&@.,?!'"_`. This is a visual animation component, not a system keyboard or text editor.

## Timed input

The host can keep timed events in its normal scene tokens and select the latest event using `collection.topBy`. A track such as this types `Hi `, including a word pause:

```tsx
tokens: {
  typing: [
    { atMs: -10000, character: "", count: 0 },
    { atMs: 800, character: "H", count: 1 },
    { atMs: 900, character: "i", count: 2 },
    { atMs: 1100, character: " ", count: 3 },
  ],
},
computeds: {
  event: computed.struct(() => collection.topBy(tokens.typing,
    event => event.atMs <= time.ms ? event.atMs : -100000, 1)[0]),
  pressed: computed.string(() => time.ms - computed.event.atMs < 55
    ? computed.event.character : ""),
  uppercase: computed.boolean(() => text.lower(computed.event.character)
    !== computed.event.character),
},
```

Pass those values to the asset. Use the same event's `count` to reveal the host's draft prefix, rather than maintaining a second typing clock.

```tsx
<AssetScene id="keyboard" asset="mobile-keyboard" inputs={{
  pressed: computed.pressed,
  uppercase: computed.uppercase,
  symbols: false,
}} />
```

Register the asset in the programmatic scene state's `imageAssets`:

```json
{
  "mobile-keyboard": {
    "scene": {
      "identity": "mobile-keyboard",
      "sourceTsx": "<contents of scene.tsx>",
      "dependencies": {}
    }
  }
}
```

See [AI chat reveal](../../mock-ui/ai-chat-reveal/) for the complete host scene and `templates/mock-ui/verify.py` for runnable native packaging, rendering and synchronization checks.
