# Comment cards

Two transparent YouTube-style comments by fictional Nia Brooks (`@niabuilds`) and Elias Reed (`@eliasframes`), with original Mont workflow observations and invented ages/likes. They are illustrative copy, not actual user testimonials or fetched comments.

- Native `scene.tsx`: 960 × 540, 8 seconds, no opaque comment-card backgrounds. Names, metadata, progressively typed body text, counts, Reply labels, timing, and palette remain editable.
- The live [public YouTube comments interface](https://www.youtube.com/watch?v=jNQXAC9IVRw) was inspected: 36 px avatars, Roboto 14/20 comment body, 12/18 age text, Like/Dislike controls, and title-case Reply. This stylized close-up uses native Inter 17 px body, 32 px portraits, title-case Reply, and paired thumb controls. It deliberately preserves the source animation's transparent presentation rather than flattening a page screenshot.
- `assets/nia-brooks.png` and `assets/elias-reed.png` are circular derivatives of the original generated [Nia](../../../assets/template-media/people/nia-brooks.png) and [Elias](../../../assets/template-media/people/elias-reed.png) portraits. Full model/prompt/time/SHA256 provenance is [recorded here](../../../assets/template-media/approved-scenes-provenance.json). Source gradient avatars and initials overlays are removed. No observed person's image/name/message is used. The small retained `source-1.svg` is the platform thumb glyph, rasterized for the installed native renderer; its rotation supplies the dislike control.
- First portrait starts at 0.3 s with a 450 ms back-out pop; text begins at 0.7 s with 24 ms characters, capped at 2.2 s/comment. Second comment follows the first typing duration plus 1.05 s. Actions appear after typing; full reading hold lasts to 7.47 s; cubic-in exit clears by 7.95 s.
- The two-line comments remain shaped text, not bitmap lettering. Metadata positions follow measured author widths. Body strings, authors, ages, likes, and palette are variables; width/height uniformly scale the stage. Source CSS text shadow is intentionally omitted because native glyph-box shadows are not equivalent.

## Native preview and checks

Requires a CLI build with **embedded-image template materialization support**. `0.1.0-preview.16` lacks this capability and fails standalone rendering with missing image resources. Availability of a public CLI release with this support is not claimed here; the gallery GIF remains independently viewable.

Preview independently with `npx usemont templates preview comment-cards`. To create and render without opening an app:

```sh
npx usemont templates create comment-cards ./comment-cards
npx usemont render -C ./comment-cards --output ./comment-cards.mp4 --fps 30
```

The catalog downloads only `scene.tsx`: portraits and thumb glyphs are embedded; fonts use Mont's existing catalog. The [gallery GIF](../../../assets/previews/comment-cards.gif) comes from the approved standalone MP4, with full-frame landscape scaling and Rec.709-to-sRGB conversion; it is not scene media.

Text, card geometry, timing, and colors remain native editable objects. Always-live offscreen image nodes preserve embedded resources; small transparent image nodes use explicit zero-width transparent borders. No screenshot, browser, DOM, CSS, or GSAP renderer is used in the native scene.

## Source and attribution

Derivative timing/layout reference: [Hyperframes Comment cards](https://hyperframes.heygen.com/catalog/blocks/yt-comment-card.md), by HeyGen. Native implementation, generated fictional portraits, and Mont copy are original work. Retained thumb glyph/derivative timing are Apache-2.0, Copyright 2026 HeyGen, Inc.; see [family license](../LICENSE). Platform names identify the visual reference, not affiliation or endorsement.
