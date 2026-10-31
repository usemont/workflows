//! Source core/music.js: music is a second clock, never a replacement for authored timing.
use serde_json::{json, Value};
use crate::grammar::fields;
#[derive(Clone, Debug)]
pub(crate) struct Music { pub beats: std::sync::Arc<[f64]>, down: std::sync::Arc<[bool]>, pub(crate) energy: std::sync::Arc<[(f64,f64)]>, pub snap: bool }
#[derive(Clone, Copy, Debug)]
pub(crate) struct Beat { pub i: isize, pub dt: f64, pub down: bool, pub e: f64 }
impl Beat { pub fn pulse(self, decay:f64)->f64 { if self.i<0 {0.0} else if self.dt<0.0 {1.0+self.dt/0.035} else {(-self.dt/if decay==0.0 {0.14}else{decay}).exp()} } }
fn time(v:&Value)->Result<f64,String>{let t=if let Some(n)=v.as_f64(){n}else if let Some(s)=v.as_str(){let mut n=0.0;for p in s.trim().replace(',',".").split(':'){n=n*60.0+p.parse::<f64>().map_err(|_|"invalid music time")?;}n}else{return Err("music time must be a number or timestamp".into())};if t.is_finite(){Ok(t)}else{Err("music time must be finite".into())}}
fn nearest(a:&[f64],t:f64)->Option<usize>{if a.is_empty(){return None}let i=a.partition_point(|b|*b<t).min(a.len()-1);Some(if i>0&&(a[i-1]-t).abs()<=(a[i]-t).abs(){i-1}else{i})}
fn within(a:&[f64],t:f64,w:f64)->Option<f64>{nearest(a,t).and_then(|i|((a[i]-t).abs()<=w).then_some(a[i]))}
impl Music {
 pub fn parse(value:Option<&Value>,until:f64)->Result<Option<Self>,String>{
  let Some(v)=value.filter(|v|!v.is_null()) else{return Ok(None)};
  if !v.is_array(){fields(v,&["beats","downbeats","bpm","offset","energy","beatsPerBar","snapTransitions"],"music")?;}
  let list=if v.is_array(){Some(v)}else{v.get("beats")}.map(|b|b.as_array().ok_or("music.beats must be an array")).transpose()?;
  let mut beats=Vec::new();let mut object_down=Vec::new();
  for b in list.into_iter().flatten(){let t=if b.is_object(){fields(b,&["t","time","down"],"music beat")?;if let Some(d)=b.get("down"){if !d.is_boolean(){return Err("music beat.down must be boolean".into())}}time(b.get("t").or_else(||b.get("time")).ok_or("music beat requires t or time")?)?}else{time(b)?};beats.push(t);if b["down"].as_bool()==Some(true){object_down.push(t);}}
  let bpm=v.get("bpm").map(|b|b.as_f64().filter(|n|n.is_finite()&&*n>=40.0&&*n<=240.0).ok_or("music.bpm must be between 40 and 240")).transpose()?;
  let offset=v.get("offset").map(time).transpose()?.unwrap_or(0.0);
  if beats.len()<2 {if let Some(bpm)=bpm {let count=((until-offset)/(60.0/bpm)).ceil().max(0.0);if count>1_000_000.0{return Err("music grid exceeds one million beats".into())}beats=(0..count as usize).map(|i|((offset+i as f64*60.0/bpm)*1000.0).round()/1000.0).collect();}}
  beats.retain(|b|*b>=0.0);beats.sort_by(f64::total_cmp);let original=beats;let mut beats=Vec::new();for(i,b)in original.iter().enumerate(){if i==0||b-original[i-1]>0.12{beats.push(*b)}}
  let mut downs=v.get("downbeats").map(|a|a.as_array().ok_or("music.downbeats must be an array")).transpose()?.map(|a|a.iter().map(time).collect::<Result<Vec<_>,_>>()).transpose()?.unwrap_or_default();if !object_down.is_empty(){downs=object_down;}
  let per=v.get("beatsPerBar").map(|n|n.as_f64().filter(|n|n.is_finite()).ok_or("music.beatsPerBar must be finite")).transpose()?.unwrap_or(4.0).round();let per=if per==0.0{4}else{per.clamp(1.0,8.0) as usize};let mut down=vec![false;beats.len()];if downs.is_empty(){for i in (0..beats.len()).step_by(per){down[i]=true}}else{for d in downs{if let Some(i)=nearest(&beats,d){if (beats[i]-d).abs()<0.1{down[i]=true}}}}
  let mut energy=Vec::new();if let Some(es)=v.get("energy"){for e in es.as_array().ok_or("music.energy must be an array")?{let(t,val)=if let Some(a)=e.as_array(){if a.len()!=2{return Err("music energy pair requires time and value".into())}(time(&a[0])?,&a[1])}else{fields(e,&["t","v","value"],"music energy")?;(time(e.get("t").ok_or("music energy requires t")?)?,e.get("v").or_else(||e.get("value")).ok_or("music energy requires value")?)};let val=val.as_f64().filter(|n|n.is_finite()).ok_or("music energy must be finite")?;energy.push((t,val.clamp(0.0,1.0)));}}energy.sort_by(|a,b|a.0.total_cmp(&b.0));
  let snap=v.get("snapTransitions").map(|s|s.as_bool().ok_or("music.snapTransitions must be boolean")).transpose()?.unwrap_or(true);
  Ok((beats.len()>=2).then_some(Self{beats:beats.into(),down:down.into(),energy:energy.into(),snap}))
 }
 pub fn energy_at(&self,t:f64)->f64{let Some(first)=self.energy.first()else{return 1.0};if t<=first.0{return first.1}for pair in self.energy.windows(2){let(a,b)=(pair[0],pair[1]);if t<=b.0{return a.1+(b.1-a.1)*(t-a.0)/(b.0-a.0).max(1e-6)}}self.energy.last().unwrap().1}
 pub fn beat_at(&self,t:f64)->Beat{let i=self.beats.partition_point(|b|*b<=t) as isize-1;let next=(i+1)as usize;let (i,dt)=if next<self.beats.len()&&self.beats[next]-t<0.035{(next as isize,t-self.beats[next])}else{(i,if i>=0{t-self.beats[i as usize]}else{99.0})};Beat{i,dt,down:i>=0&&self.down[i as usize],e:self.energy_at(t)}}
 pub fn amp(&self,t:f64,amount:f64,decay:f64,weak:f64)->f64{let b=self.beat_at(t);amount*b.pulse(decay)*b.e*if b.down{1.0}else{weak}}
 pub fn attach_beats(&self,doc:&mut Value){let start=doc["start"].as_f64().unwrap_or(0.0);let end=doc["end"].as_f64().unwrap_or(start);doc["_beats"]=json!(self.beats.iter().filter(|b|**b>=start-0.05&&**b<=end+0.6).map(|b|((b-start)*1000.0).round()/1000.0).collect::<Vec<_>>());}
 /// Source normVideo snaps adjacent boundaries unless snapTransitions is false.
 pub fn snap_boundaries(&self,scenes:&mut [Value]){if !self.snap{return}let downs=self.beats.iter().zip(self.down.iter()).filter_map(|(b,d)|d.then_some(*b)).collect::<Vec<_>>();for k in 1..scenes.len(){let(a,b)=scenes.split_at_mut(k);let(a,b)=(&mut a[k-1],&mut b[0]);let(start,bound,end)=(a["start"].as_f64().unwrap(),a["end"].as_f64().unwrap(),b["end"].as_f64().unwrap());if (bound-b["start"].as_f64().unwrap()).abs()>0.01{continue}let Some(to)=within(&downs,bound,0.4).or_else(||within(&self.beats,bound,0.2))else{continue};let ats=|s:&Value|{let st=s["start"].as_f64().unwrap();let en=s["end"].as_f64().unwrap();let mut ts=vec![s["subtitleAt"].as_f64(),s["highlightAt"].as_f64(),s["focus"]["at"].as_f64()];if let Some(items)=s["items"].as_array(){ts.extend(items.iter().map(|i|i["at"].as_f64()));}ts.into_iter().flatten().filter(|t|*t>st&&*t<en).collect::<Vec<_>>()};let lo=ats(a).into_iter().map(|t|t+1.5).fold(start+2.5,f64::max);let hi=ats(b).into_iter().map(|t|t-0.15).fold(end-2.5,f64::min);if to>=lo&&to<=hi{a["end"]=json!(to);b["start"]=json!(to);}}}
}
#[cfg(test)]mod tests{use super::*;
#[test]fn source_anticipation_downbeats_energy_and_snap_constraints(){let m=Music::parse(Some(&json!({"beats":[0,0.5,1,1.5,2,2.5,3,3.5,4,4.5,5,5.5,6,6.5,7,7.5,8,8.5,9,9.5,10],"energy":[[0,0],[2,1]]})),12.0).unwrap().unwrap();assert!((m.amp(0.9825,1.0,0.14,0.4)-0.09825).abs()<1e-9);assert_eq!(m.beat_at(2.0).down,true);assert_eq!(m.amp(2.0,1.0,0.14,0.4),1.0);let mut s=vec![json!({"start":0,"end":5.1,"items":[]}),json!({"start":5.1,"end":10,"items":[]})];m.snap_boundaries(&mut s);assert_eq!(s[0]["end"],5.0);s[0]["end"]=json!(5.1);s[1]["start"]=json!(5.1);s[0]["items"]=json!([{"at":4.0}]);m.snap_boundaries(&mut s);assert_eq!(s[0]["end"],5.1);}
#[test]fn automatic_reveals_snap_boundaries_but_authored_reveals_keep_their_clock(){
 let m=Music::parse(Some(&json!({"bpm":120})),12.0).unwrap().unwrap();
 let mut doc=json!({"template":"list","variant":"bullets","start":0,"end":8,"title":"Music","items":[{"label":"Automatic"},{"label":"Authored","at":2.3}]});
 m.attach_beats(&mut doc);
 let brands=std::collections::BTreeMap::new();
 let mut scene=crate::scene::Scene::new("music-consumer".into(),1280,720,8000.0,0.0,crate::scene::Theme::get("daylight").unwrap());
 crate::layout::render(&mut scene,&doc,&brands).unwrap();
 let items=scene.transition_nodes.iter().filter(|n|n["grp"]=="item").collect::<Vec<_>>();
 assert_eq!(items[0]["a"],1.0);assert_eq!(items[1]["a"],2.3);
 let mut video=json!({"music":{"bpm":120},"scenes":[{"template":"title","variant":"text","title":"A","start":0,"end":5.1},{"template":"title","variant":"text","title":"B","start":5.1,"end":10,"transition":"cut"}]});
 let spans=crate::compile_video(&video,1280,720).unwrap();
 let end=spans.iter().map(|s|s["placement"]["startMs"].as_i64().unwrap()+s["placement"]["durationMs"].as_i64().unwrap()).max().unwrap();assert_eq!(end,10000);
 assert_eq!(spans[0]["placement"]["durationMs"],4989);
 video["music"]["snapTransitions"]=json!(false);
 let unsnapped=crate::compile_video(&video,1280,720).unwrap();
 assert_eq!(unsnapped[0]["placement"]["durationMs"],5089);
 assert!(Music::parse(Some(&json!({"bpm":120,"unknown":true})),10.0).is_err());
}
}
