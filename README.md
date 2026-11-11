<p><a href="https://usemont.com"><img src="https://raw.githubusercontent.com/usemont/.github/main/assets/logo.svg" alt="Mont" width="150" height="40"></a></p>

![Mont kinetic typography templates](assets/header.png)

# Mont Workflows

Preview animated templates, customize text and colors, and render videos from your terminal with [Mont CLI](https://github.com/usemont/mont-cli).

[Installation](#installation) · [Try a template](#try-a-template) · [Kinetic typography](#kinetic-typography) · [Product marketing](#product-marketing) · [Mock UI](#mock-ui) · [Notifications](#notifications) · [Social posts](#social-posts) · [Subscribe & follow](#subscribe--follow) · [Cursors](#cursors) · [Transitions](#transitions) · [Tools](#tools) · [Terminal showcase](#terminal-showcase) · [Workflows](#workflows) · [Contribute](#contribute)

## Installation

**Apple Silicon · macOS 14+ · Node.js 22+**

Install [Node.js 22 or newer](https://nodejs.org/en/download), then run:

```sh
npx usemont --help
npx usemont templates list
```

## Try a template

```sh
npx usemont templates preview morph-text
```

Preview opens an editable project at `out/morph-text/project`. Keep the web editor
command running while editing.

Install FFmpeg and ffprobe for media import and MP4 rendering.

## Kinetic typography

<table>
<tr>
<td align="center" valign="top"><a href="templates/kinetic/stretch-row-repeater"><img src="assets/previews/stretch-row-repeater-square.gif" width="260" alt="Elastic typographic ribbons"></a><br><a href="templates/kinetic/stretch-row-repeater"><strong>Stretch row repeater</strong></a><br><code>npx usemont templates preview stretch-row-repeater</code></td>
<td align="center" valign="top"><a href="templates/kinetic/letter-inflate-swell"><img src="assets/previews/letter-inflate-swell-square.gif" width="260" alt="Letters deform through an elastic swell"></a><br><a href="templates/kinetic/letter-inflate-swell"><strong>Letter inflate swell</strong></a><br><code>npx usemont templates preview letter-inflate-swell</code></td>
<td align="center" valign="top"><a href="templates/kinetic/spiral-type"><img src="assets/previews/spiral-type-square.gif" width="260" alt="Orbiting typography follows a spiral"></a><br><a href="templates/kinetic/spiral-type"><strong>Spiral type</strong></a><br><code>npx usemont templates preview spiral-type</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/kinetic/morph-text"><img src="assets/previews/morph-text-square.gif" width="260" alt="Bold phrases merge through a liquid ink morph"></a><br><a href="templates/kinetic/morph-text"><strong>Morph text</strong></a><br><code>npx usemont templates preview morph-text</code></td>
<td align="center" valign="top"><a href="templates/kinetic/bottom-up-letters"><img src="assets/previews/bottom-up-letters-square.gif" width="260" alt="Bottom up letters rise with staggered motion and a gentle overshoot"></a><br><a href="templates/kinetic/bottom-up-letters"><strong>Bottom-up letters</strong></a><br><code>npx usemont templates preview bottom-up-letters</code></td>
<td align="center" valign="top"><a href="templates/kinetic/focus-blur-resolve"><img src="assets/previews/focus-blur-resolve-square.gif" width="260" alt="Focus blur resolve sharpens word by word, then blurs away in order"></a><br><a href="templates/kinetic/focus-blur-resolve"><strong>Focus blur resolve</strong></a><br><code>npx usemont templates preview focus-blur-resolve</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/kinetic/camera-follow-captions"><img src="assets/previews/camera-follow-captions-square.gif" width="260" alt="Gold words pack into a grid as the camera pulls outward"></a><br><a href="templates/kinetic/camera-follow-captions"><strong>Camera follow captions</strong></a><br><code>npx usemont templates preview camera-follow-captions</code></td>
<td align="center" valign="top"><a href="templates/kinetic/caret-swap"><img src="assets/previews/caret-swap-square.gif" width="260" alt="A chartreuse caret wipes between two phrases with smooth motion and letter-by-letter typing"></a><br><a href="templates/kinetic/caret-swap"><strong>Caret swap</strong></a><br><code>npx usemont templates preview caret-swap</code></td>
</tr>
</table>

## Product marketing

<table>
<tr>
<td align="center" valign="top"><a href="templates/product-marketing/subscription-pile-up"><img src="assets/previews/subscription-pile-up.gif" width="360" alt="Eight video-production tool subscription cards pop into a loose constellation with bouncy entrances and gentle continuous wobble"></a><br><a href="templates/product-marketing/subscription-pile-up"><strong>Subscription pile-up</strong></a><br><code>npx usemont templates preview subscription-pile-up</code></td>
<td align="center" valign="top"><a href="templates/product-marketing/punch-through-words"><img src="assets/previews/punch-through-words.gif" width="360" alt="White words stamp onto a widening field of subscription cards with synchronized camera impacts, blur and spring reveals"></a><br><a href="templates/product-marketing/punch-through-words"><strong>Punch-through words</strong></a><br><code>npx usemont templates preview punch-through-words</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/product-marketing/card-vortex"><img src="assets/previews/card-vortex.gif" width="360" alt="Video-tool cards accelerate through a clockwise vortex with individual banking and motion blur before collapsing into a light point"></a><br><a href="templates/product-marketing/card-vortex"><strong>Card vortex</strong></a><br><code>npx usemont templates preview card-vortex</code></td>
<td align="center" valign="top"><a href="templates/product-marketing/tile-wall-reveal"><img src="assets/previews/tile-wall-reveal.gif" width="360" alt="A video-editing panel zooms out and rotates left into an isometric wall of original mini-tools that keeps panning along its vertical axis"></a><br><a href="templates/product-marketing/tile-wall-reveal"><strong>Tile wall reveal</strong></a><br><code>npx usemont templates preview tile-wall-reveal</code></td>
</tr>
</table>

## Mock UI

<table>
<tr>
<td align="center" valign="top"><a href="templates/mock-ui/chatgpt-exchange"><img src="assets/previews/chatgpt-exchange-square.gif" width="360" alt="An original ChatGPT conversation types into a compact composer and streams a reply"></a><br><a href="templates/mock-ui/chatgpt-exchange"><strong>ChatGPT exchange</strong></a><br><code>npx usemont templates preview chatgpt-exchange</code></td>
<td align="center" valign="top"><a href="templates/mock-ui/claude-exchange"><img src="assets/previews/claude-exchange-square.gif" width="360" alt="A Claude-style conversation explains making videos with Mont CLI, with sans-serif responses and monospace commands"></a><br><a href="templates/mock-ui/claude-exchange"><strong>Claude exchange</strong></a><br><code>npx usemont templates preview claude-exchange</code></td>
</tr>
<tr>
<td align="center" valign="top" colspan="2"><a href="templates/mock-ui/ai-chat-reveal"><img src="assets/previews/ai-chat-reveal.gif" width="270" alt="An original mobile AI chat with a reusable keyboard synchronized to typed letters, spaces, Shift and punctuation"></a><br><a href="templates/mock-ui/ai-chat-reveal"><strong>AI chat reveal</strong></a><br><code>npx --yes --package=usemont -- python3 templates/mock-ui/ai-chat-reveal/create.py ./ai-chat</code><br><code>npx usemont edit -C ./ai-chat</code><br>Includes the reusable <a href="templates/components/mobile-keyboard">mobile keyboard scene</a>.</td>
</tr>
</table>

## Notifications

<table>
<tr>
<td align="center" valign="top"><a href="templates/notifications/notification-cascade"><img src="assets/previews/notification-cascade.gif" width="270" alt="Four fictional messages cascade into frosted-glass notification cards with continuous corners over a photographed phone"></a><br><a href="templates/notifications/notification-cascade"><strong>Notification cascade</strong></a><br><code>npx usemont templates preview notification-cascade</code></td>
<td align="center" valign="top"><a href="templates/notifications/slack-notification-ad"><img src="assets/previews/slack-notification-ad.gif" width="270" alt="Fictional Slack messages build into a portrait lock-screen stack over an aspect-preserving studio photograph"></a><br><a href="templates/notifications/slack-notification-ad"><strong>Slack notification ad</strong></a><br><code>npx usemont templates preview slack-notification-ad</code></td>
</tr>
</table>

## Social posts

<table>
<tr>
<td align="center" valign="top"><a href="templates/social-posts/comment-cards"><img src="assets/previews/comment-cards.gif" width="640" alt="Two fictional creator comments pop in with original portraits, progressively typed text, and paired thumb controls"></a><br><a href="templates/social-posts/comment-cards"><strong>Comment cards</strong></a><br><code>npx usemont templates preview comment-cards</code></td>
</tr>
</table>

## Subscribe & follow

<table>
<tr>
<td align="center" valign="top"><a href="templates/subscribe/tiktok-follow"><img src="assets/previews/tiktok-follow.gif" width="270" alt="A Mont TikTok-style follow banner with a button press, Following state and subtle particles"></a><br><a href="templates/subscribe/tiktok-follow"><strong>TikTok follow</strong></a><br><code>npx usemont templates preview tiktok-follow</code></td>
<td align="center" valign="top"><a href="templates/subscribe/instagram-follow"><img src="assets/previews/instagram-follow.gif" width="270" alt="A Mont Instagram-style follow banner with a button press, Following state and subtle particles"></a><br><a href="templates/subscribe/instagram-follow"><strong>Instagram follow</strong></a><br><code>npx usemont templates preview instagram-follow</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/subscribe/creator-follow-card"><img src="assets/previews/creator-follow-card.gif" width="360" alt="A fictional creator card with original abstract artwork, a curved Mont cursor click, a restrained particle burst and gentle residual drift"></a><br><a href="templates/subscribe/creator-follow-card"><strong>Creator follow card</strong></a><br><code>npx usemont templates preview creator-follow-card</code></td>
<td align="center" valign="top"><a href="templates/subscribe/youtube-subscribe"><img src="assets/previews/youtube-subscribe.gif" width="360" alt="A Mont YouTube-style subscribe lower third with the actual Mont icon, a Subscribed state and subtle particles"></a><br><a href="templates/subscribe/youtube-subscribe"><strong>YouTube subscribe</strong></a><br><code>npx usemont templates preview youtube-subscribe</code></td>
</tr>
<tr>
<td align="center" valign="top" colspan="2"><a href="templates/subscribe/x-follow-card"><img src="assets/previews/x-follow-card.gif" width="640" alt="A fictional X creator profile whose follower portraits settle before the Follow button is clicked"></a><br><a href="templates/subscribe/x-follow-card"><strong>X follow card</strong></a><br><code>npx usemont templates preview x-follow-card</code></td>
</tr>
</table>

## Cursors

<table>
<tr>
<td align="center" valign="top"><a href="templates/cursors/drag-and-drop"><img src="assets/previews/drag-and-drop.gif" width="640" alt="A Mont arrow smoothly drags an original video-production task from Ideas to In motion on a neutral board"></a><br><a href="templates/cursors/drag-and-drop"><strong>Drag and drop</strong></a><br><code>npx usemont templates preview drag-and-drop</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/cursors/text-select-cursor"><img src="assets/previews/text-select-cursor.gif" width="640" alt="A Mont cursor selects editable document text, reveals a formatting toolbar and clicks Bold with restrained anticipation and follow-through"></a><br><a href="templates/cursors/text-select-cursor"><strong>Text-select cursor</strong></a><br><code>npx usemont templates preview text-select-cursor</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/cursors/annotate-cursor"><img src="assets/previews/annotate-cursor.gif" width="640" alt="A vector marker circles an editable metric with smooth tapered ink, overshoots the starting angle without connecting, and lifts aside"></a><br><a href="templates/cursors/annotate-cursor"><strong>Annotate cursor</strong></a><br><code>npx usemont templates preview annotate-cursor</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/cursors/agent-cursor"><img src="assets/previews/agent-cursor.gif" width="640" alt="A Mont AI cursor types its intent, selects vertical format and enables captions while concentric golden-ratio artwork reflows smoothly"></a><br><a href="templates/cursors/agent-cursor"><strong>Agent cursor</strong></a><br><code>npx usemont templates preview agent-cursor</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/cursors/website-3d-cursor"><img src="assets/previews/website-3d-cursor.gif" width="480" alt="A cursor guides a light-themed fictional video editor as its panels land in depth with a gently following camera"></a><br><a href="templates/cursors/website-3d-cursor"><strong>Editor 3D assembly</strong></a><br><code>npx usemont templates preview website-3d-cursor</code></td>
</tr>
</table>

## Carousels

<table>
<tr>
<td align="center" valign="top"><a href="templates/carousels/circle-carousel"><img src="assets/previews/circle-carousel.gif" width="260" alt="Original posters circle in perspective, pause for a featured poster, then resume"></a><br><a href="templates/carousels/circle-carousel"><strong>Circle carousel</strong></a><br><code>npx usemont templates preview circle-carousel</code></td>
<td align="center" valign="top"><a href="templates/carousels/orbit-carousel"><img src="assets/previews/orbit-carousel.gif" width="260" alt="Original posters orbit a sphere, pause for a central feature, then resume"></a><br><a href="templates/carousels/orbit-carousel"><strong>Orbit carousel</strong></a><br><code>npx usemont templates preview orbit-carousel</code></td>
<td align="center" valign="top"><a href="templates/carousels/path-carousel"><img src="assets/previews/path-carousel.gif" width="260" alt="Original posters follow a sigmoid path, feature the central poster, then continue"></a><br><a href="templates/carousels/path-carousel"><strong>Path carousel</strong></a><br><code>npx usemont templates preview path-carousel</code></td>
</tr>
</table>

## Transitions

Square A/B previews. Each template includes a standalone scene and a reusable live-frame transition; see [timeline setup and licenses](templates/transitions/README.md).

<table>
<tr>
<td align="center" valign="top"><a href="templates/transitions/cube-flip-transition"><img src="assets/previews/cube-flip-transition.gif" width="360" alt="Cube Flip: square transition from animated scene A to scene B"></a><br><a href="templates/transitions/cube-flip-transition"><strong>Cube Flip</strong></a><br><code>npx usemont templates preview cube-flip-transition</code></td>
<td align="center" valign="top"><a href="templates/transitions/circular-wipe-transition"><img src="assets/previews/circular-wipe-transition.gif" width="360" alt="Circular Wipe: square transition from animated scene A to scene B"></a><br><a href="templates/transitions/circular-wipe-transition"><strong>Circular Wipe</strong></a><br><code>npx usemont templates preview circular-wipe-transition</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/transitions/reverse-circle-wipe-transition"><img src="assets/previews/reverse-circle-wipe-transition.gif" width="360" alt="Reverse Circle Wipe: square transition from animated scene A to scene B"></a><br><a href="templates/transitions/reverse-circle-wipe-transition"><strong>Reverse Circle Wipe</strong></a><br><code>npx usemont templates preview reverse-circle-wipe-transition</code></td>
<td align="center" valign="top"><a href="templates/transitions/dither-dissolve-transition"><img src="assets/previews/dither-dissolve-transition.gif" width="360" alt="Dither Dissolve: square transition from animated scene A to scene B"></a><br><a href="templates/transitions/dither-dissolve-transition"><strong>Dither Dissolve</strong></a><br><code>npx usemont templates preview dither-dissolve-transition</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/transitions/liquid-metal-wipe-transition"><img src="assets/previews/liquid-metal-wipe-transition.gif" width="360" alt="Liquid Metal Wipe: square transition from animated scene A to scene B"></a><br><a href="templates/transitions/liquid-metal-wipe-transition"><strong>Liquid Metal Wipe</strong></a><br><code>npx usemont templates preview liquid-metal-wipe-transition</code></td>
<td align="center" valign="top"><a href="templates/transitions/page-turn-transition"><img src="assets/previews/page-turn-transition.gif" width="360" alt="Page Turn: square transition from animated scene A to scene B"></a><br><a href="templates/transitions/page-turn-transition"><strong>Page Turn</strong></a><br><code>npx usemont templates preview page-turn-transition</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/transitions/object-flyby-wipe-transition"><img src="assets/previews/object-flyby-wipe-transition.gif" width="360" alt="Object Flyby Wipe: square transition from animated scene A to scene B"></a><br><a href="templates/transitions/object-flyby-wipe-transition"><strong>Object Flyby Wipe</strong></a><br><code>npx usemont templates preview object-flyby-wipe-transition</code></td>
<td align="center" valign="top"><a href="templates/transitions/noise-field-dissolve-transition"><img src="assets/previews/noise-field-dissolve-transition.gif" width="360" alt="Noise Field Dissolve: square transition from animated scene A to scene B"></a><br><a href="templates/transitions/noise-field-dissolve-transition"><strong>Noise Field Dissolve</strong></a><br><code>npx usemont templates preview noise-field-dissolve-transition</code></td>
</tr>
</table>

## Tools

[Editable synthetic recorder examples](templates/tools), using the SDK's recording layouts, themes, and cursor styles.

<table>
<tr>
<td align="center" valign="top"><a href="templates/tools/corner-bubble"><img src="assets/previews/recorder-corner-bubble.gif" width="260" alt="Corner bubble: landscape desktop and circular presenter with Studio styling"></a><br><a href="templates/tools/corner-bubble"><strong>Corner bubble</strong></a><br><code>npx usemont tools preview corner-bubble</code></td>
<td align="center" valign="top"><a href="templates/tools/corner-tile"><img src="assets/previews/recorder-corner-tile.gif" width="260" alt="Corner tile: landscape desktop and presenter tile with Paper styling"></a><br><a href="templates/tools/corner-tile"><strong>Corner tile</strong></a><br><code>npx usemont tools preview corner-tile</code></td>
<td align="center" valign="top"><a href="templates/tools/side-by-side"><img src="assets/previews/recorder-side-by-side.gif" width="260" alt="Side by side: equal-height screen and presenter with Glass styling"></a><br><a href="templates/tools/side-by-side"><strong>Side by side</strong></a><br><code>npx usemont tools preview side-by-side</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/tools/presenter"><img src="assets/previews/recorder-presenter.gif" width="260" alt="Presenter layout with desktop, camera and Device styling"></a><br><a href="templates/tools/presenter"><strong>Presenter</strong></a><br><code>npx usemont tools preview presenter</code></td>
<td align="center" valign="top"><a href="templates/tools/edge-bubble"><img src="assets/previews/recorder-edge-bubble.gif" width="260" alt="Edge bubble: landscape desktop and presenter with Bold styling"></a><br><a href="templates/tools/edge-bubble"><strong>Edge bubble</strong></a><br><code>npx usemont tools preview edge-bubble</code></td>
<td align="center" valign="top"><a href="templates/tools/screen-only"><img src="assets/previews/recorder-screen-only.gif" width="260" alt="Screen only: cursor-follow desktop with Ambient styling"></a><br><a href="templates/tools/screen-only"><strong>Screen only</strong></a><br><code>npx usemont tools preview screen-only</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/tools/cinematic"><img src="assets/previews/recorder-cinematic.gif" width="260" alt="Cinematic landscape recording with Paper styling"></a><br><a href="templates/tools/cinematic"><strong>Cinematic</strong></a><br><code>npx usemont tools preview cinematic</code></td>
<td align="center" valign="top"><a href="templates/tools/shorts-stack"><img src="assets/previews/recorder-shorts-stack.gif" width="180" alt="Shorts stack: portrait screen and presenter with Glass styling"></a><br><a href="templates/tools/shorts-stack"><strong>Shorts stack</strong></a><br><code>npx usemont tools preview shorts-stack</code></td>
<td align="center" valign="top"><a href="templates/tools/square-post"><img src="assets/previews/recorder-square-post.gif" width="260" alt="Square post: screen and presenter with Device styling"></a><br><a href="templates/tools/square-post"><strong>Square post</strong></a><br><code>npx usemont tools preview square-post</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/tools/camera-only"><img src="assets/previews/recorder-camera-only.gif" width="260" alt="Camera only: fictional AI presenter with Studio styling"></a><br><a href="templates/tools/camera-only"><strong>Camera only</strong></a><br><code>npx usemont tools preview camera-only</code></td>
</tr>
</table>

## Terminal showcase

<table>
<tr>
<td align="center" valign="top"><a href="templates/showcase/terminal-showcase"><img src="assets/previews/terminal-showcase.gif" width="640" alt="Recorded CLI typing, animated loader, and colored output"></a><br><a href="templates/showcase/terminal-showcase"><strong>Terminal showcase</strong></a><br><code>npx usemont templates preview terminal-showcase</code></td>
</tr>
</table>

## Workflows

- **[Markdown → narrated explainer](workflows/explainer)** — Markdown, scene generation, voice, captions, and MP4. **Experimental · paid APIs.** Follow the [setup](workflows/explainer/README.md) and [security notes](workflows/explainer/SECURITY.md).

## Contribute

Contributions welcome via issues and pull requests. Review unfamiliar workflows before running them: they can send inputs to configured APIs. Never commit credentials or generated projects.

Check carousel motion: `node templates/carousels/verify.mjs`.

[Apache-2.0](LICENSE) · Nauchu AB · [Agent skills](https://github.com/usemont/skills) · [CLI releases](https://github.com/usemont/mont-cli/releases)
