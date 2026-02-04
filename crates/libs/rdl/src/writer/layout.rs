use super::*;

#[derive(Default)]
pub struct Layout {
    modules: BTreeMap<String, Layout>,
    winrt: BTreeMap<String, String>,        // key is type name
    win32: BTreeMap<(String, i32), String>, // key is type name + arches
}

impl Layout {
    pub fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
            winrt: BTreeMap::new(),
            win32: BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        namespace: &str,
        name: &str,
        arches: i32,
        winrt: bool,
        tokens: String,
    ) {
        if let Some((first, rest)) = namespace.split_once('.') {
            self.modules
                .entry(first.to_string())
                .or_default()
                .insert(rest, name, arches, winrt, tokens)
        } else if winrt {
            self.modules
                .entry(namespace.to_string())
                .or_default()
                .winrt
                .insert(name.to_string(), tokens);
        } else {
            self.modules
                .entry(namespace.to_string())
                .or_default()
                .win32
                .insert((name.to_string(), arches), tokens);
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        for (name, module) in &self.modules {
            output.push_str(&module.to_module(name));
        }

        output
    }

    fn to_module(&self, name: &str) -> String {
        let mut output = String::new();

        if !self.modules.is_empty() {
            output.push_str("mod ");
            output.push_str(name);
            output.push('{');

            for (name, module) in &self.modules {
                output.push_str(&module.to_module(name));
            }

            output.push('}')
        }

        if !self.winrt.is_empty() {
            output.push_str("#[winrt] mod ");
            output.push_str(name);
            output.push('{');

            for tokens in self.winrt.values() {
                output.push_str(tokens);
            }

            output.push('}')
        }

        if !self.win32.is_empty() {
            output.push_str("#[win32] mod ");
            output.push_str(name);
            output.push('{');

            for tokens in self.win32.values() {
                output.push_str(tokens);
            }

            output.push('}')
        }

        output
    }
}
