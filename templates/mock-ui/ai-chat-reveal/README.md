# AI chat reveal

An unbranded mobile conversation about creating a video with Mont MCP. Authored UI animation, not a recording or live assistant output. Uses native Mont primitives and the reusable [mobile keyboard](../../components/mobile-keyboard/).

- **Canvas:** 540 × 960 logical pixels; 11.5 seconds.
- **Style:** warm paper, dark ink, pale green accents, Inter typography.
- **Sequence:** keyboard rises, synchronized typing, send clears and docks the composer, thinking dot, uneven word bursts, reading hold. No closing card.

## Preview

From a clone of this repository with `mont` installed:

```sh
python3 templates/mock-ui/ai-chat-reveal/create.py ./ai-chat
mont edit -C ./ai-chat
```

The creator snapshots both editable TSX sources into the project and registers the keyboard as a scene asset. No engine changes or bundled video are required. The CLI's catalog preview currently loads a single source without scene-asset registration, so this composed template uses the command above instead of `mont templates preview`.

## Timing

| Time | State |
| --- | --- |
| 0.18–0.52s | Keyboard and composer rise |
| 0.80–3.50s | Prompt types; matching letter, Shift, space and punctuation keys highlight |
| 3.50–3.80s | Completed draft holds |
| 3.80–4.16s | User bubble appears; keyboard drops; input clears |
| 4.16–4.81s | Thinking dot pulses |
| 4.81–9.11s | Assistant streams words with irregular token beats |
| 9.11–11.31s | Completed reply holds |

Edit `variables` for the prompt, assistant label, placeholder, palette and timing. `tokens.typing` contains the prompt's timed keystrokes: `atMs` is relative to typing start, `count` is the visible prefix length, and `character` selects the matching keyboard key. Keep these characters consistent with `variables.prompt`; word pauses are simply larger timestamp gaps. `typingMs` scales the complete sequence. Width/height uniformly scale and center the portrait composition. Update `durationMs` when extending the interaction. Typing, thinking, streaming and reading hold clamp to 800, 250, 1800 and 1000ms minimum respectively.

Reply words are inline records with `copy`, `prefix`, `row`, and `beat`. `prefix` is the preceding text on that row, including spaces. Native font measurement positions each word; rows are intentionally authored rather than automatically wrapped. Edit prefixes and row numbers with the copy. Beats may repeat for short word bursts; the default stream has 32 beat slots. Each word fades from muted to dark ink over 150ms. All text is statically shaped; only opacity and transforms animate.

The `AssetScene` named `mobile-keyboard` receives the active character, case, symbol page and palette. `create.py` registers `templates/components/mobile-keyboard/scene.tsx` with the native `imageAssets` descriptor. The same timed event controls visible text and key highlighting. This remains an authored animation, not an interactive text field.

## Verification

Run `python3 templates/mock-ui/verify.py` from the repository root with `mont` and `ffmpeg` installed. It renders all Mock UI scenes and checks progressive typing, complete sent text, restored placeholders, streamed replies and stable reading holds. The portrait scene additionally checks capital/lowercase keys, Shift, space, punctuation, released keys during pauses and the final conversation hold.
