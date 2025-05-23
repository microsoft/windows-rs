use std::fmt::Write;

#[derive(Default)]
pub struct Formatter {
    output: String,
    indent: usize,
    namespace: String,
}

impl Formatter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            namespace: String::new(),
        }
    }

    pub fn write_fmt(&mut self, f: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.write_indent();
        self.output.write_fmt(f)
    }

    pub fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.output.push_str("    ");
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
                self.write_indent();
                self.output.push_str("}\n");
            }
        }

        for name in next {
            writeln!(self, "mod {name} {{").unwrap();
            self.indent();
        }

        self.namespace = namespace.to_string();
    }

    pub fn into_string(mut self) -> String {
        while self.dedent() {
            self.write_indent();
            self.output.push_str("}\n");
        }

        self.output
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn dedent(&mut self) -> bool {
        if self.indent > 0 {
            self.indent -= 1;
            true
        } else {
            false
        }
    }
}
