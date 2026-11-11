# Markdown → editable Mont project + narrated MP4

Turn Markdown into an editable native Mont project and narrated MP4. The workflow uses durable provider calls and the scene templates in [`templates/engine/native`](../../templates/engine/native).

## Run the example

Use [Mont CLI](../../README.md#installation) with `explainer.*`, `provider.*`, `workflow.map`, `workflow.script`, `workflow.select`, and `workflow.read-text` operations.

From this directory, review `explainer.mont`, the child definitions, and prompts. Copy `config.example.json` to `config.local.json` and configure your provider adapters:

| Adapter | Protocol and OpenRouter endpoint | Example model |
| --- | --- | --- |
| `clef` | decisions, `/api/alpha/decisions` | `cloudflare/clef` |
| `jev` | decisions, `/api/alpha/decisions` | `typesafe/jev-1.14` |
| `haiku` | chat, `/api/v1/chat/completions` | `anthropic/claude-haiku-5.5` |
| `voice` | speech, `/api/v1/audio/speech` | `hexgrad/kokoro-82m`, voice `af_heart` |
| `transcription` | transcription, `/api/v1/audio/transcriptions` | `openai/whisper-large-v3` |

Check endpoint/model availability and account access before running, especially the example Haiku model ID. Configure an available implementation of each required protocol. Transcription uses JSON `input_audio` with word timestamps; missing timestamps or speech failures fail the run.

The `clef` input selects the decisions adapter (`clef` by default; `"clef":"jev"` selects Jev). `fill` selects the fill adapter (`haiku` by default), while `haiku` selects family proposal chat. For direct Typesafe decisions, configure `https://api.typesafe.ai/v1/systemone`, model `jev-latest`, and credential reference `TYPESAFE_API_KEY`.

Provide `OPENROUTER_API_KEY` through your secret manager. Keep key values out of definitions, inputs, prompts, and tracked config. Fund the provider account and set a spending cap before running; see [Trust, costs, and attribution](#trust-costs-and-attribution).

```sh
npx usemont --json -C "$PWD" workflows run "$PWD/explainer.mont" \
  --inputs "$PWD/inputs.example.json" \
  --config "$PWD/config.local.json" \
  --run-id explainer-example
```

`markdown` is a UTF-8 filename. Relative `markdown`, `workspace`, and MP4 `output` paths resolve from the CLI's `-C` directory. For another document, use an absolute Markdown filename, a fresh workspace directory and MP4 filename, and a unique run ID. Existing workspace paths are rejected.

Customize resolution and frame rate through inputs (defaults: 1920×1080, 30 fps). `theme` accepts the native engine's themes. `brands` defaults to `{}`; brand/image sources require embedded `data:image/...` URIs. The workflow publishes speech/transcription artifacts and derives timing from the generated audio.

Successful outputs include the native `video` description, narration WAV path, editable workspace path, render result, and full `policy` review/selection state. Success requires project creation and MP4 rendering.

**Experimental workflow.** Review the rendered video's appearance, captions, transitions, and timing before publishing.

## Stages and reusable parts

1. Parse Markdown into narration blocks, preserving headings and lists.
2. Classify procedures and scene boundaries; merge short scenes at soft boundaries and split long scenes at approved gaps.
3. Select templates and repeatedly merge the leftmost eligible adjacent scenes. Merges require the same current template and section, at most 30 estimated seconds, and probability ≥ 0.5; `mergeAudit` records the sequence.
4. Generate spoken scripts, synthesize speech, align timestamped words, and build captions. Empty/unmatched transcription fails. Join equal-format RIFF WAV files with 0.25 s leading and 0.45 s trailing silence per scene; subsequent timing follows this audio.
5. Fill native templates and validate content, scene identity, timing, text limits, icons, and reveals.
6. Review narration coverage, titles, colour, and structured relationships. Each deficient scene gets at most one targeted repair and independent recheck. Invalid or regressing repairs retain the original; unrepaired unsupported relationships fail.
7. Propose picture, action, and story scenes. Malformed proposals get one bounded repair; rejected optional proposals retain the validated base scene.
8. Run two independent merit passes and compose accepted proposals with **story > action > picture** precedence. Stories replace whole scenes; actions and pictures cover at least 80% of a scene. Consecutive full action scenes are excluded.
9. Ground supplied brand marks against narration, create the editable workspace, import narration, and render the MP4.

Templates cover title, statement, illustration/picture, media, list, sequence, comparison, rule-exception, tool-selection, principle, framework, categories, action, and story families. Media requires a usable image. To use a brand, supply a kit entry such as `{"Acme":{"src":"data:image/svg+xml;base64,...","aliases":["Acme Inc"]}}` and name it in narration. Known brands render their image; unknown brands use a generic glyph. Sources are limited to 16 MiB decoded and 4096 pixels per dimension. SVG rendering uses native resvg with embedded fonts. Embed referenced images as bounded data URIs; external SVG resources are rejected.

## Portable assets and operation API

`explainer.mont` contains the full DAG. `workflows/decisions.mont`, `workflows/chat.mont`, and `workflows/voice.mont` are reusable durable child graphs. `messages.mont` uses `json.stringify` to construct provider messages. Edit grammar, icon catalog, classification criteria, and fill/title/tone/picture/action/story prompts to customize the workflow.

Literal `$asset` and `$json` references resolve inside this collection's root and are frozen before execution; keep references within that root. A singleton `{"$image":"brands/acme.svg"}` in a definition embeds a local SVG/PNG/JPEG/GIF/WebP file as a base64 data URI, subject to root confinement and the shared asset-byte limit. External `--inputs` JSON must already contain data URIs.

Decisions questions require `instructions` and object-valued `criteria`; binary `noul` questions need explicit `true` and `false` criteria. Decision children receive narration `state` and `questions` unchanged and use the selected `clef` adapter. Chat children receive the dedicated prompt as the system message and the complete request as JSON in the user message.

Response collections: fill/general repairs → `scenes`; title repairs → `titles`; colour repairs → `tones`; picture proposals → `pictures`; action/story proposals → `stories`. Return an explicit empty collection when no proposal applies. Provider failures or missing required collections fail the run.

Every native policy operation has the same ports:

```text
explainer.{parse,classify,procedures,boundaries,templates,comparisons,merge,
  voice_script,align,audio_join,request,fill,review,repair,recheck,route,
  proposals,family_repair,merit,compose}
  state:Object, response:Object, policy:Object
  → result:Object, items:Array, state:String, questions:Object, text:String, file:String
```

`result` is the full policy state; `items` contains child-map requests; `state`/`questions` contain decision batches; `text` contains a narration script when present; `file` contains a published narration artifact when present. Provider nodes handle inference; `audio_join` publishes the narration artifact. `request` modes are `fill`, `coverage`, `titles`, `tone`, `relations`, `route`, and `merit`. `policy.json` supplies shared grammar/catalog/classification; graph nodes supply mode, family, or prompt. Provider configuration is reusable across workflows.

## Trust, costs, and attribution

Workflows are **trusted executable instructions**. Local workspace/read/import operations can read files, and provider nodes can send narration, visual descriptions, and prompts to configured endpoints. Review downloaded graphs, scripts, assets, and prompts. Configure endpoints and environment-variable credential references in local provider configuration. See [SECURITY.md](SECURITY.md).

Inference costs depend on document length, selected families, two-pass merit reviews, and bounded repairs. `maxAttempts: 1` disables automatic paid provider retries; explicit repair stages also incur provider costs.

- **Completed run:** repeat the same command and run ID to retrieve its durable result without new provider calls. This is a replay, not a way to regenerate changed inputs or restore a deleted MP4. Keep the journal, project, narration, and rendered output together.
- **Failed run:** failure is terminal. Correcting credentials, funding, or configuration does not retry that ID. Preserve its audit trail, fix the reported cause, then choose a **new run ID and new workspace/output paths** and run again. This is a new paid execution, not a continuation of completed provider work. Do not delete or edit journals to force a retry.
- **Changed document or intentional regeneration:** use a new run ID and fresh workspace/output paths even if the prior run succeeded. Never reuse the example's ID for unrelated inputs.

OpenRouter requires at least $0.50 of funded balance for audio transcription. Check [provider credits](https://openrouter.ai/settings/credits) and current funding requirements before a live run. Spending limits require a funded account balance; HTTP 402 fails the run. Use a restricted development key and spending cap for repeated runs.

Licensed under Apache-2.0. Native generator copyright Nauchu AB. Pipeline prompts and policy criteria are adapted from the user-supplied `mont_motion_pipeline` source archive.
