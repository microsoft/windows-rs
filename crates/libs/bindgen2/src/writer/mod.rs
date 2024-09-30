mod r#struct;
mod format;

use super::*;

pub struct Writer {
    pub reader: &'static Reader,
    pub output: String,
    pub flat: bool,
    pub package: bool,
    pub no_comment: bool,
    pub no_allow: bool,
    pub rustfmt: String,
}

impl Writer {
    pub fn write(&self, tree: &ItemTree) {
        dbg!(tree);

        if self.package {
            self.write_package(tree);
        } else {
            self.write_file(tree);
        }
    }

    fn write_file(&self, tree: &ItemTree) {
        let tokens = if self.flat {
            self.write_flat(tree)
        } else {
            self.write_modules(tree)
        };

        write_to_file(&self.output, self.format(&tokens.into_string()));
    }

    fn write_flat(&self, tree: &ItemTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for item in &tree.items {
            tokens.combine(self.write_item(item));
        }

        for tree in tree.nested.values() {
            tokens.combine(self.write_flat(tree));
        }

        tokens
    }

    fn write_modules(&self, tree: &ItemTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for item in &tree.items {
            tokens.combine(self.write_item(item));
        }

        for (name, tree) in &tree.nested {
            let name = to_ident(name);
            let nested = self.write_modules(tree);
            tokens.combine(quote!{ pub mod #name { #nested } });
        }

        tokens
    }

    fn write_package(&self, _tree: &ItemTree) {}

    fn write_item(&self, item: &'static Item) -> TokenStream {
        match item {
            Item::Struct(def) => self.write_struct(def),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }
}
