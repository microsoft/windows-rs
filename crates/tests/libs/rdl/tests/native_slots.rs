use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_slots_{name}.winmd"))
}

fn vocabulary(name: &str) -> PathBuf {
    let path = output(&format!("vocabulary_{name}"));
    windows_rdl::reader()
        .input_text(include_str!("../../../../../metadata/metadata.rdl"))
        .output(&path)
        .write()
        .unwrap();
    path
}

fn compile(source: &str, path: &Path, reference: &Path) {
    windows_rdl::reader()
        .reference(reference)
        .input_text(source)
        .output(path)
        .write()
        .unwrap();
}

fn check(path: &Path, root_namespace: &str, derived_namespace: &str, leaf_namespace: &str) {
    let index = Index::read(path).unwrap();
    index.validate_native_contracts(None).unwrap();
    let root = index.expect(root_namespace, "Root");
    assert_eq!(
        root.native_vtable().unwrap(),
        Some(NativeVtable {
            size: 2,
            inherited: 0
        })
    );
    let methods: Vec<_> = root.methods().collect();
    assert_eq!(methods.len(), 2);
    assert_eq!(methods[0].signature(&[]).types, [Type::F32]);
    assert_eq!(methods[1].signature(&[]).types, [Type::I32]);
    assert_eq!(
        methods
            .iter()
            .map(|method| method.native_slot().unwrap().unwrap().index)
            .collect::<Vec<_>>(),
        [1, 0]
    );
    assert!(methods.iter().all(
        |method| method.name() == "Echo" && method.flags().contains(MethodAttributes::NewSlot)
    ));
    let derived = index.expect(derived_namespace, "Derived");
    assert_eq!(
        derived.native_vtable().unwrap(),
        Some(NativeVtable {
            size: 3,
            inherited: 2
        })
    );
    let methods: Vec<_> = derived.methods().collect();
    assert_eq!(methods.len(), 2);
    assert_eq!(
        methods[0].native_slot().unwrap(),
        Some(NativeSlot {
            index: 2,
            reuse: None
        })
    );
    assert_eq!(
        methods[1].native_slot().unwrap(),
        Some(NativeSlot {
            index: 1,
            reuse: Some(TypeName::named(root_namespace, "Root")),
        })
    );
    assert!(!methods[1].flags().contains(MethodAttributes::NewSlot));
    let leaf = index.expect(leaf_namespace, "Leaf");
    let methods: Vec<_> = leaf.methods().collect();
    assert_eq!(
        methods
            .iter()
            .map(|method| method.native_slot().unwrap().unwrap())
            .collect::<Vec<_>>(),
        [
            NativeSlot {
                index: 1,
                reuse: Some(TypeName::named(derived_namespace, "Derived"))
            },
            NativeSlot {
                index: 0,
                reuse: Some(TypeName::named(root_namespace, "Root"))
            },
        ]
    );
    assert_eq!(methods[0].name(), "Renamed");
    assert!(
        methods
            .iter()
            .all(|method| !method.flags().contains(MethodAttributes::NewSlot))
    );
}

#[test]
fn reviewed_native_slots_preserve_source_order_and_inherited_declaration_identity() {
    let reference = vocabulary("reviewed");
    let source = include_str!("../native_slots.rdl");
    let input = output("reviewed");
    compile(source, &input, &reference);
    check(&input, "Test", "Test", "Test");
    let index = Index::read(&input).unwrap();
    let cv: Vec<_> = index.expect("Test", "Cv").methods().collect();
    assert_eq!(cv[0].signature(&[]).types, cv[1].signature(&[]).types);
    assert_eq!(
        cv[0].signature(&[]).return_type,
        cv[1].signature(&[]).return_type
    );
    assert_eq!(cv[0].signature(&[]).flags, cv[1].signature(&[]).flags);
    assert_ne!(cv[0].native_slot().unwrap(), cv[1].native_slot().unwrap());
    let payload = |method: MethodDef<'_>| {
        method
            .find_attribute("NativeAnnotationAttribute")
            .unwrap()
            .value()[2]
            .1
            .clone()
    };
    assert_ne!(payload(cv[0]), payload(cv[1]));
    let rdl = input.with_extension("rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(&rdl).unwrap(),
        source.replace("\r\n", "\n")
    );
    let dir = Path::new(env!("OUT_DIR")).join("native_slots_roundtrip");
    std::fs::create_dir_all(&dir).unwrap();
    let roundtrip = dir.join(input.file_name().unwrap());
    compile(
        &std::fs::read_to_string(&rdl).unwrap(),
        &roundtrip,
        &reference,
    );
    assert_eq!(
        std::fs::read(&input).unwrap(),
        std::fs::read(&roundtrip).unwrap()
    );
    let merged = output("merged");
    merge().input(&input).output(&merged).merge().unwrap();
    check(&merged, "Test", "Test", "Test");
    let remapped = output("remapped");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .route("Root", "Bases")
        .route("Derived", "Derived")
        .route("Leaf", "Leaves")
        .fallback("Other")
        .output(&remapped)
        .remap()
        .unwrap();
    check(&remapped, "Bases", "Derived", "Leaves");
    let rdl = remapped.with_extension("rdl");
    windows_rdl::writer()
        .input(&remapped)
        .output(&rdl)
        .write()
        .unwrap();
    let recompiled = output("remapped_roundtrip");
    compile(
        &std::fs::read_to_string(rdl).unwrap(),
        &recompiled,
        &reference,
    );
    check(&recompiled, "Bases", "Derived", "Leaves");
    let inventory =
        windows_rdl::inventory(Path::new(env!("CARGO_MANIFEST_DIR")).join("native_slots.rdl"))
            .unwrap();
    assert!(
        inventory
            .iter()
            .any(|entry| entry.name == "Renamed" && entry.attributes.contains("reuse = Derived"))
    );
}

#[test]
fn invalid_models_fail_before_output_replacement() {
    let reference = vocabulary("invalid");
    let base =
        "#[no_guid] #[vtable(size=1,inherited=0)] interface Base { #[slot(0)] fn A(&self); }";
    for (declaration, diagnostic) in [
        (
            "#[no_guid] #[vtable(size=1,inherited=0)] interface I { fn A(&self); }",
            "missing its slot",
        ),
        (
            "#[no_guid] interface I { #[slot(0)] fn A(&self); }",
            "requires a vtable",
        ),
        (
            "#[no_guid] #[vtable(size=2,inherited=0)] interface I { #[slot(0)] fn A(&self); }",
            "complete vtable suffix",
        ),
        (
            "#[no_guid] #[vtable(size=1,inherited=0)] interface I { #[slot(1)] fn A(&self); }",
            "within the vtable",
        ),
        (
            "#[no_guid] #[vtable(size=1,inherited=0)] interface I { #[slot(0)] fn A(&self); #[slot(0)] fn B(&self); }",
            "unique",
        ),
        (
            "#[no_guid] #[vtable(size=1,inherited=1)] interface I {}",
            "root interface",
        ),
        (
            "#[no_guid] #[vtable(size=0,inherited=1)] interface I {}",
            "inherited <= size",
        ),
        (
            "#[no_guid] #[vtable(size=1,inherited=0)] interface I : Base { #[slot(0)] fn A(&self); }",
            "disagrees",
        ),
        (
            "#[no_guid] #[vtable(size=2,inherited=1)] interface I : Base { #[slot(0)] fn A(&self); #[slot(1)] fn B(&self); }",
            "inherited prefix",
        ),
        (
            "#[no_guid] #[vtable(size=2,inherited=1)] interface I : Base { #[slot(1,reuse=Base)] fn B(&self); }",
            "inherited prefix",
        ),
        (
            "#[no_guid] #[vtable(size=1,inherited=1)] interface I : Base { #[slot(0,reuse=I)] fn A(&self); }",
            "ancestor",
        ),
        (
            "#[no_guid] #[vtable(size=0,inherited=0)] interface I : I {}",
            "cyclic",
        ),
        (
            "#[no_guid] #[vtable(size=0,inherited=0)] interface I<T> {}",
            "nongeneric",
        ),
        (
            "#[no_guid] #[vtable(size=1,inherited=0)] interface I { #[slot(0)] fn A<T>(&self); }",
            "generic methods",
        ),
        (
            "#[no_guid] #[vtable(size=1,inherited=0)] interface I { #[slot(0)] fn A(&self,...); }",
            "variadic",
        ),
        (
            "#[no_guid] #[vtable(size=1,inherited=0)] interface I { #[arch(X86)] #[slot(0)] fn A(&self); }",
            "interface-level",
        ),
        (
            "#[no_guid] #[Windows::Win32::Metadata::NativeVtable(1,0)] interface I {
            #[arch(X86)] #[Windows::Win32::Metadata::NativeSlot(0)] fn A(&self); }",
            "interface-level",
        ),
        (
            "#[no_guid] #[Windows::Win32::Metadata::NativeVtable(1,0)] interface I {
            #[Windows::Win32::Metadata::NativeSlot(0)] fn A<T>(&self); }",
            "generic methods",
        ),
        (
            "#[no_guid] #[Windows::Win32::Metadata::NativeVtable(1,0)] interface I {
            #[Windows::Win32::Metadata::NativeSlot(0)] fn A(&self,...); }",
            "variadic",
        ),
        (
            "#[no_guid] #[vtable(size=0,inherited=0)] #[vtable(size=0,inherited=0)] interface I {}",
            "duplicate",
        ),
        (
            "#[no_guid] #[vtable(size=1,inherited=0)] interface I { #[slot(0)] #[slot(0)] fn A(&self); }",
            "duplicate",
        ),
    ] {
        let path = output("invalid");
        std::fs::write(&path, b"last good output").unwrap();
        let result = windows_rdl::reader()
            .reference(&reference)
            .input_text(&format!("#[win32] mod Test {{ {base} {declaration} }}"))
            .output(&path)
            .write();
        assert!(result.is_err(), "unexpected acceptance: {declaration}");
        let error = result.unwrap_err();
        assert!(
            error.to_string().contains(diagnostic),
            "{declaration}: {error}"
        );
        assert_eq!(std::fs::read(path).unwrap(), b"last good output");
    }
}

#[test]
fn reference_and_unmodeled_base_evidence_is_not_replaced_by_method_row_order() {
    let reference = vocabulary("external");
    let base = output("external_base");
    compile(
        "#[win32] mod External { #[no_guid] interface Base {} }",
        &base,
        &reference,
    );
    let input = output("external");
    windows_rdl::reader().reference(&reference).reference(&base).input_text(
        "#[win32] mod Test { #[no_guid] #[vtable(size=4,inherited=3)] interface I : External::Base {
        #[slot(3)] fn New(&self); #[slot(1,reuse=External::Base)] fn Override(&self); } }")
        .output(&input).write().unwrap();
    let index = Index::read(&input).unwrap();
    index.validate_native_contracts(None).unwrap();
    assert_eq!(
        index
            .expect("Test", "I")
            .native_vtable()
            .unwrap()
            .unwrap()
            .inherited,
        3
    );
    let rdl = input.with_extension("rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&rdl)
        .write()
        .unwrap();
    assert!(
        std::fs::read_to_string(rdl)
            .unwrap()
            .contains("reuse = External::Base")
    );
}

#[test]
fn architecture_merge_preserves_differing_native_orders_with_identical_method_signatures() {
    let reference = vocabulary("architecture");
    let mut merger = merge();
    for (arch, first, second) in [(1, 0, 1), (2, 1, 0)] {
        let input = output(&format!("architecture_{arch}"));
        compile(&format!(
            "#[win32] mod Test {{ #[no_guid] #[vtable(size=2,inherited=0)] interface I {{
            #[slot({first})] fn A(&self, value:i32); #[slot({second})] fn B(&self, value:i32); }} }}"),
            &input, &reference);
        merger.arch_input(input, arch);
    }

    let merged = output("architecture_merged");
    merger.output(&merged).merge().unwrap();
    assert_eq!(Index::read(&merged).unwrap().get("Test", "I").count(), 2);
    let remapped = output("architecture_remapped");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .fallback("Routed")
        .output(&remapped)
        .remap()
        .unwrap();
    for (arch, first) in [(1, 0), (2, 1)] {
        let index = Index::new_for_architecture(vec![File::read(&remapped).unwrap()], arch);
        let interface = index.expect("Routed", "I");
        assert_eq!(
            interface
                .methods()
                .next()
                .unwrap()
                .native_slot()
                .unwrap()
                .unwrap()
                .index,
            first
        );
    }
}

#[test]
fn architecture_merge_preserves_parameter_and_return_contracts_with_identical_slots() {
    let reference = vocabulary("parameter_architecture");
    for (case, parameter, result) in [
        ("direction", true, false),
        ("parameter", true, false),
        ("return", false, true),
    ] {
        let mut merger = merge();
        for arch in [1, 2] {
            let input = output(&format!("{case}_architecture_{arch}"));
            let direction = if case == "direction" && arch == 1 {
                "in"
            } else {
                "out"
            };
            let annotation = |enabled: bool| {
                if enabled && case != "direction" {
                    format!(
                        "#[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"evidence {arch}\")]"
                    )
                } else {
                    String::new()
                }
            };
            compile(
                &format!(
                    "#[win32] mod Test {{ #[no_guid] #[vtable(size=1,inherited=0)] interface I {{
                    #[slot(0)] fn A(&self, {} #[{direction}] value:*mut i32)
                    -> {} void; }} }}",
                    annotation(parameter),
                    annotation(result)
                ),
                &input,
                &reference,
            );
            merger.arch_input(input, arch);
        }
        let merged = output(&format!("{case}_architecture_merged"));
        merger.output(&merged).merge().unwrap();
        assert_eq!(
            Index::read(&merged).unwrap().get("Test", "I").count(),
            2,
            "{case}"
        );
        for arch in [1, 2] {
            let index = Index::new_for_architecture(vec![File::read(&merged).unwrap()], arch);
            let method = index.expect("Test", "I").methods().next().unwrap();
            if case == "direction" {
                assert_eq!(
                    method
                        .params()
                        .find(|param| param.sequence() == 1)
                        .unwrap()
                        .flags(),
                    if arch == 1 {
                        ParamAttributes::In
                    } else {
                        ParamAttributes::Out
                    }
                );
            } else {
                let sequence = if result { 0 } else { 1 };
                let param = method
                    .params()
                    .find(|param| param.sequence() == sequence)
                    .unwrap();
                assert_eq!(
                    param
                        .find_attribute("NativeAnnotationAttribute")
                        .unwrap()
                        .value()[2]
                        .1,
                    Value::Utf8(format!("evidence {arch}"))
                );
            }
        }
    }
}

#[test]
fn neutral_reuse_owner_must_exist_on_every_known_architecture_lineage() {
    let reference = vocabulary("lineage");
    let bases = "#[no_guid] #[vtable(size=1,inherited=0)] interface Root {
            #[slot(0)] fn A(&self); }
            #[no_guid] #[vtable(size=1,inherited=0)] interface Other {
            #[slot(0)] fn A(&self); }
            #[arch(X86)] #[no_guid] #[vtable(size=1,inherited=1)] interface Base: Root {}
            #[arch(X64)] #[no_guid] #[vtable(size=1,inherited=1)] interface Base: Other {}";
    for (arch, accepted) in [("", false), ("#[arch(X86)]", true)] {
        let path = output(if accepted {
            "lineage_x86"
        } else {
            "lineage_neutral"
        });
        std::fs::write(&path, b"last good output").unwrap();
        let result = windows_rdl::reader()
            .reference(&reference)
            .input_text(&format!(
                "#[win32] mod Test {{ {bases} {arch} #[no_guid]
                #[vtable(size=1,inherited=1)] interface I: Base {{
                #[slot(0,reuse=Root)] fn A(&self); }} }}"
            ))
            .output(&path)
            .write();
        if accepted {
            result.unwrap();
        } else {
            assert!(result.unwrap_err().to_string().contains("ancestor"));
            assert_eq!(std::fs::read(path).unwrap(), b"last good output");
        }
    }
}
