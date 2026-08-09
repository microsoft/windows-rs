#![doc = include_str!("../readme.md")]

use windows_metadata2::{
    AnyRowId, AttributeArgument, AttributeValue, ConstantValue, Database, Entity, FileId,
    MethodSignature, TypeAttributes, TypeCategory, TypeDefinition, TypeKind,
    tables::{Field, MethodDef, TypeDef},
};

mod enum_model;
mod error;
mod guid;
mod model;
mod native;
mod native_constant;
mod native_function;
mod native_type;
mod struct_model;
mod tokens;
mod ty;
mod win32;

pub use enum_model::Enum;
pub use error::Error;
pub use model::{Value, Values};
pub use native_constant::Constant;
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

#[derive(Clone, Copy)]
struct ValueEntry {
    entity: Entity<TypeDef>,
    kind: ValueKind,
}

/// Owns metadata and the deterministic set of projected items.
pub struct Generator {
    database: Database,
    values: Vec<ValueEntry>,
}

/// A borrowed projected WinRT value item.
#[derive(Clone, Copy)]
pub struct ValueItem<'a> {
    definition: TypeDefinition<'a>,
    kind: ValueKind,
}

impl Generator {
    /// Selects projected items from an owned metadata database.
    pub fn new(database: Database) -> Result<Self, Error> {
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

            values.push((
                definition.namespace()?.to_string(),
                definition.name()?.to_string(),
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
        })
    }

    /// Returns the owned metadata database.
    pub const fn database(&self) -> &Database {
        &self.database
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

    fn fixture(source: &str) -> Generator {
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
        Generator::new(Database::new([image]).unwrap()).unwrap()
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
