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

struct ToolRequest {
    output: String,
    filter: Filter,
    implementations: Vec<String>,
    minimal: bool,
    dead_code: bool,
}

fn parse_tool_request(metadata: &Metadata, source: &str) -> ToolRequest {
    enum Section {
        None,
        Implement,
        Filter,
    }

    let mut output = None;
    let mut filter = Filter::new();
    let mut implementations = Vec::new();
    let mut minimal = false;
    let mut dead_code = false;
    let mut section = Section::None;

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        if line.starts_with("--") {
            section = match line {
                "--implement" => Section::Implement,
                "--filter" => Section::Filter,
                _ => Section::None,
            };
            minimal |= line.split_whitespace().any(|part| part == "--minimal");
            dead_code |= line.split_whitespace().any(|part| part == "--dead-code");
            if let Some(path) = line.strip_prefix("--out ") {
                output = Some(path.to_string());
            }
            continue;
        }
        match section {
            Section::None => {}
            Section::Implement => implementations.push(line.to_string()),
            Section::Filter => include_tool_filter(metadata, &mut filter, line),
        }
    }

    ToolRequest {
        output: output.unwrap(),
        filter,
        implementations,
        minimal,
        dead_code,
    }
}

fn include_tool_filter(metadata: &Metadata, filter: &mut Filter, path: &str) {
    if let Some((prefix, names)) = path
        .strip_suffix('}')
        .and_then(|path| path.rsplit_once("::{"))
    {
        let parts = tool_path(prefix);
        let (parent, ty) = parts.split_at(parts.len() - 1);
        if parent.is_empty() {
            let namespaces = tool_type_namespaces(metadata, ty[0]);
            assert!(!namespaces.is_empty(), "unresolved tool filter: {path}");
            for namespace in namespaces {
                for method in names.split(',').map(str::trim) {
                    filter.include_method(namespace, ty[0], method);
                }
            }
            return;
        }
        if tool_type_exists(metadata, &parent.join("."), ty[0]) {
            for method in names.split(',').map(str::trim) {
                filter.include_method(parent.join("."), ty[0], method);
            }
        } else {
            let namespace = parts.join(".");
            for name in names.split(',').map(str::trim) {
                filter.include_item(&namespace, name);
            }
        }
        return;
    }

    let parts = tool_path(path);
    if parts.len() == 1 {
        filter.include_name(parts[0]);
        return;
    }
    let (namespace, name) = parts.split_at(parts.len() - 1);
    if tool_type_exists(metadata, &namespace.join("."), name[0]) {
        filter.include_item(namespace.join("."), name[0]);
        return;
    }
    if tool_namespace_exists(metadata, &namespace.join(".")) {
        filter.include_item(namespace.join("."), name[0]);
        return;
    }
    let (namespace, ty) = namespace.split_at(namespace.len() - 1);
    if namespace.is_empty() {
        let namespaces = tool_type_namespaces(metadata, ty[0]);
        assert!(!namespaces.is_empty(), "unresolved tool filter: {path}");
        for namespace in namespaces {
            filter.include_method(namespace, ty[0], name[0]);
        }
        return;
    }
    assert!(
        tool_type_exists(metadata, &namespace.join("."), ty[0]),
        "unresolved tool filter: {path}"
    );
    filter.include_method(namespace.join("."), ty[0], name[0]);
}

fn tool_type_namespaces<'a>(metadata: &'a Metadata, ty: &str) -> Vec<&'a str> {
    metadata
        .shared
        .database
        .type_names()
        .filter_map(|(namespace, name, _)| (name == ty).then_some(namespace))
        .collect()
}

fn tool_path(path: &str) -> Vec<&str> {
    path.split([':', '.'])
        .filter(|part| !part.is_empty())
        .collect()
}

fn tool_type_exists(metadata: &Metadata, namespace: &str, name: &str) -> bool {
    metadata
        .shared
        .winrt_entries
        .iter()
        .any(|(candidate_namespace, candidate_name, _)| {
            candidate_namespace == namespace && candidate_name == name
        })
        || !metadata
            .shared
            .database
            .type_definitions(namespace, name)
            .is_empty()
}

fn tool_namespace_exists(metadata: &Metadata, namespace: &str) -> bool {
    metadata
        .shared
        .database
        .type_names()
        .any(|(candidate, _, _)| candidate == namespace)
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

fn normalize_existing_output(tokens: TokenStream) -> String {
    normalize_fn_parameters(tokens)
        .replace(":: <", "::<")
        .replace("> ::", ">::")
        .replace("> :", ">:")
        .replace("> ,", ">,")
        .replace(">, >", "> >")
        .replace("> >", ">>")
        .replace("> >", ">>")
        .replace("& 'static", "&'static")
        .replace("& *", "&*")
        .replace("& <", "&<")
        .replace("< *", "<*")
        .replace("'static , {", "'static {")
        .replace("? ;", "?;")
        .replace(
            "Err (err) => err . into () }",
            "Err (err) => err . into () , }",
        )
        .replace(" + Send + 'static", " + 'static")
}

fn normalize_minimal_delegate_constructors(tokens: TokenStream) -> String {
    let mut output = normalize_existing_output(tokens);
    let mut search = 0;
    while let Some(relative) = output[search..].find("let handler") {
        let start = search + relative;
        let mut depth = 0;
        let mut end = None;
        for (offset, character) in output[start..].char_indices() {
            match character {
                '{' => depth += 1,
                '}' => depth -= 1,
                ';' if depth == 0 => {
                    end = Some(start + offset + 1);
                    break;
                }
                _ => {}
            }
        }
        let end = end.unwrap();
        output.replace_range(start..end, "let handler = HANDLER;");
        search = start + "let handler = HANDLER;".len();
    }

    let mut search = 0;
    while let Some(relative) = output[search..].find("pub fn new") {
        let method = search + relative;
        let Some(open) = output[..method].rfind('{') else {
            break;
        };
        let Some(start) = output[..open].rfind("impl ") else {
            break;
        };
        let mut depth = 0;
        let mut end = None;
        for (offset, character) in output[open..].char_indices() {
            match character {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(open + offset + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        let end = end.unwrap();
        if output[start..end].contains("DelegateBox") {
            output.replace_range(start..end, "");
            search = start;
        } else {
            search = method + "pub fn new".len();
        }
    }
    output
        .replace(">, } ;", "> } ;")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
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
            WinrtKind::Enum => 0,
            WinrtKind::Struct => 1,
            WinrtKind::Delegate | WinrtKind::Interface | WinrtKind::Class => unreachable!(),
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
    assert_eq!(first.values().count(), second.values().count());
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
                TypeCategory::Enum => WinrtKind::Enum,
                TypeCategory::Struct => WinrtKind::Struct,
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
        generator
            .render_projection(Layout::Flat, Projection::Sys)
            .unwrap()
            .to_string(),
        expected.to_string()
    );
}

#[test]
fn winrt_delegates_match_existing_golden_tokens() {
    for (name, source, expected) in [
        (
            "delegate",
            include_str!("../../../tests/libs/bindgen/input/delegate.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/delegate.rs"),
        ),
        (
            "delegate_generic",
            include_str!("../../../tests/libs/bindgen/input/delegate_generic.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/delegate_generic.rs"),
        ),
        (
            "delegate_types",
            include_str!("../../../tests/libs/bindgen/input/delegate_types.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/delegate_types.rs"),
        ),
    ] {
        let generator = fixture(source);
        let expected: TokenStream = expected.parse().unwrap();
        assert_eq!(
            normalize_fn_parameters(generator.render(Layout::Flat).unwrap())
                .replace(":: <", "::<")
                .replace("> ::", ">::")
                .replace("> ,", ">,")
                .replace(">, >", "> >")
                .replace(" , {", " {")
                .replace(" , }", " }"),
            normalize_fn_parameters(expected)
                .replace(":: <", "::<")
                .replace("> ::", ">::")
                .replace("> ,", ">,")
                .replace(" , {", " {")
                .replace(" , }", " }"),
            "{name}"
        );
    }
}

#[test]
fn winrt_interfaces_match_existing_golden_tokens() {
    for (name, source, expected) in [
        (
            "interface_void",
            include_str!("../../../tests/libs/bindgen/input/interface_void.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/interface_void.rs"),
        ),
        (
            "interface",
            include_str!("../../../tests/libs/bindgen/input/interface.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/interface.rs"),
        ),
        (
            "interface_generic",
            include_str!("../../../tests/libs/bindgen/input/interface_generic.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/interface_generic.rs"),
        ),
        (
            "interface_hierarchy",
            include_str!("../../../tests/libs/bindgen/input/interface_hierarchy.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/interface_hierarchy.rs"),
        ),
        (
            "event",
            include_str!("../../../tests/libs/bindgen/input/event.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/event.rs"),
        ),
    ] {
        let expected: TokenStream = expected.parse().unwrap();
        let normalize = |tokens| {
            normalize_fn_parameters(tokens)
                .replace(":: <", "::<")
                .replace("> ::", ">::")
                .replace("> ,", ">,")
                .replace(">, >", "> >")
                .replace("> >", ">>")
                .replace("& 'static", "&'static")
                .replace("& *", "&*")
                .replace("& <", "&<")
                .replace("> :", ">:")
                .replace("+ 'static ,", "+ 'static")
                .replace("? ;", "?;")
                .replace(
                    "Err (err) => err . into () }",
                    "Err (err) => err . into () , }",
                )
        };
        assert_eq!(
            normalize(fixture(source).render(Layout::Flat).unwrap()),
            normalize(expected),
            "{name}"
        );
    }
}

#[test]
fn winrt_interface_corpus_lowers_and_renders() {
    let generator = generator();
    let values = generator.lower_values();
    let mut count = 0;
    let mut unsupported = Vec::new();
    for entry in generator
        .winrt
        .iter()
        .filter(|entry| entry.kind == WinrtKind::Interface)
    {
        let definition = generator.shared.database.definition(entry.entity).unwrap();
        let namespace = definition.namespace().unwrap();
        let name = definition.name().unwrap();
        let result = winrt_interface::Interface::lower(
            &generator.shared.database,
            definition,
            &generator.shared.interface_relationships,
            &format!("{namespace}.{name}"),
        )
        .and_then(|model| {
            for projection in [Projection::Default, Projection::Minimal] {
                model.write(
                    values,
                    namespace,
                    Layout::Modules,
                    projection,
                    &MemberSelection::All,
                    None,
                    false,
                )?;
            }
            Ok(())
        });
        if let Err(error) = result {
            unsupported.push((format!("{namespace}.{name}"), error.to_string()));
        }

        count += 1;
    }

    assert_eq!(count, 8_105);
    assert!(unsupported.is_empty(), "{unsupported:#?}");
}

#[test]
fn filtered_winrt_interfaces_close_required_and_method_dependencies() {
    let metadata = fixture_metadata(
        r#"
            #[winrt]
            mod Test {
                #[repr(i32)]
                enum Kind {
                    First = 0,
                }

                interface IBase {
                    fn Get(&self) -> Kind;
                }

                interface IDerived: IBase {
                    fn Set(&self, value: Kind);
                }
            }
        "#,
    );
    let generator = metadata
        .generator(Request::filtered(Filter::names(["IDerived"])))
        .unwrap();
    let output = generator.render(Layout::Flat).unwrap().to_string();
    assert!(output.contains("pub struct Kind"), "{output}");
    assert!(output.contains("define_interface ! (IBase"));
    assert!(output.contains("define_interface ! (IDerived"));
    assert!(output.contains("required_hierarchy ! (IDerived , IBase"));
}

#[test]
fn generic_required_interface_substitutes_method_types() {
    let output = fixture(
        r#"
            #[winrt]
            mod Test {
                interface IBase<T> {
                    fn Get(&self) -> T;
                }

                interface IMiddle<T>: IBase<T> {
                    fn Put(&self, value: T);
                }

                interface IDerived: IMiddle<i32> {
                    fn Set(&self, value: i32);
                }
            }
        "#,
    )
    .render(Layout::Flat)
    .unwrap()
    .to_string();
    assert!(output.contains("required_hierarchy ! (IDerived , IBase < i32 > , IMiddle < i32 >"));
    assert!(output.contains("pub trait IDerived_Impl : IBase_Impl < i32 > + IMiddle_Impl < i32 >"));
    let normalized = normalize_fn_parameters(output.parse().unwrap());
    assert!(normalized.contains("pub fn Get (& self) -> windows_core :: Result < i32 >"));
}

#[test]
fn winrt_classes_match_existing_golden_tokens() {
    for (name, source, expected, layout) in [
        (
            "class",
            include_str!("../../../tests/libs/bindgen/input/class.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/class.rs"),
            Layout::Flat,
        ),
        (
            "class_hierarchy",
            include_str!("../../../tests/libs/bindgen/input/class_hierarchy.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/class_hierarchy.rs"),
            Layout::Modules,
        ),
        (
            "class_static",
            include_str!("../../../tests/libs/bindgen/input/class_static.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/class_static.rs"),
            Layout::Flat,
        ),
    ] {
        let normalize = |tokens| {
            normalize_fn_parameters(tokens)
                .replace(":: <", "::<")
                .replace("> ::", ">::")
                .replace("> ,", ">,")
                .replace(">, >", "> >")
                .replace("> >", ">>")
                .replace("& 'static", "&'static")
                .replace("& *", "&*")
                .replace("& <", "&<")
                .replace("? ;", "?;")
                .replace(
                    "Err (err) => err . into () }",
                    "Err (err) => err . into () , }",
                )
        };
        assert_eq!(
            normalize(fixture(source).render(layout).unwrap()),
            normalize(expected.parse().unwrap()),
            "{name}"
        );
    }
}

#[test]
fn focused_minimal_winrt_output_matches_existing_tokens() {
    let normalize = |tokens| {
        normalize_fn_parameters(tokens)
            .replace(":: <", "::<")
            .replace("> ::", ">::")
            .replace("> ,", ">,")
            .replace(">, >", "> >")
            .replace("> >", ">>")
            .replace("& 'static", "&'static")
            .replace("& *", "&*")
            .replace("& <", "&<")
            .replace("? ;", "?;")
            .replace(
                "Err (err) => err . into () }",
                "Err (err) => err . into () , }",
            )
    };
    for (name, source, expected) in [
        (
            "enum",
            include_str!("../../../tests/libs/bindgen/input/winrt_enum_minimal.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/winrt_enum_minimal.rs"),
        ),
        (
            "struct",
            include_str!("../../../tests/libs/bindgen/input/winrt_struct_minimal.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/winrt_struct_minimal.rs"),
        ),
        (
            "delegate",
            include_str!("../../../tests/libs/bindgen/input/delegate_minimal.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/delegate_minimal.rs"),
        ),
        (
            "interface",
            include_str!("../../../tests/libs/bindgen/input/interface_minimal.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/interface_minimal.rs"),
        ),
        (
            "class",
            include_str!("../../../tests/libs/bindgen/input/class_minimal.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/class_minimal.rs"),
        ),
    ] {
        let actual = fixture(source)
            .render_projection(Layout::Flat, Projection::Minimal)
            .unwrap();
        let expected = expected.parse().unwrap();
        let actual = normalize(actual);
        if name == "delegate" {
            assert!(actual.contains(" + Send + 'static"));
        }
        assert_eq!(
            actual.replace(" + Send + 'static", " + 'static"),
            normalize(expected),
            "{name}"
        );
    }
}

#[test]
fn minimal_delegate_and_event_wrappers_preserve_abi_safety() {
    let output = fixture(include_str!(
        "../../../tests/libs/bindgen/input/event_minimal.rdl"
    ))
    .render_projection(Layout::Flat, Projection::Minimal)
    .unwrap()
    .to_string();
    assert!(output.contains("F : Fn (i32) + Send + 'static"));
    assert!(output.contains("let handler = < Delegate > :: new (handler)"));
    assert!(output.contains("windows_core :: EventRevoker :: new"));

    let output = fixture(
        r#"
            #[winrt]
            mod Test {
                delegate fn Transform(value: i32) -> i32;
            }
        "#,
    )
    .render_projection(Layout::Flat, Projection::Minimal)
    .unwrap()
    .to_string();
    assert!(output.contains("F : Fn (i32) -> i32 + Send + 'static"));
    assert!(output.contains("result__ . write ((this . invoke) (value))"));
}

#[test]
fn forwarded_event_revokers_retain_the_cast_interface() {
    let output = fixture(
        r#"
            #[winrt]
            mod Test {
                interface IBase {
                    #[special]
                    fn add_Changed(&self, handler: Handler) -> i64;
                    #[special]
                    fn remove_Changed(&self, token: i64);
                }
                interface IDerived: IBase {}
                delegate fn Handler(value: i32);
            }
        "#,
    )
    .render(Layout::Flat)
    .unwrap()
    .to_string();
    assert!(output.contains("EventRevoker :: new (this . clone ()"));
}

#[test]
fn focused_winrt_member_filters_match_existing_tokens() {
    for (name, source, expected, ty, methods, projection) in [
        (
            "interface",
            include_str!("../../../tests/libs/bindgen/input/method_filter_allow.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/method_filter_allow.rs"),
            "Interface",
            &["First"][..],
            Projection::Default,
        ),
        (
            "class",
            include_str!("../../../tests/libs/bindgen/input/method_filter_class.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/method_filter_class.rs"),
            "Widget",
            &["Act"][..],
            Projection::Default,
        ),
        (
            "property",
            include_str!("../../../tests/libs/bindgen/input/method_filter_property.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/method_filter_property.rs"),
            "Interface",
            &["get_Value", "put_Value"][..],
            Projection::Default,
        ),
        (
            "minimal values",
            include_str!("../../../tests/libs/bindgen/input/minimal_deps.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/minimal_deps.rs"),
            "Interface",
            &["Method"][..],
            Projection::Minimal,
        ),
        (
            "minimal delegate",
            include_str!("../../../tests/libs/bindgen/input/minimal_delegate_dep.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/minimal_delegate_dep.rs"),
            "Sink",
            &["Subscribe"][..],
            Projection::Minimal,
        ),
        (
            "minimal class",
            include_str!("../../../tests/libs/bindgen/input/minimal_class_dep.rdl"),
            include_str!("../../../tests/libs/bindgen/expected/minimal_class_dep.rs"),
            "Factory",
            &["Create"][..],
            Projection::Minimal,
        ),
    ] {
        let metadata = fixture_metadata(source);
        let mut filter = Filter::new();
        for method in methods {
            filter.include_method("Test", ty, *method);
        }
        let actual = metadata
            .generator(Request::filtered(filter))
            .unwrap()
            .render_projection(Layout::Flat, projection)
            .unwrap();
        let expected = expected.parse().unwrap();
        assert_eq!(
            normalize_existing_output(actual),
            normalize_existing_output(expected),
            "{name}"
        );
    }
}

#[test]
fn member_filtered_events_retain_the_remove_abi() {
    let metadata = fixture_metadata(include_str!(
        "../../../tests/libs/bindgen/input/event_minimal.rdl"
    ));
    let mut filter = Filter::new();
    filter.include_method("Test", "Interface", "add_Changed");
    let output = metadata
        .generator(Request::filtered(filter))
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();

    assert!(output.contains("pub fn Changed"));
    assert!(output.contains("pub Changed : unsafe extern \"system\" fn"));
    assert!(output.contains("pub RemoveChanged : unsafe extern \"system\" fn"));
    assert!(!output.contains("pub fn Method"));
    assert!(output.contains("pub Method : unsafe extern \"system\" fn"));
}

#[test]
fn member_filters_retain_the_vtable_prefix() {
    let metadata = fixture_metadata(
        r#"
            #[winrt]
            mod Test {
                struct Value {
                    value: i32,
                }
                interface Interface {
                    fn First(&self) -> Value;
                    fn Second(&self) -> i32;
                }
            }
        "#,
    );
    let mut filter = Filter::new();
    filter.include_method("Test", "Interface", "Second");
    let output = metadata
        .generator(Request::filtered(filter))
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();

    assert!(output.contains("pub struct Value"));
    assert!(!output.contains("pub fn First"));
    assert!(output.contains("pub fn Second"));
    assert!(output.contains("pub First : unsafe extern \"system\" fn"));
    assert!(output.contains("pub Second : unsafe extern \"system\" fn"));
}

#[test]
fn explicit_implementations_control_minimal_vtables() {
    let metadata = fixture_metadata(
        r#"
            #[winrt]
            mod Test {
                struct Value {
                    value: i32,
                }
                interface Interface {
                    fn First(&self) -> Value;
                    fn Second(&self) -> i32;
                }
            }
        "#,
    );
    let mut filter = Filter::new();
    filter.include_method("Test", "Interface", "Second");
    let output = metadata
        .generator(
            Request::filtered(filter)
                .implementations(Filter::new())
                .projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();
    assert!(!output.contains("pub struct Value"));
    assert!(!output.contains("Interface_Impl"));
    assert!(output.contains("First : usize"));
    assert!(output.contains("pub Second : unsafe extern \"system\" fn"));

    let mut filter = Filter::new();
    filter.include_item("Test", "Interface");
    let mut implementations = Filter::new();
    implementations.include_item("Test", "Interface");
    let output = metadata
        .generator(
            Request::filtered(filter)
                .implementations(implementations)
                .projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();
    assert!(output.contains("pub trait Interface_Impl"));
    assert!(output.contains("pub struct Value"));
}

#[test]
fn implement_all_emits_exclusive_interface_implementations() {
    let metadata = fixture_metadata(
        r#"
            #[winrt]
            mod Windows {
                mod Foundation {
                    mod Metadata {
                        attribute ExclusiveToAttribute {
                            fn(r#type: Type);
                        }
                    }
                }
            }
            #[winrt]
            mod Test {
                class Class {
                    IClass,
                }
                #[Windows::Foundation::Metadata::ExclusiveTo(Class)]
                interface IClass {
                    fn Method(&self) -> i32;
                }
            }
        "#,
    );
    let client = metadata
        .generator(Request::all())
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();
    assert!(!client.contains("pub trait IClass_Impl"), "{client}");

    let output = metadata
        .generator(Request::all().implement_all())
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();

    assert!(output.contains("pub trait IClass_Impl"), "{output}");
    assert!(output.contains("pub const fn new < Identity : IClass_Impl"));
}

#[test]
fn rich_native_functions_wrap_direct_and_output_returns() {
    let output = fixture(
        r#"
            #[win32]
            mod Test {
                #[library("test.dll")]
                extern fn Direct(value: i32) -> u32;
                #[library("test.dll")]
                extern fn Output(#[out] value: *mut u32);
                #[library("test.dll")]
                extern fn OutputPointers(#[out] first: *mut u32, #[out] second: *mut u32);
            }
        "#,
    )
    .render(Layout::Flat)
    .unwrap()
    .to_string();

    assert!(
        output.contains("pub unsafe fn Direct (value : i32) -> u32"),
        "{output}"
    );
    assert!(
        output.contains("pub unsafe fn Output () -> u32"),
        "{output}"
    );
    assert!(output.contains("Output (& mut result__)"), "{output}");
    assert!(
        output.contains("pub unsafe fn OutputPointers (first : * mut u32 , second : * mut u32)"),
        "{output}"
    );
    assert!(
        output.contains("OutputPointers (first as _ , second as _)"),
        "{output}"
    );
}

#[test]
fn native_custom_derives_join_sys_struct_derives() {
    let output = fixture_metadata(
        r#"
            #[win32]
            mod Test {
                struct Value {
                    field: u32,
                }
            }
        "#,
    )
    .generator(Request::all().sys().derive("Value", "Debug"))
    .unwrap()
    .render(Layout::Flat)
    .unwrap()
    .to_string();

    assert!(
        output.contains("derive (Clone , Copy , Debug , Default)"),
        "{output}"
    );
}

#[test]
fn class_overloads_are_named_across_exclusive_interfaces() {
    let output = fixture(
        r#"
            #[winrt]
            mod Windows {
                mod Foundation {
                    mod Metadata {
                        attribute ExclusiveToAttribute {
                            fn(r#type: Type);
                        }
                        attribute OverloadAttribute {
                            fn(method: String);
                        }
                    }
                }
            }
            #[winrt]
            mod Test {
                class D {
                    ID,
                    ID2,
                }
                #[Windows::Foundation::Metadata::ExclusiveTo(D)]
                interface ID {
                    #[Windows::Foundation::Metadata::Overload("Method")]
                    fn Method(&self);
                    #[Windows::Foundation::Metadata::Overload("Method2")]
                    fn Method(&self, value: i32);
                }
                #[Windows::Foundation::Metadata::ExclusiveTo(D)]
                interface ID2 {
                    #[Windows::Foundation::Metadata::Overload("Method")]
                    fn Method(&self, value: i32, value2: i32);
                    #[Windows::Foundation::Metadata::Overload("Method2")]
                    fn Method(&self, value: i32, value2: i32, value3: i32);
                }
                class E {
                    IE,
                }
                #[Windows::Foundation::Metadata::ExclusiveTo(E)]
                interface IE {
                    #[Windows::Foundation::Metadata::Overload("MethodOne")]
                    fn Method(&self);
                    #[Windows::Foundation::Metadata::Overload("MethodTwo")]
                    fn Method(&self, value: i32);
                }
            }
        "#,
    )
    .render(Layout::Flat)
    .unwrap()
    .to_string();

    assert!(output.contains("pub fn Method (& self"));
    assert!(output.contains("pub fn Method2 (& self"));
    assert!(output.contains("pub fn Method3 (& self"));
    assert!(output.contains("pub fn Method4 (& self"));
    assert!(output.contains("pub fn MethodOne (& self"));
    assert!(output.contains("pub fn MethodTwo (& self"));
    assert!(!output.contains("pub fn Method22 (& self"));
}

#[test]
fn winrt_producer_output_arrays_use_array_proxy() {
    let output = fixture(
        r#"
            #[winrt]
            mod Test {
                interface Interface {
                    fn GetValues(&self, values: &mut [i32]);
                }
            }
        "#,
    )
    .render(Layout::Flat)
    .unwrap()
    .to_string();

    assert!(output.contains("values : & mut windows_core :: Array < i32 >"));
    assert!(output.contains("values_array_size : * mut u32"));
    assert!(output.contains("windows_core :: imp :: array_proxy"));
}

#[test]
fn winrt_values_preserve_field_names_and_array_element_types() {
    let output = fixture_metadata(
        r#"
            #[winrt]
            mod Test {
                struct Value {
                    PascalCase: i32,
                }
                interface Interface {
                    fn GetValues(&self, values: &mut [Object]);
                }
            }
        "#,
    )
    .generator(Request::all().preserve_field_names())
    .unwrap()
    .render(Layout::Flat)
    .unwrap()
    .to_string();

    assert!(output.contains("pub PascalCase : i32"), "{output}");
    assert!(
        output.contains("values : & mut windows_core :: Array < windows_core :: IInspectable >"),
        "{output}"
    );
    assert!(
        output.contains("values : * mut * mut windows_core :: IInspectable"),
        "{output}"
    );
}

#[test]
fn winrt_reference_conveniences_use_external_crate() {
    let metadata = fixture_metadata(
        r#"
            #[winrt]
            mod Windows {
                mod Foundation {
                    interface IReference<T> {
                        fn Value(&self) -> T;
                    }
                }
            }
            #[winrt]
            mod Test {
                interface Interface {
                    fn Reference(&self) -> Windows::Foundation::IReference<i32>;
                    fn SetReference(
                        &self,
                        value: Windows::Foundation::IReference<i32>,
                    );
                }
            }
        "#,
    );
    let mut filter = Filter::new();
    filter.include_namespace("Test");
    let output = metadata
        .generator(Request::filtered(filter).implement_all())
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();

    assert!(
        output.contains("pub fn Reference (& self ,) -> windows_core :: Result < i32 >"),
        "{output}"
    );
    assert!(output.contains("value : Option < i32 >"));
    assert!(output.contains("windows_reference :: IReference < i32 >"));
    assert!(!output.contains("define_interface ! (IReference"));
}

#[test]
fn winrt_noexcept_methods_are_infallible() {
    let metadata = fixture_metadata(
        r#"
            #[winrt]
            mod Windows {
                mod Foundation {
                    mod Metadata {
                        attribute NoExceptionAttribute {
                            fn();
                        }
                    }
                }
            }
            #[winrt]
            mod Test {
                interface Interface {
                    #[Windows::Foundation::Metadata::NoException]
                    fn Value(&self) -> i32;
                    #[Windows::Foundation::Metadata::NoException]
                    fn SetValue(&self, value: i32);
                }
            }
        "#,
    );
    let output = metadata
        .generator(Request::all().implement_all())
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();

    assert!(
        output.contains("pub fn Value (& self ,) -> i32"),
        "{output}"
    );
    assert!(
        output.contains("pub fn SetValue (& self , value : i32 ,)"),
        "{output}"
    );
    assert!(output.contains("fn Value (& self ,) -> i32"), "{output}");
    assert!(
        output.contains("fn SetValue (& self , value : i32)"),
        "{output}"
    );
    assert!(
        output.contains("debug_assert ! (hresult__ . 0 == 0)"),
        "{output}"
    );
}

#[test]
fn composable_factories_emit_regular_and_compose_methods() {
    let output = fixture(
        r#"
            #[winrt]
            mod Windows {
                mod Foundation {
                    mod Metadata {
                        #[repr(i32)]
                        enum CompositionType {
                            Protected = 1,
                            Public = 2,
                        }
                        attribute ComposableAttribute {
                            fn(
                                r#type: Type,
                                compositionType: CompositionType,
                                version: u32,
                            );
                        }
                    }
                }
            }
            #[winrt]
            mod Test {
                #[Windows::Foundation::Metadata::Composable(
                    IComposableFactory,
                    Public,
                    65536,
                )]
                class Composable {
                    IComposable,
                }
                interface IComposable {}
                interface IComposableFactory {
                    fn CreateInstance(
                        &self,
                        baseInterface: Object,
                        innerInterface: &mut Object,
                    ) -> Composable;
                    fn WithValue(
                        &self,
                        value: i32,
                        baseInterface: Object,
                        innerInterface: &mut Object,
                    ) -> Composable;
                }
            }
        "#,
    )
    .render(Layout::Flat)
    .unwrap()
    .to_string();

    assert!(output.contains("pub fn new ()"), "{output}");
    assert!(
        output.contains("pub fn compose < T > (compose : T)"),
        "{output}"
    );
    assert!(output.contains("pub fn WithValue (value : i32"), "{output}");
    assert!(
        output.contains("pub fn WithValue_compose < T > (value : i32 , compose : T)"),
        "{output}"
    );
    assert!(output.contains("T : windows_core :: Compose"), "{output}");
}

#[test]
fn class_member_filters_route_static_methods() {
    let metadata = fixture_metadata(include_str!(
        "../../../tests/libs/bindgen/input/class_static.rdl"
    ));
    let mut filter = Filter::new();
    filter.include_method("Test", "Class", "Create");
    let output = metadata
        .generator(Request::filtered(filter))
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();

    assert!(output.contains("pub fn Create"));
    assert!(!output.contains("pub fn Method"));
    assert!(output.contains("pub fn new"));
}

#[test]
fn interface_member_filters_follow_required_interfaces() {
    let metadata = fixture_metadata(
        r#"
            #[winrt]
            mod Test {
                interface IBase {
                    fn BaseMethod(&self) -> i32;
                    fn Unused(&self) -> i32;
                }
                interface IDerived: IBase {
                    fn DerivedMethod(&self) -> i32;
                }
            }
        "#,
    );
    let mut filter = Filter::new();
    filter.include_method("Test", "IDerived", "BaseMethod");
    let output = metadata
        .generator(Request::filtered(filter))
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();

    assert!(output.contains("impl IDerived"));
    assert!(output.contains("pub fn BaseMethod"));
    assert!(!output.contains("pub fn Unused"));
    assert!(!output.contains("pub fn DerivedMethod"));
    assert!(output.contains("pub BaseMethod : unsafe extern \"system\" fn"));
}

#[test]
fn winrt_class_corpus_lowers_and_renders() {
    let generator = generator();
    let values = generator.lower_values();
    let mut count = 0;
    let mut no_default = 0;
    let mut default_activatable = 0;
    let mut factory_activatable = 0;
    let mut static_factories = 0;
    let mut composable_factories = 0;
    let mut derived = 0;
    let mut agile = 0;
    let mut async_defaults = 0;
    let mut unsupported = Vec::new();
    for entry in generator
        .winrt
        .iter()
        .filter(|entry| entry.kind == WinrtKind::Class)
    {
        let definition = generator.shared.database.definition(entry.entity).unwrap();
        let namespace = definition.namespace().unwrap();
        let name = definition.name().unwrap();
        let default = generator
            .shared
            .interface_relationships
            .get(&entry.entity)
            .and_then(|relationships| {
                relationships
                    .iter()
                    .filter_map(|relationship| relationship.resolve().ok())
                    .find(|item| item.default)
            });
        no_default += usize::from(default.is_none());
        if let Some(default) = default {
            let name = generator
                .shared
                .database
                .definition(default.entity)
                .unwrap()
                .name()
                .unwrap();
            async_defaults += usize::from(matches!(
                trim_generic_arity(name),
                "IAsyncAction"
                    | "IAsyncActionWithProgress"
                    | "IAsyncOperation"
                    | "IAsyncOperationWithProgress"
            ));
        }
        if let Some(base) = definition.base_type().unwrap()
            && let Some((base_namespace, base_name)) = generator
                .shared
                .database
                .type_name(base.file, base.ty)
                .unwrap()
        {
            derived += usize::from(!(base_namespace == "System" && base_name == "Object"));
        }
        for attribute in definition.attributes().unwrap() {
            match attribute.name().unwrap() {
                Some("ActivatableAttribute") => {
                    let has_factory = attribute.arguments(&()).unwrap().iter().any(|argument| {
                        matches!(
                            argument,
                            AttributeArgument::Fixed {
                                value: AttributeValue::TypeName(_),
                                ..
                            }
                        )
                    });
                    factory_activatable += usize::from(has_factory);
                    default_activatable += usize::from(!has_factory);
                }
                Some("StaticAttribute") => static_factories += 1,
                Some("ComposableAttribute") => composable_factories += 1,
                Some("MarshalingBehaviorAttribute") => {
                    agile +=
                        usize::from(attribute.arguments(&()).unwrap().iter().any(|argument| {
                            matches!(
                                argument,
                                AttributeArgument::Fixed {
                                    value: AttributeValue::Enum { value, .. },
                                    ..
                                } if matches!(value.as_ref(), AttributeValue::I32(2))
                            )
                        }));
                }
                _ => {}
            }
        }
        let result = winrt_class::Class::lower(
            &generator.shared.database,
            definition,
            &generator.shared.interface_relationships,
            &format!("{namespace}.{name}"),
        )
        .and_then(|model| {
            for projection in [Projection::Default, Projection::Minimal] {
                model.write(
                    values,
                    namespace,
                    Layout::Modules,
                    projection,
                    &MemberSelection::All,
                    None,
                    &BTreeMap::new(),
                )?;
            }
            Ok(())
        });
        if let Err(error) = result {
            unsupported.push((format!("{namespace}.{name}"), error.to_string()));
        }
        count += 1;
    }
    assert_eq!(count, 4_516);
    assert_eq!(
        (
            no_default,
            default_activatable,
            factory_activatable,
            static_factories,
            composable_factories,
            derived,
            agile,
            async_defaults,
        ),
        (257, 719, 415, 1564, 374, 718, 4116, 10)
    );
    assert!(unsupported.is_empty(), "{unsupported:#?}");
}

#[test]
fn filtered_winrt_classes_close_hierarchy_and_factory_dependencies() {
    for (source, filter, required) in [
        (
            include_str!("../../../tests/libs/bindgen/input/class_hierarchy.rdl"),
            "Leaf",
            &["Base", "Middle", "Leaf", "IBase", "IMiddle", "ILeaf"][..],
        ),
        (
            include_str!("../../../tests/libs/bindgen/input/class_static.rdl"),
            "Class",
            &["Class", "IClass", "IClassStatics"][..],
        ),
    ] {
        let metadata = fixture_metadata(source);
        let generator = metadata
            .generator(Request::filtered(Filter::names([filter])))
            .unwrap();
        let output = generator.render(Layout::Flat).unwrap().to_string();
        for name in required {
            assert!(output.contains(name), "{name}");
        }
    }
}

#[test]
fn winrt_class_special_policies_render() {
    let metadata = Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();

    let output = metadata
        .generator(Request::filtered(Filter::names([
            "DeleteSmsMessageOperation",
        ])))
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();
    assert!(
        output.contains("pub type DeleteSmsMessageOperation = windows_future :: IAsyncAction"),
        "{output}"
    );

    let output = metadata
        .generator(Request::filtered(Filter::names(["HtmlUtilities"])))
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();
    assert!(output.contains("pub struct HtmlUtilities ;"), "{output}");
    assert!(output.contains("pub fn ConvertToText"), "{output}");

    let output = metadata
        .generator(Request::filtered(Filter::names(["SmsAppMessage"])))
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();
    assert!(
        output.contains("unsafe impl Send for SmsAppMessage"),
        "{output}"
    );
    assert!(
        output.contains("unsafe impl Sync for SmsAppMessage"),
        "{output}"
    );

    let output = fixture(
        r#"
            #[winrt]
            mod Test {
                class FakeAsync {
                    IAsyncAction,
                }
                interface IAsyncAction {}
            }
        "#,
    )
    .render(Layout::Flat)
    .unwrap()
    .to_string();
    assert!(output.contains("pub struct FakeAsync"), "{output}");
    assert!(!output.contains("pub type FakeAsync"), "{output}");
}

#[test]
fn minimal_native_types_route_canonical_external_types() {
    for name in ["Matrix3x2", "Matrix4x4", "Vector2", "Vector3", "Vector4"] {
        assert_eq!(
            external::minimal_crate("Windows.Foundation.Numerics", name),
            Some("windows_numerics")
        );
    }

    let metadata = Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();

    let output = metadata
        .generator(
            Request::filtered(Filter::names(["D2D1_BRUSH_PROPERTIES"]))
                .projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();

    assert!(
        output.contains("pub transform : windows_numerics :: Matrix3x2"),
        "{output}"
    );
}

#[test]
fn rich_native_struct_fields_own_interfaces() {
    let metadata = Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();
    let generator = metadata
        .generator(
            Request::filtered(Filter::names(["D2D1_BITMAP_PROPERTIES1"]))
                .projection(Projection::Minimal),
        )
        .unwrap();

    let output = generator
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();
    assert!(
        output.contains(
            "pub colorContext : core :: mem :: ManuallyDrop < Option < ID2D1ColorContext >>"
        ),
        "{output}"
    );
    assert!(
        output.contains(
            "# [derive (Clone , Debug , Default , PartialEq)] pub struct D2D1_BITMAP_PROPERTIES1"
        ),
        "{output}"
    );

    let output = generator
        .render_projection(Layout::Flat, Projection::Sys)
        .unwrap()
        .to_string();
    assert!(
        output.contains("pub colorContext : * mut core :: ffi :: c_void"),
        "{output}"
    );
}

#[test]
fn rich_native_interfaces_project_complete_inheritance() {
    let metadata = Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();
    let output = metadata
        .generator(
            Request::filtered(Filter::names(["ID2D1Bitmap1"])).projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();

    assert!(
        output.contains("impl core :: ops :: Deref for ID2D1Bitmap1 { type Target = ID2D1Bitmap ;"),
        "{output}"
    );
    assert!(
        output.contains(
            "interface_hierarchy ! (ID2D1Bitmap1 , windows_core :: IUnknown , ID2D1Resource , ID2D1Image , ID2D1Bitmap)"
        ),
        "{output}"
    );
    assert!(
        output.contains("pub struct ID2D1Bitmap1_Vtbl { pub base__ : ID2D1Bitmap_Vtbl"),
        "{output}"
    );
}

#[test]
fn void_com_methods_project_interface_outputs() {
    let metadata = Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();
    let output = metadata
        .generator(
            Request::filtered(Filter::names(["ID2D1DeviceContext"]))
                .projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();

    assert!(
        output.contains("unsafe fn GetTarget (& self ,) -> windows_core :: Result < ID2D1Image >"),
        "{output}"
    );
    assert!(
        output.contains(
            "(windows_core :: Interface :: vtable (self) . GetTarget) (windows_core :: Interface :: as_raw (self) , & mut result__) ; windows_core :: Type :: from_abi (result__)"
        ),
        "{output}"
    );
}

#[test]
fn native_com_methods_project_unique_input_buffers_as_slices() {
    let metadata = Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();
    let output = metadata
        .generator(
            Request::filtered(Filter::names([
                "ID2D1Factory1",
                "ID2D1RenderTarget",
                "IDWriteFactory",
            ]))
            .projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();

    for expected in [
        "unsafe fn CreateStrokeStyle (& self , strokestyleproperties : * const D2D1_STROKE_STYLE_PROPERTIES1 , dashes : Option < & [f32] > ,)",
        "unsafe fn CreateGradientStopCollection (& self , gradientstops : & [D2D1_GRADIENT_STOP] ,",
        "unsafe fn DrawText < P2 , P4 > (& self , string : & [u16] ,",
        "unsafe fn CreateTextLayout < P2 > (& self , string : & [u16] ,",
        "dashes . map_or (core :: ptr :: null () , | slice | slice . as_ptr ()) , dashes . map_or (0 , | slice | slice . len () . try_into () . unwrap ())",
        "gradientstops . as_ptr () , gradientstops . len () . try_into () . unwrap ()",
        "string . as_ptr () , string . len () . try_into () . unwrap ()",
    ] {
        assert!(output.contains(expected), "{expected}\n{output}");
    }
}

#[test]
fn native_com_methods_project_bool_inputs() {
    let metadata = Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();
    let output = metadata
        .generator(
            Request::filtered(Filter::names(["ID2D1Effect", "IDWriteTextLayout"]))
                .projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();

    for expected in [
        "unsafe fn SetInput < P1 > (& self , index : u32 , input : P1 , invalidate : bool ,)",
        "input . param () . abi () , invalidate . into ()",
        "unsafe fn HitTestTextPosition (& self , textposition : u32 , istrailinghit : bool ,",
        "textposition , istrailinghit . into ()",
        "pub SetInput : unsafe extern \"system\" fn (* mut core :: ffi :: c_void , u32 , * mut core :: ffi :: c_void , windows_core :: BOOL)",
    ] {
        assert!(output.contains(expected), "{expected}\n{output}");
    }
}

#[test]
fn large_native_output_structs_remain_explicit_parameters() {
    let metadata = Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();
    let output = metadata
        .generator(
            Request::filtered(Filter::names(["IDWriteTextLayout"])).projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();

    assert!(
        output.contains(
            "unsafe fn GetMetrics (& self , textmetrics : * mut DWRITE_TEXT_METRICS ,) -> windows_core :: HRESULT"
        ),
        "{output}"
    );
    assert!(
        !output.contains("Result < DWRITE_TEXT_METRICS >"),
        "{output}"
    );
}

#[test]
fn retval_sizing_uses_maximum_pointer_width() {
    let database = Database::new([]).unwrap();
    let ty = native::Type::Array {
        element: Box::new(native::Type::Pointer {
            mutable: true,
            element: Box::new(native::Type::Void),
        }),
        len: 3,
    };
    assert!(ty.exceeds_retval_limit(&database).unwrap());
}

#[test]
fn native_com_methods_project_pcwstr_inputs() {
    let metadata = Metadata::new(
        Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();
    let output = metadata
        .generator(
            Request::filtered(Filter::names(["IDWriteFactory", "IWICImagingFactory"]))
                .projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap()
        .to_string();

    for expected in [
        "unsafe fn CreateTextFormat < P0 , P1 , P6 > (& self , fontfamilyname : P0 ,",
        "localename : P6 ,) -> windows_core :: Result < IDWriteTextFormat > where P0 : windows_core :: Param < windows_core :: PCWSTR > ,",
        "P6 : windows_core :: Param < windows_core :: PCWSTR > ,",
        "fontfamilyname . param () . abi ()",
        "localename . param () . abi ()",
        "unsafe fn CreateDecoderFromFilename < P0 > (& self , wzfilename : P0 ,",
        "where P0 : windows_core :: Param < windows_core :: PCWSTR > ,",
        "wzfilename . param () . abi ()",
    ] {
        assert!(output.contains(expected), "{expected}\n{output}");
    }
}

#[test]
fn winrt_class_preserves_closed_generic_interfaces() {
    let output = fixture(
        r#"
            #[winrt]
            mod Test {
                class Class {
                    IClass,
                    IValue<i32>,
                    IValue<u32>,
                }
                interface IClass {}
                interface IValue<T> {
                    fn Value(&self) -> T;
                }
            }
        "#,
    )
    .render(Layout::Flat)
    .unwrap()
    .to_string();

    assert!(output.contains("required_hierarchy ! (Class , IValue < i32 > , IValue < u32 >)"));
    assert!(
        output.contains("pub fn Value (& self ,) -> windows_core :: Result < i32 >"),
        "{output}"
    );
    assert!(
        output.contains("pub fn Value2 (& self ,) -> windows_core :: Result < u32 >"),
        "{output}"
    );
}

#[test]
fn winrt_delegate_corpus_lowers_and_renders() {
    let generator = generator();
    let mut count = 0;
    let mut input_vectors = 0;
    let mut output_vectors = 0;
    let mut return_vectors = 0;
    let mut architecture_delegates = 0;
    let mut noexcept_delegates = 0;
    for entry in generator
        .winrt
        .iter()
        .filter(|entry| entry.kind == WinrtKind::Delegate)
    {
        let definition = generator.shared.database.definition(entry.entity).unwrap();
        let namespace = definition.namespace().unwrap();
        let name = definition.name().unwrap();
        let invoke = definition.methods().unwrap().next().unwrap();
        architecture_delegates += usize::from(definition.architectures().unwrap() != 0);
        noexcept_delegates += usize::from(
            invoke
                .find_attribute("NoExceptionAttribute")
                .unwrap()
                .is_some(),
        );
        let signature = invoke.signature().unwrap();
        let parameters = invoke.parameters_by_sequence().unwrap();
        for (ty, parameter) in signature.parameters.iter().zip(parameters.parameters()) {
            if matches!(ty.kind, TypeKind::Vector(_)) {
                if parameter.is_none_or(|parameter| parameter.flags().unwrap() & 0x2 == 0) {
                    input_vectors += 1;
                } else {
                    output_vectors += 1;
                }
            }
        }
        if matches!(signature.return_type.kind, TypeKind::Vector(_)) {
            return_vectors += 1;
        }
        let model = winrt_delegate::Delegate::lower(
            &generator.shared.database,
            definition,
            &format!("{namespace}.{name}"),
        )
        .unwrap();
        for projection in [Projection::Default, Projection::Minimal] {
            model
                .write(
                    generator.lower_values(),
                    namespace,
                    Layout::Modules,
                    projection,
                    false,
                )
                .unwrap();
        }
        count += 1;
    }
    assert_eq!(
        (
            count,
            input_vectors,
            output_vectors,
            return_vectors,
            architecture_delegates,
            noexcept_delegates,
        ),
        (137, 1, 0, 0, 0, 0)
    );
}

#[test]
fn filtered_winrt_delegates_close_value_dependencies() {
    let metadata = fixture_metadata(
        r#"
                #[winrt]
                mod Test {
                    #[repr(i32)]
                    enum Kind {
                        First = 0,
                    }

                    delegate fn Handler(value: Kind);
                    delegate fn Outer(handler: Handler);
                }
            "#,
    );
    let generator = metadata
        .generator(Request::filtered(Filter::names(["Outer"])))
        .unwrap();
    let output = generator.render(Layout::Flat).unwrap().to_string();
    assert!(output.contains("pub struct Kind"));
    assert!(output.contains("define_interface ! (Handler"));
    assert!(output.contains("define_interface ! (Outer"));
}

#[test]
fn filtered_generic_delegate_uses_projected_name() {
    let metadata = fixture_metadata(include_str!(
        "../../../tests/libs/bindgen/input/delegate_generic.rdl"
    ));
    let generator = metadata
        .generator(Request::filtered(Filter::names(["Handler"])))
        .unwrap();
    assert!(
        generator
            .render(Layout::Flat)
            .unwrap()
            .to_string()
            .contains("Handler_Vtbl")
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

    let output = generator
        .render_projection(Layout::Modules, Projection::Sys)
        .unwrap()
        .to_string();
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

    assert_eq!(generator.values().count(), 1);
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

    assert_eq!(generator.values().count(), 3);
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

    assert_eq!(generator.values().count(), 2);
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
fn native_dependency_closure_keeps_same_namespace_aliases() {
    let metadata = Metadata::from_images([
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let output = metadata
        .generator(Request::filtered(Filter::names(["CONTEXT", "STRRET"])).sys())
        .unwrap()
        .render(Layout::Flat)
        .unwrap()
        .to_string();

    assert!(output.contains("pub type PWSTR"));
    assert!(output.contains("pub type XMM_SAVE_AREA32"));
    assert!(output.contains("pub struct XSAVE_FORMAT"));
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
    let output = generator
        .render_projection(Layout::Modules, Projection::Sys)
        .unwrap()
        .to_string();
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
fn rich_native_interface_without_com_identity_is_rejected() {
    let metadata = fixture_metadata(
        r#"
            #[win32]
            mod Test {
                interface Interface {
                    fn Method(&self);
                }
            }
        "#,
    );
    let error = metadata
        .generator(Request::filtered(Filter::names(["Interface"])))
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap_err();

    assert!(matches!(
        error,
        Error::UnsupportedType { name, shape }
            if name == "Test.Interface"
                && shape == "rich native interface without COM identity"
    ));
}

#[test]
fn native_interface_member_filter_keeps_placeholders_and_shell_dependencies() {
    let metadata = Metadata::from_images([
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let mut filter = Filter::new();
    include_tool_filter(
        &metadata,
        &mut filter,
        "IUIAnimationManager2::{CreateAnimationVariable}",
    );
    let output = normalize_existing_output(
        metadata
            .generator(Request::filtered(filter).projection(Projection::Minimal))
            .unwrap()
            .render_projection(Layout::Flat, Projection::Minimal)
            .unwrap(),
    );

    assert!(
        output.contains(
            "CreateAnimationVariable (& self , f64) -> windows_core :: Result < \
         IUIAnimationVariable2 >"
        ),
        "{output}"
    );
    assert!(output.contains("CreateAnimationVectorVariable : usize"));
    assert!(output.contains("define_interface ! (IUIAnimationVariable2"));
    assert!(!output.contains("impl IUIAnimationVariable2 {"));
    assert!(!output.contains("IUIAnimationManager2_Impl"));
}

#[test]
fn native_com_methods_project_scalar_retvals_and_optional_pointers() {
    let metadata = Metadata::from_images([
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let mut filter = Filter::new();
    include_tool_filter(
        &metadata,
        &mut filter,
        "IUIAnimationStoryboard2::{AddKeyframeAfterTransition, Schedule}",
    );
    let output = normalize_existing_output(
        metadata
            .generator(Request::filtered(filter).projection(Projection::Minimal))
            .unwrap()
            .render_projection(Layout::Flat, Projection::Minimal)
            .unwrap(),
    );

    assert!(
        output.contains(
            "AddKeyframeAfterTransition < P0 > (& self , P0) -> windows_core :: Result < \
         UI_ANIMATION_KEYFRAME >"
        ),
        "{output}"
    );
    assert!(
        output.contains("Schedule (& self , f64 , Option <* mut UI_ANIMATION_SCHEDULING_RESULT")
    );
}

#[test]
fn complete_native_com_query_emits_consumer_and_producer_projection() {
    let metadata = Metadata::from_images([
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let mut filter = Filter::new();
    include_tool_filter(&metadata, &mut filter, "IAgileReference.Resolve");
    let output = normalize_existing_output(
        metadata
            .generator(Request::filtered(filter).projection(Projection::Minimal))
            .unwrap()
            .render_projection(Layout::Flat, Projection::Minimal)
            .unwrap(),
    );

    assert!(output.contains(
        "unsafe fn Resolve < T > (& self) -> windows_core :: Result < T > where T : \
         windows_core :: Interface"
    ));
    assert!(
        output
            .contains("pub trait IAgileReference_Impl : windows_core :: IUnknownImpl { fn Resolve")
    );
    assert!(output.contains("pub const fn new < Identity : IAgileReference_Impl"));
}

#[test]
fn explicit_implementations_control_native_producers_and_closure() {
    let metadata = Metadata::from_images([
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let mut filter = Filter::new();
    include_tool_filter(
        &metadata,
        &mut filter,
        "IUIAnimationManager2::{CreateAnimationVariable}",
    );

    let output = normalize_existing_output(
        metadata
            .generator(
                Request::filtered(filter.clone())
                    .implementations(Filter::new())
                    .projection(Projection::Minimal),
            )
            .unwrap()
            .render_projection(Layout::Flat, Projection::Minimal)
            .unwrap(),
    );
    assert!(output.contains("CreateAnimationVariable"));
    assert!(output.contains("CreateAnimationVectorVariable : usize"));
    assert!(output.contains("impl IUIAnimationManager2 {"));
    assert!(!output.contains("IUIAnimationManager2_Impl"));
    assert!(!output.contains("RuntimeName for IUIAnimationManager2"));

    let mut implementations = Filter::new();
    implementations.include_name("IUIAnimationManager2");
    let output = normalize_existing_output(
        metadata
            .generator(
                Request::filtered(filter)
                    .implementations(implementations)
                    .projection(Projection::Minimal),
            )
            .unwrap()
            .render_projection(Layout::Flat, Projection::Minimal)
            .unwrap(),
    );
    assert!(output.contains("pub trait IUIAnimationManager2_Impl"));
    assert!(output.contains("pub CreateAnimationVectorVariable : unsafe extern"));
    assert!(!output.contains("impl IUIAnimationManager2 {"));
    assert!(!output.contains("unsafe fn CreateAnimationVectorVariable"));
    assert!(output.contains("RuntimeName for IUIAnimationManager2"));
}

#[test]
fn native_producer_interface_inputs_are_borrowed() {
    let metadata = Metadata::from_images([
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let filter = Filter::names(["ID2D1CommandSink"]);
    let implementations = Filter::names(["ID2D1CommandSink"]);
    let output = normalize_existing_output(
        metadata
            .generator(
                Request::filtered(filter)
                    .implementations(implementations)
                    .projection(Projection::Minimal),
            )
            .unwrap()
            .render_projection(Layout::Flat, Projection::Minimal)
            .unwrap(),
    );

    assert!(
        output.contains("fn DrawBitmap (& self , windows_core :: Ref < ID2D1Bitmap >,"),
        "{output}"
    );
    assert!(output.contains(
        "ID2D1CommandSink_Impl :: DrawBitmap (this , core :: mem :: transmute_copy (& bitmap)"
    ));
    assert!(output.contains(
        "pub DrawBitmap : unsafe extern \"system\" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void"
    ));
}

#[test]
fn native_com_query_preserves_ordinary_parameters() {
    let metadata = Metadata::from_images([
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let mut filter = Filter::new();
    include_tool_filter(&metadata, &mut filter, "IDXGISwapChain.GetBuffer");
    let output = normalize_existing_output(
        metadata
            .generator(Request::filtered(filter).projection(Projection::Minimal))
            .unwrap()
            .render_projection(Layout::Flat, Projection::Minimal)
            .unwrap(),
    );

    assert!(output.contains(
        "unsafe fn GetBuffer < T > (& self , u32) -> windows_core :: Result < T > where T : \
         windows_core :: Interface"
    ));
    assert!(output.contains(
        "pub GetBuffer : unsafe extern \"system\" fn (* mut core :: ffi :: c_void , u32 , \
         * const windows_core :: GUID , * mut * mut core :: ffi :: c_void)"
    ));
}

#[test]
fn native_com_projects_void_scalar_and_indirect_struct_returns() {
    let metadata = Metadata::from_images([
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let mut filter = Filter::new();
    include_tool_filter(&metadata, &mut filter, "ID2D1RenderTarget.BeginDraw");
    include_tool_filter(&metadata, &mut filter, "IDWriteTextLayout.GetMaxWidth");
    include_tool_filter(&metadata, &mut filter, "ID2D1Bitmap.GetSize");
    let output = normalize_existing_output(
        metadata
            .generator(Request::filtered(filter).projection(Projection::Minimal))
            .unwrap()
            .render_projection(Layout::Flat, Projection::Minimal)
            .unwrap(),
    );

    assert!(output.contains("unsafe fn BeginDraw (& self) { unsafe"));
    assert!(output.contains("unsafe fn GetMaxWidth (& self) -> f32"));
    assert!(output.contains("unsafe fn GetSize (& self) -> D2D_SIZE_F"));
    assert!(output.contains(
        "pub GetSize : unsafe extern \"system\" fn (* mut core :: ffi :: c_void , \
         * mut D2D_SIZE_F)"
    ));
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
        generator
            .render_projection(Layout::Modules, Projection::Sys)
            .unwrap()
            .to_string(),
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
            .render_projection(Layout::Modules, Projection::Sys)
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
            .render_projection(Layout::Modules, Projection::Sys)
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
fn tool_webview_reactor_request_matches_committed_output() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut paths = std::fs::read_dir(root.join("crates/tools/reactor/winmd"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "winmd")
        })
        .collect::<Vec<_>>();
    paths.sort();
    let bytes = paths
        .iter()
        .map(|path| std::fs::read(path).unwrap())
        .collect::<Vec<_>>();
    let images = paths
        .iter()
        .zip(bytes)
        .map(|(path, bytes)| {
            Image::new(bytes).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
        })
        .chain([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ]);
    let metadata = Metadata::from_images(images).unwrap();
    let request = parse_tool_request(
        &metadata,
        &std::fs::read_to_string(root.join("crates/tools/webview/src/reactor.txt")).unwrap(),
    );
    assert!(request.minimal);
    assert!(!request.dead_code);
    assert_eq!(
        request.implementations,
        [
            "Windows.Foundation.TypedEventHandler",
            "Microsoft.UI.Xaml.RoutedEventHandler",
        ]
    );
    let mut implementations = Filter::new();
    for name in &request.implementations {
        let (namespace, name) = name.rsplit_once('.').unwrap();
        implementations.include_item(namespace, name);
    }
    let actual = metadata
        .generator(
            Request::filtered(request.filter)
                .implementations(implementations)
                .projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap();
    let expected: TokenStream = std::fs::read_to_string(root.join(request.output))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        normalize_minimal_delegate_constructors(actual),
        normalize_minimal_delegate_constructors(expected)
    );
}

#[test]
#[ignore = "requires target/webview/WebView2.winmd generated by tool_webview"]
fn tool_webview_native_request_matches_committed_output() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let metadata = Metadata::from_images([
        Image::read(root.join("target/webview/WebView2.winmd")).unwrap(),
        Image::new(windows_default::WINRT).unwrap(),
        Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let request = parse_tool_request(
        &metadata,
        &std::fs::read_to_string(root.join("crates/tools/webview/src/webview.txt")).unwrap(),
    );
    assert!(request.minimal);
    assert!(request.dead_code);
    assert_eq!(request.implementations.len(), 28);
    let mut implementations = Filter::new();
    for name in &request.implementations {
        let (namespace, name) = name.rsplit_once('.').unwrap();
        implementations.include_item(namespace, name);
    }
    let actual = metadata
        .generator(
            Request::filtered(request.filter)
                .implementations(implementations)
                .projection(Projection::Minimal),
        )
        .unwrap()
        .render_projection(Layout::Flat, Projection::Minimal)
        .unwrap();
    let expected: TokenStream = std::fs::read_to_string(root.join(request.output))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        normalize_existing_output(actual),
        normalize_existing_output(expected)
    );
}
#[test]
fn architecture_source_gates() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden = [
        (
            "feature_dependencies(",
            "formatted output dependency recovery",
        ),
        ("MinimalPublic", "combined projection and visibility modes"),
        ("WebView2", "consumer-specific WebView policy"),
        ("Direct2D", "consumer-specific Direct2D policy"),
        ("VSS", "consumer-specific VSS policy"),
        ("Reactor", "consumer-specific Reactor policy"),
        (
            "context.layout == Layout::Package",
            "package policy outside a context method",
        ),
        (
            "context.projection.",
            "projection policy outside a context method",
        ),
    ];

    for entry in std::fs::read_dir(&source).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|extension| extension != "rs")
            || path.file_name().is_some_and(|name| name == "tests.rs")
        {
            continue;
        }
        let contents = std::fs::read_to_string(&path).unwrap();
        for (pattern, reason) in forbidden {
            assert!(
                !contents.contains(pattern),
                "{} contains {reason}: {pattern}",
                path.display()
            );
        }
        if path.file_name().is_none_or(|name| name != "canonical.rs") {
            for name in [
                "\"GUID\"",
                "\"Guid\"",
                "\"HRESULT\"",
                "\"HResult\"",
                "\"EventRegistrationToken\"",
            ] {
                assert!(
                    !contents.contains(name),
                    "{} contains canonical ABI identity {name}",
                    path.display()
                );
            }
        }
        if path.file_name().is_none_or(|name| {
            !matches!(
                name.to_str(),
                Some("build.rs" | "lib.rs" | "output.rs" | "tests.rs")
            )
        }) {
            assert!(
                !contents.contains("Layout::Package"),
                "{} contains package branching outside layout policy",
                path.display()
            );
        }
    }
}
