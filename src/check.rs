use synt_extension_api::Error;

use crate::load::Load;
use crate::remote;
use crate::report::Report;
use crate::up::Up;

const COMMAND: &str = "uptime && cat /proc/loadavg";

pub fn check(target: &str) -> Result<Report, Error> {
    let stdout = remote::run(target, COMMAND)?;

    parse(&stdout)
}

fn parse(stdout: &str) -> Result<Report, Error> {
    let mut lines = stdout.lines().filter(|line| !line.trim().is_empty());

    let uptime = lines.next().ok_or_else(|| missing("uptime"))?;
    let loadavg = lines.next().ok_or_else(|| missing("/proc/loadavg"))?;

    Ok(Report {
        up: uptime.parse::<Up>()?,
        load: loadavg.parse::<Load>()?,
    })
}

fn missing(what: &str) -> Error {
    Error::new("unexpected_output", format!("{what} printed nothing"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_both_lines() {
        let stdout = " 10:15:02 up 3 days,  4:05,  1 user,  load average: 0.08, 0.03, 0.01\n\
                      0.08 0.03 0.01 1/131 4242\n";

        let report = parse(stdout).expect("valid output");

        assert_eq!(report.up.text, "3 days, 4:05");
        assert_eq!(report.up.seconds, 3 * 86_400 + 4 * 3_600 + 5 * 60);
        assert!((report.load.one - 0.08).abs() < f64::EPSILON);
    }

    #[test]
    fn fails_without_loadavg() {
        let error = parse(" 10:15:02 up 5 min,  0 users,  load average: 0.00, 0.00, 0.00\n")
            .expect_err("one line");

        assert_eq!(error.code(), "unexpected_output");
    }
}
