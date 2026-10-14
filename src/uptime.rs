use synt_extension_api::{ActionInfo, Error, Extension, Risk, register_extension};

use crate::check::check;
use crate::view::tree;

const CHECK: &str = "uptime.check";
const VIEW: &str = "uptime";

struct Uptime;

impl Extension for Uptime {
    fn describe() -> Vec<ActionInfo> {
        let check = ActionInfo::new(CHECK, "Check uptime and load", Risk::Read).takes_target();

        vec![check]
    }

    fn run(action: &str, target: Option<&str>, _params: &str) -> Result<String, Error> {
        if action != CHECK {
            return Err(Error::new("unknown_action", action));
        }

        let target = target.ok_or_else(|| Error::new("missing_target", "pick a host to check"))?;
        let report = check(target)?;

        serde_json::to_string(&report).map_err(|error| Error::new("internal", error.to_string()))
    }

    fn render(view: &str, target: Option<&str>, _params: &str) -> Result<String, Error> {
        if view != VIEW {
            return Err(Error::new("unknown_view", view));
        }

        let target = target.ok_or_else(|| Error::new("missing_target", "pick a host to show"))?;
        let report = check(target)?;

        Ok(tree(target, &report).to_json())
    }
}

register_extension!(Uptime);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_one_read_action_with_a_target() {
        let actions = Uptime::describe();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].id(), CHECK);
        assert_eq!(actions[0].risk(), Risk::Read);
    }

    #[test]
    fn refuses_unknown_actions() {
        let error = Uptime::run("uptime.reboot", Some("web-1"), "{}").expect_err("unknown");

        assert_eq!(error.code(), "unknown_action");
    }

    #[test]
    fn needs_a_target() {
        let error = Uptime::run(CHECK, None, "{}").expect_err("no target");

        assert_eq!(error.code(), "missing_target");
    }

    #[test]
    fn renders_only_its_own_view_and_only_for_a_host() {
        let other = Uptime::render("disk", Some("web-1"), "{}").expect_err("unknown view");
        let untargeted = Uptime::render(VIEW, None, "{}").expect_err("no target");

        assert_eq!(other.code(), "unknown_view");
        assert_eq!(untargeted.code(), "missing_target");
    }
}
