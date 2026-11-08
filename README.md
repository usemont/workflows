<p><a href="https://usemont.com"><img src="https://raw.githubusercontent.com/usemont/.github/main/assets/logo.svg" alt="Mont" width="150" height="40"></a></p>

![Mont kinetic typography templates](assets/header.png)

# Mont Workflows

Ready-to-run video workflows and editable motion templates for [Mont CLI](https://github.com/usemont/mont-cli).

[Try a template](#try-a-template) · [Kinetic typography](#kinetic-typography) · [Mock UI](#mock-ui) · [Notifications](#notifications) · [Social posts](#social-posts) · [Subscribe & follow](#subscribe--follow) · [Cursors](#cursors) · [Terminal showcase](#terminal-showcase) · [Workflows](#workflows) · [Contribute](#contribute)

## Try a template

```sh
# install (macOS)
brew install usemont/tap/mont
# generate preview
mont templates preview stretch-row-repeater
```

On Linux, use the [signed CLI installation guide](https://github.com/usemont/mont-cli/blob/main/docs/install.md#linux).

Saves `preview.mp4` and an editable project to `out/stretch-row-repeater/`.

Explore the latest collection with `mont templates list` and `mont workflows list`.
Inspect settings with `mont templates show <name>` before rendering.

**Image-template CLI requirement:** Image-backed templates require a CLI build with embedded-image template materialization support. `0.1.0-preview.16` does not support their standalone create/preview flow. The GIFs are independently viewable; availability of a public CLI release with this capability is not claimed here.

## Kinetic typography

<table>
<tr>
<td align="center" valign="top"><a href="templates/kinetic/stretch-row-repeater"><img src="assets/previews/stretch-row-repeater-square.gif" width="260" alt="Elastic typographic ribbons"></a><br><a href="templates/kinetic/stretch-row-repeater"><strong>Stretch row repeater</strong></a><br><code>mont templates preview stretch-row-repeater</code></td>
<td align="center" valign="top"><a href="templates/kinetic/letter-inflate-swell"><img src="assets/previews/letter-inflate-swell-square.gif" width="260" alt="Letters deform through an elastic swell"></a><br><a href="templates/kinetic/letter-inflate-swell"><strong>Letter inflate swell</strong></a><br><code>mont templates preview letter-inflate-swell</code></td>
<td align="center" valign="top"><a href="templates/kinetic/spiral-type"><img src="assets/previews/spiral-type-square.gif" width="260" alt="Orbiting typography follows a spiral"></a><br><a href="templates/kinetic/spiral-type"><strong>Spiral type</strong></a><br><code>mont templates preview spiral-type</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/kinetic/morph-text"><img src="assets/previews/morph-text-square.gif" width="260" alt="Bold phrases merge through a liquid ink morph"></a><br><a href="templates/kinetic/morph-text"><strong>Morph text</strong></a><br><code>mont templates preview morph-text</code></td>
<td align="center" valign="top"><a href="templates/kinetic/bottom-up-letters"><img src="assets/previews/bottom-up-letters-square.gif" width="260" alt="Bottom up letters rise with staggered motion and a gentle overshoot"></a><br><a href="templates/kinetic/bottom-up-letters"><strong>Bottom-up letters</strong></a><br><code>mont templates preview bottom-up-letters</code></td>
<td align="center" valign="top"><a href="templates/kinetic/focus-blur-resolve"><img src="assets/previews/focus-blur-resolve-square.gif" width="260" alt="Focus blur resolve sharpens word by word, then blurs away in order"></a><br><a href="templates/kinetic/focus-blur-resolve"><strong>Focus blur resolve</strong></a><br><code>mont templates preview focus-blur-resolve</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/kinetic/camera-follow-captions"><img src="assets/previews/camera-follow-captions-square.gif" width="260" alt="Gold words pack into a grid as the camera pulls outward"></a><br><a href="templates/kinetic/camera-follow-captions"><strong>Camera follow captions</strong></a><br><code>mont templates preview camera-follow-captions</code></td>
<td align="center" valign="top"><a href="templates/kinetic/caret-swap"><img src="assets/previews/caret-swap-square.gif" width="260" alt="A chartreuse caret wipes between two phrases with smooth motion and letter-by-letter typing"></a><br><a href="templates/kinetic/caret-swap"><strong>Caret swap</strong></a><br><code>mont templates preview caret-swap</code></td>
</tr>
</table>

## Mock UI

<table>
<tr>
<td align="center" valign="top"><a href="templates/mock-ui/chatgpt-exchange"><img src="assets/previews/chatgpt-exchange-square.gif" width="360" alt="An original ChatGPT conversation types into a compact composer and streams a reply"></a><br><a href="templates/mock-ui/chatgpt-exchange"><strong>ChatGPT exchange</strong></a><br><code>mont templates preview chatgpt-exchange</code></td>
<td align="center" valign="top"><a href="templates/mock-ui/claude-exchange"><img src="assets/previews/claude-exchange-square.gif" width="360" alt="A Claude-style conversation explains making videos with Mont CLI, with sans-serif responses and monospace commands"></a><br><a href="templates/mock-ui/claude-exchange"><strong>Claude exchange</strong></a><br><code>mont templates preview claude-exchange</code></td>
</tr>
<tr>
<td align="center" valign="top" colspan="2"><a href="templates/mock-ui/ai-chat-reveal"><img src="assets/previews/ai-chat-reveal.gif" width="270" alt="An original mobile AI chat with a reusable keyboard synchronized to typed letters, spaces, Shift and punctuation"></a><br><a href="templates/mock-ui/ai-chat-reveal"><strong>AI chat reveal</strong></a><br><code>python3 templates/mock-ui/ai-chat-reveal/create.py ./ai-chat</code><br><code>mont edit -C ./ai-chat</code><br>Includes the reusable <a href="templates/components/mobile-keyboard">mobile keyboard scene</a>.</td>
</tr>
</table>

## Notifications

<table>
<tr>
<td align="center" valign="top"><a href="templates/notifications/notification-cascade"><img src="assets/previews/notification-cascade.gif" width="270" alt="Four fictional messages cascade into frosted-glass notification cards with continuous corners over a photographed phone"></a><br><a href="templates/notifications/notification-cascade"><strong>Notification cascade</strong></a><br><code>mont templates preview notification-cascade</code></td>
<td align="center" valign="top"><a href="templates/notifications/slack-notification-ad"><img src="assets/previews/slack-notification-ad.gif" width="270" alt="Fictional Slack messages build into a portrait lock-screen stack over an aspect-preserving studio photograph"></a><br><a href="templates/notifications/slack-notification-ad"><strong>Slack notification ad</strong></a><br><code>mont templates preview slack-notification-ad</code></td>
</tr>
</table>

## Social posts

<table>
<tr>
<td align="center" valign="top"><a href="templates/social-posts/comment-cards"><img src="assets/previews/comment-cards.gif" width="640" alt="Two fictional creator comments pop in with original portraits, progressively typed text, and paired thumb controls"></a><br><a href="templates/social-posts/comment-cards"><strong>Comment cards</strong></a><br><code>mont templates preview comment-cards</code></td>
</tr>
</table>

## Subscribe & follow

<table>
<tr>
<td align="center" valign="top"><a href="templates/subscribe/tiktok-follow"><img src="assets/previews/tiktok-follow.gif" width="270" alt="A Mont TikTok-style follow banner with a button press, Following state and subtle particles"></a><br><a href="templates/subscribe/tiktok-follow"><strong>TikTok follow</strong></a><br><code>mont templates preview tiktok-follow</code></td>
<td align="center" valign="top"><a href="templates/subscribe/instagram-follow"><img src="assets/previews/instagram-follow.gif" width="270" alt="A Mont Instagram-style follow banner with a button press, Following state and subtle particles"></a><br><a href="templates/subscribe/instagram-follow"><strong>Instagram follow</strong></a><br><code>mont templates preview instagram-follow</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/subscribe/creator-follow-card"><img src="assets/previews/creator-follow-card.gif" width="360" alt="A fictional creator card with original abstract artwork, a curved Mont cursor click, a restrained particle burst and gentle residual drift"></a><br><a href="templates/subscribe/creator-follow-card"><strong>Creator follow card</strong></a><br><code>mont templates preview creator-follow-card</code></td>
<td align="center" valign="top"><a href="templates/subscribe/youtube-subscribe"><img src="assets/previews/youtube-subscribe.gif" width="360" alt="A Mont YouTube-style subscribe lower third with the actual Mont icon, a Subscribed state and subtle particles"></a><br><a href="templates/subscribe/youtube-subscribe"><strong>YouTube subscribe</strong></a><br><code>mont templates preview youtube-subscribe</code></td>
</tr>
<tr>
<td align="center" valign="top" colspan="2"><a href="templates/subscribe/x-follow-card"><img src="assets/previews/x-follow-card.gif" width="640" alt="A fictional X creator profile whose follower portraits settle before the Follow button is clicked"></a><br><a href="templates/subscribe/x-follow-card"><strong>X follow card</strong></a><br><code>mont templates preview x-follow-card</code></td>
</tr>
</table>

## Cursors

<table>
<tr>
<td align="center" valign="top"><a href="templates/cursors/drag-and-drop"><img src="assets/previews/drag-and-drop.gif" width="640" alt="A Mont arrow smoothly drags an original video-production task from Ideas to In motion on a neutral board"></a><br><a href="templates/cursors/drag-and-drop"><strong>Drag and drop</strong></a><br><code>mont templates preview drag-and-drop</code></td>
</tr>
<tr>
<td align="center" valign="top"><a href="templates/cursors/text-select-cursor"><img src="assets/previews/text-select-cursor.gif" width="640" alt="A Mont cursor selects editable document text, reveals a formatting toolbar and clicks Bold with restrained anticipation and follow-through"></a><br><a href="templates/cursors/text-select-cursor"><strong>Text-select cursor</strong></a><br><code>mont templates preview text-select-cursor</code></td>
</tr>
</table>

## Terminal showcase

<table>
<tr>
<td align="center" valign="top"><a href="templates/showcase/terminal-showcase"><img src="assets/previews/terminal-showcase.gif" width="640" alt="Recorded CLI typing, animated loader, and colored output"></a><br><a href="templates/showcase/terminal-showcase"><strong>Terminal showcase</strong></a><br><code>mont templates preview terminal-showcase</code></td>
</tr>
</table>

## Workflows

- **[Markdown → narrated explainer](workflows/explainer)** — Markdown, scene generation, voice, captions, and MP4. **Experimental: visual review incomplete; uses paid APIs.** Read its [setup](workflows/explainer/README.md) and [security notes](workflows/explainer/SECURITY.md) first.

## Contribute

Contributions welcome via issues and pull requests. Review unfamiliar workflows before running them: they can send inputs to configured APIs. Never commit credentials or generated projects.

[Apache-2.0](LICENSE) · Nauchu AB · [Agent skills](https://github.com/usemont/skills) · [CLI releases](https://github.com/usemont/mont-cli/releases)
