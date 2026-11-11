#!/usr/bin/env python3
"""Check chart geometry, morph continuity, proportional areas and invalid datasets."""
import json
from create import DEFAULT_SERIES, HERE, run, series_tokens

rows, total = series_tokens(DEFAULT_SERIES)
assert total == 2840
assert abs(sum(d['share'] for d in rows) - 1) < 1e-12
for bad in ([], [{'label': 'A', 'value': 0}] * 3, [{'label': 'A', 'value': -1}] * 3, [{'label': 'A', 'value': float('nan')}] * 3):
    try:
        series_tokens(bad)
    except ValueError:
        pass
    else:
        raise AssertionError('invalid dataset accepted')
helpers = (HERE / 'scene.tsx').read_text().split('export default', 1)[0]
print(run('node', '-e', helpers + '\nconst rows = ' + json.dumps(rows) + ';\n' + r'''
const assert = require('node:assert/strict');
const close = (a,b) => assert.ok(Math.abs(a-b) < 1e-7, `${a} != ${b}`);
close(stage(0,'showcase'),0);
close(phase(2400,'showcase'),0);
for (let k=0;k<9;k++) {
  const start=2400+k*1700;
  close(phase(start+700,'showcase'),.5);
  for (const offset of [1400,1550,1699]) {
    close(phase(start+offset,'showcase'),1);
    assert.equal(Math.floor(stage(start+offset,'showcase')),k,'hold stays on the completed morph');
  }
}
close(stage(18000,'showcase'),9);
for (const value of [0,9,10,99,100,999,1000,2840,1000000000]) {
  const digits=digitCount(value);
  const text=Array.from({length:digits},(_,i)=>String(Math.floor(value/10**(digits-1-i))%10)).join('');
  assert.equal(text,String(value),'counter must not drop digits at powers of ten');
}
for (let k=0;k<10;k++) for (const d of rows) for(let j=0;j<=128;j++) {
  const p=contour(k,d,j/128,rows.length);
  assert.ok(Number.isFinite(p.x)&&Number.isFinite(p.y));
  assert.ok(p.x>=700 && p.x<=1720 && p.y>=270 && p.y<=830, `clipped family ${k}: ${JSON.stringify(p)}`);
}
for(let k=0;k<9;k++) for(const d of rows) for(let j=0;j<=128;j++) {
  const p=j/128;
  const a=chartPoint(k,d,p,rows.length,1,18000), b=chartPoint(k+1,d,p,rows.length,0,18000);
  close(a.x,b.x); close(a.y,b.y);
}
for(let k=0;k<9;k++) {
  const boundary=2400+(k+1)*1700;
  for(const d of rows) {
    const before=chartPoint(Math.floor(stage(boundary-.001,'showcase')),d,.31,rows.length,phase(boundary-.001,'showcase'),boundary-.001);
    const after=chartPoint(Math.floor(stage(boundary+.001,'showcase')),d,.31,rows.length,phase(boundary+.001,'showcase'),boundary+.001);
    assert.ok(Math.hypot(after.x-before.x,after.y-before.y)<.01,'morph must not jump at the next family');
  }
}
for (const d of rows) {
  const center=barCenter(d,rows.length);
  for (const blend of [0,.1,.3,.55]) for (let j=0;j<64;j++) {
    const a=morphPoint(0,d,j/128,rows.length,blend), b=morphPoint(0,d,j/128+.5,rows.length,blend);
    close((a.x+b.x)/2,center.x); close((a.y+b.y)/2,center.y);
  }
  for(let j=0;j<128;j++) {
    const circle=morphPoint(0,d,j/128,rows.length,.55);
    close(Math.hypot(circle.x-center.x,circle.y-center.y),9);
  }
  for (const blend of [.55,.7,.9,1]) {
    const line=linePoint(rows,0,d.index/(rows.length-1),blend);
    const left=morphPoint(0,d,0,rows.length,blend), right=morphPoint(0,d,.5,rows.length,blend);
    close(line.x,(left.x+right.x)/2); close(line.y,(left.y+right.y)/2);
  }
  for (const blend of [0,.25,.75,1]) for (const p of [.1,.3,.7]) {
    const ring=morphPoint(3,d,p,rows.length,blend);
    close(Math.hypot(ring.x-1260,ring.y-574),p<=.5?250:142*(1-blend));
  }
}
for (let j=0;j<=128;j++) {
  const p=j/128, edge=linePoint(rows,2,p,1);
  close(Math.hypot(edge.x-1260,edge.y-574),250);
  for (const blend of [.2,.5,.8]) {
    const line=linePoint(rows,2,p,blend);
    const i=Math.min(rows.length-1,Math.floor(p*rows.length)), local=(p*rows.length-i)/2;
    const fill=morphPoint(2,rows[i],local,rows.length,blend);
    close(line.x,fill.x); close(line.y,fill.y);
  }
}
assert.ok(lineVisibility(2,.7)>.8,'trend must remain visible while wrapping into the ring');
close(lineVisibility(2,1),0);
for(let k=0;k<9;k++) for(const blend of [0,.2,.5,.8,1]) for(const d of rows) for(let j=0;j<=128;j++) {
  const p=morphPoint(k,d,j/128,rows.length,blend);
  assert.ok(Number.isFinite(p.x)&&Number.isFinite(p.y),'finite intermediate contour');
}
for (const d of rows) {
  const origin=polarPoint(0,0), edge=contour(5,d,0,rows.length);
  close(((edge.x-origin.x)**2+(edge.y-origin.y)**2)/(265*265),d.normalized);
  const arcEnd=contour(5,d,.5,rows.length), radius=265*Math.sqrt(d.normalized);
  close(Math.hypot(arcEnd.x-edge.x,arcEnd.y-edge.y),2*radius*Math.sin(Math.PI/rows.length));
  const bubble=contour(8,d,0,rows.length);
  close((bubble.x-(754+900*d.hours/10))**2/169,d.team);
}
console.log('Chart bounds, proportional areas, centred circle morphs, matched line/ring edges, morph continuity and input validation passed.');
''').decode().strip())
