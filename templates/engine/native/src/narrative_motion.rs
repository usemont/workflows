//! Continuous scene-clock ports of the v9.12 action and story families.
use super::{actor, Actor};
use crate::{layout::branded_node, scene::Scene};
use serde_json::{json, Value};
use std::{collections::{BTreeMap, BTreeSet}, f64::consts::PI, sync::LazyLock};

fn clamp(x:f64)->f64{x.clamp(0.0,1.0)}
fn io(x:f64)->f64{let x=clamp(x);if x<0.5{8.0*x.powi(4)}else{1.0-(-2.0*x+2.0).powi(4)/2.0}}
fn round(x:f64)->f64{(x+0.5).floor()}
fn q(x:&str)->String{serde_json::to_string(x).unwrap()}
fn c(x:impl AsRef<str>)->String{format!("Math.min(1, Math.max(0, ({})))",x.as_ref())}
fn ease(s:&mut Scene,x:impl AsRef<str>)->String{let u=s.number(c(x));s.number(format!("{u} < 0.5 ? 8 * Math.pow({u}, 4) : 1 - Math.pow(-2 * {u} + 2, 4) / 2"))}
fn envelope(s:&mut Scene,t:&str,a:f64,b:f64,r:f64,f:f64)->String{s.number(format!("{t} < {} || {t} > {} ? 0 : {t} < {a} ? ({t} - {a} + {r}) / {r} : {t} > {b} ? 1 - ({t} - {b}) / {f} : 1",a-r,b+f))}
fn motion(name:&str)->(f64,f64,f64,f64){match name{"midnight"=>(0.7,1.4,3.6,0.26),"blueprint"=>(0.55,3.0,8.0,0.12),"signal"=>(1.5,2.5,4.2,0.18),_=>(1.0,2.1,5.0,0.2)}}
fn spring(s:&mut Scene,u:impl AsRef<str>)->String{let u=s.number(u);let(_,f,d,_)=motion(s.theme.name);s.number(format!("{u} <= 0 ? 0 : 1 - Math.exp(-{} * {u}) * Math.cos({} * {u})",d*1.15,2.0*PI*f*0.8))}
fn rebound(s:&mut Scene,u:impl AsRef<str>)->String{let u=s.number(u);let(_,f,d,_)=motion(s.theme.name);s.number(format!("{u} <= 0 ? 0 : Math.exp(-{d} * {u}) * Math.sin({} * {u})",2.0*PI*f))}
fn windup(s:&mut Scene,u:impl AsRef<str>)->String{let u=s.number(u);let a=motion(s.theme.name).3;s.number(format!("{u} < -{a} ? 0 : {u} < 0 ? 1 - Math.pow(1 - ({u} + {a}) / {a}, 3) : Math.max(0, 1 - {u} / 0.08)"))}
fn back(s:&mut Scene,k:&str)->String{s.number(format!("1 + 2.7 * Math.pow({k} - 1, 3) + 1.7 * Math.pow({k} - 1, 2)"))}
fn add(s:&mut Scene,a:&str,b:impl AsRef<str>)->String{s.number(format!("({a}) + ({})",b.as_ref()))}
fn mul(s:&mut Scene,a:&str,b:impl AsRef<str>)->String{s.number(format!("({a}) * ({})",b.as_ref()))}
#[derive(Clone)]
struct Fx{x:String,y:String,sx:String,sy:String,r:String,o:String}
impl Default for Fx{fn default()->Self{Self{x:"0".into(),y:"0".into(),sx:"1".into(),sy:"1".into(),r:"0".into(),o:"1".into()}}}
fn root(s:&mut Scene,id:&str,n:&Value,f:&Fx){
    let index=n["_narrativeIndex"].as_u64().map(|i|i as usize).unwrap_or_else(||s.transition_nodes.iter().position(|entry|entry==n).unwrap());
    let (x,y,w,h)=node_box(n);
    let scale=s.native_scale();
    let layer=n["_composedLayer"].as_str().map(|v|format!(" layer={{{v}}}")).unwrap_or_default();
    s.push_native(&format!("<MotionBox id=\"transition-node-{index}\" pivotX={{{}}} pivotY={{{}}}{layer}>",(x+w/2.0)*scale,(y+h/2.0)*scale));
    let mut n=n.clone();
    if s.transition_holds.contains(&index){n["_transitionHold"]=json!(true);}else{
        n.as_object_mut().unwrap().remove("_transitionHold");
        for(k,v)in[("_composedDx",&f.x),("_composedDy",&f.y),("_composedScaleX",&f.sx),("_composedScaleY",&f.sy),("_composedRot",&f.r),("_composedOpacity",&f.o)]{n[k]=json!(v);}
        n.as_object_mut().unwrap().remove("_composedLayer");
    }
    let until=n["gone"].as_f64().map(|g|g*1000.0).or_else(||s.exit_scene.then_some(s.exit_start.unwrap_or(s.duration-950.0)));
    let pill=n["t"]=="callout";
    s.begin_theme_motion(id,&n,x,y,w,h,n["a"].as_f64().unwrap()*1000.0,until,pill,!pill);
}
fn end_root(s:&mut Scene){s.end_move();s.end_move();}
fn held(v:&Value)->Value{let mut v=v.clone();v["_transitionHold"]=json!(true);v}
fn box_node(t:&str,g:&str,x:f64,y:f64,w:f64,h:f64,a:f64)->Value{json!({"t":t,"grp":g,"x":round(x),"y":round(y),"w":round(w),"h":round(h),"fixA":a,"label":"","icon":""})}
fn node_box(n:&Value)->(f64,f64,f64,f64){(n["x"].as_f64().unwrap(),n["y"].as_f64().unwrap(),n["w"].as_f64().unwrap(),n["h"].as_f64().unwrap())}
fn frame(s:&mut Scene,id:&str,sx:&str,sy:&str,r:&str,o:&str,px:f64,py:f64){let z=s.native_scale();s.push_native(&format!("<MotionBox id={} scaleX={{{sx}}} scaleY={{{sy}}} rotation={{{r}}} opacity={{{o}}} pivotX={{{}}} pivotY={{{}}}>",q(id),px*z,py*z));}
fn opacity(s:&mut Scene,id:&str,expr:&str){s.push_native(&format!("<MotionBox id={} opacity={{{expr}}}>",q(id)));}
fn path(s:&mut Scene,id:&str,d:&str,rect:(f64,f64,f64,f64),vb:(f64,f64),fill:&str,stroke:&str,sw:&str,op:&str,dash:&str){let(x,y,w,h)=rect;let z=s.native_scale();let attr=|x:&str|if x.starts_with("computed."){format!("{{{x}}}")}else{q(x)};s.push_native(&format!("<Path id={} d={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} viewBox={} fill={} stroke={} strokeWidth={{{sw}}} opacity={{{op}}} strokeDasharray={} strokeLinecap=\"round\" strokeLinejoin=\"round\"/>",q(id),attr(d),x*z,y*z,w*z,h*z,q(&format!("0 0 {} {}",vb.0,vb.1)),attr(fill),attr(stroke),attr(dash)));}
// Absolute source SVGs occupy the CSS padding box and retain their viewBox aspect ratio.
fn svg_box(s:&mut Scene,id:&str,node:&Value,vb:(f64,f64),border:&str){
    let(x,y,w,h)=node_box(node);let(vw,vh)=vb;let z=s.native_scale();
    let fit=s.number(format!("Math.min(({w} - 2 * ({border})) / {vw}, ({h} - 2 * ({border})) / {vh})"));
    let dx=s.number(format!("(({w} - {vw} * {fit}) / 2) * {z}"));let dy=s.number(format!("(({h} - {vh} * {fit}) / 2) * {z}"));
    s.push_native(&format!("<MotionBox id={} x={{{dx}}} y={{{dy}}} scaleX={{{fit}}} scaleY={{{fit}}} pivotX={{{}}} pivotY={{{}}}>",q(id),x*z,y*z));
}
fn card_border(s:&Scene)->f64{let width=THEMES[s.theme.name]["cardBorder"].as_str().unwrap().split("px").next().unwrap().parse::<f64>().unwrap();if width>0.0{width.floor().max(1.0)}else{0.0}}
fn rounded_expr(s:&mut Scene,x:&str,y:&str,w:&str,h:f64,r:f64)->String{let r=s.number(format!("Math.min({r}, ({w}) / 2, {})",h/2.0));let mut parts=Vec::new();let mut p=|text:&str,e:String|{parts.push(q(text));parts.push(s.css_number(&e,1));};p("M",format!("({x}) + {r}"));p(" ",y.into());p("H",format!("({x}) + ({w}) - {r}"));for(text,e)in[("A",r.clone()),(" ",r.clone()),(" 0 0 1 ",format!("({x}) + ({w})")),(" ",format!("({y}) + {r}")),("V",format!("({y}) + {h} - {r}")),("A",r.clone()),(" ",r.clone()),(" 0 0 1 ",format!("({x}) + ({w}) - {r}")),(" ",format!("({y}) + {h}")),("H",format!("({x}) + {r}")),("A",r.clone()),(" ",r.clone()),(" 0 0 1 ",x.into()),(" ",format!("({y}) + {h} - {r}")),("V",format!("({y}) + {r}")),("A",r.clone()),(" ",r.clone()),(" 0 0 1 ",format!("({x}) + {r}")),(" ",y.into())]{p(text,e);}parts.push(q("Z"));s.string(parts.join(" + "))}
fn d(s:&mut Scene,parts:&[(&str,String)])->String{let mut out=Vec::new();for(a,b)in parts{out.push(q(a));out.push(s.css_number(b,1));}s.string(out.join(" + "))}
fn mix(s:&mut Scene,a:&str,b:&str,k:&str)->String{let(mut a,mut b)=(crate::scene::source_color(a).unwrap(),crate::scene::source_color(b).unwrap());if a[3]==0.0{a[..3].copy_from_slice(&b[..3]);}if b[3]==0.0{b[..3].copy_from_slice(&a[..3]);}let mut out=vec![q("rgba(")];for i in 0..4{if i>0{out.push(q(","));}let e=format!("{} + ({} - {}) * ({k})",a[i],b[i],a[i]);out.push(if i==3{s.css_number(&e,3)}else{s.number(format!("Math.floor(({e}) + 0.5)"))});}out.push(q(")"));s.string(out.join(" + "))}
fn item(a:&Actor)->Value{let mut v=json!({"t":match a.tone.as_str(){"accent"=>"pictFocus","warning"=>"pictExc",_=>"pictTile"},"label":"","icon":a.icon,"mod":a.modifier});if let Some(b)=&a.brand{v["brand"]=json!(b)}v}

#[derive(Clone)]
struct Step{verb:String,at:f64,dur:f64,holder:usize,to:Option<usize>,blocked:bool,own:Vec<usize>,source:Option<Actor>,risk:bool,side:f64}
fn cast(v:&Value,actors:&mut Vec<Actor>,keys:&mut Vec<String>)->Result<usize,String>{let a = source_actor(v)?;let key=format!("{}|{}",a.icon,v["label"].as_str().unwrap_or(""));if let Some(i)=keys.iter().position(|k|k==&key){Ok(i)}else{keys.push(key);actors.push(a);Ok(actors.len()-1)}}
fn steps(v:&Value,n:usize)->Result<(Vec<Actor>,Vec<Step>),String>{let mut actors=vec![];let mut keys=vec![];let mut out=vec![];let mut holder=None;let mut cursor=0.9;for x in v["steps"].as_array().unwrap(){let by=x.get("by").map(|v|cast(v,&mut actors,&mut keys)).transpose()?;let verb=x["do"].as_str().unwrap();if holder.is_none(){holder=if by.is_some(){by}else if verb=="pass"{x.get("from").map(|v|cast(v,&mut actors,&mut keys)).transpose()?}else{None};}let h=holder.unwrap();let blocked=x["blocked"].as_bool().unwrap_or(false);let raw=x[if matches!(verb,"inspect"|"compare"){"finds"}else{"lines"}].as_array();let mut own=vec![];if let Some(raw)=raw{for v in raw{let mut i=v.as_f64().unwrap_or(0.0);if !matches!(verb,"inspect"|"strike"|"redact"){i=round(i);}if i>=1.0&&i<=n as f64&&(matches!(verb,"inspect"|"strike"|"redact")||!own.contains(&((i-1.0)as usize))){own.push((i-1.0)as usize);}}}let max=match verb{"strike"|"redact"|"fill"=>3,"fake"=>2,"restyle"=>4,_=>8};own.truncate(max);if own.is_empty()&&raw.is_none()&&matches!(verb,"strike"|"redact"){own.push(1.min(n-1));}if own.is_empty()&&matches!(verb,"fake"|"fill"|"restyle"){own.push(if verb=="fill"{n-1}else{1.min(n-1)});}let dur=match verb{"draft"=>if actors[h].manner=="machine"{0.55+0.17*n as f64}else{0.7+0.5*n as f64},"pass"=>if blocked{2.0}else{1.4},"inspect"=>0.6+0.36*n as f64,"approve"|"reject"=>1.3,"strike"|"redact"=>0.5+0.7*own.len()as f64,"compare"=>0.9+0.42*n as f64,"fake"=>0.9+1.1*own.len()as f64,"fill"=>0.5+0.75*own.len()as f64,"restyle"=>0.5+0.6*own.len()as f64,_=>2.4};let at=x["at"].as_f64().map(|a|a.max(cursor-0.45)).unwrap_or(cursor);let to=if verb=="pass"{Some(cast(&x["to"],&mut actors,&mut keys)?)}else{None};let source=if verb=="compare"{Some(source_actor(x.get("against").unwrap_or(&json!("book")))?)}else{None};out.push(Step{verb:verb.into(),at,dur,holder:h,to,blocked,own,source,risk:x["risk"]!="low",side:1.0});if let Some(to)=to{if !blocked{holder=Some(to)}}cursor=at+dur+0.45;}Ok((actors,out))}
fn line_y(i:usize,n:usize,h:f64,msg:bool)->f64{let top=h*if msg{0.3}else{0.27};top+i as f64*(h-top-h*0.13)/(n-1)as f64}
struct World{cx:f64,cy:f64,gap:f64,size:f64,dw:f64,dh:f64,tool:f64,source:f64,off:f64,one:bool,beside:f64}
impl World{fn actor(&self,i:usize)->(f64,f64){(if self.one{self.cx-self.off/2.0-self.beside}else{self.cx-self.gap/2.0+i as f64*self.gap},self.cy)}fn slot(&self,i:usize)->(f64,f64){let(x,y)=self.actor(i);(x+if i==0{self.off}else{-self.off},y)}}
fn camera(s:&mut Scene,w:&World,steps:&[Step],t:&str)->String{let mut x = "0".to_owned();for a in steps.iter().filter(|a|a.verb=="pass"&&!a.blocked&&a.to.unwrap()>=2){let k=ease(s,format!("({t} - {} - 0.1) / 1.15",a.at));x=add(s,&x,format!("{} * {k}",w.gap));}x}
fn flight(s:&mut Scene,w:&World,steps:&[Step],t:&str,cam:&str)->Fx{let mut f=Fx::default();let mut sc = "1".to_owned();let base = w.slot(steps[0].holder);let amp=motion(s.theme.name).0;if let Some(a)=steps.first().filter(|a|a.verb=="draft"){let k=spring(s,format!("({t} - {}) / 0.9",a.at));let p=w.actor(a.holder);f.x=add(s,&f.x,format!("{} * (1 - {})",p.0-base.0,c(&k)));f.y=add(s,&f.y,format!("{} * (1 - {})",p.1-base.1,c(&k)));sc=mul(s,&sc,format!("0.35 + 0.65 * Math.min(1.08, Math.max(0, {k}))"));f.r=add(s,&f.r,format!("(1 - {}) * -8",c(format!("({t} - {}) / 0.7",a.at))));}let mut from=base;for a in steps.iter().filter(|a|a.verb=="pass"){let to=w.slot(a.to.unwrap());let over=from.0<w.actor(a.holder).0;let t0=a.at+0.3;let t1=a.at+if over{1.35}else{1.15};let on=format!("{t} >= {}",t0-0.25);let u=s.number(format!("({t} - {t0}) / {}",t1-t0));let wu=windup(s,format!("{t} - {t0}"));let direction=if to.0>from.0{1.0}else{-1.0};f.x=add(s,&f.x,format!("{on} ? -10 * {amp} * {wu} * {direction} : 0"));f.sy=mul(s,&f.sy,format!("{on} ? 1 - 0.06 * {amp} * {wu} : 1"));let h:f64=match s.theme.name{"midnight"=>92.0,"signal"=>64.0,"daylight"=>46.0,_=>14.0};let h=if over{h.max(w.size*0.95+40.0)}else{h};let pts:Vec<_>=(0..=32).map(|i|{let v=i as f64/32.0;(from.0+(to.0-from.0)*v,from.1+(to.1-from.1)*v-h*(PI*v).sin()+if s.theme.name=="signal"{18.0*(3.0*PI*v).sin()*(PI*v).sin()}else{0.0})}).collect();let mut lens=vec![0.0];for p in pts.windows(2){lens.push(lens.last().unwrap()+((p[1].0-p[0].0)*(p[1].0-p[0].0)+(p[1].1-p[0].1)*(p[1].1-p[0].1)).sqrt());}let total=lens[32];let b=ease(s,format!("({t} - {t1} - 0.3) / 0.75"));let fraction=if a.blocked{let stop=((from.0+to.0)/2.0-34.0-w.dw/2.0-from.0)/(to.0-from.0);s.number(format!("{stop} * Math.pow({}, 3) * (1 - {b})",c(&u)))}else{ease(s,&u)};let distance=s.number(format!("Math.max(0, Math.min({total}, {fraction} * {total}))"));for axis in 0..2{let mut expr=if axis==0{pts[32].0}else{pts[32].1}.to_string();for j in (1..=32).rev(){let p=if axis==0{pts[j-1].0}else{pts[j-1].1};let delta=if axis==0{pts[j].0-p}else{pts[j].1-p};expr=format!("{distance} <= {} ? {p} + {delta} * ({distance} - {}) / {} : ({expr})",lens[j],lens[j-1],lens[j]-lens[j-1]);}let p=s.number(expr);if axis==0{f.x=add(s,&f.x,format!("{on} ? {p} - {} : 0",from.0));}else{f.y=add(s,&f.y,format!("{on} ? {p} - {} : 0",from.1));}}if a.blocked{let hit=s.number(format!("{t} - {t1}"));let impact=format!("{on} && {hit} > 0 && {hit} < 0.7");f.x=add(s,&f.x,format!("{impact} ? -12 * Math.exp(-6 * {hit}) * Math.sin({} * {hit}) : 0",2.0*PI*6.0));f.sx=mul(s,&f.sx,format!("{impact} ? 1 - 0.12 * Math.exp(-6 * {hit}) : 1"));f.r=add(s,&f.r,format!("{impact} ? 4 * Math.exp(-6 * {hit}) * Math.sin({} * {hit}) : 0",2.0*PI*5.0));sc=mul(s,&sc,format!("{on} && {u} > 0 && {b} < 1 ? 1 - 0.1 * Math.sin({PI} * {}) : 1",c(&u)));}else{sc=mul(s,&sc,format!("{on} && {u} > 0 && {u} < 1 ? 1 - 0.12 * Math.sin({PI} * {u}) : 1"));f.r=add(s,&f.r,format!("{on} && {u} > 0 && {u} < 1 ? 7 * Math.sin({PI} * {u}) * {direction} * (1 - 2 * {u}) : 0"));let rb=rebound(s,format!("{t} - {t1}"));f.sy=mul(s,&f.sy,format!("{on} ? 1 - 0.08 * {amp} * {rb} : 1"));f.sx=mul(s,&f.sx,format!("{on} ? 1 + 0.05 * {amp} * {rb} : 1"));}if !a.blocked{from=to;}}f.x=add(s,&f.x,format!("-{cam}"));for a in steps.iter().filter(|a|matches!(a.verb.as_str(),"approve"|"reject")){let rb=rebound(s,format!("{t} - {} - 0.75",a.at));f.sy=mul(s,&f.sy,format!("1 - 0.1 * {amp} * {rb}"));f.sx=mul(s,&f.sx,format!("1 + 0.06 * {amp} * {rb}"));}f.sx=mul(s,&sc,&f.sx);f.sy=mul(s,&sc,&f.sy);f}
// JSON.stringify uses JS number spelling and enumerates integer keys before other keys.
fn stringify(v:&Value)->String{match v{Value::Number(n)=>{let f=n.as_f64().unwrap();if f==0.0{"0".into()}else{let raw=f.to_string();if (1e-6..1e21).contains(&f.abs()){raw}else{let e=format!("{f:e}");let(a,b)=e.split_once('e').unwrap();let exponent=b.parse::<i32>().unwrap();format!("{a}e{}{exponent}",if exponent>=0{"+"}else{""})}}},Value::Array(v)=>format!("[{}]",v.iter().map(stringify).collect::<Vec<_>>().join(",")),Value::Object(v)=>{let mut keys:Vec<_>=v.keys().collect();keys.sort_by_key(|k|k.parse::<u32>().ok().filter(|i|*i<u32::MAX&&i.to_string()==**k).map(|i|(0,i)).unwrap_or((1,0)));format!("{{{}}}",keys.into_iter().map(|k|format!("{}:{}",q(k),stringify(&v[k]))).collect::<Vec<_>>().join(","))},_=>v.to_string()}}
fn widths(v:&Value,n:usize)->Vec<f64>{let mut a=stringify(v).encode_utf16().fold(2166136261u32,|h,c|(h^c as u32).wrapping_mul(16777619));(0..n).map(|i|{a=a.wrapping_add(0x6D2B79F5);let mut t=(a^(a>>15)).wrapping_mul(a|1);t^=t.wrapping_add((t^(t>>7)).wrapping_mul(t|61));let r=(t^(t>>14))as f64/4294967296.0;if i==n-1{0.38+r*0.2}else{0.62+r*0.34}}).collect()}

pub(super)fn action(s:&mut Scene,v:&Value,brands:&BTreeMap<String,String>)->Result<(),String>{let document=v;let v=&document["action"];let n = v["subject"]["lines"].as_u64().unwrap_or(if v["subject"]["kind"] == "message" {3} else {6}).clamp(2, if v["subject"]["kind"] == "message" {3} else {8}) as usize;let msg=v["subject"]["kind"]=="message";let(actors,mut steps)=steps(v,n)?;let bw=s.w-2.0*s.margin;let narrow=bw<1000.0;let one=actors.len()==1;let k=if narrow{if one{1.15}else{0.82}}else if one{1.5}else{1.15};let w=World{cx:s.w/2.0,cy:{let top=if document["title"].as_str().unwrap_or("").is_empty(){112.0}else{s.top+6.0};top+(s.bottom-top)/2.0-6.0},gap:if narrow{round(bw*0.78)}else{850.0},size:164.0*k,dw:if msg{236.0*k}else{164.0*k},dh:if msg{128.0*k}else{210.0*k},tool:84.0*k.min(1.25),source:132.0*k,off:if one{if narrow{250.0}else{330.0}}else{216.0*k},one,beside:if one&&steps.iter().any(|a|matches!(a.verb.as_str(),"compare"|"weigh")){if narrow{70.0}else{120.0}}else{0.0}};let base = w.slot(steps[0].holder);let mut held_slot=base;for a in &mut steps{a.side=if held_slot.0>=w.actor(a.holder).0{1.0}else{-1.0};if a.verb=="pass"&&!a.blocked{held_slot=w.slot(a.to.unwrap());}}
let mut nodes=heading(s,document);let mut actor_nodes=vec![];let mut label_nodes=vec![];for(i,a)in actors.iter().enumerate(){let p=w.actor(i);let first=steps.iter().filter(|b|b.holder==i||b.to==Some(i)).map(|b|b.at).fold(f64::INFINITY,f64::min);let enter=if i==0{0.2}else{0.35f64.max(first-0.55)};actor_nodes.push(nodes.len());let mut node=box_node(item(a)["t"].as_str().unwrap(),"item",p.0-w.size/2.0,p.1-w.size/2.0,w.size,w.size,enter);{ node["glyph"] = json!(a.icon); node["mod"] = json!(a.modifier); if let Some(brand)=&a.brand {node["brand"] = json!(brand);node["brandBase"] = json!(a.icon);} }nodes.push(node);if !a.label.is_empty(){let lx=if p.0>0.0&&p.0<s.w{(p.0-120.0).clamp(4.0,s.w-244.0)}else{p.0-120.0};label_nodes.push(Some(nodes.len()));let mut node=box_node("actLabel","item",lx,p.1+w.size/2.0+12.0,240.0,32.0*1.3f64.min(k),enter+0.12);node["label"]=json!(a.label);node["fs"]=json!(round(24.0*1.3f64.min(k)));node["lh"]=json!(round(30.0*1.3f64.min(k)));node["align"]=json!("center");nodes.push(node);}else{label_nodes.push(None);}}
let subject=nodes.len();let appear=if steps[0].verb=="draft"{steps[0].at}else{0.3f64.max(steps[0].at-0.7)};nodes.push(box_node(if v["subject"]["tone"]=="warning"{"actDocExc"}else{"actDoc"},"item",base.0-w.dw/2.0,base.1-w.dh/2.0,w.dw,w.dh,appear));let mut tools=vec![];let mut from=base;for(i,a)in steps.iter().enumerate().filter(|(_,a)|a.verb=="pass"){let to=w.slot(a.to.unwrap());if a.blocked{let hit=a.at+if from.0<w.actor(a.holder).0{1.35}else{1.15};let node=box_node("actBan","item",(from.0+to.0)/2.0-34.0,from.1-34.0,68.0,68.0,hit-0.05);tools.push((nodes.len(),i,Some((hit,(from.0+to.0)/2.0,from.1))));nodes.push(node);}else{from=to;}}
for(i,a)in steps.iter().enumerate().filter(|(_,a)|!matches!(a.verb.as_str(),"draft"|"pass")){let size=if a.verb=="compare"{w.source}else{w.tool};tools.push((nodes.len(),i,None));{let mut node=box_node(if a.verb=="compare"{"pictTile"}else{"actTool"},"item",base.0-size/2.0,base.1-size/2.0,size,size,0.1f64.max(a.at-if a.verb=="compare"{0.3}else{0.45}));if let Some(source)=&a.source{node["glyph"]=json!(source.icon);node["mod"]=json!("check");}nodes.push(node);}}if steps[0].verb=="draft"&&actors[0].manner!="machine"{tools.push((nodes.len(),0,None));nodes.push(box_node("actTool","item",base.0-w.tool*0.4,base.1-w.tool*0.4,w.tool*0.8,w.tool*0.8,steps[0].at+0.25));}
let nodes=s.prepare_layout_nodes(document,&nodes,&[])?.0;render_heading(s,&nodes)?;let t=s.number("computed.sourceClock / 1000");let cam=camera(s,&w,&steps,&t);let thing=flight(s,&w,&steps,&t,&cam);let amp=motion(s.theme.name).0;
for(i,a)in actors.iter().enumerate(){
    let node=&nodes[actor_nodes[i]];let mut f=Fx::default();f.x=s.number(format!("-{cam}"));let mut sc="1".to_owned();
    for b in &steps{
        if b.holder==i{
            let e=envelope(s,&t,b.at,b.at+b.dur,0.25,0.3);
            if a.manner=="machine"{f.y=add(s,&f.y,format!("-{e} * 2.5 * Math.abs(Math.sin({} * {t}))",2.0*PI*3.2));}
            else{f.y=add(s,&f.y,format!("-{e} * 1.5 * Math.sin({} * {t})",2.0*PI*1.8));f.r=add(s,&f.r,format!("{e} * 1.6 * Math.sin({} * {t})",2.0*PI*0.9));}
            let wu=windup(s,format!("{t} - {}",b.at));f.sx=mul(s,&f.sx,format!("{e} != 0 ? 1 + 0.03 * {amp} * {wu} : 1"));f.sy=mul(s,&f.sy,format!("{e} != 0 ? 1 - 0.05 * {amp} * {wu} : 1"));
            if b.blocked{let u=s.number(format!("{t} - {} - 1.05",b.at));f.x=add(s,&f.x,format!("{u} > 0 && {u} < 0.7 ? -5 * Math.exp(-6 * {u}) * Math.sin({} * {u}) : 0",2.0*PI*6.0));}
        }
        if b.to==Some(i)&&!b.blocked{let rb=rebound(s,format!("{t} - {} - 1.15",b.at));sc=mul(s,&sc,format!("1 + 0.06 * {amp} * {rb}"));}
    }
    f.sx=mul(s,&sc,&f.sx);f.sy=mul(s,&sc,&f.sy);root(s,&format!("act-actor-motion-{i}"),node,&f);
    branded_node(s,&format!("act-actor-{i}"),&held(&item(a)),node_box(node),true,node["a"].as_f64().unwrap()*1000.0,None,brands)?;
    let busy=steps.iter().filter(|b|b.holder==i&&matches!(b.verb.as_str(),"draft"|"inspect")).map(|b|envelope(s,&t,b.at,b.at+b.dur,0.25,0.3)).collect::<Vec<_>>();
    if !busy.is_empty(){
        let border=match a.tone.as_str(){"accent"=>0.0,"warning"=>2.0,_=>card_border(s)}.to_string();
        svg_box(s,&format!("act-busy-svg-{i}"),node,(100.0,100.0),&border);
        let rect=(node["x"].as_f64().unwrap(),node["y"].as_f64().unwrap(),100.0,100.0);
        let busy=s.number(format!("Math.max(0, {})",busy.join(",")));let op=s.css_number(&format!("{busy} > 0.01 ? {busy} : 0"),3);let bubble=rounded_expr(s,"27","-38","46",26.0,13.0);
        let line_color=role(s,"line");path(s,&format!("act-busy-{i}"),&bubble,rect,(100.0,100.0),s.theme.surface,&line_color,"1.2",&op,"");
        for j in 0..3{let b=s.number(format!("Math.max(0, Math.sin({} * ({t} * {} - {})))",2.0*PI,if a.manner=="machine"{2.6}else{1.6},j as f64*0.18));let dot=d(s,&[("M",(39+j*11).to_string()),(" ",format!("-25 - 3 * {b}"))]);let dot=s.string(format!("{dot} + {}",q("h0.01")));let col=mix(s,s.theme.muted,s.theme.accent,&b);path(s,&format!("act-dot-{i}-{j}"),&dot,rect,(100.0,100.0),"none",&col,"5.2",&op,"");}
        s.end_move();
    }
    end_root(s);
    if let Some(j)=label_nodes[i]{let n=&nodes[j];let name_fx=Fx{x:s.number(format!("-{cam}")),..Default::default()};root(s,&format!("act-name-motion-{i}"),n,&name_fx);let(x,y,w,h)=node_box(n);s.layout_node("actLabel",&format!("act-name-{i}"),&held(n),x,y,w,h,-100000.0,None)?;end_root(s);}
}
root(s,"act-subject-motion",&nodes[subject],&thing);let sn=&nodes[subject];let(x,y,ww,hh)=node_box(sn);s.layout_node(sn["t"].as_str().unwrap(),"act-subject-body",&held(sn),x,y,ww,hh,-100000.0,None)?;draw_subject(s,v,&steps,&actors,n,msg,sn,&t)?;end_root(s);
for(ni,si,ban)in tools{draw_tool(s,&nodes[ni],ni,&steps[si],ban,&w,&thing,&nodes[subject],n,&t,&cam,brands)?;}Ok(())}

fn pick(s:&mut Scene,index:&str,values:&[f64])->String{let mut e=values.last().unwrap_or(&0.0).to_string();for(i,v)in values.iter().enumerate().rev().skip(1){e=format!("{index} == {i} ? {v} : ({e})");}s.number(e)}
fn draw_tool(s:&mut Scene,node:&Value,i:usize,a:&Step,ban:Option<(f64,f64,f64)>,w:&World,thing:&Fx,sn:&Value,n:usize,t:&str,cam:&str,brands:&BTreeMap<String,String>)->Result<(),String>{let amp=motion(s.theme.name).0;let bx=s.number(format!("{} + {}",sn["x"].as_f64().unwrap(),thing.x));let by=s.number(format!("{} + {}",sn["y"].as_f64().unwrap(),thing.y));let mut x = "0".to_owned();let mut y = "0".to_owned();let mut sc = "1".to_owned();let mut rot = "0".to_owned();let mut op=envelope(s,t,a.at-0.15,a.at+a.dur-0.05,0.3,0.35);let side=a.side;
if let Some((hit,xx,yy))=ban{let u=s.number(format!("{t} - {hit}"));x=s.number(format!("{xx} - {cam}"));y=yy.to_string();op=s.number(c(format!("{u} * 8")));let k=spring(s,&u);sc=s.number(format!("Math.max(0, {k})"));let rb=rebound(s,&u);rot=s.number(format!("10 * {amp} * {rb}"));}else{match a.verb.as_str(){"inspect"=>{let u=s.number(c(format!("({t} - {} - 0.25) / {}",a.at,a.dur-0.5)));x=s.number(format!("{bx} + {} * (0.52 + 0.2 * Math.sin({} * {u}))",w.dw,3.0*PI));y=s.number(format!("{by} + {} + {u} * {}",line_y(0,n,w.dh,false),line_y(n-1,n,w.dh,false)-line_y(0,n,w.dh,false)));sc=s.number(format!("0.9 + 0.1 * {op}"));},"approve"|"reject"=>{let hit=a.at+0.75;let pre=s.number(c(format!("({t} - {} + 0.15) / 0.5",a.at)));let up=windup(s,format!("{t} - {hit}"));let after=s.number(c(format!("({t} - {hit} - 0.3) / 0.45")));x=s.number(format!("{bx} + {} + 18",w.dw/2.0));let descent=c(format!("({t} - {hit} + 0.18) / 0.18"));y=s.number(format!("{t} >= {} && {t} < {hit} ? {by} + {} - 40 * (1 - ({t} - {hit} + 0.18) / 0.18) : {by} + {} - 150 * Math.pow(1 - {pre}, 3) - 26 * {up} - 70 * (1 - Math.pow(1 - {after}, 3)) + ({t} < {hit} ? -40 * (1 - {descent}) : 0)",hit-0.18,w.dh/2.0,w.dh/2.0));op=s.number(format!("{} * (1 - {after})",c(format!("{pre} * 2"))));let rb=rebound(s,format!("{t} - {hit}"));sc=s.number(format!("1 - 0.1 * {rb}"));rot=s.number(format!("-8 * (1 - {pre}) + 6 * {up}"));},"compare"=>{let u=s.number(format!("{t} - {}",a.at-0.1));let k=spring(s,format!("{u} / 0.8"));let out=s.number(c(format!("({t} - {} - 0.35) / 0.4",a.at+a.dur)));let per=(a.dur-0.7)/n as f64;let mut pul="0".to_owned();for q in 0..n{let v=s.number(format!("{t} - {}",a.at+0.5+q as f64*per));pul=s.number(format!("Math.max({pul}, ({v} > 0 && {v} < 0.4 ? Math.sin({PI} * {v} / 0.4) : 0))"));}x=s.number(format!("{bx} + {} + {side} * ({} + 60 * (1 - {}))",w.dw/2.0,w.dw/2.0+46.0+w.source/2.0,c(&k)));y=s.number(format!("{by} + {} - 10",w.dh/2.0));op=s.number(format!("{} * (1 - {out})",c(format!("{u} * 4"))));sc=s.number(format!("(0.6 + 0.4 * Math.min(1.06, Math.max(0, {k}))) * (1 + 0.05 * {pul}) * (1 - 0.3 * {out})"));rot=s.number(format!("-4 * {side} * (1 - {})",c(format!("{u} / 0.8"))));},"weigh"=>{let u=s.number(format!("{t} - {}",a.at));let e=ease(s,format!("{u} / 1.4"));x=s.number(format!("{bx} + {}",w.dw/2.0+side*(w.dw/2.0+92.0)));y=s.number(format!("{by} + {} - 8 * Math.sin({PI} * {})",w.dh*0.42,c(format!("{u} / 0.6"))));op=envelope(s,t,a.at-0.1,a.at+a.dur+0.6,0.3,0.4);rot=s.number(format!("({} * {e} + 26 * Math.exp(-2.2 * {u}) * Math.sin({} * {u})) * {} * {}",if a.risk{16}else{0},2.0*PI*1.1,c(format!("{u} * 3")),-side));sc="1.6".into();},"fake"|"fill"|"restyle"=>{let per=match a.verb.as_str(){"fake"=>a.dur/a.own.len()as f64,"fill"=>0.75,_=>0.6};let elapsed=if a.verb=="fake"{format!("{t} - {}",a.at)}else{format!("{t} - {} - 0.25",a.at)};let idx=s.number(format!("Math.min({}, Math.max(0, Math.floor(({elapsed}) / {per})))",a.own.len()-1));let u=s.number(c(format!("({elapsed} - {idx} * {per}) / {per}")));let ys=a.own.iter().map(|&l|line_y(l,n,w.dh,false)).collect::<Vec<_>>();let yy=pick(s,&idx,&ys);if a.verb=="fake"{let prev=s.number(format!("Math.max(0, {idx} - 1)"));let py=pick(s,&prev,&ys);let mv=ease(s,format!("{u} / 0.3"));x=s.number(format!("{bx} + {} * (0.42 + 0.12 * Math.sin({PI} * {}))",w.dw,c(format!("{u} / 0.45"))));y=s.number(format!("{by} + {py} + ({yy} - {py}) * {mv}"));sc=s.number(format!("0.9 + 0.1 * {op} + 0.12 * Math.sin({PI} * {})",c(format!("({u} - 0.3) / 0.25"))));}else{let f=c(format!("({elapsed} - {idx} * {per}) / {}",per*0.8));x=s.number(format!("{bx} + {} + ({f}) * {}",w.dw*0.13,w.dw*0.7));y=s.number(format!("{by} + {yy} - 6"));rot=s.number(format!("4 * Math.sin({} * {t})",2.0*PI*4.0));}},_=>{let write=a.verb=="draft";let k=s.number(c(if write{format!("({t} - {} - 0.2) / {}",a.at,a.dur-0.2)}else{format!("({t} - {}) / {}",a.at,a.dur)}));let len=if write{n}else{a.own.len()};let idx=s.number(format!("Math.min({}, Math.floor({k} * {len}))",len-1));let ys=(0..len).map(|j|line_y(if write{j}else{a.own[j]},n,w.dh,false)).collect::<Vec<_>>();let yy=pick(s,&idx,&ys);let frac=s.number(format!("{k} * {len} - Math.floor({k} * {len})"));x=s.number(format!("{bx} + 16 + {frac} * {}",w.dw-if write{40.0}else{32.0}));y=s.number(format!("{by} + {yy}"));if write{rot=s.number(format!("4 * Math.sin({} * {t})",2.0*PI*4.0));}}}}
let(nx,ny,nw,nh)=node_box(node);let fx=Fx{x:s.number(format!("{x} - {}",nx+nw/2.0)),y:s.number(format!("{y} - {}",ny+nh/2.0)),sx:sc.clone(),sy:sc,r:rot,o:op};root(s,&format!("act-tool-motion-{i}"),node,&fx);
if let Some(source)=&a.source{
    let mut v=held(&item(source));v["mod"]=json!("check");v["t"]=json!("pictTile");v.as_object_mut().unwrap().remove("brand");
    branded_node(s,&format!("act-tool-{i}"),&v,node_box(node),true,node["a"].as_f64().unwrap()*1000.0,None,brands)?;
}else{
    let ty=if ban.is_some(){"actBan"}else{"actTool"};s.layout_node(ty,&format!("act-tool-card-{i}"),&held(node),nx,ny,nw,nh,-100000.0,None)?;
    let icon=if ban.is_some(){"ban"}else{match a.verb.as_str(){"inspect"|"fake"=>"search","approve"|"reject"=>"stamp","weigh"=>"legal",_=>"pen"}};
    let size=round(nw*0.5/24.0*1000.0)/1000.0*24.0;let off=round(nw*0.25*10.0)/10.0;
    let warning=if ban.is_some(){"1".into()}else if icon=="legal"&&a.risk{s.number(format!("{t} > {} ? 1 : 0",a.at+1.2))}else{"0".into()};
    for(layer,col)in[(0,THEMES[s.theme.name]["focusIcon"].as_str().unwrap()),(1,s.theme.warning)]{let vis=s.number(format!("{warning} == {layer} ? 1 : 0"));opacity(s,&format!("act-tool-color-{i}-{layer}"),&vis);glyph(s,&format!("act-tool-icon-{i}-{layer}"),icon,nx+off,ny+off,size,col,if ban.is_some(){2.6}else{2.3});s.end_move();}
}
end_root(s);Ok(())}

#[derive(Clone)]struct Line{prog:String,shift:String,gone:String,slot:String,zig:String,fake:String,dash:String,flag:String,flash:String,tick:String,bad:String,strike:String,black:bool}
fn draw_subject(s:&mut Scene,v:&Value,steps:&[Step],actors:&[Actor],n:usize,msg:bool,node:&Value,t:&str)->Result<(),String>{let bw=if msg{236.0}else{164.0};let bh=if msg{128.0}else{210.0};let x0=round(bw*0.13);let lw=bw-2.0*x0;let lengths=widths(v,n);let made=steps.first().filter(|a|a.verb=="draft");let mut lines=vec![];for i in 0..n{let prog=if let Some(a)=made{let machine=actors[a.holder].manner=="machine";let per=if machine{0.17}else{0.5};s.number(c(format!("({t} - {} - {}) / {}",a.at+if machine{0.5}else{0.65},i as f64*per,per*if machine{0.9}else{0.85})))}else{"1".into()};lines.push(Line{prog,shift:"0".into(),gone:"0".into(),slot:"0".into(),zig:"0".into(),fake:"0".into(),dash:"0".into(),flag:"0".into(),flash:"0".into(),tick:"0".into(),bad:"0".into(),strike:"0".into(),black:false});}let draft=lines.iter().map(|l|l.prog.clone()).collect::<Vec<_>>();let mut risk = "0".to_owned();let mut calm = "0".to_owned();for a in steps{match a.verb.as_str(){"fill"|"restyle"|"fake"=>for(q,&i)in a.own.iter().enumerate(){let l=&mut lines[i];if a.verb=="fake"{let at=a.at+(q as f64+0.35)*a.dur/a.own.len()as f64;l.dash=s.number(c(format!("({t} - {at}) / 0.45")));l.fake=s.number(c(format!("({t} - {at} - 0.5) / 0.5")));}else{let fill=a.verb=="fill";let at=a.at+0.25+q as f64*if fill{0.75}else{0.6};let dur=if fill{0.55}else{0.5};let k=s.number(c(format!("({t} - {at}) / {dur}")));if fill{l.prog=k.clone();l.slot=s.number(format!("{} * (1 - {k})",draft[i]));}else{let e=ease(s,&k);l.zig=s.number(format!("1 - {e}"));}let flash=format!("{k} >= 1 ? 1 - {} : {}",c(format!("({t} - {at} - {dur}) / 0.6")),if fill{format!("{k} > 0 ? 1 : 0")}else{"0".into()});l.flash=s.number(format!("Math.max({}, ({flash}))",l.flash));}},"compare"=>for(i,l)in lines.iter_mut().enumerate(){let at=a.at+0.5+i as f64*(a.dur-0.7)/n as f64;let k=s.number(c(format!("({t} - {at}) / 0.3")));l.tick=s.number(format!("{k} > 0 ? {k} : {}",l.tick));let bad=a.own.contains(&i);l.bad=s.number(format!("{k} > 0 ? {} : {}",u8::from(bad),l.bad));if bad{l.flag=s.number(format!("Math.max({}, {k})",l.flag));}else{l.flash=s.number(format!("{k} > 0 ? Math.max({}, {k} < 1 ? {k} : 1 - {}) : {}",l.flash,c(format!("({t} - {} - 0.8 - {}) / 0.5",a.at,i as f64*(a.dur-0.7)/n as f64)),l.flash));}},"weigh"=>{let k=s.number(c(format!("({t} - {} - 1.2) / 0.5",a.at)));if a.risk{risk=s.number(format!("Math.max({risk}, {k})"));}else{calm=s.number(format!("Math.max({calm}, {k} * (1 - {}))",c(format!("({t} - {} - 2.2) / 0.8",a.at))));}},_=>{}}}
for a in steps{if a.verb=="inspect"{let u=s.number(format!("{} * {}",c(format!("({t} - {} - 0.25) / {}",a.at,a.dur-0.5)),n-1));for(i,l)in lines.iter_mut().enumerate(){let dk=s.number(format!("{u} - {i}"));if a.own.contains(&i){l.flag=s.number(format!("{dk} >= 0 ? Math.max({}, {}) : {}",l.flag,c(format!("{dk} / 0.35")),l.flag));}else{l.flash=s.number(format!("{dk} >= 0 ? Math.max({}, {dk} < 0.6 ? 1 - {dk} / 0.6 : 0) : {}",l.flash,l.flash));}}}if matches!(a.verb.as_str(),"strike"|"redact"){for(q,&i)in a.own.iter().enumerate(){let at=a.at+q as f64*0.7;lines[i].strike=s.number(c(format!("({t} - {at}) / 0.55")));lines[i].black=a.verb=="redact";if a.verb=="strike"{let gone=s.number(c(format!("({t} - {at} - 0.7) / 0.45")));lines[i].gone=gone.clone();for l in lines.iter_mut().skip(i+1){l.shift=add(s,&l.shift,format!("-{gone} * {}",line_y(1,n,bh,msg)-line_y(0,n,bh,msg)));}}}}}
let border=if node["t"]=="actDocExc"{2.0}else{card_border(s)}.to_string();svg_box(s,"act-subject-svg",node,(bw,bh),&border);
let rect=(node["x"].as_f64().unwrap(),node["y"].as_f64().unwrap(),bw,bh);let vb=(bw,bh);let ink=if v["subject"]["tone"]=="warning"{s.theme.warning}else{s.theme.ink};if !msg{let icon=v["subject"]["icon"].as_str().map(alias).filter(|v| ICONS.get(*v).is_some());let hx=x0+if icon.is_some(){30.0}else{0.0};if let Some(icon)=icon{let(nx,ny,nw,nh)=rect;glyph(s,"act-subject-header-icon",icon,nx+x0*nw/bw,ny+round((bh*0.13-11.0)*10.0)/10.0*nh/bh,22.08*nw/bw,if v["subject"]["tone"]=="warning"{s.theme.warning}else{s.theme.accent},2.2);}let hp=made.map(|a|s.number(c(format!("({t} - {} - 0.35) / 0.25",a.at)))).unwrap_or("1".into());let hd=d(s,&[("M",hx.to_string()),(" ",(bh*0.13).to_string()),("H",format!("{hx} + {} * {hp}",lw*0.55-(hx-x0)))]);let op=s.number(format!("{hp} > 0 ? 1 : 0"));path(s,"act-header",&hd,rect,vb,"none",ink,"5.5",&op,"");}
let line_color=role(s,"line");let warning_tint=role(s,"exceptionTint");
for(i,l)in lines.iter().enumerate(){let y=s.number(format!("{} + {}",line_y(i,n,bh,msg),l.shift));let full=lw*lengths[i];let len=s.number(format!("{full} * {}",l.prog));let op=s.number(format!("1 - {}",l.gone));let guard=format!("{op} > 0.01");let rr=rounded_expr(s,&(x0-5.0).to_string(),&format!("{y} - 9"),&(full+10.0).to_string(),18.0,9.0);let slot=s.css_number(&format!("{guard} && {} > 0.01 ? {} * {op} : 0",l.slot,l.slot),3);path(s,&format!("act-slot-{i}"),&rr,rect,vb,"none",s.theme.muted,"2",&slot,"7 7");let fake=s.css_number(&format!("{guard} && {} > 0 ? {} * 0.85 : 0",l.fake,l.fake),3);path(s,&format!("act-fake-slot-{i}"),&rr,rect,vb,&warning_tint,s.theme.warning,"2",&fake,"7 7");let rr=rounded_expr(s,&(x0-7.0).to_string(),&format!("{y} - 9"),&(full+14.0).to_string(),18.0,9.0);let flag=s.css_number(&format!("{guard} ? {} * {op} : 0",l.flag),3);path(s,&format!("act-flag-{i}"),&rr,rect,vb,&warning_tint,"none","0",&flag,"");let rop=s.css_number(&format!("{guard} ? 0.55 * {risk} * {op} : 0"),3);path(s,&format!("act-risk-{i}"),&rr,rect,vb,&warning_tint,"none","0",&rop,"");let lop=s.number(format!("{op} * (1 - {})",l.fake));let visible=s.css_number(&format!("{guard} && {len} > 0.5 && {lop} > 0.01 ? {lop} : 0"),3);let clean=d(s,&[("M",x0.to_string()),(" ",y.clone()),("H",format!("{x0} + {len}"))]);let mut teeth=vec![q("M"),x0.to_string(),q(" "),s.css_number(&y,1)];let mut zx=9.0;let mut zi=0;while zx<full{let yy=s.css_number(&format!("{y} + {} * {}",if zi%2==0{1.0}else{-1.0}*6.0*(0.7+0.3*((zi*7)%3)as f64/2.0),l.zig),1);teeth.push(format!("({len} > {zx} ? {} + {} + {} + {yy} : {})",q("L"),x0+zx,q(" "),q("")));zx+=9.0;zi+=1;}teeth.extend([q("L"),s.css_number(&format!("{x0} + {len}"),1),q(" "),s.css_number(&y,1)]);let rough=s.string(teeth.join(" + "));let dl=s.string(format!("{} > 0.01 ? {rough} : {clean}",l.zig));let warning=s.number(format!("{} > 0 ? {} : {} > 0 ? {} : {} > 0 ? 0.45 * {} : 0",l.flag,l.flag,l.dash,l.dash,l.zig,l.zig));let accent=s.number(format!("{warning} > 0 ? 0 : {} > 0 ? {} : {calm}",l.flash,l.flash));let wc=mix(s,&line_color,s.theme.warning,&warning);let ac=mix(s,&line_color,s.theme.accent,&accent);let color=s.string(format!("{warning} > 0 ? {wc} : {ac}"));let dw=s.css_number(&format!("10 - 5 * {}",l.dash),1);let dh=s.css_number(&format!("2 + 12 * {}",l.dash),1);let dash=s.string(format!("{} > 0 ? {dw} + {} + {dh} : {}",l.dash,q(" "),q("")));let sw=s.number(format!("{} > 0 ? 4.2 : 3.6",l.flag));path(s,&format!("act-line-{i}"),&dl,rect,vb,"none",&color,&sw,&visible,&dash);
let tx=(bw-12.0).min(x0+full+18.0);let tsc=back(s,&l.tick);let good=d(s,&[("M",format!("{tx} - 7 * {tsc}")),(" ",y.clone()),("l",format!("5 * {tsc}")),(" ",format!("5 * {tsc}")),("l",format!("9 * {tsc}")),(" ",format!("-10 * {tsc}"))]);let bad=d(s,&[("M",format!("{tx} - 6 * {tsc}")),(" ",format!("{y} - 6 * {tsc}")),("l",format!("12 * {tsc}")),(" ",format!("12 * {tsc}")),("M",format!("{tx} + 6 * {tsc}")),(" ",format!("{y} - 6 * {tsc}")),("l",format!("-12 * {tsc}")),(" ",format!("12 * {tsc}"))]);let tick=s.string(format!("{} > 0 ? {bad} : {good}",l.bad));let col=s.string(format!("{} > 0 ? {} : {}",l.bad,q(s.theme.warning),q(s.theme.accent)));let top=s.css_number(&format!("{guard} ? {} : 0",c(format!("{} * 3",l.tick))),3);path(s,&format!("act-tick-{i}"),&tick,rect,vb,"none",&col,"3.6",&top,"");let fx=(bw-9.0).min(x0+full+16.0);let mark=d(s,&[("M",fx.to_string()),(" ",format!("{y} - 7"))]);let end=d(s,&[(" M",fx.to_string()),(" ",format!("{y} + 6"))]);let mark=s.string(format!("{mark} + {} + {end} + {}",q("v8"),q("v0.5")));let fop=s.css_number(&format!("{guard} && {} <= 0 ? {} : 0",l.bad,l.flag),3);path(s,&format!("act-flag-mark-{i}"),&mark,rect,vb,"none",s.theme.warning,"3.6",&fop,"");if l.black{let rd=rounded_expr(s,&(x0-4.0).to_string(),&format!("{y} - 6"),&format!("{} * {}",full+8.0,l.strike),12.0,3.0);let show=s.number(format!("{guard} && {} > 0 ? 1 : 0",l.strike));path(s,&format!("act-redaction-{i}"),&rd,rect,vb,s.theme.ink,"none","0",&show,"");}else{let rd=d(s,&[("M",(x0-3.0).to_string()),(" ",y.clone()),("H",format!("{} + {} * {}",x0-3.0,full+6.0,l.strike))]);let show=s.css_number(&format!("{guard} && {} > 0 ? {op} : 0",l.strike),3);path(s,&format!("act-redaction-{i}"),&rd,rect,vb,"none",s.theme.warning,"2.4",&show,"");}}
for(i,a)in steps.iter().enumerate().filter(|(_,a)|matches!(a.verb.as_str(),"approve"|"reject")){
    let hu=s.number(format!("{t} - {} - 0.75",a.at));let k=s.number(c(format!("{hu} / 0.22")));let bk=back(s,&k);
    let sc=s.css_number(&format!("1.35 - 0.35 * {bk}"),3);let op=s.css_number(&format!("{hu} >= 0 ? {} : 0",c(format!("{k} * 3"))),3);
    let(nx,ny,nw,nh)=rect;let cx=round(bw/2.0*10.0)/10.0;let cy=round(bh*0.6*10.0)/10.0;
    let stampw=bw*0.76;let stamph=round(bw*0.3);let left=round(-stampw/2.0*10.0)/10.0;let top=round(-stamph/2.0*10.0)/10.0;
    frame(s,&format!("act-imprint-transform-{i}"),&sc,&sc,"-9",&op,nx+cx*nw/bw,ny+cy*nh/bh);
    let stamprect=(nx+(cx+left)*nw/bw,ny+(cy+top)*nh/bh,stampw*nw/bw,stamph*nh/bh);let stampvb=(stampw,stamph);
    let ok=a.verb=="approve";let col=if ok{s.theme.accent}else{s.theme.warning};let tint=role(s,if ok{"accentTint"}else{"exceptionTint"});
    let rd=rounded_expr(s,"0","0",&stampw.to_string(),stamph,9.0);
    path(s,&format!("act-imprint-fill-{i}"),&rd,stamprect,stampvb,&tint,"none","0","0.92","");
    path(s,&format!("act-imprint-{i}"),&rd,stamprect,stampvb,"none",col,"3","1","");
    let z=stamph/46.0;let mark=if ok{d(s,&[("M",(stampw/2.0-13.0*z).to_string()),(" ",(23.0*z).to_string()),("l",(9.0*z).to_string()),(" ",(9.0*z).to_string()),(" ",(17.0*z).to_string()),(" ",(-18.0*z).to_string())])}else{d(s,&[("M",(stampw/2.0-11.0*z).to_string()),(" ",(12.0*z).to_string()),("l",(22.0*z).to_string()),(" ",(22.0*z).to_string()),(" M",(stampw/2.0+11.0*z).to_string()),(" ",(12.0*z).to_string()),("l",(-22.0*z).to_string()),(" ",(22.0*z).to_string())])};
    path(s,&format!("act-imprint-glyph-{i}"),&mark,stamprect,stampvb,"none",col,"4.2","1","");s.end_move();
}s.end_move();Ok(())}

static CATALOGUE:LazyLock<Value>=LazyLock::new(||serde_json::from_str(include_str!("pictures.json")).unwrap());
static ICONS:LazyLock<Value>=LazyLock::new(||serde_json::from_str(include_str!("icons.json")).unwrap());
static THEMES:LazyLock<Value>=LazyLock::new(||serde_json::from_str(include_str!("themes.json")).unwrap());

fn alias(name:&str)->&str{match name{
    "doc"=>"document","user"|"person"=>"human","users"|"team"|"people"=>"group",
    "company"=>"building","computer"|"laptop"=>"machine","agent"=>"bot","ai"=>"spark",
    "bolt"=>"rocket","commit"=>"check","approval"=>"stamp","prohibited"=>"ban","scales"=>"legal",_=>name
}}
pub(super)fn actor_icon(name:&str)->&str{if CATALOGUE["BASES"].get(name).is_some(){name}else{alias(name)}}
fn short(value:&str,len:usize)->String{String::from_utf16_lossy(&value.encode_utf16().take(len).collect::<Vec<_>>())}
fn words(value:&Value,len:usize)->String{short(&value.as_str().unwrap_or("").split_whitespace().collect::<Vec<_>>().join(" "),len)}
fn source_ids(v:&Value)->Vec<String>{if let Some(v)=v.as_str(){vec![v.into()]}else{v.as_array().map(|v|v.iter().filter_map(Value::as_str).map(str::to_owned).collect()).unwrap_or_default()}}
fn source_actor(v:&Value)->Result<Actor,String>{
    let mut a=actor(v)?;
    a.label=short(&a.label,22);
    a.brand=a.brand.map(|v|short(&v,40));
    Ok(a)
}
fn heading(s:&Scene,doc:&Value)->Vec<Value>{
    let title=doc["title"].as_str().unwrap_or("");
    if title.is_empty(){return vec![];}
    let bw=s.w-2.0*s.margin;
    let(fs,lines)=if s.tall{crate::typography::fit_block(title, &s.theme, 52.0, 36.0, 2, bw)}else{(crate::typography::fit_one_line(title, &s.theme, 52.0, 36.0, bw),1)};
    let lh=if s.tall{round(fs*1.12)}else{60.0};
    let h=if s.tall{lines as f64*lh}else{64.0};
    vec![json!({"t":"heading","grp":"heading","x":s.margin,"y":if s.tall{s.top-36.0-h}else{60.0},"w":bw,"h":h,"label":title,"icon":"","fs":fs,"lh":lh,"align":if s.tall{"center"}else{"left"}})]
}
fn render_heading(s:&mut Scene,nodes:&[Value])->Result<(),String>{
    for n in nodes.iter().filter(|n|n["t"]=="heading"){
        root(s,"narrative-heading-motion",n,&Fx::default());
        let(x,y,w,h)=node_box(n);
        s.layout_node("heading","narrative-heading-body",&held(n),x,y,w,h,-100000.0,None)?;
        end_root(s);
    }
    Ok(())
}
fn glyph(s:&mut Scene,id:&str,icon:&str,x:f64,y:f64,size:f64,color:&str,stroke:f64){
    if let Some(data)=ICONS.get(icon).and_then(Value::as_str){path(s,id,data,(x,y,size,size),(24.0,24.0),"none",color,&stroke.to_string(),"1","");}
}
fn circle(s:&mut Scene,x:&str,y:&str,r:f64)->String{
    let out=d(s,&[("M",format!("({x}) - {r}")),(" ",y.into()),(&format!("A{r} {r} 0 1 0 "),format!("({x}) + {r}")),(" ",y.into()),(&format!("A{r} {r} 0 1 0 "),format!("({x}) - {r}")),(" ",y.into())]);
    s.string(format!("{out} + {}",q("Z")))
}

#[derive(Clone)]
struct Entity{label:String,icon:String,who:String,sub:String,brand:String}
#[derive(Clone,Copy)]
struct Pose{x:f64,y:f64,scale:f64,opacity:f64}
#[derive(Clone,Copy)]
struct Move{at:f64,pose:Pose,fade:bool}
#[derive(Clone)]
struct State{at:f64,kind:String}
#[derive(Clone)]
struct Content{at:f64,entity:String}
struct Card{born:f64,moves:Vec<Move>,content:Vec<Content>,flips:Vec<f64>,states:Vec<State>,hits:Vec<State>,clone:bool}
#[derive(Clone)]
struct Board{entity:String,side:i8,apart:bool,back:bool}
struct Token{entity:String,at:f64,hops:Vec<(usize,f64)>,gone:Option<f64>}
struct End{entity:String,at:f64,by:Option<String>,sub:f64}
struct Story{entities:BTreeMap<String,Entity>,cards:Vec<Card>,token:Option<Token>,end:Option<End>}

fn arrangement(s:&Scene,board:&[Board],cy:f64)->BTreeMap<String,Pose>{
    let mut out=BTreeMap::new();let n=board.len();if n==0{return out;}
    let split=board.iter().any(|b|b.side==1);let apart=board.iter().any(|b|b.apart);
    let bw=s.w-2.0*s.margin;let cx=s.w/2.0;
    if s.tall{
        let groups:Vec<Vec<&Board>>=(0..2).map(|group|board.iter().filter(|b|if split{(b.side==1)==(group==1)}else{b.apart==(group==1)}).collect()).filter(|g:&Vec<&Board>|!g.is_empty()).collect();
        let rows:Vec<_>=groups.iter().enumerate().flat_map(|(i,g)|g.chunks(2).map(move|r|(i,r))).collect();
        let r=rows.len()as f64;let gg=if groups.len()>1{70.0}else{0.0};
        let scale=1.0f64.min((s.bottom-s.top-150.0-(r-1.0)*84.0-(groups.len()-1)as f64*gg)/r/236.0).min(if rows.iter().any(|(_,row)|row.len()>1){(bw-40.0)/2.0/232.0}else{1.0});
        let height=236.0*scale;let width=232.0*scale;
        let total=r*height+(r-1.0)*84.0+(groups.len()-1)as f64*gg;
        let mut y=cy-total/2.0+height/2.0;
        for(i,(group,row))in rows.iter().enumerate(){
            if i>0&&*group!=rows[i-1].0{y+=gg;}
            for(j,b)in row.iter().enumerate(){out.insert(b.entity.clone(),Pose{x:cx+(j as f64-(row.len()-1)as f64/2.0)*(width+40.0),y,scale:scale*if b.back{0.88}else{1.0},opacity:if b.back{0.55}else{1.0}});}
            y+=height+84.0;
        }
    }else{
        let gap=if n>=4{52.0}else{70.0};let big=if split{150.0}else if apart{130.0}else{0.0};
        let scale=if n==1{1.0}else{(if n<=3{1.0f64}else{0.92}).min((bw-gap*(n-1)as f64-big)/n as f64/232.0)};
        let width=232.0*scale;let total=n as f64*width+(n-1)as f64*gap+big;
        let mut x=cx-total/2.0+width/2.0;
        for(i,b)in board.iter().enumerate(){
            if i>0&&((split&&b.side==1&&board[i-1].side!=1)||(b.apart&&!board[i-1].apart)){x+=big;}
            out.insert(b.entity.clone(),Pose{x,y:cy-if n>1{6.0}else{0.0}+if b.apart{18.0}else{0.0},scale:scale*if b.back{0.88}else{1.0},opacity:if b.back{0.55}else{1.0}});
            x+=width+gap;
        }
    }
    out
}
fn rebound_at(t:f64,theme:&str)->f64{let(_,f,d,_)=motion(theme);if t<=0.0{0.0}else{(-d*t).exp()*(2.0*PI*f*t).sin()}}
fn position_at(card:&Card,t:f64,theme:Option<&str>)->Pose{
    let mut p=card.moves[0].pose;
    for(i,m)in card.moves.iter().enumerate().skip(1){
        let u=clamp((t-m.at)/0.85);if u<=0.0{break;}
        let e=if m.fade{u.powi(3)}else{io(u)};
        p=Pose{x:p.x+(m.pose.x-p.x)*e,y:p.y+(m.pose.y-p.y)*e-if card.clone&&i==1{46.0*(PI*u).sin()}else{0.0},scale:p.scale+(m.pose.scale-p.scale)*e,opacity:p.opacity+(m.pose.opacity-p.opacity)*e};
        if let Some(theme)=theme{if !m.fade&&u>=1.0{p.scale*=1.0+0.03*motion(theme).0*rebound_at(t-m.at-0.85,theme);}}
    }
    p
}
fn new_card(cards:&mut Vec<Card>,by:&mut BTreeMap<String,usize>,entity:&str,at:f64,mut pose:Pose)->usize{
    pose.opacity=1.0;let id=cards.len();
    cards.push(Card{born:at,moves:vec![Move{at,pose,fade:false}],content:vec![Content{at,entity:entity.into()}],flips:vec![],states:vec![],hits:vec![],clone:false});
    by.insert(entity.into(),id);id
}
fn state(card:&mut Card,at:f64,kind:&str){let event=State{at,kind:kind.into()};card.states.push(event.clone());card.hits.push(event);}
fn place(s:&Scene,board:&[Board],cards:&mut[Card],by:&BTreeMap<String,usize>,at:f64,born:&[String],cy:f64)->BTreeMap<String,Pose>{
    let a=arrangement(s,board,cy);
    for b in board{if !born.contains(&b.entity){if let Some(id)=by.get(&b.entity){cards[*id].moves.push(Move{at,pose:a[&b.entity],fade:false});}}}
    a
}
fn story_plan(s:&Scene,doc:&Value,cy:f64)->Story{
    let spec=&doc["story"];
    // STORY.canon preserves real ICON_SET entries even when an actor alias exists.
    let entities=spec["entities"].as_object().unwrap().iter().take(14).map(|(id,e)|{let icon=e["icon"].as_str().unwrap_or("");(id.clone(),Entity{label:words(&e["label"],30),icon:if ICONS.get(icon).is_some(){icon}else{alias(icon)}.into(),who:words(&e["who"],26),sub:words(&e["sub"],40),brand:words(&e["brand"],40)})}).collect();
    let mut beats:Vec<_>=spec["beats"].as_array().unwrap().iter().filter(|b|b["at"].is_number()).collect();
    beats.sort_by(|a,b|a["at"].as_f64().unwrap().total_cmp(&b["at"].as_f64().unwrap()));
    let(mut cards,mut by,mut board,mut token,mut end)=(vec![],BTreeMap::new(),Vec::<Board>::new(),None,None);
    let duration=s.duration/1000.0;
    for b in beats{
        let at=b["at"].as_f64().unwrap()-s.start;
        let rel=b["rel"].as_str().unwrap();let of=b["of"].as_str().unwrap_or("");
        let to=source_ids(&b["to"]);
        match rel{
            "rule"=>{
                let on=source_ids(&b["on"]);
                for entity in &on{board.push(Board{entity:entity.clone(),side:0,apart:false,back:false});}
                let a=arrangement(s,&board,cy);
                for(i,entity)in on.iter().enumerate(){new_card(&mut cards,&mut by,entity,(at-0.15).max(0.2)+0.3*i as f64,a[entity]);}
            },
            "example"=>{
                let id=by.remove(of).unwrap();let target=&to[0];by.insert(target.clone(),id);
                for node in &mut board{if node.entity==of{node.entity=target.clone();}}
                let card=&mut cards[id];card.flips.push(at);card.content.push(Content{at:at+0.31,entity:target.clone()});
                if !card.states.is_empty(){card.states.push(State{at:at+0.31,kind:"none".into()});}
            },
            "also"|"except"|"because"=>{
                let source=by[of];let index=board.iter().position(|n|n.entity==of).unwrap();
                let add:Vec<_>=to.iter().map(|entity|Board{entity:entity.clone(),side:board[index].side,apart:rel=="except",back:false}).collect();
                if rel=="because"{board.insert(index,add[0].clone());}else if rel=="except"{board.push(add[0].clone());}else{board.splice(index+1..index+1,add);}
                let a=place(s,&board,&mut cards,&by,at,&to,cy);
                let old=cards[source].states.last().cloned();
                for(i,target)in to.iter().enumerate(){
                    let born=at+0.45+0.42*i as f64;let p=position_at(&cards[source],born,None);
                    let id=new_card(&mut cards,&mut by,of,born,p);let card=&mut cards[id];card.clone=true;
                    card.moves.push(Move{at:born,pose:a[target],fade:false});card.flips.push(born+0.12);card.content.push(Content{at:born + 0.12 + 0.62 / 2.0,entity:target.clone()});
                    if rel=="also"{if let Some(st)=&old{card.states.push(State{at:born,kind:st.kind.clone()});}}
                    by.insert(of.into(),source);by.insert(target.clone(),id);
                }
            },
            "contrast"=>{
                for node in &mut board{node.side=-1;}let target=&to[0];
                board.push(Board{entity:target.clone(),side:1,apart:false,back:false});
                let a=place(s,&board,&mut cards,&by,at,&to,cy);new_card(&mut cards,&mut by,target,at+0.35,a[target]);
            },
            "then"=>{
                for node in &mut board{node.back=true;}
                while board.iter().filter(|n|n.back&&!n.apart&&n.side!=1).count()>2{
                    let i=board.iter().position(|n|n.back&&!n.apart&&n.side!=1).unwrap();let old=board.remove(i);
                    let card=&mut cards[by[&old.entity]];let p=card.moves.last().unwrap().pose;
                    card.moves.push(Move{at,pose:Pose{x:-260.0,opacity:0.0,..p},fade:false});
                }
                let target=&to[0];board.push(Board{entity:target.clone(),side:0,apart:false,back:false});
                let a=place(s,&board,&mut cards,&by,at,&to,cy);new_card(&mut cards,&mut by,target,at+0.3,a[target]);
            },
            "so"=>{
                let fixes=source_ids(&b["fixes"]);let list:Vec<_>=fixes.iter().filter(|e|board.iter().any(|n|&n.entity==*e)).filter_map(|e|by.get(e).copied()).collect();
                let actor=b["by"].as_str();
                if let Some(actor)=actor{if !list.is_empty(){token=Some(Token{entity:actor.into(),at:at-0.1,hops:list.iter().enumerate().map(|(i,id)|(*id,at+0.3+0.3*i as f64)).collect(),gone:None});}}
                for(i,id)in list.iter().enumerate(){let entity=cards[*id].content.last().unwrap().entity.clone();let kind=b["set"][&entity].as_str().unwrap_or("ok");state(&mut cards[*id],at+0.3+0.3*i as f64+0.06,kind);}
                if let Some(entity)=b["into"].as_str(){
                    let mt=(duration-1.7).min(at+0.3+0.3*list.len().saturating_sub(1)as f64+if list.is_empty(){0.1}else{0.5});
                    end=Some(End{entity:entity.into(),at:mt+0.4,by:actor.map(Into::into),sub:b["subAt"].as_f64().map(|t|(t-s.start).max(mt+0.85)).unwrap_or(mt+1.1)});
                    for card in &mut cards{let scale=card.moves.last().unwrap().pose.scale*0.5;card.moves.push(Move{at:mt,pose:Pose{x:s.w/2.0,y:cy,scale,opacity:0.0},fade:true});}
                    if let Some(token)=&mut token{token.gone=Some(mt+0.1);}
                }
            },
            _=>{}
        }
        if rel!="so"{if let Some(states)=b["set"].as_object(){
            let late=if matches!(rel,"also"|"except"|"because"){0.45+0.42*to.len()as f64+0.85}else if rel=="example"{1.2}else{0.8};
            let time=b["setAt"].as_f64().map(|t|t-s.start).unwrap_or(at+late);
            for(entity,kind)in states{state(&mut cards[by[entity]],time,kind.as_str().unwrap());}
        }}
    }
    Story{entities,cards,token,end}
}

struct Moving{x:String,y:String,scale:String,opacity:String}
fn position(s:&mut Scene,card:&Card,t:&str)->Moving{
    let p=card.moves[0].pose;let mut p=Moving{x:p.x.to_string(),y:p.y.to_string(),scale:p.scale.to_string(),opacity:p.opacity.to_string()};
    let mut gate=f64::NEG_INFINITY;let amp=motion(s.theme.name).0;
    for(i,m)in card.moves.iter().enumerate().skip(1){
        gate=gate.max(m.at);let active=format!("{t} > {gate}");
        let u=s.number(c(format!("({t} - {}) / 0.85",m.at)));
        let e=if m.fade{s.number(format!("Math.pow({u}, 3)"))}else{ease(s,&u)};
        p.x=s.number(format!("{active} ? {} + ({} - {}) * {e} : {}",p.x,m.pose.x,p.x,p.x));
        p.y=s.number(format!("{active} ? {} + ({} - {}) * {e} - {} * Math.sin({PI} * {u}) : {}",p.y,m.pose.y,p.y,if card.clone&&i==1{46}else{0},p.y));
        p.scale=s.number(format!("{active} ? {} + ({} - {}) * {e} : {}",p.scale,m.pose.scale,p.scale,p.scale));
        p.opacity=s.number(format!("{active} ? {} + ({} - {}) * {e} : {}",p.opacity,m.pose.opacity,p.opacity,p.opacity));
        if !m.fade{let rb=rebound(s,format!("{t} - {} - 0.85",m.at));p.scale=mul(s,&p.scale,format!("{active} && {u} >= 1 ? 1 + 0.03 * {amp} * {rb} : 1"));}
    }
    p
}
fn look(state:&str)->(&str,&str){match state{"unknown"=>("","question"),"risk"=>("warning","alert"),"blocked"=>("warning","error"),"ok"=>("accent","check"),"focus"=>("accent",""),"muted"=>("muted",""),_=>("","")}}
fn selected_content(s:&mut Scene,card:&Card,t:&str)->String{
    let mut out="0".to_owned();for(i,event)in card.content.iter().enumerate(){out=s.number(format!("{t} >= {} ? {i} : {out}",event.at));}out
}
fn selected_state(s:&mut Scene,card:&Card,t:&str)->String{
    let mut out="-1".to_owned();for(i,event)in card.states.iter().enumerate(){out=s.number(format!("{t} >= {} ? {i} : {out}",event.at));}out
}
fn flip(s:&mut Scene,card:&Card,t:&str)->String{
    let mut out="0".to_owned();for at in &card.flips{let u=s.number(format!("({t} - {at}) / 0.62"));out=s.number(format!("{u} > 0 && {u} < 1 ? ({u} < 0.5 ? 90 * Math.pow(2 * {u}, 3) : -90 * Math.pow(1 - ({u} - 0.5) * 2, 3)) : {out}"));}out
}
fn indexed(node:&Value,index:usize,layer:&str)->Value{let mut n=node.clone();n["_narrativeIndex"]=json!(index);n["_composedLayer"]=json!(layer);n}

pub(super)fn story(s:&mut Scene,doc:&Value,brands:&BTreeMap<String,String>)->Result<(),String>{
    let top=if doc["title"].as_str().unwrap_or("").is_empty(){96.0}else{s.top+6.0};let cy=top+(s.bottom-top)/2.0-20.0;let cx=s.w/2.0;
    let plan=story_plan(s,doc,cy);let mut nodes=heading(s,doc);
    let mut card_nodes=vec![];let mut who_nodes=vec![];
    for(id,card)in plan.cards.iter().enumerate(){
        let entity=&plan.entities[&card.content[0].entity];card_nodes.push(nodes.len());
        let mut n=box_node("storyCard","detail",cx-116.0,cy-118.0,232.0,236.0,card.born);n["label"]=json!(entity.label);n["icon"]=json!(entity.icon);n["depth"]=json!(1);nodes.push(n);
        let mut seen=BTreeSet::new();
        for content in &card.content{let entity=&plan.entities[&content.entity];if entity.who.is_empty()||!seen.insert(&content.entity){continue;}
            let width=crate::typography::pill_width(&entity.who, &s.theme, 22.0);let index=nodes.len();
            let mut n=box_node("callout","detail",cx-width/2.0,cy-118.0-58.0-22.0,width,44.0,card.born);
            let hash=entity.who.encode_utf16().fold(2166136261u32,|h,c|(h^c as u32).wrapping_mul(16777619));
            n["label"]=json!(entity.who);n["fs"]=json!(22);n["callout"]=json!(true);n["tilt"]=json!(((hash&7)as f64-3.5)*0.6);
            nodes.push(n);who_nodes.push((index,id,content.clone()));
        }
    }
    let token_node=plan.token.as_ref().map(|token|{let index=nodes.len();let mut n=box_node("storyToken","item",cx-44.0,cy-44.0,88.0,88.0,token.at);n["icon"]=json!(plan.entities[&token.entity].icon);n["depth"]=json!(1);nodes.push(n);index});
    let mut end_nodes=vec![];
    if let Some(end)=&plan.end{
        let bw=s.w-2.0*s.margin;let entity=&plan.entities[&end.entity];let mut n=box_node("storyEnd","focus",cx-280.0f64.min(bw/2.0),cy-76.0,560.0f64.min(bw),152.0,end.at);
        n["label"]=json!(entity.label);n["icon"]=json!(end.by.as_ref().map(|by|plan.entities[by].icon.as_str()).unwrap_or(""));n["depth"]=json!(1);end_nodes.push((nodes.len(),true));nodes.push(n);
        if !entity.sub.is_empty(){let mut n=box_node("storySub","item",cx-400.0f64.min(bw/2.0),cy+98.0,800.0f64.min(bw),if s.tall{96.0}else{50.0},end.sub);n["label"]=json!(entity.sub);n["fs"]=json!(36);n["lh"]=json!(44);n["align"]=json!("center");end_nodes.push((nodes.len(),false));nodes.push(n);}
    }
    let nodes=s.prepare_layout_nodes(doc,&nodes,&[])?.0;render_heading(s,&nodes)?;
    let t=s.number("computed.sourceClock / 1000");let amp=motion(s.theme.name).0;
    for(id,card)in plan.cards.iter().enumerate(){
        let index=card_nodes[id];let n=&nodes[index];let p=position(s,card,&t);
        let content=selected_content(s,card,&t);let st=selected_state(s,card,&t);
        let mut f=Fx{x:s.number(format!("{} - {cx}",p.x)),y:s.number(format!("{} - {}",p.y,n["y"].as_f64().unwrap()+118.0)),sx:p.scale.clone(),sy:p.scale.clone(),o:p.opacity.clone(),..Default::default()};
        let muted:Vec<_>=card.states.iter().enumerate().filter(|(_,event)|event.kind=="muted").map(|(i,_)|format!("{st} == {i}")).collect();
        if !muted.is_empty(){f.o=mul(s,&f.o,format!("{} ? 0.45 : 1",muted.join(" || ")));}
        for hit in &card.hits{let u=s.number(format!("{t} - {}",hit.at));if matches!(hit.kind.as_str(),"blocked"|"risk"){f.x=add(s,&f.x,format!("{u} > 0 && {u} <= 1 ? 9 * Math.exp(-6 * {u}) * Math.sin({} * {u}) : 0",2.0*PI*6.0));}else{let rb=rebound(s,&u);let pop=s.number(format!("{u} > 0 && {u} <= 1 ? 1 + 0.05 * {amp} * {rb} : 1"));f.sx=mul(s,&f.sx,&pop);f.sy=mul(s,&f.sy,&pop);}}
        let layer=if card.clone{s.number(format!("{t} < {} ? 0 : {}",card.born+0.4,id+1))}else{(id+1).to_string()};
        let mut n=indexed(n,index,&layer);n["_composedRotateY"]=json!(flip(s,card,&t));root(s,&format!("story-card-motion-{id}"),&n,&f);
        for(i,event)in card.content.iter().enumerate(){let entity=&plan.entities[&event.entity];
            for kind in ["none","unknown","risk","blocked","ok","focus","muted"]{
                let eligible:Vec<_>=card.states.iter().enumerate().filter(|(_,event)|event.kind==kind).map(|(j,_)|format!("{st} == {j}")).chain((kind=="none").then(||format!("{st} == -1"))).collect();
                if eligible.is_empty(){continue;}let visible=s.number(format!("{content} == {i} && ({}) ? 1 : 0",eligible.join(" || ")));
                opacity(s,&format!("story-card-content-{id}-{i}-{kind}"),&visible);
                let mut child=held(&n);child["label"]=json!(entity.label);child["icon"]=json!(entity.icon);child["brand"]=json!(entity.brand);child["tone"]=json!(look(kind).0);
                let(x,y,w,h)=node_box(&n);s.layout_node("storyCard",&format!("story-card-{id}-{i}-{kind}"),&child,x,y,w,h,-100000.0,None)?;
                s.end_move();
            }
        }
        story_badges(s,card,&n,id,&t);end_root(s);
        for(index,_,content)in who_nodes.iter().filter(|(_,owner,_)|*owner==id){story_who(s,&nodes[*index],*index,card,id,content,&plan.token,cy,&t)?;}
    }
    if let(Some(token),Some(index))=(&plan.token,token_node){
        let(mut tx,mut ty,mut land)=("80".to_owned(),(cy-60.0).to_string(),"0".to_owned());
        for(k,(id,at))in token.hops.iter().enumerate(){
            let previous=if k==0{Pose{x:80.0,y:cy-60.0,scale:0.0,opacity:1.0}}else{let mut p=position_at(&plan.cards[token.hops[k-1].0],at-0.3,Some(s.theme.name));p.y-=118.0*p.scale+4.0;p};
            let mut target=position_at(&plan.cards[*id],*at,Some(s.theme.name));target.y-=118.0*target.scale+4.0;
            let u=s.number(c(format!("({t} - {at} + 0.3) / 0.3")));let e=ease(s,&u);let gate=format!("{t} >= {}",at-0.3);
            tx=s.number(format!("{gate} ? {} + {} * {e} : {tx}",previous.x,target.x-previous.x));
            ty=s.number(format!("{gate} ? {} + {} * {e} - 46 * Math.sin({PI} * {u}) : {ty}",previous.y,target.y-previous.y));
            let rb=rebound(s,format!("{t} - {at}"));land=s.number(format!("{gate} ? {rb} : {land}"));
        }
        let gone=token.gone.map(|at|s.number(c(format!("({t} - {at}) / 0.4")))).unwrap_or("0".into());
        let scale=s.number(format!("(1 + 0.12 * {amp} * {land}) * (1 - 0.6 * {gone})"));
        let f=Fx{x:s.number(format!("{tx} - {cx}")),y:s.number(format!("{ty} - {cy}")),sx:scale.clone(),sy:scale,o:s.number(format!("1 - {gone}")),..Default::default()};
        let n=indexed(&nodes[index],index,"20");root(s,"story-token-motion",&n,&f);let(x,y,w,h)=node_box(&n);s.layout_node("storyToken","story-token-body",&held(&n),x,y,w,h,-100000.0,None)?;end_root(s);
    }
    for(index,end)in end_nodes{let n=indexed(&nodes[index],index,if end{"30"}else{"0"});let kind=if end{"storyEnd"}else{"storySub"};root(s,&format!("{kind}-motion"),&n,&Fx::default());let(x,y,w,h)=node_box(&n);s.layout_node(kind,&format!("{kind}-body"),&held(&n),x,y,w,h,-100000.0,None)?;end_root(s);}
    let _=brands;Ok(())
}

fn role(s:&Scene,name:&str)->String{if name=="onAccent"{s.theme.surface.into()}else{THEMES[s.theme.name][name].as_str().unwrap_or(name).into()}}
fn story_badges(s:&mut Scene,card:&Card,node:&Value,id:usize,t:&str){
    let selected=selected_state(s,card,t);let mut border=card_border(s).to_string();for(i,event)in card.states.iter().enumerate(){let width=if matches!(look(&event.kind).0,"warning"|"accent"){2.0}else{card_border(s)};border=s.number(format!("{selected} == {i} ? {width} : ({border})"));}
    let(nx,ny,w,h)=node_box(node);let cx=w-10.0;let cy=10.0;let amp=motion(s.theme.name).0;let(_,freq,decay,_)=motion(s.theme.name);
    for(i,event)in card.states.iter().enumerate(){
        let badge=look(&event.kind).1;if badge.is_empty(){continue;}let m=&CATALOGUE["MODS"][badge];let u=s.number(format!("{t} - {} - 0.05",event.at));
        let mut next="0".to_owned();for later in card.states.iter().skip(i+1).rev(){let k=c(format!("({t} - {}) / 0.2",later.at));next=s.number(format!("{t} >= {} ? {k} : {next}",later.at-0.001));}
        let spring=spring(s,&u);let scale=s.css_number(&format!("{t} >= {} && {u} > 0 && {next} < 1 ? Math.max(0, {spring}) * (1 - Math.pow({next}, 3)) : 0",event.at-0.001),3);
        let rot=s.css_number(&format!("12 * {amp} * Math.exp(-{decay} * {u}) * Math.sin({} * {u})",2.0*PI*freq),1);
        let z=s.native_scale();let px=s.number(format!("({nx} + {cx} + {border}) * {z}"));let py=s.number(format!("({ny} + {cy} + {border}) * {z}"));
        s.push_native(&format!("<MotionBox id={} scaleX={{{scale}}} scaleY={{{scale}}} rotation={{{rot}}} pivotX={{{px}}} pivotY={{{py}}}>",q(&format!("story-badge-motion-{id}-{i}"))));
        svg_box(s,&format!("story-badge-svg-{id}-{i}"),node,(w,h),&border);
        let outline=circle(s,&cx.to_string(),&cy.to_string(),26.0);let bg=role(s,m["bg"].as_str().unwrap());let ring=m["ring"].as_str().unwrap();let stroke=if ring.is_empty(){THEMES[s.theme.name]["bg"].as_str().unwrap().to_owned()}else{role(s,ring)};
        path(s,&format!("story-badge-{id}-{i}"),&outline,(nx,ny,w,h),(w,h),&bg,&stroke,if ring.is_empty(){"4"}else{"3"},"1","");
        let ink=role(s,m["ink"].as_str().unwrap());let g=round((26.0*1.25/24.0)*1000.0)/1000.0;let gx=round((cx-26.0*1.25/2.0)*10.0)/10.0;let gy=round((cy-26.0*1.25/2.0)*10.0)/10.0;
        for(j,p)in m["g"].as_array().unwrap().iter().enumerate(){let fill=p[1]=="f";path(s,&format!("story-badge-glyph-{id}-{i}-{j}"),p[0].as_str().unwrap(),(nx+gx,ny+gy,24.0*g,24.0*g),(24.0,24.0),if fill{&ink}else{"none"},if fill{"none"}else{&ink},"2.6","1","");}
        s.end_move();
        s.end_move();
    }
}

fn story_who(s:&mut Scene,node:&Value,index:usize,card:&Card,id:usize,content:&Content,token:&Option<Token>,cy:f64,t:&str)->Result<(),String>{
    let p=position(s,card,t);let(nx,ny,w,h)=node_box(node);let cx=s.w/2.0;
    let chosen=selected_content(s,card,t);let eligible:Vec<_>=card.content.iter().enumerate().filter(|(_,x)|x.entity==content.entity).map(|(i,_)|format!("{chosen} == {i}")).collect();
    let since=card.content.iter().map(|event|format!("{t} >= {}",event.at)).collect::<Vec<_>>().join(" || ");
    let start=card.born.max(content.at)+0.3;let u=s.number(format!("{t} - {start}"));
    let mut k=s.number(format!("({}) && ({since}) ? {} * (1 - {}) : 0",eligible.join(" || "),c(format!("{u} / 0.35")),c(format!("({u} - 4.2) / 0.4"))));
    for at in &card.flips{k=s.number(format!("{t} > {} && {t} < {} ? 0 : {k}",at-0.05,at+0.62));}
    k=mul(s,&k,format!("{} * {}",c(format!("({} - 0.55) / 0.1",p.scale)),c(format!("({} - 0.7) / 0.2",p.opacity))));
    if let Some(token)=token{k=mul(s,&k,format!("{t} > {} ? 1 - {} : 1",token.at,c(format!("({t} - {}) / 0.3",token.at))));}
    let up=if s.tall{s.number(format!("{} <= {} ? 1 : 0",p.y,cy+1.0))}else{if id%2==0{"1"}else{"0"}.into()};
    let side=s.number(format!("{} < {} ? -1 : {} > {} ? 1 : {}",p.x,cx-40.0,p.x,cx+40.0,if id%4<2{1}else{-1}));
    let x=s.number(format!("Math.max({}, Math.min({}, {} + {side} * {}))",44.0+w/2.0,s.w-44.0-w/2.0,p.x,70.0f64.min(w*0.25)));
    let y=s.number(format!("{} + ({up} == 1 ? -1 : 1) * (118 * {} + 58)",p.y,p.scale));
    let ax=s.number(format!("{} - {side} * 8",p.x));let ay=s.number(format!("{} + ({up} == 1 ? -1 : 1) * (118 * {} + 2)",p.y,p.scale));
    let scale=s.number(format!("0.9 + 0.1 * {k}"));let f=Fx{x:s.number(format!("{x} - {}",nx+w/2.0)),y:s.number(format!("{y} - {}",ny+h/2.0)),sx:scale.clone(),sy:scale,o:k.clone(),..Default::default()};
    let n=indexed(node,index,"25");root(s,&format!("story-who-motion-{index}"),&n,&f);s.layout_node("callout",&format!("story-who-{index}"),&held(&n),nx,ny,w,h,-100000.0,None)?;
    let visible=s.number(format!("{k} > 0.01 ? 1 : 0"));opacity(s,&format!("story-leader-visible-{index}"),&visible);
    svg_box(s,&format!("story-leader-svg-{index}"),node,(w,h),"2");
    let fx=s.number(format!("{} + ({ax} > {x} ? 14 : -14)",w/2.0));let fy=s.number(format!("{up} == 1 ? {} : -4",h+4.0));
    let tx=s.number(format!("{ax} - {x} + {}",w/2.0));let ty=s.number(format!("{ay} - {y} + {}",h/2.0));
    let mx=s.number(format!("({fx} + {tx}) / 2 + ({ax} > {x} ? 26 : -26)"));let my=s.number(format!("({fy} + {ty}) / 2"));
    let rv=s.number(c(format!("({t} - {} - 0.35) / 0.45",card.born.max(content.at))));
    let mut parts=vec![];for j in 0..=14{let u=s.number(format!("{} * {rv}",j as f64/14.0));let v=s.number(format!("1 - {u}"));parts.push((if j==0{"M"}else{" L"},format!("{v} * {v} * {fx} + 2 * {v} * {u} * {mx} + {u} * {u} * {tx}")));parts.push((" ",format!("{v} * {v} * {fy} + 2 * {v} * {u} * {my} + {u} * {u} * {ty}")));}
    let leader=d(s,&parts);path(s,&format!("story-leader-{index}"),&leader,(nx,ny,w,h),(w,h),"none",s.theme.muted,"2.4","1","7 7");
    let endpoint=circle(s,&tx,&ty,5.0);let show=s.number(format!("{rv} > 0.95 ? 1 : 0"));path(s,&format!("story-leader-end-{index}"),&endpoint,(nx,ny,w,h),(w,h),s.theme.muted,"none","0",&show,"");
    s.end_move();s.end_move();end_root(s);Ok(())
}
