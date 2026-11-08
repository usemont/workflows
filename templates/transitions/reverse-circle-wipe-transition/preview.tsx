// Standalone animated A/B preview. Use scene.tsx for live timeline host frames.
export default defineSpanScene({
  id: "reverse-circle-wipe-transition",
  width: 540,
  height: 540,
  durationMs: 1000,
  editModeTimeMs: 500,
  background: "transparent",
  render() {
    return (
      <Scene background="transparent">
        <Group id="preview-clipA" textureSource width={540} height={540} sceneInput={{width: 540, height: 540}}>
          <Rect id="preview-clipA-review-background" x={0} y={0} width={540} height={540} fill="#1763CA" />
          <Rect id="preview-clipA-tile-0-0" x={0.0} y={0.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-0" x={180.0} y={0.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-0" x={360.0} y={0.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-1" x={90.0} y={90.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-1" x={270.0} y={90.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-1" x={450.0} y={90.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-2" x={0.0} y={180.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-2" x={180.0} y={180.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-2" x={360.0} y={180.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-3" x={90.0} y={270.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-3" x={270.0} y={270.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-3" x={450.0} y={270.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-4" x={0.0} y={360.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-4" x={180.0} y={360.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-4" x={360.0} y={360.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-5" x={90.0} y={450.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-5" x={270.0} y={450.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-5" x={450.0} y={450.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-6" x={0.0} y={540.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-6" x={180.0} y={540.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-6" x={360.0} y={540.0} width={90.0} height={90.0} fill="#2978D9" />
          <Rect id="preview-clipA-v-0" x={0.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-1" x={90.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-2" x={180.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-3" x={270.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-4" x={360.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-5" x={450.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-6" x={540.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-0" x={0} y={0.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-1" x={0} y={90.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-2" x={0} y={180.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-3" x={0} y={270.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-4" x={0} y={360.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-5" x={0} y={450.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-6" x={0} y={540.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-edge" x={5} y={5} width={530} height={530} fill="transparent" stroke="#ffffff" strokeWidth={10} />
          <Text id="preview-clipA-label" text="A" x={135.0} y={135.0} width={270.0} height={270.0} size={205.2} lineHeight={270.0} fontFamily="Archivo" weight={900} color="#ffffff" align="center" />
          <Circle id="preview-clipA-moving-dot" x={70.2} y={410.4} width={32.4} height={32.4} fill="#ffffff">
          <Animate prop="x" from={70.2} to={432.0} start="0ms" duration="2600ms" ease="linear" />
          </Circle>
        </Group>
        <Group id="preview-clipB" textureSource width={540} height={540} sceneInput={{width: 540, height: 540}}>
          <Rect id="preview-clipB-review-background" x={0} y={0} width={540} height={540} fill="#E87924" />
          <Rect id="preview-clipB-tile-0-0" x={0.0} y={0.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-0" x={180.0} y={0.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-0" x={360.0} y={0.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-1" x={90.0} y={90.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-1" x={270.0} y={90.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-1" x={450.0} y={90.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-2" x={0.0} y={180.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-2" x={180.0} y={180.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-2" x={360.0} y={180.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-3" x={90.0} y={270.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-3" x={270.0} y={270.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-3" x={450.0} y={270.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-4" x={0.0} y={360.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-4" x={180.0} y={360.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-4" x={360.0} y={360.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-5" x={90.0} y={450.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-5" x={270.0} y={450.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-5" x={450.0} y={450.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-6" x={0.0} y={540.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-6" x={180.0} y={540.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-6" x={360.0} y={540.0} width={90.0} height={90.0} fill="#F58F37" />
          <Rect id="preview-clipB-v-0" x={0.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-1" x={90.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-2" x={180.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-3" x={270.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-4" x={360.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-5" x={450.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-6" x={540.0} y={0} width={2} height={540} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-0" x={0} y={0.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-1" x={0} y={90.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-2" x={0} y={180.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-3" x={0} y={270.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-4" x={0} y={360.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-5" x={0} y={450.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-6" x={0} y={540.0} width={540} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-edge" x={5} y={5} width={530} height={530} fill="transparent" stroke="#ffffff" strokeWidth={10} />
          <Text id="preview-clipB-label" text="B" x={135.0} y={135.0} width={270.0} height={270.0} size={205.2} lineHeight={270.0} fontFamily="Archivo" weight={900} color="#ffffff" align="center" />
          <Circle id="preview-clipB-moving-dot" x={70.2} y={410.4} width={32.4} height={32.4} fill="#ffffff">
          <Animate prop="x" from={70.2} to={432.0} start="0ms" duration="2600ms" ease="linear" />
          </Circle>
        </Group>
        <Procedural.Visual id="reverse-circle-wipe" x={0} y={0} width={540} height={540} layer={4} render={(api) => {
          const shader = api.shader.wgsl({
            id: "reverse-circle-wipe-shader",
            processor: "generic-fragment-shader",
            textureSrcs: ["texture-source://preview-clipA", "texture-source://preview-clipB"],
            uniforms: { progress: api.time.seconds / 1 },
            code: "struct Uniforms {\n  resolution: vec2f,\n  progress: f32,\n};\n@group(0) @binding(0) var<uniform> uniforms: Uniforms;\n@group(0) @binding(1) var clipAImage: texture_2d<f32>;\n@group(0) @binding(2) var clipASampler: sampler;\n@group(0) @binding(3) var clipBImage: texture_2d<f32>;\n@group(0) @binding(4) var clipBSampler: sampler;\n\nfn easeInOutCubic(t: f32) -> f32 {\n  let left = 4.0 * t * t * t;\n  let right = 1.0 - pow(-2.0 * t + 2.0, 3.0) * 0.5;\n  return select(right, left, t < 0.5);\n}\n\n@fragment\nfn main(@builtin(position) position: vec4f) -> @location(0) vec4f {\n  let uv = position.xy / uniforms.resolution;\n  let p = clamp(easeInOutCubic(clamp(uniforms.progress, 0.0, 1.0)), 0.0, 1.0);\n  let aspect = uniforms.resolution.x / uniforms.resolution.y;\n  let circular = (uv - vec2f(0.5, 0.5)) * vec2f(aspect, 1.0);\n  let edge = max(0.0025, 1.8 / min(uniforms.resolution.x, uniforms.resolution.y));\n  let maxRadius = length(vec2f(aspect, 1.0) * 0.5) + edge * 2.0;\n  let radius = maxRadius * (1.0 - p);\n  let dist = length(circular);\n  let keepA = 1.0 - smoothstep(radius - edge, radius + edge, dist);\n  let a = textureSample(clipAImage, clipASampler, uv);\n  let b = textureSample(clipBImage, clipBSampler, uv);\n  var color = mix(b, a, keepA);\n  let rim = 1.0 - smoothstep(edge * 0.2, edge * 1.2, abs(dist - radius));\n  let visibleRim = rim * smoothstep(0.015, 0.08, radius) * (1.0 - smoothstep(0.94, 1.0, p));\n  color = mix(color, vec4f(0.07, 0.09, 0.12, 1.0), visibleRim * 0.75);\n  return vec4f(color.rgb, 1.0);\n}"
          });
          return api.frame([shader]);
        }} />
      </Scene>
    );
  }
});
