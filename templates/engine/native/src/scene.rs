use crate::grammar;
use serde_json::{json, Map, Value};

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
                caption_bg: "#FFFFFFE6",
                caption_ink: "#15171C",
                font: "Geist",
                radius: 18.0,
                weight: 600,
                enter: 800.0,
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
                tint: "#7AA2FF29",
                warning: "#FF9D5C",
                warning_tint: "#FF9D5C29",
                line: "#EEF1F661",
                caption_bg: "#0000009E",
                caption_ink: "#EEF1F6",
                font: "Space Grotesk",
                radius: 22.0,
                weight: 600,
                enter: 1050.0,
            },
            "blueprint" => Self {
                name: "blueprint",
                bg: "#0E2C66",
                ink: "#FFFFFF",
                muted: "#FFFFFFC7",
                surface: "#0E2C66C7",
                focus: "#FFFFFF",
                focus_ink: "#0E2C66",
                accent: "#FFD23F",
                tint: "#FFD23F29",
                warning: "#FF8A7A",
                warning_tint: "#FF8A7A29",
                line: "#FFFFFFB8",
                caption_bg: "#FFFFFF",
                caption_ink: "#0E2C66",
                font: "IBM Plex Sans",
                radius: 6.0,
                weight: 600,
                enter: 750.0,
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
                    if value.is_empty() || value.len() > 80 || value.chars().any(char::is_control) {
                        return Err("invalid themeTokens.fontFamily".into());
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
        json!({"name":self.name,"background":self.bg,"ink":self.ink,"muted":self.muted,"surface":self.surface,"focus":self.focus,"focusInk":self.focus_ink,"accent":self.accent,"accentTint":self.tint,"exception":self.warning,"exceptionTint":self.warning_tint,"line":self.line,"fontFamily":self.font,"headWeight":self.weight,"radius":self.radius,"entranceMs":self.enter,"caption":{"background":self.caption_bg,"ink":self.caption_ink},"highlight":match self.name {"blueprint"=>"underline","signal"=>"block","midnight"=>"glow",_=>"marker"}})
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
    pub theme: Theme<'a>,
    pub anchors: Vec<(f64, f64)>,
    width: u32,
    height: u32,
    scale: f64,
    body: String,
    variables: Map<String, Value>,
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
        let margin = if tall { 48.0 } else { 80.0 };
        Self {
            id,
            w,
            h,
            tall,
            margin,
            top: if tall { 220.0 } else { 155.0 },
            bottom: h - if tall { 260.0 } else { 130.0 },
            start,
            duration,
            theme,
            anchors: Vec::new(),
            width,
            height,
            scale,
            body: String::new(),
            variables: Map::new(),
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
        // A staggered child may start after its short-lived parent state ends.
        // Keep it hidden instead of placing the exit before the entrance.
        if until.is_some_and(|end| end <= at) {
            return String::new();
        }
        let d = self
            .theme
            .enter
            .min(450.0)
            .min((until.unwrap_or(self.duration) - at).max(1.0) * 0.45);
        let mut s=format!("<Animate prop=\"opacity\" from={{0}} to={{1}} start=\"{at}ms\" duration=\"{d}ms\" ease=\"outCubic\" />");
        if let Some(end) = until {
            s.push_str(&format!("<Animate prop=\"opacity\" from={{1}} to={{0}} start=\"{end}ms\" duration=\"1ms\" ease=\"linear\" />"));
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
        // Wrap sizing is deterministic and independent of installed font metrics. Native text owns shaping.
        let lines = ((label.chars().count() as f64 * size * 0.58 / w.max(1.0)).ceil()).max(1.0);
        let size = size.min(h / (lines * 1.25)).max(12.0).min(size);
        let a = self.animation(at, until);
        let lift = if self.theme.name == "signal" {
            10.0
        } else {
            18.0
        };
        self.body.push_str(&format!("<Text id={} text={{vars.{key}}} x={{{}}} y={{{}}} width={{{}}} height={{{}}} size={{{}}} fontFamily={} weight={} color={} align={} opacity={{0}}>{a}<Animate prop=\"y\" from={{{}}} to={{{}}} start=\"{at}ms\" duration=\"350ms\" ease=\"outCubic\" /></Text>",q(id),x*s,y*s,w*s,h*s,size*s,q(self.theme.font),q(&self.theme.weight.to_string()),q(color),q(align),(y+lift)*s,y*s));
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
        if kind == "none" {
            return;
        }
        let color = if kind == "blocked" {
            self.theme.warning
        } else {
            self.theme.line
        };
        let (a, b) = if kind == "back" { (b, a) } else { (a, b) };
        let mid = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
        let points = if self.theme.name == "blueprint" || self.theme.name == "daylight" {
            vec![a, (mid.0, a.1), (mid.0, b.1), b]
        } else {
            vec![a, (mid.0, mid.1 - 15.0), b]
        };
        if kind == "dashed" {
            for i in 0..8 {
                let f = i as f64 / 8.0;
                let g = f + 0.065;
                self.line(
                    &format!("{id}-{i}"),
                    &[
                        (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f),
                        (a.0 + (b.0 - a.0) * g, a.1 + (b.1 - a.1) * g),
                    ],
                    color,
                    2.0,
                    at,
                    until,
                );
            }
        } else {
            self.line(
                id,
                &points,
                color,
                if self.theme.name == "signal" {
                    3.0
                } else {
                    2.5
                },
                at,
                until,
            );
        }
        if matches!(kind, "arrow" | "back") {
            let angle = (b.1 - a.1).atan2(b.0 - a.0);
            let r = 11.0;
            self.line(
                &format!("{id}-arrow"),
                &[
                    (b.0 - r * (angle - 0.5).cos(), b.1 - r * (angle - 0.5).sin()),
                    b,
                    (b.0 - r * (angle + 0.5).cos(), b.1 - r * (angle + 0.5).sin()),
                ],
                color,
                2.5,
                at,
                until,
            );
        }
        if kind == "blocked" {
            self.icon(
                &format!("{id}-blocked"),
                "ban",
                mid.0 - 15.0,
                mid.1 - 15.0,
                30.0,
                color,
                at,
                until,
            );
        }
    }
    pub fn icon(
        &mut self,
        id: &str,
        name: &str,
        x: f64,
        y: f64,
        size: f64,
        color: &str,
        at: f64,
        until: Option<f64>,
    ) {
        // Small native line drawings, not font glyphs. Related catalogue icons share a semantic silhouette.
        let shape: &[&[(f64, f64)]] = match name {
            "check" | "verified" | "compliant" | "commit" => &[&[(3., 12.), (9., 18.), (21., 5.)]],
            "x" | "ban" | "delete" => &[&[(5., 5.), (19., 19.)], &[(19., 5.), (5., 19.)]],
            "warning" | "alert" | "incident" | "threat" => &[
                &[(12., 2.), (23., 22.), (1., 22.), (12., 2.)],
                &[(12., 8.), (12., 14.)],
                &[(12., 18.), (12., 19.)],
            ],
            "human" | "user" | "person" | "expert" | "manager" | "customer" | "approver"
            | "candidate" | "newcomer" | "leaver" | "partner" | "support" => &[
                &[
                    (8., 3.),
                    (16., 3.),
                    (17., 8.),
                    (12., 11.),
                    (7., 8.),
                    (8., 3.),
                ],
                &[
                    (3., 23.),
                    (4., 17.),
                    (8., 13.),
                    (16., 13.),
                    (20., 17.),
                    (21., 23.),
                ],
            ],
            "bot" | "agent" | "ai" | "chip" | "brain" => &[
                &[(3., 7.), (21., 7.), (21., 20.), (3., 20.), (3., 7.)],
                &[(12., 7.), (12., 2.)],
                &[(7., 12.), (9., 12.)],
                &[(15., 12.), (17., 12.)],
                &[(8., 17.), (16., 17.)],
            ],
            "machine" | "computer" | "laptop" | "monitor" | "app" | "dashboard" | "server" => &[
                &[(2., 3.), (22., 3.), (22., 17.), (2., 17.), (2., 3.)],
                &[(12., 17.), (12., 22.)],
                &[(6., 22.), (18., 22.)],
            ],
            "spark" | "bolt" | "energy" | "rocket" | "star" => &[&[
                (12., 1.),
                (15., 9.),
                (23., 12.),
                (15., 15.),
                (12., 23.),
                (9., 15.),
                (1., 12.),
                (9., 9.),
                (12., 1.),
            ]],
            "shield" | "lock" | "private" | "confidential" | "insurance" => &[&[
                (12., 2.),
                (22., 6.),
                (20., 16.),
                (12., 23.),
                (4., 16.),
                (2., 6.),
                (12., 2.),
            ]],
            "clock" | "timer" | "history" | "hourglass" | "loop" | "sync" => &[
                &[
                    (5., 3.),
                    (19., 3.),
                    (23., 12.),
                    (19., 21.),
                    (5., 21.),
                    (1., 12.),
                    (5., 3.),
                ],
                &[(12., 6.), (12., 12.), (17., 15.)],
            ],
            "search" | "eye" | "audit" | "assessment" | "test" => &[
                &[
                    (4., 3.),
                    (13., 3.),
                    (17., 8.),
                    (17., 14.),
                    (12., 18.),
                    (5., 17.),
                    (1., 12.),
                    (1., 7.),
                    (4., 3.),
                ],
                &[(16., 16.), (23., 23.)],
            ],
            "legal" | "scales" | "gavel" => &[
                &[(12., 2.), (12., 22.)],
                &[(3., 6.), (21., 6.)],
                &[(6., 6.), (2., 16.), (10., 16.), (6., 6.)],
                &[(18., 6.), (14., 16.), (22., 16.), (18., 6.)],
                &[(7., 22.), (17., 22.)],
            ],
            "chat" | "message" | "mail" | "feedback" | "forum" => &[
                &[
                    (2., 3.),
                    (22., 3.),
                    (22., 17.),
                    (10., 17.),
                    (4., 23.),
                    (4., 17.),
                    (2., 17.),
                    (2., 3.),
                ],
                &[(6., 8.), (18., 8.)],
                &[(6., 12.), (15., 12.)],
            ],
            "group" | "team" | "people" | "users" | "meeting" => &[
                &[
                    (9., 3.),
                    (15., 3.),
                    (16., 8.),
                    (12., 11.),
                    (8., 8.),
                    (9., 3.),
                ],
                &[(5., 22.), (6., 15.), (18., 15.), (19., 22.)],
                &[(2., 8.), (5., 11.), (2., 15.), (1., 21.)],
                &[(22., 8.), (19., 11.), (22., 15.), (23., 21.)],
            ],
            "question" | "info" => &[
                &[
                    (6., 5.),
                    (10., 2.),
                    (16., 3.),
                    (18., 7.),
                    (12., 13.),
                    (12., 16.),
                ],
                &[(12., 21.), (12., 22.)],
            ],
            "play" | "send" | "upload" | "download" | "arrow" => {
                &[&[(3., 3.), (22., 12.), (3., 21.), (3., 3.)]]
            }
            "signpost" => &[
                &[(12., 2.), (12., 23.)],
                &[(3., 4.), (19., 4.), (23., 8.), (19., 12.), (3., 12.), (3., 4.)],
                &[(21., 14.), (5., 14.), (1., 18.), (5., 22.), (21., 22.), (21., 14.)],
            ],
            "exchange" => &[
                &[(2., 7.), (22., 7.), (17., 2.)],
                &[(22., 7.), (17., 12.)],
                &[(22., 17.), (2., 17.), (7., 12.)],
                &[(2., 17.), (7., 22.)],
            ],
            "building" | "company" | "office" | "school" | "government" | "home" | "store"
            | "factory" => &[
                &[
                    (3., 23.),
                    (3., 8.),
                    (12., 1.),
                    (21., 8.),
                    (21., 23.),
                    (3., 23.),
                ],
                &[(9., 23.), (9., 16.), (15., 16.), (15., 23.)],
                &[(7., 10.), (17., 10.)],
            ],
            "chart" | "trend" | "decline" | "gauge" => &[
                &[(2., 2.), (2., 22.), (23., 22.)],
                &[(5., 17.), (10., 10.), (15., 13.), (22., 3.)],
            ],
            "health" | "firstaid" | "hospital" => &[&[
                (8., 2.),
                (16., 2.),
                (16., 8.),
                (22., 8.),
                (22., 16.),
                (16., 16.),
                (16., 22.),
                (8., 22.),
                (8., 16.),
                (2., 16.),
                (2., 8.),
                (8., 8.),
                (8., 2.),
            ]],
            "heart" | "wellbeing" | "dislike" => &[&[
                (12., 22.),
                (2., 12.),
                (2., 5.),
                (7., 2.),
                (12., 6.),
                (17., 2.),
                (22., 5.),
                (22., 12.),
                (12., 22.),
            ]],
            "gear" | "tool" | "admin" | "service" => &[
                &[
                    (8., 1.),
                    (16., 1.),
                    (17., 5.),
                    (22., 8.),
                    (22., 16.),
                    (18., 18.),
                    (16., 23.),
                    (8., 23.),
                    (6., 19.),
                    (1., 16.),
                    (1., 8.),
                    (6., 6.),
                    (8., 1.),
                ],
                &[
                    (9., 8.),
                    (15., 8.),
                    (17., 12.),
                    (15., 16.),
                    (9., 16.),
                    (7., 12.),
                    (9., 8.),
                ],
            ],
            "money" | "coins" | "wallet" | "savings" | "payout" | "card" | "calculator" => &[
                &[(1., 5.), (23., 5.), (23., 20.), (1., 20.), (1., 5.)],
                &[
                    (10., 8.),
                    (15., 8.),
                    (16., 12.),
                    (14., 17.),
                    (9., 17.),
                    (8., 12.),
                    (10., 8.),
                ],
                &[(4., 10.), (5., 10.)],
                &[(19., 15.), (20., 15.)],
            ],
            "book" | "newspaper" | "evidence" => &[
                &[
                    (1., 3.),
                    (8., 3.),
                    (12., 6.),
                    (16., 3.),
                    (23., 3.),
                    (23., 21.),
                    (16., 21.),
                    (12., 23.),
                    (8., 21.),
                    (1., 21.),
                    (1., 3.),
                ],
                &[(12., 6.), (12., 23.)],
            ],
            "box" | "archive" | "inbox" | "gift" => &[
                &[
                    (1., 6.),
                    (12., 1.),
                    (23., 6.),
                    (23., 20.),
                    (12., 24.),
                    (1., 20.),
                    (1., 6.),
                ],
                &[(1., 6.), (12., 11.), (23., 6.)],
                &[(12., 11.), (12., 24.)],
            ],
            "database" | "harddrive" => &[
                &[
                    (2., 5.),
                    (6., 2.),
                    (18., 2.),
                    (22., 5.),
                    (18., 8.),
                    (6., 8.),
                    (2., 5.),
                    (2., 19.),
                    (6., 22.),
                    (18., 22.),
                    (22., 19.),
                    (22., 5.),
                ],
                &[(2., 12.), (6., 15.), (18., 15.), (22., 12.)],
            ],
            "globe" | "translate" | "wifi" | "copyright" => &[
                &[
                    (6., 2.),
                    (18., 2.),
                    (23., 12.),
                    (18., 22.),
                    (6., 22.),
                    (1., 12.),
                    (6., 2.),
                ],
                &[(12., 1.), (7., 12.), (12., 23.), (17., 12.), (12., 1.)],
                &[(2., 8.), (22., 8.)],
                &[(2., 16.), (22., 16.)],
            ],
            "cloud" | "leave" => &[&[
                (4., 20.),
                (1., 16.),
                (2., 11.),
                (7., 10.),
                (8., 5.),
                (13., 2.),
                (19., 5.),
                (20., 11.),
                (23., 13.),
                (23., 18.),
                (20., 20.),
                (4., 20.),
            ]],
            "branch" | "workflow" | "orgchart" | "route" | "steps" | "share" => &[
                &[(8., 1.), (16., 1.), (16., 7.), (8., 7.), (8., 1.)],
                &[(12., 7.), (12., 12.), (3., 12.), (3., 17.)],
                &[(12., 12.), (21., 12.), (21., 17.)],
                &[(1., 17.), (6., 17.), (6., 23.), (1., 23.), (1., 17.)],
                &[(18., 17.), (23., 17.), (23., 23.), (18., 23.), (18., 17.)],
            ],
            "code" | "terminal" | "prompt" | "keyboard" => &[
                &[(8., 5.), (1., 12.), (8., 19.)],
                &[(16., 5.), (23., 12.), (16., 19.)],
                &[(14., 2.), (10., 22.)],
            ],
            "pen" | "pencil" | "design" => &[
                &[
                    (2., 16.),
                    (17., 1.),
                    (23., 7.),
                    (8., 22.),
                    (1., 23.),
                    (2., 16.),
                ],
                &[(15., 3.), (21., 9.)],
                &[(2., 16.), (8., 22.)],
            ],
            "link" | "plug" | "usb" | "copy" => &[
                &[(14., 5.), (18., 1.), (23., 6.), (14., 15.), (9., 10.)],
                &[(10., 19.), (6., 23.), (1., 18.), (10., 9.), (15., 14.)],
            ],
            "target" | "location" | "map" | "travel" => &[
                &[
                    (12., 23.),
                    (2., 10.),
                    (3., 4.),
                    (8., 1.),
                    (16., 1.),
                    (21., 4.),
                    (22., 10.),
                    (12., 23.),
                ],
                &[
                    (9., 6.),
                    (15., 6.),
                    (16., 11.),
                    (12., 14.),
                    (8., 11.),
                    (9., 6.),
                ],
            ],
            "flag" | "ticket" | "tag" => &[&[
                (3., 23.),
                (3., 1.),
                (22., 1.),
                (16., 7.),
                (22., 13.),
                (3., 13.),
            ]],
            "calendar" | "task" | "project" => &[
                &[(2., 4.), (22., 4.), (22., 22.), (2., 22.), (2., 4.)],
                &[(2., 9.), (22., 9.)],
                &[(7., 1.), (7., 6.)],
                &[(17., 1.), (17., 6.)],
                &[(6., 14.), (10., 14.)],
                &[(14., 18.), (18., 18.)],
            ],
            "plus" => &[&[(12., 2.), (12., 22.)], &[(2., 12.), (22., 12.)]],
            "percent" => &[
                &[(3., 22.), (21., 2.)],
                &[(2., 2.), (7., 2.), (7., 7.), (2., 7.), (2., 2.)],
                &[(17., 17.), (22., 17.), (22., 22.), (17., 22.), (17., 17.)],
            ],
            "flask" | "testtube" => &[
                &[(8., 1.), (16., 1.)],
                &[
                    (9., 1.),
                    (9., 9.),
                    (2., 21.),
                    (3., 23.),
                    (21., 23.),
                    (22., 21.),
                    (15., 9.),
                    (15., 1.),
                ],
                &[(6., 16.), (18., 16.)],
            ],
            "phone" | "call" => &[
                &[
                    (7., 1.),
                    (17., 1.),
                    (19., 3.),
                    (19., 21.),
                    (17., 23.),
                    (7., 23.),
                    (5., 21.),
                    (5., 3.),
                    (7., 1.),
                ],
                &[(10., 19.), (14., 19.)],
            ],
            "folder" => &[&[
                (1., 4.),
                (10., 4.),
                (13., 8.),
                (23., 8.),
                (23., 22.),
                (1., 22.),
                (1., 4.),
            ]],
            "filter" => &[&[
                (1., 3.),
                (23., 3.),
                (15., 12.),
                (15., 21.),
                (9., 23.),
                (9., 12.),
                (1., 3.),
            ]],
            "bell" | "announce" | "escalation" => &[
                &[
                    (3., 19.),
                    (6., 15.),
                    (6., 7.),
                    (10., 3.),
                    (14., 3.),
                    (18., 7.),
                    (18., 15.),
                    (21., 19.),
                    (3., 19.),
                ],
                &[(9., 22.), (15., 22.)],
            ],
            "bulb" | "idea" => &[
                &[
                    (8., 18.),
                    (8., 14.),
                    (3., 9.),
                    (4., 4.),
                    (9., 1.),
                    (15., 1.),
                    (20., 4.),
                    (21., 9.),
                    (16., 14.),
                    (16., 18.),
                    (8., 18.),
                ],
                &[(8., 21.), (16., 21.)],
            ],
            "car" | "truck" | "forklift" | "cart" => &[
                &[
                    (1., 17.),
                    (1., 10.),
                    (5., 5.),
                    (16., 5.),
                    (20., 10.),
                    (23., 10.),
                    (23., 18.),
                    (1., 18.),
                ],
                &[(1., 10.), (20., 10.)],
                &[(4., 18.), (4., 23.), (8., 23.), (8., 18.)],
                &[(16., 18.), (16., 23.), (20., 23.), (20., 18.)],
            ],
            "layers" | "slides" => &[
                &[(1., 7.), (12., 1.), (23., 7.), (12., 13.), (1., 7.)],
                &[(1., 12.), (12., 18.), (23., 12.)],
                &[(1., 17.), (12., 23.), (23., 17.)],
            ],
            "exit" | "undo" => &[
                &[(10., 2.), (2., 2.), (2., 22.), (10., 22.)],
                &[(7., 12.), (23., 12.), (17., 6.)],
                &[(23., 12.), (17., 18.)],
            ],
            _ => &[
                &[
                    (4., 2.),
                    (15., 2.),
                    (21., 8.),
                    (21., 23.),
                    (4., 23.),
                    (4., 2.),
                ],
                &[(15., 2.), (15., 8.), (21., 8.)],
                &[(8., 12.), (17., 12.)],
                &[(8., 16.), (17., 16.)],
                &[(8., 20.), (14., 20.)],
            ],
        };
        for (i, points) in shape.iter().enumerate() {
            let p: Vec<_> = points
                .iter()
                .map(|(a, b)| (x + a / 24.0 * size, y + b / 24.0 * size))
                .collect();
            self.line(&format!("{id}-stroke-{i}"), &p, color, 2.4, at, until);
        }
    }
    pub fn node(
        &mut self,
        id: &str,
        item: &Value,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        focus: bool,
        at: f64,
        until: Option<f64>,
    ) {
        let tone = item
            .get("tone")
            .and_then(Value::as_str)
            .unwrap_or("default");
        let (bg, ink) = if focus && item.get("tone").is_none() {
            (self.theme.focus, self.theme.focus_ink)
        } else {
            self.theme.tone(tone)
        };
        self.rect(&format!("{id}-surface"), x, y, w, h, bg, at, until);
        let icon = item
            .get("icon")
            .or_else(|| item.get("base"))
            .and_then(Value::as_str)
            .unwrap_or("");
        let label = item.get("label").and_then(Value::as_str).unwrap_or("");
        let icon_size = if label.is_empty() {
            (h * 0.62).min(110.0)
        } else {
            (h * 0.34).min(64.0)
        };
        if !icon.is_empty() {
            self.icon(
                &format!("{id}-icon"),
                icon,
                x + (w - icon_size) / 2.0,
                if label.is_empty() {
                    y + (h - icon_size) / 2.0
                } else {
                    y + 18.0
                },
                icon_size,
                ink,
                at + 70.0,
                until,
            );
        }
        let ty = if icon.is_empty() {
            y + 16.0
        } else {
            y + icon_size + 32.0
        };
        if !label.is_empty() {
            self.text(
                &format!("{id}-label"),
                label,
                x + 12.0,
                ty,
                w - 24.0,
                (y + h - ty - 12.0).max(28.0),
                28.0,
                ink,
                at + 100.0,
                until,
                "center",
            );
        }
        if let Some(m) = item
            .get("mod")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty() && *s != "none")
        {
            self.rect(
                &format!("{id}-badge-bg"),
                x + w - 30.0,
                y - 10.0,
                40.0,
                40.0,
                self.theme.accent,
                at,
                until,
            );
            self.icon(
                &format!("{id}-badge"),
                if m == "error" {
                    "x"
                } else if m == "alert" {
                    "warning"
                } else {
                    m
                },
                x + w - 24.0,
                y - 4.0,
                28.0,
                self.theme.surface,
                at,
                until,
            );
        }
        self.anchors.push((x + w / 2.0, y + h / 2.0));
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
    pub fn finish(self) -> Result<(String, Value), String> {
        let vars = self
            .variables
            .iter()
            .map(|(key, value)| format!("{key}:stringVar({value})"))
            .collect::<Vec<_>>()
            .join(",");
        // A span's Scene background is plan metadata, not a composited draw.
        // Paint it explicitly; caption spans must leave the underlying scene visible.
        let background = if self.id.contains("caption") {
            String::new()
        } else {
            format!("<Rect id=\"scene-background\" x={{0}} y={{0}} width={{{}}} height={{{}}} fill={} />", self.width, self.height, q(self.theme.bg))
        };
        let source=format!("export default defineSpanScene({{id:{},width:{},height:{},durationMs:{},variables:{{{vars}}},render({{vars}}){{return <Scene>{background}{}</Scene>;}}}});",q(&self.id),self.width,self.height,self.duration,self.body);
        Ok((source, Value::Object(self.variables)))
    }
}
