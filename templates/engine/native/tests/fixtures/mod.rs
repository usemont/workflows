use serde_json::{json, Value};

pub const IMAGE_SOURCE:&str="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='64' height='48'%3E%3Cpath d='M0 0H64V48H0Z' fill='red'/%3E%3C/svg%3E";

// Grammar examples adapted from engine/fixtures/{v96_lists_tones,check_verbs,story_morph}.json
// in the supplied v9.12 archive. Timings are rebased for isolated native scene sampling.
pub fn scene(family: &str, variant: &str) -> Value {
    let mut v = json!({"start":0,"end":24,"template":family,"variant":variant,"title":"Human expertise for specialist work"});
    match family {
        "title" => {
            v["subtitle"] = json!("Keep a human in control");
            v["icon"] = json!("expert");
        }
        "statement" => {
            v["text"] = json!("Use appropriate expertise and retain human control");
            v["highlight"] = json!("human control");
            v["highlightAt"] = json!(2);
            v["by"] = json!("Human oversight");
        }
        "illustration" => {
            v["text"] = json!("Give work back for human review");
            v["picture"] = json!({"layout":"row","items":[{"base":"bot"},{"base":"human"}],"links":["arrow"],"beats":[{"at":2,"items":[1],"links":[0]}],"steps":[{"at":5,"items":[{"base":"bot"},{"base":"human","mod":"check"}]}]});
        }
        "media" => {
            v["image"] = json!(IMAGE_SOURCE);
            v["subtitle"] = json!("Review the original source");
        }
        "action" => {
            v["action"] = json!({"subject":{"kind":"document","lines":5},"steps":[{"do":"compare","by":{"icon":"human","label":"You"},"against":{"icon":"book","label":"Source"},"finds":[3],"at":0.9},{"do":"approve","at":6}]});
        }
        "story" => {
            v["story"] = story();
        }
        _ => {
            let n = if family == "framework" { 3 } else { 2 };
            let mut items = Vec::new();
            for i in 0..n {
                let mut item = json!({"label":if i==0{"Approval needed"}else if i==1{"Low-risk back-office AI"}else{"Independent review"},"icon":if i==0{"approver"}else if i==1{"bot"}else{"search"},"at":1.0+i as f64*3.0,"tone":if i==0{"warning"}else{"accent"}});
                if matches!(family, "comparison" | "categories" | "tool-selection") {
                    item["details"] = json!([if i == 0 {
                        "Check correctness"
                    } else {
                        "Retain responsibility"
                    }]);
                }
                if family == "principle" {
                    item["rel"] = json!(if i == 0 { "example" } else { "because" });
                }
                items.push(item);
            }
            v["items"] = json!(items);
            if matches!(
                family,
                "framework" | "tool-selection" | "rule-exception" | "principle"
            ) {
                v["focus"] = json!({"label":"AI use at work","icon":"branch","at":0.3});
            }
            if family == "list" {
                v["takeaway"] = json!({"label":"You remain responsible","at":10,"tone":"warning"});
            }
        }
    }
    v
}
pub fn story() -> Value {
    json!({"entities":{"g":{"label":"AI guidance","icon":"document"},"law":{"label":"Legal advice","icon":"legal","who":"Not a solicitor"},"med":{"label":"Medical","icon":"health","who":"Not a clinician"},"fin":{"label":"Finance","icon":"money","who":"Not an accountant"},"eng":{"label":"Engineering","icon":"gear","who":"Not an engineer"},"exp":{"label":"Expert","icon":"expert","kind":"actor"},"end":{"label":"Bring in expertise","sub":"Keep a human in control","kind":"conclusion"}},"beats":[{"rel":"rule","on":"g","at":0.6,"set":{"g":"unknown"},"setAt":2.4},{"rel":"example","of":"g","to":"law","at":6.3,"set":{"law":"blocked"},"setAt":9.6},{"rel":"also","of":"law","to":["med","fin","eng"],"at":13},{"rel":"so","by":"exp","fixes":["law","med","fin","eng"],"into":"end","at":17.3,"subAt":20}]})
}
