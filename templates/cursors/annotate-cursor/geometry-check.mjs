import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';

const source = readFileSync(new URL('./scene.tsx', import.meta.url), 'utf8');
const context = vm.createContext({ Math });
vm.runInContext(source.slice(source.indexOf('function annotationX'), source.indexOf('export default')), context);
const { annotationX: x, annotationY: y, annotationPressure: pressure } = context;
const points = Array.from({ length: 321 }, (_, i) => ({ x: x(i / 320), y: y(i / 320) }));
const close = (a, b) => assert.ok(Math.abs(a - b) < 1e-8, `${a} != ${b}`);
close(x(0), 836); close(y(0), 514);
const gap = Math.hypot(x(1) - x(0), y(1) - y(0));
assert.ok(gap >= 28 && gap <= 44, `leave a small visible gap, got ${gap}px`);
const center = { x: 836 - 163 * Math.cos(2.45), y: 514 + 106 * Math.sin(2.45) };
const angles = points.map(p => Math.atan2((p.y - center.y) / 106, (p.x - center.x) / 163));
let sweep = 0;
for (let i = 1; i < angles.length; i++) {
  const delta = angles[i] - angles[i - 1];
  sweep += Math.atan2(Math.sin(delta), Math.cos(delta));
}
assert.ok(sweep > 2 * Math.PI + 0.15 && sweep < 2 * Math.PI + 0.5, 'the tail must overshoot a full turn, not stop short');
for (let i = 0; i < 320; i++) {
  const a = points[i], b = points[i + 1], p = (i + 0.5) / 320;
  assert.ok(Math.hypot(b.x - a.x, b.y - a.y) < 5, 'outline chords must stay short');
  assert.ok(Math.hypot(x(p) - (a.x + b.x) / 2, y(p) - (a.y + b.y) / 2) < 0.08, 'chord error must stay subpixel');
}
let crossing = false;
let clearance = Infinity;
for (let i = 0; i < 40; i++) for (let j = 270; j < 320; j++) {
  const a = points[i], b = points[i + 1], c = points[j], d = points[j + 1];
  clearance = Math.min(clearance, Math.hypot(a.x - c.x, a.y - c.y));
  const rx = b.x - a.x, ry = b.y - a.y, sx = d.x - c.x, sy = d.y - c.y;
  const determinant = rx * sy - ry * sx, dx = c.x - a.x, dy = c.y - a.y;
  const t = (dx * sy - dy * sx) / determinant, u = (dx * ry - dy * rx) / determinant;
  crossing ||= t > 0 && t < 1 && u > 0 && u < 1;
}
assert.ok(!crossing, 'the finishing arc must not reconnect or cross the initial contact arc');
assert.ok(clearance > 12, `overlapping arcs need visible ink clearance, got ${clearance}px`);
close(pressure(0), 0); close(pressure(1), 0);
assert.ok(pressure(0.0125) > 0.38 && pressure(0.0125) < 0.42, 'short eased entrance');
assert.ok(pressure(0.9825) > 0.38 && pressure(0.9825) < 0.42, 'short eased release');
for (let i = 25; i <= 965; i++) assert.ok(pressure(i / 1000) >= 0.765 - 1e-12 && pressure(i / 1000) <= 0.835 + 1e-12, 'quiet body pressure');
console.log('Smooth overshooting annotation, separated overlapping arcs, and compact pressure tapers passed.');
