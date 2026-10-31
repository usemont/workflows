// Rounded, glyph-local contour inflation; measured advances retain word spacing.
// Subpixel clip overlap closes native antialias seams without font substitution.
export default defineSpanScene({
  id: "letter-inflate-swell",
  width: 960,
  height: 540,
  durationMs: 12000,
  editPreviewMs: 1800,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    text: stringVar("aerate"),
    background: colorVar("#142A3A"),
    mint: colorVar("#DAF3DE"),
    copper: colorVar("#CA8968"),
    bulge: numberVar(0.85),
    cycleMs: numberVar(6000),
    phase: numberVar(0),
  },
  computeds: {
    clock: computed.number(() => (time.loop(Math.max(2000, Math.min(30000, vars.cycleMs))) + Math.max(0, Math.min(1, vars.phase))) % 1),
    breath: computed.number(() => Math.pow(Math.sin(computed.clock * Math.PI), 2)),
    crest: computed.number(() => 0.18 + computed.clock * 0.68),
    amount: computed.number(() => Math.max(0, Math.min(1.2, vars.bulge)) * computed.breath),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.background} />
      <Ellipse id="copper-atmosphere" x={vars.width * 0.55} y={vars.height * 0.46}
        width={Math.min(vars.width, vars.height) * 0.39} height={Math.min(vars.width, vars.height) * 0.29}
        fill={vars.copper} opacity={0.12} blur={Math.min(vars.width, vars.height) * 0.13} />
      <Repeat items={[text.glyphs(vars.text, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0]]} render={(measure) => (
        <Group id="air-sculpture"
          x={vars.width * 0.48 - measure.totalAdvance * (1 + computed.amount * 0.62) * Math.min(vars.width * 0.78 / Math.max(1, measure.totalAdvance * 1.75), vars.height * 0.42 / 120) / 2}
          y={vars.height * 0.49 - 60 * Math.min(vars.width * 0.78 / Math.max(1, measure.totalAdvance * 1.75), vars.height * 0.42 / 120)}
          width={measure.totalAdvance * 1.75 + 40} height={120} pivotX={0} pivotY={0}
          scaleX={Math.min(vars.width * 0.78 / Math.max(1, measure.totalAdvance * 1.75), vars.height * 0.42 / 120)}
          scaleY={Math.min(vars.width * 0.78 / Math.max(1, measure.totalAdvance * 1.75), vars.height * 0.42 / 120)}>
          <Repeat items={text.glyphs(vars.text, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})} render={(glyph) => (
            <Group id={"swelling-letter-" + glyph.index} x={glyph.offset * (1 + computed.amount * 0.62)}
              y={0} width={0} height={0} pivotX={0} pivotY={0}>
              <Repeat items={collection.range(0, Math.max(24, Math.ceil(120 * Math.min(vars.width * 0.78 / Math.max(1, measure.totalAdvance * 1.75), vars.height * 0.42 / 120))))} render={(band) => (
                <Text id={"contour-band-" + glyph.index + "-" + band} text={glyph.text}
                  x={0} y={0} width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 20} height={120}
                  size={100} lineHeight={120} fontFamily={glyph.fontFamily} weight={glyph.fontWeight} color={vars.mint}
                  pivotX={glyph.advance / 2} pivotY={0}
                  scaleX={1 + computed.amount * 0.75 * Math.exp(-Math.pow((band / Math.max(24, Math.ceil(120 * Math.min(vars.width * 0.78 / Math.max(1, measure.totalAdvance * 1.75), vars.height * 0.42 / 120))) - computed.crest) / 0.28, 2))}
                  clipTop={(band - 0.6) / Math.max(24, Math.ceil(120 * Math.min(vars.width * 0.78 / Math.max(1, measure.totalAdvance * 1.75), vars.height * 0.42 / 120))) * 100}
                  clipBottom={(1 - (band + 1.6) / Math.max(24, Math.ceil(120 * Math.min(vars.width * 0.78 / Math.max(1, measure.totalAdvance * 1.75), vars.height * 0.42 / 120)))) * 100}
                  clipLeft={-100} clipRight={-100} />
              )} />
            </Group>
          )} />
        </Group>
      )} />
    </Scene>;
  },
});