# Mont workflows and native templates

**Experimental; complete explainer visual fidelity is not accepted.** Source-reference visual and temporal verification remains incomplete. Some focused comparisons differ only in small rasterized regions; those results do not establish full-workflow correctness or complete collection fidelity. CLI compatibility and visual acceptance are separate: preview.5 can execute this collection, but execution tests do not certify visual quality.

Reusable, editable video workflows for [Mont CLI](https://github.com/usemont/mont-cli), published by Nauchu AB under Apache-2.0. Workflows and templates live in one repository so their grammar and execution contracts stay versioned together.

## Markdown → narrated explainer

Start with the [explainer workflow](workflows/explainer/README.md). It reads Markdown, classifies and groups narration, selects and fills native scenes, reviews and repairs them, synthesizes speech, aligns word-timed captions, creates an editable Mont workspace, and renders an MP4. Provider endpoints, models, voices, and credential environment references are local configuration. Clef, Jev, and Haiku are reusable nodes, not hardcoded credentials or model aliases.

```sh
git clone https://github.com/usemont/workflows.git
cd workflows/workflows/explainer
# Review config.example.json and supply OPENROUTER_API_KEY through your secret manager.
mont --json -C "$PWD" workflow run "$PWD/explainer.mont" \
  --inputs "$PWD/inputs.example.json" \
  --config "$PWD/config.example.json" --run-id explainer-example
```

**Requires Mont CLI 0.1.0-preview.5 or newer.** Earlier public packages cannot execute this workflow. Install or upgrade through the [CLI distribution](https://github.com/usemont/mont-cli), which also documents rendering dependencies. This repository does not distribute the proprietary CLI or SDK. Inference requires provider access and incurs costs. Review the configuration before running; no key is included.

## Native templates

[`templates/engine/native`](templates/engine/native) is the public, standalone Rust scene generator, included in this source collection with no private SDK dependency. It emits editable native Mont scene descriptions rather than HTML, browser recordings, or pre-rendered footage. The proprietary SDK compiler and renderer are not included; running the full workflow requires a compatible separately distributed Mont CLI.

- 14 families, 32 variants: titles, statements, illustrations, media, lists, frameworks, comparisons, categories, tool selection, rule exceptions, principles, sequences, document actions, and relational stories.
- Four themes: daylight, midnight, blueprint, signal; explicit token overrides.
- Landscape and vertical layouts, native icons, narration-relative scene timing, caption spans, and deterministic animation.

```sh
cargo test --locked --manifest-path templates/engine/native/Cargo.toml
```

Public generator tests validate structural template behavior without the proprietary SDK. Native execution was verified on 2026-10-30: real provider narration and word alignment, editable project creation, and a 28.325-second H.264/AAC MP4 at 1920×1080/30 fps, plus a native 1080×1920 re-render. **These outputs failed visual acceptance.** See the [execution verification and recovery details](workflows/explainer/README.md#native-execution-verification-not-visual-acceptance).

## Kinetic typography

Three editable native recreations inspired by [UXPilot's kinetic typography references](https://uxpilot.ai/kinetic-typography). Each directory contains a `scene.tsx` and a self-contained `template.mont` workflow that creates a Mont workspace, adds the scene, validates it, and exports an MP4. No inference API, Python, browser capture, or baked source video is used.

| Template | Duration | Editable controls |
| --- | --- | --- |
| [`stretch-row-repeater`](templates/kinetic/stretch-row-repeater) | 12 s | `text`, `alternateText`, `width`, `height`, `crimson`, `cream`, `stretch`, `cycleMs`, `phase` |
| [`letter-inflate-swell`](templates/kinetic/letter-inflate-swell) | 12 s | `text`, `width`, `height`, `background`, `pink`, `glow`, corner copy, `speed`, `staggerMs`, `swellHeight`, `swellWidth` |
| [`spiral-type`](templates/kinetic/spiral-type) | 9 s | `text`, `width`, `height`, `background`, `ink`, `speed`, `innerRadius`, `pitch` |

Run from this repository with Mont CLI 0.1.0-preview.5 or newer:

```sh
template=stretch-row-repeater # or letter-inflate-swell / spiral-type
mkdir -p "$PWD/out/$template"
mont --json -C "$PWD/out/$template" workflow run \
  "$PWD/templates/kinetic/$template/template.mont" --run-id "$template"
```

The output directory contains `preview.mp4` and the editable workspace under `project/`. Workflows accept `workspace`, `output`, `fps` (default 24), and the template controls above as top-level inputs. For example, save `{"width":1080,"height":1920,"text":"MAKE SPACE"}` in a local settings file and add `--inputs "$PWD/settings.json"`. Use a new run ID and output directory for a new generation; resuming an existing run uses its frozen source and inputs.

| Format | `width` | `height` |
| --- | ---: | ---: |
| Landscape, 16:9 | 1920 | 1080 |
| Portrait, 9:16 | 1080 | 1920 |
| Square, 1:1 | 1080 | 1080 |

Dimensions drive the project, span frame, and native scene together. Layouts adapt their row count, text fit, metadata placement, and spiral coverage rather than stretching a fixed source canvas. Full phrases replace positional character slots; native font measurement preserves proportional advances and whitespace. Text repetition is bounded, and unsupported inputs produce an error rather than silently dropping glyphs.

The row repeater retains independent horizontal letter deformation and opposed row motion. Inflate retains staggered compact-to-rounded crossfades and intentional over-canvas swelling; its rounded face is **Titan One, not the reference's unavailable custom face**, so its letterforms are not pixel-identical. Spiral follows one continuous Archimedean curve with tangent rotation and an inner fade. These templates require preview.5 or newer for native text measurement and repetition support. Fonts resolve through Mont's pinned catalog; first use of a non-embedded face requires network access.

The native compiler/renderer behavioral checks live in the owning monorepo at `projects/explainer-templates/tests/kinetic_templates.rs`. They cover short/long/default copy, measured text sizes, visible text bounds, metadata placement, letter staggering, seeking/reset, independent row deformation, spiral pitch, tangent orientation, and the central hole across all three aspect ratios, including Full HD dimensions. These checks require the private compiler and are not part of the standalone public generator's test suite.

Native CLI export verification on 2026-10-31 produced all nine template/format combinations at 24 fps: landscape 640×360, portrait 360×640, and square 480×480, with changed copy. Each MP4 decoded completely, had the expected duration and frame count, and retained synchronized editable-project and span dimensions. Rendered contact sheets were visually inspected at nine points per video, including swelling/reset and spiral edge coverage. Full HD layouts were behavior-tested, not exported in this matrix. This verifies the responsive implementation, not pixel identity with the reference videos or acceptance of the historical explainer outputs above.

## Publication boundaries

- **[usemont/workflows](https://github.com/usemont/workflows):** `.mont` workflows, reusable templates, their assets, documentation, and public generator checks. A workflow may consume multiple templates without crossing release boundaries.
- **[usemont/skills](https://github.com/usemont/skills):** agent instructions that discover and operate the CLI and workflow collection; link to examples instead of maintaining copies.
- **[usemont/mont-cli](https://github.com/usemont/mont-cli):** installation, releases, and runtime issue reporting. The proprietary SDK/compiler stays outside this source collection.

A separate template repository adds compatibility coordination without a current consumer need. Split only when templates need an independent release or contribution lifecycle.

## Reuse and contribute

Copy the child `.mont` graphs under `workflows/explainer/workflows` into your own collection, or edit the top-level DAG and public prompts. `.mont` contains typed JSON; `$asset` and `$json` references resolve against the top-level definition directory and cannot escape it. Do not move a child graph without its referenced assets.

Keep contributions focused and include a runnable behavior check. Do not commit credentials, provider responses, generated projects, media output, or private run journals. Open issues and pull requests in this repository for workflow/template changes; report CLI runtime problems in [usemont/mont-cli](https://github.com/usemont/mont-cli/issues).

Workflows are trusted executable instructions, **not a sandbox**: they can read local inputs and transmit them to configured APIs. Read the [security and recovery notes](workflows/explainer/SECURITY.md) before executing an unfamiliar collection.

## License

Copyright 2026 Nauchu AB. Apache-2.0; see [LICENSE](LICENSE). Prompts and selection criteria were adapted from the user-supplied `mont_motion_pipeline` archive. This public repository contains no Python orchestration, legacy HTML runtime, private SDK implementation, or credentials.
