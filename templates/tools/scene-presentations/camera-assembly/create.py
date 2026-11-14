#!/usr/bin/env python3
"""Create an editable four-step 3D presentation, with or without a presenter."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import uuid

HERE = Path(__file__).resolve().parent
MEDIA = HERE.parents[3] / "assets/template-media/recorders"


def state_for(variant, presenter=None):
    presenter = (presenter or MEDIA / "presenter-explaining.mp4").resolve()
    if variant == "scene-camera" and not presenter.is_file():
        raise FileNotFoundError(presenter)
    replacements = {"@scene": (HERE / "scene.tsx").read_text(),
                    "@presenter": str(presenter)}

    def resolve(value):
        if isinstance(value, dict):
            return {key: resolve(item) for key, item in value.items()}
        if isinstance(value, list):
            return [resolve(item) for item in value]
        return replacements.get(value, value) if isinstance(value, str) else value

    state = resolve(json.loads((HERE / "recording.example.json").read_text())[variant])
    scene = state["imageAssets"]["video.mont.steps.camera-assembly"]["scene"]
    scene["identity"] = "sha256:" + hashlib.sha256(scene["source"].encode()).hexdigest()
    return state


def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stderr or result.stdout).decode())
    return json.loads(result.stdout)


def create(workspace, variant, presenter=None):
    state = state_for(variant, presenter)
    workspace = workspace.resolve()
    run("mont", "init", str(workspace), "--json")

    def dispatch(command):
        run("mont", "dispatch", "-C", str(workspace), "-", "--json", stdin=json.dumps({
            "commandId": str(uuid.uuid4()), "command": command}).encode())

    for key, value in (("sceneWidth", 1920), ("sceneHeight", 1080)):
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    bindings = {}
    if variant == "scene-camera":
        imported = run("mont", "import", state["variables"]["cameraSource"],
                       "-C", str(workspace), "--json")["result"]
        bindings["camera"] = imported["sourceId"]
        state["variables"]["cameraSource"] = "host-frame:camera"
        # Keep the resource, not the full-frame clip added by `mont import`.
        dispatch({"type": "setProjectValue", "pointer": "/programmaticSpans", "value": []})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "camera-assembly",
        "placement": {"startMs": 0, "durationMs": 10000, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": state, "sourceBindings": bindings},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    return workspace


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--variant", choices=("scene-camera", "scene-only"), default="scene-camera")
    parser.add_argument("--presenter", type=Path, help="Replace the fictional AI presenter with local footage")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    workspace = create(args.workspace, args.variant, args.presenter)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        run("mont", "render", "-C", str(workspace), "--output", str(args.output.resolve()), "--fps", "24", "--json")
