//! Source transition-plan.js and transitions.js, expressed as editable native timelines.
use crate::{grammar, layout, scene::{q, Scene, Theme}, span};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::LazyLock;
static SOURCE_NODE_TYPES:LazyLock<Value>=LazyLock::new(||serde_json::from_str(include_str!("node-types.json")).expect("source types"));
static SOURCE_THEMES:LazyLock<Value>=LazyLock::new(||serde_json::from_str(include_str!("themes.json")).expect("source themes"));

const TYPES: [&str;9] = ["slide","wipe","zoom","blinds","iris","cut","match","lift","pull"];
fn duration(kind:&str)->f64 { match kind {"wipe"|"iris"=>0.85,"zoom"|"pull"=>0.9,"cut"=>0.02,"match"=>1.25,"lift"=>0.8,_=>1.0} }
fn clamp(x:f64)->f64{x.clamp(0.0,1.0)}
fn cubic(x:f64)->f64{1.0-(1.0-clamp(x)).powi(3)}
fn expo(x:f64)->f64{let x=clamp(x);if x==1.0{1.0}else{1.0-2.0_f64.powf(-10.0*x)}}
fn inout(x:f64)->f64{let x=clamp(x);if x<0.5{8.0*x.powi(4)}else{1.0-(-2.0*x+2.0).powi(4)/2.0}}
fn inout3(x:f64)->f64{let x=clamp(x);if x<0.5{4.0*x.powi(3)}else{1.0-(-2.0*x+2.0).powi(3)/2.0}}
fn n(v:&Value,k:&str)->f64{v[k].as_f64().unwrap_or(0.0)}
fn word<'a>(v:&'a Value,k:&str)->&'a str{v[k].as_str().unwrap_or("")}
fn rounded(x:f64,digits:i32)->f64{let scale=10.0_f64.powi(digits);(x*scale).round()/scale}
#[derive(Clone,Debug)]
struct Pair{a:usize,b:usize,ac:(f64,f64),bc:(f64,f64),scale:f64}
#[derive(Clone,Debug)]
struct Plan{kind:String,start:f64,end:f64,origin:Option<(f64,f64)>,pairs:Vec<Pair>}
// Use the source duration constant, not end-start: subtraction changes the side
// of the cut's p=0.5 boundary at clocks such as 11.999 seconds.
fn progress_at(plan:&Plan,t:f64)->f64{(t-plan.start)/duration(&plan.kind)}
fn pair(a:usize,b:usize,na:&Value,nb:&Value)->Pair{Pair{a,b,ac:(n(na,"x")+n(na,"w")/2.0,n(na,"y")+n(na,"h")/2.0),bc:(n(nb,"x")+n(nb,"w")/2.0,n(nb,"y")+n(nb,"h")/2.0),scale:n(na,"w")/n(nb,"w")}}
fn match_key(v:&Value,types:&Value)->String{
    if !matches!(word(v,"grp"),"item"|"focus")||types[word(v,"t")]["text"].as_i64()==Some(1)||n(v,"w")<80.0{return String::new()}
    let label=word(v,"label").to_ascii_lowercase().split(|c:char|!c.is_ascii_alphanumeric()).filter(|s|!s.is_empty()).collect::<Vec<_>>().join(" ");
    let icon=["brand","glyph","icon"].iter().map(|k|word(v,k)).find(|s|!s.is_empty()).unwrap_or("");
    format!("{icon}|{label}")
}
fn matches(a:&[Value],b:&[Value],dur_a:f64,heading:bool)->Vec<Pair>{
    let types=&*SOURCE_NODE_TYPES;
    let mut out=Vec::new();let(mut used_a,mut used_b)=(BTreeSet::new(),BTreeSet::new());
    if heading {if let(Some(i),Some(j))=(a.iter().position(|n|n["grp"]=="heading"),b.iter().position(|n|n["grp"]=="heading")){if a[i]["label"]==b[j]["label"]{out.push(pair(i,j,&a[i],&b[j]));used_a.insert(i);used_b.insert(j);}}}
    for icon_only in [false,true]{
        let key=|v:&Value|{let k=match_key(v,&types);if !icon_only{return k}let (icon,label)=k.split_once('|').unwrap_or(("",""));if label.is_empty()||!word(v,"brand").is_empty(){icon.to_owned()}else{String::new()}};
        let count=|nodes:&[Value]|{let mut counts=BTreeMap::new();for node in nodes{let k=key(node);if !k.is_empty()&&k!="|"{*counts.entry(k).or_insert(0usize)+=1;}}counts};
        let(ca,cb)=(count(a),count(b));
        for(i,na)in a.iter().enumerate(){let k=key(na);if k.is_empty()||k=="|"||used_a.contains(&i)||ca.get(&k)!=Some(&1)||cb.get(&k)!=Some(&1)||n(na,"a")>dur_a-0.2||na["gone"].as_f64().is_some_and(|g|g<dur_a-0.05){continue}
            if let Some((j,nb))=b.iter().enumerate().find(|(j,nb)|!used_b.contains(j)&&key(nb)==k){if n(nb,"a")<=1.4{used_a.insert(i);used_b.insert(j);out.push(pair(i,j,na,nb));}}
        }
    }out
}
fn plans(specs:&[Value],nodes:&[Vec<Value>],themes:&[Theme<'_>])->Vec<Plan>{
    let tokens=&*SOURCE_THEMES;
    let(mut section,mut last_section,mut anchor)=(-1isize,Value::Null,None);let mut out=Vec::new();
    for k in 0..specs.len()-1{
        let(a,b)=(&specs[k],&specs[k+1]);let same=themes[k].name==themes[k+1].name;let t=&tokens[themes[k+1].name]["transition"];
        let mut pairs=Vec::new();let mut origin=None;
        let mut kind=if !word(b,"transition").is_empty(){word(b,"transition").to_owned()}
        else if matches!(word(b,"piece"),"insert"|"resume")&&same&&!word(a,"title").is_empty()&&a["title"]==b["title"]{pairs=matches(&nodes[k],&nodes[k+1],n(a,"end")-n(a,"start"),true);"match".into()}
        else if b["piece"]=="insert"{let clock=a["clock"].as_f64().unwrap_or(n(a,"start"));anchor=nodes[k].iter().filter(|v|matches!(word(v,"grp"),"item"|"focus")&&n(v,"a")<=n(b,"start")-clock).max_by(|x,y|n(x,"a").total_cmp(&n(y,"a"))).map(|v|(n(v,"x")+n(v,"w")/2.0,n(v,"y")+n(v,"h")/2.0));origin=anchor;"zoom".into()}
        else if b["piece"]=="resume"{origin=anchor;"pull".into()}
        else{if same&&a["template"]!="title"&&b["template"]!="title"{pairs=matches(&nodes[k],&nodes[k+1],n(a,"end")-n(a,"start"),false);}if pairs.is_empty(){String::new()}else{"match".into()}};
        if kind.is_empty()&&(b["template"]=="title"||(!word(b,"section").is_empty()&&!word(a,"section").is_empty()&&a["section"]!=b["section"])){kind=t["section"].as_str().or_else(||t["type"].as_str()).unwrap().into();}
        if kind.is_empty()&&same&&a["template"]==b["template"]{kind="lift".into();}
        if b["section"]!=last_section||section<0{section+=1;last_section=b["section"].clone();}
        if kind.is_empty(){kind=t["pool"].as_array().map(|pool|pool[section as usize%pool.len()].as_str().unwrap()).unwrap_or_else(||t["type"].as_str().unwrap()).into();}
        if !TYPES.contains(&kind.as_str()){kind="slide".into();}
        // Explicit match still computes its shared nodes (the source plan's missing match is not executable).
        if kind=="match"&&pairs.is_empty(){pairs=matches(&nodes[k],&nodes[k+1],n(a,"end")-n(a,"start"),false);}
        let d=duration(&kind);let start=n(a,"end")-d*0.55;out.push(Plan{kind,start,end:start+d,origin,pairs});
    }out
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]enum Active{Scene(usize),Transition(usize)}
fn active(specs:&[Value],plans:&[Plan],t:f64)->Active{if let Some(i)=plans.iter().position(|p|t>=p.start&&t<p.end){return Active::Transition(i)}let mut i=0;while i+1<specs.len()&&t>=n(&specs[i],"end"){i+=1}Active::Scene(i)}
fn intervals(specs:&[Value],plans:&[Plan],total:f64)->Vec<(f64,f64,Active)>{
    let mut boundaries=vec![0.0,total];for s in specs{boundaries.push(n(s,"end").clamp(0.0,total));}for p in plans{boundaries.extend([p.start.clamp(0.0,total),p.end.clamp(0.0,total)]);}boundaries.sort_by(f64::total_cmp);boundaries.dedup();
    let mut result:Vec<(f64,f64,Active)>=Vec::new();for b in boundaries.windows(2){if b[1]<=b[0]{continue}let state=active(specs,plans,(b[0]+b[1])/2.0);if let Some(last)=result.last_mut(){if last.2==state{last.1=b[1];continue}}result.push((b[0],b[1],state));}result
}
#[derive(Clone,Debug)]enum Mask{Poly(Vec<(f64,f64)>),Circle(f64),Linear{start:f64,end:f64},Radial{cx:f64,cy:f64,inner:f64,outer:f64,invert:bool}}
#[derive(Clone,Debug)]struct Fx{x:f64,y:f64,scale:f64,opacity:f64,blur:f64,visible:bool,mask:Option<Mask>}
impl Default for Fx{fn default()->Self{Self{x:0.0,y:0.0,scale:1.0,opacity:1.0,blur:0.0,visible:true,mask:None}}}
#[derive(Clone,Debug)]struct Frame{a:Fx,b:Fx,bg:Fx,overlays:Vec<Overlay>}
#[derive(Clone,Debug)]enum Overlay{Panel{points:Vec<(f64,f64)>,color:String},Blind{x:f64,top:f64,bottom:f64,width:f64,color:String,accent:String},Ring{radius:f64,width:f64,color:String}}
fn fx(kind:&str,p:f64,w:f64,h:f64,cx:f64,cy:f64,theme:Theme<'_>)->Frame{
    let mut f=Frame{a:Fx::default(),b:Fx::default(),bg:Fx::default(),overlays:Vec::new()};
    match kind{
        "slide"=>{let e=inout(p);f.a.x=-e*w;f.b.x=(1.0-e)*w;let x=(1.0-e)*(w+140.0)-70.0;f.bg.mask=Some(Mask::Linear{start:x-70.0,end:x+70.0});},
        "zoom"=>{let pa=clamp(p/0.6);let pb=clamp((p-0.35)/0.65);let ea=pa.powi(3);let eb=expo(pb);f.a.scale=1.0+0.35*ea;f.a.blur=18.0*ea;f.a.opacity=1.0-ea;f.b.scale=0.84+0.16*eb;f.b.blur=14.0*(1.0-eb);f.b.opacity=clamp(pb*1.8);f.b.visible=p>0.3;let r=cubic(p)*980.0;f.bg.mask=Some(Mask::Radial{cx,cy,inner:(r-180.0).max(0.0),outer:r,invert:false});},
        "blinds"=>{for i in 0..8{let c=inout3((p-i as f64*0.02)/0.38);let r=inout3((p-0.56-i as f64*0.02)/0.38);f.overlays.push(Overlay::Blind{x:i as f64*w/8.0-0.5,top:r*h,bottom:c*h,width:w/8.0+1.0,color:theme.bg.into(),accent:theme.accent.into()});}f.a.visible=p<0.54;f.b.visible=p>=0.54;f.bg.visible=p>=0.54;},
        "wipe"=>{let xl=-240.0+(w+480.0)*inout3(p/0.72);let xt=-240.0+(w+480.0)*inout3((p-0.28)/0.72);f.a.x=-60.0*inout3(p/0.72);f.a.mask=Some(Mask::Poly(vec![(xl+120.0,-40.0),(w+400.0,-40.0),(w+400.0,h+40.0),(xl-120.0,h+40.0)]));f.b.x=90.0*(1.0-inout3((p-0.28)/0.72));f.b.visible=p>0.28;f.b.mask=Some(Mask::Poly(vec![(-400.0,-40.0),(xt+120.0,-40.0),(xt-120.0,h+40.0),(-400.0,h+40.0)]));f.bg.mask=Some(Mask::Poly(vec![(-10.0,0.0),(xt+120.0,0.0),(xt-120.0,h),(-10.0,h)]));f.overlays.push(Overlay::Panel{points:vec![(xt+120.0,0.0),(xl+120.0,0.0),(xl-120.0,h),(xt-120.0,h)],color:theme.focus.into()});f.overlays.push(Overlay::Panel{points:vec![(xl+94.0,0.0),(xl+120.0,0.0),(xl-120.0,h),(xl-146.0,h)],color:theme.accent.into()});},
        "iris"=>{let r=inout3(p)*860.0;f.a.scale=1.0-0.06*p;f.a.opacity=1.0-0.3*p;f.a.mask=Some(Mask::Radial{cx:w/2.0,cy,inner:r,outer:r+1.0,invert:true});f.b.scale=1.06-0.06*cubic(p);f.b.mask=Some(Mask::Circle(r));f.b.visible=p>0.01;f.bg.mask=Some(Mask::Circle(r));f.overlays.push(Overlay::Ring{radius:r,width:4.0+6.0*(1.0-p),color:theme.accent.into()});},
        "pull"=>{let a=clamp(p/0.6).powi(3);let b=expo((p-0.3)/0.7);f.a.scale=1.0-0.16*a;f.a.blur=12.0*a;f.a.opacity=1.0-a;f.b.scale=1.22-0.22*b;f.b.blur=14.0*(1.0-b);f.b.opacity=clamp((p-0.3)*2.2);f.b.visible=p>0.3;},
        "lift"=>{let a=clamp(p/0.5).powi(3);let b=expo((p-0.4)/0.6);f.a.y=-46.0*a;f.a.opacity=1.0-a;f.a.visible=p<0.5;f.b.y=46.0*(1.0-b);f.b.opacity=b;f.b.visible=p>0.4;},
        "match"=>{f.a.opacity=1.0-cubic(p/0.55);},
        _=>{f.a.visible=p<0.5;f.b.visible=p>=0.5;f.bg.visible=p>=0.5;}
    }f
}
fn animation(prop:&str,from:f64,to:f64,start:f64,end:f64)->String{format!("<Animate prop={} from={{{from}}} to={{{to}}} start=\"{start}ms\" duration=\"{}ms\" ease=\"linear\"/>",q(prop),end-start)}
fn track(prop:&str,times:&[f64],values:&[f64])->String{let mut out=String::new();if let Some(v)=values.first(){out.push_str(&animation(prop,*v,*v,0.0,0.0));}for(i,t)in times.windows(2).enumerate(){if values[i]!=values[i+1]{out.push_str(&animation(prop,values[i],values[i+1],t[0],t[1]));}}out}
fn path(points:&[(f64,f64)],scale:f64)->String{let mut d=String::new();for(i,(x,y))in points.iter().enumerate(){d.push_str(&format!("{}{} {}",if i==0{"M"}else{"L"},rounded(*x,1)*scale,rounded(*y,1)*scale));}
    // Z adds an implicit segment only when the endpoint differs from the start.
    // Emit it explicitly so a collapsed polygon retains the same morph topology.
    if let Some((x,y))=points.first(){d.push_str(&format!("L{} {}",rounded(*x,1)*scale,rounded(*y,1)*scale));}d.push('Z');d}
fn animated_circle(id:&str,geometry:&[(f64,f64,f64)],times:&[f64],fill:&str,stroke:&str,mask_only:bool)->String{
    let(x,y,size)=geometry[0];let mut out=format!("<Rect id={} maskOnly={{{mask_only}}} x={{{x}}} y={{{y}}} width={{{size}}} height={{{size}}} radius={{{}}} fill={} stroke={} strokeWidth={{0}}>",q(id),size/2.0,q(fill),q(stroke));
    for prop in ["x","y","width","height","radius"]{let values=geometry.iter().map(|(x,y,size)|match prop{"x"=>*x,"y"=>*y,"radius"=>size/2.0,_=>*size}).collect::<Vec<_>>();out.push_str(&track(prop,times,&values));}
    out.push_str("</Rect>");out
}
fn animated_path(id:&str,paths:&[String],times:&[f64],width:u32,height:u32,fill:&str,stroke:&str,sw:f64,mask_only:bool)->String{let mut out=format!("<Path id={} maskOnly={{{mask_only}}} d={} x={{0}} y={{0}} width={{{width}}} height={{{height}}} viewBox=\"0 0 {width} {height}\" fill={} stroke={} strokeWidth={{{sw}}}>",q(id),q(&paths[0]),q(fill),q(stroke));for(i,t)in times.windows(2).enumerate(){out.push_str(&format!("<Animate prop=\"d\" from={} to={} start=\"{}ms\" duration=\"{}ms\" ease=\"linear\"/>",q(&paths[i]),q(&paths[i+1]),t[0],t[1]-t[0]));}out.push_str("</Path>");out}
fn wrapper(body:&str,id:&str,frames:&[Fx],times:&[f64],width:u32,height:u32,w:f64,cy:f64,origin:(f64,f64))->String{
    let s=width as f64/w;let mut masks=String::new();let mut mask_attr=String::new();
    if let Some(mask)=&frames[0].mask{let mask_id=format!("{id}-mask");mask_attr=format!(" maskObjectId={}",q(&mask_id));match mask{
        Mask::Linear{..}=>{masks=format!("<Rect id={} maskOnly={{true}} x={{0}} y={{0}} width={{{width}}} height={{{height}}} fill=\"#FFFFFF\">",q(&mask_id));for prop in ["linearMaskStartX","linearMaskEndX"]{let values=frames.iter().map(|f|if let Some(Mask::Linear{start,end})=&f.mask{rounded(if prop=="linearMaskStartX"{*start}else{*end},1)*s}else{unreachable!()}).collect::<Vec<_>>();masks.push_str(&track(prop,times,&values));}masks.push_str("</Rect>");},
        Mask::Radial{invert,..}=>{masks=format!("<Rect id={} maskOnly={{true}} x={{0}} y={{0}} width={{{width}}} height={{{height}}} fill=\"#FFFFFF\" radialMaskInvert={{{invert}}}>",q(&mask_id));for prop in ["radialMaskCenterX","radialMaskCenterY","radialMaskInnerRadius","radialMaskOuterRadius"]{let values=frames.iter().map(|f|if let Some(Mask::Radial{cx,cy,inner,outer,..})=&f.mask{rounded(match prop{"radialMaskCenterX"=>*cx,"radialMaskCenterY"=>*cy,"radialMaskInnerRadius"=>*inner,_=>*outer},1)*s}else{unreachable!()}).collect::<Vec<_>>();masks.push_str(&track(prop,times,&values));}masks.push_str("</Rect>");},
        Mask::Circle(_)=>{let geometry=frames.iter().map(|f|if let Some(Mask::Circle(r))=&f.mask{let r=rounded(*r,1)*s;(w/2.0*s-r,cy*s-r,2.0*r)}else{unreachable!()}).collect::<Vec<_>>();masks=animated_circle(&mask_id,&geometry,times,"#FFFFFF","none",true);},
        Mask::Poly(_)=>{let paths=frames.iter().map(|f|if let Some(Mask::Poly(points))=&f.mask{path(points,s)}else{unreachable!()}).collect::<Vec<_>>();masks=animated_path(&mask_id,&paths,times,width,height,"#FFFFFF","none",0.0,true);}
    }}
    // CSS masks live in the slot's local space. Share its transform, but not
    // its opacity or blur: otherwise a fading slot also fades its mask twice.
    let transform_id=if masks.is_empty(){q(id)}else{q(&format!("{id}-transform"))};
    let mut out=format!("<MotionBox id={transform_id} pivotX={{{}}} pivotY={{{}}}>",origin.0*s,origin.1*s);
    for prop in ["x","y","scaleX","scaleY"]{let values=frames.iter().map(|f|match prop{"x"=>rounded(f.x,1)*s,"y"=>rounded(f.y,1)*s,_=>rounded(f.scale,3)}).collect::<Vec<_>>();out.push_str(&track(prop,times,&values));}
    if !masks.is_empty(){out.push_str(&format!("{masks}<MotionBox id={}{mask_attr}>",q(id)));}
    for prop in ["opacity","blur"]{let values=frames.iter().map(|f|if prop=="opacity"{if f.visible{rounded(f.opacity,3)}else{0.0}}else if f.blur>0.05{rounded(f.blur,1)*s}else{0.0}).collect::<Vec<_>>();out.push_str(&track(prop,times,&values));}
    out.push_str(body);out.push_str("</MotionBox>");if !masks.is_empty(){out.push_str("</MotionBox>");}out
}
fn shift_time(source:&str,offset:f64)->String{
    if offset==0.0{return source.to_owned()}let mut result=String::with_capacity(source.len());let mut rest=source;while let Some(i)=rest.find("start=\""){result.push_str(&rest[..i+7]);rest=&rest[i+7..];if let Some(end)=rest.find("ms\""){if let Ok(value)=rest[..end].parse::<f64>(){result.push_str(&(value+offset).to_string());rest=&rest[end..];continue}}// Only generated millisecond animation attributes are rewritten.
    }result.push_str(rest);result
}
fn document(id:&str,body:&str,variables:&Value,computeds:&str,clock_offset:f64,width:u32,height:u32,duration:f64)->String{let vars=variables.as_object().unwrap().iter().map(|(k,v)|format!("{k}:stringVar({v})")).collect::<Vec<_>>().join(",");format!("export default defineSpanScene({{id:{},width:{width},height:{height},durationMs:{duration},variables:{{{vars}}},computeds:{{sourceClock:computed.number(() => time.ms - ({clock_offset})),{computeds}}},render({{vars}}){{return <Scene>{body}</Scene>;}}}});",q(id))}
fn samples(start:f64,end:f64,plan:Option<&Plan>)->Vec<f64>{
    let length=(end-start)*1000.0;let mut out=vec![0.0,length];
    // Frame samples belong to the global movie clock, not the transition's padded start.
    for frame in ((start*30.0).floor()as i64+1)..(end*30.0).ceil()as i64{
        let t=(frame as f64/30.0-start)*1000.0;
        if t>0.0&&t<length{out.push(t);}
    }
    if let Some(p)=plan{for u in [0.01,0.28,0.3,0.35,0.4,0.5,0.54,0.55,0.6,0.72,0.85]{
        let t=(p.start+duration(&p.kind)*u-start)*1000.0;
        if t>0.0&&t<length{out.push((t-0.000001).max(0.0));out.push(t);out.push((t+0.000001).min(length));}
    }}
    out.sort_by(f64::total_cmp);out.dedup();out
}
fn clocked(spec:&Value)->Value{let mut v=spec.clone();if let(Some(a),Some(b))=(spec["clock"].as_f64(),spec["clockEnd"].as_f64()){v["start"]=json!(a);v["end"]=json!(b);}v}
fn scene<'a>(spec:&Value,index:usize,width:u32,height:u32,theme:Theme<'a>,brands:&'a BTreeMap<String,String>,exit:bool,holds:BTreeSet<usize>,end:f64,camera:Vec<(f64,f64)>,music:Option<&crate::music::Music>)->Result<Scene<'a>,String>{let physical_end=n(spec,"end");let mut spec=clocked(spec);if let Some(music)=music{music.attach_beats(&mut spec);}let mut s=Scene::new(format!("explainer-scene-{index}"),width,height,(n(&spec,"end")-n(&spec,"start"))*1000.0,n(&spec,"start"),theme);s.music=music.cloned();s.exit_scene=exit;s.exit_start=Some((physical_end-n(&spec,"start")-0.95)*1000.0);s.transition_holds=holds;s.render_end=((end-n(&spec,"start"))*1000.0).max(s.duration);s.background_camera=camera;layout::render(&mut s,&spec,brands)?;grammar::callouts(&mut s,&spec,"",0)?;Ok(s)}
fn node_fx(mut source:String,nodes:usize,plan:&Plan,incoming:bool,times:&[f64],progress:&[f64],scale:f64)->String{
    for i in 0..nodes{let pair=plan.pairs.iter().find(|m|if incoming{m.b==i}else{m.a==i});let mut anim=String::new();if incoming{if let Some(m)=pair{for prop in ["x","y","scaleX","scaleY"]{let values=progress.iter().map(|p|{let e=inout(p/0.85);match prop{"x"=>(m.ac.0-m.bc.0)*(1.0-e)*scale,"y"=>(m.ac.1-m.bc.1)*(1.0-e)*scale,_=>m.scale+(1.0-m.scale)*e}}).collect::<Vec<_>>();anim.push_str(&track(prop,times,&values));}}else{anim=track("opacity",times,&progress.iter().map(|p|clamp((p-0.35)/0.45)).collect::<Vec<_>>());}}else if pair.is_some(){anim=track("opacity",times,&vec![0.0;times.len()]);}if !anim.is_empty(){let marker=format!("<MotionBox id=\"transition-node-{i}\"");if let Some(pos)=source.find(&marker){if let Some(end)=source[pos..].find('>'){source.insert_str(pos+end+1,&anim);}}}}
    source
}
fn overlay_scene(frames:&[Frame],times:&[f64],width:u32,height:u32,w:f64,cy:f64)->String{
    let mut overlay=String::new();let s=width as f64/w;
    for index in 0..frames[0].overlays.len(){
        let id=format!("overlay-{index}");
        match &frames[0].overlays[index]{
            Overlay::Panel{color,..}=>{
                let paths=frames.iter().map(|f|if let Overlay::Panel{points,..}=&f.overlays[index]{path(points,s)}else{unreachable!()}).collect::<Vec<_>>();
                overlay.push_str(&animated_path(&id,&paths,times,width,height,color,"none",0.0,false));
            },
            Overlay::Blind{color,accent,..}=>{
                let polygons=frames.iter().map(|f|if let Overlay::Blind{x,top,bottom,width,..}=&f.overlays[index]{path(&[(*x,*top),(*x+width,*top),(*x+width,*bottom),(*x,*bottom)],s)}else{unreachable!()}).collect::<Vec<_>>();
                let mut panel=animated_path(&id,&polygons,times,width,height,color,"none",0.0,false);
                let accents=frames.iter().map(|f|if let Overlay::Blind{x,top,bottom,width,..}=&f.overlays[index]{let top=(bottom-3.0).max(*top);path(&[(*x,top),(*x+width,top),(*x+width,*bottom),(*x,*bottom)],s)}else{unreachable!()}).collect::<Vec<_>>();
                panel.push_str(&animated_path(&format!("{id}-accent"),&accents,times,width,height,accent,"none",0.0,false));
                let visible=frames.iter().map(|f|if let Overlay::Blind{top,bottom,..}=&f.overlays[index]{if bottom-top>0.5{1.0}else{0.0}}else{unreachable!()}).collect::<Vec<_>>();
                overlay.push_str(&format!("<MotionBox id={}>{}{panel}</MotionBox>",q(&format!("{id}-visibility")),track("opacity",times,&visible)));
            },
            Overlay::Ring{color,..}=>{
                // Both CSS borders and native SDF strokes lie inside these
                // outer bounds. Blink floors the authored CSS border width.
                let geometry=frames.iter().map(|f|if let Overlay::Ring{radius,width,..}=&f.overlays[index]{let border=rounded(*width,1).floor();let size=rounded(2.0*radius,1).max(2.0*border);(rounded(w/2.0-radius,1)*s,rounded(cy-radius,1)*s,size*s)}else{unreachable!()}).collect::<Vec<_>>();
                let mut ring=animated_circle(&id,&geometry,times,"none",color,false);
                let widths=frames.iter().map(|f|if let Overlay::Ring{width,..}=&f.overlays[index]{rounded(*width,1).floor()*s}else{unreachable!()}).collect::<Vec<_>>();
                ring.insert_str(ring.len()-7,&track("strokeWidth",times,&widths));
                let visible=frames.iter().map(|f|if let Overlay::Ring{radius,..}=&f.overlays[index]{if *radius>4.0&&*radius<820.0{1.0}else{0.0}}else{unreachable!()}).collect::<Vec<_>>();
                overlay.push_str(&format!("<MotionBox id={}>{}{ring}</MotionBox>",q(&format!("{id}-visibility")),track("opacity",times,&visible)));
            }
        }
    }overlay
}
pub(crate) fn compile(specs:&[Value],width:u32,height:u32,default_theme:Theme<'_>,brands:&BTreeMap<String,String>,music:Option<&crate::music::Music>)->Result<(Vec<Value>,f64),String>{
    let mut previous=0.0;let mut themes=Vec::with_capacity(specs.len());let mut nodes=Vec::with_capacity(specs.len());
    for(i,spec)in specs.iter().enumerate(){grammar::scene(spec).map_err(|e|format!("scenes[{i}]: {e}"))?;if let Some(kind)=spec.get("transition"){if !kind.as_str().is_some_and(|kind|TYPES.contains(&kind)){return Err(format!("scenes[{i}]: transition must be one of {}",TYPES.join(", ")));}}if spec.get("clock").is_some()||spec.get("clockEnd").is_some(){let(a,b)=(grammar::number(spec,"clock")?,grammar::number(spec,"clockEnd")?);if b<=a{return Err(format!("scenes[{i}]: clockEnd must exceed clock"));}}let(start,end)=(grammar::number(spec,"start")?,grammar::number(spec,"end")?);if start<previous||end-start<0.001||end*1000.0>i64::MAX as f64{return Err("scenes must have increasing nonoverlapping start/end times".into())}previous=end;let theme=if word(spec,"theme").is_empty(){default_theme}else{Theme::get(word(spec,"theme"))?};themes.push(theme);nodes.push(scene(spec,i,width,height,theme,brands,false,BTreeSet::new(),end,Vec::new(),music)?.transition_nodes);}
    let plans=plans(specs,&nodes,&themes);let mut spans=Vec::new();let w=if height as f64>=width as f64*0.95{720.0}else{1280.0};let h=height as f64*w/width as f64;let cy=if height as f64>=width as f64*0.95{630.0}else{368.0};
    for(segment,(start,end,state))in intervals(specs,&plans,previous).into_iter().enumerate(){
        // Native placement is integer milliseconds; retain the source Plan clock but
        // anchor all generated animation times to the actual emitted placement.
        let(start,end)=((start*1000.0).round()/1000.0,(end*1000.0).round()/1000.0);
        if end<=start{continue}
        let plan=if let Active::Transition(k)=state{Some(&plans[k])}else{None};let times=samples(start,end,plan);let progress=times.iter().map(|ms|plan.map(|p|progress_at(p,start+ms/1000.0)).unwrap_or(0.0)).collect::<Vec<_>>();let k=match state{Active::Scene(k)|Active::Transition(k)=>k};let origin=plan.and_then(|p|p.origin).unwrap_or((w/2.0,cy));let frames=plan.map(|p|progress.iter().map(|u|fx(&p.kind,*u,w,h,origin.0,origin.1,themes[k+1])).collect::<Vec<_>>());
        let camera=times.iter().zip(&progress).map(|(ms,p)|((start+ms/1000.0)*1000.0,if plan.is_some(){(k as f64+inout(*p))*w}else{k as f64*w})).collect::<Vec<_>>();
        for incoming in [false,true]{if incoming&&plan.is_none(){continue}let i=k+usize::from(incoming);let spec=&specs[i];let clock=spec["clock"].as_f64().unwrap_or(n(spec,"start"));let holds=if incoming&&plan.is_some_and(|p|p.kind=="match"){plan.unwrap().pairs.iter().map(|m|m.b).collect()}else{BTreeSet::new()};let exit=!incoming&&(i+1==specs.len()||plans.get(i).is_some_and(|p|p.kind=="cut"));let local_camera=camera.iter().map(|(t,x)|(t-clock*1000.0,*x)).collect();let scene=scene(spec,i,width,height,themes[i],brands,exit,holds,end,local_camera,music)?;let(bg,fg,variables,computeds)=scene.finish_parts();let offset=(clock-start)*1000.0;let mut fg=shift_time(&fg,offset);let bg=shift_time(&bg,offset);
            if let Some(plan)=plan{if plan.kind=="match"{fg=node_fx(fg,nodes[i].len(),plan,incoming,&times,&progress,width as f64/w);}}
            let mut emit=|part:&str,body:String,layer:i32|->Result<(),String>{let id=format!("explainer-{segment}-{i}-{part}");let source=document(&id,&body,&variables,&computeds,offset,width,height,(end-start)*1000.0);spans.push(span(id,&source,variables.clone(),start*1000.0,(end-start)*1000.0,width,height,layer)?);Ok(())};
            if !incoming||themes[k].name!=themes[k+1].name{let body=if incoming{wrapper(&bg,"incoming-background",&frames.as_ref().unwrap().iter().map(|f|f.bg.clone()).collect::<Vec<_>>(),&times,width,height,w,cy,(w/2.0,cy))}else{bg};emit("background",body,if incoming{1}else{0})?;}
            let body=if let Some(frames)=&frames{wrapper(&fg,if incoming{"incoming-slot"}else{"outgoing-slot"},&frames.iter().map(|f|if incoming{f.b.clone()}else{f.a.clone()}).collect::<Vec<_>>(),&times,width,height,w,cy,if matches!(plan.unwrap().kind.as_str(),"zoom"|"pull"){origin}else{(640.0,cy)})}else{fg};emit("content",body,if incoming{11}else{10})?;
        }
        if let Some(frames)=frames{let overlay=overlay_scene(&frames,&times,width,height,w,cy);if !overlay.is_empty(){let id=format!("explainer-{segment}-overlays");let vars=json!({});let source=document(&id,&overlay,&vars,"",0.0,width,height,(end-start)*1000.0);spans.push(span(id,&source,vars,start*1000.0,(end-start)*1000.0,width,height,20)?);}}
    }Ok((spans,previous))
}
#[cfg(test)]mod tests{use super::*;
#[test]fn every_transition_uses_source_padding_and_half_open_boundary(){let specs=vec![json!({"start":0,"end":5}),json!({"start":5,"end":10})];for kind in TYPES{let d=duration(kind);let p=Plan{kind:kind.into(),start:5.0-d*0.55,end:5.0+d*0.45,origin:None,pairs:vec![]};assert_eq!(active(&specs,&[p.clone()],p.start),Active::Transition(0));assert_eq!(active(&specs,&[p.clone()],p.end),Active::Scene(1));assert_eq!(active(&specs,&[p.clone()],5.0),Active::Transition(0));assert!((p.end-p.start-d).abs()<1e-12);}}
#[test]fn earliest_transition_wins_overlapping_windows_and_active_scene_ignores_gaps(){let specs=vec![json!({"end":1}),json!({"start":3,"end":1.1}),json!({"start":4,"end":6})];let plans=vec![Plan{kind:"slide".into(),start:0.45,end:1.45,origin:None,pairs:vec![]},Plan{kind:"slide".into(),start:0.55,end:1.55,origin:None,pairs:vec![]}];assert_eq!(active(&specs,&plans,1.2),Active::Transition(0));assert_eq!(active(&specs,&plans,1.45),Active::Transition(1));assert_eq!(active(&specs,&plans,2.0),Active::Scene(2));}
#[test]fn source_time_shift_preserves_resume_clock_and_negative_preroll(){let s="<Animate start=\"100ms\" duration=\"20ms\"/><Animate start=\"-40ms\"/>";assert_eq!(shift_time(s,500.0),"<Animate start=\"600ms\" duration=\"20ms\"/><Animate start=\"460ms\"/>");}
#[test]fn source_transition_states_are_distinct_in_both_formats(){let theme=Theme::get("signal").unwrap();for(w,h,cy)in[(1280.0,720.0,368.0),(720.0,1280.0,630.0)]{let slide=fx("slide",0.5,w,h,w/2.0,cy,theme);assert_eq!(slide.a.x,-w/2.0);assert_eq!(slide.b.x,w/2.0);let wipe=fx("wipe",0.5,w,h,w/2.0,cy,theme);assert_eq!(wipe.overlays.len(),2);assert!(matches!(wipe.a.mask,Some(Mask::Poly(_))));let zoom=fx("zoom",0.5,w,h,w/2.0,cy,theme);assert!(zoom.a.blur>0.0);assert!(matches!(zoom.bg.mask,Some(Mask::Radial{invert:false,..})));let iris=fx("iris",0.5,w,h,w/2.0,cy,theme);assert!(matches!(iris.a.mask,Some(Mask::Radial{invert:true,..})));assert!(!fx("cut",0.5,w,h,w/2.0,cy,theme).a.visible);assert!(fx("cut",0.5,w,h,w/2.0,cy,theme).b.visible);}}
#[test]fn frame_samples_preserve_global_clock_across_fractional_transition_padding(){let start=4.5325;let times=samples(start,5.3825,None);for frame in 136..162{let expected=(frame as f64/30.0-start)*1000.0;assert!(times.iter().any(|t|(t-expected).abs()<1e-9));}}
#[test]fn match_requires_unique_visible_elements_and_holds_shared_heading(){
    let a=vec![json!({"grp":"heading","t":"heading","label":"Shared","x":0,"y":0,"w":400,"h":50,"a":0.12}),json!({"grp":"item","t":"cardTile","label":"Account","glyph":"user","x":100,"y":100,"w":200,"h":100,"a":0.8})];
    let mut b=a.clone();b[1]["x"]=json!(500);b[1]["w"]=json!(100);
    let matched=matches(&a,&b,5.0,true);assert_eq!(matched.len(),2);assert_eq!(matched[1].scale,2.0);
    b[1]["a"]=json!(1.41);assert_eq!(matches(&a,&b,5.0,false).len(),0);
    b[1]["a"]=json!(1.0);let duplicate=b[1].clone();b.push(duplicate);assert_eq!(matches(&a,&b,5.0,false).len(),0);
}
#[test]fn collapsed_polygons_keep_explicit_closed_morph_topology(){
    let collapsed=path(&[(0.0,0.0),(100.0,0.0),(100.0,0.0),(0.0,0.0)],1.0);
    let expanded=path(&[(0.0,0.0),(100.0,0.0),(100.0,80.0),(0.0,80.0)],1.0);
    assert_eq!(collapsed,"M0 0L100 0L100 0L0 0L0 0Z");
    assert_eq!(expanded,"M0 0L100 0L100 80L0 80L0 0Z");
}
#[test]fn cut_keeps_source_floating_point_side_at_11999_milliseconds(){
    let d=duration("cut");let start=12.0-d*0.55;
    let plan=Plan{kind:"cut".into(),start,end:start+d,origin:None,pairs:vec![]};
    let p=progress_at(&plan,11.999);
    assert!(p<0.5);
    let frame=fx("cut",p,1280.0,720.0,640.0,368.0,Theme::get("daylight").unwrap());
    assert!(frame.a.visible);assert!(!frame.b.visible);
    assert!(progress_at(&plan,11.999000001)>0.5);
}
}
