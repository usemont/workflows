# Claude exchange

An editable Mont scene explains creating a video with Mont CLI—create a template workspace, edit it, then render an MP4. This is authored UI motion, not a recording or real model output.

- **Canvas:** 960 × 540, **duration:** 11.5 seconds.
- **UI:** dark `#191919` canvas; 656 × 103 composer, 14px corners, plus/microphone/send controls; separate 36px Project / Opus 5.5 Medium / Manual tray.
- **Fonts:** embedded Inter for UI and assistant prose, and JetBrains Mono for CLI commands.

## Default timing

| Time | State |
| --- | --- |
| 0–0.65s | Empty composer and voice waveform |
| 0.65–3.10s | Prompt types with two brief pauses |
| 3.10–3.45s | Completed draft; send button presses |
| 3.45–3.85s | Input clears, composer docks, user bubble appears |
| 3.45–7.35s | Stop control replaces send while responding |
| 3.95–7.35s | Assistant streams four short lines |
| 7.35–11.50s | Finished reply holds; voice control returns |

Edit `variables` in `scene.tsx` for prompt, placeholder, reply lines, model, tray labels, palette, canvas, or timing. Keep replies within the 600px content column; prompts fit by horizontal scaling. Width/height scale and center the logical 960 × 540 layout uniformly. Update scene `width`/`height` metadata when changing export dimensions; 1280 × 720 uses the same layout. Set `durationMs` to the resulting cycle length when changing timing, and `editPreviewMs` to the desired editing still.

The prompt uses static native `text.glyphs` records and opacity reveals. Replies keep each line as a single shaped `Text` node, with background-colored reveal rectangles; CLI flags and hyphenated template names remain intact.

## Verification

From the repository root, run `npx --yes --package=usemont -- python3 templates/mock-ui/verify.py` with `mont` and `ffmpeg` installed. It renders the available Mock UI scenes and checks progressive typing, complete sent text, cleared input, streamed replies, and the final reading hold.
