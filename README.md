# Mont workflows and native templates

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

**Requires an unreleased Mont build supporting the explainer/provider workflow operations.** The currently published CLI packages cannot execute this workflow. This repository does not distribute the proprietary CLI or SDK; do not install an older release expecting these commands. Rendering also needs the runtime dependencies documented by your CLI distribution. Inference requires provider access and incurs costs. Review the configuration before running; no key is included.

## Native templates

[`templates/engine/native`](templates/engine/native) is the public, standalone Rust scene generator, included in this source collection with no private SDK dependency. It emits editable native Mont scene descriptions rather than HTML, browser recordings, or pre-rendered footage. The proprietary SDK compiler and renderer are not included; running the full workflow requires a compatible separately distributed Mont CLI.

- 14 families, 32 variants: titles, statements, illustrations, media, lists, frameworks, comparisons, categories, tool selection, rule exceptions, principles, sequences, document actions, and relational stories.
- Four themes: daylight, midnight, blueprint, signal; explicit token overrides.
- Landscape and vertical layouts, native icons, narration-relative scene timing, caption spans, and deterministic animation.

```sh
cargo test --locked --manifest-path templates/engine/native/Cargo.toml
```

Public generator tests validate template behavior without the proprietary SDK. Full native workflow acceptance was completed on 2026-10-30: real provider narration and word alignment, editable project creation, and a 28.325-second H.264/AAC MP4 at 1920×1080/30 fps. The accepted scene/audio payload also rendered natively at 1080×1920. See the [acceptance and recovery details](workflows/explainer/README.md#verified-native-acceptance).

## Reuse and contribute

Copy the child `.mont` graphs under `workflows/explainer/workflows` into your own collection, or edit the top-level DAG and public prompts. `.mont` contains typed JSON; `$asset` and `$json` references resolve against the top-level definition directory and cannot escape it. Do not move a child graph without its referenced assets.

Keep contributions focused and include a runnable behavior check. Do not commit credentials, provider responses, generated projects, media output, or private run journals. Open issues and pull requests in this repository for workflow/template changes; report CLI runtime problems in [usemont/mont-cli](https://github.com/usemont/mont-cli/issues).

Workflows are trusted executable instructions, **not a sandbox**: they can read local inputs and transmit them to configured APIs. Read the [security and recovery notes](workflows/explainer/SECURITY.md) before executing an unfamiliar collection.

## License

Copyright 2026 Nauchu AB. Apache-2.0; see [LICENSE](LICENSE). Prompts and selection criteria were adapted from the user-supplied `mont_motion_pipeline` archive. This public repository contains no Python orchestration, legacy HTML runtime, private SDK implementation, or credentials.
