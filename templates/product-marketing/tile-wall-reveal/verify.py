#!/usr/bin/env python3
"""Check camera staging, then render and inspect the actual portable scene."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import uuid

INPUTS = dict(width=960, height=960, speed=1, holdMs=350, pullbackMs=1100,
              tiltMs=1500, panSpeed=70, background="#080D19")


def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stdout + result.stderr).decode(errors="replace"))
    return result.stdout


def check_motion(helpers=None):
    if helpers is None:
        helpers = Path(__file__).with_name("scene.tsx").read_text().split("export default defineSpanScene", 1)[0]
    metrics = json.loads(run("node", "-e", helpers + """
const hold=350, duration=1100, tiltStart=hold, tiltDuration=1500;
const scales=Array.from({length:111},(_,i)=>wallScale(hold+i*10,hold,duration));
console.log(JSON.stringify({
  opening:wallScale(0,hold,duration), settled:wallScale(4000,hold,duration),
  leaving:Math.abs(wallScale(hold+1,hold,duration)-wallScale(hold,hold,duration)),
  arriving:Math.abs(wallScale(hold+duration,hold,duration)-wallScale(hold+duration-1,hold,duration)),
  reversals:scales.slice(1).filter((s,i)=>s>scales[i]+1e-8).length,
  tiltBefore:wallLean(hold-1,tiltStart,tiltDuration),tiltAfter:wallLean(3500,tiltStart,tiltDuration),
  squash:isoSquash(1),rotation:wallRotation(1),
  panBefore:wallPan(hold-1,tiltStart,tiltDuration,70),
  panStart:wallPan(hold+1,tiltStart,tiltDuration,70),
  panA:wallPan(3000,tiltStart,tiltDuration,70),
  panB:wallPan(4000,tiltStart,tiltDuration,70),
  panC:wallPan(5000,tiltStart,tiltDuration,70)
}));
"""))
    assert metrics["opening"] > 3.2 and metrics["settled"] == 1, metrics
    assert max(metrics["leaving"], metrics["arriving"]) < 0.00002, ("Camera must ease out of and into rest", metrics)
    assert metrics["reversals"] == 0, ("Pullback must not bounce or reverse", metrics)
    assert abs(metrics["squash"] - 1 / (3 ** 0.5)) < 1e-9 and metrics["rotation"] == -45, ("Leftward isometric landing with equal foreshortening and 30-degree grid axes", metrics)
    assert metrics["panBefore"] == 0 and metrics["panStart"] < 0.00001, ("Pan must accelerate from rest", metrics)
    assert abs(metrics["panB"] - metrics["panA"] - 70) < 1e-8 and abs(metrics["panC"] - metrics["panB"] - 70) < 1e-8, ("Local vertical pan must continue steadily after landing", metrics)
    assert metrics["tiltBefore"] == 0 and metrics["tiltAfter"] == 1, metrics
    print(json.dumps(metrics, indent=2))


def render(output):
    source = Path(__file__).with_name("scene.tsx").read_text()
    with tempfile.TemporaryDirectory(prefix="mont-tile-wall-") as folder:
        workspace = str(Path(folder) / "workspace")
        run("mont", "init", workspace, "--json")

        def dispatch(command):
            run("mont", "dispatch", "-C", workspace, "-", "--json", stdin=json.dumps({
                "commandId": str(uuid.uuid4()), "command": command,
            }).encode())

        for key in ("sceneWidth", "sceneHeight"):
            dispatch(dict(type="setProjectValue", pointer="/" + key, value=960))
        dispatch({"type": "user_intent:programmatic_span.create", "request": {
            "requestedId": "tile-wall-reveal",
            "placement": dict(startMs=0, durationMs=6000, trackId="template", layer=0),
            "capability": {"kind": "programmatic.scene", "version": 1, "state": {
                "sourceTsx": source, "variables": INPUTS,
                "frame": dict(x=0, y=0, width=960, height=960),
            }},
            "interaction": dict(selectable=True),
            "producer": dict(kind="programmatic", id="mont-templates", version="1"),
        }})
        run("mont", "render", "-C", workspace, "--output", str(output), "--fps", "30", "--json")


def check_video(video):
    meta = json.loads(run("ffprobe", "-v", "error", "-select_streams", "v:0",
                          "-show_entries", "stream=width,height,nb_frames", "-of", "json", str(video)))["streams"][0]
    assert (meta["width"], meta["height"], int(meta["nb_frames"])) == (960, 960, 180), meta

    def frame(seconds):
        return run("ffmpeg", "-v", "error", "-ss", str(seconds), "-i", str(video),
                   "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "rgb24", "-")

    first, revealed, final = frame(0.2), frame(1.65), frame(5.8)
    warm = sum(first[p] > 170 and first[p+1] > 120 and first[p+2] < 190
               for p in range(0, len(first), 12))
    assert warm > 1500, ("Opening editing illustration must be visible, not an empty panel", warm)
    changes = sum(abs(first[i]-revealed[i]) > 24 for i in range(0, len(first), 51))
    assert changes > 9000, ("The closeup must visibly reveal the surrounding wall", changes)
    earlier = frame(4.2)
    motion = sum(abs(earlier[p] - final[p]) > 24 for p in range(0, len(final), 51))
    assert motion > 6000, ("Wall must keep travelling after isometric landing", motion)
    print(json.dumps(dict(square="960x960",frames=180,changed_samples=changes,
                          opening_warm_samples=warm,continuing_pan_samples=motion),indent=2))


if __name__ == "__main__":
    output = Path(sys.argv[1] if len(sys.argv)>1 else "/tmp/tile-wall-reveal.mp4").resolve()
    check_motion()
    render(output)
    check_video(output)
    print(output)
