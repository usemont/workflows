#!/usr/bin/env python3
"""Create the native 18s recording-layouts launch example, optionally render it."""
import argparse
import json
import math
from pathlib import Path
import uuid
from create import run

HERE = Path(__file__).resolve().parent
MEDIA = HERE.parents[2] / "assets/template-media"


def cursor_expressions(samples):
    # Keep recorded click times, but spend the extra preview time reading, not travelling.
    expressions = {axis: str(samples[0]["values"]["cursor" + axis]) for axis in ("X", "Y")}
    presses = []
    previous = samples[0]["values"]
    for i, sample in enumerate(samples):
        values = sample["values"]
        if not values["cursorPressed"] or (i and samples[i - 1]["values"]["cursorPressed"]):
            continue
        click = sample["timeMs"] * 2.25
        dx, dy = values["cursorX"] - previous["cursorX"], values["cursorY"] - previous["cursorY"]
        distance = math.hypot(dx, dy)
        end = click - 150
        start = end - min(680, 360 + distance * .4)
        for axis, delta, bend in (("X", dx, -dy), ("Y", dy, dx)):
            offset = bend / max(distance, 1) * min(12, distance * .04)
            expressions[axis] += f"+({delta:.6f})*travel(computed.t,{start:.6f},{end:.6f})+({offset:.6f})*arc(computed.t,{start:.6f},{end:.6f})"
        release = next(s["timeMs"] * 2.25 for s in samples[i + 1:] if not s["values"]["cursorPressed"])
        presses.append(f"(computed.t>={click} && computed.t<{release}?1:0)")
        previous = values
    return {**expressions, "PRESSED": "+".join(presses) or "0"}


def create(workspace):
    workspace = Path(workspace).resolve()
    run("mont", "init", str(workspace), "--json")
    bindings = {}
    for key, name in (("screen", "desktop.mp4"), ("camera", "presenter-explaining.mp4")):
        result = json.loads(run("mont", "import", str(MEDIA / "recorders" / name), "-C", str(workspace), "--json"))["result"]
        bindings[key] = result["sourceId"]
    resources = json.loads((workspace / "project.json").read_text())["resources"]
    image_assets = json.loads((HERE / "recording-assets.json").read_text())
    samples = json.loads((HERE.parents[1] / "tools/recorder-demo/storyboard.json").read_text())["samples"]
    source = (HERE / "recording-layouts.tsx").read_text()
    for name, expression in cursor_expressions(samples).items():
        source = source.replace("CURSOR_" + name, expression)

    def dispatch(command):
        run("mont", "dispatch", "-C", str(workspace), "-", "--json", stdin=json.dumps({"commandId": str(uuid.uuid4()), "command": command}).encode())

    for key, value in (("sceneWidth", 1920), ("sceneHeight", 1080), ("programmaticSpans", []), ("resources", resources)):
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "recording-layouts-launch",
        "placement": {"startMs": 0, "durationMs": 18000, "trackId": "launch", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "sourceBindings": bindings, "state": {
            "sourceTsx": source, "variables": {"screenSource": "host-frame:screen", "cameraSource": "host-frame:camera"}, "imageAssets": image_assets,
            "frame": {"x": 0, "y": 0, "width": 1920, "height": 1080}}},
        "interaction": {"selectable": True}, "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    return workspace


def check_motion():
    helpers = (HERE / "recording-layouts.tsx").read_text().split("export default", 1)[0]
    run("node", "-e", helpers + """
const assert=require('node:assert/strict');
for(const curve of [ease,arrive]) {
  for(const [a,b] of [[4600,5700],[8600,9750],[12300,13400]]) {
    assert.equal(curve(a-1,a,b),0); assert.equal(curve(a,a,b),0);
    assert.equal(curve(b,a,b),1); assert.equal(curve(b+1,a,b),1);
    assert(Math.abs(curve(b,a,b)-curve(b-1,a,b))<1e-5);
    for(let t=a;t<b;t+=5) {
      assert(curve(t,a,b)>=0 && curve(t,a,b)<=1);
      assert(curve(t+5,a,b)>=curve(t,a,b));
    }
  }
}
// The acceleration/deceleration handoff must not introduce a position or velocity jump.
const h=.00001, left=(ease(.28,0,1)-ease(.28-h,0,1))/h, right=(ease(.28+h,0,1)-ease(.28,0,1))/h;
assert(Math.abs(left-right)<.001);
for(let t=0;t<=18000;t+=10) {
  for(const endpoints of [[.88,.65,.33,.88],[.14,.24,.55,.14],[.14,.7,.84,.14]]) {
    const size=morph(t,...endpoints); assert(size>0 && size<1);
  }
}
// Side-by-side source fits by height: camera and screen have equal displayed heights.
assert(1920*.65/1280 > 840*.7/800);
assert.equal(morph(7000,.14,.7,.84,.14),.7);
// The canonical corner bubble is circular and extends below the screen.
const bubbleDiameter=1920*.14;
const bubbleBottom=Math.min(840,.695*840+bubbleDiameter);
assert(bubbleBottom>(.07+.86)*840);
assert(bubbleDiameter/2 > 1920*.024);
""")
    print("Layout continuity, bounds and equal-height checks passed")
    samples = json.loads((HERE.parents[1] / "tools/recorder-demo/storyboard.json").read_text())["samples"]
    expressions = cursor_expressions(samples)
    actions = [s for i, s in enumerate(samples) if s["values"]["cursorPressed"] and not samples[i - 1]["values"]["cursorPressed"]]
    run("node", "-e", helpers + f"""
const assert=require('node:assert/strict');
const expressions={json.dumps(expressions)}, actions={json.dumps(actions)};
const value=(key,t)=>new Function('computed', 'return '+expressions[key])({{t}});
for(const action of actions) {{
  const t=action.timeMs*2.25;
  for(const axis of ['X','Y']) {{
    const target=action.values['cursor'+axis];
    assert(Math.abs(value(axis,t)-target)<.001);
    assert(Math.abs(value(axis,t-100)-target)<.001);
    assert(Math.abs(value(axis,t-900)-value(axis,t-850))<.001);
  }}
  assert.equal(value('PRESSED',t),1);
  assert.equal(value('PRESSED',t+250),0);
}}
for(let t=0;t<18000;t+=10) {{
  const speed=Math.hypot(value('X',t+10)-value('X',t),value('Y',t+10)-value('Y',t))*100;
  assert(speed<2300);
}}
""")
    print("Cursor click alignment, reading holds and travel-speed checks passed")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    check_motion()
    workspace = create(args.workspace)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        run("mont", "render", "-C", str(workspace), "--output", str(args.output.resolve()), "--fps", "30", "--json")
        print(args.output)
