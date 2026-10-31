fn route_inout(u:f64)->f64{if u<0.5{8.0*u.powi(4)}else{1.0-(-2.0*u+2.0).powi(4)/2.0}}
impl<'a> Scene<'a>{
    pub fn routed_edge(&mut self,id:&str,points:&[(f64,f64)],kind:&str,at:f64,until:Option<f64>,flashes:&[f64],mode:Option<usize>){
        if kind=="none"||points.len()<2{return;}
        if kind=="back"{let reversed=points.iter().copied().rev().collect::<Vec<_>>();self.routed_edge(id,&reversed,"flow",at,until,flashes,mode);return;}
        let k=match kind{"arrow"=>"flow","dashed"=>"branch","line"=>"plain",v=>v};
        let mut lengths=vec![0.0];for p in points.windows(2){lengths.push(lengths.last().unwrap()+(p[1].0-p[0].0).hypot(p[1].1-p[0].1));}let total=*lengths.last().unwrap();if total<=0.0{return;}
        let token=&SOURCE_THEMES[self.theme.name];let c=&token["connector"];let base=c["width"].as_f64().unwrap();let arrow=c["arrow"].as_f64().unwrap();let draw=token["motion"]["draw"].as_f64().unwrap()*1000.0;
        let strong=matches!(k,"flow"|"select");let blocked=k=="blocked";let leader=k=="leader";let has_head=matches!(k,"flow"|"exception"|"branch");
        let width=if leader{(base*0.7).max(1.6)}else{base+if strong||k=="affinity"{0.5}else{0.0}};
        let color=if matches!(k,"exception"|"blocked"){self.theme.warning}else if strong{self.theme.accent}else if leader{self.theme.muted}else{self.theme.line};
        let secs=self.number("computed.sourceClock / 1000");
        let u=self.number(format!("Math.min(1, Math.max(0, (computed.sourceClock - {at}) / {draw}))"));
        let f=self.number(format!("{u} < 0.5 ? 8 * Math.pow({u}, 4) : 1 - Math.pow(-2 * {u} + 2, 4) / 2"));
        let g=self.number(until.map(|end|format!("Math.pow(Math.min(1, Math.max(0, (computed.sourceClock - {end}) / 360)), 3)")).unwrap_or_else(||"0".into()));
        let(mut flash,mut flash_u)=("0".to_owned(),"-1".to_owned());
        for &beat in flashes{
            let u=self.number(format!("{secs} - {beat}"));
            let env=self.number(format!("{u} < 0 ? 0 : ({u} < 0.18 ? 1 - Math.pow(1 - {u} / 0.18, 3) : ({u} < 1 ? 1 : Math.pow(1 - Math.min(1, Math.max(0, ({u} - 1) / 0.45)), 3)))"));
            // Strict comparison preserves the first authored flash on equal plateaus.
            flash_u=self.number(format!("{env} > {flash} ? {u} : ({flash_u})"));
            flash=self.number(format!("Math.max({flash}, {env})"));
        }
        let mut selected="0".to_owned();
        if let Some(mode)=mode.filter(|_|self.selection_count>0){for(i,&at)in self.selection_steps.iter().enumerate(){selected=format!("{secs} >= {at} ? {} : ({selected})",usize::from(i%self.selection_count==mode));}}
        let selected=self.number(selected);
        let w=self.number(if leader{width.to_string()}else{format!("{width} + {} * {selected} + 1.4 * {flash}",if strong{0.0}else{0.5})});
        let paint=if leader{q(color)}else{let amount=if matches!(k,"blocked"|"exception"){selected.clone()}else{self.number(format!("{selected} > 0 ? 1 : {flash}"))};format!("{{{}}}",self.source_mix_expression(color,self.theme.accent,&amount))};
        let css_width=self.css_number(&w,1);
        let start=self.number(format!("{g} * {total}"));
        let stop=self.number(if blocked{format!("Math.min({f} * {total}, {} - {w} * 2)",total*0.58)}else{format!("{f} * {total}")});
        let line_end=self.number(format!("Math.max({start}, {stop} - {})",if has_head{arrow*0.6}else{0.0}));
        let (sx,sy)=self.connector_point(points,&lengths,&start);
        let (ex,ey)=self.connector_point(points,&lengths,&line_end);
        let(sx,sy,ex,ey)=(self.css_number(&sx,1),self.css_number(&sy,1),self.css_number(&ex,1),self.css_number(&ey,1));
        let mut d=format!("\"M\" + {sx} + \" \" + {sy}");
        for(i,p)in points.iter().enumerate().skip(1){let x=(p.0*10.0+0.5).floor()/10.0;let y=(p.1*10.0+0.5).floor()/10.0;d.push_str(&format!(" + ({start} < {} && {line_end} > {} ? {} : \"\")",lengths[i],lengths[i],q(&format!(" L{x} {y}"))));}
        d.push_str(&format!(" + \" L\" + {ex} + \" \" + {ey}"));
        d=format!("{line_end} - {start} < 0.5 ? \"M0 0\" : ({d})");
        let dash=if matches!(k,"exception"|"branch"){q("9 9")}else if leader{let n=(width*32.0+0.5).floor()/10.0;q(&format!("{n} {n}"))}else if k=="affinity"{
            let gap=self.css_number(&format!("{w} * 4"),1);let name=format!("motion{}",self.counter);self.counter+=1;self.computeds.push(format!("{name}:computed.points(() => [0.1, {gap}])"));format!("{{computed.{name}}}")
        }else{q("none")};
        let offset=if leader{self.css_number(&start,1)}else if matches!(k,"exception"|"branch"|"affinity"){
            let period=if k=="affinity"{self.number(format!("0.1 + {w} * 4"))}else{"18".into()};
            let distance=self.number(format!("Math.max(0, (computed.sourceClock - {at}) / 1000) * {} - {start}",if k=="affinity"{18}else{28}));
            self.css_number(&format!("-({distance} - Math.floor({distance} / {period}) * {period})"),1)
        }else{"0".into()};
        let op=self.number(format!("{f} > 0.001 && {g} < 0.999 ? 1 : 0"));
        self.connector_svg(id,&d,&format!("fill=\"none\" stroke={paint} strokeWidth={{{css_width}}} strokeLinecap=\"round\" strokeLinejoin=\"round\" strokeDasharray={dash} strokeDashoffset={{{offset}}} opacity={{{op}}}"));
        if has_head||blocked||leader{
            let visible=if leader{format!("{f} > 0.9 && {g} < 0.92")}else if blocked{format!("{f} * {total} > {} - {w} * 2 && {g} < 0.92",total*0.58)}else{format!("{f} > 0.04 && {g} < 0.92")};
            let hop=self.css_number(&format!("{visible} ? 1 - {g} : 0"),1);
            let head=if leader{
                let u=self.number(format!("Math.min(1, Math.max(0, ({f} - 0.9) / 0.1)) - 1"));
                let r=self.number(format!("{} * Math.max(0.01, 1 + 2.7 * Math.pow({u}, 3) + 1.7 * Math.pow({u}, 2))",width*2.4));
                self.connector_circle(&points.last().unwrap().0.to_string(),&points.last().unwrap().1.to_string(),&r)
            }else{
                let tip=if blocked{(total*0.58).to_string()}else{stop.clone()};let back=self.number(format!("Math.max(0, {tip} - 4)"));
                let(tx,ty)=self.connector_point(points,&lengths,&tip);let(bx,by)=self.connector_point(points,&lengths,&back);
                let dx=self.number(format!("{tx} - {bx}"));let dy=self.number(format!("{ty} - {by}"));let len=self.number(format!("Math.sqrt({dx} * {dx} + {dy} * {dy})"));
                let ux=self.number(format!("{dx} / ({len} > 0 ? {len} : 1)"));let uy=self.number(format!("{dy} / ({len} > 0 ? {len} : 1)"));
                let vertices=if blocked{
                    let u=self.number(format!("Math.min(1, Math.max(0, ({f} * {total} - {} + {w} * 2) / 30)) - 1",total*0.58));
                    let hl=self.number(format!("{} * (1 + 2.7 * Math.pow({u}, 3) + 1.7 * Math.pow({u}, 2))",arrow*1.25));let ht=self.number(format!("{w} * 1.3"));
                    vec![(format!("{tx} - {uy} * {hl} - {ux} * {ht}"),format!("{ty} + {ux} * {hl} - {uy} * {ht}")),(format!("{tx} + {uy} * {hl} - {ux} * {ht}"),format!("{ty} - {ux} * {hl} - {uy} * {ht}")),(format!("{tx} + {uy} * {hl} + {ux} * {ht}"),format!("{ty} - {ux} * {hl} + {uy} * {ht}")),(format!("{tx} - {uy} * {hl} + {ux} * {ht}"),format!("{ty} + {ux} * {hl} + {uy} * {ht}"))]
                }else{vec![(tx.clone(),ty.clone()),(format!("{tx} - {ux} * {arrow} - {uy} * {}",arrow*0.6),format!("{ty} - {uy} * {arrow} + {ux} * {}",arrow*0.6)),(format!("{tx} - {ux} * {arrow} + {uy} * {}",arrow*0.6),format!("{ty} - {uy} * {arrow} - {ux} * {}",arrow*0.6))]};
                let mut d="\"\"".to_owned();for(i,(x,y))in vertices.iter().enumerate(){let x=self.css_number(x,1);let y=self.css_number(y,1);d.push_str(&format!(" + {} + {x} + \" \" + {y}",q(if i==0{"M"}else{" L"})));}d.push_str(" + \" Z\"");d
            };
            self.connector_svg(&format!("{id}-head"),&head,&format!("fill={paint} stroke={paint} strokeWidth={{1}} strokeLinejoin=\"round\" opacity={{{hop}}}"));
        }
        if !flashes.is_empty()||(k=="flow"&&c["pulse"].as_bool().unwrap_or(false)){
            let before=until.map(|end|format!("computed.sourceClock < {end}")).unwrap_or_else(||"true".into());
            let beat=self.number(format!("{flash} > 0 && {flash_u} >= 0 && {flash_u} < 1.1 && {before} ? 1 : 0"));
            let pos=self.number(format!("(computed.sourceClock - {at} - {draw}) / 1000 * 150"));let pos=self.number(format!("{pos} - Math.floor({pos} / {}) * {} - 40",total+80.0,total+80.0));
            let flow=if k=="flow"&&c["pulse"].as_bool().unwrap_or(false){format!("computed.sourceClock > {} && {before} && {pos} > 0 && {pos} < {}",at+draw,total-arrow)}else{"false".into()};
            let distance=self.number(format!("{beat} > 0 ? Math.min(1, {flash_u} / 0.9) * {} : ({flow} ? {pos} : 0)",if blocked{total*0.58}else{total}));
            let(px,py)=self.connector_point(points,&lengths,&distance);
            let sx=self.css_number(&format!("{beat} > 0 || ({flow}) ? {px} : 0"),1);let sy=self.css_number(&format!("{beat} > 0 || ({flow}) ? {py} : 0"),1);
            let r=self.css_number(&format!("{w} * 1.9"),1);
            let pop=self.css_number(&format!("{beat} > 0 ? Math.min(1, Math.max(0, {flash} * 1.2)) : ({flow} ? Math.min(1, Math.max(0, Math.min({pos}, {} - {pos}) / 30)) : 0)",total-arrow),1);
            let d=self.connector_circle(&sx,&sy,&r);self.connector_svg(&format!("{id}-pulse"),&d,&format!("fill={paint} stroke=\"none\" opacity={{{pop}}}"));
        }
    }
    fn connector_point(&mut self,points:&[(f64,f64)],lengths:&[f64],distance:&str)->(String,String){
        let mut coords=[String::new(),String::new()];
        for(axis,out)in coords.iter_mut().enumerate(){
            let mut intervals=Vec::with_capacity(points.len()-1);
            for i in (1..points.len()).rev(){let span=lengths[i]-lengths[i-1];let a=if axis==0{points[i-1].0}else{points[i-1].1};let b=if axis==0{points[i].0}else{points[i].1};let value=if span>0.0{format!("{a} + ({b} - {a}) * ({distance} - {}) / {span}",lengths[i-1])}else{a.to_string()};intervals.push((lengths[i],value));}
            let last=if axis==0{points.last().unwrap().0}else{points.last().unwrap().1};
            *out=source_event_expression(distance,&intervals,&last.to_string(),"<=");
        }
        (self.number(format!("{distance} <= 0 ? {} : ({})",points[0].0,coords[0])),self.number(format!("{distance} <= 0 ? {} : ({})",points[0].1,coords[1])))
    }
    fn connector_circle(&mut self,x:&str,y:&str,r:&str)->String{let left=self.css_number(&format!("{x} - {r}"),1);let right=self.css_number(&format!("{x} + {r}"),1);let y=self.css_number(y,1);let r=self.css_number(r,1);format!("\"M\" + {left} + \" \" + {y} + \"A\" + {r} + \" \" + {r} + \" 0 1 0 \" + {right} + \" \" + {y} + \"A\" + {r} + \" \" + {r} + \" 0 1 0 \" + {left} + \" \" + {y} + \"Z\"")}
    fn connector_svg(&mut self,id:&str,d:&str,attrs:&str){let name=format!("motion{}",self.counter);self.counter+=1;self.computeds.push(format!("{name}:computed.string(() => {d})"));let s=self.scale;self.body.push_str(&format!("<Path id={} d={{computed.{name}}} x={{0}} y={{0}} width={{{}}} height={{{}}} viewBox={} trimEnd={{1}} {attrs}/>",q(id),self.w*s,self.h*s,q(&format!("0 0 {} {}",self.w,self.h))));}
    fn source_mix_expression(&mut self,left:&str,right:&str,amount:&str)->String{
        let(mut a,mut b)=(source_color(left).expect("source color"),source_color(right).expect("source color"));if a[3]==0.0{a[..3].copy_from_slice(&b[..3]);}if b[3]==0.0{b[..3].copy_from_slice(&a[..3]);}
        let channels=(0..3).map(|i|format!("Math.floor(({} + ({} - {}) * ({amount})) + 0.5) / 255",a[i],b[i],a[i])).collect::<Vec<_>>();let alpha=self.css_number(&format!("{} + ({} - {}) * ({amount})",a[3],b[3],a[3]),3);
        let name=format!("motion{}",self.counter);self.counter+=1;self.computeds.push(format!("{name}:computed.color(() => color.rgb({}, {}, {}, {alpha}))",channels[0],channels[1],channels[2]));format!("computed.{name}")
    }
}
