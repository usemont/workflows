// Native replay of recorded data only: no command is executed.
// Keep this literal in sync with tui.cast when editing the bundled example.
export default defineSpanScene({
  id: "terminal-showcase-tui",
  width: 960,
  height: 540,
  durationMs: 10000,
  editPreviewMs: 3000,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    windowTitle: stringVar("atlas / preview board"),
    background: colorVar("#080e18"),
    windowBackground: colorVar("#10151f"),
    chromeBackground: colorVar("#182130"),
    foreground: colorVar("#e2e8f0"),
    recording: stringVar("{\"version\":3,\"term\":{\"cols\":68,\"rows\":14,\"type\":\"xterm-256color\"},\"title\":\"Atlas preview board — fictional recorded output\"}\n[0.0,\"o\",\"\\u001b[38;2;125;211;252mdemo@atlas\\u001b[0m $ atlas board --demo\\r\\n\"]\n[0.6,\"o\",\"\\u001b[?1049h\\u001b[?25l\\u001b[2J\\u001b[H\\u001b[1;3H\\u001b[38;2;125;211;252mATLAS / PREVIEW BOARD\\u001b[0m\\u001b[2;3H\\u001b[38;2;148;163;184mdemo workspace · recorded data only\\u001b[0m\\u001b[4;3H\\u001b[38;2;148;163;184m┌──────────────────────────────────────────────────────────────┐\\u001b[0m\\u001b[5;3H│ SCENE                      LENGTH       STATUS               │\\u001b[6;3H\\u001b[48;2;36;60;86m\\u001b[38;2;125;211;252m│ > 01  Opening              00:04        Ready                │\\u001b[0m\\u001b[7;3H│   02  Product              00:05        Ready                │\\u001b[8;3H│   03  Sign-off             00:03        Ready                │\\u001b[9;3H\\u001b[38;2;148;163;184m└──────────────────────────────────────────────────────────────┘\\u001b[0m\\u001b[11;3H↑ ↓  Select scene       Enter  Preview       q  Close\\u001b[13;3H\\u001b[38;2;148;163;184mSelected: Opening / 3 scenes / 00:12 total\\u001b[0m\"]\n[1.8,\"o\",\"\\u001b[6;3H│   01  Opening              00:04        Ready                │\\u001b[7;3H\\u001b[48;2;36;60;86m\\u001b[38;2;125;211;252m│ > 02  Product              00:05        Ready                │\\u001b[0m\\u001b[13;3H\\u001b[2K\\u001b[38;2;148;163;184mSelected: Product / 3 scenes / 00:12 total\\u001b[0m\"]\n[1.5,\"o\",\"\\u001b[7;3H│   02  Product              00:05        Ready                │\\u001b[8;3H\\u001b[48;2;36;60;86m\\u001b[38;2;125;211;252m│ > 03  Sign-off             00:03        Ready                │\\u001b[0m\\u001b[13;3H\\u001b[2K\\u001b[38;2;148;163;184mSelected: Sign-off / 3 scenes / 00:12 total\\u001b[0m\"]\n[1.4,\"r\",\"72x16\"]\n[0.0,\"o\",\"\\u001b[15;3H\\u001b[38;2;125;211;252mViewport resized: 72 × 16 · same recorded grid\\u001b[0m\"]\n[1.1,\"o\",\"\\u001b[13;3H\\u001b[2K\\u001b[38;2;134;239;172mPreview ready · fictional sample, nothing executed\\u001b[0m\"]\n[1.4,\"o\",\"\\u001b[?1049l\\u001b[?25h\\u001b[38;2;134;239;172mBoard closed. Main screen restored.\\u001b[0m\\r\\n\\u001b[38;2;125;211;252mdemo@atlas\\u001b[0m $ \"]\n"),
  },
  render({ vars }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" x={0} y={0} width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="terminal-window" x={vars.width * 0.05} y={vars.height * 0.1625}
        width={vars.width * 0.9} height={vars.height * 0.675}>
        <Rect id="window-border" x={0} y={0} width={vars.width * 0.9} height={vars.height * 0.675} radius={14} fill={vars.chromeBackground}
          shadowBlur={24} shadowOffsetY={12} shadowColor="#000000" shadowOpacity={0.45} />
        <Rect id="window-surface" x={1} y={45} width={vars.width * 0.9 - 2} height={vars.height * 0.675 - 46} radius={12} fill={vars.windowBackground} />
        <Rect id="window-surface-top" x={1} y={45} width={vars.width * 0.9 - 2} height={12} fill={vars.windowBackground} />
        <Rect id="close" x={20} y={19} width={9} height={9} radius={4.5} fill="#f48787" />
        <Rect id="minimize" x={37} y={19} width={9} height={9} radius={4.5} fill="#eacb83" />
        <Rect id="maximize" x={54} y={19} width={9} height={9} radius={4.5} fill="#98d6ad" />
        <Text id="window-title" text={vars.windowTitle} x={82} y={16}
          width={vars.width * 0.9 - 110} height={24} size={13} fontFamily="JetBrains Mono" fontWeight={500} color={vars.foreground} />
        <Terminal id="terminal" recording={vars.recording} x={22} y={59}
          width={vars.width * 0.9 - 44} height={vars.height * 0.675 - 76}
          fontFamily="JetBrains Mono" fontSize={18} foreground={vars.foreground} background={vars.windowBackground} />
      </Group>
    </Scene>;
  },
});
