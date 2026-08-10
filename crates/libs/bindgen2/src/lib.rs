#![doc = include_str!("../readme.md")]

use std::sync::Arc;
use windows_metadata2::{
    AnyRowId, AttributeArgument, AttributeValue, ConstantValue, Database, Entity, FileId, Image,
    MethodSignature, TypeAttributes, TypeCategory, TypeDefinition, TypeKind,
    tables::{Field, MethodDef, TypeDef},
};

mod enum_model;
mod error;
mod filter;
mod guid;
mod model;
mod native;
mod native_constant;
mod native_default;
mod native_delegate;
mod native_function;
mod native_signature;
mod native_type;
mod output;
mod struct_model;
mod tokens;
mod ty;
mod win32;

pub use enum_model::Enum;
pub use error::Error;
pub use filter::Filter;
pub use model::{Value, Values};
pub use native_constant::Constant;
pub use native_delegate::Delegate;
pub use native_function::Function;
pub use native_type::{NativeType, NativeTypeKind};
pub use struct_model::Struct;
pub use win32::Win32Items;

/// A projected WinRT value category.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueKind {
    Enum,
    Struct,
}

/// Generated Rust output layout.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Layout {
    /// Emit nested Rust modules for metadata namespaces.
    #[default]
    Modules,
    /// Emit one flat list of items.
    Flat,
}

/// Options for one generation request.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Options {
    /// Generated Rust output layout.
    pub layout: Layout,
}

#[derive(Clone, Copy)]
struct ValueEntry {
    entity: Entity<TypeDef>,
    kind: ValueKind,
}

/// Owns a reusable validated metadata database.
pub struct Metadata {
    database: Arc<Database>,
}

/// Owns one deterministic generation request over shared metadata.
pub struct Generator {
    database: Arc<Database>,
    values: Vec<ValueEntry>,
    filter: Option<Filter>,
    options: Options,
}

/// A borrowed projected WinRT value item.
#[derive(Clone, Copy)]
pub struct ValueItem<'a> {
    definition: TypeDefinition<'a>,
    kind: ValueKind,
}

impl Metadata {
    /// Wraps an owned validated metadata database for reuse.
    pub fn new(database: Database) -> Self {
        Self {
            database: Arc::new(database),
        }
    }

    /// Builds a reusable database from owned metadata images.
    pub fn from_images(images: impl IntoIterator<Item = Image>) -> Result<Self, Error> {
        Ok(Self::new(Database::new(images)?))
    }

    /// Returns the validated metadata database.
    pub fn database(&self) -> &Database {
        &self.database
    }

    /// Creates an independent generation request sharing this database.
    pub fn generator(&self) -> Result<Generator, Error> {
        self.generator_with(Options::default())
    }

    /// Creates a generation request with explicit options.
    pub fn generator_with(&self, options: Options) -> Result<Generator, Error> {
        Generator::from_shared(self.database.clone(), options, None)
    }

    /// Creates a filtered generation request with default options.
    pub fn generator_filtered(&self, filter: Filter) -> Result<Generator, Error> {
        self.generator_with_filter(Options::default(), filter)
    }

    /// Creates a filtered generation request with explicit options.
    pub fn generator_with_filter(
        &self,
        options: Options,
        filter: Filter,
    ) -> Result<Generator, Error> {
        Generator::from_shared(self.database.clone(), options, Some(filter))
    }
}

impl Generator {
    /// Selects projected items from an owned metadata database.
    pub fn new(database: Database) -> Result<Self, Error> {
        Metadata::new(database).generator()
    }

    fn from_shared(
        database: Arc<Database>,
        options: Options,
        filter: Option<Filter>,
    ) -> Result<Self, Error> {
        let mut values = Vec::new();

        for definition in database.definitions() {
            if !definition.is_windows_runtime()? {
                continue;
            }

            let kind = match definition.category()? {
                TypeCategory::Enum => ValueKind::Enum,
                TypeCategory::Struct => {
                    if definition.has_attribute("ApiContractAttribute")? {
                        continue;
                    }
                    ValueKind::Struct
                }
                _ => continue,
            };

            let namespace = definition.namespace()?;
            let name = definition.name()?;
            if filter
                .as_ref()
                .is_some_and(|filter| !filter.includes(namespace, name))
            {
                continue;
            }

            values.push((
                namespace.to_string(),
                name.to_string(),
                ValueEntry {
                    entity: definition.entity(),
                    kind,
                },
            ));
        }

        values.sort_by(|left, right| {
            (&left.0, &left.1, left.2.entity).cmp(&(&right.0, &right.1, right.2.entity))
        });

        Ok(Self {
            database,
            values: values.into_iter().map(|(_, _, entry)| entry).collect(),
            filter,
            options,
        })
    }

    /// Returns the shared metadata database.
    pub fn database(&self) -> &Database {
        &self.database
    }

    /// Returns this request's generation options.
    pub const fn options(&self) -> Options {
        self.options
    }

    /// Iterates projected values in deterministic namespace/name/entity order.
    pub fn values(&self) -> impl ExactSizeIterator<Item = ValueItem<'_>> {
        self.values.iter().map(|entry| ValueItem {
            definition: self.database.definition(entry.entity).unwrap(),
            kind: entry.kind,
        })
    }
}

impl<'a> ValueItem<'a> {
    /// Returns the metadata definition.
    pub const fn definition(self) -> TypeDefinition<'a> {
        self.definition
    }

    /// Returns the projected value category.
    pub const fn kind(self) -> ValueKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use windows_metadata2::Image;

    static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

    fn generator() -> Generator {
        Generator::new(
            Database::new([
                Image::new(windows_default::WINRT).unwrap(),
                Image::new(windows_default::WIN32).unwrap(),
            ])
            .unwrap(),
        )
        .unwrap()
    }

    fn fixture_metadata(source: &str) -> Metadata {
        let path = std::env::temp_dir().join(format!(
            "windows_bindgen2_{}_{}.winmd",
            std::process::id(),
            NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
        ));
        windows_rdl::reader()
            .input_text(source)
            .output(&path)
            .write()
            .unwrap();
        let image = Image::read(&path).unwrap();
        std::fs::remove_file(path).unwrap();
        Metadata::from_images([image]).unwrap()
    }

    fn fixture(source: &str) -> Generator {
        fixture_metadata(source).generator().unwrap()
    }

    #[test]
    fn values_are_deterministic_and_borrow_database_names() {
        let generator = generator();
        let actual: Vec<_> = generator
            .values()
            .map(|item| {
                let definition = item.definition();
                (
                    definition.namespace().unwrap(),
                    definition.name().unwrap(),
                    item.kind(),
                    definition.entity(),
                )
            })
            .collect();

        let counts = actual.iter().fold([0; 2], |mut counts, item| {
            counts[match item.2 {
                ValueKind::Enum => 0,
                ValueKind::Struct => 1,
            }] += 1;
            counts
        });
        assert_eq!(counts, [1_731, 125]);
        assert!(actual.windows(2).all(|pair| {
            (&pair[0].0, &pair[0].1, pair[0].3) < (&pair[1].0, &pair[1].1, pair[1].3)
        }));
        assert_eq!(
            actual,
            generator
                .values()
                .map(|item| {
                    let definition = item.definition();
                    (
                        definition.namespace().unwrap(),
                        definition.name().unwrap(),
                        item.kind(),
                        definition.entity(),
                    )
                })
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn metadata_database_is_reused_across_requests() {
        let metadata = Metadata::from_images([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap();
        let first = metadata.generator().unwrap();
        let second = metadata.generator().unwrap();

        assert!(std::ptr::eq(first.database(), second.database()));
        assert_eq!(first.values().len(), second.values().len());
        let first = first.win32_items().unwrap();
        let second = second.win32_items().unwrap();
        assert_eq!(first.type_count(), second.type_count());
        assert_eq!(first.delegate_count(), second.delegate_count());
        assert_eq!(first.constant_count(), second.constant_count());
        assert_eq!(first.function_count(), second.function_count());
    }

    #[test]
    fn values_apply_only_current_projection_policy() {
        let generator = generator();
        for item in generator.values() {
            let definition = item.definition();
            assert!(definition.is_windows_runtime().unwrap());
            assert!(!definition.has_attribute("ApiContractAttribute").unwrap());
            assert_eq!(
                item.kind(),
                match definition.category().unwrap() {
                    TypeCategory::Enum => ValueKind::Enum,
                    TypeCategory::Struct => ValueKind::Struct,
                    rest => panic!("unexpected value category {rest:?}"),
                }
            );
        }
    }

    #[test]
    fn lowers_and_renders_the_value_corpus_with_exact_accounting() {
        let values = generator().lower_values().unwrap();
        assert_eq!(values.len(), 1_856);

        let mut counts = [0; 2];
        let mut unsupported = Vec::new();
        for (namespace, name, value) in values.iter() {
            counts[match value {
                Value::Enum(_) => 0,
                Value::Struct(_) => 1,
            }] += 1;
            if let Err(error) = values.write(namespace, name) {
                unsupported.push((format!("{namespace}.{name}"), error.to_string()));
            }
        }

        assert_eq!(counts, [1_731, 125]);
        assert!(unsupported.is_empty(), "{unsupported:#?}");
        assert!(
            values
                .write("Windows.Web.Http", "HttpProgress")
                .unwrap()
                .to_string()
                .contains("61c17706-2d65-11e0-9ae8-d48564015472")
        );
    }

    #[test]
    fn focused_value_output_matches_existing_golden_tokens() {
        let enum_values = fixture(
            r#"
                #[winrt]
                mod Test {
                    #[repr(i32)]
                    enum Enum {
                        First = 0,
                        Second = 1,
                        Third = 2,
                    }
                }
            "#,
        )
        .lower_values()
        .unwrap();
        let enum_expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/winrt_enum.rs")
                .parse()
                .unwrap();
        assert_eq!(
            enum_values.write("Test", "Enum").unwrap().to_string(),
            enum_expected.to_string()
        );

        let struct_values = fixture(
            r#"
                #[winrt]
                mod Test {
                    struct Struct {
                        x: i32,
                        y: i32,
                    }
                }
            "#,
        )
        .lower_values()
        .unwrap();
        let struct_expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/winrt_struct.rs")
                .parse()
                .unwrap();
        assert_eq!(
            struct_values.write("Test", "Struct").unwrap().to_string(),
            struct_expected.to_string()
        );
    }

    #[test]
    fn module_output_matches_existing_nested_golden_tokens() {
        let generator = fixture(
            r#"
                #[win32]
                mod Test {
                    #[repr(i32)]
                    enum Enum {
                        First = 0,
                        Second = 1,
                        Third = 2,
                    }

                    mod Inner {
                        #[repr(i32)]
                        enum Enum {
                            First = 0,
                            Second = 1,
                        }
                    }
                }
            "#,
        );
        let expected: TokenStream = include_str!("../../../tests/libs/bindgen/expected/modules.rs")
            .parse()
            .unwrap();
        assert_eq!(
            generator.write_modules().unwrap().to_string(),
            expected.to_string()
        );
    }

    #[test]
    fn request_options_select_flat_output() {
        let metadata = fixture_metadata(include_str!(
            "../../../tests/libs/bindgen/input/struct_default_sys.rdl"
        ));
        let generator = metadata
            .generator_with(Options {
                layout: Layout::Flat,
            })
            .unwrap();
        let expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/struct_default_sys.rs")
                .parse()
                .unwrap();

        assert_eq!(generator.options().layout, Layout::Flat);
        assert_eq!(generator.write().unwrap().to_string(), expected.to_string());
        assert_eq!(
            generator.write().unwrap().to_string(),
            generator.write_flat().unwrap().to_string()
        );
    }

    #[test]
    fn flat_output_rejects_cross_namespace_name_collisions() {
        let metadata = fixture_metadata(
            r#"
                #[win32]
                mod First {
                    type Shared = u32;
                }
                #[win32]
                mod Second {
                    type Shared = u16;
                }
            "#,
        );
        let generator = metadata
            .generator_with(Options {
                layout: Layout::Flat,
            })
            .unwrap();

        assert!(matches!(
            generator.write(),
            Err(Error::FlatNameCollision {
                name,
                first_namespace,
                second_namespace,
            }) if name == "Shared"
                && first_namespace == "First"
                && second_namespace == "Second"
        ));
    }

    #[test]
    fn exact_filters_limit_winrt_and_win32_selection() {
        let metadata = fixture_metadata(
            r#"
                #[win32]
                mod First {
                    type Shared = u32;
                    const ONLY_FIRST: u32 = 1;
                }
                #[win32]
                mod Second {
                    type Shared = u16;
                    const ONLY_SECOND: u32 = 2;
                }
                #[winrt]
                mod Managed {
                    #[repr(i32)]
                    enum Kind {
                        First = 0,
                    }
                }
            "#,
        );
        let mut filter = Filter::new();
        filter
            .include_name("Shared")
            .include_item("First", "ONLY_FIRST")
            .include_namespace("Managed");
        let generator = metadata.generator_filtered(filter).unwrap();
        let items = generator.win32_items().unwrap();

        assert_eq!(generator.values().len(), 1);
        assert_eq!(items.type_count(), 2);
        assert_eq!(items.constant_count(), 1);
        assert_eq!(items.function_count(), 0);

        let output = generator.write_modules().unwrap().to_string();
        assert!(output.contains("pub mod First"));
        assert!(output.contains("pub const ONLY_FIRST"));
        assert!(output.contains("pub mod Second"));
        assert!(!output.contains("ONLY_SECOND"));
        assert!(output.contains("pub mod Managed"));
        assert!(output.contains("pub struct Kind"));
    }

    #[test]
    fn module_output_combines_supported_winrt_and_win32_items() {
        let generator = fixture(
            r#"
                #[winrt]
                mod Managed {
                    #[repr(i32)]
                    enum Kind {
                        First = 0,
                    }
                }
                #[win32]
                mod Native {
                    const VALUE: u32 = 42;
                }
            "#,
        );
        let output = generator.write_modules().unwrap().to_string();
        assert!(output.contains("pub mod Managed"));
        assert!(output.contains("pub struct Kind"));
        assert!(output.contains("pub mod Native"));
        assert!(output.contains("pub const VALUE : u32 = 42"));
    }

    #[test]
    fn focused_native_type_output_matches_existing_golden_tokens() {
        let generator = fixture(
            r#"
                #[win32]
                mod Test {
                    type NativePtr = *const u8;
                    type NativePtrAlias = NativePtr;
                    struct Struct {
                        field: NativePtrAlias,
                        other: i32,
                    }
                    #[repr(i32)]
                    enum Enum {
                        First = 1,
                        Second = 2,
                        Third = 3,
                    }
                    union Value {
                        i: i32,
                        f: f32,
                        p: *mut u8,
                    }
                }
            "#,
        );
        let items = generator.win32_items().unwrap();

        let native_ptr = items.native_type("Test", "NativePtr").unwrap().write_sys();
        let native_ptr_alias = items
            .native_type("Test", "NativePtrAlias")
            .unwrap()
            .write_sys();
        let structure = items.native_type("Test", "Struct").unwrap().write_sys();
        let actual = quote! { #native_ptr #native_ptr_alias #structure };
        let expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/struct_typedef_pointer_sys.rs")
                .parse()
                .unwrap();
        assert_eq!(actual.to_string(), expected.to_string());

        let actual = items.native_type("Test", "Enum").unwrap().write_sys();
        let expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/enum_sys.rs")
                .parse()
                .unwrap();
        assert_eq!(actual.to_string(), expected.to_string());

        let actual = items.native_type("Test", "Value").unwrap().write_sys();
        let expected: TokenStream = include_str!("../../../tests/libs/bindgen/expected/union.rs")
            .parse()
            .unwrap();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn true_nested_native_types_match_existing_golden_tokens() {
        let generator = fixture(include_str!(
            "../../../tests/libs/bindgen/input/struct_nested_anon_sys.rdl"
        ));
        let items = generator.win32_items().unwrap();
        assert_eq!(items.nested_type_count(), 10);
        let types = items.native_types().collect::<Result<Vec<_>, _>>().unwrap();
        let types = types.iter().map(NativeType::write_sys);
        let actual = quote! { #(#types)* };
        let expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/struct_nested_anon_sys.rs")
                .parse()
                .unwrap();
        assert_eq!(actual.to_string(), expected.to_string());
        let expected = quote! { pub mod Test { #expected } };
        assert_eq!(
            generator.write_modules().unwrap().to_string(),
            expected.to_string()
        );
    }

    #[test]
    fn native_default_policy_matches_existing_golden_tokens() {
        let generator = fixture(include_str!(
            "../../../tests/libs/bindgen/input/struct_default_sys.rdl"
        ));
        let items = generator.win32_items().unwrap();
        let types = items.native_types().collect::<Result<Vec<_>, _>>().unwrap();
        let expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/struct_default_sys.rs")
                .parse()
                .unwrap();
        let types = types.iter().map(NativeType::write_sys);
        let actual = quote! { #(#types)* };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn native_delegates_match_existing_golden_tokens() {
        let generator = fixture(include_str!(
            "../../../tests/libs/bindgen/input/callback.rdl"
        ));
        let items = generator.win32_items().unwrap();
        let delegates = items.delegates().collect::<Result<Vec<_>, _>>().unwrap();
        let delegates = delegates.iter().map(Delegate::write_sys);
        let actual = quote! { #(#delegates)* };
        let expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/callback.rs")
                .parse()
                .unwrap();
        assert_eq!(
            actual.to_string().replace("> ;", ">;"),
            expected.to_string()
        );

        let generator = fixture(include_str!(
            "../../../tests/libs/bindgen/input/arch_delegate_dependency_sys.rdl"
        ));
        let expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/arch_delegate_dependency_sys.rs")
                .parse()
                .unwrap();
        let expected = quote! { pub mod Test { #expected } };
        assert_eq!(
            generator
                .write_modules()
                .unwrap()
                .to_string()
                .replace("> ;", ">;"),
            expected.to_string()
        );
    }

    #[test]
    fn architecture_gates_match_existing_flat_sys_tokens() {
        let generator = fixture(
            r#"
                #[win32]
                mod Test {
                    #[arch(X64 | Arm64)]
                    type ArchScalar = i32;
                    #[arch(X86)]
                    type ArchScalar = i16;
                }
            "#,
        );
        let items = generator.win32_items().unwrap();
        let types = items.native_types().collect::<Result<Vec<_>, _>>().unwrap();
        let types = types.iter().map(NativeType::write_sys);
        let actual = quote! { #(#types)* };
        let expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/arch_typedef_sys.rs")
                .parse()
                .unwrap();
        assert_eq!(actual.to_string(), expected.to_string());

        let generator = fixture(
            r#"
                #[win32]
                mod Test {
                    #[repr(i32)]
                    #[arch(X64)]
                    enum ArchEnum {
                        First = 1,
                        X64Only = 2,
                    }
                    #[arch(Arm64)]
                    union ArchUnion {
                        value: i32,
                    }
                }
            "#,
        );
        let items = generator.win32_items().unwrap();
        let types = items.native_types().collect::<Result<Vec<_>, _>>().unwrap();
        let types = types.iter().map(NativeType::write_sys);
        let output = quote! { #(#types)* }.to_string();
        assert_eq!(output.matches("target_arch = \"x86_64\"").count(), 3);
        assert_eq!(output.matches("target_arch = \"arm64ec\"").count(), 3);
        assert_eq!(output.matches("target_arch = \"aarch64\"").count(), 2);

        let generator = fixture(
            r#"
                #[win32]
                mod Test {
                    #[repr(i32)]
                    #[arch(Arm64)]
                    enum ArchEnum {
                        First = 1,
                        Arm64Only = 3,
                    }
                    #[repr(i32)]
                    #[arch(X64)]
                    enum ArchEnum {
                        First = 1,
                        X64Only = 4,
                    }
                }
            "#,
        );
        let expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/arch_enum_sys.rs")
                .parse()
                .unwrap();
        let expected = quote! { pub mod Test { #expected } };
        assert_eq!(
            generator.write_modules().unwrap().to_string(),
            expected.to_string()
        );

        let generator = fixture(
            r#"
                #[win32]
                mod Test {
                    #[arch(X64 | Arm64)]
                    const VALUE: u32 = 64;
                    #[arch(X86)]
                    const VALUE: u32 = 32;
                    #[arch(X64 | Arm64)]
                    #[library("test.dll")]
                    extern fn ArchFunction(value: i64) -> i64;
                    #[arch(X86)]
                    #[library("test.dll")]
                    extern fn ArchFunction(value: i32) -> i32;
                }
            "#,
        );
        let items = generator.win32_items().unwrap();
        let constants = items.constants().collect::<Result<Vec<_>, _>>().unwrap();
        let functions = items.functions().collect::<Result<Vec<_>, _>>().unwrap();
        let constants = constants.iter().map(Constant::write_sys);
        let functions = functions.iter().map(Function::write_sys);
        let actual = quote! { #(#constants)* #(#functions)* };
        let expected: TokenStream = r#"
            #[cfg(target_arch = "x86")]
            pub const VALUE: u32 = 32;
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            pub const VALUE: u32 = 64;
            #[cfg(target_arch = "x86")]
            windows_link::link!("test.dll" "system" fn ArchFunction(value: i32) -> i32);
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            windows_link::link!("test.dll" "system" fn ArchFunction(value: i64) -> i64);
        "#
        .parse()
        .unwrap();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn win32_apis_selection_has_exact_corpus_counts() {
        let generator = generator();
        let items = generator.win32_items().unwrap();
        assert_eq!(
            [
                items.type_count(),
                items.constant_count(),
                items.function_count()
            ],
            [30_109, 83_641, 14_559]
        );
    }

    #[test]
    fn focused_win32_output_matches_existing_flat_sys_tokens() {
        let generator = fixture(
            r#"
                #[win32]
                mod Test {
                    const A_U8: u8 = 255;
                    #[library("test.dll")]
                    extern fn SysFunction() -> u32;
                }
            "#,
        );
        let items = generator.win32_items().unwrap();

        let constant_expected: TokenStream = "pub const A_U8: u8 = 255;".parse().unwrap();
        assert_eq!(
            items
                .constant("Test", "A_U8")
                .unwrap()
                .write_sys()
                .to_string(),
            constant_expected.to_string()
        );

        let function_expected: TokenStream =
            include_str!("../../../tests/libs/bindgen/expected/fn_sys.rs")
                .parse()
                .unwrap();
        assert_eq!(
            items
                .function("Test", "SysFunction")
                .unwrap()
                .write_sys()
                .to_string(),
            function_expected.to_string()
        );

        let pointer_generator = fixture(
            r#"
                #[win32]
                mod Test {
                    struct Struct {
                        x: i32,
                        y: i32,
                    }
                    #[library("test.dll")]
                    extern fn SysFlatFunction(s: *const Struct) -> i32;
                    const GREETING: String = "hello";
                }
            "#,
        );
        let pointer_items = pointer_generator.win32_items().unwrap();
        let pointer_expected: TokenStream =
            r#"windows_link::link!("test.dll" "system" fn SysFlatFunction(s: *const Struct) -> i32);"#
                .parse()
                .unwrap();
        assert_eq!(
            pointer_items
                .function("Test", "SysFlatFunction")
                .unwrap()
                .write_sys()
                .to_string(),
            pointer_expected.to_string()
        );
        let string_expected: TokenStream =
            "pub const GREETING: PCWSTR = [104, 101, 108, 108, 111, 0].as_ptr();"
                .parse()
                .unwrap();
        assert_eq!(
            pointer_items
                .constant("Test", "GREETING")
                .unwrap()
                .write_sys()
                .to_string(),
            string_expected.to_string()
        );

        let alias_generator = fixture(
            r#"
                #[win32]
                mod Test {
                    type MyI32 = i32;
                    type MyU64 = u64;
                    const I_TYPED: MyI32 = 42;
                    const J_TYPED: MyU64 = 999;
                }
            "#,
        );
        let alias_items = alias_generator.win32_items().unwrap();
        let signed_expected: TokenStream =
            "pub const I_TYPED: MyI32 = 0x2A_u32 as _;".parse().unwrap();
        assert_eq!(
            alias_items
                .constant("Test", "I_TYPED")
                .unwrap()
                .write_sys()
                .to_string(),
            signed_expected.to_string()
        );
        let unsigned_expected: TokenStream = "pub const J_TYPED: MyU64 = 999;".parse().unwrap();
        assert_eq!(
            alias_items
                .constant("Test", "J_TYPED")
                .unwrap()
                .write_sys()
                .to_string(),
            unsigned_expected.to_string()
        );

        let guid_generator = fixture(
            r#"
                #[win32]
                mod Test {
                    const IID_INTERFACE: GUID =
                        0x00000000_0000_0000_c000_000000000046;
                }
            "#,
        );
        let guid_items = guid_generator.win32_items().unwrap();
        let guid_expected: TokenStream = "pub const IID_INTERFACE: windows_sys::core::GUID = \
             windows_sys::core::GUID::from_u128(\
             0x00000000_0000_0000_c000_000000000046);"
            .parse()
            .unwrap();
        assert_eq!(
            guid_items
                .constant("Test", "IID_INTERFACE")
                .unwrap()
                .write_sys()
                .to_string(),
            guid_expected.to_string()
        );
    }
}
