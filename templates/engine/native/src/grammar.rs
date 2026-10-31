use crate::scene::{Scene, Theme};
use serde_json::{json, Value};
use std::collections::BTreeMap;
pub(crate) const TEMPLATES: &[(&str, &[&str], usize, usize, usize, bool)] = &[
    ("title", &["text", "emblem", "editorial"], 0, 0, 0, false),
    ("statement", &["keypoint", "quote"], 0, 0, 0, false),
    (
        "illustration",
        &["picture", "stage", "panel"],
        0,
        0,
        0,
        false,
    ),
    ("media", &["frame", "split"], 0, 0, 0, false),
    (
        "list",
        &["bullets", "checklist", "warnings", "cards", "spotlight"],
        2,
        8,
        0,
        false,
    ),
    ("framework", &["canopy", "hub"], 3, 6, 0, true),
    ("comparison", &["versus", "lanes"], 2, 3, 6, false),
    ("categories", &["constellation", "shelves"], 2, 6, 3, false),
    (
        "tool-selection",
        &["switchboard", "mode-columns"],
        2,
        4,
        1,
        true,
    ),
    ("rule-exception", &["branch", "detour"], 2, 4, 0, true),
    (
        "principle",
        &["spotlight", "satellites", "chain"],
        1,
        4,
        0,
        true,
    ),
    ("sequence", &["track", "lifecycle"], 2, 6, 0, false),
    ("action", &["stage"], 0, 0, 0, false),
    ("story", &["morph"], 0, 0, 0, false),
];
pub(crate) const ACTIONS: &[&str] = &[
    "draft", "pass", "inspect", "approve", "reject", "strike", "redact", "compare", "fake", "fill",
    "restyle", "weigh",
];
pub(crate) const STORY_RELS: &[&str] = &[
    "rule", "example", "also", "except", "contrast", "because", "then", "state", "so",
];
pub(crate) const STATES: &[&str] = &["unknown", "risk", "blocked", "ok", "focus", "muted", "none"];
pub(crate) const RELS: &[&str] = &[
    "follows", "example", "also", "so", "because", "except", "contrast",
];
pub(crate) const TONES: &[&str] = &["default", "accent", "warning", "muted"];
pub(crate) const LINKS: &[&str] = &["arrow", "back", "line", "dashed", "blocked", "none"];
pub(crate) const MODS: &[&str] = &[
    "check", "alert", "error", "eye", "search", "question", "clock", "none", "",
];
const ICONS:&str="spark doc box search bolt check loop layers plug book pencil terminal globe map play branch flask shield trend clock flag eye code commit user chat database cloud lock key rocket star gear chart folder link mail bulb target warning calendar heart plus x human group customer manager expert newcomer leaver approver candidate partner support building office government factory store warehouse home orgchart school document contract policy checklist assessment report invoice certificate archive note spreadsheet slides confidential pen server machine phone app bot chip puzzle workflow tool filter upload download sync dashboard prompt brain unlock compliant threat fingerprint private audit bug incident escalation ban legal gavel stamp verified money coins wallet card percent calculator decline trophy cart truck tag call meeting announce bell share send inbox translate feedback dislike hourglass route steps info question wellbeing health leave accessible location ticket admin payout hospital wifi printer phishing copyright gift competitor project service test firstaid exit design evidence task travel hotel car coffee insurance savings delete copy extinguisher forklift firewall undo monitor keyboard headphones usb harddrive camera mic leaf recycle energy history gauge newspaper forum footprints rainbow testtube person users team people company computer laptop agent ai approval prohibited scales signpost exchange timer";

pub fn manifest() -> Value {
    let mut templates = serde_json::Map::new();
    for (name, variants, min, max, details, focus) in TEMPLATES {
        templates.insert((*name).into(),json!({"variants":variants,"minItems":min,"maxItems":max,"minDetails":usize::from(*name == "tool-selection"),"maxDetails":details,"focusRequired":focus}));
    }
    let themes: Vec<_> = ["daylight", "midnight", "blueprint", "signal"]
        .iter()
        .map(|t| Theme::get(t).unwrap().value())
        .collect();
    let mut value: Value = serde_json::from_str(include_str!("../../manifest.json"))
        .expect("published native template grammar");
    value["templates"] = Value::Object(templates);
    value["themes"] = json!(themes);
    value["icons"] = json!(ICONS.split_whitespace().collect::<Vec<_>>());
    value
}

pub(crate) fn fields(v: &Value, allowed: &[&str], where_: &str) -> Result<(), String> {
    let o = v
        .as_object()
        .ok_or_else(|| format!("{where_} must be an object"))?;
    for k in o.keys() {
        if !allowed.contains(&k.as_str()) {
            return Err(format!("{where_}: unsupported field {k:?}"));
        }
    }
    Ok(())
}
pub(crate) fn text<'a>(v: &'a Value, key: &str) -> Result<&'a str, String> {
    v.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| format!("{key} must be a nonempty string"))
}
pub(crate) fn number(v: &Value, key: &str) -> Result<f64, String> {
    v.get(key)
        .and_then(Value::as_f64)
        .filter(|n| n.is_finite())
        .ok_or_else(|| format!("{key} must be a finite number"))
}
pub(crate) fn array<'a>(
    v: &'a Value,
    key: &str,
    min: usize,
    max: usize,
) -> Result<&'a Vec<Value>, String> {
    v.get(key)
        .and_then(Value::as_array)
        .filter(|a| a.len() >= min && a.len() <= max)
        .ok_or_else(|| format!("{key} must contain {min}..{max} items"))
}
pub(crate) fn choice<'a>(v: &'a Value, key: &str, choices: &[&str]) -> Result<&'a str, String> {
    let s = text(v, key)?;
    if choices.contains(&s) {
        Ok(s)
    } else {
        Err(format!(
            "{key}: unsupported value {s:?}; expected {}",
            choices.join(", ")
        ))
    }
}
pub(crate) fn optional_choice(v: &Value, key: &str, choices: &[&str]) -> Result<(), String> {
    if v.get(key).is_some() {
        choice(v, key, choices)?;
    }
    Ok(())
}
pub(crate) fn icon(name: &str) -> Result<(), String> {
    if ICONS.split_whitespace().any(|i| i == name) {
        Ok(())
    } else {
        Err(format!("unknown icon {name:?}"))
    }
}
pub(crate) fn optional_icon(v: &Value, key: &str) -> Result<(), String> {
    if v.get(key).is_some() {
        icon(text(v, key)?)?;
    }
    Ok(())
}
pub(crate) fn optional_text(v: &Value, key: &str) -> Result<(), String> {
    if v.get(key).is_some() {
        text(v, key)?;
    }
    Ok(())
}
pub(crate) fn image_source(uri:&str)->Result<(),String>{
    if !uri.trim().starts_with("data:image/") {
        return Err("native images require embedded data:image URIs; resolve local workflow files with {\"$image\":\"relative/path.svg\"}; HTTP, file URLs, and bare paths are not fetched".into());
    }
    if uri.len()>16*1024*1024*3{return Err("image source exceeds the 16 MiB decoded source limit".into());}
    if !uri.contains(','){return Err("image data URI requires a payload separator".into());}
    Ok(())
}
pub(crate) fn brand_key(name:&str)->String{
    let mut key=String::with_capacity(name.len());let mut space=false;
    for c in name.chars().flat_map(char::to_lowercase){
        if c.is_ascii_alphanumeric(){if space&&!key.is_empty(){key.push(' ');}key.push(c);space=false;}else{space=true;}
    }
    key
}
pub(crate) fn brands(v: Option<&Value>) -> Result<BTreeMap<String, String>, String> {
    let mut b = BTreeMap::new();
    if let Some(v) = v {
        let o = v
            .as_object()
            .ok_or("brands must map brand ids to image URIs")?;
        for (k, v) in o {
            let uri = v
                .as_str()
                .filter(|s| !s.is_empty())
                .ok_or("brand URI must be a nonempty string")?;
            image_source(uri)?;
            b.insert(brand_key(k), uri.into());
        }
    }
    Ok(b)
}
pub(crate) fn scene(v: &Value) -> Result<(), String> {
    let name = text(v, "template")?;
    let config = TEMPLATES
        .iter()
        .find(|x| x.0 == name)
        .ok_or_else(|| format!("unknown template {name:?}"))?;
    choice(v, "variant", config.1)?;
    let mut allowed = vec![
        "id", "section", "start", "end", "template", "variant", "title", "callouts",
        "theme", "transition", "piece", "clock", "clockEnd",
    ];
    allowed.extend(match name {
        "title" => vec!["subtitle", "subtitleAt", "icon"],
        "statement" => vec!["text", "highlight", "highlightAt", "by", "label"],
        "illustration" => vec!["text", "picture", "highlight", "highlightAt"],
        "media" => vec!["image", "subtitle", "subtitleAt", "alt", "fit"],
        "list" => vec!["items", "takeaway", "intro"],
        "action" => vec!["action"],
        "story" => vec!["story"],
        _ => vec!["items", "focus"],
    });
    fields(v, &allowed, "scene")?;
    optional_text(v, "title")?;
    optional_text(v, "id")?;
    optional_text(v, "section")?;
    optional_choice(v, "theme", &["daylight", "midnight", "blueprint", "signal"])?;
    optional_choice(v, "piece", &["insert", "resume"])?;
    if v.get("clock").is_some() { number(v, "clock")?; }
    if v.get("clockEnd").is_some() { number(v, "clockEnd")?; }
    if v.get("subtitleAt").is_some() {
        text(v, "subtitle")?;
        let at = number(v, "subtitleAt")?;
        if at < number(v, "start")? || at >= number(v, "end")? {
            return Err("subtitleAt must fall within the scene".into());
        }
    }
    let spotlight = name == "list" && text(v, "variant")? == "spotlight";
    if let Some(intro) = v.get("intro") {
        if !spotlight {
            return Err("intro is only accepted by list spotlight".into());
        }
        fields(intro, &["picture", "label"], "spotlight intro")?;
        optional_text(intro, "label")?;
        picture(intro.get("picture").ok_or("intro picture is required")?)?;
    }
    match name {
        "title" => {
            text(v, "title")?;
            optional_text(v, "subtitle")?;
            optional_icon(v, "icon")?;
        }
        "statement" | "illustration" => {
            let t = if name == "illustration"
                && text(v, "variant")? == "picture"
                && v.get("text").is_none()
            {
                ""
            } else {
                text(v, "text")?
            };
            if let Some(h) = v.get("highlight") {
                let h = h
                    .as_str()
                    .filter(|h| !h.is_empty() && t.contains(h))
                    .ok_or("highlight must be a nonempty substring of text")?;
                let _ = h;
            }
            for k in ["by", "label"] {
                optional_text(v, k)?;
            }
            if v.get("highlightAt").is_some() {
                number(v, "highlightAt")?;
            }
            if name == "illustration" {
                picture(v.get("picture").ok_or("picture is required")?)?;
            }
        }
        "media" => {
            image_source(text(v, "image")?)?;
            optional_text(v, "subtitle")?;
            optional_text(v, "alt")?;
            optional_choice(v, "fit", &["contain", "cover"])?;
        }
        "action" | "story" => {}
        _ => {
            text(v, "title")?;
            let items = array(v, "items", config.2, config.3)?;
            for item in items {
                fields(
                    item,
                    &[
                        "label",
                        "icon",
                        "at",
                        "tone",
                        "details",
                        "detailsAt",
                        "rel",
                        "brand",
                        "goneAt",
                        "hide",
                        "picture",
                        "role",
                    ],
                    "item",
                )?;
                text(item, "label")?;
                optional_icon(item, "icon")?;
                optional_choice(item, "tone", TONES)?;
                optional_text(item, "brand")?;
                if let Some(spec) = item.get("picture") {
                    if !spotlight {
                        return Err("item picture is only accepted by list spotlight".into());
                    }
                    picture(spec)?;
                }
                if name != "list" {
                    icon(text(item, "icon")?)?;
                }
                if item.get("at").is_some() {
                    number(item, "at")?;
                }
                if name == "rule-exception" {
                    optional_choice(item, "role", &["default", "rule"])?;
                } else if item.get("role").is_some() {
                    return Err("role is only accepted by rule-exception items".into());
                }
                if config.4 > 0 {
                    let required = usize::from(name == "tool-selection");
                    let d = if item.get("details").is_some() || required > 0 {
                        array(item, "details", required, config.4)?.as_slice()
                    } else {
                        &[]
                    };
                    for detail in d {
                        if !detail.as_str().is_some_and(|s| !s.trim().is_empty()) {
                            return Err("details must be nonempty strings".into());
                        }
                    }
                    if let Some(times) = item.get("detailsAt") {
                        let times = times
                            .as_array()
                            .filter(|t| t.len() == d.len())
                            .ok_or("detailsAt must have one time per detail")?;
                        if times.iter().any(|t| !t.is_number()) {
                            return Err("detailsAt must be numbers".into());
                        }
                    }
                } else if item.get("details").is_some() || item.get("detailsAt").is_some() {
                    return Err(format!("{name} does not accept details"));
                }
                if name == "principle" {
                    optional_choice(item, "rel", RELS)?;
                } else if item.get("rel").is_some() {
                    return Err("rel is only accepted by principle items".into());
                }
            }
            if config.5 {
                let focus = v.get("focus").ok_or("focus is required")?;
                fields(focus, &["label", "icon", "at", "brand", "tone"], "focus")?;
                text(focus, "label")?;
                icon(text(focus, "icon")?)?;
                optional_text(focus, "brand")?;
                optional_choice(focus, "tone", TONES)?;
            } else if v.get("focus").is_some() {
                return Err(format!("{name} does not accept focus"));
            }
            if let Some(t) = v.get("takeaway") {
                fields(t, &["label", "at", "tone"], "takeaway")?;
                text(t, "label")?;
                if t.get("at").is_some() {
                    number(t, "at")?;
                }
                optional_choice(t, "tone", TONES)?;
            }
        }
    }
    Ok(())
}
pub(crate) fn picture(v: &Value) -> Result<(), String> {
    fields(
        v,
        &["layout", "items", "links", "outside", "steps", "beats", "callouts"],
        "picture",
    )?;
    let layout = choice(
        v,
        "layout",
        &["single", "row", "hub", "tree", "cycle", "boundary"],
    )?;
    let (min, max) = match layout {
        "single" => (1, 1),
        "row" => (2, 3),
        "hub" => (3, 5),
        "tree" => (3, 4),
        "cycle" => (3, 3),
        _ => (1, 2),
    };
    let items = array(v, "items", min, max)?;
    for item in items {
        picture_item(item)?;
    }
    if let Some(out) = v.get("outside") {
        if layout != "boundary" {
            return Err("outside requires boundary layout".into());
        }
        picture_item(out)?;
    }
    validate_links(v)?;
    if let Some(steps) = v.get("steps") {
        let steps = steps
            .as_array()
            .filter(|a| a.len() <= 4)
            .ok_or("picture accepts at most four steps")?;
        let mut last = f64::NEG_INFINITY;
        for step in steps {
            fields(step, &["at", "items", "links", "outside"], "picture step")?;
            let at = number(step, "at")?;
            if at <= last {
                return Err("picture step times must increase".into());
            }
            last = at;
            if let Some(i) = step.get("items") {
                let i = i
                    .as_array()
                    .filter(|a| a.len() == items.len())
                    .ok_or("picture step must preserve item slot count")?;
                for item in i {
                    picture_item(item)?;
                }
            }
            if let Some(out) = step.get("outside") {
                if layout != "boundary" {
                    return Err("outside requires boundary layout".into());
                }
                picture_item(out)?;
            }
            validate_links(step)?;
        }
    }
    if let Some(beats) = v.get("beats") {
        let beats = beats
            .as_array()
            .filter(|a| a.len() <= 12)
            .ok_or("picture accepts at most twelve beats")?;
        let mut previous = f64::NEG_INFINITY;
        let edge_count = match layout {
            "single" => 0,
            "row" => items.len() - 1,
            "hub" | "tree" => items.len() - 1,
            "cycle" => 3,
            "boundary" => 1,
            _ => 0,
        };
        for beat in beats {
            fields(beat, &["at", "items", "links"], "picture beat")?;
            let at = number(beat, "at")?;
            if at <= previous {
                return Err("picture beat times must increase".into());
            }
            previous = at;
            let mut count = 0;
            for (key, max) in [
                ("items", items.len() + usize::from(layout == "boundary")),
                ("links", edge_count),
            ] {
                if let Some(indices) = beat.get(key) {
                    let indices = indices
                        .as_array()
                        .ok_or("picture beat indices must be arrays")?;
                    let mut seen = std::collections::BTreeSet::new();
                    for index in indices {
                        let i = index
                            .as_u64()
                            .filter(|i| (*i as usize) < max)
                            .ok_or("picture beat index is out of range")?;
                        if !seen.insert(i) {
                            return Err("duplicate picture beat index".into());
                        }
                        count += 1;
                    }
                }
            }
            if count == 0 {
                return Err("picture beat must emphasize an item or link".into());
            }
        }
    }
    Ok(())
}
fn picture_item(v: &Value) -> Result<(), String> {
    if v.is_null() {
        return Ok(());
    }
    fields(v, &["base", "mod", "tone", "brand"], "picture item")?;
    icon(text(v, "base")?)?;
    optional_choice(v, "mod", MODS)?;
    optional_choice(v, "tone", TONES)?;
    optional_text(v, "brand")?;
    Ok(())
}
fn validate_links(v: &Value) -> Result<(), String> {
    if let Some(links) = v.get("links") {
        for link in links.as_array().ok_or("picture links must be an array")? {
            if !link.as_str().is_some_and(|s| LINKS.contains(&s)) {
                return Err("invalid picture link".into());
            }
        }
    }
    Ok(())
}
pub(crate) fn callouts(s: &mut Scene, v: &Value, prefix: &str, anchor_offset: usize) -> Result<(), String> {
    if let Some(c) = v.get("callouts") {
        if v["template"] == "action" {
            return Err("action does not accept callouts".into());
        }
        for (i, c) in c
            .as_array()
            .filter(|c| c.len() <= 8)
            .ok_or("at most eight callouts")?
            .iter()
            .enumerate()
        {
            fields(c, &["text", "at", "on", "tone", "end"], "callout")?;
            let label = text(c, "text")?;
            if label.chars().count() > 32 {
                return Err("callout text exceeds 32 characters".into());
            }
            optional_choice(c, "tone", TONES)?;
            let at = s.local(number(c, "at")?)?;
            let until = c
                .get("end")
                .map(|_| s.local_end(number(c, "end")?))
                .transpose()?;
            if until.is_some_and(|end| end <= at) {
                return Err("callout end must follow at".into());
            }
            let anchor = if c.get("on").is_some_and(|n| !n.is_null()) {
                let n = c["on"]
                    .as_u64()
                    .ok_or("callout on must be a nonnegative item index")?
                    as usize;
                *s.anchors
                    .get(anchor_offset..).and_then(|anchors| anchors.get(n))
                    .ok_or("callout target item does not exist")?
            } else {
                (s.w / 2.0, s.bottom)
            };
            let y = (anchor.1 - 100.0).max(s.top);
            let x = (anchor.0 - 150.0).clamp(s.margin, s.w - s.margin - 300.0);
            let (bg, ink) = s
                .theme
                .tone(c.get("tone").and_then(Value::as_str).unwrap_or("accent"));
            s.rect(
                &format!("{prefix}callout-{i}-surface"),
                x,
                y,
                300.0,
                64.0,
                bg,
                at,
                until,
            );
            s.text(
                &format!("{prefix}callout-{i}"),
                label,
                x + 12.0,
                y + 12.0,
                276.0,
                40.0,
                24.0,
                ink,
                at,
                until,
                "center",
            );
            s.edge(
                &format!("{prefix}callout-{i}-leader"),
                (x + 150.0, y + 64.0),
                anchor,
                "line",
                at,
                until,
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod timing_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn omitted_takeaway_timing_stays_owned_by_choreography() {
        let mut spec = json!({"template":"list","variant":"bullets","title":"Default timing",
            "start":0,"end":24,"items":[{"label":"First"},{"label":"Second"}],
            "takeaway":{"label":"Still responsible","tone":"warning"}});
        scene(&spec).unwrap();
        let mut output = Scene::new("defaults".into(),1280,720,24000.0,0.0,crate::scene::Theme::get("daylight").unwrap());
        let brands = std::collections::BTreeMap::new();
        crate::layout::render(&mut output,&spec,&brands).unwrap();
        let takeaway = output.transition_nodes.iter().find(|n| n["t"] == "takeaway").unwrap();
        let themes: Value = serde_json::from_str(include_str!("themes.json")).unwrap();
        assert_eq!(takeaway["a"],json!(0.95 * themes["daylight"]["motion"]["tempo"].as_f64().unwrap()));
        assert!(spec["takeaway"].get("at").is_none());
        spec["takeaway"]["at"] = json!("invalid");
        assert_eq!(scene(&spec).unwrap_err(),"at must be a finite number");
    }
}
