// Measured Titan One clusters, compact Arimo, and a deliberately overcanvas air swell.
// The bundled faces are visual substitutes, not a proprietary font recreation.
export default defineSpanScene({
  id: "letter-inflate-swell",
  width: 640,
  height: 360,
  durationMs: 12000,
  variables: {
    width: numberVar(640),
    height: numberVar(360),
    text: stringVar("PLUMP"),
    background: colorVar("#21112b"),
    pink: colorVar("#ff7eb8"),
    glow: colorVar("#6d3b58"),
    brand: stringVar("Puffmouth Type Co."),
    description: stringVar("A soft display face"),
    detail: stringVar("Six weights of air"),
    release: stringVar("Out now"),
    speed: numberVar(1),
    staggerMs: numberVar(140),
    swellHeight: numberVar(1),
    swellWidth: numberVar(1)
  },
  computeds: {
    clock: computed.number(() => (time.ms * Math.min(3, Math.max(0.1, vars.speed))) % 4000),
    releaseAir: computed.number(() => math.smoothstep(computed.clock, 3550, 3760)),
    metadata: computed.number(() => 1 - math.smoothstep(computed.clock, 2430, 2580) + math.smoothstep(computed.clock, 3660, 3810)),
    inflation: computed.number(() => math.smoothstep(computed.clock, 580, 1260) * (1 - computed.releaseAir)),
    giant: computed.number(() => math.smoothstep(computed.clock, 2430, 2940) * (1 - computed.releaseAir)),
    unit: computed.number(() => Math.min(vars.width, vars.height) / 360),
    padding: computed.number(() => Math.min(vars.width, vars.height) * 0.075),
    labelSize: computed.number(() => Math.max(11, Math.min(vars.width, vars.height) * 0.028)),
    labelLine: computed.number(() => Math.max(16, Math.min(vars.width, vars.height) * 0.045))
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.background} />
      <Ellipse id="ambient-air" x={vars.width * 0.15} y={vars.height * 0.18} width={vars.width * 0.7} height={vars.height * 0.64} fill={vars.glow} opacity={0.12} blur={Math.min(vars.width, vars.height) * 0.133} />
      <Ellipse id="central-glow" x={vars.width * 0.32} y={vars.height * 0.29} width={vars.width * 0.36} height={vars.height * 0.42} fill={vars.glow} opacity={0.24} blur={Math.min(vars.width, vars.height) * 0.111} />
      <Group id="corner-metadata" x={0} y={0} width={vars.width} height={vars.height} opacity={computed.metadata}>
        <Text id="brand" text={vars.brand} x={computed.padding} y={computed.padding} width={vars.width < vars.height * 0.8 ? vars.width - 2 * computed.padding : (vars.width - 2 * computed.padding) * 0.46} height={computed.labelLine * 2} size={computed.labelSize} lineHeight={computed.labelLine} fontFamily="Inter" weight="600" color={vars.pink} />
        <Text id="description" text={vars.description} x={vars.width < vars.height * 0.8 ? computed.padding : vars.width * 0.52} y={vars.width < vars.height * 0.8 ? computed.padding + computed.labelLine * 2.4 : computed.padding} width={vars.width < vars.height * 0.8 ? vars.width - 2 * computed.padding : vars.width * 0.48 - computed.padding} height={computed.labelLine * 2} size={computed.labelSize} lineHeight={computed.labelLine} fontFamily="Inter" weight="500" color={vars.pink} align={vars.width < vars.height * 0.8 ? "left" : "right"} />
        <Text id="detail" text={vars.detail} x={computed.padding} y={vars.height - computed.padding - computed.labelLine * (vars.width < vars.height * 0.8 ? 4.4 : 2)} width={vars.width < vars.height * 0.8 ? vars.width - 2 * computed.padding : (vars.width - 2 * computed.padding) * 0.6} height={computed.labelLine * 2} size={computed.labelSize} lineHeight={computed.labelLine} fontFamily="Inter" weight="400" color={vars.pink} />
        <Circle id="release-dot" x={vars.width - computed.padding - Math.min(vars.width * 0.25, computed.unit * 90)} y={vars.height - computed.padding - computed.labelLine * 2 + computed.labelSize * 0.5} width={computed.labelSize * 0.45} height={computed.labelSize * 0.45} fill={vars.pink} />
        <Text id="release" text={vars.release} x={vars.width - computed.padding - Math.min(vars.width * 0.25, computed.unit * 90) + computed.labelSize} y={vars.height - computed.padding - computed.labelLine * 2} width={Math.min(vars.width * 0.25, computed.unit * 90) - computed.labelSize} height={computed.labelLine * 2} size={computed.labelSize} lineHeight={computed.labelLine} fontFamily="Inter" weight="600" color={vars.pink} />
      </Group>
      <Repeat items={text.glyphs(vars.text, { fontFamily: "Titan One", fontWeight: 400, fontSize: 100 })} render={(glyph, index) => (
        <Group id={"letter-" + index}
          x={vars.width / 2 + (glyph.offset + glyph.advance / 2 - glyph.totalAdvance / 2) * (Math.min((vars.width - Math.min(vars.width, vars.height) * 0.15) * 0.94 / glyph.totalAdvance, computed.unit) * (0.42 + 0.55 * computed.inflation) + vars.width * 2 / glyph.totalAdvance * computed.giant * Math.min(2, Math.max(0, vars.swellWidth))) - glyph.advance / 2}
          y={vars.height / 2 - 60} width={glyph.advance} height={120}>
          <Group id={"compact-letter-" + index} x={0} y={0} width={glyph.advance} height={120} pivotX="50%" pivotY="50%"
            scaleX={Math.min((vars.width - Math.min(vars.width, vars.height) * 0.15) * 0.94 / glyph.totalAdvance, computed.unit) * (0.42 + 0.55 * math.smoothstep(computed.clock, 580 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1)), 700 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir))}
            scaleY={Math.min((vars.width - Math.min(vars.width, vars.height) * 0.15) * 0.94 / glyph.totalAdvance, computed.unit) * (0.4 + 0.55 * math.smoothstep(computed.clock, 580 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1)), 700 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir))}
            opacity={1 - math.smoothstep(computed.clock, 580 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1)), 700 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir)}>
            <Text id={"compact-glyph-" + index} text={glyph.text} x={glyph.advance / 2 - 100} y={-1.5} width={200} height={120} size={100} lineHeight={120} fontFamily="Arimo" weight="700" color={vars.pink} align="center" />
          </Group>
          <Group id={"bubble-letter-" + index} x={0} y={0} width={glyph.advance} height={120} pivotX="50%" pivotY="50%"
            scaleX={Math.min((vars.width - Math.min(vars.width, vars.height) * 0.15) * 0.94 / glyph.totalAdvance, computed.unit) * (0.42 + 0.55 * math.smoothstep(computed.clock, 580 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1)), 700 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir))}
            scaleY={Math.min((vars.width - Math.min(vars.width, vars.height) * 0.15) * 0.94 / glyph.totalAdvance, computed.unit) * (0.4 + 0.55 * math.smoothstep(computed.clock, 580 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1)), 700 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir))}
            opacity={math.smoothstep(computed.clock, 580 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1)), 700 + index * Math.min(Math.min(300, Math.max(0, vars.staggerMs)), 560 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir) * (1 - math.smoothstep(computed.clock, 2430 + index * Math.min(60, 240 / Math.max(1, glyph.count - 1)), 2700 + index * Math.min(60, 240 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir))}>
            <Text id={"bubble-glyph-" + index} text={glyph.text} x={0} y={-3.75} width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 4} height={120} size={100} lineHeight={120} fontFamily={glyph.fontFamily} weight={glyph.fontWeight} color={vars.pink} />
          </Group>
          <Group id={"tall-letter-" + index} x={0} y={0} width={glyph.advance} height={120} pivotX="50%" pivotY="50%"
            scaleX={Math.min((vars.width - Math.min(vars.width, vars.height) * 0.15) * 0.94 / glyph.totalAdvance, computed.unit) * 0.97 + vars.width * 2 / glyph.totalAdvance * math.smoothstep(computed.clock, 2430 + index * Math.min(60, 240 / Math.max(1, glyph.count - 1)), 2700 + index * Math.min(60, 240 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir) * Math.min(2, Math.max(0, vars.swellWidth))}
            scaleY={Math.min((vars.width - Math.min(vars.width, vars.height) * 0.15) * 0.94 / glyph.totalAdvance, computed.unit) * 0.95 + vars.height / 60 * math.smoothstep(computed.clock, 2430 + index * Math.min(60, 240 / Math.max(1, glyph.count - 1)), 2700 + index * Math.min(60, 240 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir) * Math.min(2, Math.max(0, vars.swellHeight))}
            opacity={math.smoothstep(computed.clock, 2430 + index * Math.min(60, 240 / Math.max(1, glyph.count - 1)), 2700 + index * Math.min(60, 240 / Math.max(1, glyph.count - 1))) * (1 - computed.releaseAir)}>
            <Text id={"tall-glyph-" + index} text={glyph.text} x={0} y={-3.75} width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 4} height={120} size={100} lineHeight={120} fontFamily={glyph.fontFamily} weight={glyph.fontWeight} color={vars.pink} />
          </Group>
        </Group>
      )} />
    </Scene>;
  }
});
