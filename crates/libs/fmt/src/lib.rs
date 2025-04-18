#![doc = include_str!("../readme.md")]

use std::io::Write;
use std::process::*;

pub fn rust(input: String, config: &str) -> String {
    let mut cmd = Command::new("rustfmt");
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::null());

    if !config.is_empty() {
        cmd.arg("--config");
        cmd.arg(config);
    }

    let Ok(mut child) = cmd.spawn() else {
        return input;
    };

    let Some(mut stdin) = child.stdin.take() else {
        return input;
    };

    if stdin.write_all(input.as_bytes()).is_err() {
        return input;
    }

    drop(stdin);

    let Ok(output) = child.wait_with_output() else {
        return input;
    };

    if !output.status.success() {
        return input;
    }

    if let Ok(result) = String::from_utf8(output.stdout) {
        result
    } else {
        input
    }
}

// TODO: remove this as the riddle formatter does pretty printing
pub fn riddle(input: String, config: &str) -> String {
    let rx = regex::Regex::new(r"(?m)^(\s*)(class|interface)\s").unwrap();

    let input = rx
        .replace_all(&input, |caps: &regex::Captures| {
            format!("{}#[{}]\n{}trait ", &caps[1], &caps[2], &caps[1])
        })
        .to_string();

    let input = rust(input, config);

    let rx = regex::Regex::new(r"(?m)^(\s*)#\[(class|interface)]\n\s+trait\s").unwrap();

    let input = rx
        .replace_all(&input, |caps: &regex::Captures| {
            format!("{}{} ", &caps[1], &caps[2])
        })
        .to_string();

    input
}
