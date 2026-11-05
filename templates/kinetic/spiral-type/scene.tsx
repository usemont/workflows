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
    speed: numberVar(0.85, { min: -3, max: 3, step: 0.05 }),
    innerRadius: numberVar(62, { min: 54, max: 96, step: 1 }),
    pitch: numberVar(46, { min: 40, max: 110, step: 1 }),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width, vars.height) / 540),
    fontSize: computed.number(() => 34 * computed.scale),
    inner: computed.number(() => math.clamp(vars.innerRadius, 54, 96) * computed.scale),
    pitch: computed.number(() => math.clamp(vars.pitch, 40, 110) * computed.scale),
    outer: computed.number(() => Math.sqrt(vars.width * vars.width + vars.height * vars.height) / 2 + computed.fontSize * 4),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.background} />
      <Path.Spiral id="spiral-path" center={[vars.width / 2, vars.height / 2, 0]}
        innerRadius={computed.inner} outerRadius={computed.outer} pitch={computed.pitch} startAngleDeg={-90} />
      <Text id="spiral-text" text={vars.text} editable x={0} y={0} width={vars.width} height={vars.height}
        size={computed.fontSize} lineHeight={computed.fontSize} fontFamily="DM Serif Display" weight={400}
        color={vars.ink} followPath={{ ref: "spiral-path", orientation: "tangent",
          repeat: "fill", repeatSeparator: " / ", tracking: 0.5 * computed.scale,
          phase: time.ms / 9000 * math.clamp(vars.speed, -3, 3),
          fadeStart: computed.fontSize * 2, fadeEnd: computed.fontSize * 3 }} />
    </Scene>;
  },
});