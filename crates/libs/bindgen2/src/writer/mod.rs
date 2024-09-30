mod r#struct; 

use super::*;

pub struct Writer {
    pub reader: &'static Reader,
    pub output: String,
    pub flatten: bool,
    pub package: bool,
}

impl Writer {
    pub fn write(&self, tree: &Tree) {
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

    fn write_file(&self, tree: &Tree) {
        if self.flatten {
            self.write_flat(tree);
        } else {
            self.write_modules(tree);
        }
    }

    fn write_flat(&self, _tree: &Tree) {
    }

    fn write_modules(&self, _tree: &Tree) {

    }

    // TODO: This should call write_file for each file in the package
    fn write_package(&self, _tree: &Tree) {}

    fn write_type_name(&self, namespace: &str, name: &str) {
        for item in self.reader.with_full_name(namespace, name) {
            self.write_item(item);
        }
    }

    fn write_item(&self, item: &Item) {
        match item {
            Item::Struct(def) => self.write_struct(def),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }
}
