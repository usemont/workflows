// Native adaptation of https://hyperframes.heygen.com/catalog/components/cursor-drag-drop.
// Copyright 2026 HeyGen, Inc. Apache-2.0; see ../../subscribe/LICENSE.
// Original Mont task copy, Linear-inspired surfaces, minimum-jerk motion and first-party cursor.
export default defineSpanScene({
  id: "drag-and-drop", width: 1920, height: 1080, durationMs: 3600, editPreviewMs: 1450,
  variables: {
    width: numberVar(1920), height: numberVar(1080),
    firstTask: stringVar("Map the opening beat"), cardLabel: stringVar("Shape the title reveal"),
    thirdTask: stringVar("Capture a cursor gesture"), fourthTask: stringVar("Ship the teaser cut"),
    sourceHeading: stringVar("Ideas"), targetHeading: stringVar("In motion"), tone: stringVar("light"),
    background: colorVar("#FAFAFB"), accent: colorVar("#7871D8"),
    cursorSrc: stringVar("data:image/png;base64,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"),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width / 1920, vars.height / 1080)),
    clock: computed.number(() => time.loop(3600) * 3600),
    arrival: computed.number(() => Math.max(0, Math.min(1, computed.clock / 650))),
    arrive: computed.number(() => Math.pow(computed.arrival, 3) * (10 + computed.arrival * (-15 + 6 * computed.arrival))),
    carry: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 900) / 1100))),
    carryEase: computed.number(() => Math.pow(computed.carry, 3) * (10 + computed.carry * (-15 + 6 * computed.carry))),
    // A single curved translation drives both the card's grip and the pointer hotspot.
    carryX: computed.number(() => 606.88 * (1 - computed.carryEase) * computed.carryEase + 640 * computed.carryEase * computed.carryEase),
    carryY: computed.number(() => -291.6 * (1 - computed.carryEase) * computed.carryEase - 138 * computed.carryEase * computed.carryEase),
    lift: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 700) / 200))),
    liftEase: computed.number(() => Math.pow(computed.lift, 3) * (10 + computed.lift * (-15 + 6 * computed.lift))),
    settle: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2000) / 340))),
    settleEase: computed.number(() => Math.pow(computed.settle, 3) * (10 + computed.settle * (-15 + 6 * computed.settle))),
    shadowLift: computed.number(() => computed.clock < 2000 ? computed.liftEase : 1 - computed.settleEase),
    cardScale: computed.number(() => computed.clock < 2000 ? 1 + 0.025 * computed.liftEase : 1.025 - 0.025 * computed.settleEase),
    carrySpeed: computed.number(() => 30 * Math.pow(computed.carry, 2) * Math.pow(1 - computed.carry, 2) / 1.1),
    cardTilt: computed.number(() => Math.min(2, (606.88 + 66.24 * computed.carryEase) * computed.carrySpeed / 1200 * 2) * computed.shadowLift),
    gap: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 1050) / 400))),
    gapEase: computed.number(() => Math.pow(computed.gap, 3) * (10 + computed.gap * (-15 + 6 * computed.gap))),
    proximity: computed.number(() => Math.max(0, 1 - Math.sqrt(Math.pow(640 - computed.carryX, 2) + Math.pow(-138 - computed.carryY, 2)) / 300)),
    slot: computed.number(() => computed.clock < 2000 ? computed.proximity * computed.proximity * (3 - 2 * computed.proximity) : 1 - computed.settleEase),
    departure: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2600) / 1000))),
    depart: computed.number(() => Math.pow(computed.departure, 3) * (10 + computed.departure * (-15 + 6 * computed.departure))),
    // One uninterrupted departure curve: no stop between moving aside and leaving.
    pointerX: computed.number(() => computed.clock < 650 ? 2080 * Math.pow(1 - computed.arrive, 2) + 3076.4 * (1 - computed.arrive) * computed.arrive + 730 * computed.arrive * computed.arrive : computed.clock < 2600 ? 730 + computed.carryX : 1370 * Math.pow(1 - computed.depart, 3) + 4350 * Math.pow(1 - computed.depart, 2) * computed.depart + 5460 * (1 - computed.depart) * computed.depart * computed.depart + 2080 * Math.pow(computed.depart, 3)),
    pointerY: computed.number(() => computed.clock < 650 ? 1240 * Math.pow(1 - computed.arrive, 2) + 1254 * (1 - computed.arrive) * computed.arrive + 500 * computed.arrive * computed.arrive : computed.clock < 2600 ? 500 + computed.carryY : 362 * Math.pow(1 - computed.depart, 3) + 1170 * Math.pow(1 - computed.depart, 2) * computed.depart + 1590 * (1 - computed.depart) * computed.depart * computed.depart + 1240 * Math.pow(computed.depart, 3)),
    press: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 700) / 130))),
    release: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2000) / 130))),
    pointerScale: computed.number(() => computed.clock < 2000 ? 1 - 0.06 * Math.pow(computed.press, 3) * (10 + computed.press * (-15 + 6 * computed.press)) : 0.94 + 0.06 * Math.pow(computed.release, 3) * (10 + computed.release * (-15 + 6 * computed.release))),
  },
  render({vars, computed}) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Image id="cursor-prewarm" src={vars.cursorSrc} x={-4096} y={-4096} width={1} height={1} borderWidth={0} borderColor="transparent" />
      <Group id="letterbox" x={(vars.width - 1920 * computed.scale) / 2} y={(vars.height - 1080 * computed.scale) / 2} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Group id="board" x={-144} y={-81.75} width={0} height={0} pivotX={0} pivotY={0} scaleX={1.15} scaleY={1.15}>
          <Repeat items={[{x: 380, title: vars.sourceHeading}, {x: 1020, title: vars.targetHeading}]} render={(column, index) => (
            <Group id={"column-" + index} x={column.x} y={200} width={0} height={0} pivotX={0} pivotY={0}>
              <Rect id={"column-surface-" + index} width={520} height={690} radius={16} fill={vars.tone === "dark" ? "#1B1C22" : "#F2F3F5"} stroke={vars.tone === "dark" ? "#2E3039" : "#E7E8EC"} strokeWidth={1} />
              <Ellipse id={"column-status-" + index} x={28} y={42} width={12} height={12} fill={index === 0 ? (vars.tone === "dark" ? "#70727D" : "#B0B2BD") : vars.accent} />
              <Text id={"column-heading-" + index} text={column.title} x={52} y={32} width={index === 0 ? 82 : 154} height={32} fontFamily="Inter" fontWeight={600} size={24} lineHeight={1.2} fit="none" color={vars.tone === "dark" ? "#E9E9EE" : "#303139"} />
              <Rect id={"count-badge-" + index} x={index === 0 ? 150 : 222} y={33} width={38} height={28} radius={6} fill={vars.tone === "dark" ? "#2C2D36" : "#E8E9ED"} />
              <Text id={"column-count-" + index} text={index === 0 ? (computed.clock >= 1050 ? "3" : "4") : (computed.clock >= 2000 ? "1" : "0")} x={index === 0 ? 150 : 222} y={36} width={38} height={23} fontFamily="Inter" fontWeight={500} size={18} lineHeight={1.2} fit="none" align="center" color={vars.tone === "dark" ? "#A7A9B4" : "#7C7E89"} />
            </Group>
          )} />
          <Group id="drop-target" x={1040} y={300} width={0} height={0} pivotX={0} pivotY={0} opacity={computed.slot}>
            <Rect id="drop-tint" x={-3} y={-3} width={486} height={126} radius={12} fill={vars.accent} opacity={0.04} />
            <Rect id="drop-outline" x={-3} y={-3} width={486} height={126} radius={12} fill="transparent" stroke={vars.accent} strokeWidth={1.5} />
          </Group>
          <Repeat items={[
            {y: 300, title: vars.firstTask, chip: "Outline", chipWidth: 90, color: "#8C6644", tint: "#FCE8DE", date: "Beat 1", initials: "NW"},
            {y: 576, title: vars.thirdTask, chip: "Screen", chipWidth: 86, color: "#587665", tint: "#E1F0E4", date: "Beat 3", initials: "JT"},
            {y: 714, title: vars.fourthTask, chip: "Launch", chipWidth: 86, color: "#64749A", tint: "#DFECF8", date: "Beat 4", initials: "RS"},
          ]} render={(card, index) => (
            <Group id={"stationary-card-" + index} x={400} y={card.y - (index > 0 ? 138 * computed.gapEase : 0)} width={0} height={0} pivotX={0} pivotY={0}>
              <Rect id={"card-surface-" + index} width={480} height={120} radius={10} fill={vars.tone === "dark" ? "#25262D" : "#FFFFFF"} stroke={vars.tone === "dark" ? "#373943" : "#E1E2E7"} strokeWidth={1} shadowColor="#242530" shadowOffsetY={2} shadowBlur={4} shadowOpacity={0.035} />
              <Rect id={"task-chip-" + index} x={24} y={20} width={card.chipWidth} height={28} radius={8} fill={card.tint} />
              <Text id={"task-category-" + index} text={card.chip} x={24} y={24} width={card.chipWidth} height={20} fontFamily="Inter" fontWeight={600} size={16} lineHeight={1.2} fit="none" align="center" color={card.color} />
              <Text id={"task-title-" + index} text={card.title} x={24} y={58} width={432} height={34} fontFamily="Inter" fontWeight={500} size={24} letterSpacing={-0.24} lineHeight={1.2} fit="none" color={vars.tone === "dark" ? "#E9E9EE" : "#303139"} />
              <Text id={"task-date-" + index} text={card.date} x={350} y={25} width={62} height={20} fontFamily="Inter" fontWeight={400} size={16} lineHeight={1.2} fit="none" align="right" color="#92949F" />
              <Ellipse id={"task-avatar-" + index} x={422} y={18} width={34} height={34} fill={vars.tone === "dark" ? "#3B3D4A" : "#E9EAF0"} />
              <Text id={"task-initials-" + index} text={card.initials} x={422} y={27} width={34} height={16} fontFamily="Inter" fontWeight={600} size={13} lineHeight={1.2} fit="none" align="center" color={vars.tone === "dark" ? "#C1C3CE" : "#697184"} />
            </Group>
          )} />
          {/* Pivot exactly at the grab point: scale and tilt cannot slide the card away from the cursor. */}
          <Group id="dragged-card" x={730 + computed.carryX} y={500 + computed.carryY} width={0} height={0} pivotX={0} pivotY={0} rotation={computed.cardTilt} scaleX={computed.cardScale} scaleY={computed.cardScale}>
            <Rect id="lift-shadow" x={-330} y={-62} width={480} height={120} radius={10} fill={vars.tone === "dark" ? "#25262D" : "#FFFFFF"} shadowColor="#242530" shadowBlur={24} shadowOffsetY={10} shadowOpacity={0.1 * computed.shadowLift} />
            <Rect id="dragged-surface" x={-330} y={-62} width={480} height={120} radius={10} fill={vars.tone === "dark" ? "#25262D" : "#FFFFFF"} stroke={vars.tone === "dark" ? "#373943" : "#E1E2E7"} strokeWidth={1} shadowColor="#242530" shadowBlur={4} shadowOffsetY={2} shadowOpacity={0.035} />
            <Rect id="dragged-chip" x={-306} y={-42} width={84} height={28} radius={8} fill="#F8E1EA" />
            <Text id="dragged-category" text="Type" x={-306} y={-38} width={84} height={20} fontFamily="Inter" fontWeight={600} size={16} lineHeight={1.2} fit="none" align="center" color="#876278" />
            <Text id="dragged-title" text={vars.cardLabel} x={-306} y={-4} width={432} height={34} fontFamily="Inter" fontWeight={500} size={24} letterSpacing={-0.24} lineHeight={1.2} fit="none" color={vars.tone === "dark" ? "#E9E9EE" : "#303139"} />
            <Text id="dragged-date" text="Beat 2" x={20} y={-37} width={62} height={20} fontFamily="Inter" fontWeight={400} size={16} lineHeight={1.2} fit="none" align="right" color="#92949F" />
            <Ellipse id="dragged-avatar" x={92} y={-44} width={34} height={34} fill={vars.tone === "dark" ? "#3B3D4A" : "#E9EAF0"} />
            <Text id="dragged-initials" text="DP" x={92} y={-35} width={34} height={16} fontFamily="Inter" fontWeight={600} size={13} lineHeight={1.2} fit="none" align="center" color={vars.tone === "dark" ? "#C1C3CE" : "#697184"} />
          </Group>
          <Group id="cursor" x={computed.pointerX} y={computed.pointerY} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.pointerScale} scaleY={computed.pointerScale}>
            <Image id="mont-arrow" src={vars.cursorSrc} x={-9.275362319} y={-6.956521739} width={55.652173913} height={55.652173913} borderWidth={0} borderColor="transparent" />
          </Group>
        </Group>
      </Group>
    </Scene>;
  },
});
