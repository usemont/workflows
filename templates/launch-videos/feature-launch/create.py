#!/usr/bin/env python3
"""Create an editable feature launch project; optionally render its 12-second preview."""
import argparse
import base64
import json
import mimetypes
from pathlib import Path
import subprocess
import uuid

HERE = Path(__file__).resolve().parent
EMPTY = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAAC0lEQVR4nGNgAAIAAAUAAXpeqz8AAAAASUVORK5CYII="


def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stdout + result.stderr).decode(errors="replace"))
    return result.stdout


def check_motion():
    helpers = (HERE / "scene.tsx").read_text().split("export default defineSpanScene", 1)[0]
    run("node", "-e", helpers + """
const assert = require('node:assert/strict');
// Entrances are clamped and never move backwards or overshoot.
for (const ease of [smooth, arrive]) {
  assert.equal(ease(-100,100,1000),0);
  assert.equal(ease(1200,100,1000),1);
  let previous=0;
  for (let t=100;t<=1000;t+=10) {
    const next=ease(t,100,1000);
    assert(next>=previous && next<=1);
    previous=next;
  }
}
""")
    print("Motion checks passed")


def create(workspace, inputs=None, ui_scene=None, ui_image=None, logo_image=None):
    workspace = Path(workspace).resolve()
    variables = dict(inputs or {})
    resources = {"resource:feature-launch-empty": {"kind": "image", "uri": EMPTY}}
    for key, path in (("uiImage", ui_image), ("logoImage", logo_image)):
        if path:
            mime = mimetypes.guess_type(path)[0]
            if mime not in ("image/png", "image/jpeg", "image/webp"):
                raise ValueError("Use a PNG, JPEG or WebP image")
            uri = "data:" + mime + ";base64," + base64.b64encode(Path(path).read_bytes()).decode()
            resource_id = "resource:feature-launch-" + key
            resources[resource_id] = {"kind": "image", "uri": uri}
            variables[key] = resource_id
            variables["useUiImage" if key == "uiImage" else "useLogoImage"] = True
    child = Path(ui_scene) if ui_scene else HERE / "product-ui.tsx"
    child_source = child.read_text()
    source = (HERE / "scene.tsx").read_text()
    run("mont", "init", str(workspace), "--json")

    def dispatch(command):
        run("mont", "dispatch", "-C", str(workspace), "-", "--json", stdin=json.dumps({
            "commandId": str(uuid.uuid4()), "command": command}).encode())

    for key, value in (("sceneWidth", 1920), ("sceneHeight", 1080), ("resources", resources)):
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "feature-launch",
        "placement": {"startMs": 0, "durationMs": 12000, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "sourceTsx": source, "variables": variables,
            "imageAssets": {"product-ui": {"scene": {
                "identity": "feature-launch-product-ui", "sourceTsx": child_source, "dependencies": {}}}},
            "frame": {"x": 0, "y": 0, "width": 1920, "height": 1080}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    return workspace


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--inputs", type=Path, help="JSON object of scene variable overrides")
    ui = parser.add_mutually_exclusive_group()
    ui.add_argument("--ui-scene", type=Path, help="Standalone native TSX, authored at 1360 × 650")
    ui.add_argument("--ui-image", type=Path)
    parser.add_argument("--logo-image", type=Path)
    parser.add_argument("--output", type=Path, help="Render an MP4 at 1920 × 1080, 30 fps")
    args = parser.parse_args()
    check_motion()
    inputs = json.loads(args.inputs.read_text()) if args.inputs else {}
    workspace = create(args.workspace, inputs, args.ui_scene, args.ui_image, args.logo_image)
    print(workspace)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        run("mont", "render", "-C", str(workspace), "--output", str(args.output.resolve()), "--fps", "30", "--json")
        print(args.output.resolve())
