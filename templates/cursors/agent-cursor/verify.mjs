import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import vm from "node:vm";

const source = readFileSync(new URL("scene.tsx", import.meta.url), "utf8");
const names = new Set([
  "clock", "entry", "enter", "first", "move1", "second", "move2", "leaving", "leave",
  "pointerX", "pointerY", "press1", "press2", "pointerScale", "format", "formatEase",
  "goldenRatio", "previewX", "previewY", "previewWidth", "previewHeight",
  "orbitX", "orbitY", "orbitSize", "discSize",
]);
const expressions = [...source.matchAll(/^    (\w+): computed\.number\(\(\) => (.+)\),$/gm)]
  .filter(([, name]) => names.has(name))
  .map(([, name, expression]) => [name, new vm.Script(expression)]);

function at(ms) {
  const context = vm.createContext({ Math, computed: {}, time: { loop: duration => ms / duration } });
  for (const [name, expression] of expressions) context.computed[name] = expression.runInContext(context);
  return context.computed;
}

function position(ms, x, y) {
  const c = at(ms);
  assert.ok(Math.abs(c.pointerX - x) < 1e-8 && Math.abs(c.pointerY - y) < 1e-8,
    `Incorrect hotspot at ${ms} ms`);
}

for (const [ms, x, y] of [[2000, 1166, 532], [3300, 1630, 670]]) {
  position(ms, x, y);
  assert.equal(at(ms).pointerScale, 0.94, "Missing exact click press");
}
for (const [start, end, x, y] of [[650, 1250, 840, 600], [1800, 2550, 1166, 532], [3100, 4300, 1630, 670]]) {
  for (let ms = start; ms <= end; ms += 10) position(ms, x, y);
}

// Each arc must progress toward its target on both axes: a hook or overshoot fails.
for (const [start, end, directionX, directionY] of [[0, 650, -1, -1], [1250, 1800, 1, -1], [2550, 3100, 1, 1], [4300, 4800, 1, 1]]) {
  let previous = at(start);
  for (let ms = start + 10; ms <= end; ms += 10) {
    const current = at(ms);
    assert.ok((current.pointerX - previous.pointerX) * directionX >= -1e-8, `Horizontal hook at ${ms}`);
    assert.ok((current.pointerY - previous.pointerY) * directionY >= -1e-8, `Vertical hook at ${ms}`);
    previous = current;
  }
}
position(0, 2040, 900);
position(4800, 2040, 940);

// Finite differences around every move/wait boundary catch corners or abrupt starts/stops.
for (const ms of [650, 1250, 1800, 2550, 3100, 4300]) {
  const step = 0.01;
  const before = at(ms - step), current = at(ms), after = at(ms + step);
  for (const axis of ["pointerX", "pointerY"]) {
    assert.ok(Math.abs((after[axis] - before[axis]) / (2 * step)) < 1e-6, `Nonzero stop velocity at ${ms}`);
    assert.ok(Math.abs((after[axis] - 2 * current[axis] + before[axis]) / (step * step)) < 1e-3,
      `Nonzero stop acceleration at ${ms}`);
  }
}
function geometry(id, ms) {
  const node = source.match(new RegExp(`<[^>]+id="${id}"[^>]*\\/>`));
  assert.ok(node, `Missing preview primitive ${id}`);
  const context = vm.createContext({ Math, computed: at(ms) });
  return Object.fromEntries([...node[0].matchAll(/\b(x|y|width|height)=\{([^}]+)\}/g)]
    .map(([, name, expression]) => [name, new vm.Script(expression).runInContext(context)]));
}
const phi = (1 + Math.sqrt(5)) / 2;
let previousDiameter = Infinity;
for (let ms = 1900; ms <= 2300; ms += 10) {
  const frame = geometry("preview-video", ms);
  const outer = geometry("preview-orbit", ms);
  const inner = geometry("preview-disc", ms);
  const cx = outer.x + outer.width / 2, cy = outer.y + outer.height / 2;
  assert.ok(Math.hypot(cx - inner.x - inner.width / 2, cy - inner.y - inner.height / 2) < 1e-8,
    `Preview rings must stay concentric during format change at ${ms} ms`);
  assert.ok(Math.abs((cx - frame.x) / frame.width - 1 / phi) < 1e-8,
    "Artwork center must stay on the golden-ratio column");
  assert.ok(Math.abs((cy - frame.y) / frame.height - 1 / phi ** 3) < 1e-8,
    "Artwork center must stay on the upper secondary golden-ratio row");
  assert.ok(Math.abs(outer.width / inner.width - phi) < 1e-8, "Ring diameters must follow the golden ratio");
  if (ms === 1900 || ms === 2300) {
    assert.ok(Math.abs(outer.width - Math.min(frame.width, frame.height) / phi ** 2) < 1e-8,
      "Settled artwork must follow the viewport's golden-ratio grid");
  }
  assert.ok(outer.width <= previousDiameter + 1e-8, "Artwork must not bulge while the preview narrows");
  previousDiameter = outer.width;
  for (const circle of [outer, inner]) {
    assert.equal(circle.width, circle.height, "Format changes must not squash the circles");
    assert.ok(circle.x >= frame.x && circle.y >= frame.y
      && circle.x + circle.width <= frame.x + frame.width
      && circle.y + circle.height <= frame.y + frame.height, "Artwork must stay inside the preview");
  }
}
console.log("Exact clicks, smooth cursor arcs, concentric preview rings, and golden-ratio layout through the format transition passed.");
