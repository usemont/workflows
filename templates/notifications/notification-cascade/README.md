# Notification Cascade

Native editable Mont scene, **540 × 960**, **10 seconds**. Original fictional Mont workflow copy; no baked video, full UI screenshot or bottom-right branding footers.

## Original asset inventory

Original generated `backgrounds/phone-on-desk.png` replaces the source photograph. The warm overlay, chevron and close glyph are native geometry. Messages is an honest identifying service mark; the last card uses the actual Mont icon. All four senders/messages are fictional.

Canonical Mont branding and generated-media prompts, model, timestamps and SHA256 are recorded in [approved-scene provenance](../../../assets/template-media/approved-scenes-provenance.json). Embedded copies are self-contained; no upstream people/photos or HeyGen branding is visible. Identifying service/platform marks remain their owners’ trademarks; no endorsement is implied.


## Editing and rendering limits

Edit the exported `variables` and native nodes in `scene.tsx`. Keep copy close to the supplied lengths: native text uses bounded editable boxes, not browser intrinsic layout. Native fonts use the content-addressed Mont catalog rather than bundling font binaries. Native blur/shadow compositing is an approximation, not browser CSS/PBR equivalence.

## Notification surface reference

The material follows the filmed **iOS 26** notification behavior in the [MacStories review](https://www.macstories.net/stories/ios-and-ipados-26-the-macstories-review/6/) ([actual recording](https://cdn.macstories.net/notification-1757840169857.mp4)) and the supplied Notification Centre reference: strongly diffused wallpaper, smoky neutral tint and directional glass edges—not opaque navy or a uniform white outline. Cards use continuous cubic/arc corners (radius 21, smoothing 0.6), following the MIT-licensed [figma-squircle construction](https://github.com/phamfoo/figma-squircle). Its license is retained in `scene.tsx`.

One existing Mont WGSL pass shades all four cards from one captured, animated wallpaper: 32-tap weighted frosting, restrained edge refraction, a light-facing highlight, darker side edges and a faint inner reflection. The editable `surface` variable supplies the neutral 32%-alpha tint; native text and icons remain separate editable nodes. This is an iOS-inspired approximation, not Apple's proprietary Liquid Glass shader or an exact reproduction of UIKit's continuous corners.

Keep the wallpaper capture in canvas coordinates. Shader `top0`–`top3` uniforms include the same card motion and stack exit offset as their native contents, so the frosting samples the wallpaper beneath each moving card rather than sliding with it. Arrival/exit alpha belongs on the shader's individual surfaces and each separate tint/content node, not their common parent. `../verify.py --scene notification-cascade` renders a native MP4 and checks reduced wood-grain detail through the cards while retaining wallpaper-dependent tones.

Android's [official Material 3 Expressive reference](https://blog.google/products-and-platforms/platforms/android/material-3-expressive-android-wearos-launch/) uses separate tonal/grouped surfaces and shade blur; this template does not claim to implement Android styling. “iOS 26-inspired” identifies the reference version, not the latest OS.

Always-live offscreen image nodes keep embedded resources available throughout native export. Their transparent zero-width image borders prevent native default black quads. These are rendering controls, not visible UI.

## Source and licensing

Native adaptation of the notification-cascade template in the [HyperFrames catalog](https://hyperframes.heygen.com/), Copyright 2026 HeyGen, Inc., Apache License 2.0. Derivative timing/shader/layout code remains; attribution and the full original license are retained below. The [previous source photograph](https://hyperframes.heygen.com/public/catalog/assets/b84738ef758732c7.jpg) is not bundled. Source promotional copy is replaced with the originals listed above. No permission to use upstream trademarks is claimed.

## Native preview and evidence

Requires a CLI build with **embedded-image template materialization support**. `0.1.0-preview.16` lacks this capability and fails standalone rendering with missing image resources. Availability of a public CLI release with this support is not claimed here; the gallery GIF remains independently viewable.

Preview independently with `npx usemont templates preview notification-cascade`. To create and render without opening an app:

```sh
npx usemont templates create notification-cascade ./notification-cascade
npx usemont render -C ./notification-cascade --output ./notification-cascade.mp4 --fps 30
```

The catalog downloads only `scene.tsx`: image data is embedded and fonts use Mont's existing catalog. The [gallery GIF](../../../assets/previews/notification-cascade.gif) comes from the approved standalone MP4, with full-frame portrait scaling and Rec.709-to-sRGB conversion; it is not scene media.

Native render checks (requires `mont`, `ffmpeg`, `ffprobe` and Python with Pillow):

```sh
npx --yes --package=usemont -- python3 templates/notifications/verify.py --scene notification-cascade --output /absolute/path/to/previews
```

<details>
<summary>Upstream Apache2.0 license and copyright notice</summary>

```text
Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to the Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by the Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding any notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   Copyright 2026 HeyGen, Inc.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```

</details>
