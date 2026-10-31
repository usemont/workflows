//! Native shaping and source-byte geometry. Uses the same cosmic-text version as glyphon.
use std::cell::RefCell;
use cosmic_text::{fontdb, Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Weight, Wrap};
use crate::scene::Theme;
#[path = "fonts.rs"]
mod fonts;
pub use fonts::{css_weight, FONT_ASSETS};
#[path = "typography_nodes.rs"]
mod nodes;
pub(crate) use nodes::{lock_sibling_type, source_label_fit, source_text_node};

#[derive(Clone, Debug)]
pub struct MeasuredLine { pub start: usize, pub end: usize, pub x: f64, pub y: f64, pub baseline: f64, pub width: f64, pub height: f64 }
#[derive(Clone, Debug)]
pub struct MeasuredWord { pub start: usize, pub end: usize, pub line: usize, pub x: f64, pub y: f64, pub width: f64, pub height: f64 }
#[derive(Clone, Debug, Default)]
pub struct MeasuredText { pub width: f64, pub height: f64, pub lines: Vec<MeasuredLine>, pub words: Vec<MeasuredWord> }

fn font_system() -> FontSystem {
    // new_with_fonts also loads OS fonts on native targets. An explicit database does not.
    let mut db = fontdb::Database::new();
    for &(family, weight, bytes) in FONT_ASSETS {
        let ids = db.load_font_source(fontdb::Source::Binary(std::sync::Arc::new(bytes.to_vec())));
        for id in ids {
            let mut face = db.face(id).expect("bundled font must decode").clone();
            db.remove_face(id);
            // @font-face supplies aliases even where legacy name tables say e.g. Geist SemiBold.
            face.families = vec![(family.to_owned(), fontdb::Language::English_UnitedStates)];
            face.weight = fontdb::Weight(weight);
            db.push_face_info(face);
        }
    }
    db.set_sans_serif_family("Geist");
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}
thread_local! { static FONTS: RefCell<FontSystem> = RefCell::new(font_system()); }

/// Measure a whole shaped paragraph, retaining source byte ranges for highlights.
/// `tracking` is in em, `line_height` is a multiplier; no installed-font fallback.
pub fn measure_style(text: &str, family: &str, weight: u16, tracking: f64, size: f64, max_width: f64, line_height: f64) -> MeasuredText {
    measure_style_wrapped(text, family, weight, tracking, size, max_width, line_height, Wrap::WordOrGlyph)
}

/// CSS labels permit Unicode word breaks, not emergency glyph wrapping.
pub fn measure_label_style(text: &str, family: &str, weight: u16, tracking: f64, size: f64, max_width: f64, line_height: f64) -> MeasuredText {
    measure_style_wrapped(text, family, weight, tracking, size, max_width, line_height, Wrap::Word)
}

fn measure_style_wrapped(text: &str, family: &str, weight: u16, tracking: f64, size: f64, max_width: f64, line_height: f64, wrap: Wrap) -> MeasuredText {
    let family = family.split(',').next().unwrap_or(family).trim().trim_matches(['\'', '"']);
    assert!((100..=900).contains(&weight), "invalid native font weight {weight}");
    let weight = fonts::css_weight(family, weight).unwrap_or_else(|| panic!("unbundled native explainer font: {family}"));
    assert!(size.is_finite() && size > 0.0 && max_width.is_finite() && max_width > 0.0 && line_height.is_finite() && line_height > 0.0 && tracking.is_finite(), "invalid native text metrics");
    FONTS.with_borrow_mut(|fonts| {
        let mut buffer = Buffer::new(fonts, Metrics::new(size as f32, (size * line_height) as f32));
        buffer.set_wrap(fonts, wrap);
        buffer.set_size(fonts, Some(max_width as f32), None);
        let attrs = Attrs::new().family(Family::Name(family)).weight(Weight(weight)).letter_spacing(tracking as f32);
        let attrs = if tracking != 0.0 {
            let mut features = cosmic_text::FontFeatures::new();
            features.disable(cosmic_text::FeatureTag::STANDARD_LIGATURES);
            features.disable(cosmic_text::FeatureTag::CONTEXTUAL_LIGATURES);
            attrs.font_features(features)
        } else { attrs };
        buffer.set_text(fonts, text, &attrs, Shaping::Advanced);
        buffer.shape_until_scroll(fonts, false);
        let mut next_offset = 0;
        let offsets: Vec<_> = buffer.lines.iter().map(|line| {
            let offset = next_offset;
            next_offset += line.text().len() + line.ending().as_str().len();
            offset
        }).collect();
        let mut result = MeasuredText::default();
        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                let face = fonts.db().face(glyph.font_id).expect("shaped font must exist");
                assert!(glyph.glyph_id != 0 && face.weight.0 == weight &&
                    face.families.iter().any(|(name,_)| name == family),
                    "native explainer font {family} {weight} does not support {:?}; system fallback is not reproducible",
                    &run.text[glyph.start..glyph.end]);
            }
            let offset = offsets[run.line_i];
            let start = run.glyphs.iter().map(|g|g.start).min().unwrap_or(0);
            let end = run.glyphs.iter().map(|g|g.end).max().unwrap_or(start);
            let line = result.lines.len();
            let y = run.line_top as f64;
            let height = run.line_height as f64;
            let width = run.line_w as f64;
            result.width = result.width.max(width);
            result.height = result.height.max(y + height);
            result.lines.push(MeasuredLine { start:offset+start, end:offset+end, x:0.0, y, baseline:run.line_y as f64, width, height });
            // Each word's box is a union of clusters from the SAME shaped buffer, never reshaped.
            let mut word_start = None;
            for (i,c) in run.text.char_indices().chain(std::iter::once((run.text.len(), ' '))) {
                if !c.is_whitespace() { word_start.get_or_insert(i); continue; }
                if let Some(ws) = word_start.take() {
                    let mut left = f32::INFINITY;
                    let mut right = f32::NEG_INFINITY;
                    let mut fragment_start = i;
                    let mut fragment_end = ws;
                    for g in run.glyphs.iter().filter(|g| g.start < i && g.end > ws) {
                        left = left.min(g.x); right = right.max(g.x + g.w);
                        fragment_start = fragment_start.min(g.start.max(ws));
                        fragment_end = fragment_end.max(g.end.min(i));
                    }
                    if left.is_finite() {
                        result.words.push(MeasuredWord { start:offset+fragment_start, end:offset+fragment_end, line, x:left as f64, y, width:(right-left) as f64, height });
                    }
                }
            }
        }
        result
    })
}

pub(crate) fn measure(text: &str, theme: &Theme<'_>, size: f64, max_width: f64, line_height: f64, bold: bool) -> MeasuredText {
    measure_style(text, theme.font, if bold { theme.weight as u16 } else { 600 }, theme.tracking, size, max_width, line_height)
}

// util.js's font-loaded canvas oracle always uses weight 600, no CSS tracking,
// and a 3% layout allowance. Do not conflate it with the actual DOM glyph boxes.
fn oracle_width(text: &str, theme: &Theme<'_>, size: f64) -> f64 {
    // Canvas isolates spaces, but retains kerning across hard hyphens. Buffer
    // splits both; shaping the entire string instead incorrectly kerns spaces.
    FONTS.with_borrow_mut(|fonts| {
        let attrs = cosmic_text::AttrsList::new(&Attrs::new().family(Family::Name(theme.font)).weight(Weight(600)));
        let mut width = 0.0;
        let mut start = 0;
        for (index, ch) in text.char_indices().filter(|(_, ch)| ch.is_whitespace()) {
            if start < index {
                let run = cosmic_text::ShapeWord::new(fonts, text, &attrs, start..index, 0u8.into(), false, Shaping::Advanced);
                width += run.width(size as f32) as f64;
            }
            // Canvas normalizes ASCII whitespace to a space before measuring.
            let mut bytes = [0; 4];
            let space = if ch.is_ascii_whitespace() { " " } else { ch.encode_utf8(&mut bytes) };
            let run = cosmic_text::ShapeWord::new(fonts, space, &attrs, 0..space.len(), 0u8.into(), false, Shaping::Advanced);
            width += run.width(size as f32) as f64;
            start = index + ch.len_utf8();
        }
        if start < text.len() {
            let run = cosmic_text::ShapeWord::new(fonts, text, &attrs, start..text.len(), 0u8.into(), false, Shaping::Advanced);
            width += run.width(size as f32) as f64;
        }
        width * 1.03
    })
}
pub(crate) fn pill_width(text: &str, theme: &Theme<'_>, size: f64) -> f64 {
    ((oracle_width(text, theme, size) * 1.06 + 36.0) / 2.0).round() * 2.0
}
fn oracle_lines(text: &str, theme: &Theme<'_>, size: f64, width: f64) -> usize {
    let space = oracle_width(" ", theme, size);
    let mut lines = 1; let mut used = 0.0;
    for word in text.split(' ') {
        let w = oracle_width(word, theme, size);
        if used > 0.0 && used + space + w > width { lines += 1; used = w; }
        else { used += if used > 0.0 { space + w } else { w }; }
    }
    lines
}
pub(crate) fn fit_one_line(text: &str, theme: &Theme<'_>, size_max: f64, size_min: f64, width: f64) -> f64 {
    (width * size_max / oracle_width(text, theme, size_max).max(1.0)).floor().min(size_max).max(size_min)
}
fn longest_word(text:&str)->&str {
    text.split(' ').fold("",|longest,word|if word.encode_utf16().count()>longest.encode_utf16().count(){word}else{longest})
}
pub(crate) fn fit_block(text: &str, theme: &Theme<'_>, size_max: f64, size_min: f64, max_lines: usize, width: f64) -> (f64, usize) {
    let longest = longest_word(text);
    let mut size = size_max;
    while size > size_min {
        let lines = oracle_lines(text, theme, size, width);
        if oracle_width(longest, theme, size) <= width && lines <= max_lines { return (size, lines); }
        size -= 2.0;
    }
    (size_min, oracle_lines(text, theme, size_min, width).min(max_lines))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn proportional_spacing_tracking_and_contextual_word_ranges() {
        for (family,tracking) in [("Geist",-0.025),("Space Grotesk",-0.02),("IBM Plex Sans",-0.01),("Archivo",-0.02)] {
            let m = |s:&str,t:f64,w:f64| measure_style(s,family,600,t,48.0,w,1.2);
            assert!(m("WWW",0.0,900.0).width > m("iii",0.0,900.0).width * 2.0);
            assert!(m("A  B",0.0,900.0).width > m("A B",0.0,900.0).width);
            assert!(m("Wide tracking",tracking,900.0).width < m("Wide tracking",0.0,900.0).width);
            let tracking_delta = m("MMMM",0.0,900.0).width - m("MMMM",tracking,900.0).width;
            assert!((tracking_delta - 4.0*48.0*-tracking).abs() < 0.05);
            let text = "office AV wide words";
            let measured = m(text,tracking,200.0);
            assert!(measured.lines.len() > 1);
            assert_eq!(measured.words.iter().map(|w|&text[w.start..w.end]).collect::<Vec<_>>(), vec!["office","AV","wide","words"]);
            for word in &measured.words { assert!(word.width > 0.0); assert!(word.x + word.width <= 200.1); }
        }
    }
    #[test]
    fn css_weight_matching_and_wrapped_cluster_fragments() {
        assert_eq!(css_weight("Geist",650),Some(700));
        assert_eq!(css_weight("IBM Plex Sans",800),Some(700));
        assert_eq!(css_weight("Archivo",800),Some(800));
        assert_eq!(css_weight("missing",600),None);
        let requested=measure_style("Spot label","Geist",650,-0.025,48.0,900.0,1.1);
        let face=measure_style("Spot label","Geist",700,-0.025,48.0,900.0,1.1);
        assert_eq!(requested.width,face.width);
        let text="WWWWWWWW";
        let wrapped=measure_style(text,"Geist",600,-0.025,48.0,80.0,1.1);
        assert!(wrapped.lines.len()>1);
        let fragments=wrapped.words.iter().map(|word|&text[word.start..word.end]).collect::<String>();
        assert_eq!(fragments,text);
    }
}
