use crate::{
    grammar::{self, array, choice, fields, number, text},
    layout::branded_node,
    scene::Scene,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq)]
struct Actor {
    icon: String,
    label: String,
    brand: Option<String>,
    manner: String,
    tone: String,
    modifier: String,
}
fn actor(v: &Value) -> Result<Actor, String> {
    let (name, label, brand, manner, tone, modifier) = if let Some(name) = v.as_str() {
        (name, name, None, None, "default", "")
    } else {
        fields(
            v,
            &["icon", "label", "brand", "manner", "tone", "mod"],
            "actor",
        )?;
        grammar::optional_choice(v, "manner", &["machine", "human"])?;
        grammar::optional_choice(v, "tone", grammar::TONES)?;
        grammar::optional_choice(v, "mod", grammar::MODS)?;
        let name = text(v, "icon")?;
        (
            name,
            v.get("label").and_then(Value::as_str).unwrap_or(name),
            v.get("brand").map(|_| text(v, "brand")).transpose()?,
            v.get("manner").and_then(Value::as_str),
            v.get("tone").and_then(Value::as_str).unwrap_or("default"),
            v.get("mod").and_then(Value::as_str).unwrap_or(""),
        )
    };
    grammar::icon(name)?;
    let machine = [
        "bot", "spark", "chip", "app", "server", "machine", "cloud", "brain", "code", "terminal",
        "workflow", "prompt",
    ]
    .contains(&name);
    Ok(Actor {
        icon: name.into(),
        label: label.into(),
        brand: brand.map(Into::into),
        manner: manner
            .unwrap_or(if machine { "machine" } else { "human" })
            .into(),
        tone: tone.into(),
        modifier: modifier.into(),
    })
}
fn line_indices(
    v: &Value,
    key: &str,
    lines: usize,
    max: usize,
    default: Vec<usize>,
) -> Result<Vec<usize>, String> {
    let Some(v) = v.get(key) else {
        return Ok(default);
    };
    let values = v
        .as_array()
        .filter(|a| !a.is_empty() && a.len() <= max)
        .ok_or_else(|| format!("{key} requires 1..{max} line numbers"))?;
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for value in values {
        let n = value
            .as_u64()
            .filter(|n| *n >= 1 && *n <= lines as u64)
            .ok_or_else(|| format!("{key}: line number must be 1..{lines}"))?
            as usize;
        if !seen.insert(n) {
            return Err(format!("{key}: duplicate line {n}"));
        }
        out.push(n);
    }
    Ok(out)
}
#[derive(Clone, Copy, PartialEq)]
enum Ink {
    Empty,
    Written,
    Strike,
    Redact,
    Fake,
    Rough,
    Checked,
}
#[derive(Clone)]
struct ActionState {
    holder: usize,
    lines: Vec<Ink>,
    status: String,
    verb: String,
    other: Option<usize>,
    risk: String,
}
#[derive(Clone)]
struct ActionEvent {
    at: f64,
    state: ActionState,
}
fn put_actor(actors: &mut Vec<Actor>, a: Actor) -> usize {
    if let Some(i) = actors
        .iter()
        .position(|x| x.icon == a.icon && x.label == a.label)
    {
        i
    } else {
        actors.push(a);
        actors.len() - 1
    }
}
pub(crate) fn action(
    s: &mut Scene,
    v: &Value,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    fields(v, &["subject", "steps"], "action")?;
    let subject = v.get("subject").ok_or("action.subject required")?;
    fields(
        subject,
        &["kind", "lines", "icon", "tone"],
        "action subject",
    )?;
    let kind = choice(subject, "kind", &["document", "message"])?;
    let n = subject["lines"]
        .as_u64()
        .filter(|n| *n >= 2 && *n <= if kind == "message" { 3 } else { 8 })
        .ok_or("subject lines out of range")? as usize;
    grammar::optional_icon(subject, "icon")?;
    grammar::optional_choice(subject, "tone", grammar::TONES)?;
    let steps = array(v, "steps", 1, 8)?;
    let mut actors = Vec::new();
    let first = &steps[0];
    let initial = first
        .get("by")
        .or_else(|| first.get("from"))
        .ok_or("first action step needs by (or from for pass)")?;
    let holder = put_actor(&mut actors, actor(initial)?);
    let mut state = ActionState {
        holder,
        lines: vec![
            if text(first, "do")? == "draft" {
                Ink::Empty
            } else {
                Ink::Written
            };
            n
        ],
        status: String::new(),
        verb: String::new(),
        other: None,
        risk: String::new(),
    };
    let mut events = vec![ActionEvent {
        at: 250.0,
        state: state.clone(),
    }];
    let mut cursor = 900.0;
    for (i, step) in steps.iter().enumerate() {
        fields(
            step,
            &[
                "do", "by", "from", "to", "finds", "lines", "blocked", "against", "risk", "at",
            ],
            "action step",
        )?;
        let verb = choice(step, "do", grammar::ACTIONS)?;
        let allowed = match verb {
            "draft" => vec!["do", "by", "at"],
            "pass" => vec!["do", "by", "from", "to", "blocked", "at"],
            "inspect" => vec!["do", "by", "finds", "at"],
            "approve" | "reject" => vec!["do", "by", "at"],
            "strike" | "redact" | "fake" | "fill" | "restyle" => vec!["do", "by", "lines", "at"],
            "compare" => vec!["do", "by", "against", "finds", "at"],
            _ => vec!["do", "by", "risk", "at"],
        };
        fields(step, &allowed, "action step")?;
        if let Some(by) = step.get("by").or_else(|| step.get("from")) {
            let idx = put_actor(&mut actors, actor(by)?);
            if idx != state.holder {
                return Err(format!("action step {i} actor is not the current holder"));
            }
        }
        if verb == "draft" && i != 0 {
            return Err("draft must be the first action step".into());
        }
        let at = step
            .get("at")
            .map(|_| number(step, "at").map(|n| n * 1000.0))
            .transpose()?
            .unwrap_or(cursor);
        if at < (if i == 0 { 0.0 } else { cursor }) {
            return Err("action steps overlap or start before preceding step completes".into());
        }
        let own = match verb {
            "strike" | "redact" => line_indices(step, "lines", n, 3, vec![2])?,
            "fake" => line_indices(step, "lines", n, 2, vec![2])?,
            "fill" => line_indices(step, "lines", n, 3, (1..=n.min(3)).collect())?,
            "restyle" => line_indices(step, "lines", n, 4, vec![2])?,
            "inspect" | "compare" => line_indices(
                step,
                "finds",
                n,
                if verb == "compare" { 8 } else { n },
                Vec::new(),
            )?,
            _ => Vec::new(),
        };
        let duration = match verb {
            "draft" => {
                if actors[state.holder].manner == "machine" {
                    550.0 + 170.0 * n as f64
                } else {
                    700.0 + 500.0 * n as f64
                }
            }
            "pass" => {
                if step
                    .get("blocked")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                {
                    2000.0
                } else {
                    1400.0
                }
            }
            "inspect" => 600.0 + 360.0 * n as f64,
            "approve" | "reject" => 1300.0,
            "strike" | "redact" => 500.0 + 700.0 * own.len() as f64,
            "compare" => 900.0 + 420.0 * n as f64,
            "fake" => 900.0 + 1100.0 * own.len() as f64,
            "fill" => 500.0 + 750.0 * own.len() as f64,
            "restyle" => 500.0 + 600.0 * own.len() as f64,
            _ => 2400.0,
        };
        if at + duration > s.duration {
            return Err(format!(
                "action step {i} does not fit scene; needs end at {:.3}s",
                s.start + (at + duration) / 1000.0
            ));
        }
        if matches!(verb, "fill" | "restyle") {
            for line in &own {
                state.lines[*line - 1] = if verb == "fill" {
                    Ink::Empty
                } else {
                    Ink::Rough
                };
            }
            if i == 0 {
                events[0].state = state.clone();
            }
        }
        state.verb = verb.into();
        state.status = String::new();
        state.other = None;
        state.risk = String::new();
        events.push(ActionEvent {
            at,
            state: state.clone(),
        });
        match verb {
            "draft" => {
                for line in 0..n {
                    state.lines[line] = Ink::Written;
                    events.push(ActionEvent {
                        at: at
                            + if actors[state.holder].manner == "machine" {
                                550.0 + 170.0 * line as f64
                            } else {
                                700.0 + 500.0 * line as f64
                            },
                        state: state.clone(),
                    });
                }
            }
            "pass" => {
                let target = put_actor(
                    &mut actors,
                    actor(step.get("to").ok_or("pass requires to")?)?,
                );
                if target == state.holder {
                    return Err("pass target must differ from current holder".into());
                }
                if step.get("blocked").is_some_and(|b| !b.is_boolean()) {
                    return Err("blocked must be a boolean".into());
                }
                state.other = Some(target);
                if step
                    .get("blocked")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                {
                    state.status = "blocked".into();
                    events.push(ActionEvent {
                        at: at + 350.0,
                        state: state.clone(),
                    });
                } else {
                    state.holder = target;
                    events.push(ActionEvent {
                        at: at + 450.0,
                        state: state.clone(),
                    });
                }
            }
            "inspect" | "compare" => {
                if verb == "compare" {
                    let against = step
                        .get("against")
                        .ok_or("compare requires against actor")?;
                    state.other = Some(put_actor(&mut actors, actor(against)?));
                }
                for line in 0..n {
                    state.lines[line] = if own.contains(&(line + 1)) {
                        Ink::Rough
                    } else {
                        Ink::Checked
                    };
                    events.push(ActionEvent {
                        at: at
                            + 600.0
                            + (line as f64) * if verb == "compare" { 420.0 } else { 360.0 },
                        state: state.clone(),
                    });
                }
            }
            "approve" | "reject" => {
                state.status = if verb == "approve" { "ok" } else { "blocked" }.into();
                events.push(ActionEvent {
                    at: at + 550.0,
                    state: state.clone(),
                });
            }
            "strike" | "redact" | "fake" | "fill" | "restyle" => {
                for (j, line) in own.iter().enumerate() {
                    state.lines[*line - 1] = match verb {
                        "strike" => Ink::Strike,
                        "redact" => Ink::Redact,
                        "fake" => Ink::Fake,
                        _ => Ink::Written,
                    };
                    events.push(ActionEvent {
                        at: at + 500.0 + j as f64 * (duration - 500.0) / own.len() as f64,
                        state: state.clone(),
                    });
                }
                if verb == "fake" {
                    for line in &own {
                        state.lines[*line - 1] = Ink::Empty;
                    }
                    events.push(ActionEvent {
                        at: at + duration - 100.0,
                        state: state.clone(),
                    });
                }
            }
            "weigh" => {
                state.risk = choice(step, "risk", &["high", "low"])?.into();
                events.push(ActionEvent {
                    at: at + 600.0,
                    state: state.clone(),
                });
            }
            _ => unreachable!(),
        }
        cursor = at + duration + 450.0;
    }
    if actors.len() > 6 {
        return Err("action supports at most six distinct actors".into());
    }
    let count = actors.len();
    let bw = s.w - 2.0 * s.margin;
    let cy = if s.tall { s.top + 100.0 } else { s.top + 80.0 };
    let mut positions = Vec::new();
    for (i, a) in actors.iter().enumerate() {
        let x = if count == 1 {
            s.w / 2.0
        } else {
            s.margin + 80.0 + (bw - 160.0) * i as f64 / (count - 1) as f64
        };
        positions.push((x, cy));
        let mut item = json!({"label":a.label,"icon":a.icon,"tone":a.tone,"mod":a.modifier});
        if let Some(brand) = &a.brand {
            item["brand"] = json!(brand);
        }
        branded_node(
            s,
            &format!("actor-{i}"),
            &item,
            (x - 70.0, cy - 60.0, 140.0, 135.0),
            false,
            150.0,
            None,
            brands,
        )?;
    }
    events.sort_by(|a, b| a.at.total_cmp(&b.at));
    let dw = if s.tall { 250.0 } else { 220.0 };
    let dh = if kind == "message" { 180.0 } else { 290.0 };
    let docy = if s.tall { s.top + 310.0 } else { s.top + 220.0 };
    let mut previous = positions[events[0].state.holder].0;
    for (i, event) in events.iter().enumerate() {
        let st = &event.state;
        let until = events.get(i + 1).map(|e| e.at);
        let target = positions[st.holder].0;
        let x = (target - dw / 2.0).clamp(s.margin, s.w - s.margin - dw);
        let actualcx = x + dw / 2.0;
        if let Some(other) = st.other {
            let p = positions[other];
            s.edge(
                &format!("action-link-{i}"),
                (positions[st.holder].0, cy + 100.0),
                (p.0, p.1 + 100.0),
                if st.status == "blocked" {
                    "blocked"
                } else {
                    "arrow"
                },
                event.at,
                until,
            );
        }
        s.begin_move(
            &format!("subject-move-{i}"),
            previous - actualcx,
            0.0,
            event.at,
            850.0,
        );
        let (bg, ink) = s.theme.tone(
            subject
                .get("tone")
                .and_then(Value::as_str)
                .unwrap_or("default"),
        );
        s.rect(
            &format!("subject-{i}"),
            x,
            docy,
            dw,
            dh,
            bg,
            event.at,
            until,
        );
        if let Some(icon) = subject.get("icon").and_then(Value::as_str) {
            s.icon(
                &format!("subject-icon-{i}"),
                icon,
                x + dw - 38.0,
                docy + 10.0,
                24.0,
                ink,
                event.at,
                until,
            );
        }
        for (line, state) in st.lines.iter().enumerate() {
            let yy = docy + 40.0 + line as f64 * (dh - 65.0) / n as f64;
            let lx = x + 24.0;
            let len = (dw - 48.0) * if line % 3 == 2 { 0.68 } else { 1.0 };
            let id = format!("subject-{i}-line-{}", line + 1);
            match state {
                Ink::Empty => {}
                Ink::Redact => s.rect(&id, lx, yy, len, 14.0, ink, event.at, until),
                Ink::Fake => {
                    s.line(
                        &id,
                        &[(lx, yy), (lx + len, yy)],
                        s.theme.warning,
                        2.0,
                        event.at,
                        until,
                    );
                    s.icon(
                        &format!("{id}-fake"),
                        "question",
                        lx + len - 16.0,
                        yy - 10.0,
                        20.0,
                        s.theme.warning,
                        event.at,
                        until,
                    );
                }
                Ink::Rough => s.line(
                    &id,
                    &[
                        (lx, yy),
                        (lx + len * 0.2, yy - 5.0),
                        (lx + len * 0.4, yy + 5.0),
                        (lx + len * 0.6, yy - 5.0),
                        (lx + len * 0.8, yy + 4.0),
                        (lx + len, yy),
                    ],
                    s.theme.warning,
                    3.0,
                    event.at,
                    until,
                ),
                _ => {
                    s.line(&id, &[(lx, yy), (lx + len, yy)], ink, 4.0, event.at, until);
                    if *state == Ink::Strike {
                        s.line(
                            &format!("{id}-strike"),
                            &[(lx, yy - 6.0), (lx + len, yy + 6.0)],
                            s.theme.warning,
                            3.0,
                            event.at,
                            until,
                        );
                    }
                    if *state == Ink::Checked {
                        s.icon(
                            &format!("{id}-check"),
                            "check",
                            x + dw - 22.0,
                            yy - 8.0,
                            16.0,
                            s.theme.accent,
                            event.at,
                            until,
                        );
                    }
                }
            }
        }
        if !st.status.is_empty() {
            s.icon(
                &format!("subject-status-{i}"),
                if st.status == "ok" { "check" } else { "ban" },
                x + dw - 30.0,
                docy + dh - 30.0,
                54.0,
                if st.status == "ok" {
                    s.theme.accent
                } else {
                    s.theme.warning
                },
                event.at,
                until,
            );
        }
        s.end_move();
        if !st.risk.is_empty() {
            let high = st.risk == "high";
            s.line(
                &format!("risk-beam-{i}"),
                &[
                    (
                        s.w / 2.0 - 90.0,
                        s.bottom - 20.0 + if high { -25.0 } else { 25.0 },
                    ),
                    (
                        s.w / 2.0 + 90.0,
                        s.bottom - 20.0 + if high { 25.0 } else { -25.0 },
                    ),
                ],
                s.theme.warning,
                5.0,
                event.at,
                until,
            );
            s.icon(
                &format!("risk-pivot-{i}"),
                "legal",
                s.w / 2.0 - 25.0,
                s.bottom - 20.0,
                50.0,
                s.theme.warning,
                event.at,
                until,
            );
            s.text(
                &format!("risk-{i}"),
                if high { "High risk" } else { "Low risk" },
                s.w / 2.0 - 120.0,
                s.bottom + 40.0,
                240.0,
                45.0,
                24.0,
                s.theme.warning,
                event.at,
                until,
                "center",
            );
        }
        previous = actualcx;
    }
    Ok(())
}

#[derive(Clone, Default)]
struct Board {
    ids: Vec<String>,
    states: BTreeMap<String, String>,
    display: BTreeMap<String, String>,
    edges: Vec<(String, String, String)>,
    actor: Option<String>,
    actor_target: Option<String>,
    conclusion: Option<String>,
    sub: bool,
}
#[derive(Clone)]
enum Change {
    Board(Vec<String>, Vec<(String, String, String)>),
    Set(BTreeMap<String, String>),
    Inherit(String, String),
    Display(String, String),
    Actor(String),
    Hop(String),
    Conclusion(String),
    Sub,
}
#[derive(Clone)]
struct StoryEvent {
    at: f64,
    change: Change,
}
fn ids(v: &Value) -> Result<Vec<String>, String> {
    if let Some(s) = v.as_str() {
        if s.is_empty() {
            return Err("entity id must be nonempty".into());
        }
        return Ok(vec![s.into()]);
    }
    let a = v
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or("entity reference must be an id or nonempty id array")?;
    let mut out = Vec::new();
    for i in a {
        let id = i
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or("entity references must be strings")?
            .to_string();
        if out.contains(&id) {
            return Err("duplicate entity reference".into());
        }
        out.push(id);
    }
    Ok(out)
}
fn entity_kind<'a>(
    entities: &'a serde_json::Map<String, Value>,
    id: &str,
) -> Result<&'a str, String> {
    let e = entities
        .get(id)
        .ok_or_else(|| format!("unknown story entity {id:?}"))?;
    Ok(e.get("kind").and_then(Value::as_str).unwrap_or("thing"))
}
fn fresh(
    entities: &serde_json::Map<String, Value>,
    id: &str,
    seen: &mut BTreeSet<String>,
) -> Result<(), String> {
    if entity_kind(entities, id)? != "thing" {
        return Err(format!("{id} must be a thing"));
    }
    if !seen.insert(id.into()) {
        return Err(format!("{id} is not a fresh story entity"));
    }
    Ok(())
}
pub(crate) fn story(
    s: &mut Scene,
    v: &Value,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    fields(v, &["entities", "beats"], "story")?;
    let entities = v["entities"]
        .as_object()
        .filter(|e| !e.is_empty() && e.len() <= 14)
        .ok_or("story requires 1..14 entities")?;
    for (id, e) in entities {
        if id.is_empty() {
            return Err("story entity id may not be empty".into());
        }
        fields(
            e,
            &["label", "icon", "kind", "who", "sub", "brand"],
            "story entity",
        )?;
        if text(e, "label")?.chars().count() > 30 {
            return Err("entity label exceeds 30 characters".into());
        }
        grammar::optional_choice(e, "kind", &["thing", "actor", "conclusion"])?;
        if entity_kind(entities, id)? != "conclusion" {
            grammar::icon(text(e, "icon")?)?;
        } else {
            grammar::optional_icon(e, "icon")?;
        }
        for (key, max) in [("who", 26), ("sub", 40)] {
            if let Some(v) = e.get(key) {
                if !v.as_str().is_some_and(|s| s.chars().count() <= max) {
                    return Err(format!(
                        "entity {key} exceeds {max} characters or is not text"
                    ));
                }
            }
        }
    }
    for entity in entities.values() {
        grammar::optional_text(entity, "brand")?;
    }
    let beats = array(v, "beats", 1, 8)?;
    let mut board = Vec::<String>::new();
    let mut edges = Vec::<(String, String, String)>::new();
    let mut events = Vec::new();
    let mut seen = BTreeSet::new();
    let mut last = -1.0;
    let mut ended = false;
    for (index, beat) in beats.iter().enumerate() {
        fields(
            beat,
            &[
                "rel", "at", "on", "of", "to", "by", "fixes", "into", "set", "setAt", "subAt",
            ],
            "story beat",
        )?;
        let rel = choice(beat, "rel", grammar::STORY_RELS)?;
        let at = s.local(number(beat, "at")?)?;
        if at <= last {
            return Err("story beat times must strictly increase".into());
        }
        last = at;
        if ended {
            return Err("no story beats may follow a conclusion".into());
        }
        let mut allowed = vec!["rel", "at", "set", "setAt"];
        allowed.extend(match rel {
            "rule" => vec!["on"],
            "example" | "also" | "except" | "contrast" | "because" => vec!["of", "to"],
            "then" => vec!["to"],
            "state" => vec![],
            _ => vec!["by", "fixes", "into", "subAt"],
        });
        fields(beat, &allowed, "story beat")?;
        if beat.get("subAt").is_some() && beat.get("into").is_none() {
            return Err("subAt requires a conclusion into".into());
        }
        let mut born = at;
        match rel {
            "rule" => {
                if index != 0 {
                    return Err("rule may only be the first beat".into());
                }
                board = ids(beat.get("on").ok_or("rule requires on")?)?;
                for id in &board {
                    fresh(entities, id, &mut seen)?;
                }
            }
            "example" | "also" | "except" | "contrast" | "because" => {
                let source = text(beat, "of")?.to_string();
                let pos = board
                    .iter()
                    .position(|id| id == &source)
                    .ok_or("story of must name an on-board entity")?;
                let targets = ids(beat.get("to").ok_or("story relation requires to")?)?;
                if rel != "also" && targets.len() != 1 {
                    return Err(format!("{rel} requires one target"));
                }
                for target in &targets {
                    fresh(entities, target, &mut seen)?;
                }
                if rel == "example" {
                    let target = targets[0].clone();
                    board[pos] = target.clone();
                    for edge in &mut edges {
                        if edge.0 == source {
                            edge.0 = target.clone()
                        }
                        if edge.1 == source {
                            edge.1 = target.clone()
                        }
                    }
                    events.push(StoryEvent {
                        at,
                        change: Change::Inherit(target.clone(), source.clone()),
                    });
                    events.push(StoryEvent {
                        at,
                        change: Change::Display(target.clone(), source.clone()),
                    });
                    events.push(StoryEvent {
                        at: at + 310.0,
                        change: Change::Display(target.clone(), target),
                    });
                    born = at;
                } else {
                    for (j, target) in targets.iter().enumerate() {
                        let insert = if rel == "because" {
                            pos
                        } else if rel == "also" {
                            pos + 1 + j
                        } else {
                            board.len()
                        };
                        board.insert(insert, target.clone());
                        edges.push((source.clone(), target.clone(), rel.into()));
                        let t = at
                            + if rel == "also" {
                                450.0 + 420.0 * j as f64
                            } else {
                                350.0
                            };
                        events.push(StoryEvent {
                            at: t,
                            change: Change::Board(board.clone(), edges.clone()),
                        });
                        if rel == "also" || rel == "except" {
                            events.push(StoryEvent {
                                at: t,
                                change: Change::Inherit(target.clone(), source.clone()),
                            });
                            events.push(StoryEvent {
                                at: t,
                                change: Change::Display(target.clone(), source.clone()),
                            });
                            events.push(StoryEvent {
                                at: t + 430.0,
                                change: Change::Display(target.clone(), target.clone()),
                            });
                        }
                        born = t;
                    }
                }
            }
            "then" => {
                let target = text(beat, "to")?.to_string();
                fresh(entities, &target, &mut seen)?;
                if board.is_empty() {
                    return Err("then requires an existing board".into());
                }
                if let Some(prev) = board.last() {
                    edges.push((prev.clone(), target.clone(), "then".into()));
                    events.push(StoryEvent {
                        at,
                        change: Change::Set(BTreeMap::from([(prev.clone(), "muted".into())])),
                    });
                }
                board.push(target);
                while board.len() > 2 {
                    board.remove(0);
                }
                born = at + 300.0;
            }
            "state" => {
                if beat.get("set").is_none() {
                    return Err("state requires set".into());
                }
            }
            "so" => {
                if board.is_empty() {
                    return Err("so requires an existing board".into());
                }
                let fixes = beat.get("fixes").map(ids).transpose()?.unwrap_or_default();
                for id in &fixes {
                    if !board.contains(id) {
                        return Err("so fixes must reference on-board entities".into());
                    }
                }
                if let Some(by) = beat.get("by") {
                    let id = by.as_str().ok_or("so by must be an actor id")?;
                    if entity_kind(entities, id)? != "actor" {
                        return Err("so by must reference an actor".into());
                    }
                    events.push(StoryEvent {
                        at,
                        change: Change::Actor(id.into()),
                    });
                }
                for (j, id) in fixes.iter().enumerate() {
                    if beat.get("by").is_some() {
                        events.push(StoryEvent {
                            at: at + 300.0 * j as f64,
                            change: Change::Hop(id.clone()),
                        });
                    }
                    let mut set = BTreeMap::new();
                    set.insert(id.clone(), "ok".into());
                    events.push(StoryEvent {
                        at: at + 360.0 + 300.0 * j as f64,
                        change: Change::Set(set),
                    });
                }
                if let Some(into) = beat.get("into") {
                    let id = into.as_str().ok_or("so into must be a conclusion id")?;
                    if entity_kind(entities, id)? != "conclusion" {
                        return Err("so into must reference a conclusion".into());
                    }
                    let merge = (at
                        + 300.0
                        + 300.0 * fixes.len().saturating_sub(1) as f64
                        + if fixes.is_empty() { 100.0 } else { 500.0 })
                    .min(s.duration - 1700.0);
                    if merge < at {
                        return Err("story conclusion needs at least 1.7 seconds after beat".into());
                    }
                    events.push(StoryEvent {
                        at: merge + 400.0,
                        change: Change::Conclusion(id.into()),
                    });
                    let sub = beat
                        .get("subAt")
                        .map(|_| s.local(number(beat, "subAt")?))
                        .transpose()?
                        .unwrap_or(merge + 1100.0);
                    if sub < merge + 400.0 {
                        return Err("subAt must follow conclusion appearance".into());
                    }
                    events.push(StoryEvent {
                        at: sub,
                        change: Change::Sub,
                    });
                    ended = true;
                } else if fixes.is_empty() {
                    return Err("so requires fixes or into".into());
                }
            }
            _ => unreachable!(),
        }
        if board.len() > 5 {
            return Err("story board exceeds five things".into());
        }
        if !matches!(
            rel,
            "also" | "except" | "contrast" | "because" | "state" | "so"
        ) {
            events.push(StoryEvent {
                at: born,
                change: Change::Board(board.clone(), edges.clone()),
            });
        }
        if let Some(set) = beat.get("set") {
            let set = set
                .as_object()
                .filter(|o| !o.is_empty())
                .ok_or("set must be a nonempty entity -> state object")?;
            let mut change = BTreeMap::new();
            for (id, state) in set {
                if !board.contains(id) {
                    return Err(format!("set references off-board entity {id}"));
                }
                let state = state
                    .as_str()
                    .filter(|s| grammar::STATES.contains(s))
                    .ok_or("invalid story state")?;
                change.insert(id.clone(), state.into());
            }
            let time = beat
                .get("setAt")
                .map(|_| s.local(number(beat, "setAt")?))
                .transpose()?
                .unwrap_or(
                    at + if rel == "example" {
                        1200.0
                    } else if rel == "also" {
                        (born - at) + 850.0
                    } else {
                        800.0
                    },
                );
            if time < at {
                return Err("setAt may not precede its story beat".into());
            }
            events.push(StoryEvent {
                at: time,
                change: Change::Set(change),
            });
        } else if beat.get("setAt").is_some() {
            return Err("setAt requires set".into());
        }
    }
    for event in &events {
        if event.at < 0.0 || event.at >= s.duration {
            return Err("story event extends beyond scene; increase scene end".into());
        }
    }
    events.sort_by(|a, b| a.at.total_cmp(&b.at));
    let mut state = Board::default();
    let mut previous = BTreeMap::<String, (f64, f64)>::new();
    let mut actor_previous = None;
    for (event_index, event) in events.iter().enumerate() {
        match &event.change {
            Change::Board(ids, edges) => {
                state.ids = ids.clone();
                state.edges = edges.clone();
            }
            Change::Set(set) => state.states.extend(set.clone()),
            Change::Inherit(id, source) => {
                if let Some(value) = state.states.get(source).cloned() {
                    state.states.insert(id.clone(), value);
                }
            }
            Change::Display(id, source) => {
                state.display.insert(id.clone(), source.clone());
            }
            Change::Actor(id) => state.actor = Some(id.clone()),
            Change::Hop(id) => state.actor_target = Some(id.clone()),
            Change::Conclusion(id) => state.conclusion = Some(id.clone()),
            Change::Sub => state.sub = true,
        }
        let until = events.get(event_index + 1).map(|e| e.at);
        if until == Some(event.at) {
            continue;
        }
        let bw = s.w - 2.0 * s.margin;
        let bh = s.bottom - s.top;
        let mut positions = BTreeMap::new();
        if let Some(conclusion) = &state.conclusion {
            let entity = &entities[conclusion];
            let y = s.top + bh * 0.25;
            s.text(
                &format!("conclusion-{event_index}"),
                text(entity, "label")?,
                s.margin,
                y,
                bw,
                bh * 0.45,
                60.0,
                s.theme.ink,
                event.at,
                until,
                "center",
            );
            if state.sub {
                if let Some(sub) = entity.get("sub").and_then(Value::as_str) {
                    s.text(
                        &format!("conclusion-sub-{event_index}"),
                        sub,
                        s.margin,
                        y + bh * 0.46,
                        bw,
                        100.0,
                        30.0,
                        s.theme.muted,
                        event.at,
                        until,
                        "center",
                    );
                }
            }
            continue;
        }
        let n = state.ids.len();
        if n == 0 {
            continue;
        }
        let cw = if s.tall {
            (bw - 20.0) / 2.0
        } else {
            ((bw - 26.0 * (n - 1) as f64) / n as f64).min(240.0)
        };
        let ch = if s.tall { 190.0 } else { 230.0 };
        let rows = if s.tall { n.div_ceil(2) } else { 1 };
        let totalh = rows as f64 * ch + (rows - 1) as f64 * 40.0;
        let yy = s.top + (bh - totalh).max(0.0) * 0.45;
        for (i, id) in state.ids.iter().enumerate() {
            let row = if s.tall { i / 2 } else { 0 };
            let cols = if s.tall {
                if row == rows - 1 && n % 2 == 1 {
                    1
                } else {
                    2
                }
            } else {
                n
            };
            let col = if s.tall { i % 2 } else { i };
            let total = cols as f64 * cw + (cols - 1) as f64 * 26.0;
            let x = (s.w - total) / 2.0 + col as f64 * (cw + 26.0);
            let y = yy + row as f64 * (ch + 40.0);
            positions.insert(id.clone(), (x, y));
        }
        for (j, (a, b, rel)) in state.edges.iter().enumerate() {
            if let (Some(pa), Some(pb)) = (positions.get(a), positions.get(b)) {
                let kind = if rel == "except" {
                    "blocked"
                } else if rel == "contrast" {
                    "line"
                } else {
                    "arrow"
                };
                s.edge(
                    &format!("story-edge-{event_index}-{j}"),
                    (pa.0 + cw / 2.0, pa.1 + ch / 2.0),
                    (pb.0 + cw / 2.0, pb.1 + ch / 2.0),
                    kind,
                    event.at,
                    until,
                );
                let tag = if rel == "contrast" { "but" } else { rel };
                s.text(
                    &format!("story-relation-{event_index}-{j}"),
                    tag,
                    (pa.0 + pb.0) / 2.0 + cw / 2.0 - 60.0,
                    (pa.1 + pb.1) / 2.0 + ch / 2.0 - 18.0,
                    120.0,
                    35.0,
                    20.0,
                    s.theme.accent,
                    event.at,
                    until,
                    "center",
                );
            }
        }
        for id in &state.ids {
            let p = positions[id];
            let old = previous.get(id).copied().unwrap_or((p.0, p.1 + 30.0));
            let display = state.display.get(id).unwrap_or(id);
            let mut entity = entities[display].clone();
            match state.states.get(id).map(String::as_str).unwrap_or("none") {
                "risk" => {
                    entity["tone"] = json!("warning");
                    entity["mod"] = json!("alert");
                }
                "blocked" => {
                    entity["tone"] = json!("warning");
                    entity["mod"] = json!("error");
                }
                "ok" => {
                    entity["tone"] = json!("accent");
                    entity["mod"] = json!("check");
                }
                "focus" => {
                    entity["tone"] = json!("accent");
                }
                "muted" => {
                    entity["tone"] = json!("muted");
                }
                "unknown" => {
                    entity["mod"] = json!("question");
                }
                _ => {}
            }
            s.begin_move(
                &format!("story-move-{event_index}-{id}"),
                old.0 - p.0,
                old.1 - p.1,
                event.at,
                850.0,
            );
            branded_node(
                s,
                &format!("story-{event_index}-{id}"),
                &entity,
                (p.0, p.1, cw, ch),
                false,
                event.at,
                until,
                brands,
            )?;
            if let Some(who) = entity.get("who").and_then(Value::as_str) {
                s.text(
                    &format!("who-{event_index}-{id}"),
                    who,
                    p.0,
                    p.1 + ch + 6.0,
                    cw,
                    35.0,
                    19.0,
                    s.theme.muted,
                    event.at,
                    until,
                    "center",
                );
            }
            s.end_move();
        }
        if let Some(id) = &state.actor {
            let actor = &entities[id];
            let target = state
                .actor_target
                .as_ref()
                .and_then(|id| positions.get(id))
                .map(|p| (p.0 + cw / 2.0 - 60.0, p.1 + ch + 45.0))
                .unwrap_or((s.w / 2.0 - 60.0, s.bottom + 10.0));
            let old = actor_previous.unwrap_or((target.0, target.1 + 35.0));
            s.begin_move(
                &format!("story-actor-hop-{event_index}"),
                old.0 - target.0,
                old.1 - target.1,
                event.at,
                300.0,
            );
            branded_node(
                s,
                &format!("story-actor-{event_index}"),
                actor,
                (target.0, target.1, 120.0, 110.0),
                true,
                event.at,
                until,
                brands,
            )?;
            s.end_move();
            actor_previous = Some(target);
        }
        previous = positions;
    }
    Ok(())
}
