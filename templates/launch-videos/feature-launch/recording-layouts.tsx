// One screen and one camera node survive every transition. Layout/crop belong to Mont's recorder AssetScene.
function unit(v) { return Math.max(0, Math.min(1, v)); }
// Accelerate over 28% of the move, then spend the remaining time settling; no overshoot.
function settle(u) { return u<.28?u*u*u/(.28*.28):1-Math.pow(1-u,3)/(.72*.72); }
function ease(t,a,b) { return settle(unit((t-a)/(b-a))); }
function arrive(t,a,b) { return 1-Math.pow(1-unit((t-a)/(b-a)),4); }
function morph(t,a,b,c,d) { return a+(b-a)*ease(t,4600,5700)+(c-b)*ease(t,8600,9750)+(d-c)*ease(t,12300,13400); }
function smoother(u) { return u*u*u*(u*(6*u-15)+10); }
function travel(t,a,b) { return smoother(unit((t-a)/(b-a))); }
function arc(t,a,b) { return 4*travel(t,a,b)*(1-travel(t,a,b)); }
export default defineSpanScene({
  id:"recording-layouts-launch", width:1920, height:1080, durationMs:18000, editPreviewMs:6800,
  variables:{screenSource:stringVar("host-frame:screen"),cameraSource:stringVar("host-frame:camera")},
  computeds:{
    t:computed.number(()=>time.ms),
    intro:computed.number(()=>arrive(computed.t,100,800)),
    end:computed.number(()=>ease(computed.t,15000,16100)),
    closing:computed.number(()=>arrive(computed.t,15750,16450)),
    sx:computed.number(()=>morph(computed.t,.06,.045,.625,.06)),
    sy:computed.number(()=>morph(computed.t,.07,.15,.31,.07)),
    sw:computed.number(()=>morph(computed.t,.88,.65,.33,.88)),
    sh:computed.number(()=>morph(computed.t,.86,.7,.38,.86)),
    cx:computed.number(()=>morph(computed.t,.825,.715,.045,.825)),
    cy:computed.number(()=>morph(computed.t,.695,.15,.08,.695)),
    cw:computed.number(()=>morph(computed.t,.14,.24,.55,.14)),
    ch:computed.number(()=>morph(computed.t,.14,.7,.84,.14)),
    cameraShape:computed.number(()=>morph(computed.t,1,0,0,1)),
    backdropClock:computed.struct(()=>[1920,1080,time.ms/1000,0]),
    selector:computed.number(()=>arrive(computed.t,4400,4800)+arrive(computed.t,8400,8800)-2*arrive(computed.t,12100,12500)),
    cornerLabelColor:computed.string(()=>computed.selector<.5?"#201D19":"#F1F4EF"),
    sideLabelColor:computed.string(()=>computed.selector>=.5&&computed.selector<1.5?"#201D19":"#F1F4EF"),
    presenterLabelColor:computed.string(()=>computed.selector>=1.5?"#201D19":"#F1F4EF"),
    cursorX:computed.number(()=>CURSOR_X),
    cursorY:computed.number(()=>CURSOR_Y),
    cursorPressed:computed.number(()=>CURSOR_PRESSED),
  },
  render({vars,computed}) {
    return <Scene width={1920} height={1080}>
      <Procedural.Visual id="quiet-dot-grid" width={1920} height={1080} render={(api)=>{return api.frame([api.shader.wgsl({processor:"generic-fragment-shader",uniforms:{canvas:computed.backdropClock},textureSrcs:[],code:"struct Uniforms { canvas:vec4f, }; @group(0) @binding(0) var<uniform> u:Uniforms; @fragment fn main(@builtin(position) p:vec4f)->@location(0) vec4f { let uv=p.xy/u.canvas.xy; let t=u.canvas.z; let center=vec2f(.5+.34*sin(t*.29-.9),.5+.25*cos(t*.23)); let d=(uv-center)*vec2f(1.45,1.0); let glow=exp(-dot(d,d)*7.0); let cool=exp(-dot(uv-vec2f(.2,.8),uv-vec2f(.2,.8))*5.0); let paper=vec3f(.89,.88,.85); let tint=mix(paper,vec3f(.98,.76,.56),glow*.3); let bg=mix(tint,vec3f(.75,.85,.83),cool*.16); let drift=vec2f(7.0*sin(t*.17),5.0*cos(t*.19)); let grid=length((fract((p.xy+vec2f(16.0)+drift)/32.0)-.5)*32.0); let dotAlpha=(1.0-smoothstep(1.3,2.3,grid))*(.46+.2*glow); let ink=mix(vec3f(.38,.43,.4),vec3f(.65,.3,.13),glow*.7); return vec4f(mix(bg,ink,dotAlpha),1.0); }"})]);}}/>
      <Group id="opening-title" x={110} y={58+22*(1-computed.intro)} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.intro*(1-computed.end)}>
        <Rect id="mont-tile" width={44} height={44} radius={13.2} fill="#FF6B35"/>
        <Path id="mont-mark" width={44} height={44} viewBox="0 0 40 40" d="M10.571 28.444 L16.857 19.111 L20 23.778 L23.143 19.111 L29.429 28.444" fill="none" stroke="#160A04" strokeWidth={3.13} strokeLineJoin="round" strokeLineCap="round"/>
        <Text id="title" x={64} y={2} width={1200} height={60} text="Screen and camera layouts" fontFamily="Inter" fontWeight={500} size={36} letterSpacing={-1.1} color="#242A28"/>
      </Group>
      <Group id="recording-stage" x={60+780*computed.end} y={145+145*computed.end} width={0} height={0} pivotX={0} pivotY={0} scaleX={.94-.38*computed.end} scaleY={.94-.38*computed.end}>
        <AssetScene id="layout" asset="video.mont.layout.side-by-side" width={1920} height={840} inputs={{viewportWidth:1920,viewportHeight:840,screenWidth:1280,screenHeight:800,cameraWidth:1280,cameraHeight:720,screenRectX:computed.sx,screenRectY:computed.sy,screenRectWidth:computed.sw,screenRectHeight:computed.sh,cameraRectX:computed.cx,cameraRectY:computed.cy,cameraRectWidth:computed.cw,cameraRectHeight:computed.ch,cameraAlignScreen:false,cameraShape:computed.cameraShape,screenRadiusPermille:20,cameraRadiusPermille:24,cameraMirrored:true,screenContain:true,cameraContain:false,cornerEnabled:false,screenThemeTiltDeg:3,screenZoom:1,userScreenZoom:1.06,followCursor:true,followSmoothingMs:250,cursorAvailable:true,cursorX:computed.cursorX,cursorY:computed.cursorY,motionMode:0,reducedMotion:true}}>
          <SceneSlot name="screen">
            <Video id="continuous-screen" src={vars.screenSource} sourceStartMs={0} sourceEndMs={8000} durationMs={18000} playbackRate={.444444444444} width={1280} height={800} fit="contain" volume={0}/>
            <AssetScene id="recorded-pointer" asset="video.mont.studio" width={1280} height={800} inputs={{viewportWidth:1280,viewportHeight:800,cursorX:computed.cursorX,cursorY:computed.cursorY,cursorPressed:computed.cursorPressed,cursorScale:1.4,cursorVisible:true,cursorAutoHide:false,previewActive:true,cursorSmoothingMs:40,cursorMotionBlur:.5,cursorAutoShape:false,cursorClickAnimation:true}}/>
          </SceneSlot>
          <SceneSlot name="camera"><Video id="continuous-camera" src={vars.cameraSource} sourceStartMs={0} sourceEndMs={18000} durationMs={18000} playbackRate={1} width={1280} height={720} fit="contain" volume={0}/></SceneSlot>
          <SceneSlot name="screenUnderlay"><AssetScene id="glass-screen-border" asset="video.mont.theme.glass:screenUnderlay" width="100%" height="100%" inputs={{viewportWidth:1920,viewportHeight:840,screenWidth:1280,screenHeight:800,cameraWidth:1280,cameraHeight:720,screenRectX:computed.sx,screenRectY:computed.sy,screenRectWidth:computed.sw,screenRectHeight:computed.sh,cameraRectX:computed.cx,cameraRectY:computed.cy,cameraRectWidth:computed.cw,cameraRectHeight:computed.ch,cameraAlignScreen:false,cameraShape:computed.cameraShape,screenRadiusPermille:20,cameraRadiusPermille:24,cameraMirrored:true,screenContain:true,cameraContain:false,cornerEnabled:false,screenThemeTiltDeg:3,screenZoom:1,userScreenZoom:1.06,followCursor:true,followSmoothingMs:250,cursorAvailable:true,cursorX:computed.cursorX,cursorY:computed.cursorY,motionMode:0,reducedMotion:true}}/></SceneSlot>
          <SceneSlot name="cameraUnderlay"><AssetScene id="glass-camera-border" asset="video.mont.theme.glass:cameraUnderlay" width="100%" height="100%" inputs={{viewportWidth:1920,viewportHeight:840,screenWidth:1280,screenHeight:800,cameraWidth:1280,cameraHeight:720,screenRectX:computed.sx,screenRectY:computed.sy,screenRectWidth:computed.sw,screenRectHeight:computed.sh,cameraRectX:computed.cx,cameraRectY:computed.cy,cameraRectWidth:computed.cw,cameraRectHeight:computed.ch,cameraAlignScreen:false,cameraShape:computed.cameraShape,screenRadiusPermille:20,cameraRadiusPermille:24,cameraMirrored:true,screenContain:true,cameraContain:false,cornerEnabled:false,screenThemeTiltDeg:3,screenZoom:1,userScreenZoom:1.06,followCursor:true,followSmoothingMs:250,cursorAvailable:true,cursorX:computed.cursorX,cursorY:computed.cursorY,motionMode:0,reducedMotion:true}}/></SceneSlot>
        </AssetScene>
      </Group>
      <Group id="layout-controls" x={540} y={926+35*computed.end} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.intro*(1-computed.end)}>
        <Rect id="control-shell" width={840} height={80} radius={24} fill="#28332F" shadowColor="#17251F" shadowOpacity={.16} shadowBlur={16} shadowOffsetY={5}/>
        <Rect id="selected-layout" x={5+277*computed.selector} y={5} width={276} height={70} radius={19} fill="#FF854F"/>
        <Text id="corner-label" x={10} y={25} width={265} height={38} text="Corner bubble" fontFamily="Inter" size={26} fontWeight={600} align="center" color={computed.cornerLabelColor}/>
        <Text id="side-label" x={287} y={25} width={265} height={38} text="Side by side" fontFamily="Inter" size={26} fontWeight={600} align="center" color={computed.sideLabelColor}/>
        <Text id="presenter-label" x={564} y={25} width={265} height={38} text="Presenter" fontFamily="Inter" size={26} fontWeight={600} align="center" color={computed.presenterLabelColor}/>
      </Group>
      <Group id="closing" x={135} y={355+25*(1-computed.closing)} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.closing} blur={4*(1-computed.closing)}>
        <Rect id="closing-tile" width={67} height={67} radius={20.1} fill="#FF6B35"/>
        <Path id="closing-logo" width={67} height={67} viewBox="0 0 40 40" d="M10.571 28.444 L16.857 19.111 L20 23.778 L23.143 19.111 L29.429 28.444" fill="none" stroke="#160A04" strokeWidth={3.13} strokeLineJoin="round" strokeLineCap="round"/>
        <Text id="mont" x={88} y={7} width={400} height={70} text="Mont" fontFamily="Inter" fontWeight={600} size={48} letterSpacing={-2} color="#242A28"/>
        <Text id="benefit-first" y={123} width={750} height={90} text="Switch layouts without" fontFamily="Inter" fontWeight={500} size={62} letterSpacing={-2.5} color="#242A28"/>
        <Text id="benefit-second" y={198} width={750} height={90} text="recording again." fontFamily="Inter" fontWeight={500} size={62} letterSpacing={-2.5} color="#242A28"/>
        <Text id="url" y={354} width={400} height={50} text="usemont.com" fontFamily="Inter" size={26} color="#676C65"/>
      </Group>
    </Scene>;
  }
});
