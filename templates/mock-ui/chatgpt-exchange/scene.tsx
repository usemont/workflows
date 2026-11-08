// Original dialogue, authored with native primitives. All revealed text is statically shaped.
export default defineSpanScene({
  id: "chatgpt-exchange",
  width: 960,
  height: 540,
  durationMs: 11500,
  editPreviewMs: 9000,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    prompt: stringVar("How do I make a video with Mont MCP?"),
    replyLine1: stringVar("Connect Mont MCP, then describe your video:"),
    replyLine2: stringVar("Make a vertical launch teaser in ten seconds."),
    replyLine3: stringVar("Review the scenes, then ask for an MP4 export."),
    model: stringVar("Instant"),
    greeting: stringVar("What's on your mind today?"),
    placeholder: stringVar("Ask ChatGPT"),
    background: colorVar("#0E0E0E"),
    surface: colorVar("#212121"),
    bubble: colorVar("#2F2F2F"),
    foreground: colorVar("#EDEDED"),
    muted: colorVar("#9B9B9B"),
    edge: colorVar("#343434"),
    button: colorVar("#FFFFFF"),
    buttonInk: colorVar("#171717"),
    emptyHoldMs: numberVar(800),
    typingMs: numberVar(2900),
    promptHoldMs: numberVar(350),
    dockMs: numberVar(380),
    responseDelayMs: numberVar(400),
    streamMs: numberVar(3300),
    finalHoldMs: numberVar(3370),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width / 960, vars.height / 540)),
    typeStart: computed.number(() => Math.max(200, vars.emptyHoldMs)),
    typingMs: computed.number(() => Math.max(600, vars.typingMs)),
    typeEnd: computed.number(() => computed.typeStart + computed.typingMs),
    sendAt: computed.number(() => computed.typeEnd + Math.max(100, vars.promptHoldMs)),
    dockMs: computed.number(() => Math.max(180, vars.dockMs)),
    dockEnd: computed.number(() => computed.sendAt + computed.dockMs),
    streamStart: computed.number(() => computed.dockEnd + Math.max(150, vars.responseDelayMs)),
    streamMs: computed.number(() => Math.max(900, vars.streamMs)),
    streamEnd: computed.number(() => computed.streamStart + computed.streamMs),
    cycleMs: computed.number(() => computed.streamEnd + Math.max(800, vars.finalHoldMs)),
    clockMs: computed.number(() => time.loop(computed.cycleMs) * computed.cycleMs),
    typeProgress: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.typeStart) / computed.typingMs))),
    // Two short thought pauses, with a quicker finish; no random or runtime string changes.
    typed: computed.number(() => computed.typeProgress >= 1 ? 1 : computed.typeProgress < 0.35 ? computed.typeProgress * 1.2
      : computed.typeProgress < 0.43 ? 0.42
      : computed.typeProgress < 0.78 ? 0.42 + (computed.typeProgress - 0.43) * 0.4 / 0.35
      : computed.typeProgress < 0.84 ? 0.82
      : Math.min(1, 0.82 + (computed.typeProgress - 0.84) * 0.18 / 0.16)),
    dockProgress: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.sendAt) / computed.dockMs))),
    dock: computed.number(() => 1 - Math.pow(1 - computed.dockProgress, 3)),
    bubbleIn: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.sendAt - 100) / 200))),
    blink: computed.number(() => Math.floor(Math.max(0, computed.clockMs - computed.typeEnd) / 320) % 2 === 0 ? 1 : 0),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="app" x={(vars.width - 960 * computed.scale) / 2} y={(vars.height - 540 * computed.scale) / 2}
        width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Text id="brand" text="ChatGPT" x={28} y={26} width={100} height={30}
          fontFamily="Inter" weight={400} size={19} lineHeight={28} color={vars.foreground} />
        <Group id="mode-tabs" x={418} y={24} width={0} height={0} pivotX={0} pivotY={0}>
          <Rect id="mode-track" width={124} height={36} radius={18} fill={vars.surface} />
          <Rect id="selected-chat" x={3} y={3} width={58} height={30} radius={15} fill={vars.bubble} />
          <Text id="chat-tab" text="Chat" x={13} y={6} width={40} height={24}
            fontFamily="Inter" size={14} lineHeight={24} color={vars.foreground} align="center" />
          <Text id="work-tab" text="Work" x={71} y={6} width={40} height={24}
            fontFamily="Inter" size={14} lineHeight={24} color={vars.muted} align="center" />
        </Group>
        <Text id="greeting" text={vars.greeting} x={120} y={218} width={720} height={44}
          fontFamily="Inter" weight={400} size={28} lineHeight={38} color={vars.foreground} align="center"
          opacity={1 - computed.dock} />
        <Repeat items={[text.glyphs(vars.prompt, {fontFamily: "Inter", fontWeight: 400, fontSize: 16})[0]]} render={(prompt) => (
          <Group id="exchange" width={0} height={0} pivotX={0} pivotY={0}>
            <Group id="user-message" y={10 * (1 - computed.bubbleIn)} width={0} height={0} pivotX={0} pivotY={0}
              opacity={computed.bubbleIn}>
              <Rect id="user-bubble" x={802 - Math.min(660, prompt.totalAdvance)} y={136}
                width={Math.min(660, prompt.totalAdvance) + 38} height={50} radius={25} fill={vars.bubble} />
              <Group id="user-copy" x={821 - Math.min(660, prompt.totalAdvance)} y={148}
                width={0} height={0} pivotX={0} pivotY={0}
                scaleX={Math.min(1, 660 / prompt.totalAdvance)} scaleY={Math.min(1, 660 / prompt.totalAdvance)}>
                <Text id="sent-prompt" text={vars.prompt} width={prompt.totalAdvance + 8} height={30}
                  fontFamily="Inter" weight={400} size={16} lineHeight={26} color={vars.foreground} />
              </Group>
            </Group>
            <Group id="composer" x={120} y={286 + 156 * computed.dock}
              width={0} height={0} pivotX={0} pivotY={0}>
              <Rect id="composer-edge" width={720} height={50} radius={25} fill={vars.edge} />
              <Rect id="composer-surface" x={1} y={1} width={718} height={48} radius={24} fill={vars.surface} />
              <Rect id="plus-horizontal" x={18} y={24} width={16} height={2} radius={1} fill={vars.foreground} />
              <Rect id="plus-vertical" x={25} y={17} width={2} height={16} radius={1} fill={vars.foreground} />
              <Text id="placeholder" text={vars.placeholder} x={52} y={12} width={400} height={30}
                fontFamily="Inter" weight={400} size={16} lineHeight={26} color={vars.muted}
                opacity={computed.clockMs < computed.typeStart || computed.clockMs >= computed.sendAt ? 1 : 0} />
              <Group id="draft" x={52} y={12} width={0} height={0} pivotX={0} pivotY={0}
                scaleX={Math.min(1, 465 / prompt.totalAdvance)} scaleY={Math.min(1, 465 / prompt.totalAdvance)}
                opacity={computed.clockMs >= computed.typeStart && computed.clockMs < computed.sendAt ? 1 : 0}>
                <Repeat items={text.glyphs(vars.prompt, {fontFamily: "Inter", fontWeight: 400, fontSize: 16})} render={(glyph) => (
                  <Group id={"draft-cluster-" + glyph.index} width={0} height={0} pivotX={0} pivotY={0}>
                    <Text id={"draft-glyph-" + glyph.index} text={glyph.text} x={glyph.offset}
                      width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 6} height={30}
                      fontFamily={glyph.fontFamily} weight={glyph.fontWeight} size={16} lineHeight={26} color={vars.foreground}
                      opacity={glyph.index < Math.floor(computed.typed * prompt.count) ? 1 : 0} />
                    <Rect id={"caret-" + glyph.index} x={glyph.offset + glyph.advance + 1} y={4}
                      width={1.5} height={18} fill={vars.foreground}
                      opacity={glyph.index === Math.floor(computed.typed * prompt.count) - 1
                        ? (computed.clockMs < computed.typeEnd ? 1 : computed.blink) : 0} />
                  </Group>
                )} />
                <Rect id="empty-caret" x={0} y={4} width={1.5} height={18} fill={vars.foreground}
                  opacity={Math.floor(computed.typed * prompt.count) === 0 ? 1 : 0} />
              </Group>
              <Text id="model" text={vars.model} x={540} y={13} width={62} height={26}
                fontFamily="Inter" size={14} lineHeight={24} color={vars.foreground} />
              <Rect id="model-chevron-left" x={605} y={23} width={6} height={1.5} radius={0.75}
                rotation={45} fill={vars.muted} />
              <Rect id="model-chevron-right" x={609} y={23} width={6} height={1.5} radius={0.75}
                rotation={-45} fill={vars.muted} />
              <Rect id="mic-capsule" x={641} y={15} width={6} height={11} radius={3}
                fill={vars.surface} stroke={vars.foreground} strokeWidth={1.5} />
              <Rect id="mic-base" x={638} y={27} width={12} height={1.5} radius={0.75} fill={vars.foreground} />
              <Rect id="mic-stem" x={643} y={28} width={2} height={5} radius={1} fill={vars.foreground} />
              <Group id="action" x={690} y={25} width={0} height={0} pivotX={0} pivotY={0}
                scaleX={computed.clockMs >= computed.sendAt && computed.clockMs < computed.sendAt + 120 ? 0.9 : 1}
                scaleY={computed.clockMs >= computed.sendAt && computed.clockMs < computed.sendAt + 120 ? 0.9 : 1}>
                <Rect id="action-disc" x={-17} y={-17} width={34} height={34} radius={17} fill={vars.button} />
                <Group id="voice" width={0} height={0} pivotX={0} pivotY={0}
                  opacity={computed.clockMs < computed.typeStart || computed.clockMs >= computed.streamEnd ? 1 : 0}>
                  <Repeat items={[4, 10, 16, 10, 4]} render={(bar, index) => (
                    <Rect id={"voice-bar-" + index} x={-9 + index * 4} y={-bar / 2} width={2} height={bar}
                      radius={1} fill={vars.buttonInk} />
                  )} />
                </Group>
                <Group id="send-arrow" width={0} height={0} pivotX={0} pivotY={0}
                  opacity={computed.clockMs >= computed.typeStart && computed.clockMs < computed.sendAt ? 1 : 0}>
                  <Rect id="arrow-stem" x={-1} y={-7} width={2} height={15} radius={1} fill={vars.buttonInk} />
                  <Rect id="arrow-left" x={-7} y={-5} width={9} height={2} radius={1} rotation={-45} fill={vars.buttonInk} />
                  <Rect id="arrow-right" x={-2} y={-5} width={9} height={2} radius={1} rotation={45} fill={vars.buttonInk} />
                </Group>
                <Rect id="stop" x={-5} y={-5} width={10} height={10} radius={1.5} fill={vars.buttonInk}
                  opacity={computed.clockMs >= computed.sendAt && computed.clockMs < computed.streamEnd ? 1 : 0} />
              </Group>
            </Group>
          </Group>
        )} />
        <Rect id="response-pending" x={143} y={239} width={9} height={9} radius={4.5} fill={vars.foreground}
          opacity={computed.clockMs >= computed.dockEnd && computed.clockMs < computed.streamStart ? 0.55 + 0.35 * Math.sin((computed.clockMs - computed.dockEnd) / 120) : 0} />
        <Repeat items={[vars.replyLine1, vars.replyLine2, vars.replyLine3]} render={(line, lineIndex) => (
          <Repeat items={[text.glyphs(line, {fontFamily: "Inter", fontWeight: 400, fontSize: 20})[0]]} render={(measure) => (
            <Group id={"reply-line-" + lineIndex} x={142} y={226 + lineIndex * 38}
              width={0} height={0} pivotX={0} pivotY={0}
              scaleX={Math.min(1, 676 / measure.totalAdvance)} scaleY={Math.min(1, 676 / measure.totalAdvance)}>
              <Repeat items={text.glyphs(line, {fontFamily: "Inter", fontWeight: 400, fontSize: 20})} render={(glyph) => (
                <Group id={"reply-cluster-" + lineIndex + "-" + glyph.index} width={0} height={0} pivotX={0} pivotY={0}>
                  <Text id={"reply-glyph-" + lineIndex + "-" + glyph.index} text={glyph.text} x={glyph.offset}
                    width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 8} height={36}
                    fontFamily={glyph.fontFamily} weight={glyph.fontWeight} size={20} lineHeight={30} color={vars.foreground}
                    opacity={computed.clockMs >= computed.streamStart + computed.streamMs * (lineIndex + (glyph.index + 1) / measure.count) / 3 ? 1 : 0} />
                  <Rect id={"stream-dot-" + lineIndex + "-" + glyph.index} x={glyph.offset + glyph.advance + 5} y={12}
                    width={6} height={6} radius={3} fill={vars.foreground}
                    opacity={computed.clockMs >= computed.streamStart && computed.clockMs < computed.streamEnd
                      && glyph.index === Math.floor(((computed.clockMs - computed.streamStart) * 3 / computed.streamMs - lineIndex) * measure.count) - 1 ? 1 : 0} />
                </Group>
              )} />
            </Group>
          )} />
        )} />
      </Group>
    </Scene>;
  },
});
