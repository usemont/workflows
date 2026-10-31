//! Source caption flex layout and its independent enter/exit motion layer.
use crate::{scene::Scene, typography};

fn q(s: &str) -> String { serde_json::to_string(s).expect("caption string") }
fn animate(prop: &str, from: f64, to: f64, start: f64, duration: f64, ease: &str) -> String {
    format!("<Animate prop={} from={{{from}}} to={{{to}}} start=\"{start}ms\" duration=\"{duration}ms\" ease={}/>",q(prop),q(ease))
}

fn wrap_words(widths: &[f64], gap: f64, content_width: f64) -> Vec<(usize, usize, f64)> {
    let mut lines = Vec::new();
    let (mut first, mut used) = (0, 0.0);
    for (i, &width) in widths.iter().enumerate() {
        let next = used + if i > first { gap } else { 0.0 } + width;
        // Max-content width and sequential placement accumulate gaps differently.
        // Ignore only floating-point roundoff, not a browser-sized layout tolerance.
        if i > first && next > content_width + 1.0e-9 {
            lines.push((first, i, used));
            first = i;
            used = width;
        } else {
            used = next;
        }
    }
    lines.push((first, widths.len(), used));
    lines
}

pub(crate) fn render(scene: &mut Scene<'_>, text: &str, start: f64, end: f64) {
    let theme = scene.theme.value();
    let tok = &theme["caption"];
    let n = |key: &str| tok[key].as_f64().expect("source caption numeric token");
    let s = scene.native_scale();
    let square = (scene.w - scene.h).abs() < 0.001;
    let (max_width, bottom, fs_scale) = if square { (860.0,40.0,1.0) } else if scene.tall { (640.0,210.0,1.05) } else { (1040.0,36.0,1.0) };
    let pad_x = n("padX"); let pad_y = n("padY");
    let (fs, _) = typography::fit_block(text, &scene.theme, (n("fs") * fs_scale).round(), 22.0, 2, max_width-pad_x*2.0);
    let line_height = (fs*1.3).round();
    let weight = n("weight") as u16;
    let words: Vec<_> = text.split(' ').collect();
    let widths: Vec<_> = words.iter().map(|word| typography::measure_style(word, scene.theme.font, weight, -0.005, fs, 1.0e9, line_height/fs).width).collect();
    let gap = fs*0.26;
    // Inline flex uses max-content width until constrained, then wraps within that box.
    let content_width = (widths.iter().sum::<f64>() + gap * widths.len().saturating_sub(1) as f64).min(max_width-pad_x*2.0);
    let lines = wrap_words(&widths, gap, content_width);
    let w = content_width+pad_x*2.0; let h = line_height*lines.len() as f64+pad_y*2.0;
    let x = (scene.w-w)*0.5; let y = scene.h-bottom-h;
    let enter = (start-scene.start)*1000.0; let exit = (end-scene.start)*1000.0;
    let mut out = format!("<MotionBox id=\"caption-exit\" opacity={{1}}>{}{}<MotionBox id=\"caption-position\" x={{{}}} y={{{}}} width={{{}}} height={{{}}}><MotionBox id=\"caption-enter\" width={{{}}} height={{{}}} pivotX={{{}}} pivotY={{{}}}>",animate("opacity",1.0,0.0,exit,200.0,"inCubic"),animate("y",0.0,6.0*s,exit,200.0,"inCubic"),x*s,y*s,w*s,h*s,w*s,h*s,w*0.5*s,h*0.5*s);
    match theme["motion"]["in"].as_str().expect("source motion") {
        "lift" => {out.push_str(&animate("y",12.0*s,0.0,enter,280.0,"outQuint"));out.push_str(&animate("opacity",0.0,1.0,enter,112.0,"linear"));},
        "focus" => {out.push_str(&animate("blur",8.0*s,0.0,enter,280.0,"outExpo"));out.push_str(&animate("opacity",0.0,1.0,enter,140.0,"linear"));},
        "draft" => out.push_str(&animate("clipRight",100.0,0.0,enter,280.0,"outExpo")),
        _ => {out.push_str(&animate("scale",0.86,1.0,enter,280.0,"outBack"));out.push_str(&animate("opacity",0.0,1.0,enter,70.0,"linear"));},
    }
    let backdrop = tok["backdrop"].as_f64().map(|v|format!(" backdropBlur={{{}}}",v*s)).unwrap_or_default();
    let shadow = tok["shadow"].as_str().unwrap().split_whitespace().map(|part| {
        part.strip_suffix("px").and_then(|v|v.parse::<f64>().ok())
            .map(|v|format!("{}px",v*s)).unwrap_or_else(||part.to_owned())
    }).collect::<Vec<_>>().join(" ");
    out.push_str(&format!("<Rect id=\"caption-surface\" x={{0}} y={{0}} width={{{}}} height={{{}}} radius={{{}}} fill={} shadow={}{} />",w*s,h*s,n("radius")*s,q(tok["bg"].as_str().unwrap()),q(&shadow),backdrop));
    let progress = tok["progress"].as_bool().unwrap_or(false);
    let total = text.encode_utf16().count().max(1) as f64;
    let mut run = 0;
    for (line, &(first,last,width)) in lines.iter().enumerate() {
        let mut wx = pad_x+(content_width-width)*0.5;
        for i in first..last {
            let at = enter+(end-start)*1000.0*run as f64/total;
            run += words[i].encode_utf16().count()+1;
            let word = |color: &str,id: &str| format!("<Text id={} text={} x={{{}}} y={{{}}} width={{{}}} height={{{}}} size={{{}}} lineHeight={{{}}} letterSpacing={{-0.005}} fontFamily={} weight={} color={} align=\"left\"/>",q(id),q(words[i]),wx*s,(pad_y+line as f64*line_height)*s,widths[i].max(0.001)*s,line_height*s,fs*s,line_height*s,q(scene.theme.font),q(&weight.to_string()),q(color));
            if progress {
                out.push_str(&format!("<MotionBox id=\"caption-dim-{i}\" opacity={{1}}>{}{}</MotionBox><MotionBox id=\"caption-lit-{i}\" opacity={{0}}>{}{}</MotionBox>",animate("opacity",1.0,0.0,at,0.0,"linear"),word(tok["dim"].as_str().unwrap(),&format!("caption-dim-word-{i}")),animate("opacity",0.0,1.0,at,0.0,"linear"),word(tok["ink"].as_str().unwrap(),&format!("caption-word-{i}"))));
            } else { out.push_str(&word(tok["ink"].as_str().unwrap(),&format!("caption-word-{i}"))); }
            wx += widths[i]+gap;
        }
    }
    out.push_str("</MotionBox></MotionBox></MotionBox>");
    scene.push_native(&out);
}


#[cfg(test)]
mod tests {
    use super::wrap_words;

    #[test]
    fn max_content_caption_does_not_wrap_its_last_word_from_roundoff() {
        let widths = [71.010009765625, 84.510009765625, 98.1060006621094,
            21.030000686645508, 38.970001220703125, 83.52000427246094];
        let gap = 30.0 * 0.26;
        let width = widths.iter().sum::<f64>() + gap * (widths.len() - 1) as f64;
        let lines = wrap_words(&widths, gap, width);
        assert_eq!(lines.len(), 1);
        assert_eq!((lines[0].0, lines[0].1), (0, widths.len()));
        let constrained = wrap_words(&widths, gap, width - 1.0);
        assert_eq!(constrained.iter().map(|line| (line.0, line.1)).collect::<Vec<_>>(),
            [(0, widths.len() - 1), (widths.len() - 1, widths.len())]);
    }
}
