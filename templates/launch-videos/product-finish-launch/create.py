#!/usr/bin/env python3
"""Create an editable native 3D product launch; optionally render a 12-second preview."""
import argparse
import json
from pathlib import Path
import subprocess
import uuid

HERE = Path(__file__).resolve().parent


def create(workspace, inputs, mont="mont"):
    workspace = Path(workspace).resolve()

    def run(*args, command=None):
        result = subprocess.run([mont, *args], input=None if command is None else json.dumps({
            "commandId": str(uuid.uuid4()), "command": command}).encode(), capture_output=True)
        if result.returncode:
            raise RuntimeError((result.stdout + result.stderr).decode(errors="replace"))

    run("init", str(workspace), "--json")
    for key, value in (("sceneWidth", 1920), ("sceneHeight", 1080)):
        run("dispatch", "-C", str(workspace), "-", "--json", command={
            "type": "setProjectValue", "pointer": "/" + key, "value": value})
    run("dispatch", "-C", str(workspace), "-", "--json", command={
        "type": "user_intent:programmatic_span.create", "request": {
            "requestedId": "product-finish-launch",
            "placement": {"startMs": 0, "durationMs": 12000, "trackId": "template", "layer": 0},
            "capability": {"kind": "programmatic.scene", "version": 1, "state": {
                "source": (HERE / "scene.tsx").read_text(), "variables": inputs,
                "frame": {"x": 0, "y": 0, "width": 1920, "height": 1080}}},
            "interaction": {"selectable": True},
            "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    return workspace


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--inputs", type=Path, help="JSON object with productName, accent and roughness")
    parser.add_argument("--mont", default="mont", help="Mont CLI executable")
    parser.add_argument("--output", type=Path, help="Render an MP4 at 1920 × 1080, 30 fps")
    args = parser.parse_args()
    inputs = json.loads(args.inputs.read_text()) if args.inputs else {}
    if not isinstance(inputs, dict):
        parser.error("--inputs must contain a JSON object")
    workspace = create(args.workspace, inputs, args.mont)
    print(workspace)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run([args.mont, "render", "-C", str(workspace), "--output",
                        str(args.output.resolve()), "--fps", "30", "--json"], check=True)
        print(args.output.resolve())
