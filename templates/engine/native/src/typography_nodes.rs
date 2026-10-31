//! v9.12 nodeFrame text spans. Geometry is shaped, never guessed from character counts.
use crate::scene::{q, Scene};
use cosmic_text::{Attrs, AttrsList, Family, ShapeWord, Shaping, Weight};
use serde_json::Value;
use super::{css_weight, measure_label_style, measure_style, oracle_lines, oracle_width};
static TYPES: std::sync::LazyLock<Value> = std::sync::LazyLock::new(|| serde_json::from_str(include_str!("node-types.json")).expect("source node types"));
static THEMES: std::sync::LazyLock<Value> = std::sync::LazyLock::new(|| serde_json::from_str(include_str!("themes.json")).expect("source themes"));

// Chromium's inline intrinsic widths round up to LayoutUnit (1/64px), while
// CSS lengths and distributed flex offsets truncate to that fixed-point grid.
fn inline_width(width:f64)->f64 {(width*64.0).ceil()/64.0}
fn layout_length(length:f64)->f64 {(length*64.0).trunc()/64.0}
// Flex centering is not "safe center": overflowing lines extend above/left.
fn flex_center(available:f64,content:f64)->f64 {layout_length((available-content)/2.0)}
// Blink TextFragmentPainter snaps the line's paint origin before CSS transforms;
// rounding only the final transformed baseline loses a pixel during word reveals.
fn line_paint_origin(top:f64)->f64 {(top+0.5).floor()}

fn emit(scene: &mut Scene<'_>, id:&str, text:&str, x:f64,y:f64,w:f64,h:f64,fs:f64,lh:f64,weight:u16,tracking:f64,color:&str,align:&str,decorative:bool) {
    let s=scene.native_scale();
    scene.push_native(&format!("<Text id={} text={{{}}} x={{{}}} y={{{}}} width={{{}}} height={{{}}} size={{{}}} lineHeight={{{}}} letterSpacing={{{tracking}}} fontFamily={} weight={} color={} align={} ariaHidden={{{decorative}}}/>",q(id),q(text),x*s,y*s,w*s,h*s,fs*s,lh*s,q(scene.theme.font),q(&weight.to_string()),q(color),q(align)));
}

/// Original labelFit, before flex positioning, over the WHOLE node geometry.
/// An existing lock is authoritative: lockSiblingType has already equalized its siblings.
pub(crate) fn source_label_fit(scene:&Scene<'_>,n:&Value,kind:&str,w:f64,h:f64)->(f64,usize,usize) {
    let d=TYPES.get(kind).expect("source label type");
    if d["role"]=="pill" {
        return (n["fs"].as_f64().or_else(||d["fs"].as_f64()).expect("source pill size"),1,1);
    }
    if let Some(fs)=n["lock"]["fs"].as_f64() {
        return (fs,n["lock"]["lines"].as_u64().unwrap_or(1) as usize,n["lock"]["room"].as_u64().unwrap_or(1) as usize);
    }
    label_fit(scene,n,d,w,h,999.0)
}

fn label_fit(scene:&Scene<'_>,n:&Value,d:&Value,w:f64,h:f64,cap:f64)->(f64,usize,usize) {
    let base=n["fs"].as_f64().or_else(||d["fs"].as_f64()).expect("source label base size").min(cap);
    let marker=n["marker"].as_str().is_some_and(|s|!s.is_empty());
    let icon=n["icon"].as_str().is_some_and(|s|!s.is_empty());
    let layout=d["layout"].as_str().unwrap_or("");
    let icon_size=if marker{n["ms"].as_f64().expect("source marker size")}else if icon {
        let size=d["icon"].as_f64().unwrap_or(0.0);
        if size<=1.0{(w*size).round()}else if w<170.0&&layout=="col"{(size*0.78).round()}else{size}
    }else{0.0};
    let mut pl=d["px"].as_f64().unwrap_or(0.0);
    let mut pr=d["pr"].as_f64().unwrap_or(pl);
    if d["role"]=="card"&&w<180.0{pl=pl.min(10.0);pr=pr.min(10.0);}
    let gap=d["gap"].as_f64().unwrap_or(14.0);
    let mut avail=if layout=="row"{w-pl-pr-if icon_size!=0.0{icon_size+gap}else{0.0}}else{w-2.0*pl};
    let vroom=if layout=="col"{h-if icon_size!=0.0{icon_size+14.0}else{0.0}-24.0}else if layout=="slab"{h*0.45}else{h-16.0};
    if marker{avail=w-icon_size-gap;}
    let vr=if layout=="row"&&marker{h-4.0}else{vroom};
    avail*=0.93;
    let label=n["label"].as_str().unwrap_or("");
    let one=(avail*base/oracle_width(label,&scene.theme,base).max(1.0)).floor();
    let mut fit=(base.min(one),1usize);
    if one<(base*0.82).round() {
        fit=(base.min(one).max(16.0),1);
        let longest=super::longest_word(label);
        let mut fs=(base*0.92).round();
        while fs>=16.0 {
            let max_lines=(vr/(fs*1.16)).floor().min(4.0);
            if max_lines>=2.0&&oracle_width(longest,&scene.theme,fs)<=avail {
                let lines=oracle_lines(label,&scene.theme,fs,avail);
                if lines<=max_lines as usize{fit=(fs,lines.max(2));break;}
            }
            fs-=1.0;
        }
    }
    let room=(vr/(fit.0*1.16)).floor().clamp(1.0,4.0) as usize;
    (fit.0,fit.1,room)
}

/// Original lockSiblingType: group by TYPE size, then refit every member under
/// the smallest first-pass size. The second pass can change wrapping and room.
pub(crate) fn lock_sibling_type(scene:&Scene<'_>,nodes:&mut [Value]) {
    let mut sizes=std::collections::BTreeMap::<u64,f64>::new();
    for n in nodes.iter() {
        let Some(d)=n["t"].as_str().and_then(|kind|TYPES.get(kind)) else{continue};
        let Some(key)=d["fs"].as_u64() else{continue};
        if d["role"]=="pill"||n["grp"]!="item"{continue;}
        let fit=label_fit(scene,n,d,n["w"].as_f64().expect("source node width"),n["h"].as_f64().expect("source node height"),999.0);
        sizes.entry(key).and_modify(|fs|*fs=fs.min(fit.0)).or_insert(fit.0);
    }
    for n in nodes.iter_mut() {
        let Some(d)=n["t"].as_str().and_then(|kind|TYPES.get(kind)) else{continue};
        let Some(cap)=d["fs"].as_u64().and_then(|key|sizes.get(&key)).copied() else{continue};
        if d["role"]=="pill"||n["grp"]!="item"{continue;}
        let (fs,lines,room)=label_fit(scene,n,d,n["w"].as_f64().expect("source node width"),n["h"].as_f64().expect("source node height"),cap);
        n["lock"]=serde_json::json!({"fs":fs.min(cap),"lines":lines,"room":room});
    }
}

// text-wrap:balance scores unused space at the ORIGINAL available width. Shrinking
// the box until its greedy line count changes is not equivalent at hyphen breaks.
fn balanced_label(text:&str,family:&str,weight:u16,tracking:f64,fs:f64,width:f64,lh:f64)->String {
    text.split('\n').map(|paragraph| {
        let shape=|text:&str,w:f64|measure_label_style(text,family,weight,tracking,fs,w.max(1.0),lh/fs);
        let line_count=shape(paragraph,width).lines.len();
        // Blink balances short paragraphs only; longer labels retain greedy wrapping.
        if !(2..=6).contains(&line_count) {return paragraph.to_owned();}
        let attrs=Attrs::new().family(Family::Name(family)).weight(Weight(weight)).letter_spacing(tracking as f32);
        let attrs=if tracking!=0.0 {
            let mut features=cosmic_text::FontFeatures::new();
            features.disable(cosmic_text::FeatureTag::STANDARD_LIGATURES);
            features.disable(cosmic_text::FeatureTag::CONTEXTUAL_LIGATURES);
            attrs.font_features(features)
        } else {attrs};
        let attrs=AttrsList::new(&attrs);
        // Word-only wrapping exposes Unicode line-break opportunities, including
        // existing hyphens, without introducing emergency breaks inside Approval.
        let words=shape(paragraph,1.0);
        let mut ends=vec![0];
        for line in &words.lines {
            let end=paragraph[..line.end].trim_end().len();
            if end>*ends.last().unwrap(){ends.push(end);}
        }
        if *ends.last().unwrap()!=paragraph.len(){ends.push(paragraph.len());}
        let count=ends.len();
        let mut scores=vec![vec![(f64::INFINITY,0usize);count];line_count+1];
        scores[0][0]=(0.0,0);
        for end in 1..count {
            for start in 0..end {
                // Buffer splits shaping at spaces and hard hyphens. DOM candidate
                // lines retain kerning across those opportunities until broken.
                let fragment=paragraph[ends[start]..ends[end]].trim();
                let advance=super::FONTS.with_borrow_mut(|fonts| {
                    let run=ShapeWord::new(fonts,fragment,&attrs,0..fragment.len(),0u8.into(),false,Shaping::Advanced);
                    // Preserve Chromium's rounded-up 1/64px inline advances.
                    inline_width(run.width(fs as f32) as f64)
                });
                // An indivisible token may overflow under CSS's normal wrapping;
                // it must not be replaced by emergency glyph-break candidates.
                if advance>width+1.0/64.0 && end>start+1 {continue;}
                let slack=width+1.0/64.0-advance;
                for row in 1..=line_count {
                    let score=scores[row-1][start].0+slack*slack+if slack<0.0{1.0e12}else{0.0};
                    if score<=scores[row][end].0 {scores[row][end]=(score,start);}
                }
            }
        }
        if !scores[line_count][count-1].0.is_finite(){return paragraph.to_owned();}
        let mut breaks=vec![paragraph.len()];
        let mut end=count-1;
        for row in (1..=line_count).rev() {
            end=scores[row][end].1;
            breaks.push(ends[end]);
        }
        breaks.reverse();
        breaks.windows(2).map(|range|paragraph[range[0]..range[1]].trim()).collect::<Vec<_>>().join("\n")
    }).collect::<Vec<_>>().join("\n")
}

/// Source TYPE text uses individually shaped word spans (with a .24em flex gap).
/// `at`/`until` are owned by the enclosing theme-motion group; n.a still drives word stagger.
pub(crate) fn source_text_node(scene:&mut Scene<'_>,id:&str,n:&Value,x:f64,y:f64,w:f64,h:f64,_at:f64,_until:Option<f64>)->Result<(),String> {
    let kind=n["t"].as_str().ok_or("text node requires source t")?;
    let d=TYPES.get(kind).ok_or_else(||format!("unknown source text node {kind:?}"))?;
    let label=n["label"].as_str().unwrap_or("");
    let theme=scene.theme;
    let is_text=d["text"].as_u64()==Some(1);
    let mut fs=n["fs"].as_f64().or_else(||d["fs"].as_f64()).unwrap_or(52.0);
    let align=n["align"].as_str().unwrap_or("left");
    let mut color=if d["accent"].as_u64()==Some(1){theme.accent}else if d["muted"].as_u64()==Some(1){theme.muted}else if d["role"]=="focus"{theme.focus_ink}else{theme.ink};
    if let Some(ink)=n["color"].as_str(){color=ink;}
    if !is_text {
        if let Some(ink)=n["_labelInk"].as_str(){color=ink;}
        let pill=d["role"]=="pill";
        let fit=source_label_fit(scene,n,kind,n["w"].as_f64().unwrap_or(w),n["h"].as_f64().unwrap_or(h));
        fs=fit.0;
        let clamp_lines=fit.1.max(fit.2);
        let lh=(fs*1.16).round();
        let requested=n["_labelWeight"].as_u64().map(|weight|weight as u16).unwrap_or_else(||if pill{500}else if fs>=30.0{theme.weight as u16}else{600});
        let weight=css_weight(theme.font,requested).ok_or("unbundled label font")?;
        let tracking=n["_labelTracking"].as_f64().unwrap_or_else(||if fs>=30.0{theme.tracking}else{-0.005});
        let shape=|text:&str,width:f64|measure_label_style(text,theme.font,weight,tracking,fs,width.max(1.0),lh/fs);
        let render_width=w;
        let mut rendered=if clamp_lines>1{balanced_label(label,theme.font,weight,tracking,fs,w,lh)}else{label.to_owned()};
        let measured=shape(&rendered,if clamp_lines==1{1.0e9}else{render_width});
        if measured.lines.len()>clamp_lines || (clamp_lines==1 && measured.width>w) {
            let last=&measured.lines[clamp_lines.min(measured.lines.len())-1];
            let start=last.start;
            let mut end=last.end;
            loop {
                let tail=format!("{}…",&rendered[start..end]);
                if shape(&tail,1.0e9).width<=render_width || end<=start {rendered=format!("{}{}",&rendered[..start],tail);break;}
                end=rendered[..end].char_indices().last().map(|(i,_)|i).unwrap_or(start).max(start);
            }
        }
        let height=shape(&rendered,render_width).height.min(clamp_lines as f64*lh);
        emit(scene,id,&rendered,x+if align=="center"{(w-render_width)/2.0}else{0.0},y+(h-height).max(0.0)/2.0,render_width,h,fs,lh,weight,tracking,color,align,false);
        return Ok(());
    }
    let lh=n["lh"].as_f64().unwrap_or_else(||(fs*1.1).round());
    let requested=d["fw"].as_u64().unwrap_or(theme.weight as u64) as u16;
    let weight=css_weight(theme.font,requested).ok_or("unbundled text font")?;
    // balancePad: canvas's 600-weight 3% oracle governs the source flex measure.
    let mut measure=w;
    if !label.contains('\n') {
        let lines=oracle_lines(label,&theme,fs,w);
        if lines>=2 {
            let widest=label.split(' ').map(|s|oracle_width(s,&theme,fs)).fold(0.0,f64::max);
            let (mut lo,mut hi)=((w*0.4).max(widest),w);
            if lo<hi {
                for _ in 0..14 {let mid=(lo+hi)/2.0;if oracle_lines(label,&theme,fs,mid)>lines{lo=mid}else{hi=mid}}
                let target=w.min(hi*1.06+fs*0.4);
                // Source serializes each padding side with f1, not the target width.
                let pad=(w-target).max(0.0);
                measure=w-if align=="center"{2.0*layout_length((pad/2.0*10.0).round()/10.0)}else{layout_length((pad*10.0).round()/10.0)};
            }
        }
    }
    let left=x+if align=="center"{(w-measure)/2.0}else{0.0};
    let words:Vec<&str>=label.split_inclusive('\n').flat_map(|part|{
        let mut v:Vec<_>=part.trim_end_matches('\n').split(' ').filter(|s|!s.is_empty()).collect();
        if part.ends_with('\n'){v.push("\n");} v
    }).collect();
    let gap=layout_length(fs*0.24);
    let mut rows:Vec<Vec<(usize,&str,f64)>>=vec![Vec::new()];
    let mut used=0.0;
    for (index,word) in words.iter().copied().enumerate(){
        if word=="\n"{rows.push(Vec::new());used=0.0;continue;}
        let width=inline_width(measure_style(word,theme.font,weight,theme.tracking,fs,1.0e9,lh/fs).width);
        if !rows.last().unwrap().is_empty() && used+gap+width>measure {rows.push(Vec::new());used=0.0;}
        if used>0.0{used+=gap;}
        rows.last_mut().unwrap().push((index,word,width));used+=width;
    }
    let top=y+flex_center(h,rows.len() as f64*lh);
    let a=n["a"].as_f64().unwrap_or(0.0)*1000.0;
    let hl=n["hl"].as_array();
    let highlight=&THEMES[theme.name]["highlight"];
    let hs=highlight["kind"].as_str().unwrap_or("");
    let hcolor=highlight["color"].as_str().unwrap_or(theme.accent);
    for (row_index,row) in rows.iter().enumerate(){
        let width=row.iter().map(|r|r.2).sum::<f64>()+gap*row.len().saturating_sub(1) as f64;
        let mut xx=left+if align=="center"{flex_center(measure,width)}else{0.0};
        for &(index,word,ww) in row {
            let yy=line_paint_origin(top+row_index as f64*lh);
            let s=scene.native_scale();
            let clip=matches!(theme.name,"daylight"|"blueprint");
            scene.push_native(&format!("<MotionBox id={} x={{{}}} y={{{}}} width={{{}}} height={{{}}}{}>",q(&format!("{id}-word-{index}-clip")),(xx-fs*0.05)*s,yy*s,(ww+fs*0.1)*s,(lh+fs*0.16)*s,if clip{" clipTop={0} clipBottom={0} clipLeft={0} clipRight={0}"}else{""}));
            let stagger=if theme.name=="midnight"{90.0}else if theme.name=="signal"{60.0}else{75.0};
            let duration=if theme.name=="midnight"{1000.0}else{850.0};
            let start=a+index as f64*stagger;
            let k=scene.number(format!("Math.min(1, Math.max(0, (computed.sourceClock - ({start})) / {duration}))"));
            let expo=scene.number(format!("{k} == 1 ? 1 : 1 - Math.pow(2, -10 * {k})"));
            let(mut dx,mut dy,mut sc,mut op)=("0".to_owned(),"0".to_owned(),"1".to_owned(),"1".to_owned());
            let mut blur=String::new();
            match theme.name {
                "daylight"=>{let percent=scene.css_number(&format!("(1 - {expo}) * 115"),1);dy=scene.number(format!("{percent} * {} / 100",lh*s));},
                "midnight"=>{
                    let px=scene.css_number(&format!("(1 - {expo}) * 14"),1);dy=scene.number(format!("{px} * {s}"));
                    op=scene.css_number(&format!("Math.min(1, Math.max(0, {k} * 1.6))"),3);
                    let px=scene.css_number(&format!("(1 - {expo}) * 12"),1);let px=scene.number(format!("{px} * {s}"));blur=format!(" blur={{{px}}}");
                },
                "blueprint"=>{let percent=scene.css_number(&format!("(1 - {expo}) * -104"),1);dx=scene.number(format!("{percent} * {} / 100",ww*s));},
                "signal"=>{
                    let cubic=scene.number(format!("1 - Math.pow(1 - {k}, 3)"));
                    let percent=scene.css_number(&format!("(1 - {cubic}) * -0.35 * 100"),1);dy=scene.number(format!("{percent} * {} / 100",lh*s));
                    sc=scene.css_number(&format!("0.55 + 0.45 * (1 + 2.7 * Math.pow({k} - 1, 3) + 1.7 * Math.pow({k} - 1, 2))"),3);
                    op=scene.css_number(&format!("Math.min(1, Math.max(0, {k} * 4))"),3);
                },
                _=>unreachable!(),
            }
            // Blink rounds the paint-offset translation of a transformed inline
            // box, discarding the fractional residual on non-unit scale axes.
            // This precedes the word transform; DOM layout bounds stay fractional.
            // See PaintPropertyTreeBuilder::CanPropagateSubpixelAccumulation.
            let paint_dx=line_paint_origin(xx-x)-(xx-x);
            let paint_dy=line_paint_origin(yy-y)-(yy-y);
            dx=scene.number(format!("({dx}) + ({sc} == 1 ? 0 : {})",paint_dx*s));
            dy=scene.number(format!("({dy}) + ({sc} == 1 ? 0 : {})",paint_dy*s));
            scene.push_native(&format!("<MotionBox id={} x={{{dx}}} y={{{dy}}} scaleX={{{sc}}} scaleY={{{sc}}} opacity={{{op}}} pivotX={{{}}} pivotY={{{}}}{blur}>",q(&format!("{id}-word-{index}")),(ww*0.5+fs*0.05)*s,lh*0.5*s));
            let selected=hl.and_then(|v|Some((v.first()?.as_u64()? as usize,v.get(1)?.as_u64()? as usize))).filter(|&(lo,hi)|index>=lo&&index<=hi);
            let high_start=n["hlA"].as_f64().unwrap_or(0.0)*1000.0+selected.map(|(lo,_)|(index-lo) as f64*70.0).unwrap_or(0.0);
            if let Some((_,last))=selected {
                if hs!="glow" {
                    let (hy,hh)=match hs{"marker"=>(0.86*(lh-0.42*lh),0.42*lh),"underline"=>(0.98*(lh-0.09*lh),0.09*lh),_=>(0.0,lh)};
                    let hw=ww+if index<last{gap}else{0.0};
                    scene.push_native(&format!("<Rect id={} x={{{}}} y={{{}}} width={{0}} height={{{}}} radius={{0}} fill={}><Animate prop=\"width\" from={{0}} to={{{}}} start=\"{high_start}ms\" duration=\"400ms\" ease=\"outCubic\"/></Rect>",q(&format!("{id}-highlight-{index}")),fs*0.05*s,hy*s,hh*s,q(hcolor),hw*s));
                }
            }
            emit(scene,&format!("{id}-word-{index}-text"),word,fs*0.05,0.0,ww+0.05,lh,fs,lh,weight,theme.tracking,color,"left",false);
            if selected.is_some() && matches!(hs,"glow"|"block") {
                let ink=if hs=="glow"{hcolor}else{highlight["ink"].as_str().unwrap_or(theme.ink)};
                let when=if hs=="block"{high_start+400.0*(1.0-0.45_f64.cbrt())}else{0.0};
                scene.push_native(&format!("<MotionBox id={} opacity={{0}}><Animate prop=\"opacity\" from={{0}} to={{1}} start=\"{when}ms\" duration=\"0ms\" ease=\"linear\"/>",q(&format!("{id}-word-{index}-highlight-ink"))));
                if hs=="glow"{scene.push_native(&format!(r#"<MotionBox id={} blur={{0}}><Animate prop="blur" from={{0}} to={{{}}} start="{high_start}ms" duration="400ms" ease="outCubic"/>"#,q(&format!("{id}-word-{index}-highlight-blur")),28.0*s));emit(scene,&format!("{id}-word-{index}-glow"),word,fs*0.05,0.0,ww+0.05,lh,fs,lh,weight,theme.tracking,highlight["glow"].as_str().unwrap_or(ink),"left",true);scene.push_native("</MotionBox>");}
                emit(scene,&format!("{id}-word-{index}-ink"),word,fs*0.05,0.0,ww+0.05,lh,fs,lh,weight,theme.tracking,ink,"left",true);
                scene.push_native("</MotionBox>");
            }
            scene.end_move();scene.push_native("</MotionBox>");xx+=ww+gap;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Theme;
    use serde_json::json;

    #[test]
    fn balanced_labels_use_hyphen_opportunities_without_splitting_tokens() {
        assert_eq!(balanced_label("Low-risk back-office AI","Geist",600,-0.005,20.0,192.0,23.0),
            "Low-risk back-\noffice AI");
        assert_eq!(balanced_label("Approval needed","Geist",600,-0.005,29.0,119.75,34.0),
            "Approval\nneeded");
        let approval=measure_label_style("Approval","Geist",600,-0.005,29.0,119.75,34.0/29.0);
        assert_eq!(approval.lines.len(),1);
        assert!((approval.width-126.06298828125).abs()<0.0001);
    }

    #[test]
    fn canvas_fit_retains_cross_hyphen_kerning_before_sibling_refit() {
        let scene=Scene::new("canvas-fitting".into(),1280,720,8000.0,0.0,Theme::get("blueprint").unwrap());
        // Original canvas: the positive cross-hyphen kern makes 21px miss the
        // 141.36px allowance; separately shaped back-/office incorrectly fits.
        let whole=oracle_width("back-office",&scene.theme,21.0);
        let split=oracle_width("back-",&scene.theme,21.0)+oracle_width("office",&scene.theme,21.0);
        assert!((whole-109.346924*1.03).abs()<0.0002);
        assert!((whole-split-0.21*1.03).abs()<0.0002);
        let mut nodes=vec![
            json!({"t":"row","grp":"item","label":"Low-risk back-office AI","icon":"bot","w":250,"h":72}),
            json!({"t":"row","grp":"item","label":"Independent review","icon":"search","w":250,"h":72})
        ];
        assert_eq!(source_label_fit(&scene,&nodes[0],"row",250.0,72.0),(20.0,2,2));
        lock_sibling_type(&scene,&mut nodes);
        for node in &nodes {assert_eq!(node["lock"]["fs"],18.0);}
    }


    #[test]
    fn flex_center_preserves_overflow_and_fixed_point_distribution() {
        assert_eq!(flex_center(168.0,3.0*76.0),-30.0);
        assert_eq!(flex_center(168.0,2.0*76.0),8.0);
        assert_eq!(flex_center(48.0,44.0),2.0);
        assert_eq!(flex_center(100.0,100.0+1.0/64.0),0.0);
        assert_eq!(flex_center(100.0,100.0+3.0/64.0),-1.0/64.0);
    }

    #[test]
    fn line_paint_snapping_precedes_fractional_reveal_motion() {
        // Original Chromium Keep/K stem at t=1.68: baseline raster row 576.
        // Its line box starts at 528.5, then node/word reveals add 3.6/11.13px.
        for step in 0..8 {
            let top=528.5+step as f64/8.0;
            assert_eq!((line_paint_origin(top)+3.6+11.13+32.0).round(),576.0);
        }
        assert_eq!((line_paint_origin(528.375)+3.6+11.13+32.0).round(),575.0);
        assert_eq!(line_paint_origin(-1.5),-1.0);
    }

    #[test]
    fn inline_intrinsic_widths_and_flex_gaps_use_css_layout_units() {
        let widths=[78.0979995727539,18.326000213623,106.1139907836914,28.0499992370605,107.609992980957];
        let gap=layout_length(34.0*0.24);
        assert_eq!(gap,8.15625);
        let widths=widths.map(inline_width);
        assert_eq!(widths,[78.109375,18.328125,106.125,28.0625,107.625]);
        let mut x=layout_length((1280.0-widths.iter().sum::<f64>()-gap*4.0)/2.0);
        let positions=widths.map(|width|{let left=x;x+=width+gap;left});
        assert_eq!(positions,[454.5625,540.828125,567.3125,681.59375,717.8125]);
        assert_eq!(layout_length(0.7),0.6875);
        assert_eq!(layout_length(1.0/128.0),0.0);
    }

    #[test]
    fn label_fit_boundary_and_sibling_type_caps() {
        assert_eq!(super::super::longest_word("iii WWW"),"iii");
        for name in ["daylight","midnight","blueprint","signal"] {
            let scene=Scene::new("fitting".into(),1280,720,8000.0,0.0,Theme::get(name).unwrap());
            let label="MMMM MMMM";
            let width=|one:f64|oracle_width(label,&scene.theme,36.0)*one/36.0/0.93+34.0;
            let mut node=json!({"t":"listRow","label":label,"fs":36,"marker":"dot","ms":10,"h":100});
            node["w"]=json!(width(30.25));
            assert_eq!(source_label_fit(&scene,&node,"listRow",width(30.25),100.0),(30.0,1,2));
            node["w"]=json!(width(29.75));
            assert_eq!(source_label_fit(&scene,&node,"listRow",width(29.75),100.0),(33.0,2,2));

            let narrow=oracle_width("MMMMMMMM",&scene.theme,36.0)*34.5/36.0/0.93+30.0;
            let mut nodes=vec![
                json!({"t":"list","grp":"item","label":"M","fs":36,"marker":"dot","ms":10,"w":1000,"h":84,"lock":{"fs":16}}),
                json!({"t":"list","grp":"item","label":"MMMMMMMM","fs":36,"marker":"dot","ms":10,"w":narrow,"h":84}),
                json!({"t":"rowBig","grp":"item","label":"M","fs":40,"w":1000,"h":100}),
                json!({"t":"list","grp":"other","label":"M","fs":16,"w":1000,"h":100}),
                json!({"t":"listRow","grp":"item","label":"M","fs":36,"w":1000,"h":100}),
                json!({"t":"pill","grp":"item","label":"M","w":1000,"h":100,"lock":{"fs":8}})
            ];
            lock_sibling_type(&scene,&mut nodes);
            for n in &nodes[..3]{assert_eq!(n["lock"]["fs"],34.0);}
            assert_eq!(nodes[0]["lock"]["room"],2);
            assert!(nodes[3]["lock"].is_null());
            assert_eq!(nodes[4]["lock"]["fs"],36.0);
            assert_eq!(source_label_fit(&scene,&nodes[5],"pill",1000.0,100.0),(20.0,1,1));
            for (kind,d) in TYPES.as_object().unwrap() {
                let Some(base)=d["fs"].as_f64() else{continue};
                let n=json!({"t":kind,"label":"M"});
                assert_eq!(source_label_fit(&scene,&n,kind,1000.0,500.0).0,base,"{name}/{kind}");
            }
        }
    }
}
