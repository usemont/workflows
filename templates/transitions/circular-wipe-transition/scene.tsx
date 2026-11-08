export default defineSpanScene({
  id: "circular-wipe-transition",
  width: 1080,
  height: 1080,
  durationMs: 3000,
  editModeTimeMs: 1400,
  background: "transparent",
  hostFrameInputs: [
    { id: "clipA", kind: "timeline", role: "outgoing", sourceType: "video", fit: "cover" },
    { id: "clipB", kind: "timeline", role: "incoming", sourceType: "video", fit: "cover" }
  ],
  render() {
    return (
      <Scene background="transparent">
        <Image id="outgoing" src="host-frame:clipA" x={0} y={0} width={1080} height={1080} fit="cover" layer={1} />
        <Mask.Circle id="incoming-circle-mask" x={540} y={540} width={0} height={0} fill="#ffffff" layer={2}>
          <Animate prop="x" from={540} to={-987.5} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="y" from={540} to={-987.5} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="width" from={0} to={3055} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="height" from={0} to={3055} start="0ms" duration="3000ms" ease="inOutCubic" />
        </Mask.Circle>
        <Group id="incoming-group" mask="incoming-circle-mask">
          <Image id="incoming" src="host-frame:clipB" x={0} y={0} width={1080} height={1080} fit="cover" layer={3} />
        </Group>
        <Circle id="wipe-rim" x={540} y={540} width={0} height={0} fill="transparent" stroke="#111827" strokeWidth={18} opacity={0.78} layer={8}>
          <Animate prop="x" from={540} to={-987.5} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="y" from={540} to={-987.5} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="width" from={0} to={3055} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="height" from={0} to={3055} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="opacity" from={0.86} to={0.12} start="0ms" duration="3000ms" ease="linear" />
        </Circle>
      </Scene>
    );
  }
});