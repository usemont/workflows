function clamp(v) { return Math.max(0, Math.min(1, v)); }
function smooth(t, a, b) { return Math.pow(clamp((t-a)/(b-a)),3)*(10+clamp((t-a)/(b-a))*(-15+6*clamp((t-a)/(b-a)))); }
function arrive(t, a) { return 1-Math.pow(1-clamp((t-a)/850),4); }
export default defineSpanScene({
  id: "lesson-step-localized", width: 1920, height: 1080, durationMs: 14000, editPreviewMs: 10200,
  variables: {
    locale: stringVar("en"), stepNumber: numberVar(2), stepCount: numberVar(4),
    messages: {en: {
      step: "Step", of: "of", topic: "How browser caching works",
      opening: "Your browser", emphasis: "keeps a copy.",
      explanation: "It saves some files on your device,\nso it can reuse them on your next visit.",
      firstVisit: "First visit", nextVisit: "Next visit", browser: "Browser", server: "Server",
      storedCopy: "Saved on your device", result: "Less to download next time.",
      caveat: "The browser checks again when the saved copy expires.",
      font: "Inter", weight: 600, headingSize: 108, bodySize: 34
    }},
    paper: colorVar("#F1F0E9"), ink: colorVar("#253A37"), accent: colorVar("#CB542D"), soft: colorVar("#DDE4DA")
  },
  computeds: {
    t: computed.number(() => time.ms),
    intro: computed.number(() => arrive(computed.t,150)),
    emphasis: computed.number(() => arrive(computed.t,850)),
    body: computed.number(() => arrive(computed.t,1750)),
    diagram: computed.number(() => arrive(computed.t,2500)),
    saved: computed.number(() => smooth(computed.t,4600,5350)),
    repeat: computed.number(() => smooth(computed.t,6500,7350)),
    result: computed.number(() => arrive(computed.t,8500)),
    firstPacket: computed.number(() => smooth(computed.t,3200,4500)),
    localPacket: computed.number(() => smooth(computed.t,7350,8250)),
    dots: computed.points(() => collection.map(collection.range(49),(i,p) => ({x: 1240+420*p,y: 472})))
  },
  render({vars,computed}) {
    return <Scene width={1920} height={1080}>
      <Rect id="paper" width={1920} height={1080} fill={vars.paper}/>
      <Rect id="accent-rule" x={96} y={91} width={48} height={5} radius={2.5} fill={vars.accent}/>
      <Text id="topic" text={vars.messages[vars.locale].topic} x={164} y={76} width={1220} height={45} fontFamily={vars.messages[vars.locale].font} fontSize={24} fontWeight={400} color={vars.ink}/>
      <Text id="progress" text={vars.messages[vars.locale].step+" "+vars.stepNumber+" "+vars.messages[vars.locale].of+" "+vars.stepCount} x={1370} y={76} width={450} height={45} textAlign="right" fontFamily={vars.messages[vars.locale].font} fontSize={24} fontWeight={400} color={vars.ink}/>
      <Group id="opening-motion" width={0} height={0} pivotX={0} pivotY={0} y={45*(1-computed.intro)} opacity={computed.intro}>
        <Text id="opening" text={vars.messages[vars.locale].opening} x={96} y={230} width={960} height={264} fontFamily={vars.messages[vars.locale].font} fontSize={vars.messages[vars.locale].headingSize} fontWeight={vars.messages[vars.locale].weight} lineHeight={1.08} renderConfig={{wrap:true,balance:false}} color={vars.ink}/>
      </Group>
      <Group id="emphasis-motion" width={0} height={0} pivotX={0} pivotY={0} y={42*(1-computed.emphasis)} opacity={computed.emphasis}>
        <Text id="emphasis" text={vars.messages[vars.locale].emphasis} x={96} y={502} width={960} height={255} fontFamily={vars.messages[vars.locale].font} fontSize={vars.messages[vars.locale].headingSize} fontWeight={vars.messages[vars.locale].weight} lineHeight={1.08} renderConfig={{wrap:true,balance:false}} color={vars.accent}/>
      </Group>
      <Text id="explanation" text={vars.messages[vars.locale].explanation} x={101} y={797} width={900} height={145} fontFamily={vars.messages[vars.locale].font} fontSize={vars.messages[vars.locale].bodySize} fontWeight={400} lineHeight={1.25} renderConfig={{wrap:true,balance:false}} color={vars.ink} opacity={computed.body}/>
      <Group id="diagram" width={0} height={0} pivotX={0} pivotY={0} y={30*(1-computed.diagram)} opacity={computed.diagram}>
        <Rect id="diagram-surface" x={1120} y={230} width={704} height={660} radius={32} fill="#FAFAF5"/>
        <Text id="visit-state" text={computed.repeat<0.5?vars.messages[vars.locale].firstVisit:vars.messages[vars.locale].nextVisit} x={1160} y={273} width={624} height={85} fontFamily={vars.messages[vars.locale].font} fontSize={30} fontWeight={400} lineHeight={1.15} color={vars.ink}/>
        <Path id="connection" points={computed.dots} width={1920} height={1080} fill="none" stroke="#D6DDD1" strokeWidth={3} opacity={1-0.8*computed.repeat}/>
        <Ellipse id="server-disc" x={1570} y={382} width={180} height={180} fill={vars.soft} opacity={1-0.66*computed.repeat}/>
        <Rect id="server-slot-one" x={1623} y={430} width={74} height={17} radius={4} fill={vars.ink} opacity={1-0.66*computed.repeat}/>
        <Rect id="server-slot-two" x={1623} y={465} width={74} height={17} radius={4} fill={vars.ink} opacity={1-0.66*computed.repeat}/>
        <Rect id="server-slot-three" x={1623} y={500} width={74} height={17} radius={4} fill={vars.ink} opacity={1-0.66*computed.repeat}/>
        <Text id="server-label" text={vars.messages[vars.locale].server} x={1540} y={585} width={240} height={80} textAlign="center" fontFamily={vars.messages[vars.locale].font} fontSize={26} fontWeight={400} color={vars.ink} opacity={1-0.66*computed.repeat}/>
        <Ellipse id="browser-disc" x={1150} y={382} width={180} height={180} fill={vars.ink}/>
        <Rect id="browser-window" x={1195} y={435} width={90} height={72} radius={7} fill={vars.paper}/>
        <Rect id="browser-toolbar" x={1195} y={449} width={90} height={4} fill={vars.ink}/>
        <Text id="browser-label" text={vars.messages[vars.locale].browser} x={1140} y={585} width={220} height={80} textAlign="center" fontFamily={vars.messages[vars.locale].font} fontSize={26} fontWeight={400} color={vars.ink}/>
        <Rect id="incoming-file" x={1645-420*computed.firstPacket} y={457} width={30} height={30} radius={6} fill={vars.accent} opacity={smooth(computed.t,3150,3300)*(1-smooth(computed.t,4500,4700))}/>
        <Group id="stored-copy" width={0} height={0} pivotX={0} pivotY={0} y={18*(1-computed.saved)} opacity={computed.saved}>
          <Rect id="copy-connector" x={1238} y={647} width={4} height={50} fill={vars.accent}/>
          <Rect id="copy-chip" x={1170} y={705} width={600} height={116} radius={18} fill={vars.soft}/>
          <Rect id="copy-icon" x={1202} y={741} width={34} height={42} radius={6} fill={vars.accent}/>
          <Text id="copy-label" text={vars.messages[vars.locale].storedCopy} x={1260} y={736} width={475} height={60} fontFamily={vars.messages[vars.locale].font} fontSize={28} fontWeight={400} color={vars.ink}/>
        </Group>
        <Rect id="local-file" x={1225} y={738-266*computed.localPacket} width={30} height={30} radius={6} fill={vars.accent} opacity={smooth(computed.t,7300,7500)*(1-smooth(computed.t,8250,8500))}/>
      </Group>
      <Group id="result-motion" width={0} height={0} pivotX={0} pivotY={0} y={18*(1-computed.result)} opacity={computed.result}>
        <Text id="result" text={vars.messages[vars.locale].result} x={1120} y={926} width={704} height={60} fontFamily={vars.messages[vars.locale].font} fontSize={32} fontWeight={400} color={vars.accent}/>
      </Group>
      <Text id="caveat" text={vars.messages[vars.locale].caveat} x={100} y={980} width={970} height={50} fontFamily={vars.messages[vars.locale].font} fontSize={22} fontWeight={400} lineHeight={1.2} color={vars.ink} opacity={computed.result*0.7}/>
      <Rect id="progress-track" x={100} y={1046} width={1724} height={3} fill="#DBDFD5"/>
      <Rect id="progress-fill" x={100} y={1046} width={1724*clamp(computed.t/14000)} height={3} fill={vars.accent}/>
    </Scene>;
  }
});
