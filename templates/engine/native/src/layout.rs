use crate::{grammar::text,scene::Scene};
use serde_json::{json,Value};
use std::collections::BTreeMap;
#[path="layout_geometry.rs"] mod geometry;

pub(crate) fn render<'a>(s:&mut Scene<'a>,v:&Value,brands:&'a BTreeMap<String,String>)->Result<(),String>{
    s.brands=Some(brands);
    match text(v,"template")? {
        family @ ("action"|"story")=>{
            let normalized=geometry::normalized_strings(v);let v=normalized.as_ref().unwrap_or(v);
            if family=="action"{crate::narrative::action(s,v,brands)}else{crate::narrative::story(s,v,brands)}
        },
        _=>geometry::render(s,v,brands),
    }
}

pub(crate) fn branded_node(s:&mut Scene,id:&str,item:&Value,frame:(f64,f64,f64,f64),focus:bool,at:f64,until:Option<f64>,_brands:&BTreeMap<String,String>)->Result<(),String>{
    let(x,y,w,h)=frame;
    s.layout_node(item["t"].as_str().unwrap_or(if focus{"focus"}else{"row"}),id,item,x,y,w,h,at,until)
}
