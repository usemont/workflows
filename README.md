<p><a href="https://usemont.com"><img src="https://raw.githubusercontent.com/usemont/.github/main/assets/logo.svg" alt="Mont" width="150" height="40"></a></p>

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
<tr>
<td align="center" valign="top"><a href="templates/kinetic/morph-text"><img src="assets/previews/morph-text-square.gif" width="260" alt="Bold phrases merge through a liquid ink morph"></a><br><a href="templates/kinetic/morph-text"><strong>Morph text</strong></a><br><code>mont templates preview morph-text</code></td>
<td align="center" valign="top"><a href="templates/kinetic/bottom-up-letters"><img src="assets/previews/bottom-up-letters-square.gif" width="260" alt="Bottom up letters rise with staggered motion and a gentle overshoot"></a><br><a href="templates/kinetic/bottom-up-letters"><strong>Bottom-up letters</strong></a><br><code>mont templates preview bottom-up-letters</code></td>
</tr>
</table>

## Terminal showcase

<table>
<tr>
<td align="center" valign="top"><a href="templates/showcase/terminal-showcase"><img src="assets/previews/terminal-showcase.gif" width="640" alt="Recorded CLI typing, animated loader, and colored output"></a><br><a href="templates/showcase/terminal-showcase"><strong>Terminal showcase</strong></a><br><code>mont templates preview terminal-showcase</code></td>
</tr>
</table>

## Workflows

- **[Markdown → narrated explainer](workflows/explainer)** — Markdown, scene generation, voice, captions, and MP4. **Experimental: visual review incomplete; uses paid APIs.** Read its [setup](workflows/explainer/README.md) and [security notes](workflows/explainer/SECURITY.md) first.

## Contribute

Contributions welcome via issues and pull requests. Review unfamiliar workflows before running them: they can send inputs to configured APIs. Never commit credentials or generated projects.

[Apache-2.0](LICENSE) · Nauchu AB · [Agent skills](https://github.com/usemont/skills) · [CLI releases](https://github.com/usemont/mont-cli/releases)
