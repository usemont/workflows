#[path = "narrative_motion.rs"]
mod motion;
use crate::{
    grammar::{self, array, choice, fields, number, text},
    scene::Scene,
};
use serde_json::Value;
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
        (name, "", None, None, "default", "")
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
            v.get("label").and_then(Value::as_str).unwrap_or(""),
            v.get("brand").map(|_| text(v, "brand")).transpose()?,
            v.get("manner").and_then(Value::as_str),
            v.get("tone").and_then(Value::as_str).unwrap_or("default"),
            v.get("mod").and_then(Value::as_str).unwrap_or(""),
        )
    };
    grammar::icon(name)?;
    let name = motion::actor_icon(name);
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
    let document=v;let v=&document["action"];
    fields(v, &["subject", "steps"], "action")?;
    // ACT.plan defaults an omitted subject, kind, or line count before timing steps.
    let subject = &v["subject"];
    if !subject.is_null() {
        fields(subject, &["kind", "lines", "icon", "tone"], "action subject")?;
        grammar::optional_choice(subject, "kind", &["document", "message"])?;
        grammar::optional_icon(subject, "icon")?;
        grammar::optional_choice(subject, "tone", grammar::TONES)?;
    }
    let kind = subject["kind"].as_str().unwrap_or("document");
    let n = if subject.get("lines").is_some() {
        subject["lines"]
            .as_u64()
            .filter(|n| *n >= 2 && *n <= if kind == "message" { 3 } else { 8 })
            .ok_or("subject lines out of range")? as usize
    } else if kind == "message" { 3 } else { 6 };
    let steps = array(v, "steps", 1, 8)?;
    let mut actors = Vec::new();
    let first = &steps[0];
    let initial = first
        .get("by")
        .or_else(|| first.get("from"))
        .ok_or("first action step needs by (or from for pass)")?;
    let mut holder = put_actor(&mut actors, actor(initial)?);
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
            if idx != holder {
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
        if at < (if i == 0 { 0.0 } else { cursor - 450.0 }) {
            return Err("action steps overlap or start before preceding step completes".into());
        }
        let own = match verb {
            "strike" | "redact" => line_indices(step, "lines", n, 3, vec![2])?,
            "fake" => line_indices(step, "lines", n, 2, vec![2])?,
            "fill" => line_indices(step, "lines", n, 3, vec![n])?,
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
                if actors[holder].manner == "machine" {
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
        match verb {
            "pass" => {
                let target = put_actor(
                    &mut actors,
                    actor(step.get("to").ok_or("pass requires to")?)?,
                );
                if target == holder {
                    return Err("pass target must differ from current holder".into());
                }
                if step.get("blocked").is_some_and(|b| !b.is_boolean()) {
                    return Err("blocked must be a boolean".into());
                }
                if !step.get("blocked").and_then(Value::as_bool).unwrap_or(false) {
                    holder = target;
                }
            }
            "compare" => {
                if let Some(against) = step.get("against") {
                    actor(against)?;
                }
            }
            "weigh" => {
                grammar::optional_choice(step, "risk", &["high", "low"])?;
            }
            _ => {}
        }
        cursor = at + duration + 450.0;
    }
    if actors.len() > 6 {
        return Err("action supports at most six distinct actors".into());
    }
    motion::action(s, document, brands)
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
    let document=v;let v=&document["story"];
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
        if index == 0 && rel != "rule" {
            return Err("story starts with rule".into());
        }
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
                while board.len() > 3 {
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
                        .map(|t|t.max(merge+850.0))
                        .unwrap_or(merge + 1100.0);
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
    motion::story(s, document, brands)
}
