function clamp(v) { return Math.max(0,Math.min(1,v)); }
function ease(t,a,b) { return Math.pow(clamp((t-a)/(b-a)),3)*(10+clamp((t-a)/(b-a))*(-15+6*clamp((t-a)/(b-a)))); }
function enter(t,a,b) { return 1-Math.pow(1-clamp((t-a)/(b-a)),4); }
function camera(t) { return 1200*ease(t,5300,6900)+1200*ease(t,9800,11400)-1200*ease(t,14500,16600); }
function zoom(t) { return 1-.55*ease(t,14500,16600); }
function count(t,a,b,value) { return Math.round(value*enter(t,a,b)); }
function digits(v) { return v>=100000?6:v>=10000?5:v>=1000?4:v>=100?3:v>=10?2:1; }
function glyph(v) { return v===0?"0":v===1?"1":v===2?"2":v===3?"3":v===4?"4":v===5?"5":v===6?"6":v===7?"7":v===8?"8":"9"; }
function cardX(i,t) { return -340+(i%3)*330+900*(1-enter(t,400+i*160,1900+i*160)); }
function cardY(i,t) { return -65+Math.floor(i/3)*163-32*Math.sin(Math.PI*enter(t,400+i*160,1900+i*160)); }
export default defineSpanScene({
  id:"mont-unwrapped",width:1920,height:1080,durationMs:19000,editPreviewMs:17200,
  variables:{name:stringVar("Ada"),company:stringVar("Northline"),product:stringVar("Mont"),year:numberVar(2025),videos:numberVar(48),minutes:numberVar(1260),favouriteLayout:stringVar("Side by side"),link:stringVar("usemont.com"),logoImage:stringVar("resource:mont-unwrapped-logo"),accent:colorVar("#FF6B35"),seriesLabel:stringVar("Mont Unwrapped"),yearLabel:stringVar("2025"),greetingLabel:stringVar("Ada,"),greetingSize:numberVar(114),recipientLabel:stringVar("Ada / 2025"),companyLabel:stringVar("Northline, made in Mont."),summaryLabel:stringVar("48 videos  /  1260 minutes  /  Side by side")},
  tokens:{
    films:[{i:0,color:"#E3A46F",word:"Launch",tilt:-4},{i:1,color:"#B5C2B2",word:"Lesson",tilt:3},{i:2,color:"#C4B5CC",word:"Update",tilt:-2},{i:3,color:"#A6BFC5",word:"Demo",tilt:2},{i:4,color:"#E9C378",word:"Story",tilt:-3},{i:5,color:"#C8BCAC",word:"Recap",tilt:4}],
    holes:[0,1,2,3,4,5],ticks:[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23],
    places:[0,1,2,3,4,5],
    tracks:[{i:0,w:270,x:0,color:"#D9B89C"},{i:1,w:190,x:160,color:"#B5C2B2"},{i:2,w:255,x:65,color:"#C4B5CC"}],
    glow:'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1920 1080"><defs><radialGradient id="g"><stop stop-color="%23544A3C" stop-opacity=".55"/><stop offset="1" stop-color="%23191B1D" stop-opacity="0"/></radialGradient></defs><rect width="1920" height="1080" fill="url(%23g)"/></svg>'
  },
  computeds:{
    t:computed.number(()=>time.ms),intro:computed.number(()=>enter(computed.t,100,1050)*(1-ease(computed.t,2000,2650))),
    stats:computed.number(()=>enter(computed.t,2350,3000)),reel:computed.number(()=>enter(computed.t,5900,6900)),layout:computed.number(()=>enter(computed.t,10400,11400)),
    end:computed.number(()=>ease(computed.t,14500,16600)),cx:computed.number(()=>camera(computed.t)),scale:computed.number(()=>zoom(computed.t)),
    nVideos:computed.number(()=>count(computed.t,2550,4200,vars.videos)),nMinutes:computed.number(()=>count(computed.t,6800,8700,vars.minutes)),
    ribbon:computed.points(()=>collection.map(collection.range(97),(i,p)=>({x:460+1980*p,y:215+42*Math.sin(p*12.5663706144)})))
  },
  render({vars,computed}) {
    return <Scene width={1920} height={1080}>
      <Rect id="charcoal" width={1920} height={1080} fill="#191B1D"/>
      <Image id="warm-pool" src={tokens.glow} width={1920} height={1080}/>
      <Image id="logo-preload" src={vars.logoImage} x={-4096} y={-4096} width={1} height={1}/>
      <Repeat items={tokens.ticks} render={(i)=>(<Rect id={"desk-rule-"+i} x={i*90-70+10*Math.sin(computed.t/12000)} y={0} width={1} height={1080} fill="#D3C4B0" opacity={.025}/>)} />
      <Rect id="poster" x={80} y={120} width={1760} height={855} radius={28} fill="#222426" stroke="#66635D" strokeWidth={1.5} opacity={computed.end}/>
      <Image id="mont-logo" src={vars.logoImage} x={94} y={59} width={44} height={44} fit="contain"/>
      <Text id="series" text={vars.seriesLabel} x={154} y={65} width={900} height={42} fontFamily="Inter" size={25} fontWeight={500} color="#F3EDE0"/>
      <Text id="year" text={vars.yearLabel} x={1560} y={63} width={260} height={45} align="right" fontFamily="Inter" size={26} color="#A9A49A"/>
      <Group id="greeting" width={0} height={0} pivotX={0} pivotY={0} x={490} y={224+30*(1-computed.intro)} opacity={computed.intro}>
        <Text id="recipient" text={vars.greetingLabel} width={1350} height={140} fontFamily="Inter" size={vars.greetingSize} fontWeight={600} letterSpacing={-4} color="#F3EDE0"/>
        <Text id="opening" text="here’s what you made." y={135} width={1100} height={80} fontFamily="Inter" size={52} letterSpacing={-2} color="#BEB8AB"/>
      </Group>
      <Group id="workbench-camera" x={960} y={590-10*computed.end+3*Math.sin(computed.t/3200)} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale} rotation={-.35*Math.sin(computed.t/4200)*(1-computed.end)}>
        <Group id="connected-set" x={-computed.cx} width={0} height={0} pivotX={0} pivotY={0}>
          <Path id="continuous-film-ribbon" points={computed.ribbon} width={3400} height={600} stroke="#7C7568" strokeWidth={24} fill="none" opacity={.25}/>
          <Path id="ribbon-highlight" points={computed.ribbon} width={3400} height={600} stroke="#CBBFA9" strokeWidth={1} fill="none" opacity={.4}/>
          <Group id="contact-sheet" width={0} height={0} pivotX={0} pivotY={0}>
            <Group id="video-caption" width={0} height={0} pivotX={0} pivotY={0} opacity={computed.stats}>
              <Repeat items={tokens.places} render={(index) => (<Text id={"video-digit-" + index} text={glyph(Math.floor(computed.nVideos / Math.pow(10, Math.max(0, digits(computed.nVideos) - 1 - index))) % 10)} x={-470 + index * 88} y={-346} width={100} height={165} fontFamily="Inter" size={142} fontWeight={600} letterSpacing={-8} color="#F3EDE0" opacity={digits(computed.nVideos) > index ? 1 : 0} />)} />
              <Text id="video-unit" text="videos made" x={-464} y={-184} width={720} height={64} fontFamily="Inter" size={40} color="#BEB8AB"/>
            </Group>
            <Rect id="film-backing" x={-480} y={-83} width={990} height={398} radius={16} fill="#101213" stroke="#474742" strokeWidth={2} shadowColor="#000000" shadowOpacity={.35} shadowBlur={38} shadowOffsetY={25}/>
            <Repeat items={tokens.ticks} render={(i)=>(<Group id={"perforation-"+i} x={-457+i*40} width={0} height={0} pivotX={0} pivotY={0}>
              <Rect id={"top-hole-"+i} y={-66} width={19} height={12} radius={3} fill="#777064"/>
              <Rect id={"bottom-hole-"+i} y={283} width={19} height={12} radius={3} fill="#777064"/>
            </Group>)} />
            <Repeat items={tokens.films} render={(f)=>(<Group id={"clip-"+f.i} x={cardX(f.i,computed.t)} y={cardY(f.i,computed.t)} width={0} height={0} pivotX={0} pivotY={0} rotation={f.tilt*(1-enter(computed.t,600+f.i*160,2100+f.i*160))} opacity={enter(computed.t,400+f.i*160,1000+f.i*160)}>
              <Rect id={"clip-paper-"+f.i} x={-106} y={30} width={218} height={135} radius={5} fill={f.color}/>
              <Ellipse id={"clip-sun-"+f.i} x={-75+f.i*5} y={49} width={54+f.i*7} height={54+f.i*7} fill="#F6EFDF"/>
              <Rect id={"clip-stripe-"+f.i} x={-28} y={42} width={12} height={65} rotation={24+f.i*8} fill="#262B2C" opacity={.75}/>
              <Text id={"clip-title-"+f.i} text={f.word} x={-89} y={115} width={190} height={33} fontFamily="Inter" fontWeight={600} size={22} color="#262B2C"/>
            </Group>)} />
          </Group>
          <Group id="reel-station" x={1200} width={0} height={0} pivotX={0} pivotY={0}>
            <Repeat items={tokens.places} render={(index) => (<Text id={"minutes-digit-" + index} text={glyph(Math.floor(computed.nMinutes / Math.pow(10, Math.max(0, digits(computed.nMinutes) - 1 - index))) % 10)} x={-465 + index * 88} y={-346} width={100} height={165} fontFamily="Inter" size={142} fontWeight={600} letterSpacing={-8} color="#F3EDE0" opacity={computed.reel * (digits(computed.nMinutes) > index ? 1 : 0)} />)} />
            <Text id="minutes-unit" text="minutes on your timeline" x={-459} y={-184} width={930} height={64} fontFamily="Inter" size={40} color="#BEB8AB" opacity={computed.reel}/>
            <Group id="turning-reel" x={-265} y={111} width={0} height={0} pivotX={0} pivotY={0} rotation={-170*enter(computed.t,5900,9600)-computed.t*.0018}>
              <Ellipse id="reel-shadow" x={-147} y={-137} width={294} height={294} fill="#0D0E0F"/>
              <Ellipse id="metal-reel" x={-146} y={-146} width={292} height={292} fill="#AAA699" stroke="#D8D1BD" strokeWidth={3}/>
              <Ellipse id="inner-spool" x={-115} y={-115} width={230} height={230} fill="#77766E" stroke="#C4BFB0" strokeWidth={2}/>
              <Repeat items={tokens.holes} render={(i)=>(<Ellipse id={"reel-hole-"+i} x={77*Math.cos(i*Math.PI/3)-26} y={77*Math.sin(i*Math.PI/3)-26} width={52} height={52} fill="#252829" stroke="#C4BFB0" strokeWidth={2}/>)} />
              <Ellipse id="orange-hub" x={-27} y={-27} width={54} height={54} fill={vars.accent}/>
              <Ellipse id="hub-pin" x={-8} y={-8} width={16} height={16} fill="#2B2A27"/>
            </Group>
            <Rect id="timeline-bed" x={-65} y={-16} width={540} height={286} radius={14} fill="#121617" stroke="#4D504B" strokeWidth={2}/>
            <Repeat items={tokens.tracks} render={(d)=>(<Rect id={"timeline-clip-"+d.i} x={-41+d.x} y={28+d.i*72} width={d.w*enter(computed.t,6500+d.i*230,7800+d.i*230)} height={48} radius={6} fill={d.color}/>)} />
            <Repeat items={tokens.ticks} render={(i)=>(<Rect id={"ruler-"+i} x={-41+i*21} y={1} width={1} height={i%4===0?12:6} fill="#A7A89D"/>)} />
            <Rect id="playhead" x={-40+475*ease(computed.t,6750,9250)} y={-22} width={3} height={308} fill={vars.accent}/>
            <Rect id="playhead-tab" x={-46+475*ease(computed.t,6750,9250)} y={-27} width={15} height={14} radius={3} fill={vars.accent}/>
          </Group>
          <Group id="layout-station" x={2400} width={0} height={0} pivotX={0} pivotY={0}>
            <Text id="layout-heading" text="Your favourite layout" x={-465} y={-325} width={950} height={80} fontFamily="Inter" size={48} fontWeight={500} letterSpacing={-2} color="#BEB8AB" opacity={computed.layout}/>
            <Text id="layout-name" text={vars.favouriteLayout} x={-465} y={-247} width={1100} height={100} fontFamily="Inter" size={72} fontWeight={600} letterSpacing={-3} color="#F3EDE0" opacity={computed.layout}/>
            <Rect id="glass-layout" x={-470} y={-100} width={950} height={424} radius={25} fill="#393B38" stroke="#88877C" strokeWidth={2} shadowColor="#000000" shadowOpacity={.3} shadowBlur={35} shadowOffsetY={20}/>
            <Group id="layout-screen" x={-445} y={-73} width={0} height={0} pivotX={0} pivotY={0}>
              <Rect id="screen-frame" width={880+((vars.favouriteLayout==="Presenter"?330:vars.favouriteLayout==="Corner bubble"?880:550)-880)*enter(computed.t,11300,12750)} height={365} radius={14} fill="#ECE7DA"/>
              <Rect id="screen-toolbar" x={14} y={14} width={852+((vars.favouriteLayout==="Presenter"?330:vars.favouriteLayout==="Corner bubble"?880:550)-880)*enter(computed.t,11300,12750)} height={25} radius={7} fill="#D4CBBB"/>
              <Ellipse id="screen-art-disc" x={40} y={73} width={150} height={150} fill={vars.accent}/>
              <Rect id="screen-art-panel" x={112} y={114} width={185} height={101} radius={10} fill="#344947" rotation={-8}/>
              <Rect id="screen-track-one" x={30} y={265} width={235} height={25} radius={5} fill="#BBB3CB"/>
              <Rect id="screen-track-two" x={90} y={300} width={205} height={25} radius={5} fill="#A3B7AC"/>
            </Group>
            <Group id="layout-camera" x={vars.favouriteLayout==="Corner bubble"?210:vars.favouriteLayout==="Presenter"?-90:145} y={vars.favouriteLayout==="Corner bubble"?105:-73} width={0} height={0} pivotX={0} pivotY={0} scaleX={.6+.4*enter(computed.t,11300,12750)} scaleY={.6+.4*enter(computed.t,11300,12750)} opacity={enter(computed.t,11200,11900)}>
              <Rect id="camera-frame" width={vars.favouriteLayout==="Presenter"?545:vars.favouriteLayout==="Corner bubble"?215:310} height={vars.favouriteLayout==="Corner bubble"?215:365} radius={vars.favouriteLayout==="Corner bubble"?107.5:14} fill="#B5C2B2" stroke="#E0E4D4" strokeWidth={3}/>
              <Ellipse id="presenter-head" x={vars.favouriteLayout==="Presenter"?232.5:vars.favouriteLayout==="Corner bubble"?67.5:115} y={vars.favouriteLayout==="Corner bubble"?36:65} width={80} height={80} fill="#4A5C54"/>
              <Rect id="presenter-body" x={vars.favouriteLayout==="Presenter"?188.5:vars.favouriteLayout==="Corner bubble"?23.5:71} y={vars.favouriteLayout==="Corner bubble"?123:175} width={168} height={vars.favouriteLayout==="Corner bubble"?58:151} radius={29} fill="#4A5C54"/>
            </Group>
          </Group>
        </Group>
      </Group>
      <Group id="poster-type" x={152} y={174+20*(1-computed.end)} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.end}>
        <Text id="poster-recipient" text={vars.recipientLabel} width={1450} height={98} fontFamily="Inter" size={76} fontWeight={600} letterSpacing={-3} color="#F3EDE0"/>
        <Text id="poster-company" text={vars.companyLabel} y={113} width={1400} height={65} fontFamily="Inter" size={34} color="#BEB8AB"/>
        <Rect id="poster-rule" y={635} width={1616} height={1} fill="#625F56"/>
        <Text id="poster-summary" text={vars.summaryLabel} y={668} width={1500} height={65} fontFamily="Inter" size={30} color="#F3EDE0"/>
      </Group>
      <Text id="footer" text={vars.link} x={100} y={1008} width={1700} height={44} align="right" fontFamily="Inter" size={24} color="#A9A49A" opacity={enter(computed.t,16400,17400)}/>
    </Scene>;
  }
});
