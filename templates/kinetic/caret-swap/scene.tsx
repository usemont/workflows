// A real cover wipe, a measured cluster-by-cluster type-in, then a reverse wipe home.
export default defineSpanScene({
  id: "caret-swap",
  width: 960,
  height: 540,
  durationMs: 3600,
  editPreviewMs: 2300,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    sourceText: stringVar("Make Space."),
    replacementText: stringVar("Move Ideas."),
    background: colorVar("#101C26"),
    ink: colorVar("#F4EBD8"),
    caretColor: colorVar("#C7F36B"),
    fontSize: numberVar(104),
    sourceHoldMs: numberVar(500),
    wipeMs: numberVar(350),
    collapseMs: numberVar(250),
    pauseMs: numberVar(100),
    typingMs: numberVar(900),
    finalHoldMs: numberVar(800),
    returnMs: numberVar(350),
    blinkMs: numberVar(300),
  },
  computeds: {
    ink: computed.string(() => vars.ink),
    wipeMs: computed.number(() => Math.max(150, vars.wipeMs)),
    collapseMs: computed.number(() => Math.max(150, vars.collapseMs)),
    pauseMs: computed.number(() => Math.max(50, vars.pauseMs)),
    typingMs: computed.number(() => Math.max(300, vars.typingMs)),
    returnMs: computed.number(() => Math.max(150, vars.returnMs)),
    wipeStart: computed.number(() => Math.max(200, vars.sourceHoldMs)),
    wipeEnd: computed.number(() => computed.wipeStart + computed.wipeMs),
    collapseEnd: computed.number(() => computed.wipeEnd + computed.collapseMs),
    typeStart: computed.number(() => computed.collapseEnd + computed.pauseMs),
    typeEnd: computed.number(() => computed.typeStart + computed.typingMs),
    returnStart: computed.number(() => computed.typeEnd + Math.max(400, vars.finalHoldMs)),
    returnCovered: computed.number(() => computed.returnStart + computed.wipeMs),
    cycleMs: computed.number(() => computed.returnCovered + computed.returnMs),
    clockMs: computed.number(() => time.loop(computed.cycleMs) * computed.cycleMs),
    wipeProgress: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.wipeStart) / computed.wipeMs))),
    wipe: computed.number(() => computed.wipeProgress < 0.5 ? 4 * Math.pow(computed.wipeProgress, 3) : 1 - Math.pow(-2 * computed.wipeProgress + 2, 3) / 2),
    collapseProgress: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.wipeEnd) / computed.collapseMs))),
    collapse: computed.number(() => computed.collapseProgress < 0.5 ? 4 * Math.pow(computed.collapseProgress, 3) : 1 - Math.pow(-2 * computed.collapseProgress + 2, 3) / 2),
    typed: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.typeStart) / computed.typingMs))),
    returnProgress: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.returnStart) / computed.wipeMs))),
    returnWipe: computed.number(() => computed.returnProgress < 0.5 ? 4 * Math.pow(computed.returnProgress, 3) : 1 - Math.pow(-2 * computed.returnProgress + 2, 3) / 2),
    revealProgress: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.returnCovered) / (computed.returnMs * 0.8)))),
    returnReveal: computed.number(() => computed.revealProgress < 0.5 ? 4 * Math.pow(computed.revealProgress, 3) : 1 - Math.pow(-2 * computed.revealProgress + 2, 3) / 2),
    blink: computed.number(() => Math.floor(Math.max(0, computed.clockMs - computed.typeEnd) / Math.max(100, vars.blinkMs)) % 2 === 0 ? 1 : 0),
    typeScale: computed.number(() => Math.min(Math.max(12, vars.fontSize) / 100, vars.height * 0.25 / 130)),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="paper" width={vars.width} height={vars.height} fill={vars.background} />
      <Repeat items={[text.glyphs(vars.sourceText, {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})[0]]} render={(source) => (
        <Repeat items={[text.glyphs(vars.replacementText, {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})[0]]} render={(target) => (
          <Group id="type-stage" x={vars.width / 2} y={vars.height / 2}
            width={0} height={0} pivotX={0} pivotY={0}
            scaleX={Math.min(computed.typeScale, vars.width * 0.84 / (Math.max(source.totalAdvance, target.totalAdvance) + 24))}
            scaleY={Math.min(computed.typeScale, vars.width * 0.84 / (Math.max(source.totalAdvance, target.totalAdvance) + 24))}>
            <Group id="line-anchor" x={-(Math.max(source.totalAdvance, target.totalAdvance) + 16) / 2}
              width={0} height={0} pivotX={0} pivotY={0}>
              <Text id="source-phrase" text={vars.sourceText}
                x={0} y={-65} width={source.totalAdvance + 8} height={150}
                size={100} lineHeight={130} fontFamily="Archivo" weight={900} color={computed.ink}
                opacity={computed.clockMs < computed.wipeEnd || computed.clockMs >= computed.returnCovered ? 1 : 0} />
              <Rect id="source-caret" x={source.totalAdvance + 6} y={-55} width={8} height={110}
                fill={vars.caretColor} opacity={computed.clockMs < computed.wipeStart ? 1 : 0} />
              {/* Static shaped records; only their visibility changes with the clock. */}
              <Repeat items={text.glyphs(vars.replacementText, {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})} render={(glyph) => (
                <Group id={"typed-cluster-" + glyph.index} width={0} height={0} pivotX={0} pivotY={0}
                  opacity={computed.clockMs >= computed.typeStart && computed.clockMs < computed.returnCovered ? 1 : 0}>
                  <Text id={"replacement-" + glyph.index} text={glyph.text}
                    x={glyph.offset} y={-65}
                    width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 20} height={150}
                    size={100} lineHeight={130} fontFamily={glyph.fontFamily} weight={glyph.fontWeight} color={computed.ink}
                    opacity={glyph.index < Math.floor(computed.typed * target.count) ? 1 : 0} />
                  <Rect id={"typing-caret-" + glyph.index}
                    x={glyph.offset + glyph.advance + 6} y={-55} width={8} height={110} fill={vars.caretColor}
                    opacity={glyph.index === Math.floor(computed.typed * target.count) - 1 && computed.clockMs < computed.returnStart
                      ? (computed.clockMs < computed.typeEnd || computed.clockMs >= computed.returnStart - 120 ? 1 : computed.blink) : 0} />
                </Group>
              )} />
              <Rect id="empty-prefix-caret" x={6} y={-55} width={8} height={110} fill={vars.caretColor}
                opacity={computed.clockMs >= computed.typeStart && Math.floor(computed.typed * target.count) === 0 ? 1 : 0} />
              {/* Opaque native cover: grows over visible ink before the text is hidden. */}
              <Rect id="wipe-cover" y={-55} height={110} fill={vars.caretColor}
                x={computed.clockMs < computed.wipeEnd
                  ? (source.totalAdvance + 6) * (1 - computed.wipe)
                  : computed.clockMs < computed.returnStart ? 6 * computed.collapse
                  : computed.clockMs < computed.returnCovered
                    ? (target.totalAdvance + 6) * (1 - computed.returnWipe)
                    : (source.totalAdvance + 6) * computed.returnReveal}
                width={computed.clockMs < computed.wipeEnd
                  ? 8 + (Math.max(source.totalAdvance, target.totalAdvance) + 8) * computed.wipe
                  : computed.clockMs < computed.collapseEnd
                    ? 8 + (Math.max(source.totalAdvance, target.totalAdvance) + 8) * (1 - computed.collapse)
                    : computed.clockMs < computed.returnStart ? 8
                    : computed.clockMs < computed.returnCovered
                      ? 8 + (Math.max(source.totalAdvance, target.totalAdvance) + 8) * computed.returnWipe
                      : 8 + (Math.max(source.totalAdvance, target.totalAdvance) + 8) * Math.max(0, 1 - computed.returnReveal)}
                opacity={(computed.clockMs >= computed.wipeStart && computed.clockMs < computed.typeStart) || computed.clockMs >= computed.returnStart ? 1 : 0} />
            </Group>
          </Group>
        )} />
      )} />
    </Scene>;
  },
});
