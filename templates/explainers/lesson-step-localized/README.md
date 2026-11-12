# Localized lesson step

Explainers · 1920 × 1080 · 14 seconds

A browser-caching lesson with a large, two-part headline and a diagram that downloads a file, stores it locally, then reuses it on the next visit. EN, DE, SV and JA share the same scene and timing.

The visual reference is the type-led teaching in [Hyperframes’ faceless explainer examples](https://hyperframes.heygen.com/guides/faceless-explainer). Copy, diagram and motion here are original Mont TSX.

Previews: [English](../../../assets/previews/lesson-step-localized-en.mp4) · [Deutsch](../../../assets/previews/lesson-step-localized-de.mp4) · [Svenska](../../../assets/previews/lesson-step-localized-sv.mp4) · [日本語](../../../assets/previews/lesson-step-localized-ja.mp4).

## Localization input

`localization.json` is a native template input object containing `locale`, `stepNumber`, `stepCount` and a `messages` locale table. Every visible string, including diagram labels and progress words, comes from that table. The scene looks up `messages[locale]`; the creator does not flatten or rewrite translations into TSX.

The complete input table is saved in the programmatic span’s variables. Change its `locale` to select another translation. Numeric step values are shared across translations.

```sh
python3 create.py /tmp/lesson-en --inputs localization.json --locale en --output /tmp/lesson-en.mp4
python3 create.py /tmp/lesson-de --inputs localization.json --locale de --output /tmp/lesson-de.mp4
python3 create.py /tmp/lesson-sv --inputs localization.json --locale sv --output /tmp/lesson-sv.mp4
python3 create.py /tmp/lesson-ja --inputs localization.json --locale ja --output /tmp/lesson-ja.mp4
python3 check.py
```

Requires Mont CLI 0.1.0-preview.18 or newer and Python 3. Each workspace must be new. Preview.16 lacks the Japanese font payload and cannot render the JA input.

The same JSON is accepted directly by the native template inputs protocol:

```sh
mont templates create lesson-step-localized ./lesson --inputs ./localization.json
mont -C ./lesson render --output ./lesson.mp4
```

Set `locale` in the JSON before creating the workspace. `messages` replaces the complete table when supplied through `--inputs`. To update an existing workspace, dispatch `setProjectValue` to the intended span’s `/capability/state/variables/locale` pointer. This uses native template variables; it does not depend on the cloud editor’s separate data-variants importer.

## Text layout and fonts

EN, DE and SV use embedded Inter; JA uses embedded Zen Kaku Gothic New 400. Font selection is explicit per locale, so output does not depend on system fallback fonts. German and Japanese use authored line breaks and locale-specific type sizes; long translations can adjust `headingSize`, `bodySize` and line breaks without changing the scene.

Text is not automatically shrunk to fit. Keep headings within two lines per block and body text within three lines; review previews after replacing translations. The supplied German copy exercises the longer layout, and the Japanese copy exercises CJK rendering.

`paper`, `ink`, `accent` and `soft` are optional native colour variables. All graphics remain editable vectors. No external media or workstation fonts are required.
