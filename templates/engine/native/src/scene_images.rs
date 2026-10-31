impl<'a> Scene<'a> {
    pub fn source_image(&mut self,id:&str,item:&Value,x:f64,y:f64,w:f64,h:f64,at:f64,until:Option<f64>)->Result<(),String>{
        let src=item["img"].as_str().ok_or("image node requires img")?;
        let fit=item["fit"].as_str().unwrap_or("cover");
        let s=self.scale;let radius=(self.theme.radius*1.2*10.0).round()/10.0;
        let theme=&SOURCE_THEMES[self.theme.name];
        let border=theme["cardBorder"].as_str().unwrap().split("px").next().unwrap().parse::<f64>().expect("source image border width");
        let (iw,ih)=(w-2.0*border,h-2.0*border);
        // Image CSS uses a centered origin rather than the card's 60% origin.
        self.begin_theme_motion(&format!("{id}-motion"),item,x,y-h*0.1,w,h,at,until,false,true);
        self.body.push_str(&format!("<Rect id={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} radius={{{}}} fill={} border={} shadow={}/>",q(&format!("{id}-frame")),x*s,y*s,w*s,h*s,radius*s,q(self.theme.surface),q(&source_scale_px(theme["cardBorder"].as_str().unwrap(),s)),q(&source_scale_px(theme["shadow"].as_str().unwrap(),s))));
        self.body.push_str(&format!("<MotionBox id={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} clipRadius={{{}}}>",q(&format!("{id}-clip")),(x+border)*s,(y+border)*s,iw*s,ih*s,(radius-border).max(0.0)*s));
        let secs=self.number("computed.sourceClock / 1000");
        let duration=item["dur"].as_f64().filter(|v|*v!=0.0).unwrap_or(6.0);
        let u=self.number(format!("Math.min(1, Math.max(0, ({secs} + 0.5) / {}))",duration+1.5));
        let kb=self.number(format!("-(Math.cos({} * {u}) - 1) / 2",std::f64::consts::PI));
        let zoom=self.css_number(&format!("1.1 - 0.07 * {kb}"),3);
        let pan=self.css_number(&format!("-14 + 28 * {kb}"),1);
        let dx=self.number(format!("{pan} * {zoom} * {s}"));
        // CSS scale(...) translateX(...) scales the already CSS-quantized pan.
        self.body.push_str(&format!("<MotionBox id={} x={{{dx}}} scaleX={{{zoom}}} scaleY={{{zoom}}} pivotX={{{}}} pivotY={{{}}}>",q(&format!("{id}-kenburns")),iw/2.0*s,ih/2.0*s));
        self.body.push_str(&format!("<Image id={} src={} x={{0}} y={{0}} width={{{}}} height={{{}}} fit={}/>",q(id),q(src),iw*s,ih*s,q(fit)));
        self.end_move();self.end_move();self.end_move();self.anchors.push((x+w/2.0,y+h/2.0));Ok(())
    }
}
