fn source_hash(s:&str)->u32{s.encode_utf16().fold(2166136261,|h,c|(h^c as u32).wrapping_mul(16777619))}
fn source_random(state:&mut u32)->f64{*state=state.wrapping_add(0x6D2B79F5);let mut t=(*state^(*state>>15)).wrapping_mul(1|*state);t=t.wrapping_add((t^(t>>7)).wrapping_mul(61|t))^t;((t^(t>>14))as f64)/4294967296.0}
fn source_picture_hits(nodes:&mut[Value],edges:&[Value],times:&[f64],draw:f64){
    for(e,&at)in edges.iter().zip(times){
        let(i,j)=(e["i"].as_u64().unwrap()as usize,e["j"].as_u64().unwrap()as usize);
        let(a,b)=(&nodes[i],&nodes[j]);
        let coordinate=|n:&Value,k:&str|n[k].as_f64().unwrap_or(0.0);
        let dx=coordinate(b,"x")+coordinate(b,"w")/2.0-coordinate(a,"x")-coordinate(a,"w")/2.0;
        let dy=coordinate(b,"y")+coordinate(b,"h")/2.0-coordinate(a,"y")-coordinate(a,"h")/2.0;
        let length=dx.hypot(dy);let length=if length==0.0{1.0}else{length};
        let blocked=e["k"]=="blocked";let target=if blocked{i}else{j};
        if nodes[target]["ptl"].is_null(){continue}
        let at=e["fixA"].as_f64().unwrap_or(at)+draw*if blocked{0.6}else{1.0};
        let sign=if blocked{-1.0}else{1.0};
        if nodes[target]["hits"].is_null(){nodes[target]["hits"]=json!([]);}
        nodes[target]["hits"].as_array_mut().expect("source picture hits").push(json!({"t":at,"dx":dx/length*sign,"dy":dy/length*sign}));
    }
}
fn source_mode_steps(doc:&Value,nodes:&[Value],motion_duration:f64,duration:f64)->(Vec<f64>,usize){
    let(count,last)=nodes.iter().filter(|n|!n["mode"].is_null()).fold((0,f64::NEG_INFINITY),|(count,last),n|(count+1,last.max(n["a"].as_f64().unwrap_or(0.0))));
    let mut out=Vec::new();if count==0{return(out,0)}
    let built=last+motion_duration+0.2;
    if let Some(beats)=doc["_beats"].as_array().filter(|b|b.len()>2){
        let per=(beats.last().unwrap().as_f64().unwrap()-beats[0].as_f64().unwrap())/(beats.len()-1)as f64;
        if per>0.0{
            let stride=(1.5/per).round().max(1.0)as usize;
            for(i,beat)in beats.iter().filter_map(Value::as_f64).filter(|b|*b>=built-0.1).enumerate(){if i%stride==0{out.push(beat);}}
            if let Some(&last)=out.last(){let mut last=last;while last<duration+2.0{last+=stride as f64*per;out.push(last);}return(out,count)}
        }
    }
    let mut at=built;while at<duration+2.0{out.push(at);at+=1.5;}(out,count)
}
impl<'a> Scene<'a>{
    pub(crate) fn prepare_layout_nodes(&mut self,doc:&Value,authored:&[Value],edges:&[Value])->Result<(Vec<Value>,Vec<f64>),String>{
        let template=doc["template"].as_str().ok_or("scene template missing")?;let variant=doc["variant"].as_str().unwrap_or("");let title=doc["title"].as_str().unwrap_or("");let key=format!("{template}{variant}{title}");let mut rng=source_hash(&key);
        let tempo=SOURCE_THEMES[self.theme.name]["motion"]["tempo"].as_f64().unwrap();let beat0=match template{"title"=>0.15,"list"=>0.24,"framework"=>0.13,"categories"=>0.12,"tool-selection"=>0.14,"principle"=>0.16,"sequence"=>0.34,"story"=>0.18,_=>0.2}*tempo;
        let start=doc["start"].as_f64().unwrap_or(self.start);let duration=doc["end"].as_f64().unwrap_or(start+self.duration/1000.0)-start;let latest=(duration-1.6).max(0.6);let rel=|v:&Value|v.as_f64().map(|v|(v-start).clamp(0.12,latest));
        let beats=doc["_beats"].as_array();let onbeat=|a:f64,after:f64|{let mut best=a;let mut delta=0.25;if let Some(beats)=beats{for b in beats.iter().filter_map(Value::as_f64){let d=(b-a).abs();if d<=delta&&b>after&&b>=0.1&&b<=latest{best=b;delta=d;}}}best};
        let mut nodes=authored.to_vec();let items=nodes.iter().enumerate().filter(|(_,n)|n["grp"]=="item").map(|(i,_)|i).collect::<Vec<_>>();let beat=if items.len()>1{beat0.min(1.3/(items.len()-1)as f64)}else{beat0};let opener=matches!(template,"title"|"statement"|"illustration"|"media");let mut times=vec![0.0;nodes.len()];let(mut focus_n,mut last_focus)=(0,-1.0);
        for(i,n)in nodes.iter().enumerate(){if n["grp"]=="heading"{times[i]=0.12;}else if n["grp"]=="focus"{times[i]=if let Some(t)=rel(&n["at"]){t}else{onbeat((if opener{0.25}else{0.5})*tempo+focus_n as f64*0.22*tempo+source_random(&mut rng)*0.03,last_focus+0.12)};last_focus=times[i];focus_n+=1;}}
        let focus_end=nodes.iter().enumerate().filter(|(_,n)|n["grp"]=="focus").map(|(i,_)|times[i]).fold(0.0,f64::max);let mut next=if opener{focus_end+0.45}else{(0.95*tempo).max(focus_end+0.4)};let mut prev=focus_end;let mut run=false;
        let period=beats.filter(|b|b.len()>1).map(|b|(b.last().unwrap().as_f64().unwrap()-b[0].as_f64().unwrap())/(b.len()-1)as f64).unwrap_or(0.0);let cascade=period!=0.0&&beat<period*0.6;
        for(k,&i)in items.iter().enumerate(){times[i]=if let Some(t)=rel(&nodes[i]["at"]){run=false;t}else if beats.is_none(){next+if k>0{(source_random(&mut rng)-0.5)*beat*0.4}else{0.0}}else if cascade&&run{next.min(latest)}else{run=true;onbeat(next,prev+0.12)};next=times[i]+beat;prev=times[i];}
        let mut timed=nodes.iter().enumerate().filter(|(_,n)|n["grp"]=="focus"||n["grp"]=="item").map(|(i,_)|i).collect::<Vec<_>>();timed.sort_by(|a,b|times[*a].total_cmp(&times[*b]));for p in timed.windows(2){if times[p[1]]-times[p[0]]<0.07{times[p[1]]=times[p[0]]+0.07+source_random(&mut rng)*0.03;}}
        let first=items.iter().map(|i|times[*i]).reduce(f64::min).unwrap_or(focus_end);let last=items.iter().map(|i|times[*i]).fold(focus_end,f64::max);let mut per_parent=std::collections::HashMap::<usize,usize>::new();
        for(i,n)in nodes.iter().enumerate(){if n["grp"]=="extra"{times[i]=if n["t"]=="band"{(items.get(n["lane"].as_u64().unwrap_or(0)as usize).map(|j|times[*j]).unwrap_or(first)-0.25).max(0.1)}else{latest.min(first+0.5+source_random(&mut rng)*0.05)};}}
        for(i,n)in nodes.iter().enumerate(){if n["grp"]=="detail"{let p=n["parent"].as_u64().map(|v|v as usize);let j=per_parent.entry(p.unwrap_or(usize::MAX)).or_default();*j+=1;times[i]=(latest+0.3).min(p.and_then(|j|times.get(j)).copied().unwrap_or(last)+0.42*tempo+(*j-1)as f64*0.09+source_random(&mut rng)*0.03);}}
        let mut edge_times=Vec::new();for e in edges{let i=e["i"].as_u64().ok_or("edge i missing")?as usize;let j=e["j"].as_u64().ok_or("edge j missing")?as usize;edge_times.push(times.get(i).ok_or("edge i out of range")?.max(*times.get(j).ok_or("edge j out of range")?)+0.26+source_random(&mut rng)*0.05);}
        for(i,n)in nodes.iter_mut().enumerate(){if !n["hl"].is_null(){let t=rel(&doc["highlightAt"]).map(|r|r.max(times[i]+0.2)).unwrap_or_else(||onbeat(times[i]+n["label"].as_str().unwrap_or("").split_whitespace().count()as f64*0.075+0.55,times[i]+0.3));n["hlA"]=json!(t);}if let Some(j)=n["vsBefore"].as_u64(){times[i]=(times[j as usize]-0.22).max(0.1);}if let Some(g)=n["goneAt"].as_f64(){n["gone"]=json!((times[i]+0.4).max(g-start));}}
        let versus=template=="comparison"&&variant=="versus";
        if versus{
            let mut sides=nodes.iter().enumerate().filter(|(_,n)|n["grp"]=="item"&&n["col"].is_number()).map(|(i,n)|(i,n["col"].as_i64().unwrap_or(0))).collect::<Vec<_>>();sides.sort_by_key(|p|p.1);
            if sides.len()>=2&&times[sides[1].0]-times[sides[0].0]>=1.6{let first=&nodes[sides[0].0];let dx=self.w/2.0-(first["x"].as_f64().unwrap()+first["w"].as_f64().unwrap()/2.0);let t=times[sides[1].0]-1.0;for n in &mut nodes{if n["col"].as_i64()==Some(0){n["mv"]=json!({"dx":dx,"t":t});}}}
        }
        let visual_list=template=="list"&&(variant=="cards"||variant=="spotlight");
        if versus||visual_list{
            let mut candidates=nodes.iter().enumerate().filter(|(_,n)|!n["at"].is_null()&&if versus{n["grp"]=="detail"}else{n["t"]=="cardTile"||n["t"]=="railRow"}).filter_map(|(i,n)|rel(&n["at"]).map(|t|(i,t))).collect::<Vec<_>>();candidates.sort_by(|a,b|a.1.total_cmp(&b.1));
            if versus||candidates.len()>=2{let mut queue=Vec::new();let mut previous=-1e9_f64;for(i,t)in candidates{let t=t.max(times[i]+if versus{0.45}else{0.35}).max(previous+0.55);if t>latest+1.2{continue;}queue.push((i,t));previous=t;}for(j,&(i,t))in queue.iter().enumerate(){let next=queue.get(j+1).map(|p|p.1).unwrap_or(t+3.5);let max=if versus{3.5}else if nodes[i]["t"]=="railRow"{30.0}else{4.5};nodes[i]["hb"]=json!({"t":t,"hold":(next-t-0.12).clamp(if versus{0.45}else{0.6},max)});}}
        }
        if let Some(windows)=doc["hide"].as_array(){for window in windows{let lo=window[0].as_f64().ok_or("hide start invalid")?;let hi=window[1].as_f64().ok_or("hide end invalid")?;let mut hidden=nodes.iter().enumerate().filter(|(i,n)|n["grp"]!="heading"&&start+times[*i]>=lo-0.05&&start+times[*i]<hi).map(|(i,_)|i).collect::<Vec<_>>();hidden.sort_by(|a,b|times[*a].total_cmp(&times[*b]));for(c,i)in hidden.into_iter().enumerate(){let na=hi-start+0.3+0.2*c as f64;times[i]=nodes[i]["gone"].as_f64().filter(|g|*g<=na).map(|g|g-0.01).unwrap_or(na);}}}
        for(i,n)in nodes.iter_mut().enumerate(){let rank=match n["grp"].as_str().unwrap_or(""){"item"=>1.0,"focus"=>2.0,"heading"=>3.0,_=>0.0};n["out"]=json!(rank*0.07+source_random(&mut rng)*0.05);n["seed"]=json!(source_hash(&format!("{key}:{i}")));n["ph"]=json!(source_random(&mut rng)*std::f64::consts::TAU);if let Some(t)=n["fixA"].as_f64(){times[i]=t;}}
        for(i,n)in nodes.iter().enumerate(){if let Some(j)=n["tagOf"].as_u64(){times[i]=times[j as usize]+0.3;}}
        for(i,n)in nodes.iter_mut().enumerate(){n["a"]=json!(times[i]);n["dur"]=json!(duration);}
        source_picture_hits(&mut nodes,edges,&edge_times,SOURCE_THEMES[self.theme.name]["motion"]["draw"].as_f64().expect("source draw duration"));
        (self.selection_steps,self.selection_count)=source_mode_steps(doc,&nodes,SOURCE_THEMES[self.theme.name]["motion"]["dur"].as_f64().expect("source motion duration"),duration);
        crate::typography::lock_sibling_type(self,&mut nodes);
        self.transition_nodes=nodes.clone();
        Ok((nodes,edge_times))
    }
    pub fn render_layout_nodes(&mut self,doc:&Value,authored:&[Value],edges:&[Value])->Result<(),String>{
        let(nodes,edge_times)=self.prepare_layout_nodes(doc,authored,edges)?;
        let duration=nodes.first().and_then(|n|n["dur"].as_f64()).unwrap_or(self.duration/1000.0);
        let routes=self.route_layout_edges(&nodes,edges)?;
        for(i,e)in edges.iter().enumerate(){
            let at=e["fixA"].as_f64().unwrap_or(edge_times[i]);
            let flashes=e["flashes"].as_array().map(|v|v.iter().filter_map(Value::as_f64).collect::<Vec<_>>()).unwrap_or_default();
            let until=e["gone"].as_f64().map(|g|g*1000.0).or_else(||self.exit_scene.then_some(self.exit_start.unwrap_or((duration-0.95)*1000.0)));
            self.routed_edge(&format!("edge-{i}"),&routes[i],e["k"].as_str().unwrap_or("structure"),at*1000.0,until,&flashes,e["mode"].as_u64().map(|m|m as usize));
        }
        for(i,node)in nodes.iter().enumerate(){
            let mut n=node.clone();
            if self.transition_holds.contains(&i){n["_transitionHold"]=json!(true);}
            let kind=n["t"].as_str().ok_or("node type missing")?;
            let coord=|key:&str|n[key].as_f64().ok_or_else(||format!("node {i} coordinate {key} missing"));
            let(x,y,w,h)=(coord("x")?,coord("y")?,coord("w")?,coord("h")?);
            let until=n["gone"].as_f64().map(|g|g*1000.0).or_else(||self.exit_scene.then_some(self.exit_start.unwrap_or((duration-0.95)*1000.0)));
            let id=format!("node-{i}");
            self.body.push_str(&format!("<MotionBox id=\"transition-node-{i}\" pivotX={{{}}} pivotY={{{}}}>",(x+w/2.0)*self.scale,(y+h/2.0)*self.scale));
            let at=n["a"].as_f64().unwrap()*1000.0;
            if !n["glyph"].is_null()||!n["base"].is_null(){let mut picture=n.clone();if picture["base"].is_null(){picture["base"]=n["glyph"].clone();}self.picture_tile(&id,&picture,x,y,w,h,at,until)?;}else{self.layout_node(kind,&id,&n,x,y,w,h,at,until)?;}
            self.end_move();
        }Ok(())
    }
}
#[cfg(test)]mod choreography_tests{use super::*;#[test]fn seeded_random_matches_javascript(){assert_eq!(source_hash("abc"),440920331);let mut state=1;assert!((source_random(&mut state)-0.6270739405881613).abs()<1e-15);assert_eq!(route_inout(0.5),0.5);}}
#[cfg(test)]mod source_pass_order_tests{
    use super::*;
    #[test]fn extras_consume_seeded_offsets_before_interleaved_details(){
        let theme=Theme::get("daylight").unwrap();
        let mut scene=Scene::new("timing".into(),1280,720,8000.0,0.0,theme);
        let doc=json!({"template":"comparison","variant":"versus","title":"Pass order","start":0,"end":8});
        let authored=vec![
            json!({"t":"focus","grp":"focus","label":"Focus"}),
            json!({"t":"pill","grp":"detail","parent":0}),
            json!({"t":"vs","grp":"extra"}),
            json!({"t":"pill","grp":"detail","parent":0}),
        ];
        let(nodes,_)=scene.prepare_layout_nodes(&doc,&authored,&[]).unwrap();
        let tempo=SOURCE_THEMES["daylight"]["motion"]["tempo"].as_f64().unwrap();
        let mut rng=source_hash("comparisonversusPass order");
        let focus=0.5*tempo+source_random(&mut rng)*0.03;
        let extra=focus+0.5+source_random(&mut rng)*0.05;
        let first=focus+0.42*tempo+source_random(&mut rng)*0.03;
        let second=focus+0.42*tempo+0.09+source_random(&mut rng)*0.03;
        assert_eq!(nodes[2]["a"],json!(extra));
        assert_eq!(nodes[1]["a"],json!(first));
        assert_eq!(nodes[3]["a"],json!(second));
    }
    #[test]fn connectors_are_emitted_below_cards_like_source_dom(){
        let mut scene=Scene::new("stacking".into(),1280,720,8000.0,0.0,Theme::get("daylight").unwrap());
        let doc=json!({"template":"framework","variant":"canopy","title":"Stacking","start":0,"end":8});
        let nodes=vec![
            json!({"t":"item","grp":"item","label":"First","icon":"","x":100,"y":200,"w":240,"h":160}),
            json!({"t":"item","grp":"item","label":"Second","icon":"","x":500,"y":200,"w":240,"h":160}),
        ];
        scene.render_layout_nodes(&doc,&nodes,&[json!({"i":0,"j":1,"k":"flow"})]).unwrap();
        let edge=scene.body.find("id=\"edge-0\"").unwrap();
        let node=scene.body.find("id=\"transition-node-0\"").unwrap();
        assert!(edge<node);
    }
}
#[cfg(test)]mod picture_hit_tests{
    use super::*;
    #[test]fn blocked_sender_recoils_and_receiving_picture_reacts_after_draw(){
        let mut nodes=vec![json!({"x":0,"y":0,"w":10,"h":10,"ptl":[]}),json!({"x":30,"y":40,"w":10,"h":10,"ptl":[]})];
        source_picture_hits(&mut nodes,&[json!({"i":0,"j":1,"k":"blocked","fixA":2.0}),json!({"i":0,"j":1,"k":"arrow"})],&[0.5,3.0],0.5);
        let a=&nodes[0]["hits"][0];let b=&nodes[1]["hits"][0];
        assert_eq!(a["t"],json!(2.3));assert_eq!(b["t"],json!(3.5));
        assert!((a["dx"].as_f64().unwrap()+0.6).abs()<1e-12);
        assert!((a["dy"].as_f64().unwrap()+0.8).abs()<1e-12);
        assert!((b["dx"].as_f64().unwrap()-0.6).abs()<1e-12);
        assert!((b["dy"].as_f64().unwrap()-0.8).abs()<1e-12);
    }
}
#[cfg(test)]mod selection_step_tests{
    use super::*;
    #[test]fn modes_start_after_build_and_advance_on_source_music_stride(){
        let nodes=vec![json!({"mode":0,"a":1.0}),json!({"mode":1,"a":2.0}),json!({"a":30.0})];
        let(steps,count)=source_mode_steps(&json!({}),&nodes,0.8,6.0);
        assert_eq!(count,2);assert_eq!(steps,vec![3.0,4.5,6.0,7.5]);
        let(steps,count)=source_mode_steps(&json!({"_beats":[0.0,1.0,2.0,3.0,4.0]}),&nodes,0.8,6.0);
        assert_eq!(count,2);assert_eq!(steps,vec![3.0,5.0,7.0,9.0]);
        assert_eq!(source_mode_steps(&json!({}),&[],0.8,6.0),(vec![],0));
    }
}
