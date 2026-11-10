// Original Mont composition. Every card scales around its own center.
// Stable variation seed: "subscription-pile-up / loose constellation / v2".
// Centers are packed against rotated footprints across a 60 Hz motion sweep,
// including spring overshoot and wobble: at least 44 px between card bodies.
// Per-card phase and period were sampled once; no runtime collision solver.
function age(clock, card) {
  return Math.max(0, (clock - card.atMs) / 1000);
}
function pop(clock, card) {
  return 1 - Math.exp(-8 * age(clock, card)) * Math.cos(13 * age(clock, card));
}
function floatIn(clock, card) {
  return 1 - Math.exp(-3 * age(clock, card));
}
function sway(clock, card, offset) {
  return Math.sin(age(clock, card) * 6283.185 / card.period + card.phase + offset) * floatIn(clock, card);
}

export default defineSpanScene({
  id: "subscription-pile-up",
  width: 960,
  height: 960,
  durationMs: 5500,
  editPreviewMs: 4200,
  variables: {
    width: numberVar(960), height: numberVar(960),
    speed: numberVar(1),
    background: colorVar("#101521"), surface: colorVar("#202A3B"),
    border: colorVar("#39465A"), ink: colorVar("#F3F4F8"),
    muted: colorVar("#A6B1C5"), accent: colorVar("#F5ACD1"),
  },
  tokens: {
    // Transparent diagonal lighting over the editable surface color.
    cardLighting: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 340 98"><defs><linearGradient id="wash" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="white" stop-opacity=".12"/><stop offset=".45" stop-color="white" stop-opacity=".025"/><stop offset="1" stop-color="black" stop-opacity=".24"/></linearGradient><linearGradient id="edge" x1="0" y1="0" x2=".7" y2="1"><stop offset="0" stop-color="white" stop-opacity=".2"/><stop offset="1" stop-color="white" stop-opacity=".015"/></linearGradient></defs><rect x=".5" y=".5" width="339" height="97" rx="16.5" fill="url(%23wash)" stroke="url(%23edge)"/></svg>',
    cards: [
      {name: "Screen recorder", badge: "R", price: 24, atMs: 300, x: 315, y: 187, tilt: -8, scale: 1.02, phase: 3.65, period: 2445, color: "#B9B2EF"},
      {name: "Video editor", badge: "E", price: 39, atMs: 650, x: 711, y: 288, tilt: 8.2, scale: 0.92, phase: 1.83, period: 2733, color: "#8BCBC3"},
      {name: "Auto captions", badge: "C", price: 19, atMs: 1000, x: 231, y: 359, tilt: -4.2, scale: 0.95, phase: 5.41, period: 3187, color: "#F0CAA2"},
      {name: "Motion graphics", badge: "M", price: 49, atMs: 1400, x: 563, y: 485, tilt: -9.3, scale: 1.05, phase: 5.28, period: 3291, color: "#97BDEA"},
      {name: "Audio cleanup", badge: "A", price: 29, atMs: 1800, x: 264, y: 645, tilt: 10.5, scale: 0.94, phase: 5.21, period: 3274, color: "#D6C78D"},
      {name: "AI voiceover", badge: "V", price: 59, atMs: 2200, x: 680, y: 656, tilt: -5, scale: 1, phase: 1.41, period: 3130, color: "#B4C7A3"},
      {name: "Color grading", badge: "G", price: 15, atMs: 2600, x: 387, y: 845, tilt: -9.1, scale: 0.93, phase: 5.85, period: 2739, color: "#A2BDD4"},
      {name: "Video export", badge: "X", price: 35, atMs: 3000, x: 688, y: 102, tilt: 7.1, scale: 0.79, phase: 2.01, period: 2387, color: "#D6AFC2"},
    ],
  },
  computeds: {
    badgeInk: computed.string(() => vars.background),
    textInk: computed.string(() => vars.ink),
    mutedInk: computed.string(() => vars.muted),
    priceInk: computed.string(() => vars.accent),
    scale: computed.number(() => Math.min(vars.width / 960, vars.height / 960)),
    clock: computed.number(() => time.ms * Math.max(0.25, Math.min(2, vars.speed))),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="square-stage" x={(vars.width - 960 * computed.scale) / 2} y={(vars.height - 960 * computed.scale) / 2}
        width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Repeat items={tokens.cards} render={(card, index) => (
            <Group id={"subscription-" + index} width={340} height={98} pivotX={170} pivotY={49}
              x={card.x - 170 + 5 * sway(computed.clock, card, 0)}
              y={card.y - 49 + 7 * sway(computed.clock, card, 1.7)}
              rotation={card.tilt + 2.2 * sway(computed.clock, card, 3.1)}
              scaleX={card.scale * pop(computed.clock, card)} scaleY={card.scale * pop(computed.clock, card)}
              opacity={Math.max(0, Math.min(1, (computed.clock - card.atMs) / 180))}>
              <Rect id={"card-" + index} width={340} height={98} radius={17} fill={vars.surface}
                stroke={vars.border} strokeWidth={1} shadowColor="#000000" shadowBlur={20} shadowOffsetY={8} shadowOpacity={0.20} />
              <Image id={"lighting-" + index} src={tokens.cardLighting} width={340} height={98} />
              <Rect id={"badge-" + index} x={18} y={25} width={48} height={48} radius={13} fill={card.color}
                shadowColor={card.color} shadowBlur={14} shadowOpacity={0.24} />
              <Text id={"initial-" + index} text={card.badge} x={18} y={31} width={48} height={35}
                fontFamily="Inter" weight={700} size={25} lineHeight={32} align="center" color={computed.badgeInk} />
              <Text id={"name-" + index} text={card.name} x={81} y={25} width={170} height={28}
                fontFamily="Inter" weight={600} size={19} lineHeight={25} color={computed.textInk} />
              <Text id={"renewal-" + index} text="BILLED MONTHLY" x={81} y={57} width={166} height={20}
                fontFamily="Inter" weight={500} size={10} lineHeight={16} letterSpacing={1.2} color={computed.mutedInk} />
              <Text id={"price-" + index} text={"$" + card.price} x={245} y={23} width={77} height={34}
                fontFamily="Inter" weight={600} size={27} lineHeight={34} align="right" color={computed.priceInk} />
              <Text id={"period-" + index} text="/ month" x={245} y={59} width={77} height={18}
                fontFamily="Inter" weight={400} size={11} lineHeight={16} align="right" color={computed.mutedInk} />
            </Group>
        )} />
      </Group>
    </Scene>;
  },
});
