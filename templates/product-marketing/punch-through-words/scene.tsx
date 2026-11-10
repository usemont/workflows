// The approved subscription constellation continues at its 4.2-second pose.
// Three camera pullbacks trigger new, center-origin spring entrances.
// Stable variation seed: "punch-through words / expanding tool constellation / v1".
function clamp01(value) {
  return Math.max(0, Math.min(1, value));
}
function step(clock, cue) {
  return 1 - Math.pow(1 - clamp01((clock - cue) / 180), 4);
}
function depth(clock, first, beat) {
  return step(clock, first) + step(clock, first + beat) + step(clock, first + 2 * beat);
}
function cameraZoom(clock, first, beat) {
  return 1 - 0.17 * step(clock, first) - 0.16 * step(clock, first + beat) - 0.14 * step(clock, first + 2 * beat);
}
function impact(clock, first, beat, phase) {
  return kick(clock - first, phase) + 1.45 * kick(clock - first - beat, phase + 1.8) + 1.9 * kick(clock - first - 2 * beat, phase + 4.1);
}
function kick(age, phase) {
  return age < 0 ? 0 : Math.exp(-age / 85) * Math.sin(age / 23 + phase);
}
function cardAge(clock, card, first, beat) {
  return Math.max(0, (card.atMs > 0 ? 4200 + clock - card.atMs : clock - first - (card.enterBeat - 1) * beat - card.delayMs) / 1000);
}
function spring(age) {
  return 1 - Math.exp(-8 * age) * Math.cos(13 * age);
}
function pop(clock, card, first, beat) {
  return spring(cardAge(clock, card, first, beat));
}
function floatAt(age, card, offset) {
  return Math.sin(age * 6283.185 / card.period + card.phase + offset) * (1 - Math.exp(-3 * age));
}
function sway(clock, card, offset, first, beat) {
  return floatAt(cardAge(clock, card, first, beat), card, offset);
}
// The 160 ms approach ends on the shared camera, reveal and shake impact cue.
function stamp(age) {
  return Math.pow(clamp01(age / 160), 3);
}
function wordOpacity(age, beat, last) {
  return age < 0 ? 0 : (0.32 + 0.68 * stamp(age)) * (last ? 1 : 1 - clamp01((age - beat + 100) / 100));
}
function wordScale(age) {
  return 1 + 0.45 * (1 - stamp(age));
}

export default defineSpanScene({
  id: "punch-through-words",
  width: 960,
  height: 960,
  durationMs: 3400,
  editPreviewMs: 2350,
  variables: {
    width: numberVar(960), height: numberVar(960), speed: numberVar(1),
    firstWord: stringVar("Too."), secondWord: stringVar("Many."), thirdWord: stringVar("Tools."),
    firstImpactMs: numberVar(700), beatMs: numberVar(650), fontSize: numberVar(204),
    cardBlur: numberVar(3.8), shutterMs: numberVar(22), shakePx: numberVar(12),
    vignette: numberVar(0.5), wordShade: numberVar(0.65),
    background: colorVar("#101521"), surface: colorVar("#202A3B"), border: colorVar("#39465A"),
    ink: colorVar("#F3F4F8"), muted: colorVar("#A6B1C5"), accent: colorVar("#F5ACD1"),
  },
  tokens: {
    cardLighting: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 340 98"><defs><linearGradient id="wash" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="white" stop-opacity=".12"/><stop offset=".45" stop-color="white" stop-opacity=".025"/><stop offset="1" stop-color="black" stop-opacity=".24"/></linearGradient><linearGradient id="edge" x1="0" y1="0" x2=".7" y2="1"><stop offset="0" stop-color="white" stop-opacity=".2"/><stop offset="1" stop-color="white" stop-opacity=".015"/></linearGradient></defs><rect x=".5" y=".5" width="339" height="97" rx="16.5" fill="url(%23wash)" stroke="url(%23edge)"/></svg>',
    vignette: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 960 960"><defs><radialGradient id="v"><stop offset=".26" stop-color="%2302060E" stop-opacity="0"/><stop offset=".7" stop-color="%2302060E" stop-opacity=".25"/><stop offset="1" stop-color="%2302060E" stop-opacity=".94"/></radialGradient></defs><rect width="960" height="960" fill="url(%23v)"/></svg>',
    wordShade: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 960 960"><defs><radialGradient id="s" cx=".5" cy=".57" r=".5" gradientTransform="translate(0 .171) scale(1 .7)"><stop offset="0" stop-color="%2302060E" stop-opacity=".97"/><stop offset=".43" stop-color="%2302060E" stop-opacity=".9"/><stop offset="1" stop-color="%2302060E" stop-opacity="0"/></radialGradient></defs><rect width="960" height="960" fill="url(%23s)"/></svg>',
    cards: [
      {name: "Screen recorder", badge: "R", price: 24, atMs: 300, x: 315, y: 187, tilt: -8, scale: 1.02, phase: 3.65, period: 2445, color: "#B9B2EF"},
      {name: "Video editor", badge: "E", price: 39, atMs: 650, x: 711, y: 288, tilt: 8.2, scale: 0.92, phase: 1.83, period: 2733, color: "#8BCBC3"},
      {name: "Auto captions", badge: "C", price: 19, atMs: 1000, x: 231, y: 359, tilt: -4.2, scale: 0.95, phase: 5.41, period: 3187, color: "#F0CAA2"},
      {name: "Motion graphics", badge: "M", price: 49, atMs: 1400, x: 563, y: 485, tilt: -9.3, scale: 1.05, phase: 5.28, period: 3291, color: "#97BDEA"},
      {name: "Audio cleanup", badge: "A", price: 29, atMs: 1800, x: 264, y: 645, tilt: 10.5, scale: 0.94, phase: 5.21, period: 3274, color: "#D6C78D"},
      {name: "AI voiceover", badge: "V", price: 59, atMs: 2200, x: 680, y: 656, tilt: -5, scale: 1, phase: 1.41, period: 3130, color: "#B4C7A3"},
      {name: "Color grading", badge: "G", price: 15, atMs: 2600, x: 387, y: 845, tilt: -9.1, scale: 0.93, phase: 5.85, period: 2739, color: "#A2BDD4"},
      {name: "Video export", badge: "X", price: 35, atMs: 3000, x: 688, y: 102, tilt: 7.1, scale: 0.79, phase: 2.01, period: 2387, color: "#D6AFC2"},
      {name: "Clip trimmer", badge: "T", price: 12, atMs: 0, enterBeat: 3, delayMs: 25, x: -300, y: -300, tilt: 5.8, scale: 0.94, phase: 0.85, period: 2907, color: "#A2BDD4"},
      {name: "Beat sync", badge: "B", price: 18, atMs: 0, enterBeat: 1, delayMs: 20, x: -170, y: 70, tilt: -7.4, scale: 0.88, phase: 4.12, period: 3211, color: "#D6C78D"},
      {name: "Title designer", badge: "T", price: 22, atMs: 0, enterBeat: 1, delayMs: 35, x: -90, y: 264, tilt: 9.1, scale: 1.01, phase: 2.68, period: 2519, color: "#B9B2EF"},
      {name: "Auto reframe", badge: "F", price: 14, atMs: 0, enterBeat: 1, delayMs: 85, x: -70, y: 487, tilt: -4.8, scale: 0.91, phase: 5.43, period: 3089, color: "#8BCBC3"},
      {name: "Subtitle translator", badge: "S", price: 26, atMs: 0, enterBeat: 1, delayMs: 65, x: -145, y: 729, tilt: 6.4, scale: 0.93, phase: 1.23, period: 2761, color: "#F0CAA2"},
      {name: "Video compressor", badge: "V", price: 17, atMs: 0, enterBeat: 1, delayMs: 105, x: -20, y: 984, tilt: -8.1, scale: 0.96, phase: 3.96, period: 3373, color: "#97BDEA"},
      {name: "Background cutout", badge: "B", price: 21, atMs: 0, enterBeat: 2, delayMs: 90, x: -270, y: 1210, tilt: 4.5, scale: 0.9, phase: 0.42, period: 2641, color: "#D6AFC2"},
      {name: "Storyboard", badge: "S", price: 16, atMs: 0, enterBeat: 3, delayMs: 70, x: 1210, y: -260, tilt: -6.1, scale: 0.95, phase: 4.86, period: 3017, color: "#B4C7A3"},
      {name: "Brand kit", badge: "K", price: 23, atMs: 0, enterBeat: 1, delayMs: 45, x: 1080, y: 28, tilt: 8.8, scale: 0.89, phase: 2.31, period: 2467, color: "#D6AFC2"},
      {name: "Music library", badge: "M", price: 11, atMs: 0, enterBeat: 1, delayMs: 60, x: 1070, y: 241, tilt: -5.4, scale: 0.98, phase: 5.78, period: 3301, color: "#F0CAA2"},
      {name: "Teleprompter", badge: "P", price: 13, atMs: 0, enterBeat: 1, delayMs: 15, x: 1090, y: 469, tilt: 7.3, scale: 0.93, phase: 0.67, period: 2797, color: "#A2BDD4"},
      {name: "Noise reducer", badge: "N", price: 19, atMs: 0, enterBeat: 1, delayMs: 80, x: 1085, y: 705, tilt: -9.2, scale: 0.9, phase: 3.24, period: 3121, color: "#8BCBC3"},
      {name: "LUT library", badge: "L", price: 15, atMs: 0, enterBeat: 2, delayMs: 0, x: 1140, y: 938, tilt: 4.7, scale: 0.96, phase: 1.57, period: 2579, color: "#D6C78D"},
      {name: "B-roll library", badge: "B", price: 28, atMs: 0, enterBeat: 2, delayMs: 40, x: 1220, y: 1160, tilt: -7.6, scale: 0.92, phase: 4.53, period: 3433, color: "#B9B2EF"},
      {name: "Thumbnail maker", badge: "T", price: 14, atMs: 0, enterBeat: 1, delayMs: 0, x: 170, y: -85, tilt: -8.3, scale: 0.96, phase: 2.17, period: 2837, color: "#97BDEA"},
      {name: "Transcription", badge: "T", price: 20, atMs: 0, enterBeat: 1, delayMs: 35, x: 600, y: -115, tilt: 6.2, scale: 0.94, phase: 5.02, period: 3197, color: "#B4C7A3"},
      {name: "Frame interpolation", badge: "F", price: 32, atMs: 0, enterBeat: 3, delayMs: 5, x: 900, y: -400, tilt: -4.9, scale: 0.91, phase: 1.06, period: 2693, color: "#F0CAA2"},
      {name: "Asset manager", badge: "A", price: 17, atMs: 0, enterBeat: 3, delayMs: 95, x: 60, y: 1290, tilt: 7.7, scale: 0.95, phase: 3.49, period: 3271, color: "#8BCBC3"},
      {name: "Scene composer", badge: "S", price: 31, atMs: 0, enterBeat: 1, delayMs: 100, x: 555, y: 1040, tilt: -5.6, scale: 0.98, phase: 0.29, period: 2539, color: "#D6AFC2"},
      {name: "Podcast editor", badge: "P", price: 25, atMs: 0, enterBeat: 3, delayMs: 110, x: 1130, y: 1430, tilt: 9.4, scale: 0.9, phase: 4.37, period: 2971, color: "#A2BDD4"},
      {name: "Motion presets", badge: "M", price: 13, atMs: 0, enterBeat: 1, delayMs: 105, x: 760, y: 870, tilt: -7.1, scale: 0.92, phase: 5.55, period: 3238, color: "#A2BDD4"},
      {name: "Shot planner", badge: "S", price: 16, atMs: 0, enterBeat: 2, delayMs: 80, x: 160, y: -260, tilt: -3.1, scale: 0.99, phase: 6.19, period: 2953, color: "#B4C7A3"},
      {name: "Voice isolation", badge: "V", price: 29, atMs: 0, enterBeat: 2, delayMs: 10, x: 560, y: -275, tilt: -7.3, scale: 0.93, phase: 2.47, period: 3232, color: "#B4C7A3"},
      {name: "Video review", badge: "V", price: 18, atMs: 0, enterBeat: 3, delayMs: 30, x: -460, y: -100, tilt: 1.8, scale: 0.94, phase: 5.04, period: 2658, color: "#97BDEA"},
      {name: "Frame capture", badge: "F", price: 27, atMs: 0, enterBeat: 3, delayMs: 65, x: -470, y: 330, tilt: -0.9, scale: 0.95, phase: 0.1, period: 2760, color: "#8BCBC3"},
      {name: "Audio mixer", badge: "A", price: 32, atMs: 0, enterBeat: 3, delayMs: 20, x: -430, y: 580, tilt: -4, scale: 0.97, phase: 1.21, period: 2811, color: "#D6AFC2"},
      {name: "Speed ramps", badge: "S", price: 12, atMs: 0, enterBeat: 3, delayMs: 20, x: -495, y: 860, tilt: 5.3, scale: 0.94, phase: 5.26, period: 2510, color: "#8BCBC3"},
      {name: "Render queue", badge: "R", price: 23, atMs: 0, enterBeat: 3, delayMs: 50, x: -420, y: 1080, tilt: 7.2, scale: 0.97, phase: 2.49, period: 2664, color: "#A2BDD4"},
      {name: "Media search", badge: "M", price: 13, atMs: 0, enterBeat: 3, delayMs: 100, x: 460, y: 1330, tilt: 4.3, scale: 0.96, phase: 1.09, period: 2628, color: "#D6C78D"},
      {name: "Sound effects", badge: "S", price: 25, atMs: 0, enterBeat: 3, delayMs: 15, x: 860, y: 1240, tilt: -8.1, scale: 0.91, phase: 4.73, period: 2701, color: "#D6C78D"},
      {name: "Proxy maker", badge: "P", price: 14, atMs: 0, enterBeat: 3, delayMs: 85, x: 1500, y: 95, tilt: -4.9, scale: 0.96, phase: 2.69, period: 2617, color: "#D6C78D"},
      {name: "Version history", badge: "V", price: 30, atMs: 0, enterBeat: 3, delayMs: 100, x: 1505, y: 365, tilt: -3.6, scale: 0.96, phase: 3.69, period: 3274, color: "#8BCBC3"},
      {name: "Multicam editor", badge: "M", price: 30, atMs: 0, enterBeat: 3, delayMs: 0, x: 1480, y: 650, tilt: 6.1, scale: 0.99, phase: 4.85, period: 2812, color: "#D6AFC2"},
      {name: "Background remover", badge: "B", price: 33, atMs: 0, enterBeat: 3, delayMs: 90, x: 1500, y: 880, tilt: -0.9, scale: 0.9, phase: 2.57, period: 3103, color: "#B4C7A3"},
    ],
  },
  computeds: {
    badgeInk: computed.string(() => vars.background), textInk: computed.string(() => vars.ink),
    mutedInk: computed.string(() => vars.muted), priceInk: computed.string(() => vars.accent),
    scale: computed.number(() => Math.min(vars.width / 960, vars.height / 960)),
    clock: computed.number(() => time.ms * Math.max(0.25, Math.min(2, vars.speed))),
    first: computed.number(() => Math.max(160, vars.firstImpactMs)),
    approach: computed.number(() => computed.first - 160),
    beat: computed.number(() => Math.max(300, vars.beatMs)),
    depth: computed.number(() => depth(computed.clock, computed.first, computed.beat)),
    zoom: computed.number(() => cameraZoom(computed.clock, computed.first, computed.beat)),
    // Centered shutter displacement: measured camera velocity, not Gaussian defocus.
    shutter: computed.number(() => Math.max(0, Math.min(32, vars.shutterMs)) *
      (cameraZoom(computed.clock + 8, computed.first, computed.beat) - cameraZoom(computed.clock - 8, computed.first, computed.beat)) / 16 / computed.zoom),
    shakeX: computed.number(() => Math.max(0, vars.shakePx) * impact(computed.clock, computed.first, computed.beat, 0.8)),
    shakeY: computed.number(() => Math.max(0, vars.shakePx) * 0.63 * impact(computed.clock, computed.first, computed.beat, 2.9)),
    shakeRoll: computed.number(() => 0.035 * Math.max(0, vars.shakePx) * impact(computed.clock, computed.first, computed.beat, 4.3)),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="square-stage" x={vars.width / 2} y={vars.height / 2} width={0} height={0} pivotX={0} pivotY={0}
        scaleX={computed.scale} scaleY={computed.scale}>
        <Group id="impact-camera" x={computed.shakeX} y={computed.shakeY} rotation={computed.shakeRoll} width={0} height={0} pivotX={0} pivotY={0}>
          <Group id="card-camera" width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.zoom} scaleY={computed.zoom}
            opacity={1 - 0.035 * computed.depth}>
            <Repeat items={tokens.cards} render={(card, index) => (
              <Group id={"card-focus-" + index} width={0} height={0} pivotX={0} pivotY={0}
                x={card.x - 480 + 5 * sway(computed.clock, card, 0, computed.first, computed.beat)} y={card.y - 480 + 7 * sway(computed.clock, card, 1.7, computed.first, computed.beat)}
                blur={Math.max(0, vars.cardBlur) * computed.depth / 3 / computed.zoom}
                directionalBlurX={(card.x - 480) * computed.shutter} directionalBlurY={(card.y - 480) * computed.shutter}>
                <Group id={"subscription-" + index} x={-170} y={-49} width={340} height={98} pivotX={170} pivotY={49}
                  rotation={card.tilt + 2.2 * sway(computed.clock, card, 3.1, computed.first, computed.beat)}
                  scaleX={card.scale * pop(computed.clock, card, computed.first, computed.beat)}
                  scaleY={card.scale * pop(computed.clock, card, computed.first, computed.beat)}>
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
              </Group>
            )} />
          </Group>
        </Group>
          <Image id="soft-word-underlay" src={tokens.wordShade} x={-480} y={-480} width={960} height={960}
            opacity={clamp01(vars.wordShade) * clamp01(computed.depth)} />
          <Repeat items={[
            {text: vars.firstWord, at: computed.approach, last: false},
            {text: vars.secondWord, at: computed.approach + computed.beat, last: false},
            {text: vars.thirdWord, at: computed.approach + 2 * computed.beat, last: true},
          ]} render={(word, index) => (
            <Repeat items={[text.glyphs(word.text, {fontFamily: "Archivo", fontWeight: 900, fontSize: 100})[0]]} render={(metrics) => (
              <Group id={"punch-word-" + index} width={0} height={0} pivotX={0} pivotY={0}
                scaleX={Math.min(Math.max(12, vars.fontSize) / 100, 760 / Math.max(1, metrics.totalAdvance)) * wordScale(computed.clock - word.at)}
                scaleY={Math.min(Math.max(12, vars.fontSize) / 100, 760 / Math.max(1, metrics.totalAdvance)) * wordScale(computed.clock - word.at)}
                opacity={wordOpacity(computed.clock - word.at, computed.beat, word.last)}
                blur={4.5 * (1 - stamp(computed.clock - word.at)) + (word.last ? 0 : 12 * Math.pow(clamp01((computed.clock - word.at - computed.beat + 100) / 100), 2))}>
                <Text id={"word-" + index} text={word.text} x={-metrics.totalAdvance / 2} y={-65} width={metrics.totalAdvance + 8} height={150}
                  fontFamily="Archivo" weight={900} size={100} lineHeight={130} color={computed.textInk} />
              </Group>
            )} />
          )} />
        <Image id="edge-vignette" src={tokens.vignette} x={-480} y={-480} width={960} height={960}
          opacity={clamp01(vars.vignette) * (0.32 + 0.68 * clamp01(computed.depth))} />
      </Group>
    </Scene>;
  },
});
