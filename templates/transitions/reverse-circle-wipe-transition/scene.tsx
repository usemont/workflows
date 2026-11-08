export default defineSpanScene({
  id: "reverse-circle-wipe-transition",
  width: 540,
  height: 540,
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
        <Procedural.Visual id="reverse-circle-wipe" x={0} y={0} width={540} height={540} layer={4} render={(api) => {
          const shader = api.shader.wgsl({
            id: "reverse-circle-wipe-shader",
            processor: "generic-fragment-shader",
            textureSrcs: ["host-frame:clipA", "host-frame:clipB"],
            uniforms: { progress: api.time.seconds / 1 },
            code: "struct Uniforms {\n  resolution: vec2f,\n  progress: f32,\n};\n@group(0) @binding(0) var<uniform> uniforms: Uniforms;\n@group(0) @binding(1) var clipAImage: texture_2d<f32>;\n@group(0) @binding(2) var clipASampler: sampler;\n@group(0) @binding(3) var clipBImage: texture_2d<f32>;\n@group(0) @binding(4) var clipBSampler: sampler;\n\nfn easeInOutCubic(t: f32) -> f32 {\n  let left = 4.0 * t * t * t;\n  let right = 1.0 - pow(-2.0 * t + 2.0, 3.0) * 0.5;\n  return select(right, left, t < 0.5);\n}\n\n@fragment\nfn main(@builtin(position) position: vec4f) -> @location(0) vec4f {\n  let uv = position.xy / uniforms.resolution;\n  let p = clamp(easeInOutCubic(clamp(uniforms.progress, 0.0, 1.0)), 0.0, 1.0);\n  let aspect = uniforms.resolution.x / uniforms.resolution.y;\n  let circular = (uv - vec2f(0.5, 0.5)) * vec2f(aspect, 1.0);\n  let edge = max(0.0025, 1.8 / min(uniforms.resolution.x, uniforms.resolution.y));\n  let maxRadius = length(vec2f(aspect, 1.0) * 0.5) + edge * 2.0;\n  let radius = maxRadius * (1.0 - p);\n  let dist = length(circular);\n  let keepA = 1.0 - smoothstep(radius - edge, radius + edge, dist);\n  let a = textureSample(clipAImage, clipASampler, uv);\n  let b = textureSample(clipBImage, clipBSampler, uv);\n  var color = mix(b, a, keepA);\n  let rim = 1.0 - smoothstep(edge * 0.2, edge * 1.2, abs(dist - radius));\n  let visibleRim = rim * smoothstep(0.015, 0.08, radius) * (1.0 - smoothstep(0.94, 1.0, p));\n  color = mix(color, vec4f(0.07, 0.09, 0.12, 1.0), visibleRim * 0.75);\n  return vec4f(color.rgb, 1.0);\n}"
          });
          return api.frame([shader]);
        }} />
      </Scene>
    );
  }
});