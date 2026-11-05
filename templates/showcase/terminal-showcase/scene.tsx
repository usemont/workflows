// Native replay of recorded data only: no command is executed.
// Keep this literal in sync with cli.cast when editing the bundled example.
export default defineSpanScene({
  id: "terminal-showcase",
  width: 960,
  height: 540,
  durationMs: 9000,
  editPreviewMs: 4900,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    windowTitle: stringVar("atlas / release preview"),
    background: colorVar("#080e18"),
    windowBackground: colorVar("#10151f"),
    chromeBackground: colorVar("#182130"),
    foreground: colorVar("#e2e8f0"),
    recording: stringVar("{\"version\":2,\"width\":72,\"height\":12,\"title\":\"Atlas release preview — fictional recorded output\"}\n[0.0,\"o\",\"\\u001b[?25h\\u001b[38;2;125;211;252mdemo@atlas\\u001b[0m:\\u001b[38;2;148;163;184m~/studio\\u001b[0m $ \"]\n[0.45,\"o\",\"a\"]\n[0.535,\"o\",\"t\"]\n[0.62,\"o\",\"l\"]\n[0.705,\"o\",\"a\"]\n[0.79,\"o\",\"s\"]\n[0.875,\"o\",\" \"]\n[0.96,\"o\",\"p\"]\n[1.045,\"o\",\"r\"]\n[1.13,\"o\",\"e\"]\n[1.215,\"o\",\"v\"]\n[1.3,\"o\",\"i\"]\n[1.385,\"o\",\"e\"]\n[1.47,\"o\",\"w\"]\n[1.555,\"o\",\" \"]\n[1.64,\"o\",\"-\"]\n[1.725,\"o\",\"-\"]\n[1.81,\"o\",\"d\"]\n[1.895,\"o\",\"e\"]\n[1.98,\"o\",\"m\"]\n[2.065,\"o\",\"o\"]\n[2.25,\"o\",\"\\r\\n\\r\\n\\u001b[38;2;148;163;184m  A fictional release. No commands are executed.\\u001b[0m\\r\\n\\r\\n\"]\n[2.5,\"o\",\"\\u001b[38;2;125;211;252m  ⠋ Preparing preview\\u001b[0m\"]\n[2.7,\"o\",\"\\r\\u001b[2K\\u001b[38;2;125;211;252m  ⠙ Preparing preview\\u001b[0m\"]\n[2.9,\"o\",\"\\r\\u001b[2K\\u001b[38;2;125;211;252m  ⠹ Preparing preview\\u001b[0m\"]\n[3.1,\"o\",\"\\r\\u001b[2K\\u001b[38;2;125;211;252m  ⠸ Preparing preview\\u001b[0m\"]\n[3.3,\"o\",\"\\r\\u001b[2K\\u001b[38;2;125;211;252m  ⠼ Preparing preview\\u001b[0m\"]\n[3.5,\"o\",\"\\r\\u001b[2K\\u001b[38;2;125;211;252m  ⠴ Preparing preview\\u001b[0m\"]\n[3.7,\"o\",\"\\r\\u001b[2K\\u001b[38;2;125;211;252m  ⠦ Preparing preview\\u001b[0m\"]\n[3.9,\"o\",\"\\r\\u001b[2K\\u001b[38;2;125;211;252m  ⠧ Preparing preview\\u001b[0m\"]\n[4.25,\"o\",\"\\r\\u001b[2K\\u001b[38;2;134;239;172m  ✓ Preview ready\\u001b[0m\\r\\n\"]\n[4.65,\"o\",\"    3 scenes  /  00:12  /  960 × 540\\r\\n\"]\n[5.1,\"o\",\"\\u001b[38;2;148;163;184m    Saved to demo/preview.mp4 (sample output)\\u001b[0m\\r\\n\\r\\n\"]\n[5.6,\"o\",\"\\u001b[38;2;125;211;252mdemo@atlas\\u001b[0m:\\u001b[38;2;148;163;184m~/studio\\u001b[0m $ \"]\n"),
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
