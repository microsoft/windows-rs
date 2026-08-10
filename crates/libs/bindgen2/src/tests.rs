use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_metadata2::Image;

static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

fn generator() -> Generator {
    Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap()
    .generator(Request::all())
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
    fixture_metadata(source).generator(Request::all()).unwrap()
}

fn normalize_fn_parameters(stream: TokenStream) -> String {
    fn normalize(stream: TokenStream) -> TokenStream {
        use proc_macro2::{Delimiter, Group, TokenTree};

        let mut result = Vec::<TokenTree>::new();
        for token in stream {
            let TokenTree::Group(group) = token else {
                result.push(token);
                continue;
            };
            let mut tokens = normalize(group.stream()).into_iter().collect::<Vec<_>>();
            if group.delimiter() == Delimiter::Parenthesis {
                let mut normalized = Vec::new();
                let mut position = 0;
                let mut parameter_start = true;
                while position < tokens.len() {
                    let parameter_name = parameter_start
                        && matches!(tokens.get(position), Some(TokenTree::Ident(_)))
                        && matches!(
                            tokens.get(position + 1),
                            Some(TokenTree::Punct(colon)) if colon.as_char() == ':'
                        )
                        && !matches!(
                            tokens.get(position + 2),
                            Some(TokenTree::Punct(colon)) if colon.as_char() == ':'
                        );
                    if parameter_name {
                        position += 2;
                        continue;
                    }
                    parameter_start = matches!(
                        tokens.get(position),
                        Some(TokenTree::Punct(comma)) if comma.as_char() == ','
                    );
                    normalized.push(tokens[position].clone());
                    position += 1;
                }
                if matches!(
                    normalized.last(),
                    Some(TokenTree::Punct(comma)) if comma.as_char() == ','
                ) {
                    normalized.pop();
                }
                tokens = normalized;
            }
            result.push(TokenTree::Group(Group::new(
                group.delimiter(),
                tokens.into_iter().collect(),
            )));
        }
        result.into_iter().collect()
    }

    normalize(stream)
        .to_string()
        .replace(" , >", " >")
        .replace("> ;", ">;")
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
    assert!(
        actual.windows(2).all(|pair| {
            (&pair[0].0, &pair[0].1, pair[0].3) < (&pair[1].0, &pair[1].1, pair[1].3)
        })
    );
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
    let first = metadata.generator(Request::all()).unwrap();
    let second = metadata.generator(Request::all()).unwrap();

    assert!(std::ptr::eq(
        &first.shared.database,
        &second.shared.database
    ));
    assert!(Arc::ptr_eq(
        &first.shared.win32_catalogs,
        &second.shared.win32_catalogs
    ));
    assert_eq!(first.values().len(), second.values().len());
    let first = first.win32_items();
    let second = second.win32_items();
    assert_eq!(first.native_types().count(), second.native_types().count());
    assert_eq!(first.delegates().count(), second.delegates().count());
    assert_eq!(first.constants().count(), second.constants().count());
    assert_eq!(first.functions().count(), second.functions().count());
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
    let generator = generator();
    let values = generator.lower_values();
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
    let enum_generator = fixture(
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
    );
    let enum_values = enum_generator.lower_values();
    let enum_expected: TokenStream =
        include_str!("../../../tests/libs/bindgen/expected/winrt_enum.rs")
            .parse()
            .unwrap();
    assert_eq!(
        enum_values.write("Test", "Enum").unwrap().to_string(),
        enum_expected.to_string()
    );

    let struct_generator = fixture(
        r#"
            #[winrt]
            mod Test {
                struct Struct {
                    x: i32,
                    y: i32,
                }
            }
        "#,
    );
    let struct_values = struct_generator.lower_values();
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
        generator.render(Layout::Modules).unwrap().to_string(),
        expected.to_string()
    );
}

#[test]
fn render_selects_flat_output() {
    let metadata = fixture_metadata(include_str!(
        "../../../tests/libs/bindgen/input/struct_default_sys.rdl"
    ));
    let generator = metadata.generator(Request::all()).unwrap();
    let expected: TokenStream =
        include_str!("../../../tests/libs/bindgen/expected/struct_default_sys.rs")
            .parse()
            .unwrap();

    assert_eq!(
        generator.render(Layout::Flat).unwrap().to_string(),
        expected.to_string()
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
    let generator = metadata.generator(Request::all()).unwrap();

    assert!(matches!(
        generator.render(Layout::Flat),
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
fn flat_output_uses_unqualified_cross_namespace_references() {
    let metadata = fixture_metadata(
        r#"
            #[win32]
            mod First {
                struct Value {
                    inner: i32,
                }
            }
            #[win32]
            mod Second {
                struct Container {
                    value: First::Value,
                }
            }
            #[winrt]
            mod ManagedFirst {
                struct ManagedValue {
                    inner: i32,
                }
            }
            #[winrt]
            mod ManagedSecond {
                struct ManagedContainer {
                    value: ManagedFirst::ManagedValue,
                }
            }
        "#,
    );
    let generator = metadata.generator(Request::all()).unwrap();
    let output = generator.render(Layout::Flat).unwrap().to_string();

    assert!(output.contains("pub value : Value"));
    assert!(!output.contains("super :: First :: Value"));
    assert!(output.contains("pub value : ManagedValue"));
    assert!(!output.contains("super :: ManagedFirst :: ManagedValue"));

    let output = generator.render(Layout::Modules).unwrap().to_string();
    assert!(output.contains("super :: First :: Value"));
    assert!(output.contains("super :: ManagedFirst :: ManagedValue"));
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
    let generator = metadata.generator(Request::filtered(filter)).unwrap();
    let items = generator.win32_items();

    assert_eq!(generator.values().len(), 1);
    assert_eq!(items.native_types().count(), 2);
    assert_eq!(items.constants().count(), 1);
    assert_eq!(items.functions().count(), 0);

    let output = generator.render(Layout::Modules).unwrap().to_string();
    assert!(output.contains("pub mod First"));
    assert!(output.contains("pub const ONLY_FIRST"));
    assert!(output.contains("pub mod Second"));
    assert!(!output.contains("ONLY_SECOND"));
    assert!(output.contains("pub mod Managed"));
    assert!(output.contains("pub struct Kind"));
}

#[test]
fn winrt_filters_include_transitive_value_dependencies() {
    let metadata = fixture_metadata(
        r#"
            #[winrt]
            mod First {
                #[repr(i32)]
                enum Leaf {
                    Value = 0,
                }
            }
            #[winrt]
            mod Second {
                struct Middle {
                    leaf: First::Leaf,
                }
                struct Root {
                    middle: Middle,
                }
                struct Unused {
                    value: i32,
                }
            }
        "#,
    );
    let generator = metadata
        .generator(Request::filtered(Filter::names(["Root"])))
        .unwrap();

    assert_eq!(generator.values().len(), 3);
    let output = generator.render(Layout::Modules).unwrap().to_string();
    assert!(output.contains("pub struct Leaf"));
    assert!(output.contains("pub struct Middle"));
    assert!(output.contains("pub struct Root"));
    assert!(!output.contains("pub struct Unused"));
}

#[test]
fn winrt_value_closure_terminates_on_cycles() {
    let metadata = fixture_metadata(
        r#"
            #[winrt]
            mod Test {
                struct First {
                    second: Second,
                }
                struct Second {
                    first: First,
                }
            }
        "#,
    );
    let generator = metadata
        .generator(Request::filtered(Filter::names(["First"])))
        .unwrap();

    assert_eq!(generator.values().len(), 2);
    assert!(matches!(
        generator.render(Layout::Modules),
        Err(Error::RecursiveValue(name)) if name.starts_with("Test.")
    ));
}

#[test]
fn native_filters_include_transitive_supported_dependencies() {
    let metadata = fixture_metadata(
        r#"
            #[win32]
            mod Test {
                type Leaf = u32;
                struct Middle {
                    leaf: Leaf,
                }
                struct Root {
                    middle: Middle,
                }
                type ConstantType = u16;
                const VALUE: ConstantType = 42;
                extern fn Callback(value: Root) -> Leaf;
                #[library("test.dll")]
                extern fn Use(callback: Callback, value: *const Root) -> Middle;
            }
        "#,
    );
    let generator = metadata
        .generator(Request::filtered(Filter::names(["Use", "VALUE"])))
        .unwrap();
    let items = generator.win32_items();

    assert_eq!(items.native_types().count(), 4);
    assert_eq!(items.delegates().count(), 1);
    assert_eq!(items.constants().count(), 1);
    assert_eq!(items.functions().count(), 1);

    let output = generator.render(Layout::Modules).unwrap().to_string();
    for name in [
        "Leaf",
        "Middle",
        "Root",
        "ConstantType",
        "Callback",
        "VALUE",
        "Use",
    ] {
        assert!(output.contains(name), "{name} missing from {output}");
    }
}

#[test]
fn native_dependency_closure_keeps_architecture_variants() {
    let metadata = fixture_metadata(
        r#"
            #[win32]
            mod Test {
                #[arch(X86)]
                type ArchValue = i32;
                #[arch(X64 | Arm64)]
                type ArchValue = i64;
                #[library("test.dll")]
                extern fn Use(value: ArchValue);
            }
        "#,
    );
    let generator = metadata
        .generator(Request::filtered(Filter::names(["Use"])))
        .unwrap();
    let items = generator.win32_items();

    assert_eq!(items.native_types().count(), 2);
    let output = generator.render(Layout::Modules).unwrap().to_string();
    assert_eq!(output.matches("pub type ArchValue").count(), 2);
    assert!(output.contains("target_arch = \"x86\""));
    assert!(output.contains("target_arch = \"aarch64\""));
}

#[test]
fn native_interfaces_render_vtables_and_close_base_dependencies() {
    let metadata = fixture_metadata(
        r#"
            #[win32]
            mod Test {
                struct Value {
                    inner: i32,
                }
                interface IBase {
                    fn First(&self, value: Value) -> u32;
                }
                interface IDerived: IBase {
                    fn Second(&self, value: *mut Value);
                }
            }
        "#,
    );
    let generator = metadata
        .generator(Request::filtered(Filter::names(["IDerived"])))
        .unwrap();
    let items = generator.win32_items();

    assert_eq!(items.interfaces().count(), 2);
    assert_eq!(items.native_types().count(), 1);
    let output = generator.render(Layout::Modules).unwrap().to_string();
    let expected: TokenStream = r#"
        pub mod Test {
            #[repr(C)]
            pub struct IBase_Vtbl {
                pub First: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    Value
                ) -> u32,
            }
            #[repr(C)]
            pub struct IDerived_Vtbl {
                pub base__: IBase_Vtbl,
                pub Second: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut Value
                ),
            }
            #[repr(C)]
            #[derive(Clone, Copy, Default)]
            pub struct Value {
                pub inner: i32,
            }
        }
    "#
    .parse()
    .unwrap();
    assert_eq!(output, expected.to_string());
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
    let output = generator.render(Layout::Modules).unwrap().to_string();
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
    let items = generator.win32_items();

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
    let expected: TokenStream = include_str!("../../../tests/libs/bindgen/expected/enum_sys.rs")
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
fn native_alias_policy_requires_canonical_namespace() {
    let metadata = fixture_metadata(
        r#"
            #[win32]
            mod Custom {
                type PWSTR = u32;
            }
            #[win32]
            mod Api {
                #[library("test.dll")]
                extern fn Use(value: Custom::PWSTR);
            }
        "#,
    );
    let output = metadata
        .generator(Request::all())
        .unwrap()
        .render(Layout::Modules)
        .unwrap()
        .to_string();

    assert!(output.contains("pub type PWSTR = u32"));
    assert!(output.contains("value : super :: Custom :: PWSTR"));
    assert!(!output.contains("value : super :: Custom :: PCWSTR"));
}

#[test]
fn true_nested_native_types_match_existing_golden_tokens() {
    let generator = fixture(include_str!(
        "../../../tests/libs/bindgen/input/struct_nested_anon_sys.rdl"
    ));
    let items = generator.win32_items();
    assert_eq!(generator.shared.win32_catalogs.nested_type_count(), 10);
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
        generator.render(Layout::Modules).unwrap().to_string(),
        expected.to_string()
    );
}

#[test]
fn native_default_policy_matches_existing_golden_tokens() {
    let generator = fixture(include_str!(
        "../../../tests/libs/bindgen/input/struct_default_sys.rdl"
    ));
    let items = generator.win32_items();
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
    let items = generator.win32_items();
    let delegates = items.delegates().collect::<Result<Vec<_>, _>>().unwrap();
    let delegates = delegates.iter().map(Delegate::write_sys);
    let actual = quote! { #(#delegates)* };
    let expected: TokenStream = include_str!("../../../tests/libs/bindgen/expected/callback.rs")
        .parse()
        .unwrap();
    assert_eq!(
        actual.to_string().replace("> ;", ">;"),
        expected.to_string()
    );
    let expected = quote! { pub mod Test { #expected } };
    assert_eq!(
        generator
            .render(Layout::Modules)
            .unwrap()
            .to_string()
            .replace("> ;", ">;"),
        expected.to_string()
    );

    let generator = fixture(
        r#"
            #[win32]
            mod Test {
                type First = u32;
                type Second = u64;
                extern fn Callback(value: u32) -> u32;
            }
        "#,
    );
    assert_eq!(
        generator
            .render(Layout::Modules)
            .unwrap()
            .to_string()
            .matches("pub type Callback")
            .count(),
        1
    );

    let metadata = fixture_metadata(include_str!(
        "../../../tests/libs/bindgen/input/arch_delegate_dependency_sys.rdl"
    ));
    let generator = metadata
        .generator(Request::filtered(Filter::names(["UsesCallback"])))
        .unwrap();
    let expected: TokenStream =
        include_str!("../../../tests/libs/bindgen/expected/arch_delegate_dependency_sys.rs")
            .parse()
            .unwrap();
    let expected = quote! { pub mod Test { #expected } };
    assert_eq!(
        generator
            .render(Layout::Modules)
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
    let items = generator.win32_items();
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
    let items = generator.win32_items();
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
        generator.render(Layout::Modules).unwrap().to_string(),
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
    let items = generator.win32_items();
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
    let items = generator.win32_items();
    assert_eq!(
        [
            items.native_types().count(),
            items.constants().count(),
            items.functions().count(),
            items.interfaces().count(),
        ],
        [30_109, 83_641, 14_559, 4_290]
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
    let items = generator.win32_items();

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
    let pointer_items = pointer_generator.win32_items();
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
    let alias_items = alias_generator.win32_items();
    let signed_expected: TokenStream = "pub const I_TYPED: MyI32 = 0x2A_u32 as _;".parse().unwrap();
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
    let guid_items = guid_generator.win32_items();
    let guid_expected: TokenStream = "pub const IID_INTERFACE: GUID = GUID { \
         data1: 0x00000000, data2: 0x0000, data3: 0x0000, \
         data4: [192, 0, 0, 0, 0, 0, 0, 70], };"
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

#[test]
fn tool_bindings_sys_requests_match_committed_output() {
    let metadata = Metadata::from_images([
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");

    for request in [
        "cppwinrt.txt",
        "future_impl.txt",
        "metadata.txt",
        "registry.txt",
        "result.txt",
        "services.txt",
        "strings.txt",
        "threading.txt",
        "version.txt",
    ] {
        let request_path = root.join("crates/tools/bindings/src").join(request);
        let request_text = std::fs::read_to_string(&request_path).unwrap();
        let mut output = None;
        let mut filters = Vec::new();
        let mut reading_filters = false;
        for line in request_text.lines().map(str::trim) {
            if let Some(path) = line.strip_prefix("--out ") {
                output = Some(path.to_string());
                reading_filters = false;
            } else if line == "--filter" {
                reading_filters = true;
            } else if line.starts_with("--") {
                reading_filters = false;
            } else if reading_filters && !line.is_empty() && !line.starts_with('#') {
                assert!(
                    line.bytes()
                        .all(|byte| byte == b'_' || byte.is_ascii_alphanumeric()),
                    "{request} requires unsupported filter syntax: {line}"
                );
                filters.push(line.to_string());
            }
        }
        let actual = metadata
            .generator(Request::filtered(Filter::names(filters)))
            .unwrap()
            .render(Layout::Flat)
            .unwrap();
        let expected: TokenStream = std::fs::read_to_string(root.join(output.unwrap()))
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(
            normalize_fn_parameters(actual),
            normalize_fn_parameters(expected),
            "{request} differs"
        );
    }
}
