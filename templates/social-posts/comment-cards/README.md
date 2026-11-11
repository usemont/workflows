# Comment cards

Two transparent YouTube-style comments by fictional Nia Brooks (`@niabuilds`) and Elias Reed (`@eliasframes`), with original Mont workflow observations and invented ages/likes. They are illustrative copy, not actual user testimonials or fetched comments.

- `scene.tsx`: 960 × 540, 8 seconds, transparent comment backgrounds. Names, metadata, progressively typed body text, counts, Reply labels, timing, and palette remain editable.
- The stylized close-up uses Inter 17 px body text, 32 px portraits, title-case Reply, and paired thumb controls.
- `assets/nia-brooks.png` and `assets/elias-reed.png` are circular derivatives of generated [Nia](../../../assets/template-media/people/nia-brooks.png) and [Elias](../../../assets/template-media/people/elias-reed.png) portraits. [Image provenance](../../../assets/template-media/approved-scenes-provenance.json). The retained `source-1.svg` supplies the thumb glyph; its rotation supplies the dislike control.
- First portrait starts at 0.3 s with a 450 ms back-out pop; text begins at 0.7 s with 24 ms characters, capped at 2.2 s/comment. Second comment follows the first typing duration plus 1.05 s. Actions appear after typing; full reading hold lasts to 7.47 s; cubic-in exit clears by 7.95 s.
- Metadata positions follow measured author widths. Body strings, authors, ages, likes, and palette are variables; width/height uniformly scale the stage.

## Preview and render

Requires a CLI build with **embedded-image template materialization support**.

Preview independently with `npx usemont templates preview comment-cards`. To create and render without opening an app:

```sh
npx usemont templates create comment-cards ./comment-cards
npx usemont render -C ./comment-cards --output ./comment-cards.mp4 --fps 30
```

Portraits and thumb glyphs are embedded; fonts use Mont's catalog. [Gallery preview](../../../assets/previews/comment-cards.gif).

Keep the offscreen image nodes and their zero-width transparent borders when editing; they preserve embedded resources.

## Source and attribution

Derivative timing/layout reference: [Hyperframes Comment cards](https://hyperframes.heygen.com/catalog/blocks/yt-comment-card.md), by HeyGen. Native implementation, generated fictional portraits, and Mont copy are original work. Retained thumb glyph/derivative timing are Apache-2.0, Copyright 2026 HeyGen, Inc.; see [family license](../LICENSE). Platform names identify the visual reference, not affiliation or endorsement.
