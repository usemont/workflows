// Measured glyphs rise and settle with a deterministic left-to-right stagger.
export default defineSpanScene({
  id: "bottom-up-letters",
  width: 960,
  height: 540,
  durationMs: 4200,
  editPreviewMs: 1800,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    text: stringVar("BOTTOM UP LETTERS"),
    background: colorVar("#EDE9DF"),
    ink: colorVar("#202F39"),
    riseMs: numberVar(650),
    staggerMs: numberVar(65),
    cycleMs: numberVar(4200),
  },
  computeds: {
    cycleMs: computed.number(() => Math.max(3000, vars.cycleMs)),
    clockMs: computed.number(() => time.loop(computed.cycleMs) * computed.cycleMs),
    riseMs: computed.number(() => Math.max(150, vars.riseMs)),
    staggerMs: computed.number(() => Math.max(0, vars.staggerMs)),
    exit: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.cycleMs + 650) / 450))),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="paper" width={vars.width} height={vars.height} fill={vars.background} />
      <Repeat items={[text.glyphs(vars.text, {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})[0]]} render={(measure) => (
        <Group id="title" x={vars.width / 2} y={vars.height / 2}
          width={0} height={0} pivotX={0} pivotY={0}
          scaleX={Math.min(vars.height * 0.30 / 120, vars.width * 0.86 / (measure.totalAdvance + 20))}
          scaleY={Math.min(vars.height * 0.30 / 120, vars.width * 0.86 / (measure.totalAdvance + 20))}>
          <Repeat items={text.glyphs(vars.text, {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})} render={(glyph) => (
            <Text id={"letter-" + glyph.index} text={glyph.text}
              x={glyph.offset - measure.totalAdvance / 2}
              y={-60 + 120 * Math.pow(Math.max(0, Math.min(1, 1 - (computed.clockMs - 250 - glyph.index * computed.staggerMs) / computed.riseMs)), 2)
                * (2.6 * Math.max(0, Math.min(1, 1 - (computed.clockMs - 250 - glyph.index * computed.staggerMs) / computed.riseMs)) - 1.6)
                - 120 * computed.exit * computed.exit}
              width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 20}
              height={120} size={100} lineHeight={120}
              fontFamily={glyph.fontFamily} weight={glyph.fontWeight}
              color={vars.ink} align="left"
              opacity={Math.max(0, Math.min(1, (computed.clockMs - 250 - glyph.index * computed.staggerMs) / (computed.riseMs * 0.3))) * (1 - computed.exit)} />
          )} />
        </Group>
      )} />
    </Scene>;
  },
});
