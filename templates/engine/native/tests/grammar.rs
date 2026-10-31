mod fixtures;
use mont_native_explainer_templates::compile_video;
use serde_json::json;

#[test]
fn rejects_invalid_custom_theme_tokens() {
    let base = json!({"scenes":[fixtures::scene("title","text")]});
    for tokens in [
        json!({"accent":"javascript:bad"}),
        json!({"radius":-1}),
        json!({"headWeight":950}),
        json!({"fontFamily":""}),
        json!({"fontFamily":"Inter"}),
        json!({"fontFamily":"missing-font"}),
        json!({"unrecognized":true}),
    ] {
        let mut v = base.clone();
        v["themeTokens"] = tokens;
        assert!(compile_video(&v, 1920, 1080).is_err());
    }
}
#[test]
fn captions_preserve_intervals_and_clip_exit_at_video_end() {
    for theme in ["daylight", "midnight", "blueprint", "signal"] {
        for (w,h) in [(1280,720),(720,1280)] {
            let spans=compile_video(&json!({"theme":theme,"scenes":[fixtures::scene("title","text")],"captions":[{"text":"Wide iii","start":22,"end":23},{"text":"Last word","start":23,"end":24}]}),w,h).unwrap();
            let first = spans.iter().find(|span| span["id"] == "explainer-caption-0").unwrap();
            let last = spans.iter().find(|span| span["id"] == "explainer-caption-1").unwrap();
            assert_eq!(first["placement"]["startMs"],21980);
            assert_eq!(first["placement"]["durationMs"],1020);
            assert_eq!(last["placement"]["startMs"],23000);
            assert_eq!(last["placement"]["durationMs"],1000);
        }
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
