use std::collections::BTreeSet;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use windows_clang2::*;
use windows_metadata::{HasAttributes, Type, TypeAttributes, reader::*};

#[cfg(target_arch = "x86_64")]
mod support;

fn input(source: &str, target: Target, names: &[&str]) -> Input {
    let mut input = Input::new("records.cpp", source, target);
    input
        .required
        .extend(names.iter().map(|name| name.to_string()));
    input
}

fn sdk() -> PathBuf {
    std::env::var_os("WINDOWS_CLANG2_SDK_INCLUDE").map_or_else(||
        PathBuf::from(std::env::var_os("USERPROFILE").unwrap())
            .join(".nuget\\packages\\microsoft.windows.sdk.cpp\\10.0.28000.2270\\c\\Include\\10.0.28000.0"), PathBuf::from)
}

fn vocabulary(name: &str) -> PathBuf {
    let path = Path::new(env!("OUT_DIR")).join(format!("{name}_vocabulary.winmd"));
    windows_rdl::reader()
        .input_text(include_str!("..\\..\\..\\..\\..\\metadata\\metadata.rdl"))
        .output(&path)
        .write()
        .unwrap();
    path
}

fn sdk_input(source: &str, target: Target, names: &[&str], root: &str) -> Input {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap();
    let resource = std::env::var_os("CLANG_RESOURCE_DIR").map_or_else(
        || repository.join("target\\tool-clang\\clang-resource\\22.1.8"),
        PathBuf::from,
    );
    let mut profile = input(source, target, names);
    profile
        .arguments
        .extend(["-resource-dir".into(), resource.to_str().unwrap().into()]);
    for directory in ["ucrt", "um", "shared"] {
        profile
            .arguments
            .push(format!("-I{}", sdk().join(directory).display()));
    }
    profile.roots.push(sdk().join(root));
    profile
}

fn definition<'a>(index: &'a Index, namespace: &str, name: &str) -> TypeDef<'a> {
    let mut names = name.split('/');
    let mut def = index.get(namespace, names.next().unwrap()).next().unwrap();
    for name in names {
        def = index.nested(def).find(|def| def.name() == name).unwrap();
    }
    def
}

fn type_layout(index: &Index, ty: &Type, pointer: u64, visiting: &mut BTreeSet<String>) -> Layout {
    let scalar = |size| Layout {
        size,
        alignment: size,
    };
    match ty {
        Type::Const(ty) | Type::Volatile(ty) => type_layout(index, ty, pointer, visiting),
        Type::Bool | Type::I8 | Type::U8 => scalar(1),
        Type::I16 | Type::U16 => scalar(2),
        Type::I32 | Type::U32 | Type::F32 => scalar(4),
        Type::I64 | Type::U64 | Type::F64 => scalar(8),
        Type::PtrMut(..) | Type::PtrConst(..) => scalar(pointer),
        Type::ArrayFixed(ty, count) => {
            let layout = type_layout(index, ty, pointer, visiting);
            Layout {
                size: layout.size * *count as u64,
                alignment: layout.alignment,
            }
        }
        Type::ValueName(name) => {
            definition_layout(index, &name.namespace, &name.name, pointer, visiting).0
        }
        _ => panic!("unexpected record storage type: {ty:?}"),
    }
}

fn definition_layout(
    index: &Index,
    namespace: &str,
    name: &str,
    pointer: u64,
    visiting: &mut BTreeSet<String>,
) -> (Layout, Vec<(String, u64)>) {
    let key = format!("{namespace}.{name}");
    assert!(visiting.insert(key.clone()), "cyclic by-value metadata");
    let def = definition(index, namespace, name);
    let packing = def
        .class_layout()
        .map(|layout| u64::from(layout.packing_size()));
    let fields = def.fields().collect::<Vec<_>>();
    let result = if def
        .attributes()
        .any(|attr| attr.name() == "NativeTypedefAttribute")
    {
        assert_eq!(fields.len(), 1);
        (
            type_layout(index, &fields[0].ty(), pointer, visiting),
            Vec::new(),
        )
    } else {
        let union = def.flags().contains(TypeAttributes::ExplicitLayout);
        assert!(union || def.flags().contains(TypeAttributes::SequentialLayout));
        let (mut size, mut alignment) = (0u64, 1u64);
        let mut offsets = Vec::new();
        for field in fields {
            let layout = type_layout(index, &field.ty(), pointer, visiting);
            let required = required_alignment(index, &field.ty(), &mut BTreeSet::new());
            let field_alignment = layout
                .alignment
                .min(packing.unwrap_or(u64::MAX))
                .max(required);
            alignment = alignment.max(field_alignment);
            let offset = if union {
                0
            } else {
                size.div_ceil(field_alignment) * field_alignment
            };
            offsets.push((field.name().to_string(), offset));
            size = size.max(offset + layout.size);
        }
        alignment = alignment.max(forced_alignment(def));
        (
            Layout {
                size: size.div_ceil(alignment) * alignment,
                alignment,
            },
            offsets,
        )
    };
    visiting.remove(&key);
    result
}

fn forced_alignment(def: TypeDef<'_>) -> u64 {
    def.attributes()
        .find(|attr| attr.name() == "AlignmentAttribute")
        .map_or(0, |attr| match attr.value().as_slice() {
            [(_, windows_metadata::Value::I32(value))] => u64::try_from(*value).unwrap(),
            _ => panic!("invalid alignment attribute"),
        })
}

fn required_alignment(index: &Index, ty: &Type, visiting: &mut BTreeSet<String>) -> u64 {
    match ty {
        Type::Const(ty) | Type::Volatile(ty) | Type::ArrayFixed(ty, _) => {
            required_alignment(index, ty, visiting)
        }
        Type::ValueName(name) => {
            let key = format!("{}.{}", name.namespace, name.name);
            assert!(visiting.insert(key.clone()));
            let def = definition(index, &name.namespace, &name.name);
            let required = def
                .fields()
                .map(|field| required_alignment(index, &field.ty(), visiting))
                .max()
                .unwrap_or(0)
                .max(forced_alignment(def));
            visiting.remove(&key);
            required
        }
        _ => 0,
    }
}

fn check(snapshot: &Snapshot, rdl: &str, name: &str) {
    let path = Path::new(env!("OUT_DIR")).join(format!("{name}_{:?}.winmd", snapshot.target()));
    windows_rdl::reader()
        .reference(vocabulary(name))
        .input_text(rdl)
        .output(&path)
        .write()
        .unwrap();
    let index = Index::read(&path).unwrap();
    index.validate_native_contracts(None).unwrap();
    let pointer = u64::from(snapshot.target().pointer_bits() / 8);
    for evidence in snapshot.evidence() {
        if !snapshot.fields().any(|(owner, _)| owner.id == evidence.id) {
            continue;
        }
        if snapshot
            .ownerships()
            .any(|(child, _, anonymous)| child.id == evidence.id && anonymous)
        {
            continue;
        }
        let name = publication(evidence);
        check_record(
            &index,
            snapshot,
            evidence,
            definition(&index, &name.namespace, &name.name),
            pointer,
        );
    }
    for (child, owner, anonymous) in snapshot.ownerships().filter(|(_, _, anonymous)| !anonymous) {
        assert!(!anonymous);
        let name = publication(child);
        let def = definition(&index, &name.namespace, &name.name);
        assert_eq!(def.native_owner().unwrap(), Some(publication(owner)));
    }
}

fn publication(evidence: &Evidence) -> windows_metadata::TypeName {
    let mut names = evidence.qualified_name.split("::").collect::<Vec<_>>();
    let name = names.pop().unwrap();
    let namespace = std::iter::once("Test")
        .chain(names)
        .collect::<Vec<_>>()
        .join(".");
    windows_metadata::TypeName::named(&namespace, name)
}

fn check_record(
    index: &Index,
    snapshot: &Snapshot,
    evidence: &Evidence,
    def: TypeDef<'_>,
    pointer: u64,
) {
    let name = def.full_name();
    let (layout, offsets) = definition_layout(
        index,
        &name.namespace,
        &name.name,
        pointer,
        &mut BTreeSet::new(),
    );
    assert_eq!(Some(layout), evidence.layout, "{}", evidence.qualified_name);
    let fields = snapshot
        .fields()
        .filter(|(owner, _)| owner.id == evidence.id)
        .map(|(_, field)| field)
        .collect::<Vec<_>>();
    assert_eq!(def.fields().len(), fields.len());
    for (slot, ((field, native), (_, offset))) in def.fields().zip(fields).zip(offsets).enumerate()
    {
        assert_eq!(native.ordinal, slot);
        assert_eq!(native.offset, offset);
        assert_eq!(field.is_native_anonymous().unwrap(), native.anonymous);
        if native.anonymous {
            assert!(native.name.is_empty());
        } else {
            assert_eq!(field.name(), native.name);
        }
        if let Some((child, owner, true)) = snapshot
            .ownerships()
            .find(|(child, _, anonymous)| *anonymous && Some(child.id) == native.record_id)
        {
            assert_eq!(owner.id, evidence.id);
            let Type::ValueName(name) = field.ty() else {
                panic!("direct nested record expected")
            };
            let child_def = index
                .nested(def)
                .find(|child| child.full_name() == name)
                .unwrap();
            check_record(index, snapshot, child, child_def, pointer);
        }
    }
}

#[cfg(target_arch = "x86_64")]
fn check_nested(compiler: &Compiler, target: Target) {
    let decimal = compiler
        .capture(&sdk_input(
            include_str!("../input/sdk_nested.h"),
            target,
            &["DECIMAL"],
            "shared\\wtypes.h",
        ))
        .unwrap();
    let rdl = decimal.rdl("Test").unwrap();
    assert_eq!(
        rdl,
        include_str!("../input/sdk_nested.rdl").replace("\r\n", "\n")
    );
    check(&decimal, &rdl, "sdk_nested");
    assert!(
        decimal
            .record_layouts()
            .all(|(_, contract)| contract.packing.is_none() && contract.alignment.is_none())
    );
    for (roots, name, expected) in [
        (
            &["Shapes::Owner", "Shapes::Peer"][..],
            "nested_records",
            include_str!("../input/nested_records.rdl"),
        ),
        (
            &["PackedNested"][..],
            "nested_packed",
            include_str!("../input/nested_packed.rdl"),
        ),
    ] {
        let snapshot = compiler
            .capture(&input(
                include_str!("../input/nested_records.h"),
                target,
                roots,
            ))
            .unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(rdl, expected.replace("\r\n", "\n"));
        check(&snapshot, &rdl, name);
        let original = Path::new(env!("OUT_DIR")).join(format!("{name}_{target:?}.winmd"));
        let text = original.with_extension("rdl");
        windows_rdl::writer()
            .input(&original)
            .output(&text)
            .write()
            .unwrap();
        check(
            &snapshot,
            &std::fs::read_to_string(text).unwrap(),
            &format!("{name}_roundtrip"),
        );
        if name == "nested_records" {
            let reordered = compiler
                .capture(&input(
                    include_str!("../input/nested_records.h"),
                    target,
                    &["Shapes::Peer", "Shapes::Owner::Node"],
                ))
                .unwrap();
            assert_eq!(reordered.rdl("Test").unwrap(), rdl);
            let mut moved = input(include_str!("../input/nested_records.h"), target, roots);
            moved.name = "moved.cpp".to_string();
            assert_eq!(compiler.capture(&moved).unwrap().rdl("Test").unwrap(), rdl);
            let index = Index::read(&original).unwrap();
            let owner = index.expect("Test.Shapes", "Owner");
            let fields = owner.fields().collect::<Vec<_>>();
            assert_eq!(fields[3].ty(), fields[4].ty());
            assert_ne!(
                index.expect("Test.Shapes.Owner", "Node").full_name(),
                index.expect("Test.Shapes.Peer", "Node").full_name()
            );
            assert!(fields[7].is_native_anonymous().unwrap());
            assert!(!fields[8].is_native_anonymous().unwrap());
            assert_ne!(fields[7].name(), fields[8].name());
            let native = snapshot
                .fields()
                .filter(|(owner, _)| owner.qualified_name == "Shapes::Owner")
                .map(|(_, field)| field)
                .collect::<Vec<_>>();
            assert_ne!(native[6].record_id, native[7].record_id);
            let first = snapshot
                .evidence()
                .find(|item| Some(item.id) == native[6].record_id)
                .unwrap();
            let second = snapshot
                .evidence()
                .find(|item| Some(item.id) == native[7].record_id)
                .unwrap();
            assert_eq!(first.usr, second.usr);
            let copied = snapshot.clone();
            assert_eq!(copied.rdl("Test").unwrap(), rdl);
        }
    }
    for name in [
        "AnonymousArray",
        "AnonymousPointer",
        "SharedAnonymous",
        "NamedInUnnamed",
    ] {
        let snapshot = compiler
            .capture(&input(
                include_str!("../input/nested_rejected.h"),
                target,
                &[name],
            ))
            .unwrap();
        let error = snapshot.rdl("Test").unwrap_err();
        assert_eq!(error.stage, Stage::Representation, "{name}: {error}");
    }
    for name in [
        "PrivateOwner::Inner",
        "BehavioralOwner::Inner",
        "QualifiedAnonymous",
    ] {
        let error = compiler
            .capture(&input(
                include_str!("../input/nested_rejected.h"),
                target,
                &[name],
            ))
            .unwrap_err();
        assert_eq!(error.stage, Stage::Capture, "{name}: {error}");
    }
}

#[test]
fn reviewed_records_preserve_graph_and_storage_on_three_targets() {
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(
                include_str!("../input/records.h"),
                target,
                &["Container", "Arrays", "Left"],
            ))
            .unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("../input/records.rdl").replace("\r\n", "\n")
        );
        check(&snapshot, &rdl, "records");
        let pointer = u64::from(target.pointer_bits() / 8);
        for (name, size, alignment) in [
            ("Point", 8, 4),
            ("Storage", 8, 8),
            ("Arrays", 20, 4),
            ("Left", pointer, pointer),
            ("Right", pointer, pointer),
            ("Node", if pointer == 4 { 8 } else { 16 }, pointer),
            ("Container", if pointer == 4 { 32 } else { 40 }, 8),
        ] {
            assert_eq!(
                snapshot
                    .evidence()
                    .find(|item| item.qualified_name == name)
                    .unwrap()
                    .layout,
                Some(Layout { size, alignment })
            );
        }
    }
}

#[test]
#[cfg(target_arch = "x86_64")]
fn reviewed_explicit_layouts_preserve_contracts_on_three_targets() {
    if support::worker("reviewed_explicit_layouts_preserve_contracts_on_three_targets") {
        return;
    }
    let native = PathBuf::from(std::env::var_os("WINDOWS_CLANG2_LAYOUT_NATIVE").unwrap());
    let compiler = Compiler::with_layout_bridge(native.join("windows_clang_layout.dll")).unwrap();
    let error =
        Compiler::with_layout_bridge(Path::new(env!("OUT_DIR")).join("windows_clang_layout.dll"))
            .err()
            .unwrap();
    assert_eq!(
        error.message,
        "layout bridge must accompany its matched parser"
    );
    for (name, message) in [
        ("bad_abi", "layout bridge ABI mismatch"),
        (
            "bad_hash",
            "layout bridge parser build fingerprint mismatch",
        ),
    ] {
        let error =
            Compiler::with_layout_bridge(native.join(format!("windows_clang_layout_{name}.dll")))
                .err()
                .unwrap();
        assert_eq!(error.stage, Stage::Capture);
        assert_eq!(error.message, message);
    }
    let mut retained = None;
    for target in [Target::X86, Target::X64, Target::Arm64] {
        check_nested(&compiler, target);
        let snapshot = compiler
            .capture(&input(
                include_str!("../input/layout_contracts.h"),
                target,
                &[
                    "AlignOne",
                    "AlignFour",
                    "Aligned",
                    "PackTwo",
                    "PackFour",
                    "PackedPointer",
                    "PackedUnion",
                    "PackedOuter",
                ],
            ))
            .unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("../input/layout_contracts.rdl").replace("\r\n", "\n")
        );
        let evidence = snapshot.layout_bridge.as_ref().unwrap();
        assert_eq!(evidence.abi, 3);
        assert_eq!(
            evidence.library.file_name().unwrap(),
            "windows_clang_layout.dll"
        );
        assert_eq!(evidence.parser_sha256.len(), 64);
        for (name, packing, alignment) in [
            ("PackTwo", Some(2), None),
            ("PackFour", Some(4), None),
            ("AlignOne", None, Some(1)),
            ("AlignFour", None, Some(4)),
            ("Aligned", None, Some(16)),
        ] {
            let (_, contract) = snapshot
                .record_layouts()
                .find(|(owner, _)| owner.qualified_name == name)
                .unwrap();
            assert_eq!((contract.packing, contract.alignment), (packing, alignment));
        }
        check(&snapshot, &rdl, "layout_contracts");
        let index = Index::read(
            Path::new(env!("OUT_DIR")).join(format!("layout_contracts_{target:?}.winmd")),
        )
        .unwrap();
        for (owner, contract) in snapshot.record_layouts() {
            let def = index.get("Test", &owner.qualified_name).next().unwrap();
            assert_eq!(
                def.class_layout().map(|layout| layout.packing_size()),
                contract.packing
            );
            assert_eq!(
                forced_alignment(def),
                contract.alignment.map_or(0, u64::from)
            );
        }
        let (_, contract) = snapshot
            .record_layouts()
            .find(|(owner, _)| owner.qualified_name == "PackedOuter")
            .unwrap();
        assert_eq!(contract.required_alignment, 16);
        retained = Some(snapshot);
        for name in ["FieldAligned", "PackedAttr", "GnuAligned", "CxxAligned"] {
            assert_eq!(
                compiler
                    .capture(&input(
                        include_str!("../input/layout_contracts.h"),
                        target,
                        &[name]
                    ))
                    .unwrap_err()
                    .stage,
                Stage::Capture
            );
        }
    }
    std::thread::scope(|scope| {
        let tasks = [Target::X86, Target::X64, Target::Arm64].map(|target| {
            let compiler = &compiler;
            scope.spawn(move || {
                let snapshot = compiler
                    .capture(&input(
                        include_str!("../input/layout_contracts.h"),
                        target,
                        &[
                            "AlignOne",
                            "AlignFour",
                            "PackTwo",
                            "PackFour",
                            "PackedPointer",
                            "PackedUnion",
                            "PackedOuter",
                        ],
                    ))
                    .unwrap();
                assert_eq!(
                    snapshot.rdl("Test").unwrap(),
                    include_str!("../input/layout_contracts.rdl").replace("\r\n", "\n")
                );
            })
        });
        for task in tasks {
            task.join().unwrap();
        }
    });
    let nested_retained = compiler
        .capture(&input(
            include_str!("../input/nested_records.h"),
            Target::Arm64,
            &["Shapes::Owner", "Shapes::Peer"],
        ))
        .unwrap();
    drop(compiler);
    assert_eq!(
        nested_retained.rdl("Test").unwrap(),
        include_str!("../input/nested_records.rdl").replace("\r\n", "\n")
    );
    assert_eq!(
        retained.unwrap().rdl("Test").unwrap(),
        include_str!("../input/layout_contracts.rdl").replace("\r\n", "\n")
    );
    let ordinary = Compiler::new().unwrap();
    let snapshot = ordinary
        .capture(&input(
            include_str!("../input/layout_contracts.h"),
            Target::X64,
            &["PackOne"],
        ))
        .unwrap();
    assert_eq!(
        snapshot.rdl("Test").unwrap_err().stage,
        Stage::Representation
    );
}

#[test]
fn existing_plain_class_anchor_is_storage_not_interface() {
    let compiler = Compiler::new().unwrap();
    let source = include_str!("..\\..\\clang\\input\\data_classes.h");
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(source, target, &["BitmapData"]))
            .unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("../input/bitmap.rdl").replace("\r\n", "\n")
        );
        check(&snapshot, &rdl, "bitmap");
    }

    for name in ["CProperty", "IPropertyList"] {
        assert_eq!(
            compiler
                .capture(&input(source, Target::X64, &[name]))
                .unwrap_err()
                .stage,
            Stage::Capture
        );
    }
}

#[test]
fn named_nested_records_use_the_ordinary_compiler_path() {
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(
                include_str!("../input/nested_records.h"),
                target,
                &["Shapes::Peer::Node"],
            ))
            .unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("../input/nested_named.rdl").replace("\r\n", "\n")
        );
        check(&snapshot, &rdl, "nested_named");
    }
}

#[test]
fn unsupported_record_storage_never_becomes_opaque_or_natural() {
    let compiler = Compiler::new().unwrap();
    for source in [
        "struct Record {};",
        "struct Record { unsigned value : 3; };",
        "struct Record { union { int a; float b; }; };",
        "struct Base { int value; }; struct Record : Base { int other; };",
        "struct Record { int value; void Method(); };",
        "class Record { int value; };",
        "struct Record { int &value; };",
        "struct Record { int data[]; };",
        "struct Record { int value __attribute__((annotate(\"contract\"))); };",
        "struct __declspec(align(16)) Record { int value; };",
    ] {
        assert_eq!(
            compiler
                .capture(&input(source, Target::X64, &["Record"]))
                .unwrap_err()
                .stage,
            Stage::Capture,
            "{source}"
        );
    }
    for source in [
        "#pragma pack(push, 1)\nstruct Record { char a; int b; };\n#pragma pack(pop)",
        "struct Record { int data[0]; };",
    ] {
        let snapshot = compiler
            .capture(&input(source, Target::X64, &["Record"]))
            .unwrap();
        assert_eq!(
            snapshot.rdl("Test").unwrap_err().stage,
            Stage::Representation,
            "{source}"
        );
    }
}

#[test]
fn native_msvc_compiles_the_independent_storage_assertions() {
    let visual_studio = std::env::var_os("VSINSTALLDIR").map_or_else(
        || {
            let finder = PathBuf::from(std::env::var_os("ProgramFiles(x86)").unwrap())
                .join("Microsoft Visual Studio\\Installer\\vswhere.exe");
            let output = Command::new(finder)
                .args([
                    "-latest",
                    "-products",
                    "*",
                    "-requires",
                    "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
                    "-property",
                    "installationPath",
                ])
                .output()
                .unwrap();
            assert!(output.status.success());
            PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
        },
        PathBuf::from,
    );
    let setup = visual_studio.join("VC\\Auxiliary\\Build\\vcvarsall.bat");
    assert!(setup.is_file(), "missing MSVC environment setup");
    for arch in ["x86", "amd64", "amd64_arm64"] {
        for fixture in [
            "records",
            "sdk_records",
            "layout_contracts",
            "nested_records",
            "sdk_nested",
            "interfaces",
        ] {
            let source =
                Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("input\\{fixture}.cpp"));
            let object = Path::new(env!("OUT_DIR")).join(format!("{fixture}_{arch}.obj"));
            let includes = ["ucrt", "um", "shared"]
                .map(|directory| format!("/I\"{}\"", sdk().join(directory).display()))
                .join(" ");
            let command = format!(
                "call \"{}\" {arch} >nul && cl /nologo /std:c++20 /W4 /WX /c {includes} /Fo\"{}\" \"{}\"",
                setup.display(),
                object.display(),
                source.display()
            );
            let output = Command::new(std::env::var_os("ComSpec").unwrap())
                .raw_arg(format!("/d /s /c \"{command}\""))
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{fixture}/{arch}: {}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(object.is_file());
            if fixture == "interfaces" && arch != "amd64_arm64" {
                let executable = object.with_extension("exe");
                let command = format!(
                    "call \"{}\" {arch} >nul && cl /nologo \"{}\" /Fe\"{}\"",
                    setup.display(),
                    object.display(),
                    executable.display()
                );
                let output = Command::new(std::env::var_os("ComSpec").unwrap())
                    .raw_arg(format!("/d /s /c \"{command}\""))
                    .output()
                    .unwrap();
                assert!(
                    output.status.success(),
                    "{fixture}/{arch}: {}\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                let status = Command::new(&executable).status().unwrap();
                assert!(status.success(), "{fixture}/{arch}: {status}");
            }
        }
    }
}

#[test]
fn pinned_sdk_point_rect_keep_tags_aliases_and_member_types() {
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let profile = sdk_input(
            include_str!("../input/winnt.h"),
            target,
            &["POINT", "RECT"],
            "shared\\windef.h",
        );
        let snapshot = compiler.capture(&profile).unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("../input/sdk_records.rdl").replace("\r\n", "\n")
        );
        check(&snapshot, &rdl, "sdk_records");
    }
}
