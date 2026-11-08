// Native adaptation of https://hyperframes.heygen.com/catalog/components/cursor-annotate.
// Copyright 2026 HeyGen, Inc. Apache-2.0; see ../../subscribe/LICENSE.
// Original Mont production copy; vector marker and a pressure-shaped portable ink ribbon.
function annotationX(p) {
  return 836 + 163 * ((1 + 0.18 * Math.pow(p, 6)) * Math.cos((2 * Math.PI + 0.3) * p - 2.45) - Math.cos(2.45)) + 4 * Math.sin(2 * Math.PI * p);
}
function annotationY(p) {
  return 514 + 106 * ((1 + 0.18 * Math.pow(p, 6)) * Math.sin((2 * Math.PI + 0.3) * p - 2.45) + Math.sin(2.45)) + 3 * Math.sin(4 * Math.PI * p);
}
function annotationTaper(p) {
  return p * p * (3 - 2 * p);
}
function annotationPressure(p) {
  return (0.8 + 0.035 * Math.sin(2 * Math.PI * p)) * annotationTaper(Math.min(1, p / 0.025)) * annotationTaper(Math.min(1, (1 - p) / 0.035));
}
function annotationHalfWidth(p) {
  return 3.25 * (0.04 + 0.96 * annotationPressure(p));
}
function annotationRibbonPoint(line, p, side) {
  return {
    x: path.pointAt(line, p).x + side * annotationHalfWidth(p) * path.normalAt(line, p).x,
    y: path.pointAt(line, p).y + side * annotationHalfWidth(p) * path.normalAt(line, p).y,
  };
}
export default defineSpanScene({
  id: "annotate-cursor", width: 1920, height: 1080, durationMs: 3600, editPreviewMs: 2800,
  variables: {
    width: numberVar(1920), height: numberVar(1080),
    title: stringVar("A stronger first cut"),
    subtitle: stringVar("A clear brief gives every scene a reason to stay."),
    metricValue: stringVar("82%"), metricLabel: stringVar("First-pass approval"),
    ink: colorVar("#D66B63"), background: colorVar("#FAFAFB"),
  },
  computeds: {
    scale: computed.number(() => Math.min(vars.width / 1920, vars.height / 1080)),
    clock: computed.number(() => time.loop(3600) * 3600),
    // Overshoot a full turn while opening outward, clear of the initial arc.
    // 320 short chords replace the visibly angular 37-point outline.
    loopPoints: computed.points(() => collection.map(collection.range(321), (index, progress) => ({
      x: annotationX(progress), y: annotationY(progress)
    }))),
    loopPath: computed.path(() => path.fromPoints(computed.loopPoints)),
    arrival: computed.number(() => Math.max(0, Math.min(1, computed.clock / 530))),
    arrive: computed.number(() => Math.pow(computed.arrival, 3) * (10 + computed.arrival * (-15 + 6 * computed.arrival))),
    contact: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 530) / 170))),
    touch: computed.number(() => Math.pow(computed.contact, 3) * (10 + computed.contact * (-15 + 6 * computed.contact))),
    draw: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 700) / 1380))),
    drawEase: computed.number(() => Math.pow(computed.draw, 3) * (10 + computed.draw * (-15 + 6 * computed.draw))),
    nib: computed.vector(() => path.pointAt(computed.loopPath, computed.drawEase)),
    // The fractional frontier follows the nib; width is authored geometry so
    // tapering does not depend on renderer-specific per-point pressure handling.
    inkSamples: computed.number(() => Math.max(2, Math.ceil(computed.drawEase * 320) + 1)),
    inkOutline: computed.points(() => collection.map(collection.range(computed.inkSamples * 2), (index, progress) =>
      annotationRibbonPoint(computed.loopPath,
        Math.min((index < computed.inkSamples ? index : computed.inkSamples * 2 - 1 - index) / 320, computed.drawEase),
        index < computed.inkSamples ? 1 : -1))),
    lift: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 2080) / 400))),
    aside: computed.number(() => Math.pow(computed.lift, 3) * (10 + computed.lift * (-15 + 6 * computed.lift))),
    departure: computed.number(() => Math.max(0, Math.min(1, (computed.clock - 3200) / 400))),
    depart: computed.number(() => Math.pow(computed.departure, 3) * (10 + computed.departure * (-15 + 6 * computed.departure))),
    pointerX: computed.number(() => computed.clock < 530 ? 2040 * Math.pow(1 - computed.arrive, 2) + 2580 * (1 - computed.arrive) * computed.arrive + 836 * computed.arrive * computed.arrive : computed.clock < 700 ? 836 : computed.clock < 2080 ? computed.nib.x : computed.clock < 2480 ? computed.nib.x * Math.pow(1 - computed.aside, 3) + 3090 * Math.pow(1 - computed.aside, 2) * computed.aside + 3585 * (1 - computed.aside) * computed.aside * computed.aside + 1238 * Math.pow(computed.aside, 3) : 1238 * Math.pow(1 - computed.depart, 2) + 3150 * (1 - computed.depart) * computed.depart + 2040 * computed.depart * computed.depart),
    pointerY: computed.number(() => computed.clock < 530 ? 1190 * Math.pow(1 - computed.arrive, 2) + 1320 * (1 - computed.arrive) * computed.arrive + 490 * computed.arrive * computed.arrive : computed.clock < 700 ? 490 + 24 * computed.touch : computed.clock < 2080 ? computed.nib.y : computed.clock < 2480 ? computed.nib.y * Math.pow(1 - computed.aside, 3) + 1254 * Math.pow(1 - computed.aside, 2) * computed.aside + 1914 * (1 - computed.aside) * computed.aside * computed.aside + 742 * Math.pow(computed.aside, 3) : 742 * Math.pow(1 - computed.depart, 2) + 1550 * (1 - computed.depart) * computed.depart + 1160 * computed.depart * computed.depart),
    penAngle: computed.number(() => computed.clock < 700 ? -47 - 4 * computed.touch : computed.clock < 2080 ? -51 + 2 * Math.sin(computed.drawEase * Math.PI * 2) : -51 + 7 * computed.aside),
  },
  render({vars, computed}) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="letterbox" x={(vars.width - 1920 * computed.scale) / 2} y={(vars.height - 1080 * computed.scale) / 2} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Text id="workspace" text="Mont / Production notes" x={432} y={96} width={700} height={30} fontFamily="Inter" fontWeight={500} size={19} lineHeight={1.2} fit="none" color="#848690" />
        <Rect id="document" x={420} y={152} width={1080} height={782} radius={18} fill="#FFFFFF" stroke="#E5E6EA" strokeWidth={1} shadowColor="#252631" shadowOffsetY={12} shadowBlur={35} shadowOpacity={0.045} />
        <Rect id="toolbar-divider" x={421} y={225} width={1078} height={1} fill="#ECEDEF" />
        <Rect id="document-icon" x={451} y={177} width={22} height={25} radius={4} fill="#F5F5F8" stroke="#C8CAD3" strokeWidth={1.4} />
        <Rect id="document-icon-line" x={457} y={185} width={10} height={1.5} fill="#9A9DAA" />
        <Rect id="document-icon-line-two" x={457} y={191} width={8} height={1.5} fill="#9A9DAA" />
        <Text id="file-name" text="Rough cut review" x={489} y={179} width={420} height={26} fontFamily="Inter" fontWeight={500} size={19} lineHeight={1.2} fit="none" color="#60636F" />
        <Rect id="review-chip" x={1321} y={174} width={140} height={32} radius={7} fill="#F1F5F2" />
        <Text id="review-status" text="Ready to share" x={1321} y={182} width={140} height={20} align="center" fontFamily="Inter" fontWeight={500} size={14} lineHeight={1.2} fit="none" color="#718778" />
        <Text id="eyebrow" text="THE EDIT, IN FOCUS" x={540} y={286} width={750} height={28} fontFamily="Inter" fontWeight={600} size={16} letterSpacing={1.7} lineHeight={1.2} fit="none" color="#8D909B" />
        <Text id="heading" text={vars.title} x={540} y={334} width={850} height={68} fontFamily="Inter" fontWeight={600} size={48} letterSpacing={-1.5} lineHeight={1.2} fit="none" color="#30323A" />
        <Text id="description" text={vars.subtitle} x={540} y={414} width={850} height={36} fontFamily="Inter" fontWeight={400} size={23} lineHeight={1.2} fit="none" color="#838690" />
        <Text id="turnaround-value" text="4.8h" x={540} y={520} width={240} height={76} align="center" fontFamily="Inter" fontWeight={500} size={58} letterSpacing={-2.2} lineHeight={1.2} fit="none" color="#535661" />
        <Text id="turnaround-label" text="From brief to rough cut" x={540} y={606} width={240} height={28} align="center" fontFamily="Inter" fontWeight={400} size={18} lineHeight={1.2} fit="none" color="#8A8D98" />
        <Text id="highlight-value" text={vars.metricValue} x={848} y={514} width={240} height={82} align="center" fontFamily="Inter" fontWeight={600} size={64} letterSpacing={-2.4} lineHeight={1.2} fit="none" color="#343640" />
        <Text id="highlight-label" text={vars.metricLabel} x={836} y={606} width={264} height={28} align="center" fontFamily="Inter" fontWeight={400} size={18} lineHeight={1.2} fit="none" color="#737783" />
        <Text id="scenes-value" text="12" x={1150} y={520} width={230} height={76} align="center" fontFamily="Inter" fontWeight={500} size={58} letterSpacing={-2.2} lineHeight={1.2} fit="none" color="#535661" />
        <Text id="scenes-label" text="Scenes, one clear story" x={1150} y={606} width={230} height={28} align="center" fontFamily="Inter" fontWeight={400} size={18} lineHeight={1.2} fit="none" color="#8A8D98" />
        <Rect id="note-divider" x={540} y={740} width={840} height={1} fill="#ECEDEF" />
        <Text id="note" text="Keep the story moving. Let the details support it." x={540} y={781} width={840} height={34} fontFamily="Inter" fontWeight={400} size={22} lineHeight={1.2} fit="none" color="#8A8D98" />
        <Text id="document-footer" text="FIELD NOTES  /  01" x={540} y={855} width={600} height={25} fontFamily="Inter" fontWeight={500} size={13} letterSpacing={1.8} lineHeight={1.2} fit="none" color="#B1B3BD" />
        <Path id="annotation-ink" points={computed.inkOutline} closed={true} fillRule="nonzero" width={1920} height={1080} fill={vars.ink} opacity={computed.clock >= 700 ? 1 : 0} />
        {/* Native marker, not an arrow cursor. Local (0,0) is the physical nib. */}
        <Group id="marker-position" x={computed.pointerX} y={computed.pointerY} width={0} height={0} pivotX={0} pivotY={0}>
          <Group id="marker-body" width={0} height={0} pivotX={0} pivotY={0} rotation={computed.penAngle}>
            <Path id="marker-nib" x={0} y={-7} width={19} height={14} viewBox="0 -7 19 14" d="M0 0 L19 -7 L19 7 Z" fill="#35353D" />
            <Path id="marker-ink-tip" x={0} y={-4} width={16} height={8} viewBox="0 -4 16 8" d="M0 0 L16 -4 L16 4 Z" fill={vars.ink} />
            <Rect id="marker-collar" x={18} y={-9} width={16} height={18} radius={2} fill="#35353D" />
            <Rect id="marker-barrel" x={32} y={-11} width={76} height={22} radius={5} fill="#FFFFFF" stroke="#3F4048" strokeWidth={1.5} shadowColor="#24252C" shadowOffsetY={3} shadowBlur={5} shadowOpacity={0.12} />
            <Rect id="marker-band" x={41} y={-10} width={9} height={20} fill={vars.ink} />
            <Rect id="marker-barrel-glint" x={57} y={-6} width={37} height={2} radius={1} fill="#F0F0F3" />
            <Rect id="marker-cap" x={105} y={-11.5} width={19} height={23} radius={6} fill="#35353D" />
          </Group>
        </Group>
      </Group>
    </Scene>;
  },
});
