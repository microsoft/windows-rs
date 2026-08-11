use std::io::Write;

pub fn rust(tokens: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut command = std::process::Command::new("rustfmt");
    command
        .args(["--edition", "2024", "--config", "newline_style=Unix"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());
    let mut child = command.spawn()?;
    child.stdin.take().unwrap().write_all(tokens.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err("rustfmt failed".into());
    }
    Ok(tighten_macro_whitespace(&String::from_utf8(output.stdout)?))
}

fn tighten_macro_whitespace(src: &str) -> String {
    let pass1 = replace_outside_strings(
        src,
        &[(" :: ", "::"), (":: ", "::"), (" ::", "::"), (") (", ")(")],
    );
    apply_inside_macros(&pass1)
}

fn replace_outside_strings(src: &str, patterns: &[(&str, &str)]) -> String {
    let bytes = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut position = 0;
    while position < bytes.len() {
        if bytes[position] == b'"' {
            let start = position;
            position += 1;
            while position < bytes.len() {
                if bytes[position] == b'\\' && position + 1 < bytes.len() {
                    position += 2;
                    continue;
                }
                if bytes[position] == b'"' {
                    position += 1;
                    break;
                }
                position += 1;
            }
            out.push_str(&src[start..position]);
        } else {
            let start = position;
            while position < bytes.len() && bytes[position] != b'"' {
                position += 1;
            }
            let mut chunk = src[start..position].to_string();
            for (from, to) in patterns {
                chunk = chunk.replace(from, to);
            }
            out.push_str(&chunk);
        }
    }
    out
}

fn apply_inside_macros(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut position = 0;
    while position < bytes.len() {
        if let Some(open) = find_macro_invocation(bytes, position) {
            let header_start = macro_path_start(bytes, open.bang);
            out.push_str(&src[position..header_start]);
            let close = find_matching_delimiter(bytes, open.open_index, open.open, open.close);
            out.push_str(&tighten_macro_segment(&src[header_start..=close]));
            position = close + 1;
        } else {
            out.push_str(&src[position..]);
            break;
        }
    }
    out
}

struct MacroOpen {
    bang: usize,
    open_index: usize,
    open: u8,
    close: u8,
}

fn find_macro_invocation(bytes: &[u8], start: usize) -> Option<MacroOpen> {
    let mut position = start;
    while position + 4 < bytes.len() {
        if bytes[position] == b' '
            && bytes[position + 1] == b'!'
            && bytes[position + 2] == b' '
            && position > 0
        {
            let (open, close) = match bytes[position + 3] {
                b'(' => (b'(', b')'),
                b'[' => (b'[', b']'),
                _ => {
                    position += 1;
                    continue;
                }
            };
            let previous = bytes[position - 1];
            if !(previous.is_ascii_alphanumeric() || previous == b'_') {
                position += 1;
                continue;
            }
            return Some(MacroOpen {
                bang: position + 1,
                open_index: position + 3,
                open,
                close,
            });
        }
        position += 1;
    }
    None
}

fn macro_path_start(bytes: &[u8], bang: usize) -> usize {
    let mut position = bang;
    if position > 0 && bytes[position - 1] == b' ' {
        position -= 1;
    }
    while position > 0 {
        let character = bytes[position - 1];
        if character.is_ascii_alphanumeric() || character == b'_' || character == b':' {
            position -= 1;
        } else {
            break;
        }
    }
    position
}

fn find_matching_delimiter(bytes: &[u8], open_index: usize, open: u8, close: u8) -> usize {
    let mut depth = 0usize;
    let mut position = open_index;
    let mut in_string = false;
    while position < bytes.len() {
        let character = bytes[position];
        if in_string {
            if character == b'\\' && position + 1 < bytes.len() {
                position += 2;
                continue;
            }
            if character == b'"' {
                in_string = false;
            }
        } else if character == b'"' {
            in_string = true;
        } else if character == open {
            depth += 1;
        } else if character == close && depth > 0 {
            depth -= 1;
            if depth == 0 {
                return position;
            }
        }
        position += 1;
    }
    bytes.len().saturating_sub(1)
}

fn tighten_macro_segment(segment: &str) -> String {
    let bytes = segment.as_bytes();
    let mut out = String::with_capacity(segment.len());
    let mut position = 0;
    while position < bytes.len() {
        if bytes[position] == b'"' {
            let start = position;
            position += 1;
            while position < bytes.len() {
                if bytes[position] == b'\\' && position + 1 < bytes.len() {
                    position += 2;
                    continue;
                }
                if bytes[position] == b'"' {
                    position += 1;
                    break;
                }
                position += 1;
            }
            out.push_str(&segment[start..position]);
        } else {
            let start = position;
            while position < bytes.len() && bytes[position] != b'"' {
                position += 1;
            }
            out.push_str(&tighten_non_literal(&segment[start..position]));
        }
    }
    out
}

fn tighten_non_literal(chunk: &str) -> String {
    collapse_space_before_parenthesis(
        chunk
            .replace(" ! (", "!(")
            .replace(" ! [", "![")
            .replace(" ! {", "!{")
            .replace(" )", ")")
            .replace(" ]", "]")
            .replace(" ,", ",")
            .replace(" ;", ";")
            .replace("* mut ", "*mut ")
            .replace("* const ", "*const "),
    )
}

fn collapse_space_before_parenthesis(value: String) -> String {
    let bytes = value.as_bytes();
    let mut out = String::with_capacity(value.len());
    let mut position = 0;
    while position < bytes.len() {
        if bytes[position] == b' '
            && position + 1 < bytes.len()
            && bytes[position + 1] == b'('
            && position > 0
        {
            let previous = bytes[position - 1];
            if previous.is_ascii_alphanumeric() || previous == b'_' {
                position += 1;
                continue;
            }
        }
        out.push(bytes[position] as char);
        position += 1;
    }
    out
}
