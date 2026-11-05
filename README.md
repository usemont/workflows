![Mont kinetic typography templates](assets/header.png)

# Mont Workflows

Ready-to-run video workflows and editable motion templates for [Mont CLI](https://github.com/usemont/mont-cli).

[Try a template](#try-a-template) · [Kinetic typography](#kinetic-typography) · [Terminal showcase](#terminal-showcase) · [Workflows](#workflows) · [Contribute](#contribute)

## Try a template

```sh
# install (macOS)
brew install usemont/tap/mont
# generate preview
mont templates preview stretch-row-repeater
```

On Linux, use the [signed CLI installation guide](https://github.com/usemont/mont-cli/blob/main/docs/install.md#linux).

Saves `preview.mp4` and an editable project to `out/stretch-row-repeater/`.

Explore the latest collection with `mont templates list` and `mont workflows list`.
Inspect settings with `mont templates show <name>` before rendering.

## Kinetic typography

<table>
<tr>
<td align="center" valign="top"><a href="templates/kinetic/stretch-row-repeater"><img src="assets/previews/stretch-row-repeater-square.gif" width="260" alt="Elastic typographic ribbons"></a><br><a href="templates/kinetic/stretch-row-repeater"><strong>Stretch row repeater</strong></a><br><code>mont templates preview stretch-row-repeater</code></td>
<td align="center" valign="top"><a href="templates/kinetic/letter-inflate-swell"><img src="assets/previews/letter-inflate-swell-square.gif" width="260" alt="Letters deform through an elastic swell"></a><br><a href="templates/kinetic/letter-inflate-swell"><strong>Letter inflate swell</strong></a><br><code>mont templates preview letter-inflate-swell</code></td>
<td align="center" valign="top"><a href="templates/kinetic/spiral-type"><img src="assets/previews/spiral-type-square.gif" width="260" alt="Orbiting typography follows a spiral"></a><br><a href="templates/kinetic/spiral-type"><strong>Spiral type</strong></a><br><code>mont templates preview spiral-type</code></td>
</tr>
</table>

## Terminal showcase

**[Terminal showcase](templates/showcase/terminal-showcase)** replays recorded terminal output inside an editable native terminal window. It requires a Mont build containing native `<Terminal>` support and the bundled Noto Sans Symbols 2 braille fallback. The template source is published here; older CLI builds without that support cannot compile it.

```sh
mont templates preview terminal-showcase
# From this repository, open the full-screen TUI example instead:
mont edit templates/showcase/terminal-showcase/tui.tsx
```

- `scene.tsx` embeds [`cli.cast`](templates/showcase/terminal-showcase/cli.cast): an asciicast **v2** session with character typing, a braille loader overwritten in place, and colored success output.
- `tui.tsx` embeds [`tui.cast`](templates/showcase/terminal-showcase/tui.cast): an asciicast **v3** session with an alternate screen, box drawing, cursor movement, selection changes, cursor hiding, a resize, and restoration of the main screen.
- Replace the `recording` variable (`vars.recording`) with the complete `.cast` JSONL **text**, not a file path. The TSX defaults are ordinary escaped string literals; keep the matching `.cast` source in sync when editing them. Version 2 timestamps are absolute seconds; version 3 timestamps are intervals since the previous event.
- Both examples contain a centered terminal window with a soft shadow over a configurable scene background, with no outer titles or footer. The terminal viewport has square top corners and rounded bottom corners. Edit `windowTitle`, `background`, window colors, `width`, and `height` through the scene variables. Recorded rows and columns stay fixed and are fitted with letterboxing, not reflowed. Resize events (`"r"`, such as `"72x16"`) change that recorded grid. Adjust `durationMs` for longer recordings.
- Both examples use the embedded JetBrains Mono 400/500 faces, including window labels, with bundled Noto Sans Symbols 2 fallback for braille spinner glyphs. Native and browser previews need no font download.

These fictional commands and outputs are **recorded data only**: replay never executes commands. Compilation produces ordinary native text and rectangle tracks; no browser, JavaScript runtime, file loading, or external font dependency is required.

Use the same primitive in another scene:

```tsx
<Terminal id="terminal" recording={vars.recording}
  x={32} y={48} width={896} height={432}
  fontSize={18} foreground="#e2e8f0" background="#10151f" />
```

`recording`, viewport dimensions, `fontSize`, `fontFamily`, and theme colors must resolve at compilation. The default font is the bundled JetBrains Mono; use a monospace font to preserve cell alignment. Standard group transforms and animation modifiers apply to the terminal as a whole. Output updates are discrete, including at exact event timestamps; seeking backward restores the earlier state without replaying commands.

Replay supports the VT sequences handled by `vt100`, including carriage-return loaders, scrolling, erase operations, ANSI/256/RGB colors, wide and combining characters, cursor visibility, and alternate screens. It uses the conventional xterm palette and the authored foreground/background, not header themes. Input, marker, and exit events do not produce terminal output. This is text replay, not a terminal graphics player: sixel and Kitty graphics are rejected; unsupported OSC payloads are ignored.

Compilation rejects malformed recordings and bounds work to 16 MiB of JSONL, 100,000 events, a 400×200 maximum grid, two million stored snapshot cells, and 64 million scanned snapshot cells. For larger sessions, trim the recording or reduce its grid.

## Workflows

- **[Markdown → narrated explainer](workflows/explainer)** — Markdown, scene generation, voice, captions, and MP4. **Experimental: visual review incomplete; uses paid APIs.** Read its [setup](workflows/explainer/README.md) and [security notes](workflows/explainer/SECURITY.md) first.

## Contribute

Contributions welcome via issues and pull requests. Review unfamiliar workflows before running them: they can send inputs to configured APIs. Never commit credentials or generated projects.

[Apache-2.0](LICENSE) · Nauchu AB · [Agent skills](https://github.com/usemont/skills) · [CLI releases](https://github.com/usemont/mont-cli/releases)
