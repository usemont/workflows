import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import vm from "node:vm";

function helpers(name) {
  const source = readFileSync(new URL(`${name}/scene.tsx`, import.meta.url), "utf8");
  const context = vm.createContext({ Math });
  vm.runInContext(source.slice(0, source.indexOf("export default")), context);
  return context;
}
const circle = helpers("circle-carousel");
const orbit = helpers("orbit-carousel");
const path = helpers("path-carousel");
const close = (a, b, message) => assert.ok(Math.abs(a - b) < 1e-7, message);

// Choreography must stop the gallery before featuring art and return it before resuming.
for (const scene of [circle, orbit, path]) {
  const velocity = ms => (scene.carouselTravel(ms + 0.1) - scene.carouselTravel(ms - 0.1)) / 0.2;
  const featureVelocity = ms => (scene.carouselFeature(ms + 0.1) - scene.carouselFeature(ms - 0.1)) / 0.2;
  close(scene.carouselTravel(0), 0, "Gallery must start at its loop origin");
  close(scene.carouselTravel(14000), 1, "Gallery must complete exactly one revolution");
  close(velocity(0), velocity(14000), "Loop cruise velocity must match");
  let previousTravel = -1;
  for (let ms = 0; ms <= 14000; ms += 10) {
    const travel = scene.carouselTravel(ms);
    const feature = scene.carouselFeature(ms);
    assert.ok(Number.isFinite(travel) && travel >= previousTravel, "Gallery must not reverse");
    assert.ok(feature >= 0 && feature <= 1, "Feature zoom must not overshoot");
    if (ms >= 3800 && ms <= 7750) {
      close(travel, 3400 / 9200, "Gallery must remain still during the complete feature");
    }
    if (ms <= 4000 || ms >= 7500) assert.equal(feature, 0, "Feature must return before travel resumes");
    if (ms >= 4900 && ms <= 6700) assert.equal(feature, 1, "Featured artwork needs an uninterrupted hold");
    previousTravel = travel;
  }
  for (const ms of [3800, 7750]) {
    assert.ok(Math.abs(velocity(ms)) < 1e-10, "Gallery stop/start must have zero velocity");
  }
  for (const ms of [4000, 4900, 6700, 7500]) {
    assert.ok(Math.abs(featureVelocity(ms)) < 1e-10, "Feature transitions must settle without a velocity jump");
  }
  for (const ms of [3000, 3800, 7750, 8650]) {
    assert.ok(Math.abs(velocity(ms - 0.1) - velocity(ms + 0.1)) < 1e-9, "Travel easing must join continuously");
  }
  for (let ms = 3100; ms < 3800; ms += 100) {
    assert.ok(velocity(ms) < velocity(ms - 100), "Arrival must progressively decelerate");
  }
  for (let ms = 7850; ms < 8650; ms += 100) {
    assert.ok(velocity(ms) > velocity(ms - 100), "Departure must progressively accelerate");
  }
  for (let ms = 4010; ms <= 4900; ms += 10) {
    assert.ok(scene.carouselFeature(ms) > scene.carouselFeature(ms - 10), "Feature must expand smoothly");
  }
  for (let ms = 6710; ms <= 7500; ms += 10) {
    assert.ok(scene.carouselFeature(ms) < scene.carouselFeature(ms - 10), "Feature must shrink smoothly");
  }
}

// A seamless revolution must preserve both pose and velocity, not merely fade at the seam.
for (let index = 0; index < 10; index++) {
  for (const [evaluate, period] of [
    [t => circle.wheelDepth(index, t), 1],
    [t => circle.wheelScale(index, t), 1],
    [t => orbit.orbitHorizontal(index, t), 2 * Math.PI],
    [t => orbit.orbitVertical(index, t), 2 * Math.PI],
    [t => orbit.orbitDepth(index, t), 2 * Math.PI],
  ]) {
    close(evaluate(0), evaluate(period), `Loop pose discontinuity: poster ${index}`);
    const h = 1e-4;
    close((evaluate(h) - evaluate(-h)) / (2 * h),
      (evaluate(period + h) - evaluate(period - h)) / (2 * h),
      `Loop velocity discontinuity: poster ${index}`);
  }
  for (let sample = 0; sample <= 1000; sample++) {
    const progress = sample / 1000;
    assert.ok(circle.wheelScale(index, progress) > 0, "Wheel must never flip its artwork");
    const phase = progress * 2 * Math.PI;
    const [x, y, z] = [orbit.orbitHorizontal(index, phase), orbit.orbitVertical(index, phase), orbit.orbitDepth(index, phase)];
    close(x * x + y * y + z * z, 1, "Orbit must preserve its spherical distribution");
    assert.ok(1 - 0.26 * z > 0, "Poster must remain in front of the perspective camera");
    const p = path.pathPhase(index, progress);
    const opacity = path.pathVisibility(p);
    assert.ok(Number.isFinite(path.pathTilt(p)) && path.pathScale(p) > 0);
    assert.ok(opacity >= 0 && opacity <= 1, "Path opacity must remain normalized");
    if (p < 0.19 || p > 0.79) assert.equal(opacity, 0, "Path reset must be invisible");
    close(opacity, path.pathVisibility(path.pathPhase(index, progress + 1)), "Path loop opacity mismatch");
    close(path.pathX(p), path.pathX(path.pathPhase(index, progress + 1)), "Path loop position mismatch");
  }
}

// An S-shaped path rises monotonically, flattening on either side of its inflection.
const step = 1e-4;
const slope = p => (path.pathY(p + step) - path.pathY(p - step)) / (2 * step);
const curvature = p => path.pathY(p + step) - 2 * path.pathY(p) + path.pathY(p - step);
for (let sample = 20; sample <= 80; sample++) {
  const p = sample / 100;
  assert.ok(slope(p) < 0, "The sigmoid must rise throughout its visible section");
  assert.ok(path.pathTilt(p) < 0, "Poster tilt must follow the rising path");
}
assert.ok(curvature(0.35) < 0 && curvature(0.65) > 0, "The path must change curvature across its center");
assert.ok(Math.abs(slope(0.2)) < Math.abs(slope(0.5)) && Math.abs(slope(0.8)) < Math.abs(slope(0.5)),
  "The sigmoid must flatten toward both ends");
// The path's hidden reset must also have zero fade velocity on either side.
for (const p of [1e-5, 1 - 1e-5]) {
  assert.equal(path.pathVisibility(p), 0);
}
console.log("Carousel geometry, hidden resets, eased travel, stationary feature hold and seamless loop motion passed.");
