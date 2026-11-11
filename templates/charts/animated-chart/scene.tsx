// Original Mont chart geometry. A shared sampled contour preserves identity across families.
function clamp(p) { return Math.max(0, Math.min(1, p)); }
function ease(p) { return Math.pow(clamp(p), 3) * (10 + clamp(p) * (-15 + 6 * clamp(p))); }
function digitCount(value) { return value >= 1000000000 ? 10 : value >= 100000000 ? 9 : value >= 10000000 ? 8 : value >= 1000000 ? 7 : value >= 100000 ? 6 : value >= 10000 ? 5 : value >= 1000 ? 4 : value >= 100 ? 3 : value >= 10 ? 2 : 1; }
function mix(a, b, p) { return a + (b - a) * p; }
function pointMix(a, b, p) { return {x: mix(a.x, b.x, p), y: mix(a.y, b.y, p)}; }
function modeIndex(mode) { return mode === "bar" ? 0 : mode === "line" ? 1 : mode === "area" ? 2 : mode === "doughnut" ? 3 : mode === "pie" ? 4 : mode === "polar" ? 5 : mode === "radar" ? 6 : mode === "scatter" ? 7 : mode === "bubble" ? 8 : 9; }
function stage(t, mode) { return mode === "showcase" ? Math.min(9, Math.max(0, (t - 2400) / 1700)) : modeIndex(mode); }
function phase(t, mode) { return mode === "showcase" ? ease((stage(t, mode) - Math.floor(stage(t, mode))) * 17 / 14) : 0; }
function family(k) { return k === 0 ? "Bar" : k === 1 ? "Line" : k === 2 ? "Area" : k === 3 ? "Doughnut" : k === 4 ? "Pie" : k === 5 ? "Polar area" : k === 6 ? "Radar" : k === 7 ? "Scatter" : k === 8 ? "Bubble" : "Mixed"; }
function fold(p) { return p <= 0.5 ? p * 2 : (1 - p) * 2; }
function angle(d, p) { return -Math.PI / 2 + 2 * Math.PI * (d.start + d.share * fold(p)); }
function polarPoint(a, r) { return {x: 1260 + Math.cos(a) * r, y: 574 + Math.sin(a) * r}; }
function vertex(d, n) { return polarPoint(-Math.PI / 2 + d.index * 2 * Math.PI / n, 258 * d.normalized); }
function nextVertex(d, n) { return polarPoint(-Math.PI / 2 + (d.index + 1) * 2 * Math.PI / n, 258 * d.next); }
function barCenter(d, n) { return {x: 754 + (d.index + 0.5) * 900 / n, y: 816 - 231 * d.normalized}; }
function plotCenter(d, n) { return {x: 754 + (d.index + 0.5) * 900 / n, y: 816 - 462 * d.normalized}; }
function scatterCenter(d) { return {x: 754 + 900 * d.hours / 10, y: 816 - 462 * d.normalized}; }
function circlePoint(center, p, radius) { return {x: center.x - radius * Math.cos(p * Math.PI * 2), y: center.y - radius * Math.sin(p * Math.PI * 2)}; }
function boxRadius(p, width, height) { return Math.min(width / Math.max(0.000001, Math.abs(Math.cos(p * Math.PI * 2))), height / Math.max(0.000001, Math.abs(Math.sin(p * Math.PI * 2)))); }
function barPoint(d, p, n) { return circlePoint(barCenter(d, n), p, boxRadius(p, 450 / n - 16, 231 * d.normalized)); }
function contour(k, d, p, n) {
  return k === 0 || k === 9 ? barPoint(d, p, n)
    : k === 1 ? circlePoint(plotCenter(d, n), p, 9) : k === 2 ? {
    x: 754 + (d.index + fold(p)) * 900 / n,
    y: p <= 0.5 ? 816 - 462 * (fold(p) < 0.5 ? mix(d.left, d.normalized, fold(p) * 2) : mix(d.normalized, d.right, fold(p) * 2 - 1)) : 816
  } : k === 3 || k === 4 || k === 5 ? polarPoint(k === 5 ? -Math.PI / 2 + 2 * Math.PI * (d.index + fold(p)) / n : angle(d, p), p <= 0.5 ? (k === 5 ? 265 * Math.sqrt(d.normalized) : 250) : (k === 3 ? 142 : 0))
    : k === 6 ? (p <= 0.5 ? pointMix(vertex(d, n), nextVertex(d, n), fold(p)) : polarPoint(0, 0))
    : circlePoint(scatterCenter(d), p, k === 8 ? 13 * Math.sqrt(d.team) : 10);
}
function barToDot(d, p, n, blend) { return pointMix(pointMix(contour(0, d, p, n), circlePoint(barCenter(d, n), p, 9), ease(blend / 0.55)), contour(1, d, p, n), ease((blend - 0.55) / 0.45)); }
function radialPoint(k, d, p, n, blend) { return polarPoint(mix(angle(d, p), k === 4 ? -Math.PI / 2 + 2 * Math.PI * (d.index + fold(p)) / n : angle(d, p), blend), p <= 0.5 ? mix(250, k === 4 ? 265 * Math.sqrt(d.normalized) : 250, blend) : mix(k === 3 ? 142 : 0, 0, blend)); }
function radarCenter(d, n) { return {x: (vertex(d, n).x + nextVertex(d, n).x + 1260) / 3, y: (vertex(d, n).y + nextVertex(d, n).y + 574) / 3}; }
function radarToDot(d, p, n, blend) { return pointMix(pointMix(contour(6, d, p, n), circlePoint(radarCenter(d, n), p, 10), ease(blend / 0.5)), contour(7, d, p, n), ease((blend - 0.5) / 0.5)); }
function bubbleToBar(d, p, n, blend) { return pointMix(pointMix(contour(8, d, p, n), circlePoint(scatterCenter(d), p, 10), ease(blend / 0.2)), pointMix(circlePoint(barCenter(d, n), p, 10), contour(9, d, p, n), ease((blend - 0.65) / 0.35)), ease((blend - 0.2) / 0.45)); }
function morphPoint(k, d, p, n, blend) { return k === 0 ? barToDot(d, p, n, blend) : k === 3 || k === 4 ? radialPoint(k, d, p, n, blend) : k === 6 ? radarToDot(d, p, n, blend) : k === 8 ? bubbleToBar(d, p, n, blend) : pointMix(contour(k, d, p, n), contour(Math.min(9, k + 1), d, p, n), blend); }
function chartPoint(k, d, p, n, blend, t) { return pointMix({x: plotCenter(d, n).x, y: 816}, morphPoint(k, d, p, n, blend), ease((t - d.index * 65) / 1100)); }
function weight(k, target, blend) { return mix(k === target ? 1 : 0, Math.min(9, k + 1) === target ? 1 : 0, blend); }
function cartesian(k, blend) { return mix(k < 3 || k > 6 ? 1 : 0, Math.min(9, k + 1) < 3 || k + 1 > 6 ? 1 : 0, blend); }
function inkColor(i, a, b, c, d, e, f) { return i % 6 === 0 ? a : i % 6 === 1 ? b : i % 6 === 2 ? c : i % 6 === 3 ? d : i % 6 === 4 ? e : f; }
function outlinePoint(k, d, n, blend) { return k === 0 ? pointMix(barCenter(d, n), plotCenter(d, n), ease((blend - 0.55) / 0.45)) : {x: plotCenter(d, n).x, y: 816 - 462 * mix(d.normalized, d.target, k === 8 ? ease((blend - 0.65) / 0.35) : k === 9 ? 1 : 0)}; }
function cartesianLine(rows, k, p, blend) { return pointMix(outlinePoint(k, rows[Math.min(rows.length - 1, Math.floor(p * (rows.length - 1)))], rows.length, blend), outlinePoint(k, rows[Math.min(rows.length - 1, Math.floor(p * (rows.length - 1)) + 1)], rows.length, blend), p * (rows.length - 1) - Math.floor(p * (rows.length - 1))); }
function areaEdge(rows, p) { return contour(2, rows[Math.min(rows.length - 1, Math.floor(p * rows.length))], (p * rows.length - Math.min(rows.length - 1, Math.floor(p * rows.length))) / 2, rows.length); }
function ringEdge(rows, p) { return contour(3, rows[Math.min(rows.length - 1, Math.floor(p * rows.length))], (p * rows.length - Math.min(rows.length - 1, Math.floor(p * rows.length))) / 2, rows.length); }
function linePoint(rows, k, p, blend) { return k === 1 ? pointMix(cartesianLine(rows, k, p, blend), areaEdge(rows, p), blend) : k === 2 ? pointMix(areaEdge(rows, p), ringEdge(rows, p), blend) : k === 3 || k === 4 ? ringEdge(rows, p) : cartesianLine(rows, k, p, blend); }
function lineVisibility(k, blend) { return k === 0 ? ease((blend - 0.55) / 0.45) : k === 1 || k === 9 ? 1 : k === 2 ? 1 - ease((blend - 0.65) / 0.35) : k === 8 ? ease((blend - 0.65) / 0.35) : 0; }
function roundGrid(p, radius, sides) { return polarPoint(-Math.PI / 2 + p * Math.PI * 2, radius * Math.cos(Math.PI / sides) / Math.cos((p * sides - Math.floor(p * sides) - 0.5) * Math.PI * 2 / sides)); }
export default defineSpanScene({
  id: "animated-chart", width: 1920, height: 1080, durationMs: 18000, editPreviewMs: 12300,
  variables: {
    width: numberVar(1920), height: numberVar(1080), mode: stringVar("showcase"),
    label: stringVar("Videos published"), value: numberVar(2840), delta: stringVar("+24.8%"), axisMax: numberVar(800),
    comparison: stringVar("vs. previous quarter"),
    background: colorVar("#F4F1E9"), surface: colorVar("#FFFEFA"), ink: colorVar("#28362F"), muted: colorVar("#7F877E"), grid: colorVar("#E7E8DD"),
    color1: colorVar("#E48059"), color2: colorVar("#E7B857"), color3: colorVar("#559B8D"), color4: colorVar("#708EBD"), color5: colorVar("#A08ABD"), color6: colorVar("#CE9293")
  },
  tokens: {series: [
    {"index":0,"label":"Studio","value":240,"normalized":0.3,"next":0.5125,"left":0.3,"right":0.40625,"share":0.0845070423,"start":0,"hours":2.2,"team":6,"target":0.371},
    {"index":1,"label":"Social","value":410,"normalized":0.5125,"next":0.4125,"left":0.40625,"right":0.4625,"share":0.1443661972,"start":0.0845070423,"hours":3.7,"team":11,"target":0.54525},
    {"index":2,"label":"Learning","value":330,"normalized":0.4125,"next":0.7375,"left":0.4625,"right":0.575,"share":0.1161971831,"start":0.2288732395,"hours":5.1,"team":9,"target":0.46325},
    {"index":3,"label":"Product","value":590,"normalized":0.7375,"next":0.5875,"left":0.575,"right":0.6625,"share":0.2077464789,"start":0.3450704225,"hours":6.3,"team":15,"target":0.72975},
    {"index":4,"label":"Events","value":470,"normalized":0.5875,"next":1,"left":0.6625,"right":0.79375,"share":0.1654929577,"start":0.5528169014,"hours":7.4,"team":8,"target":0.60675},
    {"index":5,"label":"Brand","value":800,"normalized":1,"next":0.3,"left":0.79375,"right":1,"share":0.2816901408,"start":0.7183098592,"hours":8.8,"team":20,"target":0.945}
  ], ticks: [0, 0.25, 0.5, 0.75, 1], tickLabels: ["0", "200", "400", "600", "800"], digits: ["0","1","2","3","4","5","6","7","8","9"], places: [0,1,2,3,4,5,6,7,8,9], rings: [0.333333, 0.666667, 1]},
  computeds: {
    geometry: computed.struct(() => ({
      contours: collection.map(tokens.series, (d, index) => collection.map(collection.range(129), (sample, p) => chartPoint(computed.index, d, p, tokens.series.length, computed.blend, computed.clock))),
      rings: collection.map(tokens.rings, (ring, index) => collection.map(collection.range(121), (sample, p) => pointMix(polarPoint(-Math.PI / 2 + p * Math.PI * 2, 265 * ring), roundGrid(p, 265 * ring, tokens.series.length), computed.radar)))
    })),
    scale: computed.number(() => Math.min(vars.width / 1920, vars.height / 1080)),
    clock: computed.number(() => time.ms),
    stage: computed.number(() => stage(computed.clock, vars.mode)),
    index: computed.number(() => Math.floor(computed.stage)),
    blend: computed.number(() => phase(computed.clock, vars.mode)),
    current: computed.number(() => computed.index + (computed.blend >= 0.5 ? 1 : 0)),
    heading: computed.string(() => family(Math.min(9, computed.current))),
    count: computed.number(() => Math.round(vars.value * (1 - Math.pow(1 - clamp(computed.clock / 1750), 4)))),
    countDigits: computed.number(() => digitCount(computed.count)),
    countSize: computed.number(() => Math.min(132, 470 / (digitCount(vars.value) * 0.63))),
    line: computed.points(() => collection.map(collection.range(129), (sample, p) => linePoint(tokens.series, computed.index, Math.min(p, ease(computed.clock / 1500)), computed.blend))),
    lineOpacity: computed.number(() => lineVisibility(computed.index, computed.blend)),
    axes: computed.number(() => cartesian(computed.index, computed.blend)),
    radial: computed.number(() => weight(computed.index, 5, computed.blend) + weight(computed.index, 6, computed.blend)),
    radar: computed.number(() => weight(computed.index, 6, computed.blend)),
    scatter: computed.number(() => weight(computed.index, 7, computed.blend) + weight(computed.index, 8, computed.blend))
  },
  render({vars, computed}) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="stage" x={(vars.width - 1920 * computed.scale) / 2} y={(vars.height - 1080 * computed.scale) / 2} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.scale} scaleY={computed.scale}>
        <Rect id="report" x={70} y={65} width={1780} height={950} radius={40} fill={vars.surface} shadowColor="#414D39" shadowBlur={46} shadowOffsetY={16} shadowOpacity={0.045} />
        <Text id="chart-family" x={1230} y={116} width={530} height={55} text={computed.heading} align="right" fontFamily="Inter" fontWeight={600} size={40} letterSpacing={-1.5} color={vars.ink} opacity={1 - 0.85 * Math.sin(Math.PI * computed.blend)} />
        <Rect id="header-rule" x={150} y={207} width={1620} height={1.5} fill={vars.grid} />
        <Text id="metric-label" x={150} y={297} width={530} height={52} text={vars.label} fontFamily="Inter" fontWeight={500} size={34} letterSpacing={-1} color={vars.ink} />
        <Group id="metric" x={150} y={366} width={0} height={0} pivotX={0} pivotY={0} scaleX={computed.countSize / 132} scaleY={computed.countSize / 132}>
          <Repeat items={tokens.places} render={(place, index) => (<Text id={"metric-digit-" + index} x={index * 83.16} width={90} height={153} text={tokens.digits[Math.floor(computed.count / Math.pow(10, Math.max(0, computed.countDigits - 1 - index))) % 10]} fontFamily="Inter" fontWeight={600} size={132} color={vars.ink} opacity={index < computed.countDigits ? 1 : 0} />)} />
        </Group>
        <Group id="delta" x={150} y={550 + 15 * (1 - ease((computed.clock - 1800) / 550))} width={0} height={0} pivotX={0} pivotY={0} opacity={ease((computed.clock - 1800) / 450)} scaleX={1 + 0.05 * Math.sin(clamp((computed.clock - 1800) / 550) * Math.PI)} scaleY={1 + 0.05 * Math.sin(clamp((computed.clock - 1800) / 550) * Math.PI)}>
          <Rect id="delta-bg" width={171} height={57} radius={28.5} fill="#EAF1E5" />
          <Text id="delta-value" x={0} y={11} width={171} height={35} text={vars.delta} align="center" fontFamily="Inter" fontWeight={600} size={26} color="#467353" />
          <Text id="delta-description" x={0} y={77} width={480} height={32} text={vars.comparison} fontFamily="Inter" size={23} color={vars.muted} />
        </Group>
        <Group id="cartesian-grid" width={0} height={0} pivotX={0} pivotY={0} opacity={computed.axes}>
          <Repeat items={tokens.ticks} render={(tick, index) => (<Rect id={"grid-" + index} x={738} y={816 - 462 * tick} width={950} height={1.5} fill={vars.grid} />)} />
          <Text id="x-axis" x={738} y={839} width={950} height={30} align="center" text={computed.scatter > 0.5 ? "Editing hours · 0–10" : "Production teams"} fontFamily="Inter" size={19} color={vars.muted} />
          <Repeat items={tokens.ticks} render={(tick, index) => (<Text id={"tick-" + index} x={653} y={802 - 462 * tick} width={70} height={28} align="right" text={tokens.tickLabels[index]} fontFamily="Inter" size={18} color={vars.muted} />)} />
        </Group>
        <Group id="radial-grid" width={0} height={0} pivotX={0} pivotY={0} opacity={computed.radial}>
          <Repeat items={tokens.rings} render={(ring, index) => (<Path id={"ring-" + index} width={1920} height={1080} points={computed.geometry.rings[index]} closed={true} fill="none" stroke={vars.grid} strokeWidth={2} />)} />
        </Group>
        <Repeat items={tokens.series} render={(d, index) => (<Path id={"datum-" + index} width={1920} height={1080} points={computed.geometry.contours[index]} closed={true} fill={inkColor(index, vars.color1, vars.color2, vars.color3, vars.color4, vars.color5, vars.color6)} stroke={vars.surface} strokeWidth={2} opacity={1 - 0.64 * weight(computed.index, 2, computed.blend) - 0.18 * computed.radar} />)} />
        <Path id="trend" width={1920} height={1080} points={computed.line} closed={false} fill="none" stroke={vars.ink} strokeWidth={5} opacity={computed.lineOpacity} />
        <Text id="mixed-legend" x={1130} y={280} width={550} height={30} text="Coloured bars: actual   /   Line: plan" align="right" fontFamily="Inter" size={20} color={vars.muted} opacity={weight(computed.index, 9, computed.blend)} />
        <Rect id="legend-rule" x={150} y={900} width={1620} height={1.5} fill={vars.grid} />
        <Repeat items={tokens.series} render={(d, index) => (<Group id={"legend-" + index} x={150 + index * 1620 / tokens.series.length} y={940} width={0} height={0} pivotX={0} pivotY={0}>
          <Rect id={"swatch-" + index} y={4} width={12} height={12} radius={6} fill={inkColor(index, vars.color1, vars.color2, vars.color3, vars.color4, vars.color5, vars.color6)} />
          <Text id={"label-" + index} x={24} width={1620 / tokens.series.length - 28} height={27} text={d.label} fontFamily="Inter" fontWeight={500} size={20} color={vars.ink} />
        </Group>)} />
      </Group>
    </Scene>;
  }
});
