use serde_json::json;
use synt_extension_api::Error;

use crate::output::Output;

pub fn run(target: &str, command: &str) -> Result<String, Error> {
    let params = json!({ "command": command }).to_string();
    let reply = synt_extension_api::call("ssh.exec", Some(target), &params)?;

    let output: Output = serde_json::from_str(&reply)
        .map_err(|error| Error::new("unexpected_output", format!("ssh.exec replied {error}")))?;

    output.into_stdout(command)
}
