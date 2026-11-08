// Native adaptation of https://hyperframes.heygen.com/catalog/components/cursor-text-select.
// Copyright 2026 HeyGen, Inc. Apache-2.0; see ../../subscribe/LICENSE.
// Original Mont document copy. First-party Mont arrow/text artwork; native proportional glyph metrics.
export default defineSpanScene({
  id: "text-select-cursor", width: 1920, height: 1080, durationMs: 5600, editPreviewMs: 4300,
  variables: {
    width: numberVar(1920), height: numberVar(1080),
    title: stringVar("A stronger opening"), beforeText: stringVar("First, "),
    phrase: stringVar("shape the opening beat"), afterText: stringVar(" in Mont."),
    bodyLine1: stringVar("Use a clear gesture, then give the moment room to breathe."),
    bodyLine2: stringVar("Keep the cut focused. Let the story do the work."),
    meta: stringVar("PRODUCTION NOTES"), footer: stringVar("Draft · edited just now"),
    background: colorVar("#FAFAFB"), accent: colorVar("#7871D8"),
    arrowSrc: stringVar("data:image/png;base64,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"), beamSrc: stringVar("data:image/png;base64,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"),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width / 1920, vars.height / 1080)),
    clock: computed.number(() => time.loop(5600) * 5600),
    // Widths are shaped by Mont's bundled Inter font, not character counts or browser canvas.
    spaceWidth: computed.number(() => text.glyphs("x x", {fontFamily: "Inter", fontWeight: 400, fontSize: 32})[1].advance),
    prefixWidth: computed.number(() => text.glyphs(vars.beforeText + " ", {fontFamily: "Inter", fontWeight: 400, fontSize: 32})[0].totalAdvance - computed.spaceWidth),
    regularWidth: computed.number(() => text.glyphs(vars.phrase, {fontFamily: "Inter", fontWeight: 400, fontSize: 32})[0].totalAdvance),
    boldWidth: computed.number(() => text.glyphs(vars.phrase, {fontFamily: "Inter", fontWeight: 700, fontSize: 32})[0].totalAdvance),
    phraseWidth: computed.number(() => computed.clock < 3150 ? computed.regularWidth : computed.boldWidth),
    startX: computed.number(() => 407 + computed.prefixWidth),
    boldX: computed.number(() => computed.startX + computed.regularWidth / 2 - 94),
    // Authored poses: arrive, anticipate, drag, release, format, then move aside.
    arrival: computed.number(() => Math.max(0, Math.min(1, computed.clock / 620))),
    arrive: computed.number(() => Math.pow(computed.arrival, 3) * (10 + computed.arrival * (-15 + 6 * computed.arrival))),
    anticipation: computed.number(() => Math.pow(Math.sin(Math.PI * Math.max(0, Math.min(1, (computed.clock - 620) / 220))), 2)),
    drag: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 900) / 1120))),
    dragEase: computed.number(() => Math.pow(computed.drag, 3) * (10 + computed.drag * (-15 + 6 * computed.drag))),
    toolbar: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2160) / 300))),
    toolbarEase: computed.number(() => Math.pow(computed.toolbar, 3) * (10 + computed.toolbar * (-15 + 6 * computed.toolbar))),
    toolbarLift: computed.number(() => 1 + 3.2 * Math.pow(computed.toolbar - 1, 3) + 2.2 * Math.pow(computed.toolbar - 1, 2)),
    approach: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2500) / 530))),
    approachEase: computed.number(() => Math.pow(computed.approach, 3) * (10 + computed.approach * (-15 + 6 * computed.approach))),
    aside: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 3380) / 560))),
    asideEase: computed.number(() => Math.pow(computed.aside, 3) * (10 + computed.aside * (-15 + 6 * computed.aside))),
    idle: computed.number(() => Math.max(0, computed.clock - 3940)),
    // Each travel has its own control point. Zero endpoint velocity preserves precise click dwells.
    pointerX: computed.number(() => computed.clock < 620 ? 1740 * Math.pow(1 - computed.arrive, 2) + 2320 * (1 - computed.arrive) * computed.arrive + computed.startX * computed.arrive * computed.arrive : computed.clock < 2500 ? computed.startX - 8 * computed.anticipation + computed.regularWidth * computed.dragEase : computed.clock < 3380 ? (computed.startX + computed.regularWidth) * Math.pow(1 - computed.approachEase, 2) + 2 * (computed.startX + computed.regularWidth - 18) * (1 - computed.approachEase) * computed.approachEase + computed.boldX * computed.approachEase * computed.approachEase : computed.boldX * Math.pow(1 - computed.asideEase, 2) + 2280 * (1 - computed.asideEase) * computed.asideEase + 1490 * computed.asideEase * computed.asideEase + 2 * Math.sin(computed.idle / 620) * (1 - Math.exp(-computed.idle / 400))),
    pointerY: computed.number(() => computed.clock < 620 ? 830 * Math.pow(1 - computed.arrive, 2) + 1000 * (1 - computed.arrive) * computed.arrive + 525 * computed.arrive * computed.arrive : computed.clock < 2500 ? 525 : computed.clock < 3380 ? 525 * Math.pow(1 - computed.approachEase, 2) + 832 * (1 - computed.approachEase) * computed.approachEase + 442 * computed.approachEase * computed.approachEase : 442 * Math.pow(1 - computed.asideEase, 2) + 1200 * (1 - computed.asideEase) * computed.asideEase + 735 * computed.asideEase * computed.asideEase + 1.5 * Math.sin(computed.idle / 780) * (1 - Math.exp(-computed.idle / 400))),
    press: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 840) / 60))),
    release: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2070) / 90))),
    boldPress: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 3080) / 70))),
    boldRelease: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 3150) / 170))),
    held: computed.number(() => computed.clock < 2500 ? computed.press - computed.release : computed.boldPress - computed.boldRelease),
    pointerScale: computed.number(() => 1 - 0.04 * computed.held),
    pointerStrain: computed.number(() => 1 + 0.025 * computed.held),
    pointerTilt: computed.number(() => computed.clock < 620 ? -3 * Math.sin(Math.PI * computed.arrive) : computed.clock < 2500 ? 0 : computed.clock < 3380 ? -2 * Math.sin(Math.PI * computed.approachEase) : 2.5 * Math.sin(Math.PI * computed.asideEase)),
    buttonScale: computed.number(() => 1 - 0.045 * (computed.boldPress - computed.boldRelease) + 0.012 * Math.sin(Math.PI * computed.boldRelease)),
    click: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 3150) / 320))),
    boldColor: computed.string(() => computed.clock >= 3150 ? "#FFFFFF" : "#3C3E48"),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Image id="arrow-prewarm" src={vars.arrowSrc} x={-4096} y={-4096} width={1} height={1} borderWidth={0} borderColor="transparent" />
      <Image id="beam-prewarm" src={vars.beamSrc} x={-4096} y={-4096} width={1} height={1} borderWidth={0} borderColor="transparent" />
      <Group id="letterbox" x={(vars.width - 1920 * computed.scale) / 2} y={(vars.height - 1080 * computed.scale) / 2} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Rect id="document" x={290} y={208} width={1340} height={666} radius={18} fill="#FFFFFF" stroke="#E5E6EB" strokeWidth={1} shadowColor="#252632" shadowBlur={30} shadowOffsetY={12} shadowOpacity={0.055} />
        <Text id="document-meta" text={vars.meta} x={407} y={265} width={1020} height={28} fontFamily="Inter" fontWeight={600} size={17} letterSpacing={1.6} lineHeight={1.2} fit="none" color="#90929D" />
        <Text id="document-title" text={vars.title} x={407} y={317} width={1100} height={67} fontFamily="Inter" fontWeight={600} size={48} letterSpacing={-1.2} lineHeight={1.2} fit="none" color="#292A32" />
        {/* Adjacent measured glyph cells form a proportional, character-by-character selection. */}
        <Repeat items={text.glyphs(vars.phrase, {fontFamily: "Inter", fontWeight: 400, fontSize: 32})} render={(glyph) => (
          <Rect id={"selection-cell-" + glyph.index} x={computed.startX + glyph.offset} y={500} width={glyph.advance + 0.1} height={45} fill={vars.accent} opacity={computed.clock >= 900 && computed.clock < 3150 && computed.regularWidth * computed.dragEase >= glyph.offset + glyph.advance / 2 ? 0.21 : 0} />
        )} />
        <Rect id="bold-selection" x={computed.startX} y={500} width={computed.boldWidth} height={45} fill={vars.accent} opacity={computed.clock >= 3150 ? 0.17 : 0} />
        <Text id="paragraph-prefix" text={vars.beforeText} x={407} y={502} width={1106} height={50} fontFamily="Inter" fontWeight={400} size={32} lineHeight={1.4} fit="none" color="#555762" />
        <Text id="selected-phrase-regular" text={vars.phrase} x={computed.startX} y={502} width={1106} height={50} fontFamily="Inter" fontWeight={400} size={32} lineHeight={1.4} fit="none" color="#555762" opacity={computed.clock < 3150 ? 1 : 0} />
        <Text id="selected-phrase-bold" text={vars.phrase} x={computed.startX} y={502} width={1106} height={50} fontFamily="Inter" fontWeight={700} size={32} lineHeight={1.4} fit="none" color="#292A32" opacity={computed.clock >= 3150 ? 1 : 0} />
        <Text id="paragraph-suffix" text={vars.afterText} x={computed.startX + computed.phraseWidth} y={502} width={600} height={50} fontFamily="Inter" fontWeight={400} size={32} lineHeight={1.4} fit="none" color="#555762" />
        <Text id="paragraph-line-2" text={vars.bodyLine1} x={407} y={568} width={1100} height={50} fontFamily="Inter" fontWeight={400} size={32} lineHeight={1.4} fit="none" color="#555762" />
        <Text id="paragraph-line-3" text={vars.bodyLine2} x={407} y={634} width={1100} height={50} fontFamily="Inter" fontWeight={400} size={32} lineHeight={1.4} fit="none" color="#555762" />
        <Rect id="document-rule" x={407} y={757} width={1106} height={1} fill="#EEEFF2" />
        <Ellipse id="draft-status" x={407} y={796} width={7} height={7} fill="#A6A8B2" />
        <Text id="document-footer" text={vars.footer} x={427} y={786} width={1020} height={30} fontFamily="Inter" fontWeight={400} size={18} lineHeight={1.3} fit="none" color="#93959F" />
        <Group id="format-toolbar" x={computed.startX + computed.regularWidth / 2 - 132} y={428 - 18 * computed.toolbarLift} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.toolbarEase}>
          <Rect id="toolbar-surface" width={264} height={64} radius={12} fill="#FFFFFF" stroke="#E0E1E7" strokeWidth={1} shadowColor="#242532" shadowBlur={18} shadowOffsetY={7} shadowOpacity={0.1} />
          <Group id="bold-press" x={38} y={32} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.buttonScale} scaleY={computed.buttonScale}>
            <Rect id="bold-button" x={-26} y={-24} width={52} height={48} radius={7} fill={computed.clock >= 3150 ? vars.accent : computed.clock >= 2940 ? "#F0EFF9" : "#FFFFFF"} />
            <Text id="bold-label" text="B" x={-26} y={-17} width={52} height={36} fontFamily="Inter" fontWeight={700} size={26} lineHeight={1.25} fit="none" align="center" color={computed.boldColor} />
          </Group>
          <Path id="italic-icon" x={94} y={22} width={18} height={21} viewBox="0 0 18 21" d="M6 1H17M1 20H12M13 1L5 20" fill="transparent" stroke="#636571" strokeWidth={2} />
          <Rect id="toolbar-divider" x={138} y={19} width={1} height={26} fill="#E5E6EB" />
          <Rect id="link-left" x={157} y={25} width={22} height={13} radius={6} fill="transparent" stroke="#636571" strokeWidth={2} rotation={-35} />
          <Rect id="link-right" x={168} y={25} width={22} height={13} radius={6} fill="transparent" stroke="#636571" strokeWidth={2} rotation={-35} />
          <Rect id="comment-icon" x={216} y={21} width={26} height={21} radius={5} fill="transparent" stroke="#636571" strokeWidth={2} />
          <Rect id="comment-tail" x={220} y={38} width={2} height={8} fill="#636571" rotation={28} />
        </Group>
        <Ellipse id="format-click" x={computed.boldX - 9 - 16 * computed.click} y={433 - 16 * computed.click} width={18 + 32 * computed.click} height={18 + 32 * computed.click} fill="transparent" stroke={vars.accent} strokeWidth={1.5} opacity={computed.clock >= 3150 ? 0.22 * Math.pow(1 - computed.click, 2) : 0} />
        <Group id="cursor" x={computed.pointerX} y={computed.pointerY} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.pointerScale * computed.pointerStrain} scaleY={computed.pointerScale / computed.pointerStrain} rotation={computed.pointerTilt}>
          <Image id="mont-arrow" src={vars.arrowSrc} x={-9.275362319} y={-6.956521739} width={55.652173913} height={55.652173913} opacity={computed.clock < 620 || computed.clock >= 2500 ? 1 : 0} borderWidth={0} borderColor="transparent" />
          <Image id="mont-ibeam" src={vars.beamSrc} x={-27.826086957} y={-27.826086957} width={55.652173913} height={55.652173913} opacity={computed.clock >= 620 && computed.clock < 2500 ? 1 : 0} borderWidth={0} borderColor="transparent" />
        </Group>
      </Group>
    </Scene>;
  },
});
