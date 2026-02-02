mod r#enum;
mod interface;
mod r#struct;

use super::*;
use interface::*;
use metadata::HasAttributes;
use proc_macro2::*;
use quote::*;
use r#enum::*;
use r#struct::*;
use windows_metadata as metadata;

// TODO: the writer is primarily an internal tool as most developers will write their own
// definitions or just accept whatever a component author provides. This is thus mostly for
// generating rdl for backfilling definitions and for testing.

#[derive(Default)]
pub struct Writer {
    input: Vec<String>,
    output: String,
    namespace: String,
    recursive: bool,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    pub fn namespace(&mut self, namespace: &str) -> &mut Self {
        self.namespace = namespace.to_string();
        self
    }

    pub fn recursive(&mut self) -> &mut Self {
        self.recursive = true;
        self
    }

    pub fn write(&self) -> Result<(), Error> {
        let mut input = vec![];

        for file_name in &self.input {
            input.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid input", file_name, 0, 0))?,
            );
        }

        let index = metadata::reader::TypeIndex::new(input);
        let index = metadata::reader::ItemIndex::new(&index);

        // TODO: key sorts the output by type name, value sorts multi-definitions by arch?
        let mut items = BTreeMap::<(&str, &str), BTreeMap<i32, String>>::new();

        for namespace in index.keys() {
            if self.recursive {
                if !namespace_starts_with(namespace, &self.namespace) {
                    continue;
                }
            } else {
                if *namespace != self.namespace {
                    continue;
                }
            }

            for (name, item) in index.namespace_items(namespace) {
                items
                    .entry((namespace, name))
                    .or_default()
                    .insert(item_arches(item), write(item).to_string());
            }
        }

        let mut output = String::new();
        let mut current_namespace = "";

        for ((namespace, _name), items) in items {
            if current_namespace != namespace {
                let mut relative = current_namespace.split('.').peekable();
                current_namespace = namespace;
                let mut namespace = namespace.split('.').peekable();
                let shares_root = relative.peek() == namespace.peek();

                while relative.peek() == namespace.peek() {
                    if relative.next().is_none() {
                        break;
                    }

                    namespace.next();
                }

                if shares_root {
                    for _ in 0..relative.count() {
                        output.push('}')
                    }
                }

                for namespace in namespace {
                    output.push_str("mod ");
                    output.push_str(namespace);
                    output.push('{')
                }

                
            }

            for (_arch, tokens) in items {
                output.push_str(&tokens);
            }
        }

        for _ in current_namespace.split('.') {
            output.push('}')
        }

        // let modules: Vec<&str> = self.namespace.split('.').collect();

        // 

        // for module in &modules {
        //     output.push_str("mod ");
        //     output.push_str(module);
        //     output.push('{')
        // }

        // for (_, item) in items.values().flatten() {
        //     output.push_str(item);
        // }

        // for _ in 0..modules.len() {
        //     output.push('}')
        // }

        write_to_file(&self.output, format(&output));

        Ok(())
    }
}

#[track_caller]
fn write_to_file<C: AsRef<[u8]>>(path: &str, contents: C) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        if std::fs::create_dir_all(parent).is_err() {
            panic!("failed to create directory `{path}`");
        }
    }

    if std::fs::write(path, contents).is_err() {
        panic!("failed to write file `{path}`");
    }
}

fn namespace_starts_with(namespace: &str, starts_with: &str) -> bool {
    namespace.starts_with(starts_with)
        && (namespace.len() == starts_with.len()
            || namespace.as_bytes().get(starts_with.len()) == Some(&b'.'))
}

fn item_arches(item: &metadata::reader::Item) -> i32 {
    match item {
        metadata::reader::Item::Type(ty) => ty.arches(),
        metadata::reader::Item::Fn(ty) => ty.arches(),
        metadata::reader::Item::Const(ty) => ty.arches(),
    }
}

fn format(tokens: &str) -> String {
    if let Some(result) = rustfmt(tokens) {
        result.replace("trait ", "interface ")
    } else {
        tokens.to_string()
    }
}

fn rustfmt(tokens: &str) -> Option<String> {
    use std::io::Write;

    let mut cmd = std::process::Command::new("rustfmt");
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::null());

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

fn write(item: &metadata::reader::Item) -> TokenStream {
    match item {
        metadata::reader::Item::Type(ty) => write_type_def(ty),
        rest => todo!("{rest:?}"),
    }
}

fn write_type_def(item: &metadata::reader::TypeDef) -> TokenStream {
    match item.category() {
        metadata::reader::TypeCategory::Struct => write_struct(item),
        metadata::reader::TypeCategory::Enum => write_enum(item),
        metadata::reader::TypeCategory::Interface => write_interface(item),
        rest => todo!("{rest:?}"),
    }
}

fn write_value(value: &metadata::Value) -> TokenStream {
    match value {
        metadata::Value::Bool(value) => quote! { #value },
        metadata::Value::U8(value) => {
            let literal = Literal::u8_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I8(value) => {
            let literal = Literal::i8_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::U16(value) => {
            let literal = Literal::u16_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I16(value) => {
            let literal = Literal::i16_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::U32(value) => {
            let literal = Literal::u32_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I32(value) => {
            let literal = Literal::i32_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::U64(value) => {
            let literal = Literal::u64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I64(value) => {
            let literal = Literal::i64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::F32(value) => quote! { #value },
        metadata::Value::F64(value) => quote! { #value },
        metadata::Value::Utf8(value) => quote! { #value },
        metadata::Value::Utf16(value) => quote! { #value },
        metadata::Value::AttributeEnum(..) => todo!(),
    }
}

fn write_type(namespace: &str, item: &metadata::Type) -> TokenStream {
    use metadata::Type::*;
    match item {
        Void => quote! { core::ffi::c_void },
        Bool => quote! { bool },
        Char => quote! { u16 },
        I8 => quote! { i8 },
        U8 => quote! { u8 },
        I16 => quote! { i16 },
        U16 => quote! { u16 },
        I32 => quote! { i32 },
        U32 => quote! { u32 },
        I64 => quote! { i64 },
        U64 => quote! { u64 },
        F32 => quote! { f32 },
        F64 => quote! { f64 },
        ISize => quote! { isize },
        USize => quote! { usize },
        String => quote! { String },
        RefMut(ty) => {
            let ty = write_type(namespace, ty);
            quote! { &mut #ty }
        }
        RefConst(ty) => {
            let ty = write_type(namespace, ty);
            quote! { & #ty }
        }
        PtrMut(ty, pointers) => {
            let mut ty = write_type(namespace, ty);

            for _ in 0..*pointers {
                ty = quote! { *mut #ty };
            }

            ty
        }
        PtrConst(ty, pointers) => {
            let mut ty = write_type(namespace, ty);

            for _ in 0..*pointers {
                ty = quote! { *const #ty };
            }

            ty
        }
        Name(type_name) => {
            let name = format_ident!("{}", &type_name.name);

            // The empty namespace test is for nested types.
            if namespace == type_name.namespace || type_name.namespace.is_empty() {
                quote! { #name }
            } else {
                let mut relative = namespace.split('.').peekable();
                let mut namespace = type_name.namespace.split('.').peekable();
                let shares_root = relative.peek() == namespace.peek();

                while relative.peek() == namespace.peek() {
                    if relative.next().is_none() {
                        break;
                    }

                    namespace.next();
                }

                let mut tokens = TokenStream::new();

                if shares_root {
                    for _ in 0..relative.count() {
                        tokens = quote! { #tokens super:: };
                    }
                }

                for namespace in namespace {
                    let namespace = format_ident!("{}", namespace);
                    tokens = quote! { #tokens #namespace ::};
                }

                quote! { #tokens #name }
            }
        }
        rest => todo!("{rest:?}"),
    }
}
