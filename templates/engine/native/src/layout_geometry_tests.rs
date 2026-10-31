use super::*;
#[test] fn picture_slot_geometry_is_source_exact(){
 let(t,_,e)=picture_geometry("hub",4,false,false).unwrap();assert_eq!(t,vec![(320.0,200.0,140.0),(85.0,200.0,104.0),(555.0,92.0,104.0),(555.0,308.0,104.0)]);assert_eq!(e,vec![(0,1,"l","r"),(0,2,"r","l"),(0,3,"r","l")]);
 let(t,f,e)=picture_geometry("boundary",2,true,true).unwrap();assert_eq!(t,vec![(200.0,100.0,112.0),(200.0,260.0,112.0),(200.0,570.0,116.0)]);assert_eq!(f,Some((84.0,16.0,232.0,328.0)));assert_eq!(e,vec![(2,usize::MAX,"t","b")]);
}
#[test] fn all_picture_counts_are_explicit(){for(name,min,max)in[("single",1,1),("row",2,3),("hub",3,5),("tree",3,4),("cycle",3,3),("boundary",1,2)]{for n in 0..7{assert_eq!(picture_geometry(name,n,false,false).is_ok(),n>=min&&n<=max,"{name}/{n}");}}assert!(picture_geometry("invented",3,false,false).is_err());}
#[test] fn tall_box_only_transposes_supported_picture_layouts(){let(w,up)=picture_size(&json!({"layout":"row"}),400.0,600.0);assert!(up);assert_eq!(w,375.0);let(w,up)=picture_size(&json!({"layout":"tree"}),400.0,600.0);assert!(!up);assert_eq!(w,400.0);}
#[test] fn callouts_replace_conflicting_picture_highlights(){let beats=picture_beats(&json!({"beats":[{"at":10.0,"items":[1],"links":[]}]}),&json!({"start":0,"end":20,"callouts":[{"at":10.1,"on":0,"text":"AI"}]}));assert_eq!(beats.len(),1);assert_eq!(beats[0]["items"],json!([0]));assert_eq!(beats[0]["at"],10.1);}
#[test]
fn principle_relations_select_the_source_layout_and_choreography() {
    let brands=BTreeMap::new();
    for (requested,relations,effective) in [
        ("spotlight",["example","because"],"chain"),
        ("satellites",["because","because"],"chain"),
        ("chain",["follows","follows"],"satellites"),
    ] {
        let doc=json!({"template":"principle","variant":requested,"title":"Principle",
            "start":0,"end":8,"focus":{"label":"Focus","icon":"user"},
            "items":[{"label":"First","icon":"check","rel":relations[0]},
                     {"label":"Second","icon":"check","rel":relations[1]}]});
        let theme=crate::scene::Theme::get("daylight").unwrap();
        let mut actual=Scene::new("actual".into(),1280,720,8000.0,0.0,theme);
        render(&mut actual,&doc,&brands).unwrap();
        let mut expected_doc=doc.clone();
        expected_doc["variant"]=json!(effective);
        let mut expected=Scene::new("expected".into(),1280,720,8000.0,0.0,theme);
        render(&mut expected,&expected_doc,&brands).unwrap();
        assert_eq!(actual.transition_nodes,expected.transition_nodes,
            "{requested}/{relations:?} must have the same geometry and seeded timing as {effective}");
    }
}
#[test]
fn source_variant_fallbacks_preserve_geometry_and_seeded_timing() {
    let brands=BTreeMap::new();
    let theme=crate::scene::Theme::get("daylight").unwrap();
    let mut docs=vec![
        json!({"template":"title","variant":"emblem","title":"No icon","start":0,"end":8}),
        json!({"template":"title","variant":"editorial","title":"No icon","start":0,"end":8}),
        json!({"template":"list","variant":"cards","title":"Missing icon","start":0,"end":8,
            "items":[{"label":"One","icon":"check"},{"label":"Two"}]}),
        json!({"template":"list","variant":"spotlight","title":"Missing time","start":0,"end":8,
            "items":[{"label":"One","icon":"check","at":1},{"label":"Two","icon":"check"}]}),
        json!({"template":"list","variant":"spotlight","title":"Missing icon","start":0,"end":8,
            "items":[{"label":"One","at":1},{"label":"Two","icon":"check","at":2}]}),
    ];
    docs.push(json!({"template":"list","variant":"cards","title":"Too many cards","start":0,"end":8,
        "items":(0..8).map(|i|json!({"label":format!("Item {i}"),"icon":"check","at":i+1})).collect::<Vec<_>>()}));
    for doc in docs {
        crate::grammar::scene(&doc).unwrap();
        let mut expected_doc=doc.clone();
        expected_doc["variant"]=json!(if doc["template"]=="title" {"text"} else {"bullets"});
        for (w,h) in [(1280,720),(720,1280)] {
            let mut actual=Scene::new("actual".into(),w,h,8000.0,0.0,theme);
            let mut expected=Scene::new("expected".into(),w,h,8000.0,0.0,theme);
            render(&mut actual,&doc,&brands).unwrap();
            render(&mut expected,&expected_doc,&brands).unwrap();
            assert_eq!(actual.transition_nodes,expected.transition_nodes,"{doc}");
        }
    }
}
#[test]
fn rule_roles_reorder_the_first_default_before_layout_and_timing() {
    let brands=BTreeMap::new();
    let theme=crate::scene::Theme::get("daylight").unwrap();
    for role in ["default","rule"] {
        for variant in ["branch","detour"] {
            let doc=json!({"template":"rule-exception","variant":variant,"title":"Rule","start":0,"end":8,
                "focus":{"label":"Decision","icon":"user"},
                "items":[{"label":"Exception","icon":"check","tone":"warning"},
                    {"label":"Default","icon":"check","role":role},
                    {"label":"Another rule","icon":"check","role":"default"}]});
            crate::grammar::scene(&doc).unwrap();
            let mut expected_doc=doc.clone();
            expected_doc["items"]=json!([doc["items"][1],doc["items"][0],doc["items"][2]]);
            for (w,h) in [(1280,720),(720,1280)] {
                let mut actual=Scene::new("actual".into(),w,h,8000.0,0.0,theme);
                let mut expected=Scene::new("expected".into(),w,h,8000.0,0.0,theme);
                render(&mut actual,&doc,&brands).unwrap();
                render(&mut expected,&expected_doc,&brands).unwrap();
                assert_eq!(actual.transition_nodes,expected.transition_nodes,"{variant}/{role}");
                assert_eq!(actual.transition_nodes.iter().find(|n|n["t"]=="rowRule").unwrap()["label"],"Default");
            }
        }
    }
}
#[test]
fn normalization_keeps_closed_icon_time_and_role_validation() {
    let mut doc=json!({"template":"list","variant":"spotlight","title":"List","start":0,"end":8,
        "items":[{"label":"One"},{"label":"Two"}]});
    crate::grammar::scene(&doc).unwrap();
    doc["items"][0]["icon"]=json!("invented");
    assert!(crate::grammar::scene(&doc).is_err());
    doc["items"][0].as_object_mut().unwrap().remove("icon");
    doc["items"][0]["at"]=json!("invalid");
    assert!(crate::grammar::scene(&doc).is_err());
    doc["items"][0].as_object_mut().unwrap().remove("at");
    doc["items"][0]["role"]=json!("default");
    assert!(crate::grammar::scene(&doc).is_err());
    doc["template"]=json!("rule-exception");doc["variant"]=json!("branch");
    doc["focus"]=json!({"label":"Focus","icon":"user"});
    doc["items"][0]["icon"]=json!("check");doc["items"][1]["icon"]=json!("check");
    doc["items"][0]["role"]=json!("invented");
    assert!(crate::grammar::scene(&doc).is_err());
}
#[test]
fn principle_chain_uses_the_resolved_item_tone_not_the_relation_tone() {
    let brands=BTreeMap::new();
    let theme=crate::scene::Theme::get("daylight").unwrap();
    for (rel,tone,resolved,kind) in [
        ("so",None,"accent","itemSel"),
        ("so",Some("default"),"accent","itemSel"),
        ("so",Some("warning"),"warning","item"),
        ("because",Some("accent"),"accent","itemSel"),
    ] {
        let mut doc=json!({"template":"principle","variant":"chain","title":"Relations","start":0,"end":8,
            "focus":{"label":"Focus","icon":"user"},
            "items":[{"label":"Point","icon":"check","rel":rel}]});
        if let Some(tone)=tone {doc["items"][0]["tone"]=json!(tone);}
        crate::grammar::scene(&doc).unwrap();
        for (w,h) in [(1280,720),(720,1280)] {
            let mut scene=Scene::new("tone".into(),w,h,8000.0,0.0,theme);
            render(&mut scene,&doc,&brands).unwrap();
            let item=scene.transition_nodes.iter().find(|n|n["grp"]=="item").unwrap();
            assert_eq!(item["tone"],resolved);
            assert_eq!(item["t"],if h>w {"row"} else {kind});
        }
    }
}
#[test]
fn spotlight_picture_label_keeps_its_item_tone() {
    let doc=json!({"template":"list","variant":"spotlight","title":"Pictures","start":0,"end":8,
        "items":[{"label":"Warning","icon":"check","at":1,"tone":"warning",
            "picture":{"layout":"single","items":[{"base":"document"}]}},
            {"label":"Next","icon":"check","at":4}]});
    crate::grammar::scene(&doc).unwrap();
    let mut scene=Scene::new("spotlight".into(),1280,720,8000.0,0.0,crate::scene::Theme::get("daylight").unwrap());
    render(&mut scene,&doc,&BTreeMap::new()).unwrap();
    let label=scene.transition_nodes.iter().find(|n|n["t"]=="spotLabel").unwrap();
    assert_eq!(label["tone"],"warning");
    assert_eq!(label["at"],1);
}
#[test]
fn source_strings_collapse_ecmascript_spaces_and_slice_utf16() {
    assert_eq!(source_string("already normalized",56),None);
    assert_eq!(source_string("\u{feff}\t A\u{a0}  B\r\n ",56),Some("A B".into()));
    assert_eq!(source_string("\u{85}word\u{85}",56),None);
    assert_eq!(source_string(&format!("{}😀tail","a".repeat(30)),32),Some(format!("{}😀","a".repeat(30))));
    // A JavaScript slice through a surrogate pair renders as the replacement character.
    assert_eq!(source_string(&format!("{}😀tail","a".repeat(31)),32),Some(format!("{}\u{fffd}","a".repeat(31))));
    assert_eq!(source_string(" a b ",2),Some("a ".into()));
}
#[test]
fn source_text_normalization_precedes_geometry_and_seeded_timing() {
    let doc=json!({"template":"tool-selection","variant":"switchboard","title":format!("\u{feff}{}  ","Heading ".repeat(12)),
        "start":0,"end":8,"focus":{"label":"f".repeat(36),"icon":"user","brand":"  Acme\t Brand "},
        "items":[{"label":"i".repeat(36),"icon":"check",
            "details":["  A\t detailed\n label with more than 26 chars  "]},{"label":"Next","icon":"check","details":["Another mode"]}]});
    crate::grammar::scene(&doc).unwrap();
    let mut expected=doc.clone();
    expected["title"]=json!("Heading Heading Heading Heading Heading Heading Heading ");
    expected["focus"]["label"]=json!("f".repeat(32));
    expected["focus"]["brand"]=json!("Acme Brand");
    expected["items"][0]["label"]=json!("i".repeat(32));
    expected["items"][0]["details"][0]=json!("A detailed label with more");
    assert_eq!(normalized_strings(&doc).unwrap(),expected);
    for (w,h) in [(1280,720),(720,1280)] {
        let theme=crate::scene::Theme::get("daylight").unwrap();
        let mut actual=Scene::new("strings".into(),w,h,8000.0,0.0,theme);
        let mut reference=Scene::new("strings".into(),w,h,8000.0,0.0,theme);
        render(&mut actual,&doc,&BTreeMap::new()).unwrap();
        render_normalized(&mut reference,&expected,&BTreeMap::new()).unwrap();
        assert_eq!(actual.transition_nodes,reference.transition_nodes);
    }
    // Variant fallback must not normalize a second time and trim a slice-created space.
    let doc=json!({"template":"title","variant":"editorial","title":"Heading ".repeat(12),"start":0,"end":8});
    let mut expected=normalized_strings(&doc).unwrap();expected["variant"]=json!("text");
    let theme=crate::scene::Theme::get("daylight").unwrap();
    let mut actual=Scene::new("fallback".into(),1280,720,8000.0,0.0,theme);
    let mut reference=Scene::new("fallback".into(),1280,720,8000.0,0.0,theme);
    render(&mut actual,&doc,&BTreeMap::new()).unwrap();
    render_normalized(&mut reference,&expected,&BTreeMap::new()).unwrap();
    assert_eq!(actual.transition_nodes,reference.transition_nodes);
}
#[test]
fn source_string_limits_preserve_resolved_media_bytes_and_callout_order() {
    let data=format!("data:image/png;base64,{}","A".repeat(4096));
    let doc=json!({"template":"media","variant":"frame","title":"A title","image":data,
        "subtitle":"s".repeat(120),"alt":"a".repeat(180),"highlight":"h".repeat(70),"by":"b".repeat(60),"label":"l".repeat(60),
        "intro":{"label":"i".repeat(60)},"takeaway":{"label":"t".repeat(100)},
        "callouts":[{"text":" Later\t tag ","at":4},{"text":" Early  tag ","at":1}]});
    let normalized=normalized_strings(&doc).unwrap();
    for (field,len) in [("subtitle",96),("alt",140),("highlight",60),("by",48),("label",48)] {assert_eq!(normalized[field].as_str().unwrap().len(),len);}
    assert_eq!(normalized["intro"]["label"].as_str().unwrap().len(),40);
    assert_eq!(normalized["takeaway"]["label"].as_str().unwrap().len(),80);
    assert_eq!(normalized["image"],doc["image"]);
    assert_eq!(normalized["callouts"][0]["text"],"Early tag");
    assert_eq!(normalized["callouts"][1]["text"],"Later tag");
    let nested=json!({"template":"illustration","picture":{"callouts":[{"text":"  Nested\t tag ","at":1}]}});
    assert_eq!(normalized_strings(&nested).unwrap()["callouts"][0]["text"],"Nested tag");
}
