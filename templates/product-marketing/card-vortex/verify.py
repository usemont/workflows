#!/usr/bin/env python3
"""Check differential vortex flow and render its square convergence/light handoff."""
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile
import uuid

INPUTS = dict(width=960, height=960, speed=1, holdMs=550, collapseMs=1700,
              staggerMs=110, circulation=300, coreRadius=70, flowRampMs=150,
              alignLagMs=55, cardBlur=0.7, defocus=3, shutterMs=20,
              vignette=0.5, flash=0.7, background="#101521", surface="#202A3B",
              border="#39465A", ink="#F3F4F8", muted="#A6B1C5", accent="#F5ACD1")


def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stdout + result.stderr).decode(errors="replace"))
    return result.stdout


def check_flow():
    helpers = Path(__file__).with_name("scene.tsx").read_text().split("export default defineSpanScene", 1)[0]
    metrics = json.loads(run("node", "-e", helpers + """
const computed = {hold:550, stagger:0,
  ramp:150, drain:270, circulation:300, coreRadius:70, coreSquared:4900, lag:55};
const inner = {x:180,y:0,tilt:0,arc:1,phase:0,delay:0,pitch:20,yaw:60,scale:1};
const outer = {...inner,x:450};
function omega(card, clock) {
  const ax=pathX(card,clock-1), ay=pathY(card,clock-1);
  const bx=pathX(card,clock+1), by=pathY(card,clock+1);
  return Math.atan2(ax*by-ay*bx,ax*bx+ay*by)/2;
}
const clock=1250, lagged=clock-computed.lag;
const heading=Math.atan2(pathY(outer,lagged+1)-pathY(outer,lagged-1),
  pathX(outer,lagged+1)-pathX(outer,lagged-1));
const error=typeof cardRotation==="function" ? cardRotation(outer,clock)*Math.PI/180-heading : Math.PI;
console.log(JSON.stringify({
  innerToOuter:omega(inner,730)/omega(outer,730),
  outerAcceleration:omega(outer,1250)/omega(outer,730),
  coreRatio:omega({...inner,x:35},850)/omega({...inner,x:60},850),
  logRelativeError:Math.max(...[2,10,100,950].map(x=>Math.abs(vortexLog(x)/Math.log(x)-1))),
  alignmentErrorDegrees:Math.abs(Math.atan2(Math.sin(error),Math.cos(error)))*180/Math.PI
}));
"""))
    assert metrics["innerToOuter"] > 3, ("Inner cards must overtake the outer flow", metrics)
    assert metrics["outerAcceleration"] > 1.8, ("Cards must accelerate as their orbit contracts", metrics)
    assert abs(metrics["coreRatio"] - 1) < 0.02, ("Core angular velocity must be capped", metrics)
    assert metrics["alignmentErrorDegrees"] < 10, ("Card heading must follow the lagged flow tangent", metrics)
    assert metrics["logRelativeError"] < 0.001, ("Portable logarithm must preserve the vortex flow", metrics)
    print(json.dumps(metrics, indent=2))


def render(output):
    source = Path(__file__).with_name("scene.tsx").read_text()
    with tempfile.TemporaryDirectory(prefix="mont-vortex-") as folder:
        workspace = str(Path(folder) / "workspace")
        run("mont", "init", workspace, "--json")

        def dispatch(command):
            run("mont", "dispatch", "-C", workspace, "-", "--json", stdin=json.dumps({
                "commandId": str(uuid.uuid4()), "command": command,
            }).encode())

        for key in ("sceneWidth", "sceneHeight"):
            dispatch(dict(type="setProjectValue", pointer="/" + key, value=960))
        dispatch({"type": "user_intent:programmatic_span.create", "request": {
            "requestedId": "card-vortex",
            "placement": dict(startMs=0, durationMs=3000, trackId="template", layer=0),
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
    assert (meta["width"], meta["height"], int(meta["nb_frames"])) == (960, 960, 90), meta

    def frame(seconds):
        return run("ffmpeg", "-v", "error", "-ss", str(seconds), "-i", str(video),
                   "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "rgb24", "-")

    def card_field(pixels):
        radii = []
        for y in range(12, 948, 4):
            for x in range(12, 948, 4):
                at = (y * 960 + x) * 3
                color = pixels[at:at + 3]
                radius = math.hypot(x - 480, y - 480)
                if radius > 45 and max(color) > 64 and max(color) - min(color) > 24:
                    radii.append(radius)
        return len(radii), math.sqrt(sum(r * r for r in radii) / max(1, len(radii))), sum(r > 200 for r in radii)

    start = card_field(frame(0.35))
    inward = card_field(frame(1.95))
    end = card_field(frame(2.9))
    assert start[0] > 100 and start[1] > 280, ("Cards must initially fill the frame", start)
    assert inward[0] > 8 and inward[1] < start[1] * 0.75, ("Cards must converge before disappearing", start, inward)
    assert end[2] == 0, ("Outer cards must clear for the light handoff", end)

    def halo(pixels):
        samples = []
        for y in range(390, 570, 6):
            for x in range(390, 570, 6):
                if 30 < math.hypot(x - 480, y - 480) < 90:
                    at = (y * 960 + x) * 3
                    samples.append(sum(pixels[at:at + 3]) / 3)
        return sum(samples) / len(samples)

    peak, quiet = halo(frame(2.36)), halo(frame(2.9))
    assert peak > quiet + 6, ("The convergence must trigger a light pulse, then settle", peak, quiet)
    print(json.dumps(dict(square="960x960", frames=90,
                          opening_colored_samples=start[0], opening_radius=round(start[1], 1),
                          converging_radius=round(inward[1], 1), final_outer_samples=end[2],
                          peak_halo=round(peak, 1), settled_halo=round(quiet, 1)), indent=2))


if __name__ == "__main__":
    output = Path(sys.argv[1] if len(sys.argv) > 1 else "/tmp/card-vortex.mp4").resolve()
    check_flow()
    render(output)
    check_video(output)
    print(output)
