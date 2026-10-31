use crate::grammar;
use serde_json::{json, Map, Value};

include!("scene_helpers.rs");
include!("scene_connectors.rs");
include!("scene_choreography.rs");
include!("scene_backgrounds.rs");
include!("scene_icons.rs");
include!("scene_feedback.rs");
include!("scene_pictures.rs");
include!("scene_images.rs");
include!("scene_styles.rs");

#[derive(Clone, Copy)]
pub(crate) struct Theme<'a> {
    pub name: &'a str,
    pub bg: &'a str,
    pub ink: &'a str,
    pub muted: &'a str,
    pub surface: &'a str,
    pub focus: &'a str,
    pub focus_ink: &'a str,
    pub accent: &'a str,
    pub tint: &'a str,
    pub warning: &'a str,
    pub warning_tint: &'a str,
    pub line: &'a str,
    pub caption_bg: &'a str,
    pub caption_ink: &'a str,
    pub font: &'a str,
    pub radius: f64,
    pub weight: u32,
    pub enter: f64,
    pub tracking: f64,
}
impl<'a> Theme<'a> {
    pub fn get(name: &str) -> Result<Self, String> {
        Ok(match name {
            "daylight" => Self {
                name: "daylight",
                bg: "#EDEFF3",
                ink: "#15171C",
                muted: "#4A5160",
                surface: "#FFFFFF",
                focus: "#15171C",
                focus_ink: "#FFFFFF",
                accent: "#2E5BE6",
                tint: "#E3EAFD",
                warning: "#C2510F",
                warning_tint: "#FBE9DD",
                line: "#8E95A2",
                caption_bg: "rgba(255,255,255,0.9)",
                caption_ink: "#15171C",
                font: "Geist",
                radius: 18.0,
                weight: 600,
                enter: 800.0,
                tracking: -0.025,
            },
            "midnight" => Self {
                name: "midnight",
                bg: "#0A0D14",
                ink: "#EEF1F6",
                muted: "#A3ABBA",
                surface: "#141A25",
                focus: "#7AA2FF",
                focus_ink: "#0A0D14",
                accent: "#7AA2FF",
                tint: "rgba(122,162,255,0.16)",
                warning: "#FF9D5C",
                warning_tint: "rgba(255,157,92,0.16)",
                line: "rgba(238,241,246,0.38)",
                caption_bg: "rgba(8,10,16,0.62)",
                caption_ink: "#EEF1F6",
                font: "Space Grotesk",
                radius: 22.0,
                weight: 600,
                enter: 1050.0,
                tracking: -0.02,
            },
            "blueprint" => Self {
                name: "blueprint",
                bg: "#0E2C66",
                ink: "#FFFFFF",
                muted: "rgba(255,255,255,0.78)",
                surface: "rgba(14,44,102,0.78)",
                focus: "#FFFFFF",
                focus_ink: "#0E2C66",
                accent: "#FFD23F",
                tint: "rgba(255,210,63,0.16)",
                warning: "#FF8A7A",
                warning_tint: "rgba(255,138,122,0.16)",
                line: "rgba(255,255,255,0.72)",
                caption_bg: "#FFFFFF",
                caption_ink: "#0E2C66",
                font: "IBM Plex Sans",
                radius: 6.0,
                weight: 600,
                enter: 750.0,
                tracking: -0.01,
            },
            "signal" => Self {
                name: "signal",
                bg: "#FFE14D",
                ink: "#141414",
                muted: "#3D3510",
                surface: "#FFFFFF",
                focus: "#141414",
                focus_ink: "#FFE14D",
                accent: "#3D2BFF",
                tint: "#E6E3FF",
                warning: "#E5484D",
                warning_tint: "#FFE3E4",
                line: "#141414",
                caption_bg: "#141414",
                caption_ink: "#FFE14D",
                font: "Archivo",
                radius: 14.0,
                weight: 800,
                enter: 620.0,
                tracking: -0.02,
            },
            _ => return Err(format!("unknown theme {name:?}")),
        })
    }
    pub fn from_video(name: &str, tokens: Option<&'a Value>) -> Result<Self, String> {
        let mut theme = Self::get(name)?;
        if let Some(tokens) = tokens {
            grammar::fields(
                tokens,
                &[
                    "background",
                    "ink",
                    "muted",
                    "surface",
                    "focus",
                    "focusInk",
                    "accent",
                    "accentTint",
                    "exception",
                    "exceptionTint",
                    "line",
                    "captionBackground",
                    "captionInk",
                    "fontFamily",
                    "headWeight",
                    "radius",
                    "entranceMs",
                ],
                "themeTokens",
            )?;
            for (key, value) in tokens.as_object().ok_or("themeTokens must be an object")? {
                if matches!(key.as_str(), "headWeight" | "radius" | "entranceMs") {
                    let n = value
                        .as_f64()
                        .filter(|n| n.is_finite())
                        .ok_or_else(|| format!("themeTokens.{key} must be finite"))?;
                    match key.as_str() {
                        "headWeight" if (100.0..=900.0).contains(&n) && n.fract() == 0.0 => {
                            theme.weight = n as u32
                        }
                        "radius" if (0.0..=100.0).contains(&n) => theme.radius = n,
                        "entranceMs" if (50.0..=3000.0).contains(&n) => theme.enter = n,
                        _ => {
                            return Err(format!("themeTokens.{key} is outside its supported range"))
                        }
                    }
                    continue;
                }
                let value = value
                    .as_str()
                    .ok_or_else(|| format!("themeTokens.{key} must be a string"))?;
                if key == "fontFamily" {
                    if crate::typography::css_weight(value, theme.weight as u16).is_none() {
                        return Err(format!("themeTokens.fontFamily {value:?} is not bundled"));
                    }
                    theme.font = value;
                    continue;
                }
                if !value.starts_with('#')
                    || !matches!(value.len(), 4 | 5 | 7 | 9)
                    || !value[1..].bytes().all(|b| b.is_ascii_hexdigit())
                {
                    return Err(format!("themeTokens.{key} must be a hexadecimal color"));
                }
                match key.as_str() {
                    "background" => theme.bg = value,
                    "ink" => theme.ink = value,
                    "muted" => theme.muted = value,
                    "surface" => theme.surface = value,
                    "focus" => theme.focus = value,
                    "focusInk" => theme.focus_ink = value,
                    "accent" => theme.accent = value,
                    "accentTint" => theme.tint = value,
                    "exception" => theme.warning = value,
                    "exceptionTint" => theme.warning_tint = value,
                    "line" => theme.line = value,
                    "captionBackground" => theme.caption_bg = value,
                    "captionInk" => theme.caption_ink = value,
                    _ => unreachable!(),
                }
            }
        }
        Ok(theme)
    }
    pub fn value(self) -> Value {
        let mut tokens=SOURCE_THEMES[self.name].clone();
        let object=tokens.as_object_mut().expect("source theme object");
        object.extend(json!({"name":self.name,"background":self.bg,"ink":self.ink,"muted":self.muted,"surface":self.surface,"focus":self.focus,"focusInk":self.focus_ink,"accent":self.accent,"accentTint":self.tint,"exception":self.warning,"exceptionTint":self.warning_tint,"line":self.line,"fontFamily":self.font,"headWeight":self.weight,"radius":self.radius,"entranceMs":self.enter,"tracking":self.tracking}).as_object().unwrap().clone());
        tokens
    }
    pub fn tone(self, tone: &str) -> (&'a str, &'a str) {
        match tone {
            "accent" => (self.tint, self.accent),
            "warning" => (self.warning_tint, self.warning),
            "muted" => (self.surface, self.muted),
            _ => (self.surface, self.ink),
        }
    }
}
pub(crate) struct Scene<'a> {
    pub id: String,
    pub w: f64,
    pub h: f64,
    pub tall: bool,
    pub margin: f64,
    pub top: f64,
    pub bottom: f64,
    pub start: f64,
    pub duration: f64,
    pub render_end:f64,
    pub transition_nodes:Vec<Value>,
    pub exit_scene: bool,
    pub exit_start: Option<f64>,
    pub transition_holds: std::collections::BTreeSet<usize>,
    pub background_camera: Vec<(f64,f64)>,
    pub selection_steps:Vec<f64>,
    pub selection_count:usize,
    pub music: Option<crate::music::Music>,
    pub brands: Option<&'a std::collections::BTreeMap<String,String>>,
    pub theme: Theme<'a>,
    pub anchors: Vec<(f64, f64)>,
    width: u32,
    height: u32,
    scale: f64,
    body: String,
    variables: Map<String, Value>,
    computeds: Vec<String>,
    music_motion: Option<(String, String, String)>,
    counter: usize,
}
pub(crate) fn q(s: &str) -> String {
    serde_json::to_string(s).expect("serializable string")
}
impl<'a> Scene<'a> {
    pub fn new(
        id: String,
        width: u32,
        height: u32,
        duration: f64,
        start: f64,
        theme: Theme<'a>,
    ) -> Self {
        let tall = height as f64 >= width as f64 * 0.95;
        let w = if tall { 720.0 } else { 1280.0 };
        let scale = width as f64 / w;
        let h = height as f64 / scale;
        let margin = if tall { 48.0 } else { 96.0 };
        Self {
            id,
            w,
            h,
            tall,
            margin,
            top: if tall { 330.0 } else { 164.0 },
            bottom: if tall { 930.0 } else { 572.0 },
            start,
            duration,
            render_end:duration,
            transition_nodes:Vec::new(),
            exit_scene:true,
            exit_start:None,
            transition_holds:std::collections::BTreeSet::new(),
            background_camera:Vec::new(),
            selection_steps:Vec::new(),
            selection_count:0,
            music:None,
            brands:None,
            theme,
            anchors: Vec::new(),
            width,
            height,
            scale,
            body: String::new(),
            variables: Map::new(),
            computeds: Vec::new(),
            music_motion: None,
            counter: 0,
        }
    }
    pub fn local_end(&self, at: f64) -> Result<f64, String> {
        let t = (at - self.start) * 1000.0;
        if !t.is_finite() || t < 0.0 || t > self.duration + 0.000001 {
            return Err(format!("end time {at} is outside scene"));
        }
        Ok(t.min(self.duration))
    }
    pub fn local(&self, absolute: f64) -> Result<f64, String> {
        let t = (absolute - self.start) * 1000.0;
        if t < 0.0 || t >= self.duration {
            Err(format!("time {absolute} must fall within the scene"))
        } else {
            Ok(t)
        }
    }
    pub fn at(&self, value: &Value, default: f64) -> Result<f64, String> {
        value
            .get("at")
            .map(|_| grammar::number(value, "at").and_then(|t| self.local(t)))
            .unwrap_or(Ok(default))
    }
    fn animation(&self, at: f64, until: Option<f64>) -> String {
        if until.is_some_and(|end| end <= at) {
            return String::new();
        }
        let speed = match self.theme.name { "daylight" => 2.6, "midnight" => 1.8, "signal" => 4.0, _ => self.theme.enter };
        let d = self.theme.enter / speed;
        let mut s = format!("<Animate prop=\"opacity\" from={{0}} to={{1}} start=\"{at}ms\" duration=\"{d}ms\" ease=\"linear\" />");
        if let Some(end) = until {
            let out = match self.theme.name { "midnight" => 500.0, "blueprint" => 400.0, "signal" => 320.0, _ => 420.0 };
            s.push_str(&format!("<Animate prop=\"opacity\" from={{1}} to={{0}} start=\"{end}ms\" duration=\"{out}ms\" ease=\"inCubic\" />"));
        }
        s
    }
    pub fn rect(
        &mut self,
        id: &str,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        color: &str,
        at: f64,
        until: Option<f64>,
    ) {
        let s = self.scale;
        let a = self.animation(at, until);
        self.body.push_str(&format!("<Rect id={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} radius={{{}}} fill={} opacity={{0}}>{a}</Rect>",q(id),x*s,y*s,w*s,h*s,self.theme.radius*s,q(color)));
    }
    pub fn text(
        &mut self,
        id: &str,
        label: &str,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        size: f64,
        color: &str,
        at: f64,
        until: Option<f64>,
        align: &str,
    ) {
        let s = self.scale;
        let key = format!("text{}", self.counter);
        self.counter += 1;
        self.variables.insert(key.clone(), json!(label));
        // Layout owns font fitting. Never shrink authored type using a character-count guess.
        let line_height = (size * 1.1).round();
        let measured = crate::typography::measure(label, &self.theme, size, w.max(1.0), line_height / size, true);
        let text_y = y + (h - measured.height).max(0.0) / 2.0;
        let a = self.animation(at, until);
        self.body.push_str(&format!("<Text id={} text={{vars.{key}}} x={{{}}} y={{{}}} width={{{}}} height={{{}}} size={{{}}} lineHeight={{{}}} letterSpacing={{{}}} fontFamily={} weight={} color={} align={} opacity={{0}}>{a}</Text>",q(id),x*s,text_y*s,w*s,h*s,size*s,line_height*s,self.theme.tracking,q(self.theme.font),q(&self.theme.weight.to_string()),q(color),q(align)));
    }
    pub fn line(
        &mut self,
        id: &str,
        points: &[(f64, f64)],
        color: &str,
        width: f64,
        at: f64,
        until: Option<f64>,
    ) {
        let p: Vec<_> = points
            .iter()
            .map(|(x, y)| json!([x * self.scale, y * self.scale]))
            .collect();
        let a = self.animation(at, until);
        self.body.push_str(&format!(
            "<Path id={} points={{{}}} stroke={} strokeWidth={{{}}} opacity={{0}}>{a}</Path>",
            q(id),
            json!(p),
            q(color),
            width * self.scale
        ));
    }
    pub fn edge(
        &mut self,
        id: &str,
        a: (f64, f64),
        b: (f64, f64),
        kind: &str,
        at: f64,
        until: Option<f64>,
    ) {
        let horizontal=(b.0-a.0).abs()>=(b.1-a.1).abs();
        let (si,sj,ax,ay,bx,by)=if horizontal {
            if b.0>=a.0 {("r","l",a.0-10.0,a.1,b.0+10.0,b.1)} else {("l","r",a.0+10.0,a.1,b.0-10.0,b.1)}
        } else if b.1>=a.1 {("b","t",a.0,a.1-10.0,b.0,b.1+10.0)} else {("t","b",a.0,a.1+10.0,b.0,b.1-10.0)};
        let nodes=vec![json!({"t":"dot","x":ax,"y":ay,"w":0.0,"h":0.0}),json!({"t":"dot","x":bx,"y":by,"w":0.0,"h":0.0})];
        let edges=vec![json!({"i":0,"j":1,"k":kind,"si":si,"sj":sj})];
        let routes=self.route_layout_edges(&nodes,&edges).expect("valid endpoint connector");
        self.routed_edge(id,&routes[0],kind,at,until,&[],None);
    }
    pub fn icon(
        &mut self, id: &str, name: &str, x: f64, y: f64, size: f64,
        color: &str, at: f64, until: Option<f64>,
    ) {
        static ICONS: std::sync::LazyLock<Value> = std::sync::LazyLock::new(|| serde_json::from_str(include_str!("icons.json")).expect("checked icon catalogue"));
        let d = ICONS.get(name).and_then(Value::as_str).unwrap_or_else(|| panic!("unknown native icon {name:?}"));
        self.svg_path(id, d, x, y, size, size, "0 0 24 24", "none", color, 1.8, at, until);
    }
    pub fn svg_path(
        &mut self, id: &str, d: &str, x: f64, y: f64, w: f64, h: f64,
        view_box: &str, fill: &str, stroke: &str, width: f64, at: f64, until: Option<f64>,
    ) {
        let s = self.scale;
        let a = self.animation(at, until);
        self.body.push_str(&format!("<Path id={} d={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} viewBox={} fill={} stroke={} strokeWidth={{{width}}} strokeLinecap=\"round\" strokeLinejoin=\"round\" opacity={{0}}>{a}</Path>", q(id),q(d),x*s,y*s,w*s,h*s,q(view_box),q(fill),q(stroke)));
    }
    pub fn node(
        &mut self, id: &str, item: &Value, x: f64, y: f64, w: f64, h: f64,
        focus: bool, at: f64, until: Option<f64>,
    ) {
        if item.get("base").is_some() {
            self.picture_tile(id,item,x,y,w,h,at,until).expect("validated picture tile");
        } else {
            self.layout_node(if focus {"focusCol"} else {"item"},id,item,x,y,w,h,at,until).expect("validated node type");
        }
    }
    pub fn image(
        &mut self,
        id: &str,
        uri: &str,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        fit: &str,
        at: f64,
        until: Option<f64>,
    ) {
        let s = self.scale;
        let a = self.animation(at, until);
        self.body.push_str(&format!("<Image id={} src={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} fit={} opacity={{0}}>{a}</Image>",q(id),q(uri),x*s,y*s,w*s,h*s,q(fit)));
    }
    pub fn begin_window(&mut self, id: &str, at: f64, until: Option<f64>) {
        let animation = self.animation(at, until);
        self.body.push_str(&format!("<MotionBox id={} x={{0}} y={{0}} width={{{}}} height={{{}}} opacity={{0}}>{animation}", q(id), self.width, self.height));
    }
    pub fn begin_move(&mut self, id: &str, dx: f64, dy: f64, at: f64, duration: f64) {
        let s = self.scale;
        self.body.push_str(&format!("<MotionBox id={} x={{0}} y={{0}} width={{{}}} height={{{}}}><Animate prop=\"x\" from={{{}}} to={{0}} start=\"{at}ms\" duration=\"{duration}ms\" ease=\"inOutCubic\" /><Animate prop=\"y\" from={{{}}} to={{0}} start=\"{at}ms\" duration=\"{duration}ms\" ease=\"inOutCubic\" />",q(id),self.width,self.height,dx*s,dy*s));
    }
    pub fn end_move(&mut self) {
        self.body.push_str("</MotionBox>");
    }
    pub fn finish_parts(mut self) -> (String, String, Value, String) {
        self.apply_theme_camera();
        let foreground = std::mem::take(&mut self.body);
        self.theme_background();
        (self.body, foreground, Value::Object(self.variables), self.computeds.join(","))
    }
    pub fn finish(mut self) -> Result<(String, Value), String> {
        if !self.id.contains("caption") {
            self.apply_theme_camera();
            let foreground=std::mem::take(&mut self.body);
            self.theme_background();
            self.body.push_str(&foreground);
        }
        let vars = self
            .variables
            .iter()
            .map(|(key, value)| format!("{key}:stringVar({value})"))
            .collect::<Vec<_>>()
            .join(",");
        let background=String::new();
        let computeds=self.computeds.join(",");
        let source=format!("export default defineSpanScene({{id:{},width:{},height:{},durationMs:{},variables:{{{vars}}},computeds:{{sourceClock:computed.number(() => time.ms),{computeds}}},render({{vars}}){{return <Scene>{background}{}</Scene>;}}}});",q(&self.id),self.width,self.height,self.duration,self.body);
        Ok((source, Value::Object(self.variables)))
    }
}
