# ChatGPT exchange

An original, fictitious exchange explains making a video with Mont MCP: connect, describe a vertical teaser, review scenes, and request an MP4 export. The compact dark ChatGPT-style layout includes Chat/Work tabs, a one-line pill composer, Instant selector, microphone, and white voice/send/stop control.

**960 × 540 · 11,500 ms** (also scales uniformly to 1280 × 720). The composer docks after sending; the input clears, the right-aligned user bubble appears, and three assistant lines stream into a reading hold. Longer placements restart at a clean new-chat cut.

| Default landmark | Time |
| --- | ---: |
| Empty composer and greeting | 0–800 ms |
| Prompt types, including two brief pauses | 800–3,700 ms |
| Complete draft holds | 3,700–4,050 ms |
| Send; input clears; composer docks | 4,050–4,430 ms |
| User bubble enters | 4,150–4,350 ms |
| Assistant waiting indicator | 4,430–4,830 ms |
| Assistant streams | 4,830–8,130 ms |
| Completed reply holds; voice control returns | 8,130–11,500 ms |

## Edit

- `prompt`, `replyLine1`–`replyLine3`, `model`, `greeting`, and `placeholder` edit the copy. Keep the model label compact. Prompt and all three reply lines must be nonempty single lines supported by native `text.glyphs`.
- `background`, `surface`, `bubble`, `foreground`, `muted`, `edge`, `button`, and `buttonInk` edit the palette.
- `emptyHoldMs`, `typingMs`, `promptHoldMs`, `dockMs`, `responseDelayMs`, `streamMs`, and `finalHoldMs` edit the sequence. Their minimums are 200/600/100/180/150/900/800 ms respectively. Update the host placement duration to the sum of the effective timings for one cycle.
- Set both `width` and `height` for a new canvas. The centered 960 × 540 logical app preserves its proportions; other aspect ratios retain background margins. Long prompt/reply lines shrink to fit rather than wrap; concise copy stays most readable.

Bundled **Inter 400** supplies both UI and dialogue. Glyph advances determine typing positions and carets; opacity animates the reveals. This is authored UI motion, not a recording or a live model request.

## Verification

From the repository root, run `npx --yes --package=usemont -- python3 templates/mock-ui/verify.py` with `mont` and `ffmpeg` installed. It renders the available Mock UI scenes and checks progressive typing, complete sent text, cleared input, streamed replies, and the final reading hold.
