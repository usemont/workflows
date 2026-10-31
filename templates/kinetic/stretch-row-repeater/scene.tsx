// Three cut editorial ribbons, not an edge-to-edge typographic wallpaper.
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
    rowHeight: computed.number(() => Math.min(vars.height * 0.19, vars.width * 0.27)),
    fontSize: computed.number(() => computed.rowHeight * 0.88),
    clock: computed.number(() => time.loop(Math.max(1000, Math.min(60000, vars.cycleMs)))),
    angle: computed.number(() => computed.clock * Math.PI * 4 + Math.max(-Math.PI * 2, Math.min(Math.PI * 2, vars.phase))),
    amplitude: computed.number(() => Math.max(0, Math.min(0.65, vars.stretch))),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.background} />
      <Repeat items={collection.range(0, 3)} render={(row) => (
        <Group id={"ribbon-" + row}
          x={vars.width * (row === 0 ? 0.06 : row === 1 ? 0.22 : -0.08)}
          y={vars.height * (row === 0 ? 0.12 : row === 1 ? 0.43 : 0.77)}
          width={vars.width * (row === 1 ? 0.92 : 1.02)} height={computed.rowHeight}
          rotation={row === 0 ? -8 : row === 1 ? 5 : -4} pivotX={0} pivotY={0}
          clipLeft={0} clipRight={0} clipTop={0} clipBottom={0}>
          <Rect id={"ribbon-paper-" + row} x={0} y={0} width={vars.width * (row === 1 ? 0.92 : 1.02)} height={computed.rowHeight} fill={row === 1 ? vars.lemon : vars.cobalt} />
          <Repeat items={text.glyphs((row === 1 ? vars.alternateText : vars.text) + "  /  ", {
            fontFamily: "Space Grotesk", fontWeight: 700,
            fontSize: Math.min(vars.height * 0.19, vars.width * 0.27) * 0.88,
            letterSpacing: 0, repeat: 3, repeatToLength: vars.width * 4,
          })} render={(glyph) => (
            <Group id={"ribbon-" + row + "-cell-" + glyph.index}
              x={glyph.offset + (row === 1 ? -computed.clock - 0.6 : computed.clock - 1.6) * glyph.cycleAdvance
                + computed.amplitude * glyph.cycleAdvance / (Math.PI * 2)
                * Math.sin(glyph.offset / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.7)}
              y={0} width={glyph.advance} height={computed.rowHeight}>
              <Text id={"ribbon-" + row + "-glyph-" + glyph.index} text={glyph.text}
                x={-glyph.bearingX} y={0}
                width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + computed.fontSize * 0.2}
                height={computed.rowHeight} size={computed.fontSize} lineHeight={computed.rowHeight}
                fontFamily={glyph.fontFamily} weight={glyph.fontWeight}
                color={row === 1 ? vars.background : vars.lemon}
                scaleX={Math.max(0.25, 1 + computed.amplitude * glyph.cycleAdvance / (Math.PI * 2 * Math.max(1, glyph.advance))
                  * (Math.sin((glyph.offset + glyph.advance) / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.7)
                    - Math.sin(glyph.offset / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.7)))}
                pivotX={0} pivotY={0} align="left" />
            </Group>
          )} />
        </Group>
      )} />
    </Scene>;
  },
});