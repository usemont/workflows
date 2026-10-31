// Measured glyph cells follow a monotonic wave: widths stay positive and neighbours
// share boundaries. Portrait adds two bands rather than shrinking a square canvas.
export default defineSpanScene({
  id: "stretch-row-repeater",
  width: 480,
  height: 480,
  durationMs: 12000,
  editPreviewMs: 0,
  variables: {
    width: numberVar(480),
    height: numberVar(480),
    text: stringVar("WORK"),
    alternateText: stringVar("PLAY"),
    crimson: colorVar("#D5170A"),
    cream: colorVar("#FDE3D2"),
    stretch: numberVar(0.58),
    cycleMs: numberVar(12000),
    phase: numberVar(0),
  },
  computeds: {
    rows: computed.number(() => vars.height > vars.width * 1.2 ? 7 : 5),
    rowHeight: computed.number(() => vars.height / (computed.rows - 0.5)),
    fontSize: computed.number(() => computed.rowHeight * 1.3),
    clock: computed.number(() => time.loop(Math.max(1000, Math.min(60000, vars.cycleMs)))),
    angle: computed.number(() => computed.clock * Math.PI * 10 + Math.max(-Math.PI * 2, Math.min(Math.PI * 2, vars.phase))),
    amplitude: computed.number(() => Math.max(0, Math.min(0.72, vars.stretch))),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.crimson} />
      <Repeat items={collection.range(0, vars.height > vars.width * 1.2 ? 7 : 5)} render={(row) => (
        <Group id={"row-" + row} x={0} y={(row - 0.25) * computed.rowHeight} width={vars.width} height={computed.rowHeight}>
          <Rect id={"row-" + row + "-band"} x={0} y={0} width={vars.width} height={computed.rowHeight} fill={row % 2 === 0 ? vars.crimson : vars.cream} />
          <Repeat items={text.glyphs(row % 2 === 0 ? vars.text : vars.alternateText, {
            fontFamily: "Archivo",
            fontWeight: 900,
            fontSize: vars.height / ((vars.height > vars.width * 1.2 ? 7 : 5) - 0.5) * 1.3,
            letterSpacing: 0,
            // Three cycles cover long phrases; measured length covers short copy.
            repeat: 3,
            repeatToLength: vars.width * 3,
          })} render={(glyph) => (
            <Group id={"row-" + row + "-cell-" + glyph.index}
              x={glyph.offset + (row % 2 === 0 ? computed.clock - 1.5 : -computed.clock - 0.5) * glyph.cycleAdvance
                + computed.amplitude * glyph.cycleAdvance / (Math.PI * 2)
                * Math.sin(glyph.offset / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.3)}
              y={0} width={glyph.advance} height={computed.rowHeight}>
              <Text id={"row-" + row + "-glyph-" + glyph.index} text={glyph.text}
                x={-glyph.bearingX * Math.max(glyph.advance * 0.03,
                  glyph.advance + computed.amplitude * glyph.cycleAdvance / (Math.PI * 2)
                  * (Math.sin((glyph.offset + glyph.advance) / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.3)
                    - Math.sin(glyph.offset / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.3))
                  - computed.fontSize * 0.012) / Math.max(1, glyph.inkWidth)}
                y={0} width={Math.max(glyph.advance, glyph.inkWidth) + computed.fontSize} height={computed.rowHeight}
                size={computed.fontSize} lineHeight={computed.rowHeight} fontFamily={glyph.fontFamily} weight={glyph.fontWeight}
                color={row % 2 === 0 ? vars.cream : vars.crimson}
                scaleX={Math.max(glyph.advance * 0.03,
                  glyph.advance + computed.amplitude * glyph.cycleAdvance / (Math.PI * 2)
                  * (Math.sin((glyph.offset + glyph.advance) / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.3)
                    - Math.sin(glyph.offset / glyph.cycleAdvance * Math.PI * 2 + computed.angle + row * 1.3))
                  - computed.fontSize * 0.012) / Math.max(1, glyph.inkWidth)}
                scaleY={1} pivotX={0} pivotY={0} align="left" />
            </Group>
          )} />
        </Group>
      )} />
    </Scene>;
  },
});
