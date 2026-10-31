use super::*;
use std::f64::consts::{PI, TAU};
include!("layout_routes.rs");
include!("layout_openers.rs");
include!("layout_picture.rs");
include!("layout_list.rs");
include!("layout_callouts.rs");
#[cfg(test)] #[path="layout_geometry_tests.rs"] mod layout_tests;
type Frame = (f64,f64,f64,f64);
struct Node { kind: &'static str, item: Value, frame: Frame }
struct Edge { a: usize, b: usize, kind: &'static str, sides: Option<(&'static str,&'static str)>, extra:Value }
struct Plan<'a,'b> { s: &'a Scene<'b>, nodes: Vec<Node>, edges: Vec<Edge> }
impl<'a,'b> Plan<'a,'b> {
    fn add(&mut self, kind: &'static str, f: Frame, item: &Value, group: &str, extra: Value) -> usize {
        let mut item=item.as_object().cloned().unwrap_or_default();
        item.insert("grp".into(),json!(group));
        if let Some(ex)=extra.as_object() { for (k,v) in ex { item.insert(k.clone(),v.clone()); } }
        if let Some(parent)=item.get("parent").and_then(Value::as_u64) {
            if !item.contains_key("tone") { if let Some(tone)=self.nodes[parent as usize].item.get("tone") {item.insert("tone".into(),tone.clone());} }
        }
        let id=self.nodes.len();
        self.nodes.push(Node{kind,item:Value::Object(item),frame:(f.0.round(),f.1.round(),f.2.round(),f.3.round())}); id
    }
    fn edge(&mut self,a:usize,b:usize,kind:&'static str,sides:Option<(&'static str,&'static str)>) { self.edges.push(Edge{a,b,kind,sides,extra:json!({})}); }
    fn width(&self,label:&str,fs:f64)->f64 { crate::typography::measure_style(label,self.s.theme.font,600,0.0,fs,1.0e9,1.1).width*1.03 }
    fn fitw(&self,item:&Value,fs:f64,extra:f64,min:f64,max:f64)->f64 { (self.width(item["label"].as_str().unwrap_or(""),fs)+extra).round().max(min).min(max) }
    fn pillw(&self,label:&str,fs:f64)->f64 { ((self.width(label,fs)*1.06+36.0)/2.0).round()*2.0 }
    fn pills(&mut self,parent:usize,labels:&[Value],x:f64,y:f64,max:f64,align:&str,big:bool)->usize {
        let fs=if big {22.0}else{20.0}; let h=if big {48.0}else{40.0};
        let mut out=Vec::new(); let mut used=0.0;
        for label in labels {let w=self.pillw(label.as_str().unwrap(),fs);let gap=if out.is_empty(){0.0}else{8.0};if used+gap+w<=max {out.push((label,w));used+=gap+w;}}
        let mut xx=x+if align=="center" {(max-used)/2.0}else if align=="right" {max-used}else{0.0};
        let count=out.len();
        for (label,w) in out {self.add(if big {"pillL"}else{"pill"},(xx,y,w,h),&json!({"label":label,"icon":""}),"detail",json!({"parent":parent}));xx+=w+8.0;} count
    }
}
fn details(item:&Value)->&[Value] {item.get("details").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])}
fn branch(item:&Value)->&'static str {if item["tone"]=="warning" {"exception"}else{"branch"}}
fn relation(item:&Value)->(&'static str,&'static str,&'static str) {match item["rel"].as_str().unwrap_or("follows") {"because"=>("affinity","because",""),"example"=>("branch","e.g.",""),"contrast"=>("structure","but",""),"except"=>("exception","except","warning"),"so"=>("flow","so","accent"),"also"=>("flow","also",""),_=>("flow","","")}}
// ECMAScript \s differs from Rust whitespace (notably BOM and U+0085).
fn source_space(c:char)->bool {matches!(c,'\u{0009}'..='\u{000d}'|' '|'\u{00a0}'|'\u{1680}'|'\u{2000}'..='\u{200a}'|'\u{2028}'|'\u{2029}'|'\u{202f}'|'\u{205f}'|'\u{3000}'|'\u{feff}')}
fn source_string(text:&str,max:usize)->Option<String> {
    let mut units=0;let mut previous_space=true;let mut changed=false;
    for c in text.chars() {let space=source_space(c);changed|=space&&(c!=' '||previous_space);previous_space=space;units+=c.len_utf16();}
    if !changed&&!previous_space&&units<=max {return None}
    if text.is_empty() {return None}
    let mut out=String::with_capacity(text.len().min(max.saturating_mul(3)));let mut pending_space=false;let mut units=0;
    for c in text.chars() {
        if source_space(c) {pending_space=!out.is_empty();continue}
        if pending_space {if units==max {break}out.push(' ');units+=1;pending_space=false;}
        if units==max {break}
        if units+c.len_utf16()>max {out.push('\u{fffd}');break}
        out.push(c);units+=c.len_utf16();
    }
    (out!=text).then_some(out)
}
fn normalized_fields(v:&Value,fields:&[(&str,usize)])->Option<Value> {
    let mut normalized=None;
    for &(key,max) in fields {
        if let Some(value)=v[key].as_str().and_then(|text|source_string(text,max)) {
            normalized.get_or_insert_with(||v.clone())[key]=Value::String(value);
        }
    }
    normalized
}
fn normalized_item(v:&Value,max:usize)->Option<Value> {
    let mut normalized=normalized_fields(v,&[("label",max),("brand",40)]);
    if let Some(details)=v["details"].as_array() {
        for (i,detail) in details.iter().enumerate() {
            if let Some(text)=detail.as_str().and_then(|text|source_string(text,26)) {
                normalized.get_or_insert_with(||v.clone())["details"][i]=Value::String(text);
            }
        }
    }
    normalized
}
pub(super) fn normalized_strings(v:&Value)->Option<Value> {
    let family=v["template"].as_str().unwrap_or("");
    let mut normalized=normalized_fields(v,&[("title",56),("subtitle",96),("text",if family=="illustration"{140}else{160}),("highlight",60),("by",48),("label",48),("alt",140)]);
    // Resolved inline assets are bytes, not the logical URL normalized by the oracle.
    if v["image"].as_str().is_some_and(|image|!image.starts_with("data:")) {
        if let Some(image)=v["image"].as_str().and_then(|image|source_string(image,2048)) {normalized.get_or_insert_with(||v.clone())["image"]=Value::String(image);}
    }
    for (key,max) in [("focus",32),("takeaway",80),("intro",40)] {
        if let Some(item)=normalized_item(&v[key],max) {normalized.get_or_insert_with(||v.clone())[key]=item;}
    }
    if let Some(items)=v["items"].as_array() {
        for (i,item) in items.iter().enumerate() {
            if let Some(item)=normalized_item(item,if family=="list"{60}else{32}) {normalized.get_or_insert_with(||v.clone())["items"][i]=item;}
        }
    }
    let raw_callouts=v.get("callouts").or_else(||v["picture"].get("callouts"));
    if let Some(items)=raw_callouts.and_then(Value::as_array) {
        let mut callouts=None;
        for (i,item) in items.iter().enumerate() {
            if let Some(item)=normalized_fields(item,&[("text",32)]) {callouts.get_or_insert_with(||items.clone())[i]=item;}
        }
        let current=callouts.as_ref().unwrap_or(items);
        if current.iter().any(|c|c["text"].as_str()==Some("")) {callouts.get_or_insert_with(||items.clone()).retain(|c|c["text"].as_str()!=Some(""));}
        let at=|c:&Value|c["at"].as_f64().unwrap_or(1e9);
        let current=callouts.as_ref().unwrap_or(items);
        if current.windows(2).any(|pair|at(&pair[0])>at(&pair[1])) {callouts.get_or_insert_with(||items.clone()).sort_by(|a,b|at(a).total_cmp(&at(b)));}
        if let Some(callouts)=callouts {normalized.get_or_insert_with(||v.clone())["callouts"]=json!(callouts);}
        else if v.get("callouts").is_none() {normalized.get_or_insert_with(||v.clone())["callouts"]=json!(items);}
    }
    normalized
}

pub(super) fn render(s:&mut Scene,v:&Value,_brands:&BTreeMap<String,String>)->Result<(),String> {
    let normalized=normalized_strings(v);let v=normalized.as_ref().unwrap_or(v);
    render_normalized(s,v,_brands)
}
fn render_normalized(s:&mut Scene,v:&Value,_brands:&BTreeMap<String,String>)->Result<(),String> {
    let family=text(v,"template")?;let variant=text(v,"variant")?;
    let effective=if family=="title"&&variant!="text"&&v.get("icon").is_none() {
        "text"
    } else if family=="list"&&matches!(variant,"cards"|"spotlight") {
        let items=v["items"].as_array().ok_or("list items missing")?;
        let icons=items.iter().all(|item|item["icon"].as_str().is_some_and(|icon|!icon.is_empty()));
        if !icons||(variant=="cards"&&items.len()>4)||(variant=="spotlight"&&items.iter().any(|item|item["at"].as_f64().is_none())) { "bullets" } else { variant }
    } else { variant };
    if effective!=variant {
        let mut normalized=v.clone();
        normalized["variant"]=json!(effective);
        return render_normalized(s,&normalized,_brands);
    }
    if family=="rule-exception" {
        let items=v["items"].as_array().ok_or("rule-exception items missing")?;
        if let Some(index)=items.iter().position(|item|matches!(item["role"].as_str(),Some("default"|"rule"))).filter(|index|*index>0) {
            let mut normalized=v.clone();
            let items=normalized["items"].as_array_mut().expect("rule-exception items");
            let default=items.remove(index);
            items.insert(0,default);
            return render_normalized(s,&normalized,_brands);
        }
    }
    if family=="principle" {
        let items=v["items"].as_array().ok_or("principle items missing")?;
        let first=items.first().and_then(|item|item["rel"].as_str()).unwrap_or("follows");
        let mixed=items.iter().any(|item|item["rel"].as_str().unwrap_or("follows")!=first);
        let effective=if mixed||(first!="follows"&&items.len()>1&&variant=="satellites") {
            "chain"
        } else if variant=="chain"&&first=="follows" { "satellites" } else { variant };
        if effective!=variant {
            let mut normalized=v.clone();
            normalized["variant"]=json!(effective);
            return render_normalized(s,&normalized,_brands);
        }
    }
    let empty=Vec::new();let items=v["items"].as_array().unwrap_or(&empty);let n=items.len();
    if n==0&&!matches!(family,"title"|"statement"|"media"|"illustration") {return Err("layout requires items".into());}
    let bw=s.w-2.0*s.margin;let top=s.top;let bot=s.bottom;let bh=bot-top;let mx=s.margin;let cx=s.w/2.0;let cy=(top+bot)/2.0;let tall=s.tall;
    let f=v.get("focus").filter(|f|!f.is_null());
    let mut p=Plan{s,nodes:Vec::new(),edges:Vec::new()};
    if !matches!(family,"title"|"statement"|"media"|"illustration"){p.heading(v);}
    match (family,variant) {
        ("title"|"statement"|"media",_)=>p.opener(v)?,
        ("illustration",_)=>p.illustration(v,variant)?,
        ("list",_)=>p.list(v,variant)?,
        ("framework","hub")=>{
            let root=f.map(|f|p.add("focus",(cx-130.0,cy-50.0,260.0,100.0),f,"focus",json!({})));
            let rx=if tall {bw/2.0-110.0}else{430.0};let ry=if tall {bh/2.0-40.0}else{160.0};
            for (i,it) in items.iter().enumerate() {let a=-PI/2.0+TAU*i as f64/n as f64-if tall&&n%2==0 {PI/n as f64}else{0.0};let w=p.fitw(it,24.0,if it.get("icon").and_then(Value::as_str).is_some_and(|i|!i.is_empty()){104.0}else{48.0},if tall {170.0}else{190.0},if tall {220.0}else{250.0});let q=p.add("row",(cx+rx*a.cos()-w/2.0,cy+ry*a.sin()-36.0,w,72.0),it,"item",json!({}));if let Some(r)=root {p.edge(r,q,"structure",None);}}
        }
        ("framework","canopy")=>{
            let root=f.map(|f|{let w=p.fitw(f,26.0,116.0,260.0,440.0_f64.min(bw));p.add("focus",(cx-w/2.0,top+8.0,w,84.0),f,"focus",json!({}))});
            if tall {let cols=if n<=1 {1}else{2};let rows=n.div_ceil(cols);let g=20.0;let y=if f.is_some(){top+132.0}else{top};let w=(bw-g*(cols-1) as f64)/cols as f64;let h=150.0_f64.min(((bot-y-g*(rows-1) as f64)/rows as f64).floor());for (i,it) in items.iter().enumerate(){let r=i/cols;let nr=cols.min(n-r*cols);let q=p.add("item",(cx-(nr as f64*w+(nr-1) as f64*g)/2.0+(i%cols) as f64*(w+g),y+r as f64*(h+g),w,h),it,"item",json!({}));if let Some(root)=root{p.edge(root,q,"structure",Some(("b","t")));}}}
            else {let g=24.0;let w=240.0_f64.min((bw-(n-1) as f64*g)/n as f64);let h=if n>4 {156.0}else{172.0};let y=if f.is_some(){bot-h-24.0}else{cy-h/2.0};let x=cx-(n as f64*w+(n-1) as f64*g)/2.0;for (i,it) in items.iter().enumerate(){let q=p.add("item",(x+i as f64*(w+g),y,w,h),it,"item",json!({}));if let Some(root)=root{p.edge(root,q,"structure",Some(("b","t")));}}}
        }
        ("comparison","lanes")=>{
            let h=(if tall {220.0_f64}else{160.0_f64}).min((bh-(n-1) as f64*20.0)/n as f64);let y0=cy-(n as f64*h+(n-1) as f64*20.0)/2.0;
            for (i,it) in items.iter().enumerate(){let y=y0+i as f64*(h+20.0);p.add("band",(mx,y,bw,h),&Value::Null,"extra",json!({"lane":i}));let q=p.add(if tall||h<116.0 {"row"}else{"rowBig"},if tall {(mx+16.0,y+12.0,bw-32.0,64.0)}else{(112.0,y+14.0,352.0,h-28.0)},it,"item",json!({}));let (x,py,w)=if tall {(mx+16.0,y+88.0,bw-32.0)}else{(496.0,y+if details(it).len()>3 {h/2.0-52.0}else{h/2.0-24.0},672.0)};let shown=p.pills(q,details(it),x,py,w,"left",true);if shown<details(it).len(){p.pills(q,&details(it)[shown..],x,py+56.0,w,"left",true);}}
        }
        ("comparison","versus")=>{
            let rows=items.iter().map(|it|details(it).len()).max().unwrap_or(0);let two=rows>3;let rowh=if two {56.0}else{60.0};
            if tall {let height=|it:&Value|96.0+if details(it).is_empty(){0.0}else{14.0+details(it).len().div_ceil(2) as f64*56.0-8.0};let total=items.iter().map(height).sum::<f64>()+(n-1) as f64*64.0;let mut y=cy-total/2.0;for (i,it) in items.iter().enumerate(){if i>0 {p.add("vs",(cx-40.0,y-56.0,80.0,48.0),&json!({"label":"vs"}),"extra",json!({"fs":34,"lh":48,"align":"center","vsBefore":p.nodes.len()+1}));}let q=p.add("rowBig",(mx,y,bw,96.0),it,"item",json!({"col":i}));let w=(bw-10.0)/2.0;for (r,d) in details(it).iter().enumerate(){p.add("pillWide",(mx+(r%2) as f64*(w+10.0),y+110.0+(r/2) as f64*56.0,w,48.0),&json!({"label":d,"at":it["detailsAt"][r]}),"detail",json!({"parent":q,"row":r,"col":i,"fs":20}));}y+=height(it)+64.0;}}
            else {let prow=items.iter().map(|it|if details(it).len()>3 {details(it).len().div_ceil(2)}else{details(it).len()}).max().unwrap_or(0);let w=(bw-(n-1) as f64*96.0)/n as f64;let h=120.0+if rows>0 {22.0+prow as f64*rowh-8.0}else{0.0};let y=cy-h/2.0;for (i,it) in items.iter().enumerate(){let x=mx+i as f64*(w+96.0);let q=p.add("rowBig",(x,y,w,120.0),it,"item",json!({"col":i}));let tw=details(it).len()>3;for (r,d) in details(it).iter().enumerate(){let pw=if tw {(w-10.0)/2.0}else{w};let px=x+if tw {(r%2) as f64*(pw+10.0)}else{0.0};let py=y+142.0+if tw {(r/2) as f64*rowh}else{r as f64*rowh};let mut ex=json!({"parent":q,"row":r,"col":i});if tw {ex["fs"]=json!(20);}p.add("pillWide",(px,py,pw,if tw {48.0}else{52.0}),&json!({"label":d,"at":it["detailsAt"][r]}),"detail",ex);}if i>0 {p.add("vs",(x-88.0,y+36.0,80.0,48.0),&json!({"label":"vs"}),"extra",json!({"fs":34,"lh":48,"align":"center","vsBefore":q}));}}}
        }
        ("categories","shelves"|"constellation")=>{
            if tall {let heights:Vec<f64>=items.iter().map(|it|64.0+if details(it).is_empty(){0.0}else{12.0+48.0*details(it).len().div_ceil(3).min(2) as f64}).collect();let mut y=cy-(heights.iter().sum::<f64>()+(n-1) as f64*22.0)/2.0;for(i,it)in items.iter().enumerate(){let w=p.fitw(it,24.0,if it["icon"].as_str().is_some_and(|s|!s.is_empty()){104.0}else{48.0},200.0,bw).min(bw);let q=p.add("rowPill",(mx,y,w,64.0),it,"item",json!({}));let shown=p.pills(q,details(it),mx+16.0,y+76.0,bw-16.0,"left",false);if shown<details(it).len(){p.pills(q,&details(it)[shown..],mx+16.0,y+124.0,bw-16.0,"left",false);}y+=heights[i]+22.0;}}
            else if variant=="shelves" {let h=72.0_f64.min((bh-(n-1) as f64*14.0)/n as f64);let y=cy-(n as f64*h+(n-1) as f64*14.0)/2.0;for(i,it)in items.iter().enumerate(){let yy=y+i as f64*(h+14.0);let q=p.add("row",(mx,yy,320.0,h),it,"item",json!({}));p.pills(q,details(it),448.0,yy+h/2.0-20.0,736.0,"left",false);}}
            else {let cols=if n<=3 {n}else if n==4 {2}else{3};let rows=n.div_ceil(cols);let cell=bw/cols as f64;for(i,it)in items.iter().enumerate(){let r=i/cols;let nr=cols.min(n-r*cols);let xx=cx+((i%cols) as f64-(nr-1) as f64/2.0)*cell;let y=if rows==1 {cy-78.0}else{top+6.0+r as f64*192.0};let w=(cell-48.0).min(p.fitw(it,24.0,if it["icon"].as_str().is_some_and(|s|!s.is_empty()){104.0}else{48.0},168.0,280.0));let q=p.add("rowPill",(xx-w/2.0,y,w,64.0),it,"item",json!({}));let max=cell-40.0;let mut ds=Vec::new();let mut used=0.0;for d in details(it){let dw=max.min(p.pillw(d.as_str().unwrap(),20.0));let gap=if ds.is_empty(){0.0}else{20.0};if used+gap+dw<=max {used+=gap+dw;ds.push((d,dw));}}let mut x=xx-used/2.0;for(d,w)in ds {let b=p.add("pill",(x,y+116.0,w,40.0),&json!({"label":d,"icon":""}),"detail",json!({"parent":q}));p.edge(q,b,"affinity",Some(("b","t")));x+=w+20.0;}}}
        }
        ("tool-selection","mode-columns"|"switchboard")=>{
            if tall {if let Some(f)=f {let w=p.fitw(f,26.0,116.0,280.0,bw);p.add("focus",(cx-w/2.0,top-10.0,w,84.0),f,"focus",json!({}));}let heights:Vec<f64>=items.iter().map(|it|84.0+if details(it).is_empty(){0.0}else{52.0}).collect();let otop=if f.is_some(){top+110.0}else{top};let total=heights.iter().sum::<f64>()+(n-1) as f64*24.0;let mut y=otop+((bot-otop-total)/2.0).max(0.0);for(i,it)in items.iter().enumerate(){let q=p.add("row",(mx+40.0,y,bw-40.0,84.0),it,"item",json!({"mode":i}));p.pills(q,details(it),mx+40.0,y+94.0,bw-40.0,"left",false);y+=heights[i]+24.0;}}
            else if variant=="mode-columns" {let root=f.map(|f|{let w=p.fitw(f,26.0,116.0,280.0,440.0);p.add("focus",(cx-w/2.0,top+4.0,w,84.0),f,"focus",json!({}))});let w=304.0_f64.min((bw-(n-1) as f64*32.0)/n as f64);let x=cx-(n as f64*w+(n-1) as f64*32.0)/2.0;let y=if f.is_some(){top+148.0}else{cy-78.0-if items.iter().any(|it|!details(it).is_empty()){30.0}else{0.0}};for(i,it)in items.iter().enumerate(){let xx=x+i as f64*(w+32.0);let q=p.add("item",(xx,y,w,152.0),it,"item",json!({"mode":i}));if let Some(root)=root {p.edge(root,q,"structure",Some(("b","t")));}p.pills(q,details(it),xx-8.0,y+168.0,w+16.0,"center",false);}}
            else {let root=f.map(|f|p.add("slab",(mx,cy-116.0,280.0,232.0),f,"focus",json!({})));let h=84.0_f64.min((bh-(n-1) as f64*20.0)/n as f64);let y=cy-(n as f64*h+(n-1) as f64*20.0)/2.0;let x=if f.is_some(){472.0}else{296.0};for(i,it)in items.iter().enumerate(){let yy=y+i as f64*(h+20.0);let q=p.add("row",(x,yy,300.0,h),it,"item",json!({"mode":i}));if let Some(root)=root{p.edge(root,q,"structure",Some(("r","l")));}p.pills(q,details(it),x+324.0,yy+h/2.0-20.0,s.w-mx-x-324.0,"left",false);}}
        }
        ("rule-exception","detour"|"branch")=>{
            let root=if let Some(f)=f {let w=p.fitw(f,26.0,116.0,if tall {240.0}else{220.0},if tall {bw}else{300.0});p.add("focus",if tall {(cx-w/2.0,top-10.0,w,100.0)}else{(mx,cy-56.0,w,112.0)},f,"focus",json!({}))}else{p.add("dot",if tall {(cx-14.0,top,28.0,28.0)}else{(if variant=="detour"&&n<=3 {200.0}else{300.0},cy-14.0,28.0,28.0)},&Value::Null,"focus",json!({}))};
            if tall {let total=110.0+(n-1) as f64*84.0+(n-1) as f64*24.0;let t=top+140.0;let mut y=t+((bot-t-total)/2.0).max(0.0);for(i,it)in items.iter().enumerate(){let h=if i==0 {110.0}else{84.0};p.add(if i==0 {"rowRule"}else{"rowAlt"},(mx+56.0,y,bw-56.0,h),it,"item",json!({}));y+=h+24.0;}}
            else if variant=="detour"&&n<=3 {let d=p.add("rowRule",(848.0,cy-60.0,336.0,120.0),&items[0],"item",json!({}));p.edge(root,d,"flow",Some(("r","l")));for(k,it)in items[1..].iter().enumerate(){let q=p.add("rowAlt",(480.0,if k==0 {top}else{bot-80.0},248.0,80.0),it,"item",json!({}));p.edge(root,q,branch(it),Some((if k==0 {"t"}else{"b"},"l")));p.edge(q,d,branch(it),Some(("r",if k==0 {"t"}else{"b"})));}}
            else {let half=(n-1).div_ceil(2);let order:Vec<usize>=(1..1+half).chain(std::iter::once(0)).chain(1+half..n).collect();let sum=116.0+(n-1) as f64*80.0;let gap=52.0_f64.min((bh-sum)/(n-1).max(1) as f64);let mut y=cy-(sum+(n-1) as f64*gap)/2.0;for i in order {let h=if i==0 {116.0}else{80.0};let q=p.add(if i==0 {"rowRule"}else{"rowAlt"},(480.0,y,if i==0 {380.0}else{320.0},h),&items[i],"item",json!({}));p.edge(root,q,if i==0 {"flow"}else{branch(&items[i])},Some(("r","l")));y+=h+gap;}}
        }
        ("principle","chain"|"satellites"|"spotlight")=>{
            let (subj,sup)=if let Some(f)=f {(f,items.as_slice())}else{(&items[0],&items[1..])};
            if variant=="chain" {let all:Vec<&Value>=std::iter::once(subj).chain(sup.iter().take(4)).collect();let count=all.len();let g=if tall {if count>=5 {44.0}else{60.0}}else if count>=5 {64.0}else{84.0};let w=if tall {bw-120.0}else{250.0_f64.min((bw-(count-1) as f64*g)/count as f64)};let h=if tall {110.0_f64.min((bh-(count-1) as f64*g)/count as f64)}else{184.0};let x=cx-(count as f64*w+(count-1) as f64*g)/2.0;let y=if tall {cy-(count as f64*h+(count-1) as f64*g)/2.0}else{cy-h/2.0+10.0};let mut prev=None;for(i,it)in all.into_iter().enumerate(){let (edge,tag,tone)=relation(it);let mut item=it.clone();if i>0&&matches!(item["tone"].as_str(),None|Some(""|"default"))&&!tone.is_empty(){item["tone"]=json!(tone);}let xx=if tall {cx-w/2.0}else{x+i as f64*(w+g)};let yy=if tall {y+i as f64*(h+g)}else{y};let q=p.add(if i==0 {if tall {"focus"}else{"focusCol"}}else if tall {"row"}else if item["tone"]=="accent" {"itemSel"}else{"item"},(xx,yy,w,h),&item,if i==0 {"focus"}else{"item"},json!({}));if let Some(pr)=prev {p.edge(pr,q,edge,Some(if tall {("b","t")}else{("r","l")}));if !tag.is_empty(){let tw=p.pillw(tag,20.0);p.add("callout",if tall {(cx+18.0,yy-g/2.0-20.0,tw,40.0)}else{(xx-g/2.0-tw/2.0,yy+h/2.0-54.0,tw,40.0)},&json!({"label":tag,"icon":"","tone":tone}),"detail",json!({"fs":20,"callout":1,"tagOf":q}));}}prev=Some(q);}}
            else if variant=="satellites" {let root=p.add("focusCol",if tall {(cx-150.0,cy-75.0,300.0,150.0)}else{(510.0,cy-78.0,260.0,150.0)},subj,"focus",json!({}));let ys=if sup.len()>2 {[top-10.0,top+82.0,bot-164.0,bot-72.0]}else{[top-10.0,bot-72.0,top+82.0,bot-164.0]};let slots=[(mx,cy-41.0,"l","r"),(864.0,cy-41.0,"r","l"),(480.0,bot-72.0,"b","t"),(480.0,top,"t","b")];for(i,it)in sup.iter().take(4).enumerate(){let (x,y,sa,sb)=if tall {(cx-(bw-80.0)/2.0,ys[i],if ys[i]<cy {"t"}else{"b"},if ys[i]<cy {"b"}else{"t"})}else{slots[i]};let q=p.add("row",(x,y,if tall {bw-80.0}else{320.0},72.0),it,"item",json!({}));p.edge(root,q,"flow",Some((sa,sb)));}}
            else if tall {p.add("focus",(mx,top-10.0,bw,112.0),subj,"focus",json!({}));let g=22.0_f64.min((bh-142.0-sup.len() as f64*84.0)/(sup.len().saturating_sub(1)).max(1) as f64);for(i,it)in sup.iter().enumerate(){p.add("list",(mx+40.0,top+132.0+i as f64*(84.0+g),bw-40.0,84.0),it,"item",json!({}));}}
            else {let root=p.add("slabXL",(mx,top+4.0,456.0,bh-8.0),subj,"focus",json!({}));let g=26.0_f64.min((bh-sup.len() as f64*92.0)/(sup.len().saturating_sub(1)).max(1) as f64);let y=cy-(sup.len() as f64*92.0+sup.len().saturating_sub(1) as f64*g)/2.0;for(i,it)in sup.iter().enumerate(){let q=p.add("list",(cx,y+i as f64*(92.0+g),544.0,92.0),it,"item",json!({}));p.edge(root,q,"structure",Some(("r","l")));}}
        }
        ("sequence","track"|"lifecycle")=>{
            let mut ids=Vec::new();
            if variant=="lifecycle" {let w=if n>=5 {214.0_f64}else{248.0_f64};let w=if tall {w.min(220.0)}else{w};let h=if n>=5 {72.0}else{84.0};let rx=if tall {bw/2.0-w/2.0+10.0}else if n<=3 {330.0}else{390.0};let ry=if tall {260.0_f64.min(bh/2.0-h/2.0)}else{(if n<=3 {140.0_f64}else{160.0_f64}).min(cy-top-h/2.0).min(bot-cy-h/2.0)};for(i,it)in items.iter().enumerate(){let a=-PI/2.0-if n==4 {PI/4.0}else{0.0}+TAU*i as f64/n as f64;let yy=if n==4 {cy+a.sin().signum()*(ry-4.0)}else{cy+ry*a.sin()};ids.push(p.add("row",(cx+rx*a.cos()-w/2.0,yy-h/2.0,w,h),it,"item",json!({})));}}
            else {let g=if tall {if n>=5 {30.0}else{44.0}}else if n>=5 {44.0}else{64.0};let w=if tall {bw-80.0}else{232.0_f64.min((bw-(n-1) as f64*g)/n as f64)};let h=if tall {100.0_f64.min((bh-(n-1) as f64*g)/n as f64)}else if n>=5 {160.0}else{180.0};let x=cx-(n as f64*w+(n-1) as f64*g)/2.0;let y=cy-(n as f64*h+(n-1) as f64*g)/2.0;for(i,it)in items.iter().enumerate(){ids.push(p.add(if tall {"row"}else{"item"},if tall {(cx-w/2.0,y+i as f64*(h+g),w,h)}else{(x+i as f64*(w+g),cy-h/2.0,w,h)},it,"item",json!({})));}}
            for pair in ids.windows(2){p.edge(pair[0],pair[1],"flow",if variant=="lifecycle" {None}else{Some(if tall {("b","t")}else{("r","l")})});}if variant=="lifecycle"&&n>2 {p.edge(*ids.last().unwrap(),ids[0],"flow",None);}
        }
        _=>return Err(format!("unsupported structural family/variant {family}/{variant}")),
    }
    p.callouts(v)?;
    let Plan{nodes,edges,..}=p;
    let nodes:Vec<Value>=nodes.into_iter().map(|node|{let mut v=node.item;let(x,y,w,h)=node.frame;v["t"]=json!(node.kind);v["x"]=json!(x);v["y"]=json!(y);v["w"]=json!(w);v["h"]=json!(h);v}).collect();
    let edges:Vec<Value>=edges.into_iter().map(|edge|{let a=&nodes[edge.a];let b=&nodes[edge.b];let dx=b["x"].as_f64().unwrap()+b["w"].as_f64().unwrap()/2.0-a["x"].as_f64().unwrap()-a["w"].as_f64().unwrap()/2.0;let dy=b["y"].as_f64().unwrap()+b["h"].as_f64().unwrap()/2.0-a["y"].as_f64().unwrap()-a["h"].as_f64().unwrap()/2.0;let sides=edge.sides.unwrap_or(if dx.abs()*0.45>dy.abs(){if dx>0.0{("r","l")}else{("l","r")}}else if dy>0.0{("b","t")}else{("t","b")});let mut out=edge.extra;out["i"]=json!(edge.a);out["j"]=json!(edge.b);out["k"]=json!(edge.kind);out["si"]=json!(sides.0);out["sj"]=json!(sides.1);if let Some(mode)=b.get("mode"){out["mode"]=mode.clone();}out}).collect();
    s.render_layout_nodes(v,&nodes,&edges)
}
