// Authored conversation, using the observed dark Claude composer geometry.
// Text is shaped once; the clock changes visibility, never Text.text.
export default defineSpanScene({
  id: "claude-exchange",
  width: 960,
  height: 540,
  durationMs: 11500,
  editPreviewMs: 8400,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    prompt: stringVar("How do I make a video with the Mont CLI?"),
    placeholder: stringVar("How can I help you today?"),
    replyIntro: stringVar("Pick a template, edit it, then render an MP4:"),
    replyLine1: stringVar("mont templates create bottom-up-letters ./video"),
    replyLine2: stringVar("mont edit ./video"),
    replyLine3: stringVar("mont -C ./video render --output ./video.mp4"),
    model: stringVar("Opus 5.5 Medium"),
    project: stringVar("Project"),
    mode: stringVar("Manual"),
    background: colorVar("#191919"),
    composer: colorVar("#303030"),
    composerEdge: colorVar("#414141"),
    userBubble: colorVar("#303030"),
    foreground: colorVar("#ECEAE5"),
    muted: colorVar("#AAA8A3"),
    accent: colorVar("#C78168"),
    sendInk: colorVar("#FAF8F4"),
    typingStartMs: numberVar(650),
    typingMs: numberVar(2450),
    sendPauseMs: numberVar(350),
    dockMs: numberVar(400),
    responseDelayMs: numberVar(500),
    streamMs: numberVar(3400),
    holdMs: numberVar(4150),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width / 960, vars.height / 540)),
    typingStart: computed.number(() => Math.max(0, vars.typingStartMs)),
    typingMs: computed.number(() => Math.max(300, vars.typingMs)),
    typeEnd: computed.number(() => computed.typingStart + computed.typingMs),
    sendAt: computed.number(() => computed.typeEnd + Math.max(100, vars.sendPauseMs)),
    dockMs: computed.number(() => Math.max(150, vars.dockMs)),
    replyAt: computed.number(() => computed.sendAt + Math.max(computed.dockMs, vars.responseDelayMs)),
    streamMs: computed.number(() => Math.max(800, vars.streamMs)),
    replyEnd: computed.number(() => computed.replyAt + computed.streamMs),
    cycleMs: computed.number(() => computed.replyEnd + Math.max(500, vars.holdMs)),
    clockMs: computed.number(() => time.loop(computed.cycleMs) * computed.cycleMs),
    typePhase: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.typingStart) / computed.typingMs))),
    // Two short thinking pauses, with a quicker finish to the sentence.
    typed: computed.number(() => computed.typePhase < 0.32 ? computed.typePhase / 0.32 * 0.38
      : computed.typePhase < 0.39 ? 0.38
      : computed.typePhase < 0.74 ? 0.38 + (computed.typePhase - 0.39) / 0.35 * 0.37
      : computed.typePhase < 0.79 ? 0.75
      : 0.75 + (computed.typePhase - 0.79) / 0.21 * 0.25),
    dockPhase: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.sendAt) / computed.dockMs))),
    dock: computed.number(() => 1 - Math.pow(1 - computed.dockPhase, 3)),
    bubbleIn: computed.number(() => Math.max(0, Math.min(1, (computed.clockMs - computed.sendAt - 80) / 240))),
    caret: computed.number(() => Math.floor(computed.clockMs / 420) % 2 === 0 ? 1 : 0),
    responding: computed.number(() => computed.clockMs >= computed.sendAt && computed.clockMs < computed.replyEnd ? 1 : 0),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="canvas" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="viewport" x={(vars.width - 960 * computed.scale) / 2} y={(vars.height - 540 * computed.scale) / 2}
        width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Repeat items={[text.glyphs(vars.prompt, {fontFamily: "Inter", fontWeight: 400, fontSize: 15})[0]]} render={(prompt) => (
          <Group id="user-message" width={0} height={0} pivotX={0} pivotY={0}
            y={8 * (1 - computed.bubbleIn)} opacity={computed.bubbleIn}>
            <Rect id="user-bubble" x={808 - Math.min(624, prompt.totalAdvance + 36)} y={66}
              width={Math.min(624, prompt.totalAdvance + 36)} height={50} radius={16} fill={vars.userBubble} />
            <Group id="bubble-copy" x={826 - Math.min(624, prompt.totalAdvance + 36)} y={79}
              width={0} height={0} pivotX={0} pivotY={0}
              scaleX={Math.min(1, 588 / Math.max(1, prompt.totalAdvance))}>
              <Text id="user-prompt" text={vars.prompt} width={prompt.totalAdvance + 12} height={26}
                size={15} lineHeight={24} fontFamily="Inter" weight={400} color={vars.foreground} />
            </Group>
          </Group>
        )} />

        <Group id="assistant" width={0} height={0} pivotX={0} pivotY={0}
          opacity={computed.clockMs >= computed.replyAt ? 1 : 0}>
          <Group id="assistant-mark" x={164} y={174} width={0} height={0} pivotX={0} pivotY={0}>
            <Repeat items={[0, 30, 60, 90, 120, 150]} render={(angle, index) => (
              <Rect id={"asterisk-" + index} x={-9} y={-1} width={18} height={2}
                radius={1} rotation={angle} fill={vars.accent} />
            )} />
          </Group>
          <Repeat items={[
            { copy: vars.replyIntro, row: 0, start: 0, span: 0.16 },
            { copy: vars.replyLine1, row: 1, start: 0.18, span: 0.26 },
            { copy: vars.replyLine2, row: 2, start: 0.46, span: 0.25 },
            { copy: vars.replyLine3, row: 3, start: 0.73, span: 0.27 },
          ]} render={(line) => (
            <Group id={"reply-line-" + line.row} x={190} y={159 + line.row * 35}
              width={0} height={0} pivotX={0} pivotY={0}>
              <Text id={"reply-" + line.row} text={line.copy} x={0} y={0} width={600} height={33}
                size={line.row === 0 ? 20 : 17} lineHeight={30}
                fontFamily={line.row === 0 ? "Inter" : "JetBrains Mono"} weight={400} color={vars.foreground} />
              <Rect id={"reply-reveal-" + line.row} y={0} width={600} height={33}
                pivotX={0} pivotY={0} fill={vars.background}
                x={600 * Math.max(0, Math.min(1, (computed.clockMs - computed.replyAt - line.start * computed.streamMs) / (line.span * computed.streamMs)))}
                scaleX={1 - Math.max(0, Math.min(1, (computed.clockMs - computed.replyAt - line.start * computed.streamMs) / (line.span * computed.streamMs)))} />
            </Group>
          )} />
        </Group>

        <Group id="composer" x={152} y={220 + 158 * computed.dock}
          width={0} height={0} pivotX={0} pivotY={0}>
          <Rect id="card-shadow" x={0} y={7} width={656} height={103} radius={14} fill="#000000" opacity={0.24} blur={12} />
          <Rect id="card-edge" width={656} height={103} radius={14} fill={vars.composerEdge} />
          <Rect id="card-fill" x={1} y={1} width={654} height={101} radius={13} fill={vars.composer} />
          <Text id="placeholder" text={vars.placeholder} x={16} y={12} width={610} height={27}
            size={15} lineHeight={25} fontFamily="Inter" weight={400} color={vars.muted}
            opacity={computed.clockMs < computed.typingStart || computed.clockMs >= computed.sendAt ? 1 : 0} />
          <Repeat items={[text.glyphs(vars.prompt, {fontFamily: "Inter", fontWeight: 400, fontSize: 15})[0]]} render={(prompt) => (
            <Group id="draft" x={16} y={12} width={0} height={0} pivotX={0} pivotY={0}
              scaleX={Math.min(1, 608 / Math.max(1, prompt.totalAdvance + 5))}
              opacity={computed.clockMs >= computed.typingStart && computed.clockMs < computed.sendAt ? 1 : 0}>
              <Repeat items={text.glyphs(vars.prompt, {fontFamily: "Inter", fontWeight: 400, fontSize: 15})} render={(glyph) => (
                <Group id={"draft-cluster-" + glyph.index} width={0} height={0} pivotX={0} pivotY={0}>
                  <Text id={"draft-glyph-" + glyph.index} text={glyph.text} x={glyph.offset} y={0}
                    width={Math.max(glyph.advance, glyph.inkWidth + Math.abs(glyph.bearingX)) + 10} height={27}
                    size={15} lineHeight={25} fontFamily={glyph.fontFamily} weight={glyph.fontWeight} color={vars.foreground}
                    opacity={glyph.index < Math.floor(computed.typed * prompt.count) ? 1 : 0} />
                  <Rect id={"draft-caret-" + glyph.index} x={glyph.offset + glyph.advance + 1} y={5}
                    width={1.5} height={17} fill={vars.foreground}
                    opacity={glyph.index === Math.floor(computed.typed * prompt.count) - 1 ? (computed.clockMs < computed.typeEnd ? 1 : computed.caret) : 0} />
                </Group>
              )} />
              <Rect id="empty-caret" x={0} y={5} width={1.5} height={17} fill={vars.foreground}
                opacity={Math.floor(computed.typed * prompt.count) === 0 ? computed.caret : 0} />
            </Group>
          )} />
          <Rect id="plus-horizontal" x={17} y={77} width={14} height={1.5} radius={0.75} fill={vars.muted} />
          <Rect id="plus-vertical" x={23.25} y={70.75} width={1.5} height={14} radius={0.75} fill={vars.muted} />
          <Group id="microphone" x={574} y={76} width={0} height={0} pivotX={0} pivotY={0}>
            <Rect id="mic-cradle" x={-7} y={-2} width={14} height={11} radius={6} fill={vars.muted} />
            <Rect id="mic-cradle-cutout" x={-5.5} y={-3.5} width={11} height={11} radius={5.5} fill={vars.composer} />
            <Rect id="mic-cradle-top-cutout" x={-8} y={-5} width={16} height={6} fill={vars.composer} />
            <Rect id="mic-capsule-front" x={-4} y={-9} width={8} height={13} radius={4} fill={vars.muted} />
            <Rect id="mic-capsule-inner" x={-2.5} y={-7.5} width={5} height={10} radius={2.5} fill={vars.composer} />
            <Rect id="mic-stem" x={-0.75} y={8} width={1.5} height={4} fill={vars.muted} />
            <Rect id="mic-base" x={-4} y={11} width={8} height={1.5} radius={0.75} fill={vars.muted} />
          </Group>
          <Group id="mic-chevron" x={593} y={78} width={0} height={0} pivotX={0} pivotY={0}>
            <Rect id="mic-chevron-left" x={-4} y={-1} width={5} height={1.3} rotation={45} fill={vars.muted} />
            <Rect id="mic-chevron-right" x={-1} y={-1} width={5} height={1.3} rotation={-45} fill={vars.muted} />
          </Group>
          <Group id="send-button" x={612} y={62} width={0} height={0} pivotX={0} pivotY={0}
            opacity={computed.clockMs >= computed.typingStart && computed.clockMs < computed.sendAt ? 1 : 0}
            scaleX={computed.clockMs > computed.sendAt - 110 ? 0.93 : 1} scaleY={computed.clockMs > computed.sendAt - 110 ? 0.93 : 1}>
            <Rect id="send-surface" width={32} height={32} radius={9} fill={vars.accent} />
            <Rect id="send-shaft" x={15.1} y={10} width={1.8} height={15} radius={0.9} fill={vars.sendInk} />
            <Rect id="send-arrow-left" x={9.4} y={10.7} width={8.5} height={1.8} radius={0.9} rotation={-45} fill={vars.sendInk} />
            <Rect id="send-arrow-right" x={14.1} y={10.7} width={8.5} height={1.8} radius={0.9} rotation={45} fill={vars.sendInk} />
          </Group>
          <Group id="stop-button" x={612} y={62} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.responding}>
            <Rect id="stop-surface" width={32} height={32} radius={9} fill={vars.accent} />
            <Rect id="stop-square" x={10.5} y={10.5} width={11} height={11} radius={2} fill={vars.sendInk} />
          </Group>
          <Group id="voice-waveform" x={628} y={78} width={0} height={0} pivotX={0} pivotY={0}
            opacity={computed.clockMs < computed.typingStart || computed.clockMs >= computed.replyEnd ? 1 : 0}>
            <Rect id="voice-bar-0" x={-8} y={-3} width={2} height={6} radius={1} fill={vars.muted} />
            <Rect id="voice-bar-1" x={-4} y={-7} width={2} height={14} radius={1} fill={vars.muted} />
            <Rect id="voice-bar-2" x={0} y={-10} width={2} height={20} radius={1} fill={vars.muted} />
            <Rect id="voice-bar-3" x={4} y={-5} width={2} height={10} radius={1} fill={vars.muted} />
            <Rect id="voice-bar-4" x={8} y={-2} width={2} height={4} radius={1} fill={vars.muted} />
          </Group>

          {/* This 36px tray is below the card, never inside its bottom row. */}
          <Text id="project-label" text={vars.project} x={8} y={111} width={130} height={25}
            size={12} lineHeight={23} fontFamily="Inter" weight={400} color={vars.muted} />
          <Repeat items={[text.glyphs(vars.project, {fontFamily: "Inter", fontWeight: 400, fontSize: 12})[0]]} render={(label) => (
            <Group id="project-chevron" x={18 + label.totalAdvance} y={123} width={0} height={0} pivotX={0} pivotY={0}>
              <Rect id="project-chevron-left" x={-3} width={4} height={1.2} rotation={45} fill={vars.muted} />
              <Rect id="project-chevron-right" width={4} height={1.2} rotation={-45} fill={vars.muted} />
            </Group>
          )} />
          <Text id="model-label" text={vars.model} x={356} y={111} width={192} height={25}
            size={12} lineHeight={23} align="right" fontFamily="Inter" weight={400} color={vars.muted} />
          <Text id="mode-label" text={vars.mode} x={565} y={111} width={55} height={25}
            size={12} lineHeight={23} align="right" fontFamily="Inter" weight={400} color={vars.muted} />
          <Group id="manual-control" x={633} y={122} width={0} height={0} pivotX={0} pivotY={0}>
            <Rect id="control-upper-track" x={-5} y={-3.5} width={12} height={1.2} fill={vars.muted} />
            <Rect id="control-upper-knob" x={-2} y={-5.5} width={2} height={5} radius={1} fill={vars.muted} />
            <Rect id="control-lower-track" x={-5} y={2.5} width={12} height={1.2} fill={vars.muted} />
            <Rect id="control-lower-knob" x={2} y={0.5} width={2} height={5} radius={1} fill={vars.muted} />
          </Group>
        </Group>
      </Group>
    </Scene>;
  },
});
