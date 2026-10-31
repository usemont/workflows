// Source: engine/src/render/motion.js pictResolve, pictFx, pictShapes and stCss.
static PICTURE_CATALOGUE: std::sync::LazyLock<Value> = std::sync::LazyLock::new(||serde_json::from_str(include_str!("pictures.json")).expect("source picture catalogue"));
#[derive(Clone,Copy)]
struct PictureSpring { amp:f64,freq:f64,decay:f64,ant:f64 }
impl PictureSpring {
    fn for_style(style:&str)->Self {let(a,f,d,n)=match style{"focus"=>(0.7,1.4,3.6,0.26),"draft"=>(0.55,3.0,8.0,0.12),"pop"=>(1.5,2.5,4.2,0.18),_=>(1.0,2.1,5.0,0.20)};Self{amp:a,freq:f,decay:d,ant:n}}
}
#[derive(Clone)]
struct PictureState {base:String,tone:String,from:f64,to:Option<f64>,swap:bool}
#[derive(Clone)]
struct PictureBadge {name:String,from:f64,to:Option<f64>}
#[derive(Clone,Copy)]
struct PictureEvent {kind:u8,t:f64,dx:f64,dy:f64}
fn picture_rgba(s:&str)->[f64;4]{
    if s=="none"||s=="transparent"{return [0.0;4]}
    if let Some(h)=s.strip_prefix('#'){let h=if h.len()==3{h.chars().flat_map(|c|[c,c]).collect::<String>()}else{h.into()};let n=u32::from_str_radix(&h[..6],16).expect("source hex color");return [((n>>16)&255)as f64,((n>>8)&255)as f64,(n&255)as f64,if h.len()==8{u8::from_str_radix(&h[6..],16).unwrap()as f64/255.0}else{1.0}]}
    let inner=s.split_once('(').expect("source color").1.trim_end_matches(')');let v:Vec<f64>=inner.split(',').map(|x|x.trim().parse().expect("source rgba component")).collect();[v[0],v[1],v[2],v.get(3).copied().unwrap_or(1.0)]
}
fn picture_mix(a:&str,b:&str,k:f64)->String{let(mut a,mut b)=(picture_rgba(a),picture_rgba(b));if a[3]==0.0{a[..3].copy_from_slice(&b[..3]);}if b[3]==0.0{b[..3].copy_from_slice(&a[..3]);}format!("rgba({},{},{},{})",(a[0]+(b[0]-a[0])*k).round(),(a[1]+(b[1]-a[1])*k).round(),(a[2]+(b[2]-a[2])*k).round(),((a[3]+(b[3]-a[3])*k)*1000.0).round()/1000.0)}
fn picture_ink<'b>(th:&'b Value,tone:&str)->&'b str{th[match tone{"accent"=>"focusIcon","warning"=>"exception","muted"=>"muted",_=>"ink"}].as_str().expect("source picture ink")}
fn picture_face<'b>(th:&'b Value,tone:&str)->&'b str{th[if tone=="accent"{"focus"}else{"surface"}].as_str().unwrap()}
// Each row selects its authored predecessor, even if the preceding tween was interrupted.
fn picture_paint_tracks(body:&mut String,prop:&str,rows:&[Value],window:f64,color:impl Fn(&str)->String){
    for pair in rows.windows(2){let old=pair[0]["tone"].as_str().unwrap_or("default");let new=pair[1]["tone"].as_str().unwrap_or("default");let t=pair[1]["t"].as_f64().unwrap()*1000.0;let from=color(old);let to=color(new);body.push_str(&format!("<Animate prop={} from={{{}}} to={{{}}} start=\"{t}ms\" duration=\"{}ms\" ease=\"inOutQuart\"/>",q(prop),q(&from),q(&to),if old==new{0.0}else{window*1000.0}));}
}
fn picture_depth_shadow(th:&Value,focus:bool,tone:&str,shadow:&str)->String{
    let d=&th["depth"];let px=d["px"].as_f64().unwrap_or(0.0);if d["kind"]=="none"||px==0.0{return shadow.into();}
    let mut edge=if d["kind"]=="outline"{format!("0 {px}px 0 -1px {}, 0 {px}px 0 0.5px {}",th["bg"].as_str().unwrap(),th[match tone{"warning"=>"exception","accent"=>"accent",_=>"line"}].as_str().unwrap())}else{let face=th[if focus{"focus"}else{match tone{"warning"=>"exceptionTint","accent"=>"accentTint",_=>"surface"}}].as_str().unwrap();let target=if focus{"#000000"}else{th[match tone{"warning"=>"exception","accent"=>"accent",_=>"ink"}].as_str().unwrap()};format!("0 {px}px 0 0 {}",picture_mix(face,target,d["mix"].as_f64().unwrap()))};
    if let Some(rim)=d["rim"].as_str().filter(|s|!s.is_empty()){edge=format!("{rim}, {edge}");}
    if !shadow.is_empty()&&shadow!="none"{edge.push_str(", ");edge.push_str(shadow);}edge
}
impl<'a> Scene<'a>{
    fn picture_rebound(&mut self,u:&str,c:PictureSpring)->String{self.number(format!("{u} <= 0 ? 0 : Math.pow({}, -{} * {u}) * Math.sin({} * {u})",std::f64::consts::E,c.decay,std::f64::consts::TAU*c.freq))}
    fn picture_windup(&mut self,u:&str,c:PictureSpring)->String{self.number(format!("{u} < -{} ? 0 : ({u} < 0 ? 1 - Math.pow(1 - ({u} + {}) / {}, 3) : Math.max(0, 1 - {u} / 0.08))",c.ant,c.ant,c.ant))}
    fn picture_beat(&mut self,u:&str)->String{self.number(format!("{u} < 0 ? 0 : ({u} < 0.18 ? 1 - Math.pow(1 - {u} / 0.18, 3) : ({u} < 1 ? 1 : Math.pow(1 - Math.min(1, Math.max(0, ({u} - 1) / 0.45)), 3)))"))}
    fn picture_fx(&mut self,events:&[PictureEvent],c:PictureSpring,t:&str)->[String;7]{
        let(mut sx,mut sy,mut sc,mut dx,mut dy,mut rot,mut ring,mut dim)=("1".to_owned(),"1".to_owned(),"1".to_owned(),"0".to_owned(),"0".to_owned(),"0".to_owned(),"0".to_owned(),"0".to_owned());
        for e in events{let u=self.number(format!("{t} - {}",e.t));let active=self.number(format!("{u} < -0.5 || {u} > 2.2 ? 0 : 1"));let a=c.amp;
            let(mut xs,mut ys,mut cs,mut xt,mut yt,mut rt,mut rg,mut dm)=("1".to_owned(),"1".to_owned(),"1".to_owned(),"0".to_owned(),"0".to_owned(),"0".to_owned(),"0".to_owned(),"0".to_owned());
            match e.kind{
                0=>{let r=self.picture_rebound(&u,c);ys=format!("1 - {} * {r}",0.06*a);xs=format!("1 + {} * {r}",0.045*a);},
                1=>{let w=self.picture_windup(&u,c);let delayed=self.number(format!("{u} - 0.05"));let r=self.picture_rebound(&delayed,c);ys=format!("1 - {} * {w} + {} * {r}",0.1*a,0.05*a);xs=format!("1 + {} * {w} - {} * {r}",0.06*a,0.03*a);yt=format!("{} * {w}",4.0*a);cs=format!("1 + {} * {r}",0.06*a);},
                2=>{let r=self.picture_rebound(&u,c);cs=format!("1 + {} * {r}",0.03*a);rt=format!("{} * {r}",1.6*a);},
                3=>{let r=self.picture_rebound(&u,c);xt=format!("{} * {r}",e.dx*7.0*a);yt=format!("{} * {r}",e.dy*7.0*a);xs=format!("1 + {} * {r}",0.03*a*e.dx.abs());ys=format!("1 + {} * {r}",0.03*a*e.dy.abs());},
                4=>{let w=self.picture_windup(&u,c);let delayed=self.number(format!("{u} - 0.04"));let r=self.picture_rebound(&delayed,c);let beat=self.picture_beat(&u);ys=format!("1 - {} * {w}",0.06*a);xs=format!("1 + {} * {w}",0.04*a);cs=format!("1 + {} * {r} + 0.04 * {beat}",0.09*a);rg=beat;},
                5=>dm=self.picture_beat(&u),6=>{let w=self.picture_windup(&u,c);cs=format!("1 + {} * {w}",0.05*a);yt=format!("-{} * {w}",5.0*a);},_=>unreachable!()
            }
            // Store each accumulator once: expressions stay shallow without duplicating event histories.
            if xs!="1"{sx=self.number(format!("{sx} * ({active} > 0 ? ({xs}) : 1)"));}if ys!="1"{sy=self.number(format!("{sy} * ({active} > 0 ? ({ys}) : 1)"));}if cs!="1"{sc=self.number(format!("{sc} * ({active} > 0 ? ({cs}) : 1)"));}
            if xt!="0"{dx=self.number(format!("{dx} + ({active} > 0 ? ({xt}) : 0)"));}if yt!="0"{dy=self.number(format!("{dy} + ({active} > 0 ? ({yt}) : 0)"));}if rt!="0"{rot=self.number(format!("{rot} + ({active} > 0 ? ({rt}) : 0)"));}if rg!="0"{ring=self.number(format!("Math.max({ring}, {active} > 0 ? {rg} : 0)"));}if dm!="0"{dim=self.number(format!("Math.max({dim}, {active} > 0 ? {dm} : 0)"));}
        }
        let sx=self.number(format!("{sc} * {sx}"));let sy=self.number(format!("{sc} * {sy}"));let dim=self.number(format!("{ring} > 0 ? 0 : {dim}"));[dx,dy,sx,sy,rot,ring,dim]
    }
    fn source_picture_tile(&mut self,id:&str,item:&Value,x:f64,y:f64,w:f64,h:f64,at:f64,until:Option<f64>)->Result<(),String>{
        let th=SOURCE_THEMES[self.theme.name].clone();let style=th["motion"]["in"].as_str().unwrap();let c=PictureSpring::for_style(style);let window=0.5*th["motion"]["tempo"].as_f64().expect("source theme tempo");let a=at/1000.0;let s=self.scale;
        let rows:Vec<Value>=item["ptl"].as_array().cloned().unwrap_or_else(||vec![json!({"base":item["base"],"mod":item["mod"],"tone":item["tone"]})]);
        if rows.is_empty(){return Err("source picture timeline must contain an initial state".into());}
        let mut states:Vec<PictureState>=Vec::new();let mut badges:Vec<PictureBadge>=Vec::new();let mut events=vec![PictureEvent{kind:0,t:a+th["motion"]["dur"].as_f64().unwrap()*if style=="pop"{0.55}else{0.7},dx:0.0,dy:0.0}];
        for(i,row)in rows.iter().enumerate(){let t=if i==0{a}else{row["t"].as_f64().ok_or("picture state requires source time")?};let base=row["base"].as_str().or_else(||item["base"].as_str()).ok_or("picture requires source base")?;let tone=row["tone"].as_str().unwrap_or("default");
            if states.last().is_none_or(|g|g.base!=base){if let Some(g)=states.last_mut(){g.to=Some(t);}states.push(PictureState{base:base.into(),tone:tone.into(),from:t,swap:i>0,to:None});}else{states.last_mut().unwrap().tone=tone.into();}
            if i>0&&(rows[i-1]["base"]!=row["base"]||rows[i-1]["tone"]!=row["tone"]){events.push(PictureEvent{kind:1,t,dx:0.0,dy:0.0});}
            let modifier=row["mod"].as_str().unwrap_or("");let last=badges.last().filter(|b|b.to.is_none()).map(|b|b.name.as_str()).unwrap_or("");let replacing=!last.is_empty();if modifier!=last{if let Some(b)=badges.last_mut(){if b.to.is_none(){b.to=Some(t);}}if !modifier.is_empty(){let from=t+if i==0{th["motion"]["dur"].as_f64().unwrap()*0.6}else if replacing{0.18}else{0.0};badges.push(PictureBadge{name:modifier.into(),from,to:None});}}
        }
        for b in &badges{events.push(PictureEvent{kind:2,t:b.from+0.06,dx:0.0,dy:0.0});}
        if let Some(hits)=item["hits"].as_array(){for hit in hits{events.push(PictureEvent{kind:3,t:hit["t"].as_f64().ok_or("picture hit requires time")?,dx:hit["dx"].as_f64().unwrap_or(0.0),dy:hit["dy"].as_f64().unwrap_or(0.0)});}}
        if let Some(gone)=item["gone"].as_f64(){events.push(PictureEvent{kind:6,t:gone,dx:0.0,dy:0.0});}
        if let Some(beats)=item["pbeats"].as_array(){for beat in beats{events.push(PictureEvent{kind:if beat["mine"].as_bool().unwrap_or(false){4}else{5},t:beat["t"].as_f64().ok_or("picture beat requires time")?,dx:0.0,dy:0.0});}}
        if item["ptl"].is_null(){events.clear();}
        let t=self.number("computed.sourceClock / 1000");let [dx,dy,sx,sy,rot,ring,dim]=self.picture_fx(&events,c,&t);
        let mut tone_dims=Vec::new();for pair in rows.windows(2){let old=pair[0]["tone"].as_str().unwrap_or("default");let new=pair[1]["tone"].as_str().unwrap_or("default");let from=if old=="muted"{1.0}else{0.0};let to=if new=="muted"{1.0}else{0.0};let k=self.number(format!("Math.min(1, Math.max(0, ({t} - {}) / {window}))",pair[1]["t"].as_f64().unwrap()));let mix=self.number(format!("{k} < 0.5 ? 8 * Math.pow({k}, 4) : 1 - Math.pow(-2 * {k} + 2, 4) / 2"));tone_dims.push((pair[1]["t"].as_f64().unwrap(),format!("{from} + ({to} - {from}) * {mix}")));}
        let initial_dim=if rows[0]["tone"]=="muted"{"1"}else{"0"};let tone_dim=if item["ptl"].is_null(){if item["dim"].as_f64().unwrap_or(0.0)!=0.0{"1".into()}else{"0".into()}}else{self.number(source_event_expression(&t,&tone_dims,initial_dim,">="))};
        let mut motion=item.clone();motion["tone"]=json!("default");motion["dim"]=json!(0);for(key,value)in[("_composedDx",dx),("_composedDy",dy),("_composedScaleX",sx),("_composedScaleY",sy),("_composedRot",rot),("_composedOpacity",format!("(1 - 0.55 * {tone_dim}) * (1 - 0.5 * {dim})"))]{motion[key]=json!(value);}
        let until=match(until,item["gone"].as_f64().map(|v|v*1000.0)){(Some(a),Some(b))=>Some(a.min(b)),(a,b)=>a.or(b)};
        self.begin_theme_motion(&format!("{id}-picture-motion"),&motion,x,y,w,h,at,until,false,true);
        let warning=self.theme.warning;let tone=rows[0]["tone"].as_str().unwrap_or("default");let border=|tone:&str|if tone=="accent"{"none".to_owned()}else if tone=="warning"{format!("2px dashed {warning}")}else{th["cardBorder"].as_str().unwrap().into()};let shadow=|tone:&str|{let plain=if tone=="accent"{"none"}else{th["shadow"].as_str().unwrap()};if item["depth"].as_bool()==Some(true)||item["depth"].as_i64().is_some_and(|v|v!=0){picture_depth_shadow(&th,tone=="accent",item["tone"].as_str().unwrap_or("default"),plain)}else{plain.into()}};
        let radius_for=|tone:&str|NODE_TYPES[if tone=="accent"{"pictFocus"}else if tone=="warning"{"pictExc"}else{"pictTile"}]["rad"].as_f64().unwrap_or(1.0)*th["radius"].as_f64().unwrap();let radius_events:Vec<_>=rows.iter().skip(1).map(|r|(r["t"].as_f64().unwrap(),radius_for(r["tone"].as_str().unwrap_or("default")).to_string())).collect();let radius=self.number(source_event_expression(&t,&radius_events,&radius_for(tone).to_string(),">="));let radius=self.css_number(&radius,1);
        // Outer spread shadows exclude the card interior, including translucent card backgrounds.
        let spread=self.css_number(&format!("2 + 4 * {ring}"),1);let spread=self.number(format!("{spread} * {s}"));let ring_op=self.css_number(&format!("0.55 * {ring}"),3);let rx=self.number(format!("{} - {spread}",x*s));let ry=self.number(format!("{} - {spread}",y*s));let rw=self.number(format!("{} + 2 * {spread}",w*s));let rh=self.number(format!("{} + 2 * {spread}",h*s));let rr=self.number(format!("{radius} * {s} + {spread}"));
        let radius=self.number(format!("{radius} * {s}"));self.body.push_str(&format!("<Rect id={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} radius={{{radius}}} fill={} border={} shadow={}>",q(id),x*s,y*s,w*s,h*s,q(picture_face(&th,tone)),q(&source_scale_px(&border(tone),s)),q(&source_scale_px(&shadow(tone),s))));
        picture_paint_tracks(&mut self.body,"fill",&rows,window,|tone|picture_face(&th,tone).into());picture_paint_tracks(&mut self.body,"border",&rows,window,|tone|source_scale_px(&border(tone),s));
        for pair in rows.windows(2){let old=pair[0]["tone"].as_str().unwrap_or("default");let new=pair[1]["tone"].as_str().unwrap_or("default");let t=pair[1]["t"].as_f64().unwrap()*1000.0;let from=if old==new{shadow(new)}else if old=="accent"{"none".into()}else{th["shadow"].as_str().unwrap().into()};self.body.push_str(&format!("<Animate prop=\"shadow\" from={{{}}} to={{{}}} start=\"{t}ms\" duration=\"{}ms\" ease=\"inOutQuart\"/>",q(&source_scale_px(&from,s)),q(&source_scale_px(&shadow(new),s)),if old==new{0.0}else{window*1000.0}));}
        self.body.push_str("</Rect>");self.anchors.push((x+w/2.0,y+h/2.0));
        self.body.push_str(&format!("<Rect id={} x={{{rx}}} y={{{ry}}} width={{{rw}}} height={{{rh}}} radius={{{rr}}} fill=\"none\" stroke={} strokeWidth={{{spread}}} opacity={{{ring_op}}}/>",q(&format!("{id}-beat-ring-shape")),q(self.theme.accent)));
        // Absolute SVGs fill the CSS padding box, not the card's border box.
        let border_width=|tone:&str|border(tone).split("px").next().unwrap().parse::<f64>().unwrap_or(0.0);
        let mut inset=border_width(tone).to_string();
        for pair in rows.windows(2){let old=pair[0]["tone"].as_str().unwrap_or("default");let new=pair[1]["tone"].as_str().unwrap_or("default");let at=pair[1]["t"].as_f64().unwrap();let k=self.number(format!("Math.min(1, Math.max(0, ({t} - {at}) / {window}))"));let mix=self.number(format!("{k} < 0.5 ? 8 * Math.pow({k}, 4) : 1 - Math.pow(-2 * {k} + 2, 4) / 2"));let width=self.css_number(&format!("{} + ({} - {}) * {mix}",border_width(old),border_width(new),border_width(old)),1);inset=self.number(format!("{t} >= {at} ? {width} : ({inset})"));}
        // CSS snaps nonzero border widths down to whole layout pixels, with a one-pixel minimum.
        let inset=self.number(format!("{inset} <= 0 ? 0 : Math.max(1, Math.floor({inset}))"));let offset=self.number(format!("{inset} * {s}"));let content_sx=self.number(format!("({w} - 2 * {inset}) / {w}"));let content_sy=self.number(format!("({h} - 2 * {inset}) / {h}"));
        self.body.push_str(&format!("<MotionBox id={} x={{{offset}}} y={{{offset}}} scaleX={{{content_sx}}} scaleY={{{content_sy}}} pivotX={{{}}} pivotY={{{}}}>",q(&format!("{id}-padding-box")),x*s,y*s));
        // nodeFrame's brand branch precedes pictShapes and uses the original glyph, not R's current base.
        let brand_src=item["brand"].as_str().and_then(|name|self.brands.and_then(|brands|brands.get(&crate::grammar::brand_key(name))));let brand_base=item["brandBase"].as_str().unwrap_or("");let glyph=item["glyph"].as_str().or_else(||item["base"].as_str()).unwrap_or("");
        if glyph==brand_base{if let Some(src)=brand_src.as_deref(){self.body.push_str(&format!("<Image id={} src={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} fit=\"contain\"/>",q(&format!("{id}-logo")),q(src),(x+w*0.2)*s,(y+h*0.2)*s,w*0.6*s,h*0.6*s));self.end_move();self.end_move();return Ok(());}}
        for(gidx,g)in states.iter().enumerate(){
            let shapes=PICTURE_CATALOGUE["BASES"].get(&g.base).and_then(Value::as_array).ok_or_else(||format!("unknown source picture base {:?}",g.base))?;
            let kin=if g.swap{self.number(format!("Math.min(1, Math.max(0, ({t} - {} - {}) / {}))",g.from,0.35*window,0.65*window))}else{"1".into()};let kout=if let Some(to)=g.to{self.number(format!("Math.min(1, Math.max(0, ({t} - {to} + 0.12) / {}))",0.6*window+0.12))}else{"0".into()};let ei=self.number(format!("1 - Math.pow(1 - {kin}, 3)"));let eo=self.number(format!("Math.pow({kout}, 3)"));
            let over=self.number(format!("{kin} < 1 ? 1 + {} * Math.sin({} * {kin}) * {kin} : 1",0.16*c.amp,std::f64::consts::PI));let pull=self.number(format!("{kout} > 0 ? 1 - {} * Math.sin({} * Math.min(1, {kout} * 1.6)) : 1",0.12*c.amp,std::f64::consts::PI));
            let(mut sx,mut sy,mut dy,mut rot,mut extra)=("1".to_owned(),"1".to_owned(),"0".to_owned(),"0".to_owned(),String::new());let mut op=format!("Math.min(1, Math.max(0, {kin} * 2)) * (1 - {eo})");
            if g.swap||g.to.is_some(){sx=self.number(format!("{over} * {pull}"));sy=sx.clone();match style{
                "lift"=>{let pct=self.css_number(&format!("(1 - {ei}) * 30 - {eo} * 30"),1);dy=self.number(format!("{pct} * {}",h*s/100.0));sy=self.number(format!("{sx} * (1 + 0.08 * (1 - {ei}) * ({kin} > 0 ? 1 : 0))"));},
                "focus"=>{sx=self.number(format!("(0.86 + 0.14 * {ei}) * (1 + 0.12 * {eo}) * {over} * {pull}"));sy=sx.clone();let blur=self.number(format!("(1 - {ei}) * 6 + {eo} * 6"));let blur=self.css_number(&format!("{blur} > 0.05 ? {blur} : 0"),1);let blur=self.number(format!("{blur} * {s}"));extra=format!(" blur={{{blur}}}");},
                "draft"=>{op=format!("{kin} > 0 && {kout} < 1 ? 1 : 0");let right=self.css_number(&format!("(1 - {ei}) * 100"),1);let left=self.css_number(&format!("{eo} * 100"),1);extra=format!(" clipRight={{{right}}} clipLeft={{{left}}} clipTop={{-10}} clipBottom={{-10}} clipX={{0}} clipY={{0}} clipWidth={{{}}} clipHeight={{{}}} clipEnabled={{1}}",w*s,h*s);},
                _=>{let back=self.number(format!("1 + 2.7 * Math.pow({kin} - 1, 3) + 1.7 * Math.pow({kin} - 1, 2)"));sx=self.number(format!("Math.max(0, {back}) * (1 - {eo}) * {over} * {pull}"));sy=sx.clone();rot=self.css_number(&format!("(1 - {ei}) * -24 + {eo} * 18"),1);}
            }}else{op="1".into();}
            let op=self.css_number(&format!("{kin} <= 0 || {kout} >= 1 ? 0 : ({op})"),3);let sx=self.css_number(&sx,3);let sy=self.css_number(&sy,3);
            let gy=self.number(format!("{} + ({dy})",y*s));
            self.body.push_str(&format!("<MotionBox id={} x={{{}}} y={{{gy}}} scaleX={{{sx}}} scaleY={{{sy}}} rotation={{{rot}}} opacity={{{op}}} pivotX={{{}}} pivotY={{{}}} width={{{}}} height={{{}}}{extra}>",q(&format!("{id}-glyph-{gidx}-swap")),x*s,w*s/2.0,h*s/2.0,w*s,h*s));
            for(i,p)in shapes.iter().enumerate(){let mode=p[1].as_str().ok_or("source picture mode missing")?;let ink=if g.to.is_some(){picture_ink(&th,&g.tone)}else if item["ptl"].is_null(){picture_ink(&th,"default")}else{picture_ink(&th,tone)};let fill=if mode=="f"{ink}else if mode=="b"{picture_face(&th,tone)}else{"none"};let stroke=if mode=="f"{"none"}else{ink};let lucide=if mode=="L"{1.000008}else{1.0};
                self.body.push_str(&format!("<Path id={} d={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} viewBox={} fill={} stroke={} strokeWidth={{{}}} strokeLinecap=\"round\" strokeLinejoin=\"round\">",q(&format!("{id}-glyph-{gidx}-{i}")),q(p[0].as_str().ok_or("source picture path missing")?),w*0.17*s,h*0.17*s,w*0.66*s*lucide,h*0.66*s*lucide,q(if mode=="L"{"0 0 24 24"}else{"0 0 100 100"}),q(fill),q(stroke),if mode=="L"{1.8}else{7.5}));
                if g.to.is_none()&&!item["ptl"].is_null(){picture_paint_tracks(&mut self.body,if mode=="f"{"fill"}else{"stroke"},&rows,window,|tone|picture_ink(&th,tone).into());}if mode=="b"{picture_paint_tracks(&mut self.body,"fill",&rows,window,|tone|picture_face(&th,tone).into());}self.body.push_str("</Path>");
            }self.end_move();
        }
        for(i,b)in badges.iter().enumerate(){let badge=PICTURE_CATALOGUE["MODS"].get(&b.name).ok_or_else(||format!("unknown source picture modifier {:?}",b.name))?;let role=|v:&Value|->String{let n=v.as_str().unwrap_or("");if n=="onAccent"{self.theme.surface.into()}else{th[n].as_str().unwrap_or(n).into()}};let bg=role(&badge["bg"]);let ink=role(&badge["ink"]);let ring=if badge["ring"]==""{self.theme.bg.into()}else{role(&badge["ring"])};
            let ub=self.number(format!("{t} - {}",b.from));let kin=self.number(format!("Math.min(1, Math.max(0, {ub} / 0.45))"));let kout=if let Some(to)=b.to{self.number(format!("Math.min(1, Math.max(0, ({t} - {to}) / 0.25))"))}else{"0".into()};let pre=if let Some(to)=b.to{let u=self.number(format!("{t} - {to}"));self.picture_windup(&u,c)}else{"0".into()};
            let enter=self.number(format!("{ub} <= 0 ? 0 : 1 - Math.pow({}, -{} * {ub}) * Math.cos({} * {ub})",std::f64::consts::E,c.decay*1.15,std::f64::consts::TAU*c.freq*0.8));let sc=self.css_number(&format!("Math.max(0, {enter}) * (1 + 0.18 * {pre}) * (1 - Math.pow({kout}, 3))"),3);let rot=self.css_number(&format!("{ub} > 0 ? {} * Math.pow({}, -{} * {ub}) * Math.sin({} * {ub}) : 0",14.0*c.amp,std::f64::consts::E,c.decay,std::f64::consts::TAU*c.freq),1);let op=self.css_number(&format!("{kin} <= 0 || {kout} >= 1 ? 0 : Math.min(1, Math.max(0, {kin} * 3)) * (1 - {kout})"),3);
            self.body.push_str(&format!("<MotionBox id={} scaleX={{{sc}}} scaleY={{{sc}}} rotation={{{rot}}} opacity={{{op}}} pivotX={{{}}} pivotY={{{}}}>",q(&format!("{id}-badge-{i}-spring")),(x+w*0.9)*s,(y+h*0.1)*s));
            self.svg_path(&format!("{id}-badge-{i}-disc"),"M73 10A17 17 0 1 0 107 10A17 17 0 1 0 73 10Z",x,y,w,h,"0 0 100 100",&bg,&ring,if badge["ring"]==""{3.5}else{3.0},-10000.0,None);
            for(j,p)in badge["g"].as_array().ok_or("source badge paths missing")?.iter().enumerate(){let fill=p[1]=="f";self.svg_path(&format!("{id}-badge-{i}-{j}"),p[0].as_str().ok_or("source badge path missing")?,x+w*79.4/100.0,y-h*0.6/100.0,w*21.24/100.0,h*21.24/100.0,"0 0 24 24",if fill{&ink}else{"none"},if fill{"none"}else{&ink},2.6,-10000.0,None);}self.end_move();
        }self.end_move();self.end_move();Ok(())
    }
}
