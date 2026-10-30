mod fixtures;
use mont_native_explainer_templates::{compile_video, manifest};
use serde_json::json;

#[test]
fn every_declared_variant_is_editable_at_both_aspect_ratios_in_every_theme() {
    let grammar = manifest();
    for (family, spec) in grammar["templates"].as_object().unwrap() {
        for variant in spec["variants"].as_array().unwrap() {
            for theme in ["daylight", "midnight", "blueprint", "signal"] {
                for (w, h) in [(1920, 1080), (1080, 1920)] {
                    let scene = fixtures::scene(family, variant.as_str().unwrap());
                    let spans = compile_video(&json!({"theme":theme,"scenes":[scene]}), w, h)
                        .unwrap_or_else(|e| panic!("{family}/{variant}/{theme}/{w}x{h}: {e}"));
                    assert_eq!(spans.len(), 1);
                    assert_eq!(spans[0]["placement"]["durationMs"], 24000);
                    assert_eq!(spans[0]["capability"]["kind"], "programmatic.scene");
                    let state = &spans[0]["capability"]["state"];
                    assert_eq!(state["frame"]["width"], w);
                    assert_eq!(state["frame"]["height"], h);
                    assert!(!state["variables"].as_object().unwrap().is_empty());
                    assert!(state["sourceTsx"].as_str().is_some());
                    assert_eq!(spans,compile_video(&json!({"theme":theme,"scenes":[fixtures::scene(family,variant.as_str().unwrap())]}),w,h).unwrap());
                }
            }
        }
    }
}
#[test]
fn preserves_lists_caption_placement_and_custom_theme_validation() {
    let mut scene = fixtures::scene("list", "bullets");
    scene["items"] = json!((0..8)
        .map(|i| json!({"label":format!("Semantic point {i}"),"at":1+i*2}))
        .collect::<Vec<_>>());
    let spans=compile_video(&json!({"themeTokens":{"accent":"#123456","fontFamily":"Inter","headWeight":600,"radius":8},"scenes":[scene],"captions":[{"text":"Caption retained","start":1.2,"end":3.4}]}),1080,1920).unwrap();
    let values = spans[0]["capability"]["state"]["variables"]
        .as_object()
        .unwrap();
    for i in 0..8 {
        assert!(values
            .values()
            .any(|s| s == &json!(format!("Semantic point {i}"))));
    }
    assert_eq!(spans[1]["placement"]["startMs"], 1200);
    assert_eq!(spans[1]["placement"]["durationMs"], 2200);
    assert_eq!(spans[1]["placement"]["layer"], 100);
    let base = json!({"scenes":[fixtures::scene("title","text")]});
    for tokens in [
        json!({"accent":"javascript:bad"}),
        json!({"radius":-1}),
        json!({"headWeight":950}),
        json!({"fontFamily":""}),
        json!({"unrecognized":true}),
    ] {
        let mut v = base.clone();
        v["themeTokens"] = tokens;
        assert!(compile_video(&v, 1920, 1080).is_err());
    }
}
#[test]
fn rejects_unsupported_grammar_and_invalid_temporal_state() {
    let base = json!({"scenes":[fixtures::scene("title","text")]});
    for (key, value) in [
        ("variant", json!("unknown")),
        ("start", json!(-1)),
        ("end", json!(0)),
        ("html", json!("<div/>")),
    ] {
        let mut v = base.clone();
        v["scenes"][0][key] = value;
        assert!(compile_video(&v, 1920, 1080).is_err(), "{key}");
    }
    let mut action = fixtures::scene("action", "stage");
    action["action"]["steps"][1]["at"] = json!(1);
    assert!(compile_video(&json!({"scenes":[action]}), 1920, 1080).is_err());
    let mut story = fixtures::scene("story", "morph");
    story["story"]["beats"][1]["of"] = json!("missing");
    assert!(compile_video(&json!({"scenes":[story]}), 1920, 1080).is_err());
    let mut pict = fixtures::scene("illustration", "picture");
    pict["picture"]["beats"][0]["items"] = json!([99]);
    assert!(compile_video(&json!({"scenes":[pict]}), 1920, 1080).is_err());
    assert!(compile_video(&base, 0, 1080).is_err());
}

#[test]
fn subtitle_timing_optional_details_and_spotlight_remain_strict() {
    for family in ["title", "media"] {
        let mut scene = fixtures::scene(family, if family == "title" { "text" } else { "frame" });
        scene["start"] = json!(10);
        scene["end"] = json!(20);
        scene["subtitle"] = json!("Spoken later");
        scene["subtitleAt"] = json!(14);
        assert!(compile_video(&json!({"scenes":[scene.clone()]}), 1920, 1080).is_ok());
        for at in [json!(9), json!(20), json!("14"), json!(null)] {
            let mut invalid = scene.clone();
            invalid["subtitleAt"] = at;
            assert!(compile_video(&json!({"scenes":[invalid]}), 1920, 1080).is_err());
        }
        scene.as_object_mut().unwrap().remove("subtitle");
        assert!(compile_video(&json!({"scenes":[scene]}), 1920, 1080).is_err());
    }
    for family in ["comparison", "categories", "tool-selection"] {
        let variant = match family {
            "comparison" => "versus",
            "categories" => "shelves",
            _ => "switchboard",
        };
        for details in [None, Some(json!([]))] {
            let mut scene = fixtures::scene(family, variant);
            for item in scene["items"].as_array_mut().unwrap() {
                item.as_object_mut().unwrap().remove("detailsAt");
                item.as_object_mut().unwrap().remove("details");
                if let Some(details) = &details {
                    item["details"] = details.clone();
                }
            }
            let result = compile_video(&json!({"scenes":[scene]}), 1920, 1080);
            assert_eq!(result.is_ok(), family != "tool-selection", "{family}: {result:?}");
        }
    }
    let mut scene = fixtures::scene("list", "spotlight");
    scene["intro"] = json!({"picture":{"layout":"single","items":[{"base":"book"}]},
        "label":"Introduction"});
    scene["items"][0]["picture"] = json!({"layout":"row","items":[null,{"base":"bot"}]});
    assert!(compile_video(&json!({"scenes":[scene.clone()]}), 1920, 1080).is_ok());
    for (pointer, value) in [
        ("/intro/unknown", json!(true)),
        ("/intro/picture/unknown", json!(true)),
        ("/items/0/picture/unknown", json!(true)),
        ("/items/0/picture/items/1/unknown", json!(true)),
        ("/items/0/picture/callouts", json!([{"text":"Invalid local target","at":2,"on":2}])),
        ("/intro/label", json!(null)),
    ] {
        let mut invalid = scene.clone();
        let (parent, key) = pointer.rsplit_once('/').unwrap();
        invalid.pointer_mut(parent).unwrap().as_object_mut().unwrap().insert(key.into(), value);
        assert!(compile_video(&json!({"scenes":[invalid]}), 1920, 1080).is_err(), "{pointer}");
    }
    scene["variant"] = json!("cards");
    assert!(compile_video(&json!({"scenes":[scene]}), 1920, 1080).is_err());
    let mut focus = fixtures::scene("framework", "hub");
    focus["focus"]["tone"] = json!("warning");
    assert!(compile_video(&json!({"scenes":[focus.clone()]}), 1920, 1080).is_ok());
    focus["focus"]["tone"] = json!("unrecognized");
    assert!(compile_video(&json!({"scenes":[focus]}), 1920, 1080).is_err());
}
