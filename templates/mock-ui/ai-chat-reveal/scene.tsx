// One editable timing track drives both the composer and the reusable keyboard scene.
export default defineSpanScene({
  id: "ai-chat-reveal",
  width: 540,
  height: 960,
  durationMs: 11500,
  editPreviewMs: 10000,
  variables: {
    width: numberVar(540), height: numberVar(960),
    assistant: stringVar("Assistant"),
    prompt: stringVar("Can I make a video just by chatting?"),
    placeholder: stringVar("Ask anything"),
    background: colorVar("#FAF9F6"), surface: colorVar("#FFFFFF"),
    foreground: colorVar("#202622"), muted: colorVar("#7D847F"),
    bubble: colorVar("#E8EDE6"), edge: colorVar("#E1E4DE"),
    accent: colorVar("#CDE7AF"), keyboard: colorVar("#E3E5DF"),
    typingMs: numberVar(2700), thinkingMs: numberVar(650),
    streamMs: numberVar(4300), readingHoldMs: numberVar(2200),
  },
  tokens: {
    // Relative milliseconds at the default typing speed; count is the visible prefix.
    typing: [
      {atMs: -10000, count: 0, character: ""},
      {atMs: 0, count: 1, character: "C"}, {atMs: 80, count: 2, character: "a"}, {atMs: 150, count: 3, character: "n"},
      {atMs: 210, count: 4, character: " "}, {atMs: 340, count: 5, character: "I"}, {atMs: 450, count: 6, character: " "},
      {atMs: 520, count: 7, character: "m"}, {atMs: 590, count: 8, character: "a"}, {atMs: 660, count: 9, character: "k"},
      {atMs: 720, count: 10, character: "e"}, {atMs: 850, count: 11, character: " "}, {atMs: 950, count: 12, character: "a"},
      {atMs: 1030, count: 13, character: " "}, {atMs: 1100, count: 14, character: "v"}, {atMs: 1180, count: 15, character: "i"},
      {atMs: 1260, count: 16, character: "d"}, {atMs: 1340, count: 17, character: "e"}, {atMs: 1410, count: 18, character: "o"},
      {atMs: 1540, count: 19, character: " "}, {atMs: 1630, count: 20, character: "j"}, {atMs: 1710, count: 21, character: "u"},
      {atMs: 1800, count: 22, character: "s"}, {atMs: 1870, count: 23, character: "t"}, {atMs: 2000, count: 24, character: " "},
      {atMs: 2080, count: 25, character: "b"}, {atMs: 2170, count: 26, character: "y"}, {atMs: 2260, count: 27, character: " "},
      {atMs: 2330, count: 28, character: "c"}, {atMs: 2400, count: 29, character: "h"}, {atMs: 2460, count: 30, character: "a"},
      {atMs: 2510, count: 31, character: "t"}, {atMs: 2560, count: 32, character: "t"}, {atMs: 2600, count: 33, character: "i"},
      {atMs: 2640, count: 34, character: "n"}, {atMs: 2670, count: 35, character: "g"}, {atMs: 2700, count: 36, character: "?"},
    ],
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width / 540, vars.height / 960)),
    typeStart: computed.number(() => 800),
    typeEnd: computed.number(() => computed.typeStart + Math.max(800, vars.typingMs)),
    sendAt: computed.number(() => computed.typeEnd + 300),
    replyAt: computed.number(() => computed.sendAt + 360 + Math.max(250, vars.thinkingMs)),
    streamMs: computed.number(() => Math.max(1800, vars.streamMs)),
    streamEnd: computed.number(() => computed.replyAt + computed.streamMs),
    endAt: computed.number(() => computed.streamEnd + Math.max(1000, vars.readingHoldMs)),
    clock: computed.number(() => Math.min(time.ms, computed.endAt)),
    keyboardIn: computed.number(() => 1 - Math.pow(1 - Math.max(0, Math.min(1, (computed.clock - 180) / 340)), 3)),
    keyboardOut: computed.number(() => Math.pow(Math.max(0, Math.min(1, (computed.clock - computed.sendAt) / 360)), 2)),
    keyboardLift: computed.number(() => computed.keyboardIn * (1 - computed.keyboardOut)),
    typingClock: computed.number(() => (computed.clock - computed.typeStart) * 2700 / Math.max(800, vars.typingMs)),
    active: computed.struct(() => collection.topBy(tokens.typing, (event) => event.atMs <= computed.typingClock ? event.atMs : -100000, 1)[0]),
    pressed: computed.string(() => computed.active.count > 0 && computed.typingClock - computed.active.atMs < 55 ? computed.active.character : ""),
    uppercase: computed.boolean(() => text.lower(computed.active.character) !== computed.active.character),
    symbols: computed.boolean(() => computed.active.character === "?"),
    bubbleIn: computed.number(() => 1 - Math.pow(1 - Math.max(0, Math.min(1, (computed.clock - computed.sendAt) / 260)), 3)),
    typed: computed.number(() => computed.active.count),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="portrait" x={(vars.width - 540 * computed.scale) / 2} y={(vars.height - 960 * computed.scale) / 2}
        width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Group id="chat" width={0} height={0} pivotX={0} pivotY={0}>
          <Text id="clock" text="9:41" x={30} y={16} width={70} height={28} fontFamily="Inter" weight={600} size={16} lineHeight={24} color={vars.foreground} />
          <Repeat items={[6, 10, 14, 18]} render={(height, i) => (
            <Rect id={"signal-" + i} x={439 + i * 5} y={36 - height} width={3} height={height} radius={1} fill={vars.foreground} />
          )} />
          <Rect id="battery" x={475} y={20} width={29} height={14} radius={4} fill={vars.background} stroke={vars.foreground} strokeWidth={1.3} />
          <Rect id="battery-fill" x={478} y={23} width={21} height={8} radius={2} fill={vars.foreground} />
          <Rect id="battery-tip" x={505} y={24} width={2} height={6} radius={1} fill={vars.foreground} />
          <Rect id="menu-top" x={28} y={76} width={20} height={2} radius={1} fill={vars.foreground} />
          <Rect id="menu-bottom" x={28} y={83} width={14} height={2} radius={1} fill={vars.foreground} />
          <Text id="assistant-name" text={vars.assistant} x={120} y={63} width={300} height={36} align="center" fontFamily="Inter" size={20} weight={600} lineHeight={30} color={vars.foreground} />
          <Rect id="new-chat-vertical" x={495} y={72} width={2} height={19} radius={1} fill={vars.foreground} />
          <Rect id="new-chat-horizontal" x={486} y={80} width={20} height={2} radius={1} fill={vars.foreground} />
          <Rect id="header-rule" x={24} y={115} width={492} height={1} fill={vars.edge} />
          <Text id="greeting" text="What will you create?" x={35} y={290} width={470} height={45} align="center" fontFamily="Inter" size={28} weight={500} lineHeight={38} color={vars.foreground} opacity={1 - computed.bubbleIn} />
          <Group id="sent" y={12 * (1 - computed.bubbleIn)} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.bubbleIn}>
            <Rect id="user-bubble" x={75} y={145} width={435} height={62} radius={25} fill={vars.bubble} />
            <Text id="sent-prompt" text={vars.prompt} x={94} y={161} width={399} height={34} fontFamily="Inter" weight={400} size={20} lineHeight={30} color={vars.foreground} />
          </Group>
          <Rect id="thinking-dot" x={32} y={256} width={11} height={11} radius={5.5} fill={vars.foreground}
            opacity={computed.clock >= computed.sendAt + 360 && computed.clock < computed.replyAt ? 0.55 + 0.35 * Math.sin((computed.clock - computed.sendAt) / 130) : 0} />
          {/* Each word owns a prefix for native measured positioning and a token beat. No font-width guesses. */}
          <Repeat items={[
            {copy: "Yes.", prefix: "", row: 0, beat: 0},
            {copy: "Connect", prefix: "Yes. ", row: 0, beat: 1},
            {copy: "Mont", prefix: "Yes. Connect ", row: 0, beat: 2},
            {copy: "MCP", prefix: "Yes. Connect Mont ", row: 0, beat: 2},
            {copy: "to", prefix: "Yes. Connect Mont MCP ", row: 0, beat: 3},
            {copy: "your", prefix: "Yes. Connect Mont MCP to ", row: 0, beat: 4},
            {copy: "AI", prefix: "", row: 1, beat: 5},
            {copy: "assistant.", prefix: "AI ", row: 1, beat: 5},
            {copy: "Then", prefix: "AI assistant. ", row: 1, beat: 7},
            {copy: "describe", prefix: "AI assistant. Then ", row: 1, beat: 8},
            {copy: "your", prefix: "AI assistant. Then describe ", row: 1, beat: 9},
            {copy: "video:", prefix: "AI assistant. Then describe your ", row: 1, beat: 9},
            {copy: "A", prefix: "", row: 3, beat: 12},
            {copy: "vertical", prefix: "A ", row: 3, beat: 13},
            {copy: "teaser", prefix: "A vertical ", row: 3, beat: 14},
            {copy: "for", prefix: "A vertical teaser ", row: 3, beat: 14},
            {copy: "my", prefix: "A vertical teaser for ", row: 3, beat: 15},
            {copy: "next", prefix: "A vertical teaser for my ", row: 3, beat: 16},
            {copy: "launch.", prefix: "", row: 4, beat: 17},
            {copy: "Review", prefix: "", row: 6, beat: 20},
            {copy: "the", prefix: "Review ", row: 6, beat: 20},
            {copy: "scenes.", prefix: "Review the ", row: 6, beat: 21},
            {copy: "Refine", prefix: "Review the scenes. ", row: 6, beat: 23},
            {copy: "the", prefix: "Review the scenes. Refine ", row: 6, beat: 24},
            {copy: "timing.", prefix: "", row: 7, beat: 25},
            {copy: "Ask", prefix: "", row: 9, beat: 28},
            {copy: "for", prefix: "Ask ", row: 9, beat: 29},
            {copy: "an", prefix: "Ask for ", row: 9, beat: 29},
            {copy: "MP4", prefix: "Ask for an ", row: 9, beat: 30},
            {copy: "export.", prefix: "Ask for an MP4 ", row: 9, beat: 31},
          ]} render={(word, index) => (
            <Repeat items={[text.glyphs(word.prefix + "M", {fontFamily: "Inter", fontWeight: 400, fontSize: 22})[0]]} render={(prefix) => (
              <Group id={"word-" + index} x={32 + (word.row >= 3 ? 24 : 0) + prefix.totalAdvance - text.glyphs("M", {fontFamily: "Inter", fontWeight: 400, fontSize: 22})[0].totalAdvance}
                y={242 + word.row * 32} width={0} height={0} pivotX={0} pivotY={0}
                opacity={computed.clock >= computed.replyAt + word.beat * computed.streamMs / 32 ? 1 : 0}>
                <Text id={"fresh-word-" + index} text={word.copy} width={170} height={36} fontFamily="Inter" weight={400} size={22} lineHeight={32} color={vars.muted} />
                <Text id={"inked-word-" + index} text={word.copy} width={170} height={36} fontFamily="Inter" weight={400} size={22} lineHeight={32} color={vars.foreground}
                  opacity={Math.max(0, Math.min(1, (computed.clock - computed.replyAt - word.beat * computed.streamMs / 32) / 150))} />
              </Group>
            )} />
          )} />
          <Repeat items={[{row: 3, beat: 12}, {row: 6, beat: 20}, {row: 9, beat: 28}]} render={(bullet, i) => (
            <Rect id={"bullet-" + i} x={34} y={257 + bullet.row * 32} width={5} height={5} radius={2.5} fill={vars.foreground}
              opacity={computed.clock >= computed.replyAt + bullet.beat * computed.streamMs / 32 ? 1 : 0} />
          )} />
          <Group id="composer" x={24} y={846 - 300 * computed.keyboardLift} width={0} height={0} pivotX={0} pivotY={0}>
            <Rect id="composer-border" width={492} height={64} radius={32} fill={vars.edge} />
            <Rect id="composer-fill" x={1} y={1} width={490} height={62} radius={31} fill={vars.surface} />
            <Rect id="attach-horizontal" x={18} y={31} width={17} height={2} radius={1} fill={vars.foreground} />
            <Rect id="attach-vertical" x={25.5} y={23.5} width={2} height={17} radius={1} fill={vars.foreground} />
            <Text id="placeholder" text={vars.placeholder} x={48} y={19} width={300} height={32} fontFamily="Inter" weight={400} size={20} lineHeight={28} color={vars.muted}
              opacity={computed.clock < computed.typeStart || computed.clock >= computed.sendAt ? 1 : 0} />
            <Repeat items={[text.glyphs(vars.prompt, {fontFamily: "Inter", fontWeight: 400, fontSize: 20})[0]]} render={(prompt) => (
              <Group id="draft" x={48} y={19} width={0} height={0} pivotX={0} pivotY={0} scaleX={Math.min(1, 365 / prompt.totalAdvance)} scaleY={Math.min(1, 365 / prompt.totalAdvance)}
                opacity={computed.clock >= computed.typeStart && computed.clock < computed.sendAt ? 1 : 0}>
                <Repeat items={text.glyphs(vars.prompt, {fontFamily: "Inter", fontWeight: 400, fontSize: 20})} render={(glyph) => (
                  <Group id={"typed-" + glyph.index} width={0} height={0} pivotX={0} pivotY={0}>
                    <Text id={"letter-" + glyph.index} text={glyph.text} x={glyph.offset} width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 6} height={32}
                      fontFamily={glyph.fontFamily} weight={glyph.fontWeight} size={20} lineHeight={28} color={vars.foreground} opacity={glyph.index < computed.typed ? 1 : 0} />
                    <Rect id={"caret-" + glyph.index} x={glyph.offset + glyph.advance + 1} y={2} width={1.5} height={23} fill={vars.foreground}
                      opacity={glyph.index === computed.typed - 1 ? (computed.clock < computed.typeEnd ? 1 : Math.floor(computed.clock / 320) % 2) : 0} />
                  </Group>
                )} />
              </Group>
            )} />
            <Rect id="send-disc" x={438} y={10} width={44} height={44} radius={22} fill={vars.foreground} />
            <Group id="send-arrow" x={460} y={32} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.clock >= computed.sendAt && computed.clock < computed.streamEnd ? 0 : 1}>
              <Rect id="arrow-stem" x={-1} y={-10} width={2} height={20} radius={1} fill={vars.surface} />
              <Rect id="arrow-left" x={-8} y={-7} width={10} height={2} radius={1} rotation={-45} fill={vars.surface} />
              <Rect id="arrow-right" x={-2} y={-7} width={10} height={2} radius={1} rotation={45} fill={vars.surface} />
            </Group>
            <Rect id="stop-square" x={454} y={26} width={12} height={12} radius={2} fill={vars.surface} opacity={computed.clock >= computed.sendAt && computed.clock < computed.streamEnd ? 1 : 0} />
          </Group>
          <Group id="keyboard" y={960 - 326 * computed.keyboardLift} width={0} height={0} pivotX={0} pivotY={0}>
            <AssetScene id="mobile-keyboard" asset="mobile-keyboard" inputs={{
              pressed: computed.pressed, uppercase: computed.uppercase, symbols: computed.symbols,
              surface: vars.surface, foreground: vars.foreground, muted: vars.muted,
              background: vars.keyboard, secondary: vars.bubble, accent: vars.accent,
            }} />
          </Group>
          <Rect id="home-indicator" x={180} y={936} width={180} height={5} radius={2.5} fill={vars.foreground} />
        </Group>
      </Group>
    </Scene>;
  },
});
