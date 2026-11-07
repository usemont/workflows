// Three measured words pull into focus in order, hold, then blur away in order.
export default defineSpanScene({
  id: "focus-blur-resolve",
  width: 960,
  height: 540,
  durationMs: 3600,
  editPreviewMs: 1600,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    firstWord: stringVar("FOCUS"),
    secondWord: stringVar("BLUR"),
    thirdWord: stringVar("RESOLVE"),
    background: colorVar("#DCE5E7"),
    ink: colorVar("#153844"),
    fontSize: numberVar(90),
    blurPx: numberVar(24),
    focusMs: numberVar(800),
    staggerMs: numberVar(200),
    holdMs: numberVar(1000),
    exitMs: numberVar(600),
  },
  computeds: {
    focusMs: computed.number(() => Math.max(150, vars.focusMs)),
    staggerMs: computed.number(() => Math.max(0, vars.staggerMs)),
    holdMs: computed.number(() => Math.max(0, vars.holdMs)),
    exitMs: computed.number(() => Math.max(150, vars.exitMs)),
    cycleMs: computed.number(() => 400 + computed.focusMs + computed.holdMs + computed.exitMs + 4 * computed.staggerMs),
    clockMs: computed.number(() => time.loop(computed.cycleMs) * computed.cycleMs),
    entry: computed.number(() => (computed.clockMs - 200) / computed.focusMs),
    exit: computed.number(() => (computed.clockMs - 200 - computed.focusMs - 2 * computed.staggerMs - computed.holdMs) / computed.exitMs),
    typeScale: computed.number(() => Math.min(Math.max(12, vars.fontSize) / 100, vars.height * 0.26 / 130)),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="paper" width={vars.width} height={vars.height} fill={vars.background} />
      <Repeat items={[text.glyphs(vars.firstWord + " " + vars.secondWord + " " + vars.thirdWord, {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})[0]]} render={(measure) => (
        <Group id="headline-layout" x={vars.width / 2} y={vars.height / 2}
          width={0} height={0} pivotX={0} pivotY={0}
          scaleX={Math.min(computed.typeScale, vars.width * 0.82 / Math.max(1, measure.totalAdvance))}
          scaleY={Math.min(computed.typeScale, vars.width * 0.82 / Math.max(1, measure.totalAdvance))}>
          <Repeat items={[
            { text: vars.firstWord, index: 0, offset: 0 },
            { text: vars.secondWord, index: 1, offset: text.glyphs(vars.firstWord + " ", {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})[0].totalAdvance },
            { text: vars.thirdWord, index: 2, offset: text.glyphs(vars.firstWord + " " + vars.secondWord + " ", {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})[0].totalAdvance },
          ]} render={(word) => (
            <Repeat items={[text.glyphs(word.text, {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})[0]]} render={(metrics) => (
                <Group id={"focus-word-" + word.index}
                  x={word.offset + metrics.totalAdvance / 2 - measure.totalAdvance / 2} y={0}
                  width={0} height={0} pivotX={0} pivotY={0}
                  scaleX={1 + 0.08 * Math.pow(1 - Math.max(0, Math.min(1, computed.entry - word.index * computed.staggerMs / computed.focusMs)), 3)}
                  scaleY={1 + 0.08 * Math.pow(1 - Math.max(0, Math.min(1, computed.entry - word.index * computed.staggerMs / computed.focusMs)), 3)}
                  opacity={Math.max(0, Math.min(1, (computed.entry - word.index * computed.staggerMs / computed.focusMs) * 3)) * (1 - Math.pow(Math.max(0, Math.min(1, computed.exit - word.index * computed.staggerMs / computed.exitMs)), 2))}>
                  <Text id={"word-" + word.index} text={word.text}
                    x={-metrics.totalAdvance / 2} y={-65} width={metrics.totalAdvance + 8} height={150}
                    size={100} lineHeight={130} fontFamily="Archivo" weight={900} color={vars.ink}
                    blur={Math.max(0, vars.blurPx) * (Math.pow(1 - Math.max(0, Math.min(1, computed.entry - word.index * computed.staggerMs / computed.focusMs)), 3) + Math.pow(Math.max(0, Math.min(1, computed.exit - word.index * computed.staggerMs / computed.exitMs)), 2))} />
                </Group>
            )} />
          )} />
        </Group>
      )} />
    </Scene>;
  },
});
