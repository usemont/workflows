# Markdown → editable Mont project + narrated MP4

This collection is an ordinary durable Mont workflow, not a Python runner, HTML renderer, shell pipeline, or single universal model prompt. It requires a Mont CLI with `explainer.*`, `provider.*`, `workflow.map`, `workflow.script`, `workflow.select`, and `workflow.read-text` operations. The public template engine lives at `../../templates/engine/native`; Mont compiles its scene descriptions into editable native ProgrammaticSpans.

## Run the example

From this directory, review `explainer.mont`, the child definitions, and every prompt before running. Configure adapters in a **local** copy of `config.example.json`. Its endpoints and model IDs reproduce the archive's actual provider choices, not substitute models hidden behind familiar names:

| Adapter | Protocol and OpenRouter endpoint | Archive model |
| --- | --- | --- |
| `clef` | decisions, `/api/alpha/decisions` | `cloudflare/clef` |
| `jev` | decisions, `/api/alpha/decisions` | `typesafe/jev-1.14` |
| `haiku` | chat, `/api/v1/chat/completions` | `anthropic/claude-haiku-5.5` |
| `voice` | speech, `/api/v1/audio/speech` | `hexgrad/kokoro-82m`, voice `af_heart` |
| `transcription` | transcription, `/api/v1/audio/transcriptions` | `openai/whisper-large-v3` |

These are source configurations, **not a claim that every model is currently served or that this account has access**. In particular, the archive's literal Haiku model ID must be checked against the provider's current catalog before a live run. If a model or decisions endpoint is unavailable, stop and explicitly configure an available implementation of that protocol; the workflow neither relabels another model as Haiku/Jev nor silently substitutes one. Transcription uses the archive's JSON `input_audio` request with word timestamps, not multipart upload. Missing timestamps and speech failures fail the run.

The default decision input `clef` selects adapter `clef`; set `"clef":"jev"` in inputs to select the real Jev decisions adapter. The `fill` input defaults to `haiku`, and the separate `haiku` input selects family proposal chat. Adapter IDs and these input selections are configurable independently. The source's direct Typesafe alternative (`https://api.typesafe.ai/v1/systemone`, model `jev-latest`, credential reference `TYPESAFE_API_KEY`) can also be configured as a decisions adapter; it is not a new wire protocol.

Make `OPENROUTER_API_KEY` available in the environment through your usual secret manager. Never put key values in a definition, inputs file, prompt, or tracked config. Credentials are not bundled, and models are not local services.

```sh
mont --json -C "$PWD" workflow run "$PWD/explainer.mont" \
  --inputs "$PWD/inputs.example.json" \
  --config "$PWD/config.example.json" \
  --run-id explainer-example
```

`markdown` is a UTF-8 **filename**, not inline Markdown. Relative `markdown`, `workspace`, and MP4 `output` paths resolve from the CLI's `-C` directory, not from the definition or the generated workspace. For another document, use an absolute Markdown filename, choose a new workspace directory and MP4 filename, and use a unique run ID. The `.mont-workspace` suffix names an editable directory, not a file format requirement; an existing workspace path is rejected rather than overwritten. The default 1920×1080, 30 fps settings are input defaults, not renderer constraints. `theme` accepts the native engine's themes. `brands` defaults to an empty object; use absolute local `src` paths for brand/image assets to avoid working-directory ambiguity. Provider speech/transcription artifacts are published by the workflow host; do not supply a fabricated audio path or estimated word timings.

Successful outputs include the native `video` description, durable narration WAV path, editable workspace path, render result, and full `policy` state containing review/repair and selection evidence. The workflow only reports success when native project creation and MP4 rendering finish; narration errors do not become silent video. Provider access and rendering are real dependencies. The public scene generator is included in this repository; the proprietary Mont SDK compiler and renderer are not. Check the [collection's CLI availability warning](../../README.md) before installing a binary.

## Native execution verification, not visual acceptance

**Visual acceptance is rejected.** The current native templates lose source theme, typography, icon, visual-detail, and motion fidelity. A successful MP4 and passing structural tests are not evidence of production-ready visuals. A compatible CLI release remains on hold until source-reference comparisons pass.

On 2026-10-30, a funded live run completed the full native workflow with the unchanged example configuration: `cloudflare/clef` decisions, `anthropic/claude-haiku-5.5` chat, `hexgrad/kokoro-82m` speech with `af_heart`, and `openai/whisper-large-v3` word timestamps. Paid review, repair, family proposal, and two-pass merit stages completed; the result included an editable native Mont project and a 28.325-second H.264/AAC MP4 at 1920×1080, 30 fps. The same accepted scene/audio payload was also rendered natively at 1080×1920; this was a vertical re-render, not a second paid inference run. Jev is configured and covered by adapter tests, but was not exercised in this live acceptance.

Replaying the completed run ID without provider credentials returned the durable result successfully, and the existing MP4's SHA-256 stayed unchanged. This verifies execution and replay only, not visual fidelity, support in older public CLI packages, or continued provider/model availability.

## Stages and reusable parts

1. Read and parse Markdown into narration blocks. Explicit Markdown headings and lists remain structural boundaries; classification decisions distinguish body, lesson/section/subsection headings, recap, list introductions, and list members.
2. Identify actual procedures versus unordered lists, ask only ambiguous scene gaps, preserve hard heading boundaries, merge short scenes only at soft boundaries, and split long scenes only at approved gaps. Unsplittable long scenes carry a reason in policy state.
3. Select templates using the source's choice plus ten feature decisions and precedence; verify near-miss comparisons. Native policy asks about all possible contiguous statement/comparison merge intervals, including mixed original template intervals, and caches each interval's complete template/feature/comparison decisions. It then repeatedly applies the **leftmost eligible** adjacent merge and reselects until no merge is eligible. Thus a reclassified merge can unlock another merge without an unbounded paid inference loop. A merge must have the same current template, section, at most 30 estimated seconds, and probability ≥ 0.5. Ties therefore resolve deterministically left to right; `mergeAudit` records the sequence.
4. Produce one spoken script per scene, synthesize it, obtain real timestamped words, align script tokens to those words, and build captions. Missing script tokens are interpolated only between actual speech anchors; empty/unmatched transcription is an error. Join validated, equal-format RIFF WAV files with 0.25 s leading and 0.45 s trailing PCM silence per scene at the actual sample rate. All subsequent timing uses that audio, not word-count estimates.
5. Fill the selected native template with its dedicated grammar. Enforce scene identity/timing, variants, counts, text limits, icon vocabulary, details grounded in exact narration, unique item reveals, focus/card limits, and absolute timed fields.
6. Review coverage of each narration unit, title specificity, semantic colour, and structured relationships. Each deficient scene gets at most one targeted repair and an independent recheck. A repair cannot worsen an existing check. Invalid/regressing repairs retain the original with recorded evidence; unsupported structured relationships that cannot be repaired fail actionably instead of being disguised as a valid diagram.
7. Independently route and propose picture diagrams, document/message actions, and relational stories with separate source prompts. Validate exact phrase anchors, closed layouts/icons, structural steps/emphasis/callouts, actor handoffs and line operations, story board membership and relation state machines, timing and enumeration preservation. A malformed proposal receives one bounded repair; an explicit empty proposal is valid. Rejected optional proposals retain the validated native base scene, not a generic card.
8. Ask two independent merit passes, require grounded fit, action distinctness and every story relation, enforce action/story density and action shape diversity, then compose with **story > action > picture** precedence. Stories replace their whole scene; actions and pictures must cover at least 80% of the scene. There are no arbitrary cut-in fragments or consecutive full action scenes. Spotlight lists may contain multiple grounded pictures. Preserve the original validated scene whenever no proposal passes.
9. Ground brand marks against narration and an explicitly supplied kit, create the editable native workspace, import the narration, and render a narrated MP4.

The grammar covers title, statement, illustration/picture, media, list, sequence, comparison, rule-exception, tool-selection, principle, framework, categories, action, and story families and their declared variants. Selection is not forced to exercise every family in one lesson. Media requires a genuine usable image; no automatic stock-image search, invented logo, screenshot substitute, or silent placeholder is provided. To use a brand, provide a kit entry such as `{"Acme":{"src":"/absolute/assets/acme.svg","aliases":["Acme Inc"]}}` and actually name it in narration. Native import resolves the supplied asset; the workflow does not fetch brand marks from the web.

## Portable assets and operation API

`explainer.mont` contains the full DAG. `workflows/decisions.mont`, `workflows/chat.mont`, and `workflows/voice.mont` are reusable durable child graphs. `messages.mont` uses real Mont-language `json.stringify` to construct provider messages. Grammar, icon catalog, classification criteria, and separate fill/title/tone/picture/action/story prompts are public editable assets. Literal `$asset` and `$json` references resolve inside this collection's root and are frozen before execution; do not introduce `..` references.

Every decisions question includes `instructions` and an object-valued `criteria`. Binary `noul` questions provide explicit `true` and `false` criteria, as required by the current OpenRouter API. Child decisions receive their batch's narration `state` and `questions` unchanged; their adapter comes from the selected `clef` input. Chat children receive the dedicated source prompt as the system message and the complete request as JSON in the user message. Fill and general repairs return `scenes`; title-only repairs return `titles`; colour-only repairs return `tones`. Picture proposals return `pictures`, while both action and story proposals return the source's `stories` collection (including an explicit empty collection when none apply). The workflow does not substitute an invented answer if a provider fails or omits a required collection.

Every native policy operation has the same ports:

```text
explainer.{parse,classify,procedures,boundaries,templates,comparisons,merge,
  voice_script,align,audio_join,request,fill,review,repair,recheck,route,
  proposals,family_repair,merit,compose}
  state:Object, response:Object, policy:Object
  → result:Object, items:Array, state:String, questions:Object, text:String, file:String
```

`result` is the full pure policy state, `items` are child-map requests, `state`/`questions` are decision batches, `text` is a narration script when present, and `file` is a published narration artifact when present. Inference remains in explicit provider nodes; policy transforms contain no provider HTTP. Only `audio_join` publishes an external artifact. `request` modes are `fill`, `coverage`, `titles`, `tone`, `relations`, `route`, and `merit`. `policy.json` supplies the shared grammar/catalog/classification; individual graph nodes supply mode, family, or prompt. Provider configuration is reusable independently of this workflow.

## Trust, costs, and attribution

Workflows are **trusted executable instructions, not a sandbox**. Local workspace/read/import operations can read files, and provider nodes can send narration, visual descriptions, and prompt content to configured endpoints. Review downloaded `.mont` graphs, scripts, assets, and prompts. Only local provider configuration selects endpoints and environment-variable credential references; a workflow does not set an endpoint or carry an inline key. See [SECURITY.md](SECURITY.md).

Inference costs depend on document length, selected families, two-pass merit reviews, and bounded repairs. `maxAttempts: 1` disables automatic paid provider retries; repair nodes are explicit, visible paid stages, not hidden transport retries.

- **Completed run:** repeat the same command and run ID to retrieve its durable result without new provider calls. This is a replay, not a way to regenerate changed inputs or restore a deleted MP4. Keep the journal, project, narration, and rendered output together.
- **Failed run:** failure is terminal. Correcting credentials, funding, or configuration does not retry that ID. Preserve its audit trail, fix the reported cause, then choose a **new run ID and new workspace/output paths** and run again. This is a new paid execution, not a continuation of completed provider work. Do not delete or edit journals to force a retry.
- **Changed document or intentional regeneration:** use a new run ID and fresh workspace/output paths even if the prior run succeeded. Never reuse the example's ID for unrelated inputs.

Provider spending limits do not fund an account balance. OpenRouter currently requires at least $0.50 of funded balance for audio transcription; an HTTP 402 is an actionable provider failure, not permission to invent captions or silently omit narration. Check [provider credits](https://openrouter.ai/settings/credits) before a live run. Keep a separate restricted development key and spending cap for repeated acceptance runs.

Licensed under Apache-2.0. Native generator copyright Nauchu AB. Pipeline prompts and policy criteria are adapted from the user-supplied `mont_motion_pipeline` source archive; no credentials, Python orchestration, or private Mont SDK/CLI implementation is included in this public collection.
