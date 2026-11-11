// Original fictional product UI. Native primitives remain editable inside the host.
export default defineSpanScene({
  id: "feature-launch-product-ui", width: 1360, height: 650, durationMs: 12000,
  variables: { accent: colorVar("#C8F27C"), brand: stringVar("Forma") },
  computeds: {
    play: computed.number(() => Math.max(0,Math.min(1,(time.ms-3600)/3900))),
    detail: computed.number(() => 1-Math.pow(1-Math.max(0,Math.min(1,(time.ms-4800)/600)),3)),
  },
  render({vars,computed}) {
    return <Scene width={1360} height={650}>
      <Rect id="app" width={1360} height={650} fill="#F4F4EE" />
      <Rect id="toolbar" width={1360} height={60} fill="#FCFCF7" />
      <Rect id="toolbar-line" y={59} width={1360} height={1} fill="#DEE1D7" />
      <Rect id="app-mark" x={22} y={18} width={24} height={24} radius={7} fill={vars.accent} />
      <Path id="app-glyph" x={29} y={24} width={11} height={13} viewBox="0 0 11 13" d="M0 13 L0 0 L11 0 L11 3 L4 3 L4 6 L9 6 L9 9 L4 9 L4 13 Z" fill="#27372E" />
      <Text id="app-brand" text={vars.brand} x={57} y={16} width={170} height={30} size={22} fontFamily="Inter" fontWeight={600} letterSpacing={-.8} color="#27372E" />
      <Text id="project" text="A little more possibility" x={475} y={21} width={400} height={25} align="center" size={16} fontFamily="Inter" color="#697368" />
      <Rect id="export" x={1221} y={14} width={114} height={33} radius={9} fill="#27372E" />
      <Text id="export-label" text="Export" x={1221} y={22} width={114} height={24} align="center" size={14} fontFamily="Inter" fontWeight={600} color="#FCFCF7" />
      <Rect id="library-divider" x={229} y={60} width={1} height={380} fill="#DEE1D7" />
      <Text id="library-label" text="Project media" x={22} y={87} width={190} height={30} size={17} fontFamily="Inter" fontWeight={600} color="#27372E" />
      <Rect id="search" x={20} y={130} width={190} height={32} radius={8} fill="#E7EAE0" />
      <Text id="search-copy" text="Search your assets" x={32} y={138} width={170} height={23} size={12} fontFamily="Inter" color="#7C8577" />
      <Repeat items={[{y:184,c:"#CBD5BC",label:"Opening scene"},{y:263,c:"#E0CBA8",label:"Paper study"},{y:342,c:"#CCD7E0",label:"Closing frame"}]} render={(item,index) => (
        <Group id={"asset-"+index} x={20} y={item.y} width={0} height={0} pivotX={0} pivotY={0}>
          <Rect id={"asset-thumb-"+index} width={80} height={60} radius={8} fill={item.c} />
          <Rect id={"asset-art-"+index} x={25} y={13} width={30} height={34} radius={15} fill="#FCFCF7" rotation={index*17-12} />
          <Text id={"asset-title-"+index} text={item.label} x={91} y={12} width={110} height={24} size={12} fontFamily="Inter" fontWeight={500} color="#344338" />
          <Text id={"asset-duration-"+index} text="00:08" x={91} y={34} width={90} height={20} size={11} fontFamily="Inter" color="#87917F" />
        </Group>
      )} />
      <Rect id="preview-well" x={250} y={78} width={844} height={360} radius={12} fill="#E6E9DF" />
      <Rect id="artboard" x={360} y={91} width={620} height={322} fill="#D8E3C8" />
      <Group id="poster-art" x={675} y={255} width={0} height={0} pivotX={0} pivotY={0} rotation={-4+3*computed.play}>
        <Rect id="poster-label" x={-266} y={-132} width={182} height={24} radius={12} fill="#2A4032" />
        <Text id="poster-label-text" text="A NEW PERSPECTIVE" x={-257} y={-126} width={164} height={19} fontFamily="Inter" size={9} fontWeight={600} letterSpacing={1} align="center" color="#EEF0D9" />
        <Text id="poster-title" text={"Make a\nlittle space."} x={-266} y={-69} width={360} height={190} fontFamily="Inter" fontWeight={600} size={62} lineHeight={1.04} letterSpacing={-3.5} color="#294031" />
        <Ellipse id="poster-oval" x={118} y={-97} width={129} height={205} fill="#F7F2D9" rotation={24} />
        <Ellipse id="poster-hole" x={150} y={-40} width={64} height={91} fill="#2C4536" rotation={24} />
        <Text id="poster-footer" text="IDEAS NEED ROOM TO GROW." x={-265} y={128} width={470} height={22} fontFamily="Inter" size={10} letterSpacing={2} color="#294031" />
      </Group>
      <Text id="inspector-title" text="Clip details" x={1120} y={90} width={200} height={28} fontFamily="Inter" size={16} fontWeight={600} color="#27372E" />
      <Text id="opacity-label" text="Opacity" x={1120} y={147} width={140} height={24} fontFamily="Inter" size={13} color="#75816F" />
      <Text id="opacity-value" text="100%" x={1250} y={147} width={75} height={24} align="right" fontFamily="Inter" size={13} color="#344338" />
      <Rect id="opacity-track" x={1120} y={184} width={201} height={4} radius={2} fill="#B5C3A9" />
      <Ellipse id="opacity-handle" x={1311} y={177} width={16} height={16} fill="#344338" />
      <Text id="timing-label" text="Timing" x={1120} y={227} width={200} height={25} fontFamily="Inter" size={13} color="#75816F" />
      <Rect id="timing-box" x={1120} y={264} width={201} height={34} radius={7} fill="#FCFCF7" stroke="#DBE0D2" strokeWidth={1} />
      <Text id="timing-value" text="00:00 — 00:08" x={1131} y={274} width={175} height={22} fontFamily="Inter" size={12} color="#344338" />
      <Rect id="timeline-surface" y={456} width={1360} height={194} fill="#FCFCF7" />
      <Rect id="timeline-rule" y={456} width={1360} height={1} fill="#DEE1D7" />
      <Text id="timeline-label" text="Timeline" x={22} y={477} width={150} height={28} fontFamily="Inter" size={16} fontWeight={600} color="#27372E" />
      <Repeat items={[0,1,2,3,4,5,6,7]} render={(item,index) => (
        <Group id={"ruler-"+index} x={252+136*index} y={478} width={0} height={0} pivotX={0} pivotY={0}>
          <Text id={"ruler-label-"+index} text={"0"+index+":00"} width={65} height={22} fontFamily="Inter" size={11} color="#8B9386" />
          <Rect id={"ruler-tick-"+index} y={24} width={1} height={8} fill="#DCE1D3" />
        </Group>
      )} />
      <Text id="video-track-label" text="Video" x={22} y={530} width={170} height={25} fontFamily="Inter" size={13} color="#7D8876" />
      <Repeat items={[{x:247,w:344,label:"Opening scene",c:"#D6E2C8"},{x:599,w:312,label:"Paper study",c:"#E4D3B7"},{x:919,w:405,label:"Closing frame",c:"#D7E1E5"}]} render={(clip,index) => (
        <Group id={"clip-"+index} x={clip.x} y={519} width={0} height={0} pivotX={0} pivotY={0}>
          <Rect id={"clip-surface-"+index} width={clip.w} height={52} radius={8} fill={clip.c} stroke={index===0 ? "#879D77" : "#00000000"} strokeWidth={2} />
          <Text id={"clip-title-"+index} text={clip.label} x={14} y={17} width={clip.w-28} height={24} fontFamily="Inter" size={13} fontWeight={500} color="#354A35" />
        </Group>
      )} />
      <Text id="sound-track-label" text="Audio" x={22} y={597} width={170} height={25} fontFamily="Inter" size={13} color="#7D8876" />
      <Rect id="audio-bed" x={247} y={586} width={1077} height={36} radius={7} fill="#E9EDD9" />
      <Repeat items={[7,14,10,19,12,6,15,20,9,13,6,18,23,11,17,8,15,21,12,7,18,10,16,22,13,6,15,20,9,14,8,17,24,11,16,7,13,19,10,15]} render={(h,index) => (
        <Rect id={"wave-"+index} x={263+26*index} y={604-h/2} width={8} height={h} radius={4} fill="#ABB793" />
      )} />
      <Rect id="playhead" x={265+620*computed.play} y={499} width={2} height={133} fill="#495E3E" />
      <Ellipse id="playhead-top" x={261+620*computed.play} y={494} width={10} height={10} fill="#495E3E" />
      <Rect id="selected-clip-accent" x={247} y={519} width={344} height={52} radius={8} fill="#00000000" stroke={vars.accent} strokeWidth={3*computed.detail} opacity={computed.detail} />
    </Scene>;
  },
});
