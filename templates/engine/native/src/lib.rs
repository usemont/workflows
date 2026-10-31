//! Native, editable explainer scenes. Times in the input grammar are absolute seconds.
//! Generated spans use scene-local milliseconds and the shared native TSX compiler.
mod caption;
mod grammar;
mod layout;
mod music;
mod narrative;
mod scene;
mod scene_transitions;
pub mod typography;

pub use grammar::manifest;
use grammar::{array, fields, number, text};
use scene::{Scene, Theme};
use serde_json::{json, Value};

/// Compile a narrated-video scene grammar to canonical programmatic spans.
/// Audio remains owned by the workflow. No asset is fetched or browser code executed.
pub fn compile_video(video: &Value, width: u32, height: u32) -> Result<Vec<Value>, String> {
    if width < 240 || height < 240 || width > 8192 || height > 8192 {
        return Err("video dimensions must be between 240 and 8192 pixels".into());
    }
    fields(
        video,
        &["theme", "themeTokens", "scenes", "captions", "brands", "music"],
        "video",
    )?;
    let theme = Theme::from_video(
        video
            .get("theme")
            .map(|_| text(video, "theme"))
            .transpose()?
            .unwrap_or("daylight"),
        video.get("themeTokens"),
    )?;
    let scenes = array(video, "scenes", 1, 1000)?;
    let brands = grammar::brands(video.get("brands"))?;
    let music = music::Music::parse(video.get("music"), scenes.last().and_then(|s|s["end"].as_f64()).unwrap_or(0.0)+2.0)?;
    let snapped;
    let scenes = if let Some(music) = music.as_ref().filter(|music|music.snap) {
        // Validate the trust boundary before source boundary normalization reads times.
        let mut previous = 0.0;
        for (index, scene) in scenes.iter().enumerate() {
            grammar::scene(scene).map_err(|e|format!("scenes[{index}]: {e}"))?;
            let (start,end) = (number(scene,"start")?,number(scene,"end")?);
            if start < previous || end-start < 0.001 || end*1000.0 > i64::MAX as f64 {
                return Err("scenes must have increasing nonoverlapping start/end times".into());
            }
            previous = end;
        }
        let mut normalized = scenes.to_vec();
        music.snap_boundaries(&mut normalized);
        snapped = normalized;
        snapped.as_slice()
    } else { scenes };
    let (mut spans, previous_end) = scene_transitions::compile(scenes, width, height, theme, &brands, music.as_ref())?;
    if let Some(captions) = video.get("captions") {
        let captions = captions.as_array().ok_or("captions must be an array")?;
        let mut last = 0.0;
        for (index, caption) in captions.iter().enumerate() {
            fields(caption, &["text", "start", "end"], "caption")?;
            let start = number(caption, "start")?;
            let end = number(caption, "end")?;
            if start < last || end - start < 0.001 || end > previous_end {
                return Err(format!(
                    "captions[{index}]: invalid or overlapping caption interval"
                ));
            }
            let display_start = (start - 0.02).max(last).max(0.0);
            last = end;
            // captionFrame selects the next eligible caption after the current active one.
            let next_start = captions.get(index + 1)
                .and_then(|next| next.get("start"))
                .and_then(Value::as_f64)
                .map(|next| (next - 0.02).max(end))
                .unwrap_or(previous_end);
            let display_end = (end + 0.2).min(previous_end).min(next_start);
            let mut scene = Scene::new(
                format!("explainer-caption-{index}"),
                width,
                height,
                (display_end - display_start) * 1000.0,
                display_start,
                theme,
            );
            caption::render(&mut scene, text(caption, "text")?, start, end);
            let (source, variables) = scene.finish()?;
            spans.push(span(
                format!("explainer-caption-{index}"),
                &source,
                variables,
                display_start * 1000.0,
                (display_end - display_start) * 1000.0,
                width,
                height,
                100,
            )?);
        }
    }
    Ok(spans)
}

fn span(
    id: String,
    source: &str,
    variables: Value,
    start: f64,
    duration: f64,
    width: u32,
    height: u32,
    layer: i32,
) -> Result<Value, String> {
    Ok(
        json!({"id":id,"placement":{"startMs":start.round() as i64,"durationMs":duration.round() as i64,"trackId":if layer==100 {"explainer-captions".to_owned()} else {format!("explainer-scenes-{layer}")},"layer":layer},
        "capability":{"kind":"programmatic.scene","version":1,"state":{"sourceTsx":source,"variables":variables,"frame":{"x":0,"y":0,"width":width,"height":height}}},
        "interaction":{"selectable":true},"producer":{"kind":"programmatic","id":"mont-explainer-templates","version":"1"}}),
    )
}
