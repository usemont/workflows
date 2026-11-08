// Standalone animated A/B preview. Use scene.tsx for live timeline host frames.
export default defineSpanScene({
  id: "circular-wipe-transition",
  width: 1080,
  height: 1080,
  durationMs: 3000,
  editModeTimeMs: 1400,
  background: "transparent",
  render() {
    return (
      <Scene background="transparent">
        <Group id="preview-clipA" textureSource width={1080} height={1080} sceneInput={{width: 1080, height: 1080}}>
          <Rect id="preview-clipA-review-background" x={0} y={0} width={1080} height={1080} fill="#1763CA" />
          <Rect id="preview-clipA-tile-0-0" x={0.0} y={0.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-0" x={360.0} y={0.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-0" x={720.0} y={0.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-1" x={180.0} y={180.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-1" x={540.0} y={180.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-1" x={900.0} y={180.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-2" x={0.0} y={360.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-2" x={360.0} y={360.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-2" x={720.0} y={360.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-3" x={180.0} y={540.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-3" x={540.0} y={540.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-3" x={900.0} y={540.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-4" x={0.0} y={720.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-4" x={360.0} y={720.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-4" x={720.0} y={720.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-5" x={180.0} y={900.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-5" x={540.0} y={900.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-5" x={900.0} y={900.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-6" x={0.0} y={1080.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-6" x={360.0} y={1080.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-6" x={720.0} y={1080.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-v-0" x={0.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-1" x={180.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-2" x={360.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-3" x={540.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-4" x={720.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-5" x={900.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-6" x={1080.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-0" x={0} y={0.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-1" x={0} y={180.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-2" x={0} y={360.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-3" x={0} y={540.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-4" x={0} y={720.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-5" x={0} y={900.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-6" x={0} y={1080.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-edge" x={5} y={5} width={1070} height={1070} fill="transparent" stroke="#ffffff" strokeWidth={10} />
          <Text id="preview-clipA-label" text="A" x={270.0} y={270.0} width={540.0} height={540.0} size={410.4} lineHeight={540.0} fontFamily="Archivo" weight={900} color="#ffffff" align="center" />
          <Circle id="preview-clipA-moving-dot" x={140.4} y={820.8} width={64.8} height={64.8} fill="#ffffff">
          <Animate prop="x" from={140.4} to={864.0} start="0ms" duration="4600.0ms" ease="linear" />
          </Circle>
        </Group>
        <Group id="preview-clipB" textureSource width={1080} height={1080} sceneInput={{width: 1080, height: 1080}}>
          <Rect id="preview-clipB-review-background" x={0} y={0} width={1080} height={1080} fill="#E87924" />
          <Rect id="preview-clipB-tile-0-0" x={0.0} y={0.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-0" x={360.0} y={0.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-0" x={720.0} y={0.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-1" x={180.0} y={180.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-1" x={540.0} y={180.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-1" x={900.0} y={180.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-2" x={0.0} y={360.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-2" x={360.0} y={360.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-2" x={720.0} y={360.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-3" x={180.0} y={540.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-3" x={540.0} y={540.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-3" x={900.0} y={540.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-4" x={0.0} y={720.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-4" x={360.0} y={720.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-4" x={720.0} y={720.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-5" x={180.0} y={900.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-5" x={540.0} y={900.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-5" x={900.0} y={900.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-6" x={0.0} y={1080.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-6" x={360.0} y={1080.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-6" x={720.0} y={1080.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-v-0" x={0.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-1" x={180.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-2" x={360.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-3" x={540.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-4" x={720.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-5" x={900.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-6" x={1080.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-0" x={0} y={0.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-1" x={0} y={180.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-2" x={0} y={360.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-3" x={0} y={540.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-4" x={0} y={720.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-5" x={0} y={900.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-6" x={0} y={1080.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-edge" x={5} y={5} width={1070} height={1070} fill="transparent" stroke="#ffffff" strokeWidth={10} />
          <Text id="preview-clipB-label" text="B" x={270.0} y={270.0} width={540.0} height={540.0} size={410.4} lineHeight={540.0} fontFamily="Archivo" weight={900} color="#ffffff" align="center" />
          <Circle id="preview-clipB-moving-dot" x={140.4} y={820.8} width={64.8} height={64.8} fill="#ffffff">
          <Animate prop="x" from={140.4} to={864.0} start="0ms" duration="4600.0ms" ease="linear" />
          </Circle>
        </Group>
        <Image id="outgoing" src="texture-source://preview-clipA" x={0} y={0} width={1080} height={1080} fit="cover" layer={1} />
        <Mask.Circle id="incoming-circle-mask" x={540} y={540} width={0} height={0} fill="#ffffff" layer={2}>
          <Animate prop="x" from={540} to={-987.5} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="y" from={540} to={-987.5} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="width" from={0} to={3055} start="0ms" duration="3000ms" ease="inOutCubic" />
          <Animate prop="height" from={0} to={3055} start="0ms" duration="3000ms" ease="inOutCubic" />
        </Mask.Circle>
        <Group id="incoming-group" mask="incoming-circle-mask">
          <Image id="incoming" src="texture-source://preview-clipB" x={0} y={0} width={1080} height={1080} fit="cover" layer={3} />
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
