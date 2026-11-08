export default defineSpanScene({
  id: "noise-field-dissolve-transition",
  width: 1080,
  height: 1080,
  durationMs: 1000,
  editModeTimeMs: 500,
  background: "transparent",
  hostFrameInputs: [
    { id: "clipA", kind: "timeline", role: "outgoing", sourceType: "video", fit: "cover" },
    { id: "clipB", kind: "timeline", role: "incoming", sourceType: "video", fit: "cover" }
  ],
  render() {
    return (
      <Scene background="transparent">
        <Procedural.Visual id="noise-field-dissolve" x={0} y={0} width={1080} height={1080} layer={4} render={(api) => {
          const shader = api.shader.wgsl({
            id: "noise-field-dissolve-shader",
            processor: "generic-fragment-shader",
            textureSrcs: ["host-frame:clipA", "host-frame:clipB"],
            uniforms: { progress: api.time.seconds / 1 },
            code: "struct Uniforms {\n  resolution: vec2f,\n  progress: f32,\n};\n@group(0) @binding(0) var<uniform> uniforms: Uniforms;\n@group(0) @binding(1) var clipAImage: texture_2d<f32>;\n@group(0) @binding(2) var clipASampler: sampler;\n@group(0) @binding(3) var clipBImage: texture_2d<f32>;\n@group(0) @binding(4) var clipBSampler: sampler;\n\nfn easeInOutCubic(t: f32) -> f32 {\n  let left = 4.0 * t * t * t;\n  let right = 1.0 - pow(-2.0 * t + 2.0, 3.0) * 0.5;\n  return select(right, left, t < 0.5);\n}\n\nfn hash21(value: vec2f) -> f32 {\n  return fract(sin(dot(value, vec2f(127.1, 311.7))) * 43758.5453);\n}\n\nfn valueNoise(p: vec2f) -> f32 {\n  let i = floor(p);\n  let f = fract(p);\n  let u = f * f * (3.0 - 2.0 * f);\n  let a = hash21(i + vec2f(0.0, 0.0));\n  let b = hash21(i + vec2f(1.0, 0.0));\n  let c = hash21(i + vec2f(0.0, 1.0));\n  let d = hash21(i + vec2f(1.0, 1.0));\n  return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);\n}\n\nfn fbm(p: vec2f) -> f32 {\n  let n1 = valueNoise(p) * 0.5;\n  let n2 = valueNoise(p * 2.03 + vec2f(5.7, 1.3)) * 0.26;\n  let n3 = valueNoise(p * 4.11 + vec2f(2.9, 8.2)) * 0.1352;\n  let n4 = valueNoise(p * 8.23 + vec2f(9.4, 4.1)) * 0.0703;\n  let n5 = valueNoise(p * 16.31 + vec2f(6.8, 7.7)) * 0.0366;\n  return n1 + n2 + n3 + n4 + n5;\n}\n\nfn sampleA(p: vec2f) -> vec4f {\n  return textureSample(clipAImage, clipASampler, clamp(p, vec2f(0.0), vec2f(1.0)));\n}\n\nfn sampleB(p: vec2f) -> vec4f {\n  return textureSample(clipBImage, clipBSampler, clamp(p, vec2f(0.0), vec2f(1.0)));\n}\n\n@fragment\nfn main(@builtin(position) position: vec4f) -> @location(0) vec4f {\n  let uv = position.xy / uniforms.resolution;\n  let raw = clamp(uniforms.progress, 0.0, 1.0);\n  if (raw <= 0.0001) {\n    return sampleA(uv);\n  }\n  if (raw >= 0.9999) {\n    return sampleB(uv);\n  }\n\n  let p = easeInOutCubic(raw);\n  let aspect = uniforms.resolution.x / uniforms.resolution.y;\n  let centered = (uv - vec2f(0.5, 0.5)) * vec2f(aspect, 1.0);\n  let swirl = vec2f(\n    sin(centered.y * 5.3 + p * 5.1),\n    cos(centered.x * 4.7 - p * 4.3)\n  ) * 0.09;\n  let low = fbm(centered * 2.0 + swirl + vec2f(p * 0.9, -p * 0.6));\n  let high = fbm(centered * 9.0 - swirl * 1.7 + vec2f(-p * 2.4, p * 1.8));\n  let cellular = hash21(floor((uv + swirl * 0.04) * uniforms.resolution / 9.0));\n  let field = clamp(low * 0.68 + high * 0.24 + cellular * 0.08, 0.0, 1.0);\n  let diagonal = dot(centered, normalize(vec2f(0.52, -0.32))) * 0.16;\n  let threshold = clamp(p + diagonal, 0.0, 1.0);\n  let softness = 0.035 + 0.045 * sin(p * 3.1415926536);\n  let reveal = smoothstep(field - softness, field + softness, threshold);\n  let edge = 1.0 - smoothstep(0.0, softness * 2.2, abs(field - threshold));\n  let micro = smoothstep(0.48, 0.92, high) * edge;\n  let a = sampleA(uv);\n  let b = sampleB(uv);\n  var color = mix(a, b, reveal);\n  color = vec4f(mix(color.rgb, vec3f(0.92, 0.98, 1.0), edge * 0.16), 1.0);\n  color = vec4f(min(color.rgb + vec3f(0.22, 0.28, 0.34) * micro * 0.18, vec3f(1.0)), 1.0);\n  return color;\n}"
          });
          return api.frame([shader]);
        }} />
      </Scene>
    );
  }
});