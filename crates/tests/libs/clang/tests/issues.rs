//! One header -> RDL fixture per reported issue in `input\issues`. Each golden RDL is reviewed
//! against the issue's contract and must compile to WinMD. Set `UPDATE_CLANG_RDL` to rewrite.

use std::path::{Path, PathBuf};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn directory() -> PathBuf {
    test_clang::input_dir().join("issues")
}

fn input(name: &str) -> Input {
    let name = format!("{name}.h");
    Input {
        source: std::fs::read_to_string(directory().join(&name)).unwrap(),
        arguments: vec!["-I".into(), directory().display().to_string()],
        macro_headers: vec![name.clone()],
        name,
        ..Default::default()
    }
}

/// Captures, projects and compares with `<golden>.rdl`, returning the omissions.
fn check(golden: &str, inputs: &[Input]) -> Vec<String> {
    helpers::ensure_libclang();
    let captured = capture_partial(inputs, &TARGETS).unwrap();
    let imports = captured
        .snapshots
        .iter()
        .flat_map(|snapshot| &snapshot.values)
        .filter(|value| {
            value.observations.iter().any(|observation| {
                matches!(
                    observation.kind,
                    ValueKind::Function {
                        language: LanguageLinkage::C,
                        ..
                    }
                )
            })
        })
        .map(|value| {
            (
                value.name.clone(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .collect();
    let partial = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    let actual: String = partial
        .output
        .partitions
        .values()
        .map(|partition| partition.rdl.as_str())
        .collect();
    let path = directory().join(format!("{golden}.rdl"));
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&path, &actual).unwrap();
    }
    assert_eq!(
        actual,
        std::fs::read_to_string(&path)
            .unwrap()
            .replace("\r\n", "\n"),
        "{golden}"
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang-issue-{golden}.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(
            partial
                .output
                .partitions
                .values()
                .map(|partition| &partition.rdl),
        )
        .output(&winmd)
        .write()
        .unwrap();
    std::fs::remove_file(winmd).unwrap();
    partial.omissions
}

fn single(name: &str) -> Vec<String> {
    check(name, &[input(name)])
}

fn assert_omitted(omissions: &[String], name: &str) {
    assert!(
        omissions.iter().any(|omission| omission.contains(name)),
        "{name} not omitted: {omissions:?}"
    );
}

// #4199: implicit, explicit and aliased anonymous enum values.
#[test]
fn anonymous_enum_sequence() {
    assert_eq!(single("anonymous_enum_sequence"), Vec::<String>::new());
}

// #4203: `wchar_t` and `__wchar_t` are the same UTF-16 code unit.
#[test]
fn wide_parameters() {
    assert_eq!(single("wide_parameters"), Vec::<String>::new());
}

// #4205: functions declared inside an `extern "C"` block.
#[test]
fn extern_c_block() {
    assert_eq!(single("extern_c_block"), Vec::<String>::new());
}

// #4208: parenthesized string literal macros.
#[test]
fn paren_string_macros() {
    assert_eq!(single("paren_string_macros"), Vec::<String>::new());
}

// #4221: elaborated `enum` parameter types.
#[test]
fn elaborated_enum_parameter() {
    assert_eq!(single("elaborated_enum_parameter"), Vec::<String>::new());
}

// #4226: `MIDL_INTERFACE`-style macros keep each interface's own UUID.
#[test]
fn uuid_macro_interfaces() {
    assert_eq!(single("uuid_macro_interfaces"), Vec::<String>::new());
}

// #4241: types in `extern "C"` blocks publish; C++-linkage functions do not.
#[test]
fn linkage_blocks() {
    let omissions = single("linkage_blocks");
    assert_eq!(omissions.len(), 1, "{omissions:?}");
    assert_omitted(&omissions, "CppOnly");
}

// #4249: an interface that adds no methods.
#[test]
fn interface_empty() {
    assert_eq!(single("interface_empty"), Vec::<String>::new());
}

// #4259: a function-like linkage macro is not a constant.
#[test]
fn extern_c_macro() {
    let omissions = single("extern_c_macro");
    assert_eq!(omissions.len(), 1, "{omissions:?}");
    assert_omitted(&omissions, "EXTERN_C");
}

// #4278: root paths select API headers; shared dependencies publish only when used.
#[test]
fn filter_roots() {
    let mut input = input("filter_roots");
    input.macro_headers.clear();
    input.root_paths = Some(vec![
        directory().join("filter_api1.h"),
        directory().join("filter_api2.h"),
    ]);
    assert_eq!(check("filter_roots", &[input]), Vec::<String>::new());
}

// #4463: unsigned integer suffixes select unsigned constant types.
#[test]
fn unsigned_macros() {
    assert_eq!(single("unsigned_macros"), Vec::<String>::new());
}

// #4934: a complemented shift mask evaluates to its value.
#[test]
fn enum_mask() {
    assert_eq!(single("enum_mask"), Vec::<String>::new());
}

// #4936, #4937, #4938: fields keep their named ABI typedefs.
#[test]
fn named_abi_typedefs() {
    assert_eq!(single("named_abi_typedefs"), Vec::<String>::new());
}

// #4940: a forward use and a definition in different inputs merge into one interface,
// independently of input order.
#[test]
fn forward_interface() {
    let first = check(
        "forward_interface",
        &[
            input("forward_interface_use"),
            input("forward_interface_definition"),
        ],
    );
    let second = check(
        "forward_interface",
        &[
            input("forward_interface_definition"),
            input("forward_interface_use"),
        ],
    );
    assert_eq!(first, Vec::<String>::new());
    assert_eq!(second, first);
}

// #4941: a type and a macro constant can share a name.
#[test]
fn type_value_name_collision() {
    assert_eq!(single("type_value_name_collision"), Vec::<String>::new());
}

// #4942: pointer sentinels over opaque interfaces either publish resolvable types or are omitted.
#[test]
fn opaque_pointer_sentinels() {
    single("opaque_pointer_sentinels");
}

// #4993: string typedef chains and unaliased `const char*` keep resolvable declarations.
#[test]
fn canonical_string_typedef() {
    assert_eq!(single("canonical_string_typedef"), Vec::<String>::new());
}

// #4994: a const pointer to a mutable pointer keeps its typedef so the chain stays representable.
#[test]
fn mixed_pointer_chain() {
    assert_eq!(single("mixed_pointer_chain"), Vec::<String>::new());
}

// #4998: an annotated in/out interface object is a borrowed interface, not an output slot.
#[test]
fn interface_value_direction() {
    assert_eq!(single("interface_value_direction"), Vec::<String>::new());
}

// #5069: equivalent typedefs from different provider headers do not make a shared interface
// ambiguous.
#[test]
fn typedef_providers() {
    let inputs = [
        input("typedef_providers_first"),
        input("typedef_providers_second"),
    ];
    assert_eq!(check("typedef_providers", &inputs), Vec::<String>::new());
}

// #5069: providers that disagree on the native type are a conflict, not a choice.
#[test]
fn typedef_providers_conflict() {
    helpers::ensure_libclang();
    let inputs = [
        input("typedef_providers_first"),
        input("typedef_providers_conflict"),
    ];
    let error = capture_partial(&inputs, &TARGETS)
        .and_then(|captured| projection::project_partial(captured, "Fixture"))
        .map(|_| ())
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("native field types differ") && error.contains("IContract"),
        "{error}"
    );
}

// #4932: concrete C++ data classes, including a forward-declared one, are records.
#[test]
fn data_classes() {
    assert_eq!(single("data_classes"), Vec::<String>::new());
}

// #4584: IID variables built through nested macros publish their GUID values.
#[test]
fn macro_iid_variables() {
    assert_eq!(single("macro_iid_variables"), Vec::<String>::new());
}

// #5190: `EXTERN_GUID` and raw initializer declarations publish their values. Copying a
// non-constexpr GUID is C++ dynamic initialization, so it has no constant value to publish.
#[test]
fn guid_declarations() {
    let omissions = single("guid_declarations");
    assert_eq!(omissions.len(), 1, "{omissions:?}");
    assert_omitted(&omissions, "CLSID_DxcUtils");
    assert_omitted(&omissions, "not a constant expression");
}

// #4995: a macro renaming a function declares a distinct native function. Both declarations
// publish; the macro alias collides with the native `PublicValue` and is omitted.
#[test]
fn function_macro_alias() {
    let omissions = single("function_macro_alias");
    assert_eq!(omissions.len(), 1, "{omissions:?}");
    assert_omitted(&omissions, "function alias `PublicValue` collides");
}

// #4313: MIDL `[optional]` comments mark the parameter optional.
#[test]
fn midl_optional() {
    assert_eq!(single("midl_optional"), Vec::<String>::new());
}

// #4229, #4246: MIDL `[propget]`/`[propput]` method comments add `#[special]`.
#[test]
fn midl_property_comments() {
    assert_eq!(single("midl_property_comments"), Vec::<String>::new());
}

// #4301: an `E operator|(E, E)` declaration, as `DEFINE_ENUM_FLAG_OPERATORS` declares, adds
// `#[flags]`; an `operator|` returning another type does not. The operators are not published.
#[test]
fn enum_flag_operators() {
    let omissions = single("enum_flag_operators");
    assert!(
        omissions
            .iter()
            .all(|omission| omission.contains("operator")),
        "{omissions:?}"
    );
}

// #4997: a header-local GUID record whose primary name is `_GUID` is not the canonical GUID, so
// its constants are omitted rather than emitted as unsupported literals.
#[test]
fn local_guid() {
    let omissions = single("local_guid");
    assert_eq!(omissions.len(), 1, "{omissions:?}");
    assert_omitted(&omissions, "requires canonical GUID metadata");
}

// #4584: a uuid-less interface takes its GUID from its `IID_<name>` constant.
#[test]
fn iid_without_uuid() {
    assert_eq!(single("iid_without_uuid"), Vec::<String>::new());
}
