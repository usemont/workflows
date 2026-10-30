use crate::{
    grammar::{array, number, text},
    scene::Scene,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub(crate) fn render(
    s: &mut Scene,
    v: &Value,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    let template = text(v, "template")?;
    let variant = text(v, "variant")?;
    let bw = s.w - 2.0 * s.margin;
    let bh = s.bottom - s.top;
    if template != "title" && template != "statement" {
        if let Some(title) = v.get("title").and_then(Value::as_str) {
            s.text(
                "heading",
                title,
                s.margin,
                if s.tall { 96.0 } else { 48.0 },
                bw,
                90.0,
                44.0,
                s.theme.ink,
                120.0,
                None,
                "left",
            );
        }
    }
    match template {
        "title" => {
            let title = text(v, "title")?;
            let subtitle = v.get("subtitle").and_then(Value::as_str).unwrap_or("");
            let icon = v.get("icon").and_then(Value::as_str).unwrap_or("spark");
            let editorial = variant == "editorial" && !s.tall;
            let x = if editorial { s.w * 0.40 } else { s.margin };
            let tw = if editorial { s.w * 0.52 } else { bw };
            let y = if editorial {
                s.h * 0.32
            } else if variant == "text" {
                s.h * 0.30
            } else {
                s.h * 0.39
            };
            if variant != "text" {
                let size = if editorial { 180.0 } else { 120.0 };
                s.icon(
                    "title-emblem",
                    icon,
                    if editorial {
                        s.w * 0.15
                    } else {
                        (s.w - size) / 2.0
                    },
                    if editorial { s.h * 0.34 } else { y - 160.0 },
                    size,
                    s.theme.accent,
                    250.0,
                    None,
                );
            }
            s.text(
                "title",
                title,
                x,
                y,
                tw,
                if s.tall { 240.0 } else { 160.0 },
                if editorial { 64.0 } else { 78.0 },
                s.theme.ink,
                300.0,
                None,
                if editorial { "left" } else { "center" },
            );
            if !subtitle.is_empty() {
                s.text(
                    "subtitle",
                    subtitle,
                    x,
                    y + if s.tall { 250.0 } else { 174.0 },
                    tw,
                    120.0,
                    30.0,
                    s.theme.muted,
                    v.get("subtitleAt")
                        .map(|_| s.local(number(v, "subtitleAt")?))
                        .transpose()?
                        .unwrap_or(700.0),
                    None,
                    if editorial { "left" } else { "center" },
                );
            }
        }
        "statement" => {
            if let Some(label) = v
                .get("label")
                .or_else(|| v.get("title"))
                .and_then(Value::as_str)
            {
                s.text(
                    "eyebrow",
                    label,
                    s.margin,
                    s.h * 0.16,
                    bw,
                    54.0,
                    24.0,
                    s.theme.accent,
                    150.0,
                    None,
                    "center",
                );
            }
            if variant == "quote" {
                s.line(
                    "quote-mark-a",
                    &[
                        (s.margin + 8.0, s.h * 0.30),
                        (s.margin - 6.0, s.h * 0.34),
                        (s.margin + 20.0, s.h * 0.34),
                        (s.margin + 20.0, s.h * 0.31),
                    ],
                    s.theme.accent,
                    8.0,
                    200.0,
                    None,
                );
            }
            words(
                s,
                v,
                s.margin,
                s.h * 0.30,
                bw,
                if s.tall { 400.0 } else { 230.0 },
                if s.tall { 48.0 } else { 56.0 },
            )?;
            if let Some(by) = v.get("by").and_then(Value::as_str) {
                s.text(
                    "byline",
                    by,
                    s.margin,
                    s.h * 0.73,
                    bw,
                    70.0,
                    28.0,
                    s.theme.muted,
                    900.0,
                    None,
                    "center",
                );
            }
        }
        "media" => {
            let split = variant == "split";
            let (x, y, w, h) = if split && !s.tall {
                (s.margin, s.top, bw * 0.55, bh)
            } else {
                (s.margin, s.top, bw, if split { bh * 0.60 } else { bh })
            };
            s.image(
                "media",
                text(v, "image")?,
                x,
                y,
                w,
                h,
                v.get("fit").and_then(Value::as_str).unwrap_or("contain"),
                200.0,
                None,
            );
            if let Some(subtitle) = v.get("subtitle").and_then(Value::as_str) {
                let (tx, ty, tw, th) = if split && !s.tall {
                    (s.w * 0.63, s.top + 20.0, s.w * 0.30, bh - 40.0)
                } else if split {
                    (s.margin, s.top + bh * 0.65, bw, bh * 0.3)
                } else {
                    (s.margin, s.bottom + 18.0, bw, 74.0)
                };
                s.text(
                    "media-subtitle",
                    subtitle,
                    tx,
                    ty,
                    tw,
                    th,
                    36.0,
                    s.theme.ink,
                    v.get("subtitleAt")
                        .map(|_| s.local(number(v, "subtitleAt")?))
                        .transpose()?
                        .unwrap_or(550.0),
                    None,
                    "left",
                );
            }
        }
        "illustration" => {
            let (x, y, w, h) = if variant == "panel" && !s.tall {
                (s.margin, s.top, bw * 0.54, bh)
            } else {
                (
                    s.margin,
                    s.top,
                    bw,
                    if variant == "picture" { bh } else { bh * 0.65 },
                )
            };
            picture(s, &v["picture"], (x, y, w, h), "", brands)?;
            if v.get("text").is_some() {
                let (tx, ty, tw, th) = if variant == "panel" && !s.tall {
                    (s.w * 0.62, s.top, s.w * 0.31, bh)
                } else if variant == "picture" {
                    (s.margin, s.bottom + 12.0, bw, 100.0)
                } else {
                    (s.margin, s.top + bh * 0.70, bw, bh * 0.27)
                };
                words(
                    s,
                    v,
                    tx,
                    ty,
                    tw,
                    th,
                    if variant == "picture" { 25.0 } else { 36.0 },
                )?;
            }
        }
        "list" => list(s, v, brands)?,
        "action" => crate::narrative::action(s, &v["action"], brands)?,
        "story" => crate::narrative::story(s, &v["story"], brands)?,
        _ => structural(s, v, brands)?,
    }
    Ok(())
}

fn words(
    s: &mut Scene,
    v: &Value,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    size: f64,
) -> Result<(), String> {
    let content = text(v, "text")?;
    let highlight = v.get("highlight").and_then(Value::as_str).unwrap_or("");
    let count = content.split_whitespace().count().max(1);
    let estimated = (content.chars().count() as f64 * size * 0.57 / w)
        .ceil()
        .max(1.0);
    let size = size.min(h / (estimated * 1.4));
    let lh = size * 1.4;
    let mut xx = x;
    let mut yy = y;
    let high_at = v
        .get("highlightAt")
        .map(|_| s.local(number(v, "highlightAt")?))
        .transpose()?
        .unwrap_or(800.0 + (count as f64 * 75.0));
    let mut offset = 0;
    let high_begin = if highlight.is_empty() {
        usize::MAX
    } else {
        content.find(highlight).unwrap_or(usize::MAX)
    };
    let high_end = high_begin.saturating_add(highlight.len());
    for (i, word) in content.split_whitespace().enumerate() {
        let pos = content[offset..].find(word).unwrap_or(0) + offset;
        offset = pos + word.len();
        let ww = word.chars().count() as f64 * size * 0.59 + size * 0.3;
        if xx + ww > x + w && xx > x {
            xx = x;
            yy += lh;
        }
        let at = (250.0 + i as f64 * 65.0).min((s.duration - 600.0).max(0.0));
        let hi = pos < high_end && pos + word.len() > high_begin;
        if hi {
            let (bg, ink) = if s.theme.name == "signal" {
                (s.theme.ink, s.theme.bg)
            } else {
                (s.theme.tint, s.theme.accent)
            };
            let hh = if s.theme.name == "blueprint" { 4.0 } else { lh };
            let hy = if s.theme.name == "blueprint" {
                yy + lh - 4.0
            } else {
                yy
            };
            s.rect(
                &format!("highlight-{i}"),
                xx - 3.0,
                hy,
                ww,
                hh,
                bg,
                high_at.min(s.duration - 1.0),
                None,
            );
            s.text(
                &format!("word-{i}"),
                word,
                xx,
                yy,
                ww,
                lh,
                size,
                s.theme.ink,
                at,
                Some(high_at.min(s.duration - 1.0)),
                "left",
            );
            s.text(
                &format!("word-highlight-{i}"),
                word,
                xx,
                yy,
                ww,
                lh,
                size,
                ink,
                high_at.min(s.duration - 1.0),
                None,
                "left",
            );
        } else {
            s.text(
                &format!("word-{i}"),
                word,
                xx,
                yy,
                ww,
                lh,
                size,
                s.theme.ink,
                at,
                None,
                "left",
            );
        }
        xx += ww;
    }
    Ok(())
}

pub(crate) fn branded_node(
    s: &mut Scene,
    id: &str,
    item: &Value,
    frame: (f64, f64, f64, f64),
    focus: bool,
    at: f64,
    until: Option<f64>,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    let (x, y, w, h) = frame;
    s.node(id, item, x, y, w, h, focus, at, until);
    if let Some(brand) = item.get("brand").and_then(Value::as_str) {
        let uri = brands
            .get(brand)
            .ok_or_else(|| format!("unknown brand {brand:?}"))?;
        s.image(
            &format!("{id}-brand"),
            uri,
            x + w * 0.35,
            y + 12.0,
            w * 0.30,
            h * 0.32,
            "contain",
            at + 100.0,
            until,
        );
    }
    Ok(())
}
fn visibility(
    s: &Scene,
    item: &Value,
    at: f64,
    until: Option<f64>,
) -> Result<Vec<(f64, Option<f64>)>, String> {
    let end = item
        .get("goneAt")
        .map(|_| s.local_end(number(item, "goneAt")?))
        .transpose()?
        .or(until)
        .unwrap_or(s.duration);
    if end <= at {
        return Err("goneAt must follow item reveal".into());
    }
    let mut intervals = Vec::new();
    let mut cursor = at;
    if let Some(hide) = item.get("hide") {
        for pair in hide.as_array().ok_or("hide must be interval arrays")? {
            let pair = pair
                .as_array()
                .filter(|p| p.len() == 2)
                .ok_or("hide entries must be [start,end]")?;
            let a = s.local(pair[0].as_f64().ok_or("hide start must be a number")?)?;
            let b = s.local_end(pair[1].as_f64().ok_or("hide end must be a number")?)?;
            if a < cursor || b <= a || b > end {
                return Err("hide intervals must be ordered and lie in item lifetime".into());
            }
            if a > cursor {
                intervals.push((cursor, Some(a)));
            }
            cursor = b;
        }
    }
    if cursor < end {
        intervals.push((cursor, if end < s.duration { Some(end) } else { None }));
    }
    Ok(intervals)
}
fn list(s: &mut Scene, v: &Value, brands: &BTreeMap<String, String>) -> Result<(), String> {
    let variant = text(v, "variant")?;
    let items = array(v, "items", 2, 8)?;
    let n = items.len();
    let bw = s.w - 2.0 * s.margin;
    let bh = s.bottom - s.top;
    if variant == "spotlight" {
        let stage = if s.tall {
            (s.margin, s.top, bw, bh * 0.65)
        } else {
            (s.w * 0.45, s.top, s.w * 0.45, bh)
        };
        if let Some(intro) = v.get("intro") {
            let until = s.at(&items[0], 300.0)?;
            if until <= 0.0 {
                return Err("spotlight intro requires time before the first item".into());
            }
            spotlight_picture(s, &intro["picture"], intro.get("label").and_then(Value::as_str),
                stage, "spotlight-intro-", 0.0, Some(until), brands)?;
        }
        for (i, item) in items.iter().enumerate() {
            let at = s.at(item, 300.0 + i as f64 * 700.0)?;
            let until = items
                .get(i + 1)
                .map(|next| s.at(next, at + 700.0))
                .transpose()?;
            if until.is_some_and(|u| u <= at) {
                return Err("spotlight item times must increase".into());
            }
            if let Some(spec) = item.get("picture") {
                spotlight_picture(s, spec, Some(text(item, "label")?), stage,
                    &format!("spotlight-{i}-"), at, until, brands)?;
            } else {
                branded_node(
                    s,
                    &format!("spotlight-{i}"),
                    item,
                    stage,
                    true,
                    at,
                    until,
                    brands,
                )?;
            }
            let visible = if s.tall { 3 } else { 5 };
            let first = i.saturating_sub(visible - 1);
            for (j, entry) in items.iter().enumerate().skip(first).take(visible) {
                let rank = j - first;
                let (x, y, w, h) = if s.tall {
                    (s.margin, s.top + bh * 0.72 + rank as f64 * 48.0, bw, 42.0)
                } else {
                    (s.margin, s.top + rank as f64 * 74.0, bw * 0.33, 64.0)
                };
                s.rect(
                    &format!("rail-{i}-{j}"),
                    x,
                    y,
                    w,
                    h,
                    if i == j {
                        s.theme.tint
                    } else {
                        s.theme.surface
                    },
                    at,
                    until,
                );
                s.text(
                    &format!("rail-label-{i}-{j}"),
                    text(entry, "label")?,
                    x + 12.0,
                    y + 8.0,
                    w - 24.0,
                    h - 12.0,
                    24.0,
                    if i == j {
                        s.theme.accent
                    } else {
                        s.theme.muted
                    },
                    at,
                    until,
                    "left",
                );
            }
        }
    } else if variant == "cards" {
        let cols = if s.tall { 2 } else { n };
        let rows = n.div_ceil(cols);
        let gap = 22.0;
        let w = (bw - gap * (cols - 1) as f64) / cols as f64;
        let h = (bh - gap * (rows - 1) as f64) / rows as f64;
        for (i, item) in items.iter().enumerate() {
            let at = s.at(item, 400.0 + i as f64 * 240.0)?;
            for (part, (at, until)) in visibility(s, item, at, None)?.iter().enumerate() {
                branded_node(
                    s,
                    &format!("card-{i}-{part}"),
                    item,
                    (
                        s.margin + (i % cols) as f64 * (w + gap),
                        s.top + (i / cols) as f64 * (h + gap),
                        w,
                        h,
                    ),
                    false,
                    *at,
                    *until,
                    brands,
                )?;
            }
        }
    } else {
        let page_size = if n > 5 { 4 } else { n };
        let pages = n.div_ceil(page_size);
        for (i, item) in items.iter().enumerate() {
            let page = i / page_size;
            let default =
                300.0 + page as f64 * s.duration / pages as f64 + (i % page_size) as f64 * 200.0;
            let at = s.at(item, default)?;
            let page_start = page * page_size;
            let page_end = ((page + 1) * page_size).min(n);
            let count = page_end - page_start;
            let until = items
                .get(page_end)
                .map(|next| s.at(next, 300.0 + (page + 1) as f64 * s.duration / pages as f64))
                .transpose()?;
            let rh = (bh / count as f64).min(if s.tall { 130.0 } else { 95.0 });
            let y = s.top + (i % page_size) as f64 * (rh + 8.0);
            for (part, (at, until)) in visibility(s, item, at, until)?.iter().enumerate() {
                let color = if variant == "warnings" {
                    s.theme.warning
                } else {
                    s.theme.accent
                };
                let icon =
                    item.get("icon")
                        .and_then(Value::as_str)
                        .unwrap_or(if variant == "checklist" {
                            "check"
                        } else if variant == "warnings" {
                            "warning"
                        } else {
                            "plus"
                        });
                s.icon(
                    &format!("list-icon-{i}-{part}"),
                    icon,
                    s.margin,
                    y + 14.0,
                    32.0,
                    color,
                    *at,
                    *until,
                );
                s.text(
                    &format!("list-label-{i}-{part}"),
                    text(item, "label")?,
                    s.margin + 54.0,
                    y + 6.0,
                    bw - 64.0,
                    rh,
                    34.0,
                    s.theme.ink,
                    *at,
                    *until,
                    "left",
                );
            }
        }
    }
    if let Some(t) = v.get("takeaway") {
        let at = s.local(number(t, "at")?)?;
        let (bg, ink) = s
            .theme
            .tone(t.get("tone").and_then(Value::as_str).unwrap_or("accent"));
        s.rect(
            "takeaway-surface",
            s.margin,
            s.bottom + 22.0,
            bw,
            80.0,
            bg,
            at,
            None,
        );
        s.text(
            "takeaway",
            text(t, "label")?,
            s.margin + 18.0,
            s.bottom + 32.0,
            bw - 36.0,
            60.0,
            28.0,
            ink,
            at,
            None,
            "center",
        );
    }
    Ok(())
}

fn structural(s: &mut Scene, v: &Value, brands: &BTreeMap<String, String>) -> Result<(), String> {
    let family = text(v, "template")?;
    let variant = text(v, "variant")?;
    let items = v["items"].as_array().ok_or("items required")?;
    let n = items.len();
    let bw = s.w - 2.0 * s.margin;
    let bh = s.bottom - s.top;
    let mut frames = Vec::new();
    let mut focus = None;
    let mut edges: Vec<(usize, usize, String)> = Vec::new();
    let vertical = s.tall;
    match (family, variant) {
        ("framework", "hub") | ("principle", "satellites") | ("sequence", "lifecycle") => {
            let cycle = family == "sequence";
            let cw = if vertical { 170.0 } else { 190.0 };
            let ch = if vertical { 135.0 } else { 150.0 };
            let cx = s.w / 2.0;
            let cy = (s.top + s.bottom) / 2.0;
            if !cycle {
                focus = Some((cx - cw / 2.0, cy - ch / 2.0, cw, ch));
            }
            for i in 0..n {
                let angle =
                    -std::f64::consts::FRAC_PI_2 + i as f64 * std::f64::consts::TAU / n as f64;
                frames.push((
                    cx + angle.cos() * (bw - cw) * 0.46 - cw / 2.0,
                    cy + angle.sin() * (bh - ch) * 0.46 - ch / 2.0,
                    cw,
                    ch,
                ));
                if cycle {
                    edges.push((i, (i + 1) % n, "arrow".into()));
                }
            }
        }
        ("framework", "canopy") | ("tool-selection", "mode-columns") => {
            let fh = 130.0;
            focus = Some((s.w / 2.0 - 140.0, s.top, 280.0, fh));
            let cols = if vertical { 2 } else { n };
            let rows = n.div_ceil(cols);
            let gap = 18.0;
            let w = (bw - gap * (cols - 1) as f64) / cols as f64;
            let h = (bh - fh - 65.0 - gap * (rows - 1) as f64) / rows as f64;
            for i in 0..n {
                frames.push((
                    s.margin + (i % cols) as f64 * (w + gap),
                    s.top + fh + 65.0 + (i / cols) as f64 * (h + gap),
                    w,
                    h,
                ));
            }
        }
        ("principle", "chain") | ("sequence", "track") => {
            let count = n + usize::from(family == "principle");
            let gap = if vertical { 30.0 } else { 38.0 };
            let w = if vertical {
                bw * 0.74
            } else {
                (bw - gap * (count - 1) as f64) / count as f64
            };
            let h = if vertical {
                (bh - gap * (count - 1) as f64) / count as f64
            } else {
                bh * 0.65
            };
            for i in 0..count {
                let f = if vertical {
                    ((s.w - w) / 2.0, s.top + i as f64 * (h + gap), w, h)
                } else {
                    (s.margin + i as f64 * (w + gap), s.top + bh * 0.15, w, h)
                };
                if family == "principle" && i == 0 {
                    focus = Some(f)
                } else {
                    frames.push(f)
                }
            }
            for i in 1..n {
                edges.push((i - 1, i, "arrow".into()));
            }
        }
        ("rule-exception", "detour") => {
            if vertical {
                focus = Some((s.margin, s.top, bw * 0.65, 130.0));
                frames.push((s.margin, s.bottom - 140.0, bw * 0.65, 140.0));
                for i in 1..n {
                    frames.push((
                        s.w * 0.42,
                        s.top + 175.0 + (i - 1) as f64 * (bh - 350.0) / (n - 1) as f64,
                        bw * 0.58,
                        (bh - 370.0) / (n - 1) as f64,
                    ));
                }
            } else {
                focus = Some((s.margin, s.top + bh * 0.35, bw * 0.25, 150.0));
                frames.push((
                    s.w - s.margin - bw * 0.25,
                    s.top + bh * 0.35,
                    bw * 0.25,
                    150.0,
                ));
                for i in 1..n {
                    frames.push((
                        s.w / 2.0 - bw * 0.13,
                        s.top + (i - 1) as f64 * (bh - 130.0) / (n - 1).max(1) as f64,
                        bw * 0.26,
                        130.0,
                    ));
                }
            }
            for i in 1..n {
                edges.push((i, 0, "dashed".into()));
            }
        }
        ("comparison", "versus") | ("categories", "constellation") => {
            let cols = if vertical {
                if family == "comparison" {
                    1
                } else {
                    2
                }
            } else {
                n.min(3)
            };
            let rows = n.div_ceil(cols);
            let gap = 24.0;
            let w = (bw - gap * (cols - 1) as f64) / cols as f64;
            let h = (bh - gap * (rows - 1) as f64) / rows as f64;
            for i in 0..n {
                frames.push((
                    s.margin + (i % cols) as f64 * (w + gap),
                    s.top + (i / cols) as f64 * (h + gap),
                    w,
                    h,
                ));
            }
            if family == "categories" {
                for i in 1..n {
                    edges.push((i - 1, i, "dashed".into()));
                }
            }
        }
        _ => {
            let has_focus = v.get("focus").is_some();
            let left = if has_focus && !vertical {
                s.w * 0.42
            } else {
                s.margin
            };
            let w = s.w - s.margin - left;
            let top = if has_focus && vertical {
                s.top + 170.0
            } else {
                s.top
            };
            let h = (s.bottom - top - 16.0 * (n - 1) as f64) / n as f64;
            if has_focus {
                focus = Some(if vertical {
                    (s.w / 2.0 - 140.0, s.top, 280.0, 130.0)
                } else {
                    (s.margin, s.top + bh / 2.0 - 90.0, bw * 0.28, 180.0)
                });
            }
            for i in 0..n {
                frames.push((left, top + i as f64 * (h + 16.0), w, h));
            }
        }
    }
    let times: Vec<_> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            s.at(
                item,
                650.0 + i as f64 * if family == "sequence" { 340.0 } else { 160.0 },
            )
        })
        .collect::<Result<_, _>>()?;
    let focus_time = v
        .get("focus")
        .map(|f| s.at(f, 300.0))
        .transpose()?
        .unwrap_or(300.0);
    let center = |f: (f64, f64, f64, f64)| (f.0 + f.2 / 2.0, f.1 + f.3 / 2.0);
    if let Some(f) = focus {
        for i in 0..n {
            let relation = items[i]
                .get("rel")
                .and_then(Value::as_str)
                .unwrap_or("follows");
            let kind = if family == "rule-exception" && i > 0 || relation == "except" {
                "blocked"
            } else if relation == "because" || relation == "contrast" {
                "line"
            } else {
                "arrow"
            };
            if variant == "chain" && i > 0 {
                continue;
            }
            s.edge(
                &format!("focus-edge-{i}"),
                center(f),
                center(frames[i]),
                kind,
                times[i].max(focus_time) + 100.0,
                None,
            );
            if family == "principle" {
                let c = center(frames[i]);
                let tag = match relation {
                    "example" => "e.g.",
                    "contrast" => "but",
                    "follows" => "",
                    r => r,
                };
                if !tag.is_empty() {
                    s.text(
                        &format!("relation-{i}"),
                        tag,
                        (center(f).0 + c.0) / 2.0 - 55.0,
                        (center(f).1 + c.1) / 2.0 - 15.0,
                        110.0,
                        30.0,
                        18.0,
                        s.theme.accent,
                        times[i] + 120.0,
                        None,
                        "center",
                    );
                }
            }
        }
    }
    for (a, b, kind) in edges {
        s.edge(
            &format!("edge-{a}-{b}"),
            center(frames[a]),
            center(frames[b]),
            &kind,
            times[a].max(times[b]) + 100.0,
            None,
        );
    }
    if let Some(f) = focus {
        branded_node(s, "focus", &v["focus"], f, true, focus_time, None, brands)?;
        s.anchors.clear();
    }
    for (i, item) in items.iter().enumerate() {
        // Colour follows the narrated meaning, not rule/exception position.
        let frame = frames[i];
        let at = times[i];
        let detail_count = item
            .get("details")
            .and_then(Value::as_array)
            .map_or(0, Vec::len);
        for (part, (at, until)) in visibility(s, item, at, None)?.iter().enumerate() {
            if detail_count == 0 {
                branded_node(
                    s,
                    &format!("item-{i}-{part}"),
                    item,
                    frame,
                    family == "rule-exception" && i == 0,
                    *at,
                    *until,
                    brands,
                )?;
            } else {
                let (x, y, w, h) = frame;
                let row = matches!(variant, "lanes" | "shelves" | "switchboard");
                let head = if row && !vertical {
                    (x, y, w * 0.36, h)
                } else {
                    (x, y, w, h * 0.43)
                };
                branded_node(
                    s,
                    &format!("item-{i}-{part}"),
                    item,
                    head,
                    false,
                    *at,
                    *until,
                    brands,
                )?;
                let (dx, dy, dw, dh) = if row && !vertical {
                    (x + w * 0.40, y, w * 0.60, h)
                } else {
                    (x + 10.0, y + h * 0.46, w - 20.0, h * 0.54)
                };
                let details = item["details"].as_array().unwrap();
                for (j, detail) in details.iter().enumerate() {
                    let dt = item
                        .get("detailsAt")
                        .and_then(Value::as_array)
                        .and_then(|a| a.get(j))
                        .map(|t| s.local(t.as_f64().ok_or("detailsAt must be numbers")?))
                        .transpose()?
                        .unwrap_or(*at + 420.0 + j as f64 * 90.0);
                    let rh = dh / detail_count as f64;
                    s.text(
                        &format!("detail-{i}-{part}-{j}"),
                        detail.as_str().unwrap(),
                        dx,
                        dy + j as f64 * rh,
                        dw,
                        rh - 3.0,
                        if family == "comparison" { 25.0 } else { 22.0 },
                        s.theme.muted,
                        dt,
                        *until,
                        "left",
                    );
                }
            }
        }
    }
    Ok(())
}

fn spotlight_picture(
    s: &mut Scene,
    spec: &Value,
    label: Option<&str>,
    frame: (f64, f64, f64, f64),
    prefix: &str,
    at: f64,
    until: Option<f64>,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    let (x, y, w, h) = frame;
    s.begin_window(&format!("{prefix}stage"), at, until);
    let label_height = if label.is_some() { 80.0 } else { 0.0 };
    picture(s, spec, (x, y, w, h - label_height), prefix, brands)?;
    if let Some(label) = label {
        s.text(&format!("{prefix}label"), label, x, y + h - label_height,
            w, label_height, 30.0, s.theme.ink, at, until, "center");
    }
    s.end_move();
    Ok(())
}

fn picture(
    s: &mut Scene,
    spec: &Value,
    frame: (f64, f64, f64, f64),
    prefix: &str,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    let (x, y, w, h) = frame;
    let layout = text(spec, "layout")?;
    let n = spec["items"].as_array().unwrap().len();
    let transpose = s.tall && matches!(layout, "single" | "row" | "hub" | "boundary");
    let mut positions: Vec<(f64, f64, f64)> = match layout {
        "single" => vec![(320., 200., 180.)],
        "row" => (0..n)
            .map(|i| {
                (
                    if n == 2 {
                        180.0 + 280.0 * i as f64
                    } else {
                        80.0 + 240.0 * i as f64
                    },
                    200.,
                    if n == 2 { 150. } else { 116. },
                )
            })
            .collect(),
        "hub" => {
            let mut p = vec![(320., 200., 140.)];
            for i in 1..n {
                let a = (i - 1) as f64 * std::f64::consts::TAU / (n - 1) as f64;
                p.push((320. + 235. * a.cos(), 200. + 140. * a.sin(), 104.));
            }
            p
        }
        "tree" => {
            let mut p = vec![(320., 70., 116.)];
            for i in 1..n {
                p.push((
                    if n == 3 {
                        190. + 260. * (i - 1) as f64
                    } else {
                        100. + 220. * (i - 1) as f64
                    },
                    320.,
                    108.,
                ));
            }
            p
        }
        "cycle" => vec![(320., 70., 116.), (520., 320., 116.), (120., 320., 116.)],
        _ => {
            let mut p = if n == 1 {
                vec![(180., 200., 140.)]
            } else {
                vec![(100., 200., 112.), (260., 200., 112.)]
            };
            p.push((570., 200., 116.));
            p
        }
    };
    if transpose {
        for p in &mut positions {
            std::mem::swap(&mut p.0, &mut p.1);
        }
    }
    let pw = if transpose { 400.0 } else { 640.0 };
    let ph = if transpose { 640.0 } else { 400.0 };
    let scale = (w / pw).min(h / ph);
    let ox = x + (w - pw * scale) / 2.0;
    let oy = y + (h - ph * scale) / 2.0;
    let anchor_start = s.anchors.len();
    let mut states = vec![(
        0.0,
        spec["items"].clone(),
        spec.get("links").cloned().unwrap_or(json!([])),
        spec.get("outside").cloned().unwrap_or(Value::Null),
    )];
    if let Some(steps) = spec.get("steps").and_then(Value::as_array) {
        for step in steps {
            let prev = states.last().unwrap();
            states.push((
                s.local(number(step, "at")?)?,
                step.get("items").cloned().unwrap_or_else(|| prev.1.clone()),
                step.get("links").cloned().unwrap_or_else(|| prev.2.clone()),
                step.get("outside")
                    .cloned()
                    .unwrap_or_else(|| prev.3.clone()),
            ));
        }
    }
    for k in 0..states.len() {
        let (at, items, links, outside) = &states[k];
        let until = states.get(k + 1).map(|s| s.0);
        let items = items.as_array().unwrap();
        let mut pairs = Vec::new();
        match layout {
            "row" => {
                for i in 1..n {
                    pairs.push((i - 1, i));
                }
            }
            "hub" | "tree" => {
                for i in 1..n {
                    pairs.push((0, i));
                }
            }
            "cycle" => {
                for i in 0..n {
                    pairs.push((i, (i + 1) % n));
                }
            }
            "boundary" => {
                let (fx, fy, fw, fh) = if transpose {
                    (84., 16., 232., 328.)
                } else {
                    (16., 84., 328., 232.)
                };
                s.rect(
                    &format!("{prefix}boundary-{k}"),
                    ox + fx * scale,
                    oy + fy * scale,
                    fw * scale,
                    fh * scale,
                    s.theme.tint,
                    *at,
                    until,
                );
                if !outside.is_null() {
                    pairs.push((n, 0));
                }
            }
            _ => {}
        }
        if links.as_array().is_some_and(|l| l.len() > pairs.len()) {
            return Err("picture has more links than its layout edges".into());
        }
        for (e, (a, b)) in pairs.iter().enumerate() {
            let visible = |i: usize| {
                if i == n {
                    !outside.is_null()
                } else {
                    !items[i].is_null()
                }
            };
            if !visible(*a) || !visible(*b) {
                continue;
            }
            let kind = links.get(e).and_then(Value::as_str).unwrap_or("arrow");
            let pa = positions[*a];
            let pb = positions[*b];
            s.edge(
                &format!("{prefix}picture-edge-{k}-{e}"),
                (ox + pa.0 * scale, oy + pa.1 * scale),
                (ox + pb.0 * scale, oy + pb.1 * scale),
                kind,
                *at + 100.0,
                until,
            );
        }
        for (i, item) in items
            .iter()
            .chain(if layout == "boundary" {
                Some(outside)
            } else {
                None
            })
            .enumerate()
        {
            if item.is_null() {
                continue;
            }
            let p = positions[i];
            branded_node(
                s,
                &format!("{prefix}picture-{k}-{i}"),
                item,
                (
                    ox + (p.0 - p.2 / 2.0) * scale,
                    oy + (p.1 - p.2 / 2.0) * scale,
                    p.2 * scale,
                    p.2 * scale,
                ),
                false,
                *at,
                until,
                brands,
            )?;
        }
        if let Some(beats) = spec.get("beats").and_then(Value::as_array) {
            for (j, beat) in beats.iter().enumerate() {
                let time = s.local(number(beat, "at")?)?;
                if time < *at || until.is_some_and(|end| time >= end) {
                    continue;
                }
                let end = Some((time + 1100.0).min(until.unwrap_or(s.duration)));
                if let Some(indices) = beat.get("items").and_then(Value::as_array) {
                    for index in indices {
                        let i = index.as_u64().unwrap() as usize;
                        let item = if i == n { outside } else { &items[i] };
                        if item.is_null() {
                            return Err("picture beat emphasizes an empty item slot".into());
                        }
                        let p = positions[i];
                        let px = ox + (p.0 - p.2 / 2.0) * scale - 8.0;
                        let py = oy + (p.1 - p.2 / 2.0) * scale - 8.0;
                        let size = p.2 * scale + 16.0;
                        s.line(
                            &format!("{prefix}picture-beat-{k}-{j}-item-{i}"),
                            &[
                                (px, py),
                                (px + size, py),
                                (px + size, py + size),
                                (px, py + size),
                                (px, py),
                            ],
                            s.theme.accent,
                            5.0,
                            time,
                            end,
                        );
                    }
                }
                if let Some(indices) = beat.get("links").and_then(Value::as_array) {
                    for index in indices {
                        let i = index.as_u64().unwrap() as usize;
                        let &(a, b) = pairs
                            .get(i)
                            .ok_or("picture beat emphasizes an absent link")?;
                        if links.get(i).and_then(Value::as_str) == Some("none") {
                            return Err("picture beat emphasizes a hidden link".into());
                        }
                        if [a, b].iter().any(|&i| {
                            if i == n {
                                outside.is_null()
                            } else {
                                items[i].is_null()
                            }
                        }) {
                            return Err(
                                "picture beat emphasizes an edge with an empty endpoint".into()
                            );
                        }
                        let pa = positions[a];
                        let pb = positions[b];
                        s.line(
                            &format!("{prefix}picture-beat-{k}-{j}-link-{i}"),
                            &[
                                (ox + pa.0 * scale, oy + pa.1 * scale),
                                (ox + pb.0 * scale, oy + pb.1 * scale),
                            ],
                            s.theme.accent,
                            5.0,
                            time,
                            end,
                        );
                    }
                }
            }
        }
    }
    s.anchors.truncate(anchor_start);
    s.anchors.extend(
        positions
            .iter()
            .map(|p| (ox + p.0 * scale, oy + p.1 * scale)),
    );
    crate::grammar::callouts(s, spec, prefix, anchor_start)?;
    Ok(())
}
