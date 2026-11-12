#!/usr/bin/env python3
"""Check personalized inputs and connected camera/card continuity."""
import json
from create import HERE, run, validate

rows = json.loads((HERE / "inputs.example.json").read_text())
for row in rows:
    validate(row)
for key, value in (("videos", -1), ("minutes", 1.5), ("year", True), ("name", ""), ("favouriteLayout", "Unknown"), ("accent", "orange")):
    try:
        validate({**rows[0], key: value})
    except ValueError:
        pass
    else:
        raise AssertionError(f"Accepted invalid {key}")
helpers = (HERE / "scene.tsx").read_text().split("export default", 1)[0]
run("node", "-e", helpers + """
const assert = require('node:assert/strict');
for (const value of [0,1,48,9999,999999]) {
  assert.equal(count(0,2550,4200,value),0);
  assert.equal(count(4200,2550,4200,value),value);
}
for(const value of [0,9,10,99,100,999,1000,9999,10000,99999,100000,999999]) {
  const rendered=Array.from({length:digits(value)},(_,index)=>glyph(Math.floor(value/Math.pow(10,digits(value)-1-index))%10)).join('');
  assert.equal(rendered,String(value));
}
for (let t=0;t<19000;t+=5) {
  assert(Math.abs(camera(t+5)-camera(t))<8);
  assert(Math.abs(zoom(t+5)-zoom(t))<.004);
  assert(zoom(t)>=.45-1e-8 && zoom(t)<=1);
}
assert.equal(camera(19000),1200);
for(let i=0;i<6;i++) {
  assert.equal(cardX(i,5000),-340+(i%3)*330);
  assert(Math.abs(cardY(i,5000)-(-65+Math.floor(i/3)*163))<1e-8);
  for(let t=0;t<4000;t+=5) assert(Math.abs(cardX(i,t+5)-cardX(i,t))<13);
}
""")
print("Input validation, counter boundaries, camera continuity and card landing passed")
