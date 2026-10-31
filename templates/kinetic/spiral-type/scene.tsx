// A finite luminous spiral floats off axis; its measured tangent remains readable.
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
    orbitText: stringVar("CHANGE YOUR ORBIT"),
    background: colorVar("#101A30"),
    ink: colorVar("#DDEAFF"),
    light: colorVar("#B1F6CF"),
    speed: numberVar(1),
    innerRadius: numberVar(62),
    pitch: numberVar(80),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width, vars.height) / 540),
    fontSize: computed.number(() => 34 * computed.scale),
    speed: computed.number(() => math.clamp(vars.speed, -3, 3)),
    inner: computed.number(() => math.clamp(vars.innerRadius, 54, 96) * computed.scale),
    pitch: computed.number(() => math.clamp(vars.pitch, 64, 110) * computed.scale),
    b: computed.number(() => computed.pitch / (2 * Math.PI)),
    outer: computed.number(() => Math.min(vars.width, vars.height) * 0.51),
    coverage: computed.number(() => (computed.outer * computed.outer - computed.inner * computed.inner) / (2 * computed.b)),
    rotation: computed.number(() => -24 + 13 * Math.sin(time.ms / 9000 * Math.PI * 2 * computed.speed)),
    drift: computed.number(() => time.ms * 0.025 * computed.scale * computed.speed),
    centerX: computed.number(() => vars.width * (vars.width > vars.height ? 0.61 : 0.54)),
    centerY: computed.number(() => vars.height * (vars.width > vars.height ? 0.53 : 0.49)),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.background} />
      <Ellipse id="orbital-atmosphere" x={computed.centerX - computed.outer * 0.77} y={computed.centerY - computed.outer * 0.65}
        width={computed.outer * 1.54} height={computed.outer * 1.3} fill={vars.light} opacity={0.065} blur={computed.outer * 0.3} />
      <Group id="tilted-orbit" x={computed.centerX} y={computed.centerY} width={0} height={0}
        pivotX={0} pivotY={0} rotation={computed.rotation} scaleY={0.84}>
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
      <Repeat items={text.glyphs(vars.orbitText + " / ", {fontFamily: "Press Start 2P", fontWeight: 400, fontSize: 12 * computed.scale, repeat: 1})} render={(glyph) => (
        <Group id={"inner-orbit-cell-" + glyph.index}
          x={computed.centerX + computed.inner * 0.61 * Math.cos(glyph.offset / Math.max(glyph.totalAdvance, 1) * Math.PI * 2 - time.ms * 0.00012 * computed.speed)}
          y={computed.centerY + computed.inner * 0.61 * Math.sin(glyph.offset / Math.max(glyph.totalAdvance, 1) * Math.PI * 2 - time.ms * 0.00012 * computed.speed)}
          width={0} height={0} pivotX={0} pivotY={0}
          rotation={glyph.offset / Math.max(glyph.totalAdvance, 1) * 360 - time.ms * 0.00012 * computed.speed * 180 / Math.PI + 90}>
          <Text id={"inner-orbit-glyph-" + glyph.index} text={glyph.text} x={0} y={-9 * computed.scale}
            width={glyph.advance + 8 * computed.scale} height={18 * computed.scale}
            size={Math.min(12 * computed.scale, computed.inner * 0.61 * Math.PI * 2 / Math.max(1, glyph.totalAdvance) * 12 * computed.scale)}
            lineHeight={12 * computed.scale} fontFamily={glyph.fontFamily} weight={glyph.fontWeight} color={vars.light} />
        </Group>
      )} />
    </Scene>;
  },
});