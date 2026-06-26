use super::*;
use std::io::Write;

impl Config<'_> {
    pub fn format(&self, tokens: &str) -> String {
        let formatted = self
            .rustfmt(tokens)
            .unwrap_or_else(|| panic!("failed to format output with `rustfmt`"));

        // `proc_macro2::TokenStream::to_string()` inserts a space between
        // most adjacent tokens (e.g. `link ! (`, `windows_core :: BOOL`).
        // `rustfmt` normalises that for ordinary Rust syntax but does not
        // touch the contents of macro invocations, so we tighten up the
        // whitespace ourselves in patterns that only appear inside macro
        // bodies after `rustfmt` has run.
        tighten_macro_whitespace(&formatted)
    }

    fn rustfmt(&self, tokens: &str) -> Option<String> {
        let mut cmd = std::process::Command::new("rustfmt");
        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::null());
        cmd.arg("--edition");
        cmd.arg("2024");

        if let Some(config) = self.bindgen.rustfmt.as_deref() {
            cmd.arg("--config");
            cmd.arg(config);
        }

        let mut child = cmd.spawn().ok()?;
        let mut stdin = child.stdin.take()?;
        stdin.write_all(tokens.as_bytes()).ok()?;
        drop(stdin);
        let output = child.wait_with_output().ok()?;

        if !output.status.success() {
            return None;
        }

        String::from_utf8(output.stdout).ok()
    }
}

/// Tighten whitespace inside macro invocations that `rustfmt` leaves untouched.
fn tighten_macro_whitespace(src: &str) -> String {
    // Pass 1: globally tighten `::` joiners outside strings. These never
    // appear in `rustfmt`-formatted Rust code.
    let pass1 = replace_outside_strings(
        src,
        &[(" :: ", "::"), (":: ", "::"), (" ::", "::"), (") (", ")(")],
    );

    // Pass 2: inside macro invocation regions, tighten the remaining patterns.
    apply_inside_macros(&pass1)
}

fn replace_outside_strings(src: &str, patterns: &[(&str, &str)]) -> String {
    let bytes = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let start = i;
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push_str(&src[start..i]);
        } else {
            let start = i;
            while i < bytes.len() && bytes[i] != b'"' {
                i += 1;
            }
            let mut chunk = src[start..i].to_string();
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
    let mut i = 0;
    while i < bytes.len() {
        if let Some(open) = find_macro_invocation(bytes, i) {
            let header_start = macro_path_start(bytes, open.bang);
            out.push_str(&src[i..header_start]);
            let close = find_matching_delim(bytes, open.open_idx, open.open, open.close);
            out.push_str(&tighten_macro_segment(&src[header_start..=close]));
            i = close + 1;
        } else {
            out.push_str(&src[i..]);
            break;
        }
    }
    out
}

struct MacroOpen {
    bang: usize,
    open_idx: usize,
    open: u8,
    close: u8,
}

fn find_macro_invocation(bytes: &[u8], start: usize) -> Option<MacroOpen> {
    let mut i = start;
    while i + 4 < bytes.len() {
        if bytes[i] == b' ' && bytes[i + 1] == b'!' && bytes[i + 2] == b' ' && i > 0 {
            let (open, close) = match bytes[i + 3] {
                b'(' => (b'(', b')'),
                b'[' => (b'[', b']'),
                _ => {
                    i += 1;
                    continue;
                }
            };
            // Require the character before the space to be an identifier
            // character; otherwise `-> !` followed by something would be
            // matched.
            let prev = bytes[i - 1];
            if !(prev.is_ascii_alphanumeric() || prev == b'_') {
                i += 1;
                continue;
            }
            return Some(MacroOpen {
                bang: i + 1,
                open_idx: i + 3,
                open,
                close,
            });
        }
        i += 1;
    }
    None
}

fn macro_path_start(bytes: &[u8], bang: usize) -> usize {
    let mut i = bang;
    if i > 0 && bytes[i - 1] == b' ' {
        i -= 1;
    }
    while i > 0 {
        let c = bytes[i - 1];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' {
            i -= 1;
        } else {
            break;
        }
    }
    i
}

fn find_matching_delim(bytes: &[u8], open_idx: usize, open: u8, close: u8) -> usize {
    let mut depth: usize = 0;
    let mut i = open_idx;
    let mut in_str = false;
    while i < bytes.len() {
        let c = bytes[i];
        if in_str {
            if c == b'\\' && i + 1 < bytes.len() {
                i += 2;
                continue;
            }
            if c == b'"' {
                in_str = false;
            }
        } else if c == b'"' {
            in_str = true;
        } else if c == open {
            depth += 1;
        } else if c == close && depth > 0 {
            depth -= 1;
            if depth == 0 {
                return i;
            }
        }
        i += 1;
    }
    bytes.len().saturating_sub(1)
}

fn tighten_macro_segment(segment: &str) -> String {
    let bytes = segment.as_bytes();
    let mut out = String::with_capacity(segment.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let start = i;
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push_str(&segment[start..i]);
        } else {
            let start = i;
            while i < bytes.len() && bytes[i] != b'"' {
                i += 1;
            }
            let chunk = &segment[start..i];
            out.push_str(&tighten_non_literal(chunk));
        }
    }
    out
}

fn tighten_non_literal(chunk: &str) -> String {
    chunk
        .replace(" ! (", "!(")
        .replace(" ! [", "![")
        .replace(" ! {", "!{")
        .replace(" )", ")")
        .replace(" ]", "]")
        .replace(" ,", ",")
        .replace(" ;", ";")
        .replace("* mut ", "*mut ")
        .replace("* const ", "*const ")
        .pipe(collapse_space_before_paren)
}

trait StrPipe {
    fn pipe<F: Fn(String) -> String>(self, f: F) -> String;
}

impl StrPipe for String {
    fn pipe<F: Fn(Self) -> Self>(self, f: F) -> String {
        f(self)
    }
}

/// Collapse `name (` → `name(` after an identifier character.
fn collapse_space_before_paren(s: String) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b' ' && i + 1 < bytes.len() && bytes[i + 1] == b'(' && i > 0 {
            let prev = bytes[i - 1];
            if prev.is_ascii_alphanumeric() || prev == b'_' {
                // Drop the space: `name (` → `name(`.
                i += 1;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_header_collapsed() {
        let input = "    windows_core :: link ! (\"user32.dll\" \"system\" fn EnableMouseInPointer (fenable : windows_core :: BOOL) -> windows_core :: BOOL);";
        let expected = "    windows_core::link!(\"user32.dll\" \"system\" fn EnableMouseInPointer(fenable : windows_core::BOOL) -> windows_core::BOOL);";
        assert_eq!(tighten_macro_whitespace(input), expected);
    }

    #[test]
    fn outside_macro_unchanged() {
        let input = "let y: *const () = std::ptr::null();";
        assert_eq!(tighten_macro_whitespace(input), input);
    }

    #[test]
    fn star_mut_inside_macro() {
        let input = "link ! (fn foo(x : * mut u8) -> ());";
        let expected = "link!(fn foo(x : *mut u8) -> ());";
        assert_eq!(tighten_macro_whitespace(input), expected);
    }
}
