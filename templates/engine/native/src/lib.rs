//! Native, editable explainer scenes. Times in the input grammar are absolute seconds.
//! Generated spans use scene-local milliseconds and the shared native TSX compiler.
mod grammar;
mod layout;
mod narrative;
mod scene;

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
        &["theme", "themeTokens", "scenes", "captions", "brands"],
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
    let mut spans = Vec::with_capacity(scenes.len());
    let mut previous_end = 0.0;
    let brands = grammar::brands(video.get("brands"))?;
    for (index, spec) in scenes.iter().enumerate() {
        let result = (|| {
            grammar::scene(spec)?;
            let start = number(spec, "start")?;
            let end = number(spec, "end")?;
            if start < previous_end || end - start < 0.001 || end * 1000.0 > i64::MAX as f64 {
                return Err("scenes must have increasing nonoverlapping start/end times".into());
            }
            previous_end = end;
            let mut scene = Scene::new(
                format!("explainer-scene-{index}"),
                width,
                height,
                (end - start) * 1000.0,
                start,
                theme,
            );
            layout::render(&mut scene, spec, &brands)?;
            grammar::callouts(&mut scene, spec, "", 0)?;
            scene.finish()
        })();
        let (source, variables) = result.map_err(|e| format!("scenes[{index}]: {e}"))?;
        spans.push(span(
            format!("explainer-scene-{index}"),
            &source,
            variables,
            number(spec, "start")? * 1000.0,
            (number(spec, "end")? - number(spec, "start")?) * 1000.0,
            width,
            height,
            0,
        )?);
    }
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
            last = end;
            let mut scene = Scene::new(
                format!("explainer-caption-{index}"),
                width,
                height,
                (end - start) * 1000.0,
                start,
                theme,
            );
            let h = if scene.tall { 112.0 } else { 76.0 };
            let y = scene.h - h - if scene.tall { 150.0 } else { 34.0 };
            scene.rect(
                "caption-surface",
                scene.margin,
                y,
                scene.w - 2.0 * scene.margin,
                h,
                theme.caption_bg,
                0.0,
                None,
            );
            scene.text(
                "caption",
                text(caption, "text")?,
                scene.margin + 20.0,
                y + 8.0,
                scene.w - 2.0 * scene.margin - 40.0,
                h - 16.0,
                30.0,
                theme.caption_ink,
                0.0,
                None,
                "center",
            );
            let (source, variables) = scene.finish()?;
            spans.push(span(
                format!("explainer-caption-{index}"),
                &source,
                variables,
                start * 1000.0,
                (end - start) * 1000.0,
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
        json!({"id":id,"placement":{"startMs":start.round() as i64,"durationMs":duration.round() as i64,"trackId":if layer==100 {"explainer-captions"} else {"explainer-scenes"},"layer":layer},
        "capability":{"kind":"programmatic.scene","version":1,"state":{"sourceTsx":source,"variables":variables,"frame":{"x":0,"y":0,"width":width,"height":height}}},
        "interaction":{"selectable":true},"producer":{"kind":"programmatic","id":"mont-explainer-templates","version":"1"}}),
    )
}
