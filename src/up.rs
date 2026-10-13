use std::str::FromStr;

use serde::Serialize;
use synt_extension_api::Error;

const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Up {
    pub text: String,
    pub seconds: u64,
}

impl FromStr for Up {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Error> {
        let (_, since) = line.split_once(" up ").ok_or_else(|| unexpected(line))?;

        let parts: Vec<&str> = since
            .split(',')
            .map(str::trim)
            .take_while(|part| !part.contains("user") && !part.starts_with("load average"))
            .filter(|part| !part.is_empty())
            .collect();

        let seconds = parts
            .iter()
            .map(|part| seconds(part))
            .sum::<Option<u64>>()
            .filter(|_| !parts.is_empty())
            .ok_or_else(|| unexpected(line))?;

        Ok(Self {
            text: parts.join(", "),
            seconds,
        })
    }
}

fn seconds(part: &str) -> Option<u64> {
    if let Some((hours, minutes)) = part.split_once(':') {
        return Some(hours.parse::<u64>().ok()? * HOUR + minutes.parse::<u64>().ok()? * MINUTE);
    }

    let (count, unit) = part.split_once(char::is_whitespace)?;
    let count = count.parse::<u64>().ok()?;

    let scale = match unit.trim().trim_end_matches('s') {
        "day" => DAY,
        "hr" | "hour" => HOUR,
        "min" => MINUTE,
        "sec" => 1,
        _ => return None,
    };

    Some(count * scale)
}

fn unexpected(line: &str) -> Error {
    Error::new("unexpected_output", format!("not an uptime line: {line}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn up(line: &str) -> Up {
        line.parse().expect("valid uptime line")
    }

    #[test]
    fn reads_days_and_hours() {
        let up = up(" 10:15:02 up 12 days,  3:04,  2 users,  load average: 0.00, 0.01, 0.05");

        assert_eq!(up.text, "12 days, 3:04");
        assert_eq!(up.seconds, 12 * DAY + 3 * HOUR + 4 * MINUTE);
    }

    #[test]
    fn reads_minutes_only() {
        assert_eq!(
            up(" 10:15:02 up 7 min,  0 users,  load average: 0.10, 0.20, 0.30").seconds,
            7 * MINUTE
        );
    }

    #[test]
    fn reads_one_day_and_minutes() {
        let up = up(" 10:15:02 up 1 day, 10 min,  1 user,  load average: 0.00, 0.00, 0.00");

        assert_eq!(up.seconds, DAY + 10 * MINUTE);
    }

    #[test]
    fn reads_hours_and_minutes() {
        assert_eq!(
            up("10:15  up  2:41, 3 users, load averages: 1.10 1.20 1.30").seconds,
            2 * HOUR + 41 * MINUTE
        );
    }

    #[test]
    fn reads_macos_units() {
        assert_eq!(
            up("10:15  up 3 days, 2 hrs, 1 user, load averages: 1 1 1").seconds,
            3 * DAY + 2 * HOUR
        );
        assert_eq!(
            up("10:15  up 42 secs, 1 user, load averages: 1 1 1").seconds,
            42
        );
    }

    #[test]
    fn reads_busybox_without_users() {
        assert_eq!(
            up(" 10:15:02 up 5 days,  1:02,  load average: 0.00, 0.00, 0.00").seconds,
            5 * DAY + HOUR + 2 * MINUTE
        );
    }

    #[test]
    fn rejects_other_text() {
        assert!("bash: uptime: command not found".parse::<Up>().is_err());
        assert!(" 10:15:02 up soon, 1 user".parse::<Up>().is_err());
    }
}
