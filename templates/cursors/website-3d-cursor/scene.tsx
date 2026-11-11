// Forma is a fictional editor. All UI and artwork are editable native scene primitives.
// Cursor, landing layers, rim and shadows share the 1440 × 1000 desktop surface.
function clamp01(p) { return Math.max(0, Math.min(1, p)); }
function smoother(p) { return p*p*p*(10+p*(-15+6*p)); }
function ease(t, start, end) { return smoother(clamp01((t-start)/(end-start))); }
function assemble(t, delay) { return 1 - Math.pow(1 - clamp01((t-delay)/900), 4); }
function reveal(t, delay) { return 1 - Math.pow(1 - clamp01((t-delay)/400), 2); }
function lift(t, delay) { return 560 * (1 - assemble(t, delay)); }
// Exact projection of parallel planes separated along local Z. Native groups need no Z prop.
// R = Rz Ry Rx; projecting height z onto the base plane gives scale 1/(1-z/(d*cosX*cosY)).
function landingScale(t, delay) { return 1/(1-lift(t,delay)/(1800*Math.cos(pitch(t)*Math.PI/180)*Math.cos(yaw(t)*Math.PI/180))); }
function landingX(t, delay, x) { return x+(landingScale(t,delay)-1)*(x-focusX(t))+landingScale(t,delay)*lift(t,delay)*Math.tan(yaw(t)*Math.PI/180)/Math.cos(pitch(t)*Math.PI/180); }
function landingY(t, delay, y) { return y+(landingScale(t,delay)-1)*(y-focusY(t))-landingScale(t,delay)*lift(t,delay)*Math.tan(pitch(t)*Math.PI/180); }
function spline(p, a, b, c, d) { return .5*((2*b)+(-a+c)*p+(2*a-5*b+4*c-d)*p*p+(-a+3*b-3*c+d)*p*p*p); }
function camera(t, a, b, c, d) {
  return t < 3000 ? spline(clamp01(t/3000), 2*a-b, a, b, c) : t < 6000 ? spline(clamp01((t-3000)/3000), a, b, c, d) : spline(clamp01((t-6000)/3000), b, c, d, 2*d-c);
}
function bezier(p, start, control, end) { return (1-p)*(1-p)*start + 2*(1-p)*p*control + p*p*end; }
function arc(t, startMs, endMs, start, control, end) { return bezier(ease(t,startMs,endMs),start,control,end); }
// Three deliberate targets: playback, video clip, opacity slider.
function pointerX(t) {
  return t < 3500 ? arc(t,800,2600,1100,980,752) : t < 6300 ? arc(t,3500,5200,752,940,790) : arc(t,6300,7900,790,1110,1398)-36*ease(t,8000,8700);
}
function pointerY(t) {
  return t < 3500 ? arc(t,800,2600,610,590,644) : t < 6300 ? arc(t,3500,5200,644,700,820) : arc(t,6300,7900,820,760,386);
}
// The independent guide preserves composition; delayed tracking supplies 75% of travel.
function followX(t) { return .6*pointerX(t-160) + .3*pointerX(t-320) + .1*pointerX(t-520); }
function followY(t) { return .6*pointerY(t-160) + .3*pointerY(t-320) + .1*pointerY(t-520); }
function guideX(t) { return camera(t,720,740,790,1060); }
function guideY(t) { return camera(t,500,540,780,520); }
function focusX(t) { return guideX(t) + .75*(followX(t)-guideX(t))*ease(t,1700,3000); }
function focusY(t) { return guideY(t) + .75*(followY(t)-guideY(t))*ease(t,1700,3000); }
function zoom(t) { return camera(t,.60,.70,.84,.76); }
// Camera travel, not a second arbitrary wobble timeline, rocks the whole window.
function pitch(t) { return camera(t,10,9,7.5,8) + .016*(focusY(t)-focusY(t-220)); }
function yaw(t) { return camera(t,-2,-1,.5,1.2) + .025*(focusX(t)-focusX(t-220)); }
function roll(t) { return camera(t,-.6,-.3,.25,.4) + .0075*(focusX(t)-focusX(t-220)) - .002*(focusY(t)-focusY(t-220)); }
export default defineSpanScene({
  id: "website-3d-cursor", width: 960, height: 960, durationMs: 9000, editPreviewMs: 4300,
  variables: {
    width: numberVar(960), height: numberVar(960), speed: numberVar(1),
    background: colorVar("#E2DFE8"), pageColor: colorVar("#F1F0F4"),
    panel: colorVar("#FCFBFD"), raised: colorVar("#EAE7F0"), border: colorVar("#D7D2DF"),
    foreground: colorVar("#302B3A"), muted: colorVar("#6E657D"),
    accent: colorVar("#CDBCE4"), accentDeep: colorVar("#70518F"), accentSoft: colorVar("#E7DDF1"),
    windowRim: colorVar("#F6F4F9"), windowBorder: colorVar("#FFFFFF"), windowGlow: colorVar("#D8C5F6"),
    editorName: stringVar("Forma"), projectName: stringVar("A little more possibility"),
    posterLine1: stringVar("Make room"), posterLine2: stringVar("for ideas."),
    cursorSrc: stringVar("data:image/png;base64,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"),
  },
  computeds: {
    clock: computed.number(() => time.ms * Math.max(.25, Math.min(2, vars.speed))),
    scale: computed.number(() => Math.min(vars.width/960, vars.height/960)),
    cameraX: computed.number(() => focusX(computed.clock)), cameraY: computed.number(() => focusY(computed.clock)),
    zoom: computed.number(() => zoom(computed.clock)),
    tiltX: computed.number(() => pitch(computed.clock)), tiltY: computed.number(() => yaw(computed.clock)),
    roll: computed.number(() => roll(computed.clock)),
    pointerX: computed.number(() => pointerX(computed.clock)), pointerY: computed.number(() => pointerY(computed.clock)),
    playback: computed.number(() => ease(computed.clock,2800,5000)),
    selected: computed.number(() => ease(computed.clock,5300,5600)),
    adjustment: computed.number(() => ease(computed.clock,8000,8700)),
  },
  render({vars, computed}) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="studio-background" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="square-stage" x={vars.width/2} y={vars.height/2} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Group id="follow-camera" width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.zoom} scaleY={computed.zoom}>
          <Group id="face-up-editor" width={0} height={0} pivotX={0} pivotY={0} rotationX={computed.tiltX} rotationY={computed.tiltY} rotation={computed.roll} perspective={1800}>
            <Group id="editor-content" x={-computed.cameraX} y={-computed.cameraY} width={0} height={0} pivotX={0} pivotY={0}>
              {/* Concentric 12px chassis; the header and panel wells are part of the land. */}
              <Rect id="window-ambient-shadow" x={-12} y={-12} width={1464} height={1024} radius={50} fill={vars.windowRim} shadowColor="#373140" shadowBlur={90} shadowOffsetY={28} shadowOpacity={.22} />
              <Rect id="window-edge-glow" x={-12} y={-12} width={1464} height={1024} radius={50} fill={vars.windowRim} shadowColor={vars.windowGlow} shadowBlur={32} shadowOpacity={.36} />
              <Rect id="window-rim" x={-12} y={-12} width={1464} height={1024} radius={50} fill={vars.windowRim} stroke={vars.windowBorder} strokeWidth={1.5} shadowColor="#302C36" shadowBlur={24} shadowOffsetY={10} shadowOpacity={.16} />
              <Rect id="editor-surface" width={1440} height={1000} radius={38} fill={vars.pageColor} stroke={vars.border} strokeWidth={1} />
              <Mask.Rect id="rounded-editor-boundary" width={1440} height={1000} radius={38} fill="#FFFFFF" />
              <Group id="editor-chrome" mask="rounded-editor-boundary" width={0} height={0} pivotX={0} pivotY={0}>
                <Rect id="header-surface" width={1440} height={70} fill={vars.panel} />
                <Rect id="header-divider" y={69} width={1440} height={1} fill={vars.border} />
                <Repeat items={[{x:20,w:246},{x:284,w:856},{x:1158,w:262}]} render={(well,index) => (
                  <Rect id={"panel-well-"+index} x={well.x} y={130} width={well.w} height={550} radius={14} fill={vars.raised} />
                )} />
                <Rect id="timeline-well" x={20} y={700} width={1400} height={280} radius={14} fill={vars.raised} />
              </Group>
              <Group id="header-controls" x={landingX(computed.clock,100,720)} y={landingY(computed.clock,100,35)} width={0} height={0} pivotX={0} pivotY={0} opacity={reveal(computed.clock,100)} blur={8*(1-reveal(computed.clock,100))} scaleX={landingScale(computed.clock,100)} scaleY={landingScale(computed.clock,100)}>
                <Group id="header-layout" x={-720} y={-35} width={0} height={0} pivotX={0} pivotY={0}>
                  <Rect id="brand-tile" x={28} y={20} width={30} height={30} radius={9} fill={vars.accent} />
                  <Path id="brand-mark" x={36} y={27} width={15} height={16} viewBox="0 0 15 16" d="M0 16 L0 0 L15 0 L15 4 L4 4 L4 7 L12 7 L12 11 L4 11 L4 16 Z" fill="#30253B" />
                  <Text id="editor-name" text={vars.editorName} x={70} y={20} width={130} height={30} fontFamily="Inter" fontWeight={600} size={23} letterSpacing={-.7} color={vars.foreground} />
                  <Rect id="brand-separator" x={207} y={24} width={1} height={22} fill={vars.border} />
                  <Text id="project-name" text={vars.projectName} x={231} y={24} width={480} height={24} fontFamily="Inter" fontWeight={500} size={17} color={vars.foreground} />
                  <Ellipse id="saved-dot" x={1023} y={31} width={7} height={7} fill={vars.muted} />
                  <Text id="saved-label" text="All changes saved" x={1041} y={25} width={170} height={22} fontFamily="Inter" size={14} color={vars.muted} />
                  <Rect id="export-button" x={1254} y={17} width={138} height={36} radius={9} fill={vars.accent} />
                  <Text id="export-label" text="Export film" x={1254} y={25} width={138} height={23} align="center" fontFamily="Inter" fontWeight={600} size={15} color="#30253B" />
                </Group>
              </Group>
              <Text id="workspace-section" text="EDITING WORKSPACE" x={30} y={93} width={340} height={20} fontFamily="Inter" fontWeight={500} size={12} letterSpacing={1.5} color={vars.muted} />
              <Text id="workspace-format" text="1920 × 1080   /   24 fps" x={1110} y={92} width={300} height={22} align="right" fontFamily="Inter" size={13} color={vars.muted} />
              {/* Media library lands as a coherent panel, not separate floating labels. */}
              <Group id="library-arrival" x={landingX(computed.clock,240,143)} y={landingY(computed.clock,240,405)} width={0} height={0} pivotX={0} pivotY={0} opacity={reveal(computed.clock,240)} blur={8*(1-reveal(computed.clock,240))} scaleX={landingScale(computed.clock,240)} scaleY={landingScale(computed.clock,240)}>
                <Group id="media-library" x={-123} y={-275} width={0} height={0} pivotX={0} pivotY={0}>
                  <Rect id="library-surface" width={246} height={550} radius={14} fill={vars.panel} stroke={vars.border} strokeWidth={1} />
                  <Text id="library-title" text="Project media" x={18} y={20} width={195} height={28} fontFamily="Inter" fontWeight={600} size={19} color={vars.foreground} />
                  <Rect id="media-search" x={16} y={64} width={214} height={35} radius={8} fill={vars.pageColor} stroke={vars.border} strokeWidth={1} />
                  <Ellipse id="search-lens" x={28} y={75} width={10} height={10} fill="transparent" stroke={vars.muted} strokeWidth={1.5} />
                  <Path id="search-handle" x={36} y={83} width={5} height={5} viewBox="0 0 5 5" d="M0 0 L5 5" stroke={vars.muted} strokeWidth={1.5} fill="none" />
                  <Text id="search-label" text="Search assets" x={50} y={74} width={160} height={20} fontFamily="Inter" size={13} color={vars.muted} />
                  <Text id="media-count" text="ALL MEDIA" x={18} y={121} width={140} height={20} fontFamily="Inter" fontWeight={500} size={11} letterSpacing={1.3} color={vars.muted} />
                  <Text id="media-count-value" text="08" x={186} y={120} width={40} height={22} align="right" fontFamily="Inter" size={13} color={vars.muted} />
                  <Repeat items={[
                    {x:16,y:158,title:"Paper study",meta:"00:12 · Video",tone:"#D9D1EB",ink:"#9F88C1"},
                    {x:128,y:158,title:"Shape loop",meta:"00:08 · Video",tone:"#9F88C1",ink:"#D9D1EB"},
                    {x:16,y:303,title:"Opening title",meta:"00:06 · Graphic",tone:"#3C334D",ink:"#C6B0F4"},
                    {x:128,y:303,title:"Quiet texture",meta:"00:10 · Video",tone:"#C4B5D8",ink:"#705891"}
                  ]} render={(asset,index) => (
                    <Group id={"media-asset-"+index} x={asset.x} y={asset.y} width={0} height={0} pivotX={0} pivotY={0}>
                      <Rect id={"media-thumb-"+index} width={102} height={79} radius={7} fill={asset.tone} />
                      <Ellipse id={"media-disc-"+index} x={25} y={13} width={53} height={53} fill={asset.ink} />
                      <Rect id={"media-cut-"+index} x={50} y={13} width={28} height={53} fill={asset.tone} />
                      <Text id={"media-title-"+index} text={asset.title} y={90} width={106} height={21} fontFamily="Inter" fontWeight={500} size={12} color={vars.foreground} />
                      <Text id={"media-meta-"+index} text={asset.meta} y={113} width={106} height={20} fontFamily="Inter" size={10} color={vars.muted} />
                    </Group>
                  )} />
                  <Rect id="import-surface" x={16} y={485} width={214} height={43} radius={8} fill={vars.raised} stroke={vars.border} strokeWidth={1} />
                  <Text id="import-label" text="+  Import media" x={16} y={497} width={214} height={24} align="center" fontFamily="Inter" fontWeight={500} size={13} color={vars.foreground} />
                </Group>
              </Group>
              <Group id="preview-arrival" x={landingX(computed.clock,360,712)} y={landingY(computed.clock,360,405)} width={0} height={0} pivotX={0} pivotY={0} opacity={reveal(computed.clock,360)} blur={8*(1-reveal(computed.clock,360))} scaleX={landingScale(computed.clock,360)} scaleY={landingScale(computed.clock,360)}>
                <Group id="preview-panel" x={-428} y={-275} width={0} height={0} pivotX={0} pivotY={0}>
                  <Rect id="preview-surface" width={856} height={550} radius={14} fill={vars.panel} stroke={vars.border} strokeWidth={1} />
                  <Text id="preview-label" text="Preview" x={20} y={17} width={170} height={23} fontFamily="Inter" fontWeight={500} size={14} color={vars.foreground} />
                  <Text id="preview-fit" text="Fit   55%" x={704} y={17} width={130} height={23} align="right" fontFamily="Inter" size={13} color={vars.muted} />
                  <Rect id="poster-surface" x={24} y={52} width={808} height={454} radius={4} fill="#D9D1EB" />
                  <Text id="poster-eyebrow" text="A FORMA ORIGINAL" x={64} y={91} width={400} height={25} fontFamily="Inter" fontWeight={600} size={12} letterSpacing={2.2} color="#594669" />
                  <Text id="poster-title-line1" text={vars.posterLine1} x={62} y={193} width={580} height={78} fontFamily="Inter" fontWeight={600} size={64} letterSpacing={-3.2} color="#30253B" />
                  <Text id="poster-title-line2" text={vars.posterLine2} x={62} y={265} width={580} height={78} fontFamily="Inter" fontWeight={600} size={64} letterSpacing={-3.2} color="#30253B" />
                  <Group id="poster-flower" x={661} y={272} width={0} height={0} pivotX={0} pivotY={0} rotation={-14+2*Math.sin(computed.clock/1600)} scaleX={.84} scaleY={.84}>
                    <Repeat items={[0,45,90,135]} render={(angle,index) => (
                      <Ellipse id={"poster-petal-"+index} x={-55} y={-132} width={110} height={264} rotation={angle} fill="#A48BC2" />
                    )} />
                    <Ellipse id="poster-flower-core" x={-29} y={-29} width={58} height={58} fill="#D9D1EB" />
                  </Group>
                  <Text id="poster-footer" text="Small beginnings. Endless possibilities." x={64} y={435} width={620} height={27} fontFamily="Inter" fontWeight={500} size={14} color="#594669" />
                  <Text id="preview-time" text={computed.playback<.5 ? "00:04:12" : "00:06:18"} x={24} y={517} width={180} height={25} fontFamily="Inter" size={13} color={vars.muted} />
                  <Group id="play-control" x={468} y={514} width={0} height={0} pivotX={0} pivotY={0}>
                    <Rect id="playback-hover" x={-23} y={-23} width={46} height={46} radius={23} fill={computed.clock>2200 && computed.clock<3500 ? vars.accentSoft : vars.raised} />
                    <Path id="play-icon" x={-5} y={-8} width={13} height={16} viewBox="0 0 13 16" d="M0 0 L13 8 L0 16 Z" fill={vars.foreground} />
                  </Group>
                  <Text id="preview-duration" text="/  00:16:00" x={644} y={517} width={188} height={25} align="right" fontFamily="Inter" size={13} color={vars.muted} />
                </Group>
              </Group>
              <Group id="inspector-arrival" x={landingX(computed.clock,480,1289)} y={landingY(computed.clock,480,405)} width={0} height={0} pivotX={0} pivotY={0} opacity={reveal(computed.clock,480)} blur={8*(1-reveal(computed.clock,480))} scaleX={landingScale(computed.clock,480)} scaleY={landingScale(computed.clock,480)}>
                <Group id="inspector-panel" x={-131} y={-275} width={0} height={0} pivotX={0} pivotY={0}>
                  <Rect id="inspector-surface" width={262} height={550} radius={14} fill={vars.panel} stroke={vars.border} strokeWidth={1} />
                  <Text id="inspector-title" text="Properties" x={20} y={20} width={210} height={27} fontFamily="Inter" fontWeight={600} size={19} color={vars.foreground} />
                  <Rect id="inspector-tabs" x={16} y={64} width={230} height={36} radius={8} fill={vars.pageColor} />
                  <Rect id="inspector-active-tab" x={19} y={67} width={108} height={30} radius={6} fill={vars.raised} />
                  <Text id="inspector-tab-video" text="Video" x={19} y={75} width={108} height={22} align="center" fontFamily="Inter" fontWeight={500} size={13} color={vars.foreground} />
                  <Text id="inspector-tab-motion" text="Motion" x={128} y={75} width={113} height={22} align="center" fontFamily="Inter" size={13} color={vars.muted} />
                  <Text id="transform-title" text="Transform" x={20} y={128} width={210} height={24} fontFamily="Inter" fontWeight={500} size={14} color={vars.foreground} />
                  <Repeat items={[{x:20,label:"X",value:"960"},{x:139,label:"Y",value:"540"}]} render={(field,index) => (
                    <Group id={"position-field-"+index} x={field.x} y={169} width={0} height={0} pivotX={0} pivotY={0}>
                      <Rect id={"position-box-"+index} width={103} height={36} radius={7} fill={vars.raised} />
                      <Text id={"position-axis-"+index} text={field.label} x={12} y={10} width={25} height={20} fontFamily="Inter" size={12} color={vars.muted} />
                      <Text id={"position-value-"+index} text={field.value} x={35} y={9} width={56} height={22} align="right" fontFamily="Inter" size={13} color={vars.foreground} />
                    </Group>
                  )} />
                  <Text id="opacity-label" text="Opacity" x={20} y={222} width={140} height={24} fontFamily="Inter" size={13} color={vars.muted} />
                  <Text id="opacity-value" text={computed.adjustment>.5 ? "85%" : "100%"} x={169} y={221} width={73} height={24} align="right" fontFamily="Inter" size={13} color={vars.foreground} />
                  <Rect id="opacity-track" x={22} y={254} width={218} height={4} radius={2} fill={vars.border} />
                  <Rect id="opacity-fill" x={22} y={254} width={218-36*computed.adjustment} height={4} radius={2} fill={vars.accentDeep} />
                  <Ellipse id="opacity-thumb" x={233-36*computed.adjustment} y={249} width={14} height={14} fill={vars.accentDeep} />
                  <Rect id="inspector-divider" x={20} y={288} width={222} height={1} fill={vars.border} />
                  <Text id="color-heading" text="Color" x={20} y={310} width={210} height={25} fontFamily="Inter" fontWeight={500} size={14} color={vars.foreground} />
                  <Repeat items={[{label:"Exposure",value:"0.0",y:354},{label:"Contrast",value:"+12",y:399},{label:"Saturation",value:"100",y:444}]} render={(field,index) => (
                    <Group id={"color-field-"+index} x={20} y={field.y} width={0} height={0} pivotX={0} pivotY={0}>
                      <Text id={"color-label-"+index} text={field.label} width={150} height={23} fontFamily="Inter" size={13} color={vars.muted} />
                      <Text id={"color-value-"+index} text={field.value} x={151} width={71} height={23} align="right" fontFamily="Inter" size={13} color={vars.foreground} />
                    </Group>
                  )} />
                  <Text id="inspector-reset" text="Reset adjustments" x={20} y={506} width={222} height={23} fontFamily="Inter" size={12} color={vars.accentDeep} />
                </Group>
              </Group>
              <Group id="timeline-arrival" x={landingX(computed.clock,600,720)} y={landingY(computed.clock,600,840)} width={0} height={0} pivotX={0} pivotY={0} opacity={reveal(computed.clock,600)} blur={8*(1-reveal(computed.clock,600))} scaleX={landingScale(computed.clock,600)} scaleY={landingScale(computed.clock,600)}>
                <Group id="timeline-panel" x={-700} y={-140} width={0} height={0} pivotX={0} pivotY={0}>
                  <Rect id="timeline-surface" width={1400} height={280} radius={14} fill={vars.panel} stroke={vars.border} strokeWidth={1} />
                  <Text id="sequence-title" text="Main sequence" x={20} y={18} width={300} height={24} fontFamily="Inter" fontWeight={500} size={14} color={vars.foreground} />
                  <Text id="sequence-length" text="00:16:00" x={1180} y={18} width={196} height={24} align="right" fontFamily="Inter" size={13} color={vars.muted} />
                  <Rect id="ruler-divider" y={52} width={1400} height={1} fill={vars.border} />
                  <Repeat items={[0,1,2,3,4,5,6,7,8]} render={(tick,index) => (
                    <Group id={"ruler-tick-"+index} x={198+tick*142} y={61} width={0} height={0} pivotX={0} pivotY={0}>
                      <Text id={"ruler-time-"+index} text={tick*2+"s"} width={90} height={18} fontFamily="Inter" size={10} color={vars.muted} />
                      <Rect id={"ruler-mark-"+index} y={22} width={1} height={188} fill={vars.border} opacity={.35} />
                    </Group>
                  )} />
                  <Repeat items={[{name:"01  Video",y:115},{name:"02  Titles",y:183},{name:"03  Audio",y:240}]} render={(track,index) => (
                    <Text id={"track-label-"+index} text={track.name} x={20} y={track.y} width={155} height={24} fontFamily="Inter" size={12} color={vars.muted} />
                  )} />
                  <Repeat items={[
                    {x:198,w:280,title:"Paper study.mov"},
                    {x:486,w:426,title:"Shape loop.mov"},
                    {x:920,w:348,title:"Quiet texture.mov"}
                  ]} render={(clip,index) => (
                    <Group id={"video-clip-"+index} x={clip.x} y={94} width={0} height={0} pivotX={0} pivotY={0}>
                      <Rect id={"video-clip-surface-"+index} width={clip.w} height={58} radius={6} fill={vars.accentSoft} stroke={index===1 && computed.selected>.5 ? vars.accentDeep : vars.accent} strokeWidth={index===1 && computed.selected>.5 ? 2 : 1} />
                      <Text id={"video-clip-name-"+index} text={clip.title} x={12} y={11} width={clip.w-24} height={19} fontFamily="Inter" fontWeight={500} size={11} color={vars.foreground} />
                      <Repeat items={[0,1,2,3,4,5]} render={(frame,frameIndex) => (
                        <Rect id={"clip-frame-"+index+"-"+frameIndex} x={12+frame*(clip.w-24)/6} y={34} width={(clip.w-36)/6} height={14} radius={2} fill={vars.accentDeep} opacity={.20+.04*(frameIndex%3)} />
                      )} />
                    </Group>
                  )} />
                  <Rect id="title-clip" x={198} y={166} width={714} height={41} radius={6} fill={vars.accentSoft} stroke={vars.accentDeep} strokeWidth={1} />
                  <Text id="title-clip-label" text="T   Make room for ideas" x={212} y={178} width={620} height={23} fontFamily="Inter" fontWeight={500} size={12} color={vars.accentDeep} />
                  <Rect id="audio-clip" x={198} y={219} width={1070} height={43} radius={6} fill={vars.raised} />
                  <Repeat items={[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69]} render={(bar,index) => (
                    <Rect id={"waveform-bar-"+index} x={212+bar*15} y={240-(5+((bar*17+11)%23))/2} width={3} height={5+((bar*17+11)%23)} radius={1.5} fill={vars.accentDeep} opacity={.54} />
                  )} />
                  <Group id="playhead" x={620+240*computed.playback} y={53} width={0} height={0} pivotX={0} pivotY={0}>
                    <Path id="playhead-cap" x={-6} width={12} height={14} viewBox="0 0 12 14" d="M0 0 L12 0 L12 8 L6 14 L0 8 Z" fill={vars.foreground} />
                    <Rect id="playhead-line" y={9} width={1.5} height={204} fill={vars.foreground} />
                  </Group>
                </Group>
              </Group>
              <Image id="prepare-cursor" src={vars.cursorSrc} x={-20000} width={1} height={1} borderWidth={0} borderColor="transparent" />
              <Group id="editor-cursor" x={computed.pointerX} y={computed.pointerY} width={0} height={0} pivotX={0} pivotY={0} opacity={ease(computed.clock,1500,2100)}>
                <Image id="mont-arrow" src={vars.cursorSrc} x={-9.275362319} y={-6.956521739} width={55.652173913} height={55.652173913} borderWidth={0} borderColor="transparent" />
              </Group>
            </Group>
          </Group>
        </Group>
      </Group>
    </Scene>;
  },
});
