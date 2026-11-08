// Standalone animated A/B preview. Use scene.tsx for live timeline host frames.
export default defineSpanScene({
  id: "page-turn-transition",
  width: 1080,
  height: 1080,
  durationMs: 1000,
  editModeTimeMs: 500,
  background: "transparent",
  render() {
    // Page turn controls baked for this catalog default:
    // corner="top-right", curlAngleDeg=30, curlRadius=0.159, shadowStrength=1.
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
        <Procedural.Visual id="page-turn" x={0} y={0} width={1080} height={1080} layer={4} render={(api) => {
          const shader = api.shader.wgsl({
            id: "page-turn-shader",
            processor: "generic-fragment-shader",
            textureSrcs: ["texture-source://preview-clipA", "texture-source://preview-clipB"],
            uniforms: { progress: api.time.seconds / 1 },
            code: "struct Uniforms {\n  resolution: vec2f,\n  progress: f32,\n};\n@group(0) @binding(0) var<uniform> uniforms: Uniforms;\n@group(0) @binding(1) var clipAImage: texture_2d<f32>;\n@group(0) @binding(2) var clipASampler: sampler;\n@group(0) @binding(3) var clipBImage: texture_2d<f32>;\n@group(0) @binding(4) var clipBSampler: sampler;\n\nfn easeInOutCubic(t: f32) -> f32 {\n  let left = 4.0 * t * t * t;\n  let right = 1.0 - pow(-2.0 * t + 2.0, 3.0) * 0.5;\n  return select(right, left, t < 0.5);\n}\n\nfn rotateToCurl(p: vec2f) -> vec2f {\n  let angle = -0.5235987756;\n  let c = cos(angle);\n  let s = sin(angle);\n  return vec2f(c * p.x - s * p.y + 0.12, s * p.x + c * p.y + 0.258);\n}\n\nfn rotateFromCurl(p: vec2f) -> vec2f {\n  let angle = 0.5235987756;\n  let c = cos(angle);\n  let s = sin(angle);\n  return vec2f(c * p.x - s * p.y + 0.15, s * p.x + c * p.y - 0.50);\n}\n\nfn inBounds(p: vec2f) -> bool {\n  return p.x >= 0.0 && p.x <= 1.0 && p.y >= 0.0 && p.y <= 1.0;\n}\n\nfn distanceToEdge(p: vec2f) -> f32 {\n  var dx = select(p.x, 1.0 - p.x, p.x > 0.5);\n  var dy = select(p.y, 1.0 - p.y, p.y > 0.5);\n  if (p.x < 0.0) { dx = -p.x; }\n  if (p.x > 1.0) { dx = p.x - 1.0; }\n  if (p.y < 0.0) { dy = -p.y; }\n  if (p.y > 1.0) { dy = p.y - 1.0; }\n  if ((p.x < 0.0 || p.x > 1.0) && (p.y < 0.0 || p.y > 1.0)) {\n    return sqrt(dx * dx + dy * dy);\n  }\n  return min(dx, dy);\n}\n\nfn antiAlias(color1: vec4f, color2: vec4f, distance: f32) -> vec4f {\n  let scaled = distance * 512.0;\n  if (scaled < 0.0) { return color2; }\n  if (scaled > 2.0) { return color1; }\n  let dd = pow(1.0 - scaled * 0.5, 3.0);\n  return color1 + (color2 - color1) * dd;\n}\n\nfn sampleA(p: vec2f) -> vec4f {\n  return textureSample(clipAImage, clipASampler, clamp(p, vec2f(0.0), vec2f(1.0)));\n}\n\nfn sampleB(p: vec2f) -> vec4f {\n  return textureSample(clipBImage, clipBSampler, clamp(p, vec2f(0.0), vec2f(1.0)));\n}\n\nfn hitPoint(hitAngle: f32, point: vec2f) -> vec2f {\n  let hitPointY = hitAngle / (2.0 * 3.141592653589793);\n  return rotateFromCurl(vec2f(point.x, hitPointY));\n}\n\nfn seeThrough(yc: f32, p: vec2f, cylinderAngle: f32, cylinderRadius: f32) -> vec4f {\n  let hitAngle = 3.141592653589793 - (acos(clamp(yc / cylinderRadius, -1.0, 1.0)) - cylinderAngle);\n  let point = hitPoint(hitAngle, rotateToCurl(p));\n  if (yc <= 0.0 && !inBounds(point)) {\n    return sampleB(p);\n  }\n  if (yc > 0.0) {\n    return sampleA(p);\n  }\n  let color = sampleA(point);\n  return antiAlias(color, vec4f(0.0), distanceToEdge(point));\n}\n\nfn seeThroughWithShadow(yc: f32, p: vec2f, point: vec2f, amount: f32, cylinderAngle: f32, cylinderRadius: f32) -> vec4f {\n  var shadow = (1.0 - distanceToEdge(point) * 30.0) / 3.0;\n  if (shadow < 0.0) {\n    shadow = 0.0;\n  } else {\n    shadow = shadow * amount;\n  }\n  let c = seeThrough(yc, p, cylinderAngle, cylinderRadius);\n  return vec4f(max(c.rgb - vec3f(shadow), vec3f(0.0)), max(c.a, 1.0));\n}\n\nfn backside(point: vec2f, yc: f32, cylinderRadius: f32) -> vec4f {\n  let color = sampleA(point);\n  var gray = (color.r + color.g + color.b) / 15.0;\n  gray = gray + 0.8 * (pow(1.0 - abs(yc / cylinderRadius), 0.2) * 0.5 + 0.5);\n  return vec4f(vec3f(gray), 1.0);\n}\n\nfn behindSurface(ycInput: f32, pointInput: vec2f, amount: f32, cylinderAngle: f32, cylinderRadius: f32, uv: vec2f) -> vec4f {\n  var yc = -cylinderRadius - cylinderRadius - ycInput;\n  let hitAngle = (acos(clamp(yc / cylinderRadius, -1.0, 1.0)) + cylinderAngle) - 3.141592653589793;\n  let point = hitPoint(hitAngle, pointInput);\n  var shadow = (1.0 - ((-cylinderRadius - ycInput) / max(amount, 0.001) * 7.0)) / 6.0;\n  shadow = shadow * (1.0 - abs(point.x - 0.5));\n  if (yc < 0.0 && inBounds(point) && (hitAngle < 3.141592653589793 || amount > 0.5)) {\n    shadow = 1.0 - (length(point - vec2f(0.5)) / 0.71);\n    shadow = shadow * pow(-yc / cylinderRadius, 3.0) * 0.5;\n  } else {\n    shadow = 0.0;\n  }\n  let color = sampleB(uv);\n  return vec4f(max(color.rgb - vec3f(max(shadow, 0.0)), vec3f(0.0)), 1.0);\n}\n\n@fragment\nfn main(@builtin(position) position: vec4f) -> @location(0) vec4f {\n  let uv = position.xy / uniforms.resolution;\n  let raw = clamp(uniforms.progress, 0.0, 1.0);\n  if (raw <= 0.0001) {\n    return sampleA(uv);\n  }\n  if (raw >= 0.9999) {\n    return sampleB(uv);\n  }\n\n  let p = easeInOutCubic(raw);\n  let minAmount = -0.16;\n  let maxAmount = 1.30;\n  let amount = p * (maxAmount - minAmount) + minAmount;\n  let cylinderRadius = 0.159;\n  let cylinderCenter = amount;\n  let cylinderAngle = 2.0 * 3.141592653589793 * amount;\n  let point = rotateToCurl(uv);\n  let yc = point.y - cylinderCenter;\n\n  if (yc < -cylinderRadius) {\n    return behindSurface(yc, point, amount, cylinderAngle, cylinderRadius, uv);\n  }\n  if (yc > cylinderRadius) {\n    return sampleA(uv);\n  }\n\n  let hitAngle = (acos(clamp(yc / cylinderRadius, -1.0, 1.0)) + cylinderAngle) - 3.141592653589793;\n  let hitAngleWrapped = hitAngle - floor(hitAngle / (2.0 * 3.141592653589793)) * (2.0 * 3.141592653589793);\n  if ((hitAngleWrapped > 3.141592653589793 && amount < 0.5) || (hitAngleWrapped > 1.5707963268 && amount < 0.0)) {\n    return seeThrough(yc, uv, cylinderAngle, cylinderRadius);\n  }\n\n  let curlPoint = hitPoint(hitAngle, point);\n  if (!inBounds(curlPoint)) {\n    return seeThroughWithShadow(yc, uv, curlPoint, amount, cylinderAngle, cylinderRadius);\n  }\n  var color = backside(curlPoint, yc, cylinderRadius);\n  var otherColor = vec4f(0.0);\n  if (yc < 0.0) {\n    var shadow = 1.0 - (length(curlPoint - vec2f(0.5)) / 0.71);\n    shadow = shadow * pow(-yc / cylinderRadius, 3.0) * 0.5;\n    otherColor = vec4f(0.0, 0.0, 0.0, max(shadow, 0.0));\n  } else {\n    otherColor = sampleA(uv);\n  }\n  color = antiAlias(color, otherColor, cylinderRadius - abs(yc));\n  let through = seeThroughWithShadow(yc, uv, curlPoint, amount, cylinderAngle, cylinderRadius);\n  return antiAlias(color, through, distanceToEdge(curlPoint));\n}"
          });
          return api.frame([shader]);
        }} />
      </Scene>
    );
  }
});
