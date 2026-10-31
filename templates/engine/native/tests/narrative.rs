use mont_native_explainer_templates::compile_video;
use serde_json::json;

#[test]
fn action_rejects_invalid_subject_kinds_and_line_settings() {
    for subject in [json!({"kind":"unknown"}), json!({"lines":"6"}), json!([])] {
        assert!(compile_video(&json!({"scenes":[{
            "start":0,"end":12,"template":"action","variant":"stage",
            "action":{"subject":subject,"steps":[{"do":"draft","by":"bot"}]}
        }]}), 1920, 1080).is_err());
    }
}


#[test]
fn action_normalizes_actor_identity_before_holder_checks_and_draft_timing() {
    for (alias, icon) in [("agent", "bot"), ("ai", "spark"), ("computer", "machine"), ("laptop", "machine")] {
        assert!(compile_video(&json!({"scenes":[{
            "start":0,"end":6,"template":"action","variant":"stage",
            "action":{"subject":{"kind":"document","lines":6},"steps":[
                {"do":"draft","by":alias},{"do":"inspect","by":icon}
            ]}
        }]}), 1920, 1080).is_ok(), "{alias} must use {icon}'s identity and machine duration");
        assert!(compile_video(&json!({"scenes":[{
            "start":0,"end":12,"template":"action","variant":"stage",
            "action":{"steps":[{"do":"pass","from":alias,"to":icon}]}
        }]}), 1920, 1080).is_err(), "{alias} and {icon} are the same pass holder");
    }
}

