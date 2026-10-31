![Mont motion templates: stretching red type, swelling pink letters, and a continuous typographic spiral](assets/header.png)

# Awesome Mont Workflows

Ready-to-run video workflows and editable motion templates for [Mont CLI](https://github.com/usemont/mont-cli).

[Try a template](#try-a-template) · [Kinetic typography](#kinetic-typography) · [Workflows](#workflows) · [Contribute](#contribute)

## Try a template

```sh
# install (macOS)
brew install usemont/tap/mont
# generate preview
mont stretch-row-repeater
```

On Linux, use the [signed CLI installation guide](https://github.com/usemont/mont-cli/blob/main/docs/install.md#linux).

Saves `preview.mp4` and an editable project to `out/stretch-row-repeater/`.

## Kinetic typography

<table>
<tr>
<td align="center" valign="top"><a href="templates/kinetic/stretch-row-repeater"><img src="assets/previews/stretch-row-repeater-square.gif" width="260" alt="Alternating rows of type stretch and slide in opposite directions"></a><br><a href="templates/kinetic/stretch-row-repeater"><strong>Stretch row repeater</strong></a><br><code>mont stretch-row-repeater</code></td>
<td align="center" valign="top"><a href="templates/kinetic/letter-inflate-swell"><img src="assets/previews/letter-inflate-swell-square.gif" width="260" alt="Pink letters inflate and swell across a dark canvas"></a><br><a href="templates/kinetic/letter-inflate-swell"><strong>Letter inflate swell</strong></a><br><code>mont letter-inflate-swell</code></td>
<td align="center" valign="top"><a href="templates/kinetic/spiral-type"><img src="assets/previews/spiral-type-square.gif" width="260" alt="Repeated typography travels along a continuous spiral"></a><br><a href="templates/kinetic/spiral-type"><strong>Spiral type</strong></a><br><code>mont spiral-type</code></td>
</tr>
</table>

## Workflows

- **[Markdown → narrated explainer](workflows/explainer)** — Markdown, scene generation, voice, captions, and MP4. **Experimental: visual review incomplete; uses paid APIs.** Read its [setup](workflows/explainer/README.md) and [security notes](workflows/explainer/SECURITY.md) first.

## Contribute

Contributions welcome via issues and pull requests. Review unfamiliar workflows before running them: they can send inputs to configured APIs. Never commit credentials or generated projects.

[Apache-2.0](LICENSE) · Nauchu AB · [Agent skills](https://github.com/usemont/skills) · [CLI releases](https://github.com/usemont/mont-cli/releases)
