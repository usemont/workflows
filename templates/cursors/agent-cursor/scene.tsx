// Native adaptation of https://hyperframes.heygen.com/catalog/components/agent-cursor.md.
// Copyright 2026 HeyGen, Inc. Apache-2.0; see ../../subscribe/LICENSE.
// Original Mont teaser setup, native first-party cursor and minimum-jerk curved motion.
export default defineSpanScene({
  id: "agent-cursor", width: 1920, height: 1080, durationMs: 4800, editPreviewMs: 4000,
  variables: {
    width: numberVar(1920), height: numberVar(1080),
    agentName: stringVar("Mont AI"), projectName: stringVar("Launch teaser"),
    accent: colorVar("#7568CC"), background: colorVar("#FAFAFB"),
    reading: stringVar("Reading setup"), formatIntent: stringVar("Set vertical format"),
    captionsIntent: stringVar("Turn on captions"), done: stringVar("Ready to create"),
    cursorSrc: stringVar("data:image/png;base64,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"),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width / 1920, vars.height / 1080)),
    clock: computed.number(() => time.loop(4800) * 4800),
    entry: computed.number(() => Math.max(0, Math.min(1, computed.clock / 650))),
    enter: computed.number(() => Math.pow(computed.entry, 3) * (10 + computed.entry * (-15 + 6 * computed.entry))),
    first: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 1250) / 550))),
    move1: computed.number(() => Math.pow(computed.first, 3) * (10 + computed.first * (-15 + 6 * computed.first))),
    second: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2550) / 550))),
    move2: computed.number(() => Math.pow(computed.second, 3) * (10 + computed.second * (-15 + 6 * computed.second))),
    leaving: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 4300) / 500))),
    leave: computed.number(() => Math.pow(computed.leaving, 3) * (10 + computed.leaving * (-15 + 6 * computed.leaving))),
    // Monotone quadratic arcs, reparameterized with zero velocity/acceleration at every dwell.
    pointerX: computed.number(() => computed.clock < 650 ? 2040 * Math.pow(1 - computed.enter, 2) + 2920 * (1 - computed.enter) * computed.enter + 840 * computed.enter * computed.enter : computed.clock < 2550 ? 840 * Math.pow(1 - computed.move1, 2) + 1960 * (1 - computed.move1) * computed.move1 + 1166 * computed.move1 * computed.move1 : computed.clock < 4300 ? 1166 * Math.pow(1 - computed.move2, 2) + 2880 * (1 - computed.move2) * computed.move2 + 1630 * computed.move2 * computed.move2 : 1630 * Math.pow(1 - computed.leave, 2) + 3780 * (1 - computed.leave) * computed.leave + 2040 * computed.leave * computed.leave),
    pointerY: computed.number(() => computed.clock < 650 ? 900 * Math.pow(1 - computed.enter, 2) + 1240 * (1 - computed.enter) * computed.enter + 600 * computed.enter * computed.enter : computed.clock < 2550 ? 600 * Math.pow(1 - computed.move1, 2) + 1070 * (1 - computed.move1) * computed.move1 + 532 * computed.move1 * computed.move1 : computed.clock < 4300 ? 532 * Math.pow(1 - computed.move2, 2) + 1096 * (1 - computed.move2) * computed.move2 + 670 * computed.move2 * computed.move2 : 670 * Math.pow(1 - computed.leave, 2) + 1440 * (1 - computed.leave) * computed.leave + 940 * computed.leave * computed.leave),
    // Measure complete shaped runs, never the currently revealed typing prefix.
    nameWidth: computed.number(() => text.glyphs(vars.agentName, {fontFamily: "Inter", fontWeight: 600, fontSize: 20})[0].totalAdvance),
    separatorWidth: computed.number(() => text.glyphs("·", {fontFamily: "Inter", fontWeight: 400, fontSize: 23})[0].totalAdvance),
    readingWidth: computed.number(() => text.glyphs(vars.reading, {fontFamily: "Inter", fontWeight: 400, fontSize: 20})[0].totalAdvance),
    formatWidth: computed.number(() => text.glyphs(vars.formatIntent, {fontFamily: "Inter", fontWeight: 400, fontSize: 20})[0].totalAdvance),
    captionsWidth: computed.number(() => text.glyphs(vars.captionsIntent, {fontFamily: "Inter", fontWeight: 400, fontSize: 20})[0].totalAdvance),
    doneWidth: computed.number(() => text.glyphs(vars.done, {fontFamily: "Inter", fontWeight: 400, fontSize: 20})[0].totalAdvance),
    intentX: computed.number(() => 66 + computed.nameWidth + computed.separatorWidth),
    resize: computed.number(() => Math.max(0, Math.min(1, (computed.clock - (computed.clock < 2150 ? 850 : computed.clock < 3650 ? 2150 : 3650)) / 180))),
    resizeEase: computed.number(() => Math.pow(computed.resize, 3) * (10 + computed.resize * (-15 + 6 * computed.resize))),
    intentWidth: computed.number(() => computed.clock < 2150 ? computed.readingWidth + (computed.formatWidth - computed.readingWidth) * computed.resizeEase : computed.clock < 3650 ? computed.formatWidth + (computed.captionsWidth - computed.formatWidth) * computed.resizeEase : computed.captionsWidth + (computed.doneWidth - computed.captionsWidth) * computed.resizeEase),
    tagWidth: computed.number(() => computed.intentX + computed.intentWidth + 16),
    leftWidth: computed.number(() => computed.intentX + Math.max(computed.captionsWidth, computed.doneWidth) + 16),
    // Anticipate the second target only if its complete labels will not fit on the right.
    flip: computed.number(() => 1630 + 18 + computed.leftWidth > 1896 ? Math.max(0, Math.min(1, (computed.clock - 2310) / 240)) : 0),
    left: computed.number(() => Math.pow(computed.flip, 3) * (10 + computed.flip * (-15 + 6 * computed.flip))),
    tagX: computed.number(() => Math.max(24, Math.min(1896 - computed.tagWidth, computed.pointerX + 18 - (computed.leftWidth + 36) * computed.left))),
    // Keep the whole side-change below the arrow's 49 px lower extent, not across its hotspot.
    tagY: computed.number(() => Math.max(24, Math.min(1000, computed.pointerY + 54 + 80 * computed.left * (1 - computed.left)))),
    tagOpacity: computed.number(() => Math.min(1, computed.clock / 180) * (1 - Math.max(0, Math.min(1, (computed.clock - 4300) / 220)))),
    press1: computed.number(() => Math.max(0, 1 - Math.abs(computed.clock - 2000) / 110)),
    press2: computed.number(() => Math.max(0, 1 - Math.abs(computed.clock - 3300) / 110)),
    pointerScale: computed.number(() => 1 - 0.06 * Math.max(computed.press1, computed.press2)),
    format: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2000) / 200))),
    formatEase: computed.number(() => Math.pow(computed.format, 3) * (10 + computed.format * (-15 + 6 * computed.format))),
    // One golden-ratio grid for both formats: concentric artwork never reflows independently.
    goldenRatio: computed.number(() => 1.618033988749895),
    previewX: computed.number(() => 268 + 144 * computed.formatEase),
    previewY: computed.number(() => 461 - 50 * computed.formatEase),
    previewWidth: computed.number(() => 504 - 288 * computed.formatEase),
    previewHeight: computed.number(() => 283.5 + 100.5 * computed.formatEase),
    orbitX: computed.number(() => computed.previewX + computed.previewWidth / computed.goldenRatio),
    orbitY: computed.number(() => computed.previewY + computed.previewHeight / Math.pow(computed.goldenRatio, 3)),
    orbitSize: computed.number(() => (283.5 * (1 - computed.formatEase) + 216 * computed.formatEase) / Math.pow(computed.goldenRatio, 2)),
    discSize: computed.number(() => computed.orbitSize / computed.goldenRatio),
    captions: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 3300) / 180))),
    captionsEase: computed.number(() => Math.pow(computed.captions, 3) * (10 + computed.captions * (-15 + 6 * computed.captions))),
    ring1: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2000) / 360))),
    ring2: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 3300) / 360))),
    readyColor: computed.string(() => computed.clock >= 3300 ? "#6F8C78" : "#9595A0"),
  },
  render({vars, computed}) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Image id="cursor-prewarm" src={vars.cursorSrc} x={-4096} y={-4096} width={1} height={1} borderWidth={0} borderColor="transparent" />
      <Group id="letterbox" x={(vars.width - 1920 * computed.scale) / 2} y={(vars.height - 1080 * computed.scale) / 2} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Rect id="app" x={180} y={154} width={1560} height={772} radius={20} fill="#FFFFFF" stroke="#E4E5EA" strokeWidth={1} shadowColor="#242530" shadowOffsetY={12} shadowBlur={40} shadowOpacity={0.065} />
        <Text id="mont" text="mont" x={228} y={187} width={90} height={38} fontFamily="Inter" fontWeight={700} size={28} letterSpacing={-1} color="#303139" />
        <Text id="breadcrumb" text="/  New teaser" x={324} y={194} width={280} height={30} fontFamily="Inter" size={19} color="#8D8E98" />
        <Ellipse id="saved-dot" x={1570} y={203} width={8} height={8} fill="#74A18B" />
        <Text id="saved" text={computed.clock >= 3300 ? "All set" : "Draft"} x={1590} y={192} width={100} height={30} fontFamily="Inter" size={18} color="#787B86" />
        <Rect id="header-rule" x={180} y={244} width={1560} height={1} fill="#ECECF0" />
        <Text id="title" text="Set up your teaser" x={236} y={278} width={900} height={54} fontFamily="Inter" fontWeight={600} size={38} letterSpacing={-0.8} color="#303139" />
        <Text id="subtitle" text="A few details. Then make something move." x={238} y={338} width={900} height={35} fontFamily="Inter" size={21} color="#858792" />
        <Rect id="preview-well" x={236} y={398} width={568} height={410} radius={14} fill="#F5F5F7" stroke="#EBEBEF" strokeWidth={1} />
        <Rect id="preview-video" x={computed.previewX} y={computed.previewY} width={computed.previewWidth} height={computed.previewHeight} radius={8} fill="#EAE6DE" />
        <Ellipse id="preview-orbit" x={computed.orbitX - computed.orbitSize / 2} y={computed.orbitY - computed.orbitSize / 2} width={computed.orbitSize} height={computed.orbitSize} fill="#C9BDD9" />
        <Ellipse id="preview-disc" x={computed.orbitX - computed.discSize / 2} y={computed.orbitY - computed.discSize / 2} width={computed.discSize} height={computed.discSize} fill="#EAE6DE" />
        <Text id="preview-eyebrow" text="MONT / FIELD NOTES" x={296 + 134 * computed.formatEase} y={485 - 41 * computed.formatEase} width={220} height={25} fontFamily="Inter" fontWeight={500} size={12} letterSpacing={1.2} color="#7E746B" />
        <Group id="preview-headline" x={296 + 134 * computed.formatEase} y={548 + 14 * computed.formatEase} width={0} height={0} pivotX={0} pivotY={0} scaleX={1 - 8 / 43 * computed.formatEase} scaleY={1 - 8 / 43 * computed.formatEase}>
          <Text id="preview-line1" text="Make" width={350} height={55} fontFamily="Inter" fontWeight={600} size={43} letterSpacing={-1.3} color="#514A43" />
          <Text id="preview-line2" text="your next" y={48} width={350} height={55} fontFamily="Inter" fontWeight={600} size={43} letterSpacing={-1.3} color="#514A43" />
          <Text id="preview-line3" text="move." y={96} width={350} height={55} fontFamily="Inter" fontWeight={600} size={43} letterSpacing={-1.3} color="#514A43" />
        </Group>
        <Group id="live-captions" x={429} y={725} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.captionsEase}>
          <Rect id="caption-box" width={182} height={36} radius={6} fill="#343139" />
          <Text id="caption-copy" text="Every frame, a new idea." x={5} y={9} width={172} height={20} fontFamily="Inter" fontWeight={500} size={13} align="center" color="#FFFFFF" />
        </Group>
        <Text id="preview-name" text={vars.projectName} x={236} y={828} width={420} height={28} fontFamily="Inter" fontWeight={500} size={20} color="#4D4E58" />
        <Text id="preview-spec" text={computed.clock >= 2000 ? "9:16 · 1080 × 1920" : "16:9 · 1920 × 1080"} x={236} y={864} width={420} height={24} fontFamily="Inter" size={16} color="#92949E" />
        <Rect id="pane-rule" x={855} y={400} width={1} height={470} fill="#ECECF0" />
        <Text id="format-heading" text="Format" x={914} y={410} width={400} height={32} fontFamily="Inter" fontWeight={600} size={23} color="#383943" />
        <Text id="format-help" text="Choose where your story lives." x={914} y={450} width={690} height={28} fontFamily="Inter" size={18} color="#91939D" />
        <Repeat items={[{x: 914, label: "16:9", sub: "Landscape"}, {x: 1082, label: "9:16", sub: "Vertical"}, {x: 1250, label: "1:1", sub: "Square"}]} render={(item, index) => (
          <Group id={"format-" + index} x={item.x} y={484} width={0} height={0} pivotX={0} pivotY={0}>
            <Rect id={"format-card-" + index} width={156} height={96} radius={10} fill={(index === 0 && computed.clock < 2000) || (index === 1 && computed.clock >= 2000) ? "#F1EFFA" : "#FFFFFF"} stroke={(index === 0 && computed.clock < 2000) || (index === 1 && computed.clock >= 2000) ? vars.accent : "#E5E5EB"} strokeWidth={1.5} />
            <Text id={"format-ratio-" + index} text={item.label} x={16} y={17} width={124} height={28} fontFamily="Inter" fontWeight={600} size={22} color={(index === 0 && computed.clock < 2000) || (index === 1 && computed.clock >= 2000) ? vars.accent : "#62646F"} />
            <Text id={"format-name-" + index} text={item.sub} x={16} y={56} width={124} height={23} fontFamily="Inter" size={15} color="#93939F" />
          </Group>
        )} />
        <Rect id="settings-rule" x={914} y={616} width={770} height={1} fill="#EBEBF0" />
        <Text id="captions-heading" text="Captions" x={914} y={643} width={500} height={32} fontFamily="Inter" fontWeight={600} size={23} color="#383943" />
        <Text id="captions-help" text="Make every word part of the story." x={914} y={686} width={620} height={28} fontFamily="Inter" size={18} color="#91939D" />
        <Rect id="captions-toggle" x={1598} y={651} width={64} height={38} radius={19} fill={computed.clock >= 3300 ? vars.accent : "#DADBE2"} />
        <Ellipse id="captions-thumb" x={1603 + 26 * computed.captionsEase} y={656} width={28} height={28} fill="#FFFFFF" shadowColor="#202030" shadowBlur={3} shadowOffsetY={1} shadowOpacity={0.1} />
        <Rect id="ready-strip" x={914} y={762} width={770} height={92} radius={10} fill={computed.clock >= 3300 ? "#F3F6F3" : "#F7F7F9"} />
        <Text id="ready-kicker" text={computed.clock >= 3300 ? "READY FOR YOUR FIRST CUT" : "YOUR STARTING POINT"} x={936} y={779} width={700} height={24} fontFamily="Inter" fontWeight={600} size={13} letterSpacing={1.1} color={computed.readyColor} />
        <Text id="ready-summary" text={computed.clock >= 3300 ? "Vertical teaser · captions on" : computed.clock >= 2000 ? "Vertical teaser · captions off" : "Landscape teaser · captions off"} x={936} y={813} width={700} height={28} fontFamily="Inter" size={20} color="#595C68" />
        <Repeat items={[{x: 1166, y: 532, at: 2000}, {x: 1630, y: 670, at: 3300}]} render={(ring, index) => (
          <Ellipse id={"click-ring-" + index} x={ring.x - 9 - 22 * (index === 0 ? computed.ring1 : computed.ring2)} y={ring.y - 9 - 22 * (index === 0 ? computed.ring1 : computed.ring2)} width={18 + 44 * (index === 0 ? computed.ring1 : computed.ring2)} height={18 + 44 * (index === 0 ? computed.ring1 : computed.ring2)} fill="transparent" stroke={vars.accent} strokeWidth={2} opacity={computed.clock >= ring.at ? (1 - (index === 0 ? computed.ring1 : computed.ring2)) * 0.8 : 0} />
        )} />
        <Group id="agent-label" x={computed.tagX} y={computed.tagY} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.tagOpacity}>
          <Rect id="label-surface" width={computed.tagWidth} height={56} radius={9} fill={vars.accent} shadowColor="#343047" shadowOffsetY={4} shadowBlur={12} shadowOpacity={0.16} />
          <Ellipse id="agent-mark" x={16} y={20} width={16} height={16} fill="#FFFFFF" opacity={computed.clock >= 3650 ? 0 : 1} />
          <Group id="final-check" x={14} y={17} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.clock >= 3650 ? 1 : 0}>
            <Ellipse id="check-disc" width={22} height={22} fill="#FFFFFF" />
            <Rect id="check-short" x={5} y={11} width={7} height={2.7} radius={1} rotation={45} fill={vars.accent} />
            <Rect id="check-long" x={9} y={10} width={11} height={2.7} radius={1} rotation={-45} fill={vars.accent} />
          </Group>
          <Text id="agent-name" text={vars.agentName} x={46} y={16} width={1024} height={28} fontFamily="Inter" fontWeight={600} size={20} color="#FFFFFF" />
          <Text id="agent-separator" text="·" x={56 + computed.nameWidth} y={14} width={64} height={30} fontFamily="Inter" size={23} color="#D7D1F1" />
          <Repeat items={[{copy: vars.reading, start: 100, end: 850}, {copy: vars.formatIntent, start: 850, end: 2150}, {copy: vars.captionsIntent, start: 2150, end: 3650}, {copy: vars.done, start: 3650, end: 4800}]} render={(status, index) => (
            <Group id={"status-" + index} x={computed.intentX} y={16} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.clock >= status.start && computed.clock < status.end ? 1 : 0}>
              <Repeat items={text.glyphs(status.copy, {fontFamily: "Inter", fontWeight: 400, fontSize: 20})} render={(glyph) => (
                <Text id={"typed-" + index + "-" + glyph.index} text={glyph.text} x={glyph.offset} width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 5} height={30} fontFamily={glyph.fontFamily} fontWeight={glyph.fontWeight} size={20} color="#FFFFFF" opacity={glyph.index < Math.floor((computed.clock - status.start) / 18) ? 1 : 0} />
              )} />
            </Group>
          )} />
        </Group>
        <Group id="agent-cursor" x={computed.pointerX} y={computed.pointerY} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.pointerScale} scaleY={computed.pointerScale}>
          <Image id="mont-arrow" src={vars.cursorSrc} x={-9.275362319} y={-6.956521739} width={55.652173913} height={55.652173913} borderWidth={0} borderColor="transparent" />
        </Group>
      </Group>
    </Scene>;
  },
});
