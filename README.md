# Awesome Mont Workflows

Ready-to-run video workflows and editable motion templates for [Mont CLI](https://github.com/usemont/mont-cli).

## Kinetic typography

<table>
<tr>
<td align="center" valign="top"><a href="templates/kinetic/stretch-row-repeater"><img src="assets/previews/stretch-row-repeater-square.gif" width="260" alt="Alternating rows of type stretch and slide in opposite directions"></a><br><a href="templates/kinetic/stretch-row-repeater"><strong>Stretch row repeater</strong></a></td>
<td align="center" valign="top"><a href="templates/kinetic/letter-inflate-swell"><img src="assets/previews/letter-inflate-swell-square.gif" width="260" alt="Pink letters inflate and swell across a dark canvas"></a><br><a href="templates/kinetic/letter-inflate-swell"><strong>Letter inflate swell</strong></a></td>
<td align="center" valign="top"><a href="templates/kinetic/spiral-type"><img src="assets/previews/spiral-type-square.gif" width="260" alt="Repeated typography travels along a continuous spiral"></a><br><a href="templates/kinetic/spiral-type"><strong>Spiral type</strong></a></td>
</tr>
</table>

**16:9 · 9:16 · 1:1.** Editable text, colors, and motion. No inference API key needed. Previews are native Mont renders.

## Try a template

[Install Mont CLI](https://github.com/usemont/mont-cli#install) **0.1.0-preview.5 or newer**, then:

```sh
git clone https://github.com/usemont/awesome-workflows.git
cd awesome-workflows
template=stretch-row-repeater # or letter-inflate-swell / spiral-type
mkdir -p "out/$template"
mont -C "$PWD/out/$template" workflow run \
  "$PWD/templates/kinetic/$template/template.mont" --run-id "$template"
```

Outputs: `out/<template>/preview.mp4` and an editable workspace in `out/<template>/project/`.
See each template for text and aspect-ratio settings. Use a new output directory and run ID for each new generation.

## Workflows

- **[Markdown → narrated explainer](workflows/explainer)** — Markdown, scene generation, voice, captions, and MP4. **Experimental: visual review incomplete; uses paid APIs.** Read its [setup](workflows/explainer/README.md) and [security notes](workflows/explainer/SECURITY.md) first.

## Use and contribute

Workflows execute instructions and can send inputs to configured APIs; review unfamiliar collections before running them. Contributions welcome via issues and pull requests. Never commit credentials or generated projects.

[Apache-2.0](LICENSE) · Nauchu AB · [Agent skills](https://github.com/usemont/skills) · [CLI releases](https://github.com/usemont/mont-cli/releases)

Kinetic templates inspired by [UXPilot](https://uxpilot.ai/kinetic-typography); recreations, not pixel-identical copies.
