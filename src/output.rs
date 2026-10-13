use serde::Deserialize;
use synt_extension_api::Error;

#[derive(Debug, Deserialize)]
pub struct Output {
    exit: Option<u32>,
    signal: Option<String>,
    stdout: String,
    stderr: String,
}

impl Output {
    pub fn into_stdout(self, command: &str) -> Result<String, Error> {
        if self.exit == Some(0) {
            return Ok(self.stdout);
        }

        let status = match (self.exit, self.signal) {
            (Some(code), _) => format!("exited with {code}"),
            (None, Some(signal)) => format!("was killed by {signal}"),
            (None, None) => "stopped without a status".to_owned(),
        };

        let stderr = self.stderr.trim();

        Err(Error::new(
            "command_failed",
            format!("`{command}` {status}: {stderr}"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn output(exit: Option<u32>, signal: Option<&str>) -> Output {
        Output {
            exit,
            signal: signal.map(str::to_owned),
            stdout: "out".to_owned(),
            stderr: "no such file\n".to_owned(),
        }
    }

    #[test]
    fn returns_stdout_on_success() {
        assert_eq!(
            output(Some(0), None)
                .into_stdout("uptime")
                .expect("success"),
            "out"
        );
    }

    #[test]
    fn reports_the_exit_code() {
        let error = output(Some(1), None)
            .into_stdout("cat /proc/loadavg")
            .expect_err("failed");

        assert_eq!(error.code(), "command_failed");
        assert_eq!(
            error.message(),
            "`cat /proc/loadavg` exited with 1: no such file"
        );
    }

    #[test]
    fn reports_the_signal() {
        let error = output(None, Some("KILL"))
            .into_stdout("uptime")
            .expect_err("killed");

        assert_eq!(error.message(), "`uptime` was killed by KILL: no such file");
    }
}
