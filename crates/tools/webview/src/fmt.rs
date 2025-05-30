use std::io::Write;

pub fn fmt(tokens: &str) -> String {
    let mut cmd = std::process::Command::new("rustfmt");
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::null());

    let tokens = tokens.to_string();

    let Ok(mut child) = cmd.spawn() else {
        return tokens;
    };

    let Some(mut stdin) = child.stdin.take() else {
        return tokens;
    };

    if stdin.write_all(tokens.as_bytes()).is_err() {
        return tokens;
    }

    drop(stdin);

    let Ok(output) = child.wait_with_output() else {
        return tokens;
    };

    if !output.status.success() {
        return tokens;
    }

    if let Ok(result) = String::from_utf8(output.stdout) {
        result
    } else {
        tokens
    }
}
