// Edge-to-edge horizontal rows with alternating direction and measured stretch.
// Shared measured cell boundaries keep the traveling stretch monotonic.
export default defineSpanScene({
  id: "stretch-row-repeater",
  width: 960,
  height: 540,
  durationMs: 12000,
  editPreviewMs: 1800,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    text: stringVar("MAKE ROOM"),
    alternateText: stringVar("FOR THE UNEXPECTED"),
    background: colorVar("#171B22"),
    cobalt: colorVar("#3857FF"),
    lemon: colorVar("#E7F478"),
    stretch: numberVar(0.48),
    cycleMs: numberVar(12000),
    phase: numberVar(0),
  },
  computeds: {
    rowHeight: computed.number(() => Math.min(vars.height * 0.28, vars.width * 0.3)),
    fontSize: computed.number(() => computed.rowHeight * 1.16),
    clock: computed.number(() => time.loop(Math.max(1000, Math.min(60000, vars.cycleMs)))),
    angle: computed.number(() => computed.clock * Math.PI * 4 + Math.max(-Math.PI * 2, Math.min(Math.PI * 2, vars.phase))),
    amplitude: computed.number(() => Math.max(0, Math.min(0.65, vars.stretch))),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.background} />
      <Repeat items={collection.range(0, Math.ceil(vars.height / computed.rowHeight) + 1)} render={(row) => (
        <Group id={"ribbon-" + row}
          x={0} y={(row - 0.5) * computed.rowHeight}
          width={vars.width} height={computed.rowHeight}
          clipLeft={0} clipRight={0} clipTop={0} clipBottom={0}>
          <Rect id={"ribbon-paper-" + row} x={0} y={0} width={vars.width} height={computed.rowHeight} fill={row % 2 === 1 ? vars.lemon : vars.cobalt} />
          <Repeat items={text.glyphs((row % 2 === 1 ? vars.alternateText : vars.text) + "  /  ", {
            fontFamily: "Archivo", fontWeight: 900,
            fontSize: Math.min(vars.height * 0.28, vars.width * 0.3) * 1.16,
            letterSpacing: 0, repeat: 3, repeatToLength: vars.width * 4,
          })} render={(glyph) => (
            <Group id={"ribbon-" + row + "-cell-" + glyph.index}
              x={glyph.offset + (row % 2 === 1 ? -computed.clock - 0.6 : computed.clock - 1.6) * glyph.cycleAdvance
                + computed.amplitude * glyph.cycleAdvance / (Math.PI * 2)
                * Math.sin(glyph.offset / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.45)}
              y={0} width={glyph.advance} height={computed.rowHeight}>
              <Text id={"ribbon-" + row + "-glyph-" + glyph.index} text={glyph.text}
                x={-glyph.bearingX} y={0}
                width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + computed.fontSize * 0.2}
                height={computed.rowHeight} size={computed.fontSize} lineHeight={computed.rowHeight}
                fontFamily={glyph.fontFamily} weight={glyph.fontWeight}
                color={row % 2 === 1 ? vars.cobalt : vars.lemon}
                scaleX={Math.max(0.25, 1 + computed.amplitude * glyph.cycleAdvance / (Math.PI * 2 * Math.max(1, glyph.advance))
                  * (Math.sin((glyph.offset + glyph.advance) / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.45)
                    - Math.sin(glyph.offset / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.45)))}
                pivotX={0} pivotY={0} align="left" />
            </Group>
          )} />
        </Group>
      )} />
    </Scene>;
  },
});