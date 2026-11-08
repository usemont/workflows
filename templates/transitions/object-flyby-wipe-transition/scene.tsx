export default defineSpanScene({
  id: "object-flyby-wipe-transition",
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
        <Procedural.Visual id="object-flyby-wipe" x={0} y={0} width={1080} height={1080} layer={4} render={(api) => {
          const shader = api.shader.wgsl({
            id: "object-flyby-wipe-shader",
            processor: "generic-fragment-shader",
            textureSrcs: ["host-frame:clipA", "host-frame:clipB"],
            uniforms: { progress: api.time.seconds / 1 },
            code: "struct Uniforms {\n  resolution: vec2f,\n  progress: f32,\n};\n@group(0) @binding(0) var<uniform> uniforms: Uniforms;\n@group(0) @binding(1) var clipAImage: texture_2d<f32>;\n@group(0) @binding(2) var clipASampler: sampler;\n@group(0) @binding(3) var clipBImage: texture_2d<f32>;\n@group(0) @binding(4) var clipBSampler: sampler;\n\nfn easeInOutCubic(t: f32) -> f32 {\n  let left = 4.0 * t * t * t;\n  let right = 1.0 - pow(-2.0 * t + 2.0, 3.0) * 0.5;\n  return select(right, left, t < 0.5);\n}\n\nfn hash21(value: vec2f) -> f32 {\n  return fract(sin(dot(value, vec2f(127.1, 311.7))) * 43758.5453);\n}\n\nfn sdRoundBox(p: vec2f, halfSize: vec2f, radius: f32) -> f32 {\n  let q = abs(p) - halfSize + vec2f(radius);\n  return length(max(q, vec2f(0.0))) + min(max(q.x, q.y), 0.0) - radius;\n}\n\nfn sampleA(p: vec2f) -> vec4f {\n  return textureSample(clipAImage, clipASampler, clamp(p, vec2f(0.0), vec2f(1.0)));\n}\n\nfn sampleB(p: vec2f) -> vec4f {\n  return textureSample(clipBImage, clipBSampler, clamp(p, vec2f(0.0), vec2f(1.0)));\n}\n\n@fragment\nfn main(@builtin(position) position: vec4f) -> @location(0) vec4f {\n  let uv = position.xy / uniforms.resolution;\n  let raw = clamp(uniforms.progress, 0.0, 1.0);\n  if (raw <= 0.0001) {\n    return sampleA(uv);\n  }\n  if (raw >= 0.9999) {\n    return sampleB(uv);\n  }\n\n  let p = easeInOutCubic(raw);\n  let aspect = uniforms.resolution.x / uniforms.resolution.y;\n  let screen = (uv - vec2f(0.5, 0.5)) * vec2f(aspect, 1.0);\n  let normal = normalize(vec2f(0.86, -0.50));\n  let tangent = vec2f(-normal.y, normal.x);\n  let extent = length(vec2f(aspect, 1.0)) * 0.68 + 0.08;\n  let edge = mix(extent, -extent, p);\n  let local = vec2f(dot(screen, normal) - edge, dot(screen, tangent) + sin(p * 3.1415926536) * 0.08);\n  let feather = max(0.0025, 1.5 / min(uniforms.resolution.x, uniforms.resolution.y));\n  let reveal = smoothstep(edge - feather, edge + feather, dot(screen, normal));\n  var color = mix(sampleA(uv), sampleB(uv), reveal);\n\n  let objectWidth = mix(0.50, 0.32, abs(p - 0.5) * 2.0);\n  let objectLength = extent * 2.6;\n  let bodyDistance = sdRoundBox(local, vec2f(objectWidth, objectLength), 0.12);\n  let body = 1.0 - smoothstep(0.0, feather * 4.0, bodyDistance);\n  let innerDistance = sdRoundBox(local - vec2f(-0.07, 0.0), vec2f(objectWidth * 0.46, objectLength * 0.92), 0.08);\n  let inner = 1.0 - smoothstep(0.0, feather * 6.0, innerDistance);\n  let glint = 1.0 - smoothstep(0.018, 0.055, abs(local.x + objectWidth * 0.42 + sin(local.y * 9.0) * 0.015));\n  let grain = hash21(floor(position.xy / 2.0)) * 0.045;\n  let rim = 1.0 - smoothstep(0.0, 0.045, abs(bodyDistance));\n  let longitudinal = 0.5 + 0.5 * sin(local.y * 13.0 + p * 16.0);\n  let objectBase = mix(vec3f(0.05, 0.06, 0.08), vec3f(0.95, 0.28, 0.11), 0.78 + longitudinal * 0.16);\n  let objectHighlight = vec3f(1.0, 0.82, 0.35) * (glint * 0.5 + inner * 0.24 + rim * 0.34);\n  let objectShadow = smoothstep(objectWidth * 0.08, objectWidth * 0.72, abs(local.x)) * 0.24;\n  let objectColor = vec4f(clamp(objectBase + objectHighlight + vec3f(grain) - vec3f(objectShadow), vec3f(0.0), vec3f(1.0)), 1.0);\n\n  let blur = exp(-local.x * local.x / 0.028) * smoothstep(objectLength + 0.25, objectLength * 0.72, abs(local.y));\n  color = vec4f(mix(color.rgb, vec3f(0.02, 0.025, 0.035), blur * 0.20), 1.0);\n  color = mix(color, objectColor, body);\n  return vec4f(color.rgb, 1.0);\n}"
          });
          return api.frame([shader]);
        }} />
      </Scene>
    );
  }
});