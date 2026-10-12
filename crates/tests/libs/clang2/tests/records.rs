use std::collections::BTreeSet;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use windows_clang2::*;
use windows_metadata::{HasAttributes, Type, TypeAttributes, reader::*};

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
    let def = index.get(namespace, name).next().unwrap();
    assert!(def.class_layout().is_none());
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
            alignment = alignment.max(layout.alignment);
            let offset = if union {
                0
            } else {
                size.div_ceil(layout.alignment) * layout.alignment
            };
            offsets.push((field.name().to_string(), offset));
            size = size.max(offset + layout.size);
        }
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

fn check(snapshot: &Snapshot, rdl: &str, name: &str) {
    let path = Path::new(env!("OUT_DIR")).join(format!("{name}_{:?}.winmd", snapshot.target()));
    windows_rdl::reader()
        .input_text(rdl)
        .output(&path)
        .write()
        .unwrap();
    let index = Index::read(&path).unwrap();
    index.validate_native_contracts(None).unwrap();
    let pointer = u64::from(snapshot.target().pointer_bits() / 8);
    for evidence in snapshot.evidence() {
        if !snapshot
            .fields()
            .any(|(owner, _)| owner.usr == evidence.usr)
        {
            continue;
        }
        let (layout, offsets) = definition_layout(
            &index,
            "Test",
            &evidence.qualified_name,
            pointer,
            &mut BTreeSet::new(),
        );
        assert_eq!(Some(layout), evidence.layout, "{}", evidence.qualified_name);
        let expected = snapshot
            .fields()
            .filter(|(owner, _)| owner.usr == evidence.usr)
            .map(|(_, field)| (field.name.clone(), field.offset))
            .collect::<Vec<_>>();
        assert_eq!(offsets, expected, "{}", evidence.qualified_name);
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
fn unsupported_record_storage_never_becomes_opaque_or_natural() {
    let compiler = Compiler::new().unwrap();
    for source in [
        "struct Record {};",
        "struct Record { unsigned value : 3; };",
        "struct Record { union { int a; float b; }; };",
        "struct Record { struct Nested { int value; } nested; };",
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
        for fixture in ["records", "sdk_records"] {
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
        }
    }
}

#[test]
fn pinned_sdk_point_rect_keep_tags_aliases_and_member_types() {
    let compiler = Compiler::new().unwrap();
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap();
    let resource = std::env::var_os("CLANG_RESOURCE_DIR").map_or_else(
        || repository.join("target\\tool-clang\\clang-resource\\22.1.8"),
        PathBuf::from,
    );
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let mut profile = input(include_str!("../input/winnt.h"), target, &["POINT", "RECT"]);
        profile
            .arguments
            .extend(["-resource-dir".into(), resource.to_str().unwrap().into()]);
        for directory in ["ucrt", "um", "shared"] {
            profile
                .arguments
                .push(format!("-I{}", sdk().join(directory).display()));
        }
        profile.roots.push(sdk().join("shared\\windef.h"));
        let snapshot = compiler.capture(&profile).unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("../input/sdk_records.rdl").replace("\r\n", "\n")
        );
        check(&snapshot, &rdl, "sdk_records");
    }
}
