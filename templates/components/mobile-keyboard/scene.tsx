// Reusable 540 × 326 keyboard. Host owns timing; pressed is the actual character.
export default defineSpanScene({
  id: "mobile-keyboard", width: 540, height: 326, durationMs: 11500,
  variables: {
    pressed: stringVar(""), uppercase: booleanVar(false), symbols: booleanVar(false),
    surface: colorVar("#FFFFFF"), foreground: colorVar("#202622"),
    muted: colorVar("#7D847F"), background: colorVar("#E3E5DF"),
    secondary: colorVar("#E8EDE6"), accent: colorVar("#CDE7AF"),
  },
  computeds: {
    pressed: computed.string(() => vars.pressed),
    uppercase: computed.boolean(() => vars.uppercase),
    symbols: computed.boolean(() => vars.symbols),
  },
  render({vars, computed}) {
    return <Scene width={540} height={326}>
      <Rect id="background" width={540} height={326} fill={vars.background} />
      <Repeat items={[
        {letters: [{lower:"q",upper:"Q",symbol:"1"},{lower:"w",upper:"W",symbol:"2"},{lower:"e",upper:"E",symbol:"3"},{lower:"r",upper:"R",symbol:"4"},{lower:"t",upper:"T",symbol:"5"},{lower:"y",upper:"Y",symbol:"6"},{lower:"u",upper:"U",symbol:"7"},{lower:"i",upper:"I",symbol:"8"},{lower:"o",upper:"O",symbol:"9"},{lower:"p",upper:"P",symbol:"0"}], x:8,y:34},
        {letters: [{lower:"a",upper:"A",symbol:"-"},{lower:"s",upper:"S",symbol:"/"},{lower:"d",upper:"D",symbol:":"},{lower:"f",upper:"F",symbol:";"},{lower:"g",upper:"G",symbol:"("},{lower:"h",upper:"H",symbol:")"},{lower:"j",upper:"J",symbol:"$"},{lower:"k",upper:"K",symbol:"&"},{lower:"l",upper:"L",symbol:"@"}], x:35,y:94},
        {letters: [{lower:"z",upper:"Z",symbol:"."},{lower:"x",upper:"X",symbol:","},{lower:"c",upper:"C",symbol:"?"},{lower:"v",upper:"V",symbol:"!"},{lower:"b",upper:"B",symbol:"'"},{lower:"n",upper:"N",symbol:'"'},{lower:"m",upper:"M",symbol:"_"}], x:89,y:154},
      ]} render={(row,r)=>(
        <Repeat items={row.letters} render={(key,k)=>(
          <Group id={"key-"+r+"-"+k} x={row.x+k*53} y={row.y} width={0} height={0} pivotX={0} pivotY={0}>
            <Rect id={"shadow-"+r+"-"+k} y={2} width={47} height={49} radius={7} fill={vars.muted}/>
            <Rect id={"face-"+r+"-"+k} width={47} height={49} radius={7}
              fill={(computed.symbols ? computed.pressed === key.symbol : computed.pressed === key.lower || computed.pressed === key.upper) ? vars.accent : vars.surface}/>
            <Text id={"lower-"+r+"-"+k} text={key.lower} y={6} width={47} height={35} align="center" fontFamily="Inter" weight={400} size={25} lineHeight={34} color={vars.foreground} opacity={!computed.symbols && !computed.uppercase ? 1 : 0}/>
            <Text id={"upper-"+r+"-"+k} text={key.upper} y={6} width={47} height={35} align="center" fontFamily="Inter" weight={400} size={25} lineHeight={34} color={vars.foreground} opacity={!computed.symbols && computed.uppercase ? 1 : 0}/>
            <Text id={"symbol-"+r+"-"+k} text={key.symbol} y={6} width={47} height={35} align="center" fontFamily="Inter" weight={400} size={25} lineHeight={34} color={vars.foreground} opacity={computed.symbols ? 1 : 0}/>
          </Group>
        )}/>
      )}/>
      <Rect id="shift-face" x={8} y={154} width={63} height={49} radius={7} fill={computed.uppercase && !computed.symbols ? vars.accent : vars.secondary}/>
      <Text id="shift" text="↑" x={15} y={161} width={48} height={34} align="center" fontFamily="Inter" weight={400} size={25} lineHeight={34} color={vars.foreground}/>
      <Rect id="delete-face" x={469} y={154} width={63} height={49} radius={7} fill={computed.pressed === "Backspace" ? vars.accent : vars.secondary}/>
      <Text id="delete" text="⌫" x={477} y={161} width={48} height={34} align="center" fontFamily="Inter" weight={400} size={25} lineHeight={34} color={vars.foreground}/>
      <Rect id="numeric-key" x={8} y={217} width={104} height={48} radius={7} fill={vars.secondary}/>
      <Text id="numeric-label" text="123" x={8} y={225} width={104} height={34} align="center" fontFamily="Inter" weight={400} size={20} lineHeight={30} color={vars.foreground} opacity={computed.symbols ? 0 : 1}/>
      <Text id="alphabet-label" text="ABC" x={8} y={225} width={104} height={34} align="center" fontFamily="Inter" weight={400} size={20} lineHeight={30} color={vars.foreground} opacity={computed.symbols ? 1 : 0}/>
      <Rect id="space-key" x={119} y={217} width={302} height={48} radius={7} fill={computed.pressed === " " ? vars.accent : vars.surface}/>
      <Text id="space-label" text="space" x={119} y={225} width={302} height={34} align="center" fontFamily="Inter" weight={400} size={18} lineHeight={30} color={vars.foreground}/>
      <Rect id="return-key" x={428} y={217} width={104} height={48} radius={7} fill={computed.pressed === "Enter" ? vars.accent : vars.secondary}/>
      <Text id="return-label" text="return" x={428} y={225} width={104} height={34} align="center" fontFamily="Inter" weight={400} size={18} lineHeight={30} color={vars.foreground}/>
    </Scene>;
  },
});
