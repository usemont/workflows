static SOURCE_THEMES: std::sync::LazyLock<Value> = std::sync::LazyLock::new(|| serde_json::from_str(include_str!("themes.json")).expect("checked source themes"));
static NODE_TYPES: std::sync::LazyLock<Value> = std::sync::LazyLock::new(|| serde_json::from_str(include_str!("node-types.json")).expect("checked node types"));

// Balance discrete event dispatch so long scenes do not create deeply nested JS syntax.
fn source_event_expression(clock:&str,events:&[(f64,String)],initial:&str,comparison:&str)->String {
    if events.is_empty(){return initial.to_owned();}
    let middle=events.len()/2;
    let (at,value)=&events[middle];
    let before=source_event_expression(clock,&events[..middle],initial,comparison);
    let after=source_event_expression(clock,&events[middle+1..],value,comparison);
    format!("{clock} {comparison} ({at}) ? ({after}) : ({before})")
}

impl<'a> Scene<'a> {
    /// Native expressions keep the source clock seekable, including fractional exits.
    pub(crate) fn number(&mut self, expression:impl AsRef<str>)->String {
        let name=format!("motion{}",self.counter);self.counter+=1;
        self.computeds.push(format!("{name}:computed.number(() => {})",expression.as_ref()));
        format!("computed.{name}")
    }
    pub(crate) fn string(&mut self, expression:impl AsRef<str>)->String {
        let name=format!("motion{}",self.counter);self.counter+=1;
        self.computeds.push(format!("{name}:computed.string(() => {})",expression.as_ref()));
        format!("computed.{name}")
    }
    /// Source f1/f3 use JavaScript Math.round: negative ties round toward +infinity.
    pub(crate) fn css_number(&mut self, expression:&str, digits:u32)->String {
        let factor=10_u32.pow(digits);
        self.number(format!("Math.floor(({expression}) * {factor} + 0.5) / {factor}"))
    }
    fn motion_music(&mut self)->(String,String,String) {
        if let Some(cached)=&self.music_motion{return cached.clone();}
        let Some(music)=self.music.clone()else{return ("-1".into(),"0".into(),"0".into())};
        let t=self.number(format!("{} + computed.sourceClock / 1000",self.start));
        let mut indices=Vec::new();let mut beat_times=Vec::new();let mut downbeats=Vec::new();
        for (i,&b) in music.beats.iter().enumerate() {
            indices.push((b-0.035,i.to_string()));
            beat_times.push((b-0.035,b.to_string()));
            downbeats.push((b-0.035,u8::from(music.beat_at(b).down).to_string()));
        }
        let index=self.number(source_event_expression(&t,&indices,"-1",">"));
        let beat_time=self.number(source_event_expression(&t,&beat_times,"0",">"));
        let down=self.number(source_event_expression(&t,&downbeats,"0",">"));
        let dt=self.number(format!("{t} - {beat_time}"));
        let initial=music.energy.first().map(|p|p.1).unwrap_or(1.0).to_string();
        let mut energy=Vec::new();
        for pair in music.energy.windows(2) {let(a,b)=(pair[0],pair[1]);energy.push((a.0,format!("{} + ({}) * ({t} - {}) / {}",a.1,b.1-a.1,a.0,(b.0-a.0).max(1e-6))));}
        if let Some(last)=music.energy.last(){energy.push((last.0,last.1.to_string()));}
        let energy=self.number(source_event_expression(&t,&energy,&initial,">"));
        let decay=SOURCE_THEMES[self.theme.name]["beat"]["decay"].as_f64().unwrap();
        let pulse=self.number(format!("{index} < 0 ? 0 : ({dt} < 0 ? 1 + {dt} / 0.035 : Math.pow({}, -{dt} / {decay})) * {energy}",std::f64::consts::E));
        self.music_motion=Some((index.clone(),pulse.clone(),down.clone()));
        (index,pulse,down)
    }
    pub fn push_native(&mut self, source: &str) { self.body.push_str(source); }
    pub fn native_scale(&self) -> f64 { self.scale }
    pub fn motion_duration(&self)->f64{self.duration.max(self.render_end)}
    pub fn begin_static_window(&mut self, id: &str, at: f64, until: f64) {
        self.body.push_str(&format!("<MotionBox id={} opacity={{0}}><Animate prop=\"opacity\" from={{0}} to={{1}} start=\"{at}ms\" duration=\"0ms\" ease=\"linear\"/><Animate prop=\"opacity\" from={{1}} to={{0}} start=\"{until}ms\" duration=\"0ms\" ease=\"linear\"/>",q(id)));
    }
    pub fn begin_transform(&mut self, id: &str, cx: f64, cy: f64, keys: &[(f64,f64,f64,f64,f64,f64,f64)]) {
        let Some(first) = keys.first() else { panic!("transform requires keyframes") };
        let s = self.scale;
        self.body.push_str(&format!("<MotionBox id={} x={{{}}} y={{{}}} scaleX={{{}}} scaleY={{{}}} rotation={{{}}} opacity={{{}}} pivotX={{{}}} pivotY={{{}}}>", q(id),first.1*s,first.2*s,first.3,first.4,first.5,first.6,cx*s,cy*s));
        for pair in keys.windows(2) {
            let (a,b) = (pair[0],pair[1]);
            for (prop,from,to) in [("x",a.1*s,b.1*s),("y",a.2*s,b.2*s),("scaleX",a.3,b.3),("scaleY",a.4,b.4),("rotation",a.5,b.5),("opacity",a.6,b.6)] {
                if from != to { self.body.push_str(&format!("<Animate prop={} from={{{from}}} to={{{to}}} start=\"{}ms\" duration=\"{}ms\" ease=\"linear\"/>",q(prop),a.0,b.0-a.0)); }
            }
        }
    }
    pub fn begin_projective_transform(&mut self,id:&str,cx:f64,cy:f64,perspective:f64,keys:&[(f64,f64,f64,f64,f64,f64,f64,f64)]){
        let first=keys.first().expect("projective transform requires keyframes");let s=self.scale;
        self.body.push_str(&format!("<MotionBox id={} x={{{}}} y={{{}}} scaleX={{{}}} scaleY={{{}}} rotation={{{}}} opacity={{{}}} rotateY={{{}}} perspective={{{}}} pivotX={{0}} pivotY={{{}}} perspectiveOriginX={{{}}} perspectiveOriginY={{{}}} backfaceVisibility=\"hidden\">",q(id),first.1*s,first.2*s,first.3,first.4,first.5,first.6,first.7,perspective*s,23.6*s,cx*s,(cy+23.6)*s));
        for pair in keys.windows(2){let(a,b)=(pair[0],pair[1]);for(prop,from,to)in[("x",a.1*s,b.1*s),("y",a.2*s,b.2*s),("scaleX",a.3,b.3),("scaleY",a.4,b.4),("rotation",a.5,b.5),("opacity",a.6,b.6),("rotateY",a.7,b.7)]{if from!=to{self.body.push_str(&format!("<Animate prop={} from={{{from}}} to={{{to}}} start=\"{}ms\" duration=\"{}ms\" ease=\"linear\"/>",q(prop),a.0,b.0-a.0));}}}
    }
    pub(crate) fn begin_theme_motion(&mut self, id: &str, item:&Value, x: f64, y: f64, w: f64, h: f64, at: f64, until: Option<f64>, pill: bool, idle: bool) {
        if item["_transitionHold"].as_bool()==Some(true){self.body.push_str(&format!("<MotionBox id={}>",q(id)));return;}
        let tok=&SOURCE_THEMES[self.theme.name]["motion"];
        let text=NODE_TYPES[item["t"].as_str().unwrap_or("")]["text"].as_i64()==Some(1);
        let duration=tok["dur"].as_f64().unwrap()*1000.0;
        let out=tok["outDur"].as_f64().unwrap()*1000.0;
        let t="computed.sourceClock";
        let secs=self.number(format!("{t} / 1000"));
        let k=self.number(format!("Math.min(1, Math.max(0, ({t} - ({at})) / {duration}))"));
        let expo=self.number(format!("{k} == 1 ? 1 : 1 - Math.pow(2, -10 * {k})"));
        let cubic=self.number(format!("1 - Math.pow(1 - {k}, 3)"));
        let (mut dx,mut dy,mut sc,mut rot,mut op)=("0".to_owned(),"0".to_owned(),"1".to_owned(),"0".to_owned(),"1".to_owned());
        match self.theme.name {
            "daylight"=>{let e=self.number(format!("1 - Math.pow(1 - {k}, 5)"));dy=format!("(1 - {e}) * {}",if pill{10}else{26});op=format!("Math.min(1, Math.max(0, {k} * 2.6))");if pill{sc=format!("0.92 + 0.08 * {e}");}},
            "midnight"=>{sc=format!("0.93 + 0.07 * {expo}");op=format!("Math.min(1, Math.max(0, {k} * 1.8))");},
            "blueprint"=>{dx=format!("(1 - {expo}) * -16");op=format!("{k} > 0 ? 1 : 0");},
            "signal"=>{sc=format!("0.55 + 0.45 * (1 + 2.7 * Math.pow({k} - 1, 3) + 1.7 * Math.pow({k} - 1, 2))");rot=format!("(1 - {cubic}) * {}",if pill{-8}else{-4});dy=format!("(1 - {cubic}) * -18");op=format!("Math.min(1, Math.max(0, {k} * 4))");},
            _=>unreachable!(),
        }
        let(amp,freq,decay,ant)=source_spring(self.theme.name);
        if let (Some(move_at),Some(move_dx))=(item["mv"]["t"].as_f64(),item["mv"]["dx"].as_f64()){
            let u=self.number(format!("{secs} - {move_at}"));
            let spring=self.number(format!("{u} <= 0 ? 0 : 1 - Math.pow({}, -{} * {u}) * Math.cos({} * {u})",std::f64::consts::E,decay*1.15,std::f64::consts::TAU*freq*0.8));
            let windup=self.number(format!("{u} < -{ant} ? 0 : ({u} < 0 ? 1 - Math.pow(1 - ({u} + {ant}) / {ant}, 3) : Math.max(0, 1 - {u} / 0.08))"));
            dx=format!("({dx}) + {move_dx} * (1 - {spring}) + {} * {windup}",if move_dx>0.0{10.0*amp}else{-10.0*amp});
        }
        let u=self.number(format!("Math.min(1, Math.max(0, ({t} - ({at}) - {}) / 1200))",duration*0.7));
        let weight=if idle{self.number(format!("{u} * {u} * (3 - 2 * {u})"))}else{"0".into()};
        let phase=item["ph"].as_f64().unwrap_or(0.0);let tau=std::f64::consts::TAU;
        let (beat,pulse,down)=if self.theme.name=="signal"||(!text&&matches!(item["grp"].as_str(),Some("focus"|"item"))){self.motion_music()}else{("-1".into(),"0".into(),"0".into())};
        match self.theme.name {
            "daylight"=>dy=format!("({dy}) + {weight} * 3 * Math.sin({tau} * {secs} / 5.4 + {phase})"),
            "midnight"=>sc=format!("({sc}) * (1 + {weight} * 0.014 * Math.sin({tau} * {secs} / 4.8 + {phase}))"),
            "blueprint"=>{dx=format!("({dx}) + {weight} * 2.2 * Math.sin({tau} * {secs} / 7.3 + {phase})");dy=format!("({dy}) + {weight} * 1.8 * Math.sin({tau} * {secs} / 9.1 + {})",phase*1.3);},
            "signal"=>{
                let fr=self.number(format!("{beat} >= 0 ? 100000 + {beat} : Math.floor(Math.floor({secs} * 30 + 0.5) / 10)"));
                let seed=item["seed"].as_u64().filter(|v|*v!=0).unwrap_or(1)as u32;
                let frames=(self.motion_duration()/1000.0*3.0).ceil()as u32+1;
                let mut jitter=[Vec::new(),Vec::new(),Vec::new()];
                let beat_count=self.music.as_ref().map(|m|m.beats.len()).unwrap_or(0)as u32;
                // Only the seeded, discrete random events are materialized; their idle envelope remains continuous.
                for frame in (0..=frames).chain((0..beat_count).map(|i|100000+i)){
                    let mut state=seed.wrapping_add(frame.wrapping_mul(7919));
                    for (i,amount)in[0.6,1.2,1.2].into_iter().enumerate(){let value=(source_random(&mut state)-0.5)*amount;jitter[i].push((frame as f64,value.to_string()));}
                }
                for events in &mut jitter{events.insert((frames+1)as usize,((frames+1)as f64,"0".into()));}
                let jr=self.number(source_event_expression(&fr,&jitter[0],"0",">="));
                let jx=self.number(source_event_expression(&fr,&jitter[1],"0",">="));
                let jy=self.number(source_event_expression(&fr,&jitter[2],"0",">="));
                rot=format!("({rot}) + {weight} * {jr}");dx=format!("({dx}) + {weight} * {jx}");dy=format!("({dy}) + {weight} * {jy}");
            },
            _=>unreachable!(),
        }
        if !text&&matches!(item["grp"].as_str(),Some("focus"|"item")){
            let bt=&SOURCE_THEMES[self.theme.name]["beat"];let part=if item["grp"]=="focus"{"focus"}else{"items"};let amount=bt[part].as_f64().unwrap_or(0.0);let weak=bt["weak"].as_f64().unwrap();
            sc=format!("({sc}) * (1 + {pulse} * ({down} > 0 ? 1 : {weak}) * {amount} * {weight} * {})",if pill{0.6}else{1.0});
            if item["grp"]=="item"{dy=format!("({dy}) - {} * {weight} * {pulse} * {down}",bt["hop"].as_f64().unwrap_or(0.0));}
        }
        let exit_delay=item["out"].as_f64().unwrap_or(0.0)*1000.0;
        let ko=until.map(|v|self.number(format!("Math.pow(Math.min(1, Math.max(0, ({t} - ({v}) - {exit_delay}) / {out})), 3)"))).unwrap_or_else(||"0".into());
        match self.theme.name {
            "daylight"=>{dy=format!("({dy}) - {ko} * 18");op=format!("({op}) * (1 - {ko})");},
            "midnight"=>{sc=format!("({sc}) * (1 + 0.06 * {ko})");op=format!("({op}) * (1 - {ko})");},
            "blueprint"=>{dx=format!("({dx}) + {ko} * 16");},
            "signal"=>{sc=format!("({sc}) * (1 - 0.18 * {ko})");dy=format!("({dy}) + {ko} * 30");op=format!("({op}) * (1 - {ko})");},
            _=>unreachable!(),
        }
        let before_exit=until.map(|v|format!("{t} < ({v})")).unwrap_or_else(||"true".into());
        if text{rot=format!("({rot}) * 0.3");op=format!("{before_exit} ? 1 : ({op})");sc=format!("{before_exit} ? 1 + (({sc}) - 1) * 0.3 : ({sc})");dx=format!("({dx}) * ({before_exit} ? 0.5 : 1)");dy=format!("({dy}) * ({before_exit} ? 0.5 : 1)");}
        let dim=if text{0.0}else if item["tone"]=="muted"&&item["glyph"].as_str().unwrap_or("").is_empty(){1.0}else{item["dim"].as_f64().unwrap_or(0.0)};
        op=format!("({op}) * {}",1.0-0.55*dim);
        rot=format!("({rot}) + {}",item["tilt"].as_f64().unwrap_or(0.0));
        if let Some(slides)=item["slides"].as_array(){for slide in slides{
            let u=self.number(format!("Math.min(1, Math.max(0, ({secs} - {}) / 0.55))",slide["t"].as_f64().unwrap()));
            let ease=self.number(format!("{u} < 0.5 ? 8 * Math.pow({u}, 4) : 1 - Math.pow(-2 * {u} + 2, 4) / 2"));
            dy=format!("({dy}) + {} * {ease}",slide["dy"].as_f64().unwrap());
        }}
        if let Some(mode)=item["mode"].as_u64().filter(|_|self.selection_count>0){
            let mut selected_events="0".to_owned();
            for(i,&at)in self.selection_steps.iter().enumerate(){let val=if i%self.selection_count==mode as usize{let u=format!("(Math.min(1, Math.max(0, ({secs} - {at}) / 0.35)) - 1)");format!("1 + 2.7 * Math.pow({u}, 3) + 1.7 * Math.pow({u}, 2)")}else{"0".into()};selected_events=format!("{secs} >= {at} ? ({val}) : ({selected_events})");}
            let selected=self.number(selected_events);dy=format!("({dy}) - 6 * {selected}");sc=format!("({sc}) * (1 + 0.025 * {selected})");
        }
        let(mut sx,mut sy)=(sc.clone(),sc);
        if let Some(beat)=item["hb"]["t"].as_f64(){
            let hold=item["hb"]["hold"].as_f64().unwrap();let u=self.number(format!("{secs} - {beat}"));
            let he=self.number(format!("{u} < 0 ? 0 : ({u} < 0.18 ? 1 - Math.pow(1 - {u} / 0.18, 3) : ({u} < {hold} ? 1 : Math.pow(1 - Math.min(1, Math.max(0, ({u} - {hold}) / 0.45)), 3)))"));
            let hw=self.number(format!("{u} < -{ant} ? 0 : ({u} < 0 ? 1 - Math.pow(1 - ({u} + {ant}) / {ant}, 3) : Math.max(0, 1 - {u} / 0.08))"));
            let hr=self.number(format!("{u} <= 0.04 ? 0 : Math.pow({}, -{decay} * ({u} - 0.04)) * Math.sin({} * ({u} - 0.04))",std::f64::consts::E,tau*freq));
            let active=format!("({he} > 0 || ({u} > -0.4 && {u} < 0))");
            let pop=self.number(format!("1 + {} * {hr} + 0.035 * {he}",0.07*amp));
            sx=format!("({sx}) * ({active} ? {pop} * (1 + {} * {hw}) : 1)",0.04*amp);
            sy=format!("({sy}) * ({active} ? {pop} * (1 - {} * {hw}) : 1)",0.06*amp);
        }
        for (key, target) in [("_composedDx", &mut dx), ("_composedDy", &mut dy), ("_composedRot", &mut rot)] {
            if let Some(value) = item[key].as_str() { *target = format!("({target}) + ({value})"); }
        }
        for (key, target) in [("_composedScaleX", &mut sx), ("_composedScaleY", &mut sy), ("_composedOpacity", &mut op)] {
            if let Some(value) = item[key].as_str() { *target = format!("({target}) * ({value})"); }
        }
        let dx=self.css_number(&dx,1);let dy=self.css_number(&dy,1);let sx=self.css_number(&sx,3);let sy=self.css_number(&sy,3);let rot=self.css_number(&rot,3);let op=self.css_number(&op,3);
        let s=self.scale;let cx=item["_motionOriginX"].as_f64().unwrap_or(if text&&item["align"]!="center"{x}else{x+w*0.5});let cy=item["_motionOriginY"].as_f64().unwrap_or(y+h*if text{0.5}else{0.6});
        let dx=self.number(format!("{dx} * {s}"));let dy=self.number(format!("{dy} * {s}"));
        let extra=if self.theme.name=="midnight"{
            let blur=format!("(1 - {expo}) * 14 + {ko} * 12");
            let blur=self.number(if text{format!("{before_exit} ? 0 : ({blur})")}else{blur});
            let blur=self.css_number(&format!("{blur} > 0.05 ? {blur} : 0"),1);
            let blur=self.number(format!("{blur} * {s}"));
            format!(" blur={{{blur}}}")
        }else if self.theme.name=="blueprint"{
            let right=if text{"0".to_owned()}else{format!("(1 - {expo}) * 100")};
            let left=format!("{ko} * 100");let enabled=self.number(format!("({left}) > 0.05 || ({right}) > 0.05 ? 1 : 0"));
            let right=self.css_number(&format!("{enabled} > 0 ? ({right}) : 0"),1);let left=self.css_number(&format!("{enabled} > 0 ? ({left}) : 0"),1);
            let outset=if h>0.0{-1200.0/h}else{0.0};
            format!(" clipEnabled={{{enabled}}} clipX={{{}}} clipY={{{}}} clipWidth={{{}}} clipHeight={{{}}} clipTop={{{outset}}} clipBottom={{{outset}}} clipRight={{{right}}} clipLeft={{{left}}}",x*s,y*s,w*s,h*s)
        }else{String::new()};
        let mut extra=extra;
        if let Some(ry)=item["_composedRotateY"].as_str(){
            let ry=self.css_number(ry,1);
            extra.push_str(&format!(" rotateY={{{ry}}} perspective={{{}}} perspectiveOriginX={{{}}} perspectiveOriginY={{{}}} backfaceVisibility=\"hidden\"",1400.0*s,cx*s,cy*s));
        }
        if let Some(layer)=item["_composedLayer"].as_str(){extra.push_str(&format!(" layer={{{layer}}}"));}
        self.body.push_str(&format!("<MotionBox id={} x={{{dx}}} y={{{dy}}} scaleX={{{sx}}} scaleY={{{sy}}} rotation={{{rot}}} opacity={{{op}}} pivotX={{{}}} pivotY={{{}}}{extra}>",q(id),cx*s,cy*s));
        if let Some(beat)=item["hb"]["t"].as_f64() {
            let start=((beat-0.45)*1000.0).next_up().max(0.0);
            let end=(beat+item["hb"]["hold"].as_f64().unwrap()+0.5)*1000.0;
            self.body.push_str(&format!("<Animate prop=\"layer\" from={{0}} to={{6}} start=\"{start}ms\" duration=\"0ms\" ease=\"linear\"/><Animate prop=\"layer\" from={{6}} to={{0}} start=\"{end}ms\" duration=\"0ms\" ease=\"linear\"/>"));
        }
    }
    pub fn layout_node(&mut self, kind:&str,id:&str,item:&Value,x:f64,y:f64,w:f64,h:f64,at:f64,until:Option<f64>) -> Result<(),String> {
        if kind=="image"{return self.source_image(id,item,x,y,w,h,at,until);}
        if matches!(kind,"pictTile"|"pictFocus"|"pictExc")&&item["icon"].as_str().is_some_and(|s|!s.is_empty()){let mut picture=item.clone();picture["base"]=item["icon"].clone();picture["t"]=json!(kind);if picture["tone"].is_null(){picture["tone"]=json!(if kind=="pictFocus"{"accent"}else if kind=="pictExc"{"warning"}else{"default"});}return self.picture_tile(id,&picture,x,y,w,h,at,until);}
        let mut definition=NODE_TYPES.get(kind).ok_or_else(||format!("unknown native node type {kind:?}"))?.clone();if definition["role"]=="card"&&item["glyph"].is_null(){if item["tone"]=="warning"{definition["sel"]=json!("exc");definition["exc"]=json!(1);}else if item["tone"]=="accent"{definition["sel"]=json!("accent");}}let d=&definition;
        let role=d["role"].as_str().unwrap_or("none");let theme=&SOURCE_THEMES[self.theme.name];
        let tone=item["tone"].as_str().unwrap_or("default");
        let focus=role=="focus";let pill=role=="pill";
        let mut bg=if focus{self.theme.focus}else if pill{theme["pill"].as_str().unwrap()}else if role=="band"{theme["band"].as_str().unwrap()}else if role=="none"{"transparent"}else{self.theme.surface};
        let mut ink=if focus{self.theme.focus_ink}else if pill{theme["pillInk"].as_str().unwrap()}else{self.theme.ink};
        if pill&&tone=="warning"{bg=theme["exceptionTint"].as_str().unwrap();ink=self.theme.warning;}else if pill&&tone=="accent"{bg=theme["accentTint"].as_str().unwrap();ink=self.theme.accent;}
        self.begin_theme_motion(&format!("{id}-motion"),item,x,y,w,h,at,until,pill,role!="band"&&kind!="callout");
        // TYPE.text already owns its fitted size and line box; label fitting is only for cards/pills.
        if d["text"].as_i64()==Some(1) {
            crate::typography::source_text_node(self,id,item,x,y,w,h,at,until)?;
            self.end_move();
            self.anchors.push((x+w/2.0,y+h/2.0));
            return Ok(());
        }
        let rad=if d["round"].as_i64()==Some(1){h/2.0}else if pill{d["rad"].as_f64().unwrap_or(h/2.0)}else{self.theme.radius*d["rad"].as_f64().unwrap_or(1.0)+if role=="band"{6.0}else{0.0}};
        let mut border=if d["exc"].as_i64()==Some(1)||d["sel"]=="exc"{format!("2px dashed {}",self.theme.warning)}else if d["sel"]=="accent"{format!("2.5px solid {}",self.theme.accent)}else if d["dash"].as_i64()==Some(1)||d["sel"]=="dash"||kind=="pictFrame"{format!("2px dashed {}",self.theme.line)}else if role=="card"{theme["cardBorder"].as_str().unwrap().to_owned()}else{"none".to_owned()};
        let mut shadow=if role=="card"{theme["shadow"].as_str().unwrap().to_owned()}else{"none".to_owned()};if item["depth"].as_bool()==Some(true)||item["depth"].as_f64().is_some_and(|v|v!=0.0){shadow=self.depth_shadow(d,tone,&shadow);}
        if (item["callout"].as_bool()==Some(true)||item["callout"].as_f64().is_some_and(|v|v!=0.0))&&pill{let oc=if tone=="warning"{self.theme.warning}else if tone=="accent"{self.theme.accent}else{self.theme.muted};bg=self.theme.surface;ink=if tone=="warning"{self.theme.warning}else if tone=="accent"{self.theme.accent}else{self.theme.ink};border=format!("2px solid {oc}");shadow=theme["shadow"].as_str().unwrap().to_owned();}
        let s=self.scale;
        let fill=if pill{self.hold_fill(item,bg)}else{q(bg)};
        let selected_attrs=if item["mode"].is_number(){let mut parts=border.splitn(3,' ');let bw=parts.next().unwrap_or("0").trim_end_matches("px").parse::<f64>().unwrap_or(0.0)*s;let _style=parts.next();let bc=parts.next().unwrap_or("transparent");format!(" stroke={} strokeWidth={{{bw}}}",q(bc))}else{format!(" border={}",q(&source_scale_px(&border,s)))};self.body.push_str(&format!("<Rect id={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} radius={{{}}} fill={}{selected_attrs} shadow={}>",q(id),x*s,y*s,w*s,h*s,rad*s,fill,q(&source_scale_px(&shadow,s))));self.selected_border(item,&border);self.body.push_str("</Rect>");
        self.hold_ring(&format!("{id}-hold-ring"),item,x,y,w,h,rad);
        let label=item["label"].as_str().unwrap_or("");let icon=item["icon"].as_str().unwrap_or("");
        let marker=item["marker"].as_str().is_some_and(|value|!value.is_empty());
        let mut size=if marker{item["ms"].as_f64().unwrap_or(0.0)}else if icon.is_empty(){0.0}else{d["icon"].as_f64().unwrap_or(0.0)};if !marker{if size<=1.0{size=(size*w).round();}else if w<170.0&&d["layout"]=="col"{size=(size*0.78).round();}}
        let max_padding=if role=="card"&&w<180.0{10.0}else{f64::MAX};let px=d["px"].as_f64().unwrap_or(0.0).min(max_padding);let pr=d["pr"].as_f64().unwrap_or(px).min(max_padding);let gap=d["gap"].as_f64().unwrap_or(14.0);let(fs,lines,room)=if d["fs"].is_number(){crate::typography::source_label_fit(self,item,kind,w,h)}else{(0.0,1,1)};
        // CSS border-box sizing reserves the used border width before flex layout.
        // Blink snaps these source borders (1.5/2.5px) down to whole CSS pixels.
        let inset=border.split_whitespace().next().and_then(|value|value.strip_suffix("px")).and_then(|value|value.parse::<f64>().ok()).unwrap_or(0.0).floor();
        let (x,y,w,h)=(x+inset,y+inset,w-2.0*inset,h-2.0*inset);
        let label_width=if d["layout"]=="row"{w-px-pr-if size>0.0{size+gap}else{0.0}}else{w-2.0*px};
        let clamp_lines=lines.max(room);
        let labelh=if fs>0.0&&!label.is_empty(){let lh=(fs*1.16).round();crate::typography::measure_label_style(label,self.theme.font,if pill{500}else if fs>=30.0{self.theme.weight as u16}else{600},if fs>=30.0{self.theme.tracking}else{-0.005},fs,if clamp_lines==1{1.0e9}else{label_width.max(1.0)},lh/fs).lines.len().min(clamp_lines) as f64*lh}else{0.0};
        let (mut tx,mut ty,mut tw,mut th)=(x+px,y+(h-labelh)/2.0,w-px-pr,labelh);let mut align=if pill&&d["left"].as_i64()==Some(1){"left"}else{"center"};
        match d["layout"].as_str().unwrap_or("") {
            "row"=>{let reserved=if size>0.0{size+if label.is_empty(){0.0}else{gap}}else{0.0};let center=d["center"].as_i64()==Some(1);let lw=if center{crate::typography::measure_style(label,self.theme.font,if pill{500}else if fs>=30.0{self.theme.weight as u16}else{600},if fs>=30.0{self.theme.tracking}else{-0.005},fs,1.0e9,1.16).width.min((w-px-pr-reserved).max(0.0))}else{w-px-pr-reserved};let left=if center{x+px+(w-px-pr-reserved-lw)/2.0}else{x+px};if size>0.0{self.layout_icon(&format!("{id}-icon"),item,icon,left,y+(h-size)/2.0,size,d,at)?;}tx=left+reserved;tw=lw;align="left";},
            "col"=>{let block=size+if size>0.0&&!label.is_empty(){14.0}else{0.0}+if label.is_empty(){0.0}else{labelh};let by=y+(h-block)/2.0;if size>0.0{self.layout_icon(&format!("{id}-icon"),item,icon,x+(w-size)/2.0,by,size,d,at)?;}ty=by+size+if size>0.0{14.0}else{0.0};th=labelh;},
            "slab"=>{if size>0.0{self.layout_icon(&format!("{id}-icon"),item,icon,x+px,y+px,size,d,at)?;}ty=y+h-px-labelh;th=labelh;align="left";},
            "center"=>{if size>0.0{self.layout_icon(&format!("{id}-icon"),item,icon,x+(w-size)/2.0,y+(h-size)/2.0,size,d,at)?;}},
            _=>{},
        }
        if !label.is_empty()&&d["fs"].is_number(){let mut text=item.clone();text["label"]=json!(label);text["fs"]=json!(fs);text["lock"]=json!({"fs":fs,"lines":lines,"room":room});text["align"]=json!(item["align"].as_str().unwrap_or(align));text["_labelInk"]=json!(ink);text["_labelWeight"]=json!(if pill{500}else if fs>=30.0{self.theme.weight}else{600});text["_labelTracking"]=json!(if fs>=30.0{self.theme.tracking}else{-0.005});if text["lh"].is_null(){text["lh"]=d["lh"].clone();}crate::typography::source_text_node(self,&format!("{id}-label"),&text,tx,ty,tw,th,-10000.0,None)?;}
        self.end_move();self.anchors.push((x+w/2.0,y+h/2.0));Ok(())
    }
    pub fn picture_tile(&mut self,id:&str,item:&Value,x:f64,y:f64,w:f64,h:f64,at:f64,until:Option<f64>) -> Result<(),String> {
        self.source_picture_tile(id,item,x,y,w,h,at,until)
    }
}

