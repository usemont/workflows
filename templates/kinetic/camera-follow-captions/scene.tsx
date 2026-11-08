// Packed, baseline-aligned word world; one shared camera only pulls outward.
export default defineSpanScene({
  id: "camera-follow-captions",
  width: 960,
  height: 540,
  durationMs: 5000,
  editPreviewMs: 3600,
  variables: {
    width: numberVar(960),
    height: numberVar(540),
    background: colorVar("#101923"),
    word1: stringVar("MAKE"),
    ink1: colorVar("#FFD65C"),
    word2: stringVar("EVERY"),
    ink2: colorVar("#FFD65C"),
    word3: stringVar("FRAME"),
    ink3: colorVar("#FFD65C"),
    word4: stringVar("FEEL"),
    ink4: colorVar("#FFD65C"),
    word5: stringVar("MORE"),
    ink5: colorVar("#FFD65C"),
    word6: stringVar("ALIVE."),
    ink6: colorVar("#FFD65C"),
  },
  computeds: {
    ink1: computed.string(() => vars.ink1),
    ink2: computed.string(() => vars.ink2),
    ink3: computed.string(() => vars.ink3),
    ink4: computed.string(() => vars.ink4),
    ink5: computed.string(() => vars.ink5),
    ink6: computed.string(() => vars.ink6),
    phi: computed.number(() => 1.61803398875),
    aspect: computed.number(() => vars.width / vars.height),
    f0: computed.number(() => 100),
    w0: computed.number(() => text.glyphs(vars.word1, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0].totalAdvance * computed.f0 / 100),
    h0: computed.number(() => computed.f0 * 0.78),
    x0: computed.number(() => 0),
    y0: computed.number(() => 0),
    bw0: computed.number(() => computed.w0),
    bh0: computed.number(() => computed.h0),
    top0: computed.number(() => 0),
    s0: computed.number(() => Math.min(vars.width * 0.84 / computed.bw0, vars.height * 0.78 / (computed.bh0 - computed.top0))),
    tx0: computed.number(() => (vars.width - computed.bw0 * computed.s0) / 2),
    ty0: computed.number(() => (vars.height - (computed.bh0 + computed.top0) * computed.s0) / 2),
    f1: computed.number(() => computed.f0 * computed.phi),
    w1: computed.number(() => text.glyphs(vars.word2, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0].totalAdvance * computed.f1 / 100),
    h1: computed.number(() => computed.f1 * 0.78),
    right1: computed.number(() => computed.bw0 / computed.bh0 < computed.aspect ? 1 : 0),
    x1: computed.number(() => computed.right1 ? computed.bw0 + computed.f1 * 0.07 : 0),
    y1: computed.number(() => computed.right1 ? computed.bh0 - computed.h1 : computed.bh0 + computed.f1 * 0.07),
    bw1: computed.number(() => Math.max(computed.bw0, computed.x1 + computed.w1)),
    bh1: computed.number(() => Math.max(computed.bh0, computed.y1 + computed.h1)),
    top1: computed.number(() => Math.min(computed.top0, computed.y1)),
    s1: computed.number(() => Math.min(computed.s0 * 0.96, vars.width * 0.84 / computed.bw1, vars.height * 0.78 / (computed.bh1 - computed.top1))),
    tx1: computed.number(() => (vars.width - computed.bw1 * computed.s1) / 2),
    ty1: computed.number(() => (vars.height - (computed.bh1 + computed.top1) * computed.s1) / 2),
    f2: computed.number(() => computed.f1 * computed.phi),
    w2: computed.number(() => text.glyphs(vars.word3, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0].totalAdvance * computed.f2 / 100),
    h2: computed.number(() => computed.f2 * 0.78),
    right2: computed.number(() => computed.bw1 / computed.bh1 < computed.aspect ? 1 : 0),
    x2: computed.number(() => computed.right2 ? computed.bw1 + computed.f2 * 0.07 : 0),
    y2: computed.number(() => computed.right2 ? computed.bh1 - computed.h2 : computed.bh1 + computed.f2 * 0.07),
    bw2: computed.number(() => Math.max(computed.bw1, computed.x2 + computed.w2)),
    bh2: computed.number(() => Math.max(computed.bh1, computed.y2 + computed.h2)),
    top2: computed.number(() => Math.min(computed.top1, computed.y2)),
    s2: computed.number(() => Math.min(computed.s1 * 0.96, vars.width * 0.84 / computed.bw2, vars.height * 0.78 / (computed.bh2 - computed.top2))),
    tx2: computed.number(() => (vars.width - computed.bw2 * computed.s2) / 2),
    ty2: computed.number(() => (vars.height - (computed.bh2 + computed.top2) * computed.s2) / 2),
    f3: computed.number(() => computed.f2 * computed.phi),
    w3: computed.number(() => text.glyphs(vars.word4, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0].totalAdvance * computed.f3 / 100),
    h3: computed.number(() => computed.f3 * 0.78),
    right3: computed.number(() => computed.bw2 / computed.bh2 < computed.aspect ? 1 : 0),
    x3: computed.number(() => computed.right3 ? computed.bw2 + computed.f3 * 0.07 : 0),
    y3: computed.number(() => computed.right3 ? computed.bh2 - computed.h3 : computed.bh2 + computed.f3 * 0.07),
    bw3: computed.number(() => Math.max(computed.bw2, computed.x3 + computed.w3)),
    bh3: computed.number(() => Math.max(computed.bh2, computed.y3 + computed.h3)),
    top3: computed.number(() => Math.min(computed.top2, computed.y3)),
    s3: computed.number(() => Math.min(computed.s2 * 0.96, vars.width * 0.84 / computed.bw3, vars.height * 0.78 / (computed.bh3 - computed.top3))),
    tx3: computed.number(() => (vars.width - computed.bw3 * computed.s3) / 2),
    ty3: computed.number(() => (vars.height - (computed.bh3 + computed.top3) * computed.s3) / 2),
    f4: computed.number(() => computed.f3 * computed.phi),
    w4: computed.number(() => text.glyphs(vars.word5, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0].totalAdvance * computed.f4 / 100),
    h4: computed.number(() => computed.f4 * 0.78),
    right4: computed.number(() => computed.bw3 / computed.bh3 < computed.aspect ? 1 : 0),
    x4: computed.number(() => computed.right4 ? computed.bw3 + computed.f4 * 0.07 : 0),
    y4: computed.number(() => computed.right4 ? computed.bh3 - computed.h4 : computed.bh3 + computed.f4 * 0.07),
    bw4: computed.number(() => Math.max(computed.bw3, computed.x4 + computed.w4)),
    bh4: computed.number(() => Math.max(computed.bh3, computed.y4 + computed.h4)),
    top4: computed.number(() => Math.min(computed.top3, computed.y4)),
    s4: computed.number(() => Math.min(computed.s3 * 0.96, vars.width * 0.84 / computed.bw4, vars.height * 0.78 / (computed.bh4 - computed.top4))),
    tx4: computed.number(() => (vars.width - computed.bw4 * computed.s4) / 2),
    ty4: computed.number(() => (vars.height - (computed.bh4 + computed.top4) * computed.s4) / 2),
    f5: computed.number(() => computed.f4 * computed.phi),
    w5: computed.number(() => text.glyphs(vars.word6, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0].totalAdvance * computed.f5 / 100),
    h5: computed.number(() => computed.f5 * 0.78),
    right5: computed.number(() => computed.bw4 / computed.bh4 < computed.aspect ? 1 : 0),
    x5: computed.number(() => computed.right5 ? computed.bw4 + computed.f5 * 0.07 : 0),
    y5: computed.number(() => computed.right5 ? computed.bh4 - computed.h5 : computed.bh4 + computed.f5 * 0.07),
    bw5: computed.number(() => Math.max(computed.bw4, computed.x5 + computed.w5)),
    bh5: computed.number(() => Math.max(computed.bh4, computed.y5 + computed.h5)),
    top5: computed.number(() => Math.min(computed.top4, computed.y5)),
    s5: computed.number(() => Math.min(computed.s4 * 0.96, vars.width * 0.84 / computed.bw5, vars.height * 0.78 / (computed.bh5 - computed.top5))),
    tx5: computed.number(() => (vars.width - computed.bw5 * computed.s5) / 2),
    ty5: computed.number(() => (vars.height - (computed.bh5 + computed.top5) * computed.s5) / 2),
    clock: computed.number(() => time.loop(5000) * 5000),
    p1: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 520) / 400))),
    e1: computed.number(() => 1 - (1 - computed.p1) * (1 - computed.p1) * (1 - computed.p1)),
    reveal1: computed.number(() => 1 - Math.pow(1 - Math.max(0, Math.min(1, (computed.clock - 520) / 160)), 3)),
    p2: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 1040) / 400))),
    e2: computed.number(() => 1 - (1 - computed.p2) * (1 - computed.p2) * (1 - computed.p2)),
    reveal2: computed.number(() => 1 - Math.pow(1 - Math.max(0, Math.min(1, (computed.clock - 1040) / 160)), 3)),
    p3: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 1560) / 400))),
    e3: computed.number(() => 1 - (1 - computed.p3) * (1 - computed.p3) * (1 - computed.p3)),
    reveal3: computed.number(() => 1 - Math.pow(1 - Math.max(0, Math.min(1, (computed.clock - 1560) / 160)), 3)),
    p4: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2080) / 400))),
    e4: computed.number(() => 1 - (1 - computed.p4) * (1 - computed.p4) * (1 - computed.p4)),
    reveal4: computed.number(() => 1 - Math.pow(1 - Math.max(0, Math.min(1, (computed.clock - 2080) / 160)), 3)),
    p5: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2600) / 400))),
    e5: computed.number(() => 1 - (1 - computed.p5) * (1 - computed.p5) * (1 - computed.p5)),
    reveal5: computed.number(() => 1 - Math.pow(1 - Math.max(0, Math.min(1, (computed.clock - 2600) / 160)), 3)),
    cameraX: computed.number(() => computed.tx0 + (computed.tx1 - computed.tx0) * computed.e1 + (computed.tx2 - computed.tx1) * computed.e2 + (computed.tx3 - computed.tx2) * computed.e3 + (computed.tx4 - computed.tx3) * computed.e4 + (computed.tx5 - computed.tx4) * computed.e5),
    cameraY: computed.number(() => computed.ty0 + (computed.ty1 - computed.ty0) * computed.e1 + (computed.ty2 - computed.ty1) * computed.e2 + (computed.ty3 - computed.ty2) * computed.e3 + (computed.ty4 - computed.ty3) * computed.e4 + (computed.ty5 - computed.ty4) * computed.e5),
    cameraS: computed.number(() => computed.s0 + (computed.s1 - computed.s0) * computed.e1 + (computed.s2 - computed.s1) * computed.e2 + (computed.s3 - computed.s2) * computed.e3 + (computed.s4 - computed.s3) * computed.e4 + (computed.s5 - computed.s4) * computed.e5),
    visibility: computed.number(() => 1 - Math.max(0, Math.min(1, (computed.clock - 4350) / 300))),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="paper" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="shared-camera" width={0} height={0} pivotX={0} pivotY={0}
        x={computed.cameraX} y={computed.cameraY}
        scaleX={computed.cameraS} scaleY={computed.cameraS} opacity={computed.visibility}>
        <Group id="fixed-word-0" width={0} height={0} pivotX={0} pivotY={0}
          x={computed.x0} y={computed.y0} scaleX={computed.f0 / 100} scaleY={computed.f0 / 100}
          opacity={1}>
          <Repeat items={[text.glyphs(vars.word1, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0]]} render={(measure) => (
            <Text id="world-word-0" text={vars.word1} x={0} y={-15} width={measure.totalAdvance + 3}
              height={130} size={100} lineHeight={100} fontFamily="Space Grotesk" weight={700} color={computed.ink1} />
          )} />
        </Group>
        <Group id="fixed-word-1" width={0} height={0} pivotX={0} pivotY={0}
          x={computed.x1} y={computed.y1} scaleX={computed.f1 / 100} scaleY={computed.f1 / 100}
          opacity={computed.reveal1}>
          <Repeat items={[text.glyphs(vars.word2, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0]]} render={(measure) => (
            <Text id="world-word-1" text={vars.word2} x={0} y={-15} width={measure.totalAdvance + 3}
              height={130} size={100} lineHeight={100} fontFamily="Space Grotesk" weight={700} color={computed.ink2} />
          )} />
        </Group>
        <Group id="fixed-word-2" width={0} height={0} pivotX={0} pivotY={0}
          x={computed.x2} y={computed.y2} scaleX={computed.f2 / 100} scaleY={computed.f2 / 100}
          opacity={computed.reveal2}>
          <Repeat items={[text.glyphs(vars.word3, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0]]} render={(measure) => (
            <Text id="world-word-2" text={vars.word3} x={0} y={-15} width={measure.totalAdvance + 3}
              height={130} size={100} lineHeight={100} fontFamily="Space Grotesk" weight={700} color={computed.ink3} />
          )} />
        </Group>
        <Group id="fixed-word-3" width={0} height={0} pivotX={0} pivotY={0}
          x={computed.x3} y={computed.y3} scaleX={computed.f3 / 100} scaleY={computed.f3 / 100}
          opacity={computed.reveal3}>
          <Repeat items={[text.glyphs(vars.word4, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0]]} render={(measure) => (
            <Text id="world-word-3" text={vars.word4} x={0} y={-15} width={measure.totalAdvance + 3}
              height={130} size={100} lineHeight={100} fontFamily="Space Grotesk" weight={700} color={computed.ink4} />
          )} />
        </Group>
        <Group id="fixed-word-4" width={0} height={0} pivotX={0} pivotY={0}
          x={computed.x4} y={computed.y4} scaleX={computed.f4 / 100} scaleY={computed.f4 / 100}
          opacity={computed.reveal4}>
          <Repeat items={[text.glyphs(vars.word5, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0]]} render={(measure) => (
            <Text id="world-word-4" text={vars.word5} x={0} y={-15} width={measure.totalAdvance + 3}
              height={130} size={100} lineHeight={100} fontFamily="Space Grotesk" weight={700} color={computed.ink5} />
          )} />
        </Group>
        <Group id="fixed-word-5" width={0} height={0} pivotX={0} pivotY={0}
          x={computed.x5} y={computed.y5} scaleX={computed.f5 / 100} scaleY={computed.f5 / 100}
          opacity={computed.reveal5}>
          <Repeat items={[text.glyphs(vars.word6, {fontFamily: "Space Grotesk", fontWeight: 700, fontSize: 100})[0]]} render={(measure) => (
            <Text id="world-word-5" text={vars.word6} x={0} y={-15} width={measure.totalAdvance + 3}
              height={130} size={100} lineHeight={100} fontFamily="Space Grotesk" weight={700} color={computed.ink6} />
          )} />
        </Group>
      </Group>
    </Scene>;
  },
});
