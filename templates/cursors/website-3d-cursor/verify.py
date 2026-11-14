#!/usr/bin/env python3
"""Check landing choreography and render the native Mont preview."""
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import uuid


def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stdout + result.stderr).decode(errors="replace"))
    return result.stdout


def check_motion():
    helpers = Path(__file__).with_name("scene.tsx").read_text().split("export default defineSpanScene", 1)[0]
    run("node", "-e", helpers + """
const assert=require('node:assert/strict');
function cameraAt(t) {
 return {x:focusX(t),y:focusY(t),scale:zoom(t),rx:pitch(t),ry:yaw(t),rz:roll(t)};
}
function pointer(t) { return {x:pointerX(t), y:pointerY(t)}; }
for(const t of [1000,2600,3000,3500,5200,6000,7900]) {
 const before=cameraAt(t-.01), after=cameraAt(t+.01);
 for(const key of ['x','y','scale','rx','ry','rz'])
  assert(Math.abs(after[key]-before[key])<.03, 'Camera discontinuity: '+key+' at '+t);
 const a=cameraAt(t-40), b=cameraAt(t+40);
 assert(Math.hypot(b.y-a.y,100*(b.scale-a.scale),b.rx-a.rx)>.08,
        'Camera freezes during a cursor accent at '+t);
}
for(let t=0;t<=9000;t+=10) {
 const a=pointer(t), b=pointer(t+10);
 assert(Math.hypot(b.x-a.x,b.y-a.y)<16, 'Cursor jumps at '+t);
 assert(a.x>=0 && a.x<=1440 && a.y>=0 && a.y<=1000, 'Cursor leaves editor at '+t);
 const view=cameraAt(t);
 assert(view.rx>=6 && view.rx<=14.5 && Math.abs(view.ry)<=4 && Math.abs(view.rz)<=1.5,
        'Cursor-led perspective overwhelms the page');
 assert(view.scale>=.60 && view.scale<=.89, 'Camera loses its close framing');
}
const play=pointer(2850), clip=pointer(5600), inspector=pointer(8200);
assert(play.x>728 && play.x<776 && play.y>620 && play.y<668, 'Cursor misses preview playback');
assert(clip.x>690 && clip.x<980 && clip.y>790 && clip.y<850, 'Cursor misses the selected timeline clip');
assert(inspector.x>1350 && inspector.x<1406 && inspector.y>374 && inspector.y<398, 'Cursor misses the inspector slider thumb');
assert(focusY(4000)<.25*guideY(4000)+.75*pointerY(4000), 'Camera must trail the moving cursor');
assert(focusY(5600)>focusY(5200), 'Camera must settle after the pointer reaches its target');
const originalX=pointerX, originalY=pointerY;
const baseline=cameraAt(4300);
pointerX=t=>originalX(t)+80*ease(t,3500,4800);
pointerY=t=>originalY(t)+140*ease(t,3500,4800);
const retargeted=cameraAt(4300);
assert(retargeted.x>baseline.x+1 && retargeted.y>baseline.y+8,
       'Camera must follow a changed cursor route, not an unrelated timeline');
assert(retargeted.rx>baseline.rx+.1 && retargeted.ry>baseline.ry+.1 && Math.abs(retargeted.rz-baseline.rz)>.03,
       'Camera travel must drive whole-window pitch, yaw and roll');
pointerX=originalX; pointerY=originalY;
// Independent 3D projection checks that the portable 2D groups really follow local Z.
function project(x,y,z,t) {
 const rx=pitch(t)*Math.PI/180, ry=yaw(t)*Math.PI/180, rz=roll(t)*Math.PI/180;
 const py=y*Math.cos(rx)-z*Math.sin(rx), pz=y*Math.sin(rx)+z*Math.cos(rx);
 const px=x*Math.cos(ry)+pz*Math.sin(ry), depth=-x*Math.sin(ry)+pz*Math.cos(ry);
 const w=1-depth/1800;
 return [(px*Math.cos(rz)-py*Math.sin(rz))/w,(px*Math.sin(rz)+py*Math.cos(rz))/w];
}
for(const [delay,cx,cy] of [[100,720,35],[240,143,405],[360,712,405],[480,1289,405],[600,720,840]]) {
 assert.equal(reveal(delay,delay),0, 'Landing content is visible before arrival');
 assert(lift(delay,delay)>0 && landingScale(delay,delay)>1, 'Layer must start above its surface');
 assert(assemble(delay+300,delay)>.75, 'Arrival must be quick, followed by a gentle settle');
 assert.equal(lift(delay+900,delay),0, 'Layer must reach surface depth exactly');
 assert.equal(landingScale(delay+900,delay),1, 'Landed layer must retain its authored size');
 assert.equal(landingX(delay+900,delay,cx),cx, 'Landing must preserve authored X');
 assert.equal(landingY(delay+900,delay,cy),cy, 'Landing must preserve authored Y');
 assert.equal(reveal(delay+900,delay),1, 'Landed content must be fully visible');
 let previous=0;
 for(let t=delay;t<=delay+900;t+=10) {
  const progress=assemble(t,delay), s=landingScale(t,delay);
  assert(progress>=previous && progress<=1, 'Landing must not reverse or overshoot text');
  previous=progress;
  for(const [dx,dy] of [[0,0],[-120,-200],[120,200]]) {
   const expected=project(cx+dx-focusX(t),cy+dy-focusY(t),lift(t,delay),t);
   const actual=project(landingX(t,delay,cx)+s*dx-focusX(t),landingY(t,delay,cy)+s*dy-focusY(t),0,t);
   assert(Math.hypot(actual[0]-expected[0],actual[1]-expected[1])<1e-8,
          'Layer must translate along the surface normal, not slide or scale in screen space');
  }
 }
}
console.log('Continuous cursor-led camera, true surface-normal landings and exact final bounds: passed');
""")
    print("Motion checks passed")


def render(output):
    source = Path(__file__).with_name("scene.tsx").read_text()
    cursor_uri = re.search(r'cursorSrc: stringVar\("([^"]+)"\)', source).group(1)
    with tempfile.TemporaryDirectory(prefix="mont-website-3d-") as folder:
        workspace = str(Path(folder) / "workspace")
        run("mont", "init", workspace, "--json")
        def dispatch(command):
            return run("mont", "dispatch", "-C", workspace, "-", "--json", stdin=json.dumps({"commandId": str(uuid.uuid4()), "command": command}).encode())
        for key in ("sceneWidth", "sceneHeight"):
            dispatch(dict(type="setProjectValue", pointer="/" + key, value=960))
        # Direct dispatch does not materialize embedded images like template import does.
        dispatch(dict(type="setProjectValue", pointer="/resources", value={
            "resource:website-cursor": {"kind": "image", "uri": cursor_uri}
        }))
        dispatch({"type": "user_intent:programmatic_span.create", "request": {
            "requestedId": "website-3d-cursor", "placement": {"startMs": 0, "durationMs": 9000, "trackId": "template", "layer": 0},
            "capability": {"kind": "programmatic.scene", "version": 1, "state": {"source": source, "variables": {"cursorSrc": "resource:website-cursor"}, "frame": {"x": 0, "y": 0, "width": 960, "height": 960}}},
            "interaction": {"selectable": True}, "producer": {"kind": "programmatic", "id": "mont-templates", "version": "1"}
        }})
        run("mont", "render", "-C", workspace, "--output", str(output), "--fps", "30", "--json")
    print(output)


if __name__ == "__main__":
    check_motion()
    if len(sys.argv) > 1:
        render(Path(sys.argv[1]).resolve())
