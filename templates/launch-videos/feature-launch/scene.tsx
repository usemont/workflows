// Feature launch: editable native UI by default, or an image through uiImage/useUiImage.
// The product-ui AssetScene slot can also be replaced by another pinned Mont scene.
function clamp01(p) { return Math.max(0, Math.min(1, p)); }
function progress(t, start, end) { return clamp01((t-start)/(end-start)); }
function smooth(t, start, end) { return .5-.5*Math.cos(Math.PI*progress(t,start,end)); }
function arrive(t, start, end) { return 1-Math.pow(1-progress(t,start,end),4); }
export default defineSpanScene({
  id: "feature-launch", width: 1920, height: 1080, durationMs: 12000, editPreviewMs: 6500,
  variables: {
    width: numberVar(1920), height: numberVar(1080),
    headline: stringVar("Make room for\nyour next idea."),
    subline: stringVar("A calmer way to bring your videos together."),
    featureTitle: stringVar("Every detail. One timeline."),
    featureBody: stringVar("Shape the cut without losing the bigger picture."),
    brand: stringVar("Forma"), endline: stringVar("Make something worth sharing."),
    ctaText: stringVar("Explore Forma"), link: stringVar("forma.example"),
    accent: colorVar("#C8F27C"), background: colorVar("#192724"),
    foreground: colorVar("#F5F3EB"), muted: colorVar("#B5C2B9"),
    uiImage: stringVar("resource:feature-launch-empty"), useUiImage: booleanVar(false),
    logoImage: stringVar("resource:feature-launch-empty"), useLogoImage: booleanVar(false),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width/1920,vars.height/1080)),
    clock: computed.number(() => time.ms),
    titleIn: computed.number(() => arrive(computed.clock,100,1050)),
    subIn: computed.number(() => arrive(computed.clock,480,1320)),
    reframe: computed.number(() => smooth(computed.clock,1850,3100)),
    panelIn: computed.number(() => arrive(computed.clock,2050,3450)),
    featureIn: computed.number(() => arrive(computed.clock,5200,5920)),
    toEnd: computed.number(() => smooth(computed.clock,8200,9100)),
    endIn: computed.number(() => arrive(computed.clock,8650,9650)),
    ctaIn: computed.number(() => arrive(computed.clock,9200,10000)),
  },
  render({vars,computed}) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Image id="ui-resource-preload" src={vars.uiImage} x={-4096} y={-4096} width={1} height={1} />
      <Image id="logo-resource-preload" src={vars.logoImage} x={-4096} y={-4096} width={1} height={1} />
      <Group id="landscape-stage" x={(vars.width-1920*computed.scale)/2} y={(vars.height-1080*computed.scale)/2} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Group id="launch" width={0} height={0} pivotX={0} pivotY={0} opacity={1-computed.toEnd} blur={8*computed.toEnd}>
          <Group id="headline-reframe" x={960} y={425-235*computed.reframe} width={0} height={0} pivotX={0} pivotY={0} scaleX={1-.48*computed.reframe} scaleY={1-.48*computed.reframe}>
            <Group id="headline-reveal" y={38*(1-computed.titleIn)} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.titleIn} blur={9*(1-computed.titleIn)}>
              <Text id="headline" text={vars.headline} x={-850} y={-150} width={1700} height={270} fontFamily="Inter" fontWeight={600} size={112} lineHeight={1.08} letterSpacing={-5} align="center" color={vars.foreground} />
            </Group>
            <Text id="subline" text={vars.subline} x={-790} y={155+16*(1-computed.subIn)} width={1580} height={64} size={31} fontFamily="Inter" letterSpacing={-.5} align="center" color={vars.muted} opacity={computed.subIn} />
          </Group>
          <Group id="product-arrival" x={960} y={635+600*(1-computed.panelIn)-12*smooth(computed.clock,3500,8100)} width={0} height={0} pivotX={0} pivotY={0} scaleX={.9+.1*computed.panelIn} scaleY={.9+.1*computed.panelIn} rotationX={10*(1-computed.panelIn)} rotation={-.7*(1-computed.panelIn)} perspective={2200} opacity={computed.panelIn} blur={6*(1-computed.panelIn)}>
            <Rect id="product-shadow" x={-688} y={-333} width={1376} height={666} radius={29} fill="#EEEFE8" shadowColor="#000000" shadowBlur={64} shadowOffsetY={27} shadowOpacity={.28} />
            <Rect id="product-rim" x={-688} y={-333} width={1376} height={666} radius={29} fill="#DEE4DA" stroke="#FFFFFF" strokeWidth={1} />
            <Mask.Rect id="product-boundary" x={-680} y={-325} width={1360} height={650} radius={21} fill="#FFFFFF" />
            <Group id="product-content" x={-680} y={-325} width={0} height={0} pivotX={0} pivotY={0} mask="product-boundary">
              <Group id="native-ui" width={0} height={0} pivotX={0} pivotY={0} opacity={vars.useUiImage ? 0 : 1}>
                <AssetScene id="product-ui" asset="product-ui" width={1360} height={650} inputs={{accent:vars.accent,brand:vars.brand}} />
              </Group>
              <Image id="product-image" src={vars.uiImage} width={1360} height={650} fit="contain" opacity={vars.useUiImage ? 1 : 0} />
            </Group>
          </Group>
          <Group id="feature-callout" x={1260+32*(1-computed.featureIn)} y={750+18*(1-computed.featureIn)} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.featureIn} blur={4*(1-computed.featureIn)}>
            <Rect id="callout-shadow" width={480} height={154} radius={21} fill={vars.accent} shadowColor="#071611" shadowOpacity={.17} shadowBlur={24} shadowOffsetY={12} />
            <Ellipse id="callout-dot" x={26} y={28} width={10} height={10} fill={vars.background} />
            <Text id="feature-title" text={vars.featureTitle} x={51} y={21} width={398} height={36} fontFamily="Inter" size={25} fontWeight={600} letterSpacing={-.7} color={vars.background} />
            <Text id="feature-body" text={vars.featureBody} x={26} y={72} width={420} height={66} fontFamily="Inter" size={23} lineHeight={1.25} letterSpacing={-.3} color={vars.background} />
          </Group>
        </Group>
        <Group id="end-card" x={960} y={458+28*(1-computed.endIn)} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.endIn} blur={5*(1-computed.endIn)}>
          <Group id="brand-lockup" x={-128} y={-133} width={0} height={0} pivotX={0} pivotY={0}>
            <Group id="native-logo" width={0} height={0} pivotX={0} pivotY={0} opacity={vars.useLogoImage ? 0 : 1}>
              <Rect id="logo-tile" width={60} height={60} radius={17} fill={vars.accent} />
              <Path id="logo-mark" x={18} y={15} width={26} height={31} viewBox="0 0 26 31" d="M0 31 L0 0 L26 0 L26 7 L8 7 L8 13 L22 13 L22 20 L8 20 L8 31 Z" fill={vars.background} />
            </Group>
            <Image id="brand-logo-image" src={vars.logoImage} width={60} height={60} fit="contain" opacity={vars.useLogoImage ? 1 : 0} />
            <Text id="brand-name" text={vars.brand} x={81} y={3} width={420} height={68} fontFamily="Inter" fontWeight={600} size={48} letterSpacing={-1.8} color={vars.foreground} />
          </Group>
          <Text id="endline" text={vars.endline} x={-800} y={-17} width={1600} height={115} fontFamily="Inter" fontWeight={500} size={68} letterSpacing={-2.8} align="center" color={vars.foreground} />
          <Group id="cta" y={154+16*(1-computed.ctaIn)} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.ctaIn}>
            <Rect id="cta-surface" x={-166} width={332} height={66} radius={33} fill={vars.accent} />
            <Text id="cta-text" text={vars.ctaText} x={-150} y={18} width={300} height={36} fontFamily="Inter" fontWeight={600} size={25} letterSpacing={-.5} align="center" color={vars.background} />
            <Text id="link" text={vars.link} x={-600} y={97} width={1200} height={36} fontFamily="Inter" size={24} align="center" color={vars.muted} />
          </Group>
        </Group>
      </Group>
    </Scene>;
  },
});
