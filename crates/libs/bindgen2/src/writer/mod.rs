mod r#struct;

use super::*;

pub struct Writer {
    pub reader: &'static Reader,
    pub output: String,
    pub flatten: bool,
    pub package: bool,
}

impl Writer {
    pub fn write(&self, tree: &ItemTree) {
        dbg!(tree);

        // TODO: before we write the output, we need to build the complete tree of what's needed.
        // The tree can then be flattened if self.flatten before the tree is passed to either
        // write_package or write_file. The tree needs to be populated either with a minimal or complete
        // set of dependencies.

        if self.package {
            self.write_package(tree);
        } else {
            self.write_file(tree);
        }
    }

    fn write_file(&self, tree: &ItemTree) {
        let tokens = if self.flatten {
            self.write_flat(tree)
        } else {
            self.write_modules(tree)
        };

        write_to_file(&self.output, tokens);
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

    fn write_modules(&self, _tree: &ItemTree) -> TokenStream {
        todo!()
    }

    // TODO: This should call write_file for each file in the package
    fn write_package(&self, _tree: &ItemTree) {}

    fn write_item(&self, item: &'static Item) -> TokenStream {
        match item {
            Item::Struct(def) => self.write_struct(def),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }
}
