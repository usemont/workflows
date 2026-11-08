import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import vm from "node:vm";

const source = readFileSync(new URL("scene.tsx", import.meta.url), "utf8");
const body = source.split("  computeds: {\n")[1].split("\n  },\n  render")[0];
const expressions = new Map([...body.matchAll(/^    (\w+): computed\.(?:number|string)\(\(\) => (.+)\),$/gm)]
  .map(([, name, expression]) => [name, new vm.Script(expression)]));

// Test motion with several shaped-text extents; native rendering verifies the actual font metrics.
function at(ms, regularWidth = 360) {
  const values = { spaceWidth: 8, prefixWidth: 82, regularWidth, boldWidth: regularWidth * 1.08 };
  const context = vm.createContext({ Math, vars: { width: 1920, height: 1080 }, time: { loop: duration => ms / duration } });
  context.computed = new Proxy(values, { get(target, name) {
    if (!(name in target)) target[name] = expressions.get(name).runInContext(context);
    return target[name];
  } });
  return context.computed;
}

for (const width of [240, 360, 520]) {
  let previous = 0;
  for (let ms = 900; ms <= 2020; ms += 10) {
    const c = at(ms, width);
    assert.ok(c.dragEase >= previous && c.dragEase <= 1, "Selection must never reverse or overshoot");
    assert.ok(Math.abs(c.pointerX - c.startX - width * c.dragEase) < 1e-6, "I-beam must track the selection frontier");
    assert.equal(c.pointerY, 525);
    previous = c.dragEase;
  }
  for (const ms of [620, 840, 900, 2020, 2160, 2460, 2500, 3030, 3380, 3940]) {
    const a = at(ms - 0.5, width), b = at(ms + 0.5, width);
    assert.ok(Math.hypot(a.pointerX - b.pointerX, a.pointerY - b.pointerY) < 0.02, `Cursor discontinuity at ${ms} ms`);
  }
  for (const ms of [3030, 3080, 3150, 3320]) {
    const c = at(ms, width);
    assert.equal(c.pointerX, c.boldX, "Cursor must stay on Bold throughout the click");
    assert.equal(c.pointerY, 442);
  }
  assert.ok(at(3150, width).pointerScale < 0.97, "Click must visibly compress");
  assert.equal(at(3320, width).pointerScale, 1, "Click must release before departure");
  for (let ms = 0; ms < 5600; ms += 1000 / 30) {
    const c = at(ms, width);
    assert.ok(c.toolbarEase >= 0 && c.toolbarEase <= 1, "Toolbar opacity must remain valid");
    assert.ok(c.pointerStrain >= 1 && c.pointerStrain <= 1.025, "Cursor deformation must stay subtle");
    assert.ok(Number.isFinite(c.pointerX) && Number.isFinite(c.pointerY));
  }
}
console.log("Monotonic selection/frontier contact, continuous travel, exact Bold dwell, press/release, and subtle deformation verified.");
