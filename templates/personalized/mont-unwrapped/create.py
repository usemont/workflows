#!/usr/bin/env python3
"""Create an editable, row-personalized Mont Unwrapped project."""
import argparse
import base64
import json
import math
from pathlib import Path
import subprocess
import uuid

HERE = Path(__file__).resolve().parent
LAYOUTS = ("Side by side", "Corner bubble", "Presenter")


def validate(inputs):
    if not isinstance(inputs, dict):
        raise ValueError("Each input row must be an object")
    for key, ceiling in (("name", 24), ("company", 48), ("product", 24), ("link", 90)):
        value = inputs.get(key, "")
        if not isinstance(value, str) or not value.strip() or len(value) > ceiling:
            raise ValueError(f"{key} must contain 1–{ceiling} characters")
    for key, ceiling in (("videos", 9999), ("minutes", 999999), ("year", 9999)):
        value = inputs.get(key)
        if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value) or value != int(value) or not 0 <= value <= ceiling:
            raise ValueError(f"{key} must be a whole number from 0 to {ceiling}")
    if inputs.get("favouriteLayout") not in LAYOUTS:
        raise ValueError("favouriteLayout must be Side by side, Corner bubble, or Presenter")
    if "accent" in inputs:
        color = inputs["accent"]
        if not isinstance(color, str) or len(color) != 7 or color[0] != "#" or any(c not in "0123456789abcdefABCDEF" for c in color[1:]):
            raise ValueError("accent must be a #RRGGBB color")


def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stdout + result.stderr).decode(errors="replace"))
    return result.stdout


def create(workspace, inputs, logo=None):
    validate(inputs)
    workspace = Path(workspace).resolve()
    variables = dict(inputs)
    variables.update(
        seriesLabel=f"{inputs['product']} Unwrapped", yearLabel=str(int(inputs["year"])),
        greetingLabel=f"{inputs['name']},", greetingSize=min(114, 1250 / ((len(inputs["name"]) + 1) * .65)),
        recipientLabel=f"{inputs['name']} / {int(inputs['year'])}",
        companyLabel=f"{inputs['company']}, made in {inputs['product']}.",
        summaryLabel=f"{int(inputs['videos'])} videos  /  {int(inputs['minutes'])} minutes  /  {inputs['favouriteLayout']}",
    )
    image = Path(logo) if logo else HERE / "assets" / "mont-logo.png"
    data = image.read_bytes()
    if not data.startswith(b"\x89PNG\r\n\x1a\n"):
        raise ValueError("Logo must be a PNG")
    variables["logoImage"] = "resource:mont-unwrapped-logo"
    resources = {variables["logoImage"]: {"kind": "image", "uri": "data:image/png;base64," + base64.b64encode(data).decode()}}
    run("mont", "init", str(workspace), "--json")

    def dispatch(command):
        run("mont", "dispatch", "-C", str(workspace), "-", "--json", stdin=json.dumps({"commandId": str(uuid.uuid4()), "command": command}).encode())

    for key, value in (("sceneWidth", 1920), ("sceneHeight", 1080), ("resources", resources)):
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "mont-unwrapped", "placement": {"startMs": 0, "durationMs": 19000, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "source": (HERE / "scene.tsx").read_text(), "variables": variables,
            "frame": {"x": 0, "y": 0, "width": 1920, "height": 1080}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    return workspace


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--inputs", type=Path, default=HERE / "inputs.example.json")
    parser.add_argument("--row", type=int, default=0, help="Zero-based row index when inputs is an array")
    parser.add_argument("--logo", type=Path, help="Optional PNG replacing the Mont logo")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    inputs = json.loads(args.inputs.read_text())
    if isinstance(inputs, list):
        if not 0 <= args.row < len(inputs):
            parser.error("--row is outside the input array")
        inputs = inputs[args.row]
    workspace = create(args.workspace, inputs, args.logo)
    print(workspace, flush=True)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        run("mont", "render", "-C", str(workspace), "--output", str(args.output.resolve()), "--fps", "30", "--json")
        print(args.output.resolve())
