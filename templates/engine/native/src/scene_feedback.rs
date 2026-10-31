fn source_spring(theme:&str)->(f64,f64,f64,f64){match theme{"daylight"=>(1.0,2.1,5.0,0.20),"midnight"=>(0.7,1.4,3.6,0.26),"blueprint"=>(0.55,3.0,8.0,0.12),"signal"=>(1.5,2.5,4.2,0.18),_=>unreachable!()}}
fn source_cubic(u:f64)->f64{1.0-(1.0-u.clamp(0.0,1.0)).powi(3)}
fn source_windup(u:f64,ant:f64)->f64{if u< -ant{0.0}else if u<0.0{source_cubic((u+ant)/ant)}else{(1.0-u/0.08).max(0.0)}}
fn source_hold(u:f64,hold:f64)->f64{if u<0.0{0.0}else if u<0.18{source_cubic(u/0.18)}else if u<hold{1.0}else{(1.0-source_cubic((u-hold)/0.45)).max(0.0)}}
impl<'a> Scene<'a>{
    fn hold_envelope(&mut self,at:f64,hold:f64)->String{
        let u=self.number(format!("computed.sourceClock / 1000 - {at}"));
        self.number(format!("{u} < 0 ? 0 : ({u} < 0.18 ? 1 - Math.pow(1 - {u} / 0.18, 3) : ({u} < {hold} ? 1 : Math.pow(1 - Math.min(1, Math.max(0, ({u} - {hold}) / 0.45)), 3)))"))
    }
    fn hold_ring(&mut self,id:&str,item:&Value,x:f64,y:f64,w:f64,h:f64,radius:f64){
        let Some(at)=item["hb"]["t"].as_f64()else{return};let hold=item["hb"]["hold"].as_f64().unwrap();let color=if item["tone"]=="warning"{self.theme.warning}else{self.theme.accent};let s=self.scale;
        let e=self.hold_envelope(at,hold);let r=self.css_number(&format!("1.5 + 2.5 * {e}"),1);let opacity=self.css_number(&format!("0.6 * {e}"),3);
        let attrs=[("x",format!("({x} - {r} * 0.5) * {s}")),("y",format!("({y} - {r} * 0.5) * {s}")),("width",format!("({w} + {r}) * {s}")),("height",format!("({h} + {r}) * {s}")),("radius",format!("({radius} + {r} * 0.5) * {s}")),("strokeWidth",format!("{r} * {s}"))].into_iter().map(|(prop,value)|format!(" {prop}={{{}}}",self.number(value))).collect::<String>();
        self.body.push_str(&format!("<Rect id={}{attrs} fill=\"transparent\" stroke={} opacity={{{opacity}}}/>",q(id),q(color)));
    }
    fn hold_fill(&mut self,item:&Value,base:&str)->String{
        let Some(at)=item["hb"]["t"].as_f64()else{return q(base)};let hold=item["hb"]["hold"].as_f64().unwrap();let target=if item["tone"]=="warning"{self.theme.warning_tint}else{self.theme.tint};
        let e=self.hold_envelope(at,hold);let amount=self.number(format!("{e} * 0.85"));format!("{{{}}}",self.source_mix_expression(base,target,&amount))
    }
}
#[cfg(test)]mod feedback_tests{use super::*;#[test]fn narration_hold_and_anticipation_have_source_boundaries(){assert_eq!(source_hold(-0.1,1.0),0.0);assert_eq!(source_hold(0.18,1.0),1.0);assert_eq!(source_hold(1.0,1.0),1.0);assert_eq!(source_hold(1.45,1.0),0.0);assert_eq!(source_windup(-0.3,0.2),0.0);assert_eq!(source_windup(0.0,0.2),1.0);assert_eq!(source_windup(0.08,0.2),0.0);assert!(source_hold(0.09,1.0)>0.5);}}
