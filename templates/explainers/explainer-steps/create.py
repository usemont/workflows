#!/usr/bin/env python3
"""Build the layered noise-cancellation explainer; optionally render a preview."""
import argparse
import re
import json
from pathlib import Path
import subprocess
import uuid

HERE = Path(__file__).resolve().parent


def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stdout + result.stderr).decode(errors="replace"))
    return result.stdout


def check_motion():
    helpers = (HERE / "scene.tsx").read_text().split("export default defineSpanScene", 1)[0]
    run("node", "-e", helpers + """
const assert = require('node:assert/strict');
assert.equal(ease(-10,0,100),0);
assert.equal(ease(200,0,100),1);
let previous=0;
for(let t=0;t<=100;t++) {
  const next=ease(t,0,100);
  assert(next>=previous && next<=1); previous=next;
}
// Opposite sound must match phase and amplitude, not merely resemble a second wave.
for (let t of [0,4000,14000,18000]) {
  for(let p=0;p<=1;p+=0.01) {
    assert(Math.abs(wave(p,t,64,1)+wave(p,t,64,-1))<1e-9);
  }
}
""")
    print("Motion and opposite-phase checks passed")


def create(workspace, inputs):
    workspace = workspace.resolve()
    source = (HERE / "scene.tsx").read_text()
    resources = {uri: {"kind": "image", "uri": uri}
                 for uri in re.findall(r'data:image/png;base64,[A-Za-z0-9+/=]+', source)}
    run("mont", "init", str(workspace), "--json")

    def dispatch(command):
        run("mont", "dispatch", "-C", str(workspace), "-", "--json", stdin=json.dumps({
            "commandId": str(uuid.uuid4()), "command": command}).encode())

    for key, value in (("sceneWidth", 1920), ("sceneHeight", 1080), ("resources", resources)):
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "explainer-steps",
        "placement": {"startMs": 0, "durationMs": 24000, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "source": source, "variables": inputs,
            "frame": {"x": 0, "y": 0, "width": 1920, "height": 1080}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    return workspace


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--inputs", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    check_motion()
    workspace = create(args.workspace, json.loads(args.inputs.read_text()) if args.inputs else {})
    print(workspace)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        run("mont", "render", "-C", str(workspace), "--output", str(args.output.resolve()), "--fps", "30", "--json")
        print(args.output.resolve())
