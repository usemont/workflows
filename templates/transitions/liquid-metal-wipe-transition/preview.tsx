// Standalone animated A/B preview. Use scene.tsx for live timeline host frames.
export default defineSpanScene({
  id: "liquid-metal-wipe-transition",
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
        <Procedural.Visual id="liquid-metal-wipe" x={0} y={0} width={1080} height={1080} layer={4} render={(api) => {
          const shader = api.shader.wgsl({
            id: "liquid-metal-wipe-shader",
            processor: "generic-fragment-shader",
            textureSrcs: ["texture-source://preview-clipA", "texture-source://preview-clipB"],
            uniforms: { progress: api.time.seconds / 1 },
            code: `struct Uniforms {
  resolution: vec2f,
  progress: f32,
};
@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var clipAImage: texture_2d<f32>;
@group(0) @binding(2) var clipASampler: sampler;
@group(0) @binding(3) var clipBImage: texture_2d<f32>;
@group(0) @binding(4) var clipBSampler: sampler;

fn easeInOutCubic(t: f32) -> f32 {
  let left = 4.0 * t * t * t;
  let right = 1.0 - pow(-2.0 * t + 2.0, 3.0) * 0.5;
  return select(right, left, t < 0.5);
}

fn hash21(value: vec2f) -> f32 {
  return fract(sin(dot(value, vec2f(127.1, 311.7))) * 43758.5453);
}

fn valueNoise(p: vec2f) -> f32 {
  let i = floor(p);
  let f = fract(p);
  let u = f * f * (3.0 - 2.0 * f);
  let a = hash21(i + vec2f(0.0, 0.0));
  let b = hash21(i + vec2f(1.0, 0.0));
  let c = hash21(i + vec2f(0.0, 1.0));
  let d = hash21(i + vec2f(1.0, 1.0));
  return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

fn fbm(p: vec2f) -> f32 {
  let n1 = valueNoise(p);
  let n2 = valueNoise(p * 2.03 + vec2f(4.7, 1.9)) * 0.5;
  let n3 = valueNoise(p * 4.11 + vec2f(8.3, 6.1)) * 0.25;
  return (n1 + n2 + n3) / 1.75;
}

fn luminance(color: vec3f) -> f32 {
  return dot(color, vec3f(0.2126, 0.7152, 0.0722));
}

@fragment
fn main(@builtin(position) position: vec4f) -> @location(0) vec4f {
  let uv = position.xy / uniforms.resolution;
  let raw = clamp(uniforms.progress, 0.0, 1.0);
  if (raw <= 0.0001) {
    return textureSample(clipAImage, clipASampler, uv);
  }
  if (raw >= 0.9999) {
    return textureSample(clipBImage, clipBSampler, uv);
  }

  let p = easeInOutCubic(raw);
  let aspect = uniforms.resolution.x / uniforms.resolution.y;
  let centered = (uv - vec2f(0.5, 0.5)) * vec2f(aspect, 1.0);
  let flow = fbm(centered * 3.2 + vec2f(p * 1.7, -p * 0.9));
  let fine = fbm(centered * 10.0 + vec2f(-p * 3.0, p * 2.0));
  let wave = sin((uv.y * 7.0 + flow * 2.2 - p * 3.1) * 6.2831853) * 0.035;
  let edge = mix(-0.18, 1.18, p) + wave + (flow - 0.5) * 0.16 + (fine - 0.5) * 0.035;
  let signedDistance = uv.x - edge;
  let feather = max(0.0035, 2.0 / min(uniforms.resolution.x, uniforms.resolution.y));
  let reveal = 1.0 - smoothstep(-feather * 1.5, feather * 1.5, signedDistance);

  let normalX = fbm(centered * 8.5 + vec2f(1.4, p * 2.7)) - 0.5;
  let normalY = fbm(centered.yx * 8.5 + vec2f(p * -1.8, 3.6)) - 0.5;
  let band = 1.0 - smoothstep(0.0, 0.18, abs(signedDistance));
  let contour = 1.0 - smoothstep(0.008, 0.034, abs(signedDistance));
  let bulge = band * band;
  let refractOffset = vec2f(normalX, normalY) * bulge * 0.035;
  let aUv = clamp(uv - refractOffset, vec2f(0.0), vec2f(1.0));
  let bUv = clamp(uv + refractOffset * 0.8, vec2f(0.0), vec2f(1.0));
  let a = textureSample(clipAImage, clipASampler, aUv);
  let b = textureSample(clipBImage, clipBSampler, bUv);
  var color = mix(a, b, reveal);

  let shineCoord = centered.x * 2.0 + centered.y * 1.35 + flow * 2.4 - p * 3.0;
  let stripe = 0.5 + 0.5 * sin(shineCoord * 18.0);
  let chrome = pow(stripe, 5.0) * 0.9 + pow(1.0 - stripe, 8.0) * 0.35;
  let baseLum = luminance(color.rgb);
  let silver = mix(vec3f(0.08, 0.09, 0.11), vec3f(0.96, 0.95, 0.9), chrome);
  let tint = mix(silver, vec3f(0.78, 0.86, 1.0), 0.18 + baseLum * 0.18);
  color = vec4f(mix(color.rgb, tint, band * 0.72), 1.0);
  color = vec4f(mix(color.rgb, vec3f(1.0, 0.98, 0.88), contour * 0.55), 1.0);
  let shadow = 1.0 - smoothstep(0.16, 0.33, abs(signedDistance)) * (1.0 - smoothstep(0.33, 0.5, abs(signedDistance)));
  color = vec4f(color.rgb - vec3f(0.05, 0.06, 0.08) * shadow * (1.0 - band) * 0.35, 1.0);
  return vec4f(clamp(color.rgb, vec3f(0.0), vec3f(1.0)), 1.0);
}`
          });
          return api.frame([shader]);
        }} />
      </Scene>
    );
  }
});
