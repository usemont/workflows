// Measured native clusters follow one continuous Archimedean spiral, including spaces.
export default defineSpanScene({
  id: "kinetic-spiral-type",
  width: 400,
  height: 712,
  durationMs: 9000,
  variables: {
    width: numberVar(400),
    height: numberVar(712),
    text: stringVar("SMALL STEPS, EVERY DAY ☺"),
    background: colorVar("#F8F7F3"),
    ink: colorVar("#141514"),
    speed: numberVar(1),
    innerRadius: numberVar(59),
    pitch: numberVar(32),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width, vars.height) / 400),
    fontSize: computed.number(() => 26 * computed.scale),
    speed: computed.number(() => math.clamp(vars.speed, -3, 3)),
    inner: computed.number(() => math.clamp(vars.innerRadius, 40, 90) * computed.scale),
    pitch: computed.number(() => math.clamp(vars.pitch, 28, 46) * computed.scale),
    b: computed.number(() => computed.pitch / (2 * Math.PI)),
    outer: computed.number(() => Math.sqrt(vars.width * vars.width + vars.height * vars.height) / 2 + computed.fontSize),
    // Glyph-sized edge bleed covers every corner; one phrase below supplies the signed drift reservoir.
    coverage: computed.number(() => math.clamp((computed.outer * computed.outer - computed.inner * computed.inner) / (2 * computed.b), 1000 * computed.scale, 48000 * computed.scale)),
    rotation: computed.number(() => time.ms * 0.04 * computed.speed),
    drift: computed.number(() => time.ms * 0.044 * computed.scale * computed.speed),
  },
  render({ vars, computed }) {
    return (
      <Scene width={vars.width} height={vars.height}>
        <Rect id="spiral-background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.background} />
        <Group id="spiral" x={vars.width / 2} y={vars.height / 2} width={0} height={0} pivotX={0} pivotY={0} rotation={computed.rotation}>
          <Repeat
            items={[text.glyphs(vars.text + " ", { fontFamily: "Inter", fontWeight: 700, fontSize: computed.fontSize, letterSpacing: 0 })[0]]}
            render={(phrase) => (
              <Repeat
                items={text.glyphs(vars.text + " ", {
                  fontFamily: "Inter",
                  fontWeight: 700,
                  fontSize: computed.fontSize,
                  letterSpacing: 0,
                  repeatToLength: computed.coverage + phrase.cycleAdvance,
                })}
                render={(glyph, index) => (
                  // r = inner + b * theta: radius increases continuously, never in circular rings.
                  // Forward clusters read inward, preserving the original inward-facing tangent.
                  <Group
                    id={"spiral-glyph-" + index}
                    x={Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.totalAdvance - glyph.offset - glyph.advance / 2 - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) * Math.cos(3.66519143 + (Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.totalAdvance - glyph.offset - glyph.advance / 2 - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) - computed.inner) / computed.b)}
                    y={Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.totalAdvance - glyph.offset - glyph.advance / 2 - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) * Math.sin(3.66519143 + (Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.totalAdvance - glyph.offset - glyph.advance / 2 - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) - computed.inner) / computed.b)}
                    width={0}
                    height={0}
                    pivotX={0}
                    pivotY={0}
                    rotation={(3.66519143 + (Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.totalAdvance - glyph.offset - glyph.advance / 2 - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))) - computed.inner) / computed.b - Math.PI / 2 - Math.atan2(computed.b, Math.sqrt(computed.inner * computed.inner + 2 * computed.b * Math.max(0, glyph.totalAdvance - glyph.offset - glyph.advance / 2 - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance))))) * 180 / Math.PI}
                    opacity={math.smoothstep(glyph.totalAdvance - glyph.offset - glyph.advance / 2 - ((computed.drift % phrase.cycleAdvance + phrase.cycleAdvance) % phrase.cycleAdvance), 0, computed.fontSize * 10)}
                  >
                    <Text
                      id={"spiral-letter-" + index}
                      text={glyph.text}
                      x={-glyph.advance / 2}
                      y={-computed.fontSize * 17 / 26}
                      width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + computed.fontSize * 0.25}
                      height={computed.fontSize * 1.5}
                      size={computed.fontSize}
                      lineHeight={computed.fontSize}
                      fontFamily={glyph.fontFamily}
                      weight={glyph.fontWeight}
                      color={vars.ink}
                      align="left"
                    />
                  </Group>
                )}
              />
            )}
          />
        </Group>
      </Scene>
    );
  },
});
