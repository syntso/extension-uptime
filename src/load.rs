use std::str::FromStr;

use serde::Serialize;
use synt_extension_api::Error;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Load {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

impl FromStr for Load {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Error> {
        let averages = line
            .split_whitespace()
            .take(3)
            .map(str::parse::<f64>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| unexpected(line))?;

        let [one, five, fifteen] = averages[..] else {
            return Err(unexpected(line));
        };

        Ok(Self { one, five, fifteen })
    }
}

fn unexpected(line: &str) -> Error {
    Error::new(
        "unexpected_output",
        format!("not a /proc/loadavg line: {line}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_three_averages() {
        let load: Load = "0.52 0.58 0.59 1/467 13817".parse().expect("valid");

        assert_eq!(
            load,
            Load {
                one: 0.52,
                five: 0.58,
                fifteen: 0.59
            }
        );
    }

    #[test]
    fn rejects_short_lines() {
        assert!("0.52 0.58".parse::<Load>().is_err());
    }

    #[test]
    fn rejects_text() {
        assert!("cat: /proc/loadavg: No such file".parse::<Load>().is_err());
    }
}
