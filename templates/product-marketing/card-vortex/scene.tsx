// The video-tool card field drains into a finite-core clockwise vortex.
// Card x/y are center offsets; long axes follow the flow with inertial lag.
// Stable trait seed: "card vortex / video tools / v1"; no per-frame randomness.
function clamp01(value) {
  return Math.max(0, Math.min(1, value));
}
function smooth(value) {
  return clamp01(value) * clamp01(value) * (3 - 2 * clamp01(value));
}
function radiusSquared(card) {
  return Math.max(1, card.x * card.x + card.y * card.y);
}
function flowAge(card, clock) {
  return Math.max(0, clock - computed.hold - card.delay * computed.stagger);
}
function flowTime(card, clock) {
  return flowAge(card, clock) - computed.ramp * (1 - Math.exp(-flowAge(card, clock) / computed.ramp));
}
function coreTime(card, clock) {
  return Math.max(0, flowTime(card, clock) - Math.max(0, radiusSquared(card) - computed.coreSquared) / computed.drain);
}
// Shared sink: outer radius squared falls linearly; the finite core drains exponentially.
function cardRadius(card, clock) {
  return Math.sqrt(Math.max(Math.min(radiusSquared(card), computed.coreSquared),
    radiusSquared(card) - computed.drain * flowTime(card, clock))) *
    Math.exp(-computed.drain * coreTime(card, clock) / (2 * computed.coreSquared));
}
function logSeries(value) {
  return value * (1 + value * value * (1 / 3 + value * value / 5));
}
// ponytail: <0.1% log error for ratios 1–950; use Math.log once portable lowering supports it.
function vortexLog(value) {
  return 16 * logSeries((Math.pow(value, 0.125) - 1) / (Math.pow(value, 0.125) + 1));
}
// Free vortex outside the core: angular speed grows as 1/r², then stays bounded.
function bend(card, clock) {
  return computed.circulation * card.arc / computed.drain *
    vortexLog(Math.max(radiusSquared(card), computed.coreSquared) /
      Math.max(computed.coreSquared, radiusSquared(card) - computed.drain * flowTime(card, clock))) +
    computed.circulation * card.arc * coreTime(card, clock) / computed.coreSquared;
}
function cardProgress(card, clock) {
  return clamp01(1 - cardRadius(card, clock) / Math.sqrt(radiusSquared(card)));
}
function pathX(card, clock) {
  return (card.x * Math.cos(bend(card, clock)) - card.y * Math.sin(bend(card, clock))) *
    cardRadius(card, clock) / Math.sqrt(radiusSquared(card)) +
    2 * (Math.sin(clock / 680 + card.phase) - Math.sin(card.phase)) * (1 - smooth(flowAge(card, clock) / 180));
}
function pathY(card, clock) {
  return (card.x * Math.sin(bend(card, clock)) + card.y * Math.cos(bend(card, clock))) *
    cardRadius(card, clock) / Math.sqrt(radiusSquared(card)) +
    2.5 * (Math.sin(clock / 750 + card.phase + 1.7) - Math.sin(card.phase + 1.7)) * (1 - smooth(flowAge(card, clock) / 180));
}
function cardScale(card, clock) {
  return 0.53 * (1 - 0.2 * cardProgress(card, clock)) *
    smooth(cardRadius(card, clock) / Math.min(computed.coreRadius, Math.sqrt(radiusSquared(card))));
}
function flowHeading(card) {
  return Math.atan2(card.y, card.x) + Math.atan2(computed.circulation * card.arc, -computed.drain / 2) -
    card.tilt * Math.PI / 180;
}
function cardRotation(card, clock) {
  return card.tilt + smooth(flowAge(card, clock) / 240) *
    Math.atan2(Math.sin(flowHeading(card)), Math.cos(flowHeading(card))) * 180 / Math.PI +
    bend(card, clock - computed.lag) * 180 / Math.PI +
    2 * (Math.sin(clock / 600 + card.phase) - Math.sin(card.phase)) * (1 - smooth(flowAge(card, clock) / 260)) +
    2.5 * Math.sin(flowAge(card, clock) / 120 + card.phase) * smooth(flowAge(card, clock) / 260);
}
function cardBank(card, clock) {
  return smooth(flowAge(card, clock) / 180) *
    Math.min(1, 2 * computed.coreRadius / Math.max(computed.coreRadius, cardRadius(card, clock)));
}

export default defineSpanScene({
  id: "card-vortex",
  width: 960,
  height: 960,
  durationMs: 3000,
  editPreviewMs: 1400,
  variables: {
    width: numberVar(960), height: numberVar(960), speed: numberVar(1),
    holdMs: numberVar(550), collapseMs: numberVar(1700), staggerMs: numberVar(110),
    circulation: numberVar(300), coreRadius: numberVar(70), flowRampMs: numberVar(150), alignLagMs: numberVar(55),
    perspective: numberVar(900), cardBlur: numberVar(0.7), defocus: numberVar(3), shutterMs: numberVar(20),
    vignette: numberVar(0.5), flash: numberVar(0.7),
    background: colorVar("#101521"), surface: colorVar("#202A3B"), border: colorVar("#39465A"),
    ink: colorVar("#F3F4F8"), muted: colorVar("#A6B1C5"), accent: colorVar("#F5ACD1"),
  },
  tokens: {
    cardLighting: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 340 98"><defs><linearGradient id="wash" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="white" stop-opacity=".12"/><stop offset=".45" stop-color="white" stop-opacity=".025"/><stop offset="1" stop-color="black" stop-opacity=".24"/></linearGradient><linearGradient id="edge" x1="0" y1="0" x2=".7" y2="1"><stop offset="0" stop-color="white" stop-opacity=".2"/><stop offset="1" stop-color="white" stop-opacity=".015"/></linearGradient></defs><rect x=".5" y=".5" width="339" height="97" rx="16.5" fill="url(%23wash)" stroke="url(%23edge)"/></svg>',
    vignette: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 960 960"><defs><radialGradient id="v"><stop offset=".26" stop-color="%2302060E" stop-opacity="0"/><stop offset=".7" stop-color="%2302060E" stop-opacity=".25"/><stop offset="1" stop-color="%2302060E" stop-opacity=".94"/></radialGradient></defs><rect width="960" height="960" fill="url(%23v)"/></svg>',
    glow: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><defs><radialGradient id="g"><stop offset="0" stop-color="%23F2F6FF" stop-opacity="1"/><stop offset=".035" stop-color="%23C9D8FF" stop-opacity=".9"/><stop offset=".13" stop-color="%238699FF" stop-opacity=".48"/><stop offset=".4" stop-color="%23516EDB" stop-opacity=".14"/><stop offset="1" stop-color="%23344685" stop-opacity="0"/></radialGradient></defs><rect width="512" height="512" fill="url(%23g)"/></svg>',
    wave: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 960 960"><defs><radialGradient id="w"><stop offset=".69" stop-color="%23A1B2FF" stop-opacity="0"/><stop offset=".85" stop-color="%23A1B2FF" stop-opacity=".08"/><stop offset=".9" stop-color="%23DCE7FF" stop-opacity=".33"/><stop offset=".96" stop-color="%23859DFF" stop-opacity=".06"/><stop offset="1" stop-color="%23859DFF" stop-opacity="0"/></radialGradient></defs><rect width="960" height="960" fill="url(%23w)"/></svg>',
    // Ordered far-to-near once, not resorted as cards turn edge-on.
    cards: [
      { "name": "Video export", "badge": "X", "price": 35, "x": 112.89, "y": -200.75, "tilt": 4.9, "scale": 0.79, "color": "#D6AFC2", "phase": 6.115, "delay": 0.246, "arc": 1.031, "pitch": -13.55, "yaw": -67.56 },
      { "name": "Beat sync", "badge": "B", "price": 18, "x": -344.34, "y": -221.0, "tilt": -7.62, "scale": 0.88, "color": "#D6C78D", "phase": 2.613, "delay": 0.976, "arc": 1.054, "pitch": 33.55, "yaw": -73.95 },
      { "name": "Brand kit", "badge": "K", "price": 23, "x": 318.92, "y": -243.18, "tilt": 7.95, "scale": 0.89, "color": "#D6AFC2", "phase": 3.891, "delay": 0.135, "arc": 0.885, "pitch": 12.11, "yaw": -67.07 },
      { "name": "Background cutout", "badge": "B", "price": 21, "x": -399.96, "y": 388.67, "tilt": 6.58, "scale": 0.9, "color": "#D6AFC2", "phase": 2.532, "delay": 0.727, "arc": 0.739, "pitch": -39.16, "yaw": -65.67 },
      { "name": "Noise reducer", "badge": "N", "price": 19, "x": 322.74, "y": 116.62, "tilt": -10.99, "scale": 0.9, "color": "#8BCBC3", "phase": 1.616, "delay": 0.452, "arc": 1.188, "pitch": 11.84, "yaw": -47.61 },
      { "name": "Podcast editor", "badge": "P", "price": 25, "x": 346.39, "y": 505.63, "tilt": 7.9, "scale": 0.9, "color": "#A2BDD4", "phase": 3.403, "delay": 0.553, "arc": 0.884, "pitch": -30.73, "yaw": -72.65 },
      { "name": "Background remover", "badge": "B", "price": 33, "x": 538.33, "y": 214.17, "tilt": 1.02, "scale": 0.9, "color": "#B4C7A3", "phase": 4.518, "delay": 0.657, "arc": 0.991, "pitch": 17.5, "yaw": 72.37 },
      { "name": "Auto reframe", "badge": "F", "price": 14, "x": -294.07, "y": 3.28, "tilt": -2.69, "scale": 0.91, "color": "#8BCBC3", "phase": 0.264, "delay": 0.863, "arc": 1.226, "pitch": 54.28, "yaw": 59.6 },
      { "name": "Frame interpolation", "badge": "F", "price": 32, "x": 220.19, "y": -467.37, "tilt": -2.94, "scale": 0.91, "color": "#F0CAA2", "phase": 4.92, "delay": 0.017, "arc": 0.801, "pitch": -23.72, "yaw": -75.41 },
      { "name": "Sound effects", "badge": "S", "price": 25, "x": 204.0, "y": 401.98, "tilt": -10.26, "scale": 0.91, "color": "#D6C78D", "phase": 5.583, "delay": 0.265, "arc": 0.898, "pitch": -19.0, "yaw": -58.73 },
      { "name": "Video editor", "badge": "E", "price": 39, "x": 120.14, "y": -99.49, "tilt": 10.14, "scale": 0.92, "color": "#8BCBC3", "phase": 1.834, "delay": 0.627, "arc": 1.109, "pitch": -15.59, "yaw": 59.43 },
      { "name": "B-roll library", "badge": "B", "price": 28, "x": 394.68, "y": 358.68, "tilt": -9.69, "scale": 0.92, "color": "#B9B2EF", "phase": 4.098, "delay": 0.432, "arc": 0.848, "pitch": -0.32, "yaw": 59.79 },
      { "name": "Motion presets", "badge": "M", "price": 13, "x": 145.97, "y": 205.67, "tilt": -5.12, "scale": 0.92, "color": "#A2BDD4", "phase": 3.198, "delay": 0.528, "arc": 1.068, "pitch": 6.77, "yaw": 73.72 },
      { "name": "Color grading", "badge": "G", "price": 15, "x": -51.94, "y": 194.08, "tilt": -6.9, "scale": 0.93, "color": "#A2BDD4", "phase": 5.406, "delay": 0.317, "arc": 1.019, "pitch": 22.57, "yaw": 72.84 },
      { "name": "Subtitle translator", "badge": "S", "price": 26, "x": -329.11, "y": 133.74, "tilt": 4.68, "scale": 0.93, "color": "#F0CAA2", "phase": 1.933, "delay": 0.952, "arc": 0.861, "pitch": 37.31, "yaw": -48.35 },
      { "name": "Teleprompter", "badge": "P", "price": 13, "x": 324.38, "y": -2.66, "tilt": 6.49, "scale": 0.93, "color": "#A2BDD4", "phase": 3.887, "delay": 0.455, "arc": 0.967, "pitch": -15.89, "yaw": 64.85 },
      { "name": "Voice isolation", "badge": "V", "price": 29, "x": 42.8, "y": -396.59, "tilt": -7.54, "scale": 0.93, "color": "#B4C7A3", "phase": 4.713, "delay": 0.14, "arc": 0.743, "pitch": 13.78, "yaw": -68.0 },
      { "name": "Audio cleanup", "badge": "A", "price": 29, "x": -116.05, "y": 84.77, "tilt": 11.73, "scale": 0.94, "color": "#D6C78D", "phase": 1.166, "delay": 0.261, "arc": 0.871, "pitch": -4.94, "yaw": -61.34 },
      { "name": "Clip trimmer", "badge": "T", "price": 12, "x": -415.04, "y": -415.92, "tilt": 7.09, "scale": 0.94, "color": "#A2BDD4", "phase": 1.904, "delay": 0.122, "arc": 1.172, "pitch": -34.05, "yaw": -45.22 },
      { "name": "Transcription", "badge": "T", "price": 20, "x": 61.64, "y": -317.47, "tilt": 7.76, "scale": 0.94, "color": "#B4C7A3", "phase": 2.692, "delay": 0.031, "arc": 1.04, "pitch": 22.21, "yaw": -61.26 },
      { "name": "Video review", "badge": "V", "price": 18, "x": -495.82, "y": -309.32, "tilt": -0.21, "scale": 0.94, "color": "#97BDEA", "phase": 1.822, "delay": 0.746, "arc": 1.025, "pitch": 13.4, "yaw": -73.92 },
      { "name": "Speed ramps", "badge": "S", "price": 12, "x": -515.05, "y": 198.35, "tilt": 3.82, "scale": 0.94, "color": "#8BCBC3", "phase": 5.047, "delay": 0.429, "arc": 0.919, "pitch": -0.79, "yaw": -56.75 },
      { "name": "Auto captions", "badge": "C", "price": 19, "x": -133.07, "y": -60.58, "tilt": -3.21, "scale": 0.95, "color": "#F0CAA2", "phase": 2.001, "delay": 0.373, "arc": 0.998, "pitch": 20.79, "yaw": 65.31 },
      { "name": "Storyboard", "badge": "S", "price": 16, "x": 389.44, "y": -391.85, "tilt": -8.18, "scale": 0.95, "color": "#B4C7A3", "phase": 5.187, "delay": 0.671, "arc": 0.995, "pitch": -3.46, "yaw": 60.57 },
      { "name": "Asset manager", "badge": "A", "price": 17, "x": -223.33, "y": 432.89, "tilt": 8.39, "scale": 0.95, "color": "#8BCBC3", "phase": 5.932, "delay": 0.295, "arc": 1.179, "pitch": -10.35, "yaw": 61.07 },
      { "name": "Frame capture", "badge": "F", "price": 27, "x": -503.49, "y": -83.11, "tilt": -1.0, "scale": 0.95, "color": "#8BCBC3", "phase": 3.645, "delay": 0.453, "arc": 0.76, "pitch": 47.87, "yaw": -48.37 },
      { "name": "Video compressor", "badge": "V", "price": 17, "x": -263.44, "y": 263.87, "tilt": -9.47, "scale": 0.96, "color": "#97BDEA", "phase": 6.206, "delay": 0.124, "arc": 1.216, "pitch": 43.31, "yaw": 49.25 },
      { "name": "LUT library", "badge": "L", "price": 15, "x": 350.53, "y": 246.13, "tilt": 4.18, "scale": 0.96, "color": "#D6C78D", "phase": 4.597, "delay": 0.391, "arc": 0.766, "pitch": 7.78, "yaw": 69.32 },
      { "name": "Thumbnail maker", "badge": "T", "price": 14, "x": -161.77, "y": -300.98, "tilt": -10.43, "scale": 0.96, "color": "#97BDEA", "phase": 0.604, "delay": 0.266, "arc": 1.099, "pitch": 29.82, "yaw": 60.56 },
      { "name": "Media search", "badge": "M", "price": 13, "x": -12.86, "y": 449.13, "tilt": 6.13, "scale": 0.96, "color": "#D6C78D", "phase": 5.174, "delay": 0.866, "arc": 0.907, "pitch": -9.46, "yaw": -69.67 },
      { "name": "Proxy maker", "badge": "P", "price": 14, "x": 539.5, "y": -200.58, "tilt": -3.91, "scale": 0.96, "color": "#D6C78D", "phase": 2.005, "delay": 0.717, "arc": 0.94, "pitch": -31.73, "yaw": -45.45 },
      { "name": "Version history", "badge": "V", "price": 30, "x": 543.0, "y": -57.32, "tilt": -3.3, "scale": 0.96, "color": "#8BCBC3", "phase": 6.032, "delay": 0.693, "arc": 0.952, "pitch": -13.25, "yaw": -49.54 },
      { "name": "Audio mixer", "badge": "A", "price": 32, "x": -484.68, "y": 51.96, "tilt": -2.06, "scale": 0.97, "color": "#D6AFC2", "phase": 3.803, "delay": 0.86, "arc": 0.785, "pitch": 11.61, "yaw": -75.57 },
      { "name": "Render queue", "badge": "R", "price": 23, "x": -478.49, "y": 321.23, "tilt": 8.51, "scale": 0.97, "color": "#A2BDD4", "phase": 5.765, "delay": 0.844, "arc": 0.95, "pitch": 43.48, "yaw": 75.91 },
      { "name": "Music library", "badge": "M", "price": 11, "x": 310.1, "y": -126.9, "tilt": -3.26, "scale": 0.98, "color": "#F0CAA2", "phase": 2.284, "delay": 0.641, "arc": 0.888, "pitch": -31.32, "yaw": -68.59 },
      { "name": "Scene composer", "badge": "S", "price": 31, "x": 40.88, "y": 299.92, "tilt": -6.46, "scale": 0.98, "color": "#D6AFC2", "phase": 3.396, "delay": 0.885, "arc": 1.06, "pitch": 31.13, "yaw": 58.62 },
      { "name": "Shot planner", "badge": "S", "price": 16, "x": -171.76, "y": -393.92, "tilt": -1.36, "scale": 0.99, "color": "#B4C7A3", "phase": 6.236, "delay": 0.265, "arc": 0.798, "pitch": -16.49, "yaw": 73.13 },
      { "name": "Multicam editor", "badge": "M", "price": 30, "x": 532.59, "y": 89.18, "tilt": 3.94, "scale": 0.99, "color": "#D6AFC2", "phase": 3.545, "delay": 0.754, "arc": 1.234, "pitch": -26.2, "yaw": 75.54 },
      { "name": "AI voiceover", "badge": "V", "price": 59, "x": 105.18, "y": 96.93, "tilt": -4.23, "scale": 1, "color": "#B4C7A3", "phase": 0.468, "delay": 0.608, "arc": 1.025, "pitch": -25.86, "yaw": -57.44 },
      { "name": "Title designer", "badge": "T", "price": 22, "x": -301.84, "y": -118.19, "tilt": 8.8, "scale": 1.01, "color": "#B9B2EF", "phase": 3.955, "delay": 0.622, "arc": 0.914, "pitch": -1.67, "yaw": 52.71 },
      { "name": "Screen recorder", "badge": "R", "price": 24, "x": -88.53, "y": -158.46, "tilt": -7.19, "scale": 1.02, "color": "#B9B2EF", "phase": 5.335, "delay": 0.282, "arc": 0.99, "pitch": 16.37, "yaw": -57.13 },
      { "name": "Motion graphics", "badge": "M", "price": 49, "x": 41.37, "y": 2.53, "tilt": -7.15, "scale": 1.05, "color": "#97BDEA", "phase": 0.183, "delay": 0.319, "arc": 1.145, "pitch": -5.3, "yaw": 59.86 },
    ],
  },
  computeds: {
    badgeInk: computed.string(() => vars.background), textInk: computed.string(() => vars.ink),
    mutedInk: computed.string(() => vars.muted), priceInk: computed.string(() => vars.accent),
    scale: computed.number(() => Math.min(vars.width / 960, vars.height / 960)),
    clock: computed.number(() => time.ms * Math.max(0.25, Math.min(2, vars.speed))),
    hold: computed.number(() => Math.max(0, vars.holdMs)),
    collapse: computed.number(() => Math.max(700, vars.collapseMs)),
    stagger: computed.number(() => Math.max(0, Math.min(400, vars.staggerMs))),
    ramp: computed.number(() => Math.max(40, Math.min(300, vars.flowRampMs))),
    coreRadius: computed.number(() => Math.max(20, Math.min(160, vars.coreRadius))),
    coreSquared: computed.number(() => computed.coreRadius * computed.coreRadius),
    // Squared reach of the authored field; every card experiences the same inward flow.
    drain: computed.number(() => 380000 / Math.max(300, computed.collapse - computed.ramp - 150)),
    circulation: computed.number(() => Math.max(20, vars.circulation)),
    lag: computed.number(() => Math.max(0, Math.min(160, vars.alignLagMs))),
    shutter: computed.number(() => Math.max(0, Math.min(32, vars.shutterMs))),
    handoff: computed.number(() => computed.hold + computed.collapse + computed.stagger),
    core: computed.number(() => smooth((computed.clock - computed.handoff + 230) / 200)),
    pulse: computed.number(() => smooth((computed.clock - computed.handoff + 80) / 80) * (1 - smooth((computed.clock - computed.handoff) / 260))),
    wave: computed.number(() => clamp01((computed.clock - computed.handoff) / 420)),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="square-stage" x={vars.width / 2} y={vars.height / 2} width={0} height={0} pivotX={0} pivotY={0}
        scaleX={computed.scale} scaleY={computed.scale}>
        {/* Centered shutter displacement follows each card's curved screen-space velocity. */}
        <Repeat items={tokens.cards} render={(card, index) => (
          <Group id={"card-flight-" + index} x={pathX(card, computed.clock)} y={pathY(card, computed.clock)}
            width={0} height={0} pivotX={0} pivotY={0}
            opacity={smooth(cardRadius(card, computed.clock) / (0.4 * Math.min(computed.coreRadius, Math.sqrt(radiusSquared(card)))))}
            blur={Math.max(0, vars.cardBlur) + Math.max(0, vars.defocus) * Math.pow(cardProgress(card, computed.clock), 2)}
            directionalBlurX={(pathX(card, computed.clock + 8) - pathX(card, computed.clock - 8)) * computed.shutter / 16}
            directionalBlurY={(pathY(card, computed.clock + 8) - pathY(card, computed.clock - 8)) * computed.shutter / 16}>
              <Group id={"subscription-" + index} x={-170} y={-49} width={340} height={98} pivotX={170} pivotY={49}
                scaleX={card.scale * cardScale(card, computed.clock)}
                scaleY={card.scale * cardScale(card, computed.clock)}
                rotation={cardRotation(card, computed.clock)}
                rotationX={cardBank(card, computed.clock) * (28 + card.pitch / 8 * Math.sin(flowAge(card, computed.clock) / 130 + card.phase))}
                rotationY={cardBank(card, computed.clock) * (card.yaw * 0.42 + 10 * Math.sin(flowAge(card, computed.clock) / 170 + card.phase))}
                perspective={Math.max(500, vars.perspective)}>
                <Rect id={"card-" + index} width={340} height={98} radius={17} fill={vars.surface}
                  stroke={vars.border} strokeWidth={1} shadowColor="#000000" shadowBlur={20} shadowOffsetY={8} shadowOpacity={0.2} />
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
            </Group>
        )} />
        <Image id="vignette" src={tokens.vignette} x={-480} y={-480} width={960} height={960}
          opacity={clamp01(vars.vignette)} />
        {/* The light handoff ends this scene; the logo assembly is a separate template. */}
        <Image id="core-glow" src={tokens.glow} x={-256} y={-256} width={512} height={512} pivotX={256} pivotY={256}
          scaleX={0.36 + 1.65 * computed.pulse} scaleY={0.36 + 1.65 * computed.pulse}
          opacity={computed.core * (0.38 + 0.62 * clamp01(vars.flash) * computed.pulse)} />
        <Image id="handoff-wave" src={tokens.wave} x={-480} y={-480} width={960} height={960} pivotX={480} pivotY={480}
          scaleX={0.025 + 1.45 * (1 - Math.pow(1 - computed.wave, 2))}
          scaleY={0.025 + 1.45 * (1 - Math.pow(1 - computed.wave, 2))}
          opacity={clamp01(vars.flash) * smooth(computed.wave / 0.1) * (1 - smooth(computed.wave))} />
        <Rect id="light-point" x={-3.5} y={-3.5} width={7} height={7} radius={3.5} fill="#F2F6FF"
          opacity={computed.core} shadowColor="#A1B2FF" shadowBlur={18} shadowOpacity={0.8} />
      </Group>
    </Scene>;
  },
});
