// Standalone animated A/B preview. Use scene.tsx for live timeline host frames.
export default defineSpanScene({
  id: "dither-dissolve-transition",
  width: 1080,
  height: 1080,
  durationMs: 1000,
  editModeTimeMs: 500,
  background: "transparent",
  render() {
    return (
      <Scene background="transparent">
        <Group id="preview-clipA" textureSource width={1080} height={1080} sceneInput={{width: 1080, height: 1080}}>
          <Rect id="preview-clipA-review-background" x={0} y={0} width={1080} height={1080} fill="#1763CA" />
          <Rect id="preview-clipA-tile-0-0" x={0.0} y={0.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-0" x={360.0} y={0.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-0" x={720.0} y={0.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-1" x={180.0} y={180.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-1" x={540.0} y={180.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-1" x={900.0} y={180.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-2" x={0.0} y={360.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-2" x={360.0} y={360.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-2" x={720.0} y={360.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-3" x={180.0} y={540.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-3" x={540.0} y={540.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-3" x={900.0} y={540.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-4" x={0.0} y={720.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-4" x={360.0} y={720.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-4" x={720.0} y={720.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-5" x={180.0} y={900.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-5" x={540.0} y={900.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-5" x={900.0} y={900.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-6" x={0.0} y={1080.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-6" x={360.0} y={1080.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-6" x={720.0} y={1080.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-v-0" x={0.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-1" x={180.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-2" x={360.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-3" x={540.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-4" x={720.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-5" x={900.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-6" x={1080.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-0" x={0} y={0.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-1" x={0} y={180.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-2" x={0} y={360.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-3" x={0} y={540.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-4" x={0} y={720.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-5" x={0} y={900.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-6" x={0} y={1080.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-edge" x={5} y={5} width={1070} height={1070} fill="transparent" stroke="#ffffff" strokeWidth={10} />
          <Text id="preview-clipA-label" text="A" x={270.0} y={270.0} width={540.0} height={540.0} size={410.4} lineHeight={540.0} fontFamily="Archivo" weight={900} color="#ffffff" align="center" />
          <Circle id="preview-clipA-moving-dot" x={140.4} y={820.8} width={64.8} height={64.8} fill="#ffffff">
          <Animate prop="x" from={140.4} to={864.0} start="0ms" duration="2600ms" ease="linear" />
          </Circle>
        </Group>
        <Group id="preview-clipB" textureSource width={1080} height={1080} sceneInput={{width: 1080, height: 1080}}>
          <Rect id="preview-clipB-review-background" x={0} y={0} width={1080} height={1080} fill="#E87924" />
          <Rect id="preview-clipB-tile-0-0" x={0.0} y={0.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-0" x={360.0} y={0.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-0" x={720.0} y={0.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-1" x={180.0} y={180.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-1" x={540.0} y={180.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-1" x={900.0} y={180.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-2" x={0.0} y={360.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-2" x={360.0} y={360.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-2" x={720.0} y={360.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-3" x={180.0} y={540.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-3" x={540.0} y={540.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-3" x={900.0} y={540.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-4" x={0.0} y={720.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-4" x={360.0} y={720.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-4" x={720.0} y={720.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-5" x={180.0} y={900.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-5" x={540.0} y={900.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-5" x={900.0} y={900.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-6" x={0.0} y={1080.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-6" x={360.0} y={1080.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-6" x={720.0} y={1080.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-v-0" x={0.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-1" x={180.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-2" x={360.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-3" x={540.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-4" x={720.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-5" x={900.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-6" x={1080.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-0" x={0} y={0.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-1" x={0} y={180.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-2" x={0} y={360.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-3" x={0} y={540.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-4" x={0} y={720.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-5" x={0} y={900.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-6" x={0} y={1080.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-edge" x={5} y={5} width={1070} height={1070} fill="transparent" stroke="#ffffff" strokeWidth={10} />
          <Text id="preview-clipB-label" text="B" x={270.0} y={270.0} width={540.0} height={540.0} size={410.4} lineHeight={540.0} fontFamily="Archivo" weight={900} color="#ffffff" align="center" />
          <Circle id="preview-clipB-moving-dot" x={140.4} y={820.8} width={64.8} height={64.8} fill="#ffffff">
          <Animate prop="x" from={140.4} to={864.0} start="0ms" duration="2600ms" ease="linear" />
          </Circle>
        </Group>
        <Procedural.Visual id="dither-dissolve" x={0} y={0} width={1080} height={1080} layer={4} render={(api) => {
          const shader = api.shader.wgsl({
            id: "dither-dissolve-shader",
            processor: "generic-fragment-shader",
            textureSrcs: ["texture-source://preview-clipA", "texture-source://preview-clipB"],
            uniforms: { progress: api.time.seconds / 1 },
            code: "struct Uniforms {\n  resolution: vec2f,\n  progress: f32,\n};\n@group(0) @binding(0) var<uniform> uniforms: Uniforms;\n@group(0) @binding(1) var clipAImage: texture_2d<f32>;\n@group(0) @binding(2) var clipASampler: sampler;\n@group(0) @binding(3) var clipBImage: texture_2d<f32>;\n@group(0) @binding(4) var clipBSampler: sampler;\n\nfn easeInOutCubic(t: f32) -> f32 {\n  let left = 4.0 * t * t * t;\n  let right = 1.0 - pow(-2.0 * t + 2.0, 3.0) * 0.5;\n  return select(right, left, t < 0.5);\n}\n\nfn bayer8(cell: vec2u) -> f32 {\n  let x = cell.x & 7u;\n  let y = cell.y & 7u;\n  let index = y * 8u + x;\n  var threshold = 0.0;\n  switch index {\n    case 0u: { threshold = 0.0; }\n    case 1u: { threshold = 48.0; }\n    case 2u: { threshold = 12.0; }\n    case 3u: { threshold = 60.0; }\n    case 4u: { threshold = 3.0; }\n    case 5u: { threshold = 51.0; }\n    case 6u: { threshold = 15.0; }\n    case 7u: { threshold = 63.0; }\n    case 8u: { threshold = 32.0; }\n    case 9u: { threshold = 16.0; }\n    case 10u: { threshold = 44.0; }\n    case 11u: { threshold = 28.0; }\n    case 12u: { threshold = 35.0; }\n    case 13u: { threshold = 19.0; }\n    case 14u: { threshold = 47.0; }\n    case 15u: { threshold = 31.0; }\n    case 16u: { threshold = 8.0; }\n    case 17u: { threshold = 56.0; }\n    case 18u: { threshold = 4.0; }\n    case 19u: { threshold = 52.0; }\n    case 20u: { threshold = 11.0; }\n    case 21u: { threshold = 59.0; }\n    case 22u: { threshold = 7.0; }\n    case 23u: { threshold = 55.0; }\n    case 24u: { threshold = 40.0; }\n    case 25u: { threshold = 24.0; }\n    case 26u: { threshold = 36.0; }\n    case 27u: { threshold = 20.0; }\n    case 28u: { threshold = 43.0; }\n    case 29u: { threshold = 27.0; }\n    case 30u: { threshold = 39.0; }\n    case 31u: { threshold = 23.0; }\n    case 32u: { threshold = 2.0; }\n    case 33u: { threshold = 50.0; }\n    case 34u: { threshold = 14.0; }\n    case 35u: { threshold = 62.0; }\n    case 36u: { threshold = 1.0; }\n    case 37u: { threshold = 49.0; }\n    case 38u: { threshold = 13.0; }\n    case 39u: { threshold = 61.0; }\n    case 40u: { threshold = 34.0; }\n    case 41u: { threshold = 18.0; }\n    case 42u: { threshold = 46.0; }\n    case 43u: { threshold = 30.0; }\n    case 44u: { threshold = 33.0; }\n    case 45u: { threshold = 17.0; }\n    case 46u: { threshold = 45.0; }\n    case 47u: { threshold = 29.0; }\n    case 48u: { threshold = 10.0; }\n    case 49u: { threshold = 58.0; }\n    case 50u: { threshold = 6.0; }\n    case 51u: { threshold = 54.0; }\n    case 52u: { threshold = 9.0; }\n    case 53u: { threshold = 57.0; }\n    case 54u: { threshold = 5.0; }\n    case 55u: { threshold = 53.0; }\n    case 56u: { threshold = 42.0; }\n    case 57u: { threshold = 26.0; }\n    case 58u: { threshold = 38.0; }\n    case 59u: { threshold = 22.0; }\n    case 60u: { threshold = 41.0; }\n    case 61u: { threshold = 25.0; }\n    case 62u: { threshold = 37.0; }\n    default: { threshold = 21.0; }\n  }\n  return (threshold + 0.5) / 64.0;\n}\n\nfn hash21(value: vec2f) -> f32 {\n  return fract(sin(dot(value, vec2f(127.1, 311.7))) * 43758.5453);\n}\n\n@fragment\nfn main(@builtin(position) position: vec4f) -> @location(0) vec4f {\n  let uv = position.xy / uniforms.resolution;\n  let raw = clamp(uniforms.progress, 0.0, 1.0);\n  if (raw <= 0.0001) {\n    return textureSample(clipAImage, clipASampler, uv);\n  }\n  if (raw >= 0.9999) {\n    return textureSample(clipBImage, clipBSampler, uv);\n  }\n  let p = easeInOutCubic(raw);\n  let minRes = max(1.0, min(uniforms.resolution.x, uniforms.resolution.y));\n  let pixelBlock = max(3.0, floor(minRes / 150.0));\n  let cell = vec2u(floor(position.xy / pixelBlock));\n  let threshold = bayer8(cell);\n  let centeredUv = uv - vec2f(0.5, 0.5);\n  let diagonal = dot(centeredUv, normalize(vec2f(0.78, -0.62))) * 0.22;\n  let grain = (hash21(vec2f(cell) * 1.37) - 0.5) * 0.08;\n  let reveal = clamp(p + diagonal + grain, 0.0, 1.0);\n  let hard = select(0.0, 1.0, threshold < reveal);\n  let edgeBand = 1.0 - smoothstep(0.035, 0.18, abs(threshold - reveal));\n  let a = textureSample(clipAImage, clipASampler, uv);\n  let b = textureSample(clipBImage, clipBSampler, uv);\n  let mixed = mix(a, b, hard);\n  let sparkle = vec3f(1.0, 0.95, 0.78) * edgeBand * 0.035;\n  return vec4f(min(mixed.rgb + sparkle, vec3f(1.0)), mixed.a);\n}"
          });
          return api.frame([shader]);
        }} />
      </Scene>
    );
  }
});
