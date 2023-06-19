pub fn try_format(tokens: String) -> String {
    use std::io::Write;

    let Ok(mut child) = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn() else {
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
