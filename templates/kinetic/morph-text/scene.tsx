export default defineSpanScene({
  id: "morph-text",
  width: 960,
  height: 540,
  durationMs: 12500,
  editPreviewMs: 750,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    phrase1: stringVar("Make room."),
    phrase2: stringVar("For bold ideas."),
    phrase3: stringVar("Less friction."),
    phrase4: stringVar("More motion."),
    phrase5: stringVar("Keep creating."),
    background: colorVar("#F3EBDD"),
    ink: colorVar("#243A46"),
    fontSize: numberVar(104),
    holdMs: numberVar(1500),
    morphMs: numberVar(1000),
  },
  computeds: {
    ink: computed.string(() => vars.ink),
    stepMs: computed.number(() => Math.max(0, vars.holdMs) + Math.max(200, vars.morphMs)),
    clockMs: computed.number(() => time.loop(computed.stepMs * 5) * computed.stepMs * 5),
    phrase: computed.number(() => Math.floor(computed.clockMs / computed.stepMs)),
    progress: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs % computed.stepMs - Math.max(0, vars.holdMs)) / Math.max(200, vars.morphMs)))),
    typeSize: computed.number(() => Math.max(12, vars.fontSize) * Math.min(vars.width / 960, vars.height / 540)),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="paper" width={vars.width} height={vars.height} fill={vars.background} />
      {/* Only these editable text layers are captured; the background stays crisp. */}
      <Group id="letterforms" textureSource={true} width={vars.width} height={vars.height}
        sceneInput={{width: vars.width, height: vars.height, timeScale: 1, timeOffsetMs: 0}}>
        <Repeat items={[vars.phrase1, vars.phrase2, vars.phrase3, vars.phrase4, vars.phrase5]} render={(phrase, index) => (
          <Repeat items={[text.glyphs(phrase, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0]]} render={(measure) => (
            <Group id={"phrase-layout-" + index}
              x={vars.width / 2} y={vars.height / 2} width={0} height={0} pivotX={0} pivotY={0}
              scaleX={Math.min(computed.typeSize / 100, vars.width * 0.88 / Math.max(1, measure.totalAdvance))}
              scaleY={Math.min(computed.typeSize / 100, vars.width * 0.88 / Math.max(1, measure.totalAdvance))}
              opacity={index === computed.phrase ? 1 - computed.progress : index === (computed.phrase + 1) % 5 ? computed.progress : 0}>
              <Text id={"phrase-" + index} text={phrase}
                x={-measure.totalAdvance / 2} y={-65} width={measure.totalAdvance + 4} height={150}
                size={100} lineHeight={130} fontFamily="Space Grotesk" weight={700} color={computed.ink}
                blur={5 * Math.min(12, index === computed.phrase ? 1 / Math.max(0.001, 1 - computed.progress) - 1 : 1 / Math.max(0.001, computed.progress) - 1)} />
            </Group>
          )} />
        )} />
      </Group>
      {/* Threshold the combined alpha, rather than displaying a blurred crossfade. */}
      <Procedural.Visual id="morph" width={vars.width} height={vars.height} render={(api) => {
        const morph = api.shader.wgsl({
          id: "ink-threshold",
          processor: "generic-fragment-shader",
          textureSrcs: ["texture-source://letterforms"],
          uniforms: {},
          code: `struct Uniforms { resolution: vec2f, };
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var letters: texture_2d<f32>;
@group(0) @binding(2) var letterSampler: sampler;
@fragment fn main(@builtin(position) position: vec4f) -> @location(0) vec4f {
  let sample = textureSample(letters, letterSampler, position.xy / u.resolution);
  let edge = max(fwidth(sample.a), 0.002);
  let coverage = smoothstep(0.392 - edge, 0.392 + edge, sample.a);
  return vec4f(sample.rgb / max(sample.a, 0.001) * coverage, coverage);
}`,
        });
        return api.frame([morph]);
      }} />
    </Scene>;
  },
});
