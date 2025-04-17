use std::fmt::Write;

#[macro_export]
macro_rules! code {
    ($printer:expr, $($args:tt)*) => {{
        let formatted = format!($($args)*);
        $printer.write_str(&formatted);
    }};
}

pub struct Printer {
    output: String,
    indent: usize,
    namespace: String,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            namespace: String::new(),
        }
    }

    pub fn write_namespace(&mut self, namespace: &str) {
        if namespace.is_empty() || self.namespace == namespace {
            return;
        }

        let mut current = self.namespace.split('.').peekable();
        let mut next = namespace.split('.').peekable();

        while current.peek() == next.peek() {
            if current.next().is_none() {
                break;
            }

            next.next();
        }

        for _ in 0..current.count() {
            if self.dedent() {
                self.write_str("}\n");
            }
        }

        for name in next {
            code!(self, "mod {name} {{");
            self.indent();
        }

        self.namespace = namespace.to_string();
    }

    pub fn write_str(&mut self, text: &str) {
        for line in text.lines() {
            if !line.is_empty() {
                write!(&mut self.output, "{}{}\n", "".repeat(self.indent), line).unwrap();
            } else {
                self.output.push('\n');
            }
        }
    }

    pub fn into_string(mut self) -> String {
        while self.dedent() {
            self.write_str("}\n");
        }

        self.output
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn dedent(&mut self) -> bool {
        if self.indent > 0 {
            self.indent = self.indent - 1;
            true
        } else {
            false
        }
    }
}

// Parenthesis ()
// Brace {}
// Bracket []

// fn namespace_starts_with(namespace: &str, starts_with: &str) -> bool {
//     !starts_with.is_empty()
//         && namespace.starts_with(starts_with)
//         && (namespace.len() == starts_with.len()
//             || namespace.as_bytes().get(starts_with.len()) == Some(&b'.'))
// }
