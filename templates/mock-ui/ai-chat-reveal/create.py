"""Create an editable chat project with its reusable keyboard scene registered.
Run from the repository root: python3 templates/mock-ui/ai-chat-reveal/create.py ./ai-chat
"""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import uuid

HERE = Path(__file__).resolve().parent


def create(workspace):
    workspace = Path(workspace).resolve()

    def run(*args):
        result = subprocess.run(args, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode:
            raise RuntimeError(result.stderr.decode() + result.stdout.decode(errors="replace"))

    run("mont", "init", str(workspace), "--json")
    with tempfile.TemporaryDirectory(prefix="mont-chat-create-") as temporary:
        envelope = Path(temporary) / "command.json"

        def dispatch(command):
            envelope.write_text(json.dumps({"commandId": str(uuid.uuid4()), "command": command}))
            run("mont", "dispatch", "-C", str(workspace), str(envelope), "--json")

        for name, value in [("sceneWidth", 540), ("sceneHeight", 960)]:
            dispatch({"type": "setProjectValue", "pointer": "/" + name, "value": value})
        dispatch({"type": "user_intent:programmatic_span.create", "request": {
            "requestedId": "ai-chat-reveal",
            "placement": {"startMs": 0, "durationMs": 11500, "trackId": "template", "layer": 0},
            "capability": {"kind": "programmatic.scene", "version": 1, "state": {
                "sourceTsx": (HERE / "scene.tsx").read_text(), "variables": {},
                "imageAssets": {"mobile-keyboard": {"scene": {
                    "identity": "mobile-keyboard",
                    "sourceTsx": (HERE.parent.parent / "components/mobile-keyboard/scene.tsx").read_text(),
                    "dependencies": {},
                }}},
                "frame": {"x": 0, "y": 0, "width": 540, "height": 960}}},
            "interaction": {"selectable": True},
            "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}}})
    return workspace


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workspace", type=Path)
    args = parser.parse_args()
    print(create(args.workspace))
