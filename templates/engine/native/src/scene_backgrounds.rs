impl<'a> Scene<'a>{
    fn background_clock(&mut self)->(String,String){
        let t=self.number(format!("{} + computed.sourceClock / 1000",self.start));
        let initial=self.background_camera.first().map(|p|p.1).unwrap_or(0.0).to_string();
        let mut events=Vec::new();
        for pair in self.background_camera.windows(2){let(a,b)=(pair[0],pair[1]);events.push((a.0,format!("{} + ({}) * ((computed.sourceClock - ({})) / ({}))",a.1,b.1-a.1,a.0,b.0-a.0)));}
        if let Some(last)=self.background_camera.last(){events.push((last.0,last.1.to_string()));}
        // These are the transition's authored camera keys, not resampled background frames.
        let camera=self.number(source_event_expression("computed.sourceClock",&events,&initial,">"));
        let px=self.number(format!("-{camera} * 0.1"));
        (t,px)
    }
    fn background_beat(&mut self,part:&str)->String{
        let(_,pulse,down)=self.motion_music();let bt=&SOURCE_THEMES[self.theme.name]["beat"];
        self.number(format!("{} * {pulse} * ({down} > 0 ? 1 : {})",bt[part].as_f64().unwrap_or(0.0),bt["weak"].as_f64().unwrap()))
    }
    fn background_glow(&mut self,id:&str,color:&str,size:f64,stop:f64,x:&str,y:&str,beat:&str,reaction:f64){
        let x=self.css_number(&format!("({x}) - {}",size/2.0),1);
        let y=self.css_number(&format!("({y}) - {}",size/2.0),1);
        let sc=if reaction==0.0{"1".into()}else{self.css_number(&format!("1 + {reaction} * {beat}"),3)};
        let s=self.scale;let x=self.number(format!("{x} * {s}"));let y=self.number(format!("{y} * {s}"));
        self.body.push_str(&format!("<MotionBox id={} x={{{x}}} y={{{y}}} scaleX={{{sc}}} scaleY={{{sc}}} pivotX={{{}}} pivotY={{{}}}><Rect id={} x={{0}} y={{0}} width={{{}}} height={{{}}} fill={}/>",q(id),size/2.0*s,size/2.0*s,q(&format!("{id}-fill")),size*s,size*s,q(&format!("radial-gradient(circle, {color} 0%, transparent {stop}%)"))));self.end_move();
    }
    fn background_pattern(&mut self,id:&str,image:&str,size:f64,vx:f64,vy:f64,parallax:f64,t:&str,px:&str,beat:&str){
        let s=self.scale;let x=self.css_number(&format!("{px} * {parallax} + {t} * {vx}"),1);let y=self.css_number(&format!("{t} * {vy}"),1);
        let x=self.number(format!("{x} * {s}"));let y=self.number(format!("{y} * {s}"));
        let dot_attrs=if id=="bg-dots"{
            let(radius,feather)=if self.theme.name=="daylight"{
                let dr=self.number(format!("1.8 + 0.7 * {beat}"));let radius=self.css_number(&dr,3);let outer=self.css_number(&format!("{dr} + 0.6"),3);
                let feather=self.number(format!("({outer} - {radius}) * {s}"));let radius=self.number(format!("{radius} * {s}"));(radius,feather)
            }else{((1.2*s).to_string(),(0.6*s).to_string())};
            format!(" backgroundDotRadius={{{radius}}} backgroundDotFeather={{{feather}}}")
        }else{String::new()};
        self.body.push_str(&format!("<Rect id={} x={{0}} y={{0}} width={{{}}} height={{{}}} fill=\"transparent\" backgroundImage={} backgroundSize={} backgroundPositionX={{{x}}} backgroundPositionY={{{y}}}{dot_attrs}/>",q(id),self.width,self.height,q(image),q(&format!("{}px {}px",size*s,size*s))));
    }
    fn theme_background(&mut self){
        self.body.push_str(&format!("<Rect id=\"scene-background\" x={{0}} y={{0}} width={{{}}} height={{{}}} fill={}/>",self.width,self.height,q(self.theme.bg)));let th=SOURCE_THEMES[self.theme.name].clone();let cx=self.w/2.0;let h=self.h;let s=self.scale;
        let(t,px)=self.background_clock();let beat=self.background_beat("bg");
        match th["bgKind"].as_str().unwrap(){
            "dots"=>{
                self.background_pattern("bg-dots",&format!("radial-gradient(circle, {} {}px, transparent {}px)",th["bgDot"].as_str().unwrap(),1.8*s,2.4*s),36.0,5.0,3.0,1.0,&t,&px,&beat);
                self.background_glow("bg-glow",th["bgGlow"].as_str().unwrap(),1120.0,62.0,&format!("{cx} + 420 * Math.sin({t} * 0.11)"),&format!("{} + 170 * Math.sin({t} * 0.17 + 1)",h/2.0),&beat,0.08);
                self.background_glow("bg-glow2",th["bgGlow2"].as_str().unwrap(),960.0,60.0,&format!("{cx} - 380 * Math.sin({t} * 0.09 + 2)"),&format!("{} - 160 * Math.cos({t} * 0.13)",h/2.0),&beat,0.0);
            },
            "aurora"=>{
                self.background_pattern("bg-dots",&format!("radial-gradient(circle, {} {}px, transparent {}px)",th["bgDot"].as_str().unwrap(),1.2*s,1.8*s),64.0,0.0,0.0,0.6,&t,&px,&beat);
                for i in 0..3{let fi=i as f64;let bx=self.number(format!("260 + {} + 160 * Math.sin({t} * {} + {}) + {px} * 1.6 + 700",fi*520.0,0.07+0.03*fi,fi*2.0));let x=format!("(({bx} % 2680) + 2680) % 2680 - 700");let y=format!("{} + 120 * Math.sin({t} * {} + {fi})",220.0+fi*150.0,0.05+0.02*fi);self.background_glow(&format!("bg-aurora-{i}"),th["aurora"][i].as_str().unwrap(),1200.0,58.0,&x,&y,&beat,0.09-fi*0.025);}
            },
            "grid"=>{
                for(key,size)in[("bgMinor",40.0),("bgMajor",200.0),("bgBeat",200.0)]{
                    let c=th[key].as_str().unwrap();let p=s*if key=="bgBeat"{1.5}else{1.0};
                    if key=="bgBeat"{let opacity=self.css_number(&format!("{beat} > 0.01 ? Math.min(1, {beat}) : 0"),3);self.body.push_str(&format!("<MotionBox id=\"bg-beat-visibility\" opacity={{{opacity}}}>"));}
                    self.background_pattern(key,&format!("linear-gradient({c} {p}px, transparent {p}px), linear-gradient(90deg, {c} {p}px, transparent {p}px)"),size,6.0,4.0,1.0,&t,&px,&beat);
                    if key=="bgBeat"{self.end_move();}
                }
                self.background_glow("bg-grid-glow",th["bgGlow"].as_str().unwrap(),1200.0,60.0,&format!("{cx} + 460 * Math.sin({t} * 0.1)"),&format!("{} + 200 * Math.sin({t} * 0.14 + 0.5)",h/2.0),&beat,0.0);
            },
            "waves"=>{
                self.background_glow("bg-wave-glow",th["bgGlow"].as_str().unwrap(),1120.0,62.0,&format!("{cx} + 400 * Math.sin({t} * 0.12)"),&format!("{} + 150 * Math.cos({t} * 0.15)",h/2.0),&beat,0.0);
                self.background_waves(th["bgLine"].as_str().unwrap(),&t,&px,&beat);
            },
            _=>unreachable!()
        }
    }
    fn background_waves(&mut self,color:&str,t:&str,px:&str,beat:&str){
        let columns=(-40..=1320).step_by(40).map(|x|(x,self.number(format!("{x} - {px}")))).collect::<Vec<_>>();
        let mut rows=Vec::new();
        for li in 0..9{
            let mut points=Vec::new();
            for(x,wx)in &columns{
                let y=self.css_number(&format!("{} + (16 * Math.sin({wx} * 0.0055 + {t} * 0.45 + {}) + 9 * Math.sin({wx} * 0.013 - {t} * 0.32 + {})) * (1 + 0.45 * {beat})",40+li*86,li as f64*0.8,li as f64*1.7),1);
                points.push(format!("{} + {y}",q(&format!("{}{} ",if *x==-40{"M"}else{" L"},x))));
            }
            let name=format!("motion{}",self.counter);self.counter+=1;
            self.computeds.push(format!("{name}:computed.string(() => {})",points.join(" + ")));
            rows.push(format!("computed.{name}"));
        }
        let name=format!("motion{}",self.counter);self.counter+=1;
        self.computeds.push(format!("{name}:computed.string(() => {})",rows.join(" + \" \" + ")));
        let s=self.scale;
        self.body.push_str(&format!("<Path id=\"bg-waves\" d={{computed.{name}}} width={{{}}} height={{{}}} viewBox={} fill=\"none\" stroke={} strokeWidth={{1}}/>",self.w*s,self.h*s,q(&format!("0 0 {} {}",self.w,self.h)),q(color)));
    }
    fn apply_theme_camera(&mut self){
        let body=std::mem::take(&mut self.body);let cam=&SOURCE_THEMES[self.theme.name]["motion"]["camera"];let amount=cam["amt"].as_f64().unwrap();let pan=cam["kind"]=="pan";
        let secs=self.number("computed.sourceClock / 1000");
        let u=self.number(format!("Math.min(1, Math.max(0, ({secs} + 0.6) / {}))",self.duration/1000.0+2.5));
        let p=self.number(format!("-(Math.cos({} * {u}) - 1) / 2",std::f64::consts::PI));
        let a=self.number(format!("Math.min(1, Math.max(0, {secs} / 0.8))"));
        let beat=self.background_beat("camera");let(_,_,down)=self.motion_music();
        let envelope=self.number(format!("{a} * {a} * (3 - 2 * {a})"));
        let kick=self.number(format!("{down} > 0 ? {beat} * {envelope} : 0"));
        let sc=self.css_number(&if pan{format!("1 + {kick}")}else{format!("1 + {amount} * {p} + {kick}")},4);
        let x=if pan{let x=self.css_number(&format!("-{amount} * {p}"),1);self.number(format!("{x} * {}",self.scale))}else{"0".into()};
        self.body.push_str(&format!("<MotionBox id=\"theme-camera\" x={{{x}}} scaleX={{{sc}}} scaleY={{{sc}}} pivotX={{{}}} pivotY={{{}}}>",self.w/2.0*self.scale,if self.tall{630.0}else{368.0}*self.scale));self.body.push_str(&body);self.end_move();
    }
}
