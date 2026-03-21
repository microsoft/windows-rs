use super::*;

#[derive(Default)]
pub struct Layout {
    modules: BTreeMap<String, Layout>,
    winrt: BTreeMap<String, Vec<String>>,
    win32: BTreeMap<String, Vec<String>>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
            winrt: BTreeMap::new(),
            win32: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, namespace: &str, name: &str, winrt: bool, tokens: String) {
        if let Some((first, rest)) = namespace.split_once('.') {
            self.modules
                .entry(first.to_string())
                .or_default()
                .insert(rest, name, winrt, tokens)
        } else if winrt {
            self.modules
                .entry(namespace.to_string())
                .or_default()
                .winrt
                .entry(name.to_string())
                .or_default()
                .push(tokens);
        } else {
            self.modules
                .entry(namespace.to_string())
                .or_default()
                .win32
                .entry(name.to_string())
                .or_default()
                .push(tokens);
        }
    }

    fn to_module(&self, name: &str) -> String {
        let mut output = String::new();

        if !self.modules.is_empty() {
            output.push_str("mod ");
            output.push_str(name);
            output.push_str(" {\n");

            for (name, module) in &self.modules {
                output.push_str(&module.to_module(name));
            }

            output.push_str("}\n");
        }

        if !self.winrt.is_empty() {
            output.push_str("#[winrt]\nmod ");
            output.push_str(name);
            output.push_str(" {\n");

            for items in self.winrt.values() {
                let mut items = items.clone();
                items.sort();
                for tokens in &items {
                    output.push_str(tokens);
                }
            }

            output.push_str("}\n");
        }

        if !self.win32.is_empty() {
            output.push_str("#[win32]\nmod ");
            output.push_str(name);
            output.push_str(" {\n");

            for items in self.win32.values() {
                let mut items = items.clone();
                items.sort();
                for tokens in &items {
                    output.push_str(tokens);
                }
            }

            output.push_str("}\n");
        }

        output
    }
}

/// Re-indent a flat RDL string.
///
/// The writer functions emit items without any leading whitespace.  This
/// function scans the assembled output line-by-line and adds the correct
/// amount of indentation based on `{` / `}` nesting depth.
pub fn reindent(s: &str) -> String {
    let mut output = String::new();
    let mut indent: usize = 0;

    for line in s.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // A line whose first non-whitespace character is `}` closes a block —
        // decrease indent before emitting it.
        if trimmed.starts_with('}') {
            indent = indent.saturating_sub(1);
        }

        for _ in 0..indent {
            output.push_str("    ");
        }
        output.push_str(trimmed);
        output.push('\n');

        // A line ending with `{` (but not `{}`) opens a new block.
        if trimmed.ends_with('{') && !trimmed.ends_with("{}") {
            indent += 1;
        }
    }

    output
}

impl std::fmt::Display for Layout {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut raw = String::new();
        for (name, module) in &self.modules {
            raw.push_str(&module.to_module(name));
        }
        write!(fmt, "{}", reindent(&raw))
    }
}
