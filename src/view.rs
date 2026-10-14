use synt_extension_api::serde_json::json;
use synt_extension_api::view::Node;

use crate::report::Report;

pub fn tree(target: &str, report: &Report) -> Node {
    let load = &report.load;

    Node::column([
        Node::title(target),
        Node::key_values([
            ("Up", json!(report.up.text)),
            ("Up seconds", json!(report.up.seconds)),
            ("Load, 1 min", json!(load.one)),
            ("Load, 5 min", json!(load.five)),
            ("Load, 15 min", json!(load.fifteen)),
        ]),
        Node::line_chart([(
            "load average",
            vec![(1.0, load.one), (5.0, load.five), (15.0, load.fifteen)],
        )]),
        Node::button("Check again", "uptime.check").target(target),
    ])
}

#[cfg(test)]
mod tests {
    use synt_extension_api::serde_json::json;

    use super::tree;
    use crate::load::Load;
    use crate::report::Report;
    use crate::up::Up;

    #[test]
    fn shows_uptime_load_and_a_chart_for_the_host() {
        let report = Report {
            up: Up {
                text: "3 days, 4:05".to_owned(),
                seconds: 273_900,
            },
            load: Load {
                one: 0.5,
                five: 0.25,
                fifteen: 0.125,
            },
        };

        let tree = tree("web-1", &report).into_value();

        assert_eq!(tree["children"][0]["text"], "web-1");
        assert_eq!(
            tree["children"][1]["items"][0],
            json!({ "key": "Up", "value": "3 days, 4:05" })
        );
        assert_eq!(
            tree["children"][2]["series"][0]["points"],
            json!([[1.0, 0.5], [5.0, 0.25], [15.0, 0.125]])
        );
        assert_eq!(tree["children"][3]["action"], "uptime.check");
        assert_eq!(tree["children"][3]["target"], "web-1");
    }
}
