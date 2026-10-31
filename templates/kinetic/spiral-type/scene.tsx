// A centered, uniformly spaced spiral fills the canvas without tilt or wobble.
export default defineSpanScene({
  id: "kinetic-spiral-type",
  width: 960,
  height: 540,
  durationMs: 9000,
  editPreviewMs: 2200,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    text: stringVar("FOLLOW A DIFFERENT GRAVITY"),
    background: colorVar("#101A30"),
    ink: colorVar("#DDEAFF"),
    speed: numberVar(0.85),
    innerRadius: numberVar(62),
    pitch: numberVar(46),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width, vars.height) / 540),
    fontSize: computed.number(() => 34 * computed.scale),
    speed: computed.number(() => math.clamp(vars.speed, -3, 3)),
    inner: computed.number(() => math.clamp(vars.innerRadius, 54, 96) * computed.scale),
    pitch: computed.number(() => math.clamp(vars.pitch, 40, 110) * computed.scale),
    b: computed.number(() => computed.pitch / (2 * Math.PI)),
    outer: computed.number(() => Math.sqrt(vars.width * vars.width + vars.height * vars.height) / 2 + computed.fontSize * 4),
    coverage: computed.number(() => (computed.outer * computed.outer - computed.inner * computed.inner) / (2 * computed.b)),
    drift: computed.number(() => time.ms * 0.025 * computed.scale * computed.speed),
    centerX: computed.number(() => vars.width * 0.5),
    centerY: computed.number(() => vars.height * 0.5),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="spiral-field" x={computed.centerX} y={computed.centerY} width={0} height={0}
        pivotX={0} pivotY={0} rotation={-90}>
        <Repeat items={[text.glyphs(vars.text + " / ", {fontFamily: "DM Serif Display", fontWeight: 400, fontSize: computed.fontSize, letterSpacing: 0.5 * computed.scale})[0]]} render={(phrase) => (
          <Repeat items={text.glyphs(vars.text + " / ", {
            fontFamily: "DM Serif Display", fontWeight: 400, fontSize: computed.fontSize,
            letterSpacing: 0.5 * computed.scale, repeatToLength: computed.coverage + phrase.cycleAdvance,
          })} render={(glyph) => (
            <Group id={"spiral-cell-" + glyph.index}
              x={Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.offset - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) * Math.cos((Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.offset - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) - computed.inner) / computed.b)}
              y={Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.offset - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) * Math.sin((Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.offset - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) - computed.inner) / computed.b)}
              width={0} height={0} pivotX={0} pivotY={0}
              rotation={((Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.offset - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) - computed.inner) / computed.b + Math.PI / 2 - Math.atan2(computed.b, Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.offset - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))))) * 180 / Math.PI}
              opacity={math.smoothstep(glyph.offset - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance), 0, computed.fontSize * 2)
                * (1 - math.smoothstep(glyph.offset - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance), computed.coverage - computed.fontSize * 3, computed.coverage))}>
              <Text id={"spiral-glyph-" + glyph.index} text={glyph.text} x={0} y={-computed.fontSize * 0.72}
                width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + computed.fontSize * 0.3}
                height={computed.fontSize * 1.5} size={computed.fontSize} lineHeight={computed.fontSize}
                fontFamily={glyph.fontFamily} weight={glyph.fontWeight} color={vars.ink} />
            </Group>
          )} />
        )} />
      </Group>
    </Scene>;
  },
});