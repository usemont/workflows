#!/usr/bin/env python3
"""Create a native Mont lesson using a complete localization JSON input table."""
import argparse
import json
from pathlib import Path
import subprocess
import uuid

HERE = Path(__file__).resolve().parent
TEXT_FIELDS = {"step", "of", "topic", "opening", "emphasis", "explanation", "firstVisit", "nextVisit", "browser", "server", "storedCopy", "result", "caveat"}


def validate(inputs):
    messages = inputs.get("messages", {})
    if inputs.get("locale") not in messages:
        raise ValueError("Selected locale is missing from messages")
    for locale, copy in messages.items():
        for field in TEXT_FIELDS:
            if not isinstance(copy.get(field), str) or not copy[field].strip():
                raise ValueError(f"{locale}.{field} must be a non-empty string")
        if (copy.get("font"), copy.get("weight")) not in (("Inter", 600), ("Zen Kaku Gothic New", 400)):
            raise ValueError(f"{locale}: use embedded Inter 600 or Zen Kaku Gothic New 400")
        for field in ("headingSize", "bodySize"):
            if not isinstance(copy.get(field), (int, float)) or not 16 <= copy[field] <= 120:
                raise ValueError(f"{locale}.{field} must be between 16 and 120")
    step, count = inputs.get("stepNumber"), inputs.get("stepCount")
    if type(step) is not int or type(count) is not int or not 1 <= step <= count:
        raise ValueError("Step numbers must satisfy 1 <= stepNumber <= stepCount")


def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stdout + result.stderr).decode(errors="replace"))
    return result.stdout


def create(workspace, inputs):
    validate(inputs)
    workspace = workspace.resolve()
    run("mont", "init", str(workspace), "--json")

    def dispatch(command):
        run("mont", "dispatch", "-C", str(workspace), "-", "--json", stdin=json.dumps({
            "commandId": str(uuid.uuid4()), "command": command}).encode())

    for key, value in (("sceneWidth", 1920), ("sceneHeight", 1080)):
        dispatch({"type": "setProjectValue", "pointer": "/" + key, "value": value})
    dispatch({"type": "user_intent:programmatic_span.create", "request": {
        "requestedId": "lesson-step-localized",
        "placement": {"startMs": 0, "durationMs": 14000, "trackId": "template", "layer": 0},
        "capability": {"kind": "programmatic.scene", "version": 1, "state": {
            "source": (HERE / "scene.tsx").read_text(), "variables": inputs,
            "frame": {"x": 0, "y": 0, "width": 1920, "height": 1080}}},
        "interaction": {"selectable": True},
        "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    return workspace


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--inputs", type=Path, default=HERE / "localization.json")
    parser.add_argument("--locale", choices=("en", "de", "sv", "ja"))
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    inputs = json.loads(args.inputs.read_text())
    if args.locale:
        inputs["locale"] = args.locale
    workspace = create(args.workspace, inputs)
    print(workspace)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        run("mont", "render", "-C", str(workspace), "--output", str(args.output.resolve()), "--fps", "30", "--json")
        print(args.output.resolve())
