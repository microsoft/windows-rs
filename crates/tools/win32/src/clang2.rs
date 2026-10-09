use super::*;
use std::collections::{BTreeMap, BTreeSet};
use windows_clang2::{
    DeclarationInfo, FunctionImport, Input, PointerSized, ProjectionOptions, ReferenceKind,
    TypeReference,
};

pub fn audio(rdl_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut options = ProjectionOptions::new("Win32Audio");
    for (native, namespace, name, kind) in [
        (
            "HRESULT",
            "Windows.Foundation",
            "HResult",
            ReferenceKind::Value,
        ),
        ("BOOL", ROOT, "BOOL", ReferenceKind::Value),
        ("IUnknown", ROOT, "IUnknown", ReferenceKind::Interface),
        (
            "IPropertyStore",
            ROOT,
            "IPropertyStore",
            ReferenceKind::Interface,
        ),
        ("tagPROPVARIANT", ROOT, "PROPVARIANT", ReferenceKind::Value),
        ("LPWSTR", ROOT, "PWSTR", ReferenceKind::Value),
        ("LPCWSTR", ROOT, "PCWSTR", ReferenceKind::Value),
    ] {
        options.references.insert(
            native.into(),
            TypeReference {
                namespace: namespace.into(),
                name: name.into(),
                kind,
            },
        );
    }
    imports(&mut options, &["mmdevapi.lib"])?;
    generate(
        "audio",
        &["mmdeviceapi.h", "endpointvolume.h"],
        &options,
        vec![Input::new(
            "clang-win32-audio-values.hpp",
            format!("{PRELUDE}\n#include <initguid.h>\n#include <mmdeviceapi.h>"),
        )],
        rdl_only,
    )
}

pub fn headers(headers: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let all = headers == ["all"];
    if !all {
        validate_headers(headers)?;
    }
    let mut options = header_options();
    let provenance = imports(&mut options, IMPORT_LIBS)?;
    let output = std::path::Path::new("target\\win32-clang2");
    std::fs::create_dir_all(output)?;
    std::fs::write(output.join("imports.tsv"), provenance)?;
    if all {
        return all_headers(&options, output);
    }
    let name = headers
        .iter()
        .map(|header| rdl_partition_stem(header))
        .collect::<Vec<_>>()
        .join("-");
    generate(&name, headers, &options, vec![], true)
}

fn all_headers(
    options: &ProjectionOptions,
    output: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut summary = String::from("header\tstatus\tselected\temitted\trejected\tblocked\terror\n");
    let mut failed = 0;
    for header in HEADERS.iter().chain(SATELLITE_HEADERS) {
        let name = rdl_partition_stem(header);
        let result = generate(&name, &[header], options, vec![], true);
        let error = result
            .as_ref()
            .err()
            .map(ToString::to_string)
            .unwrap_or_default();
        if result.is_err() {
            failed += 1;
            eprintln!("clang2 {header}: {error}");
        }
        let inventory = output.join(format!("{name}-rdl")).join("inventory.tsv");
        let counts = if inventory.exists() {
            let mut names = BTreeMap::new();
            for row in std::fs::read_to_string(inventory)?.lines().skip(1) {
                let fields: Vec<_> = row.split('\t').collect();
                if fields.len() != 6 {
                    return Err("malformed generated declaration inventory".into());
                }
                if fields[4] != "excluded" {
                    names.insert(fields[3].to_string(), fields[4].to_string());
                }
            }
            format!(
                "{}\t{}\t{}\t{}",
                names.len(),
                names.values().filter(|status| *status == "emitted").count(),
                names
                    .values()
                    .filter(|status| *status == "rejected")
                    .count(),
                names.values().filter(|status| *status == "blocked").count(),
            )
        } else {
            "unavailable\tunavailable\tunavailable\tunavailable".into()
        };
        summary.push_str(&format!(
            "{header}\t{}\t{counts}\t{}\n",
            if result.is_ok() {
                "complete"
            } else {
                "incomplete"
            },
            error.replace(['\t', '\r', '\n'], " "),
        ));
        std::fs::write(output.join("manifest.tsv"), &summary)?;
    }
    if failed != 0 {
        return Err(format!(
            "{failed} manifest headers have incomplete coverage; see {}",
            output.join("manifest.tsv").display()
        )
        .into());
    }
    Ok(())
}

fn validate_headers(headers: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if headers.is_empty()
        || headers
            .iter()
            .any(|header| !HEADERS.contains(header) && !SATELLITE_HEADERS.contains(header))
    {
        return Err("clang2 headers must name entries in the tool-win32 header manifest".into());
    }
    if headers.iter().copied().collect::<BTreeSet<_>>().len() != headers.len() {
        return Err("clang2 headers must not contain duplicate entries".into());
    }
    Ok(())
}

fn header_options() -> ProjectionOptions {
    let mut options = ProjectionOptions::new(ROOT);
    options.exclude_inline_functions = true;
    for name in [
        "UINT_PTR",
        "ULONG_PTR",
        "DWORD_PTR",
        "SIZE_T",
        "size_t",
        "rsize_t",
        "uintptr_t",
    ] {
        options
            .pointer_sized
            .insert(name.into(), PointerSized::Unsigned);
    }
    for name in ["INT_PTR", "LONG_PTR", "SSIZE_T", "intptr_t", "ptrdiff_t"] {
        options
            .pointer_sized
            .insert(name.into(), PointerSized::Signed);
    }
    options
}

fn imports(
    options: &mut ProjectionOptions,
    libraries: &[&str],
) -> Result<String, Box<dyn std::error::Error>> {
    let dirs = sdk_lib_dirs();
    let mut provenance = String::from("source\tsymbol\tlibrary\ttarget\tselected\n");
    for library in libraries {
        let path = resolve(library, &dirs, "library", "pinned SDK libraries");
        append_imports(
            options,
            library,
            windows_rdl::implib::read(&std::fs::read(path)?)?,
            &mut provenance,
        )?;
    }
    Ok(provenance)
}

fn append_imports(
    options: &mut ProjectionOptions,
    library: &str,
    imports: Vec<windows_rdl::implib::Import>,
    provenance: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = BTreeMap::new();
    for import in imports {
        if import.kind != windows_rdl::implib::ImportKind::Code {
            continue;
        }
        if let Some(previous) = entries.insert(import.symbol.clone(), import.clone())
            && previous != import
        {
            return Err(format!(
                "conflicting DLL imports within `{library}` for `{}`",
                import.symbol
            )
            .into());
        }
        let target = match import.target {
            windows_rdl::implib::ImportTarget::Name(name) => {
                windows_clang2::ImportTarget::Name(name)
            }
            windows_rdl::implib::ImportTarget::Ordinal(ordinal) => {
                windows_clang2::ImportTarget::Ordinal(ordinal)
            }
        };
        let value = FunctionImport {
            library: import.dll,
            target,
        };
        let selected = !options.imports.contains_key(&import.symbol);
        provenance.push_str(&format!(
            "{library}\t{}\t{}\t{}\t{selected}\n",
            import.symbol,
            value.library,
            match &value.target {
                windows_clang2::ImportTarget::Name(name) => name.clone(),
                windows_clang2::ImportTarget::Ordinal(ordinal) => format!("#{ordinal}"),
            },
        ));
        if selected {
            options.imports.insert(import.symbol, value);
        }
    }
    Ok(())
}

fn generate(
    name: &str,
    headers: &[&str],
    options: &ProjectionOptions,
    definitions: Vec<Input>,
    rdl_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let time = std::time::Instant::now();
    let output = std::path::PathBuf::from(if rdl_only {
        format!("target/win32-clang2/{name}-rdl")
    } else {
        format!("target/win32-clang2/{name}")
    });
    let output = output.as_path();
    clear_outputs(output)?;
    let rdl = output.join("rdl");
    let include_dirs = sdk_include_dirs();
    let files: Vec<_> = headers
        .iter()
        .map(|header| resolve(header, &include_dirs, "header", "pinned SDK include"))
        .collect();
    let inputs: Vec<_> = clang_inputs_with_prerequisites(
        headers,
        &include_dirs,
        false,
        header_profiles::prerequisites,
    )
    .into_iter()
    .map(|input| Input::new(input.name, input.source))
    .collect();
    let include_args: Vec<_> = include_dirs
        .into_iter()
        .flat_map(|dir| ["-isystem".into(), dir])
        .collect();
    let arguments = clang_arguments(
        &Arch::known("x64").unwrap(),
        &include_args,
        None,
        "crates/libs/clang2/src/sal.h",
    );
    for input in inputs.iter().chain(&definitions) {
        std::fs::write(output.join(&input.name), &input.source)?;
    }
    std::fs::write(output.join("arguments.txt"), arguments.join("\n"))?;
    let inventory = windows_clang2::discover(
        inputs.clone(),
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        &files.iter().map(String::as_str).collect::<Vec<_>>(),
    )?;
    let discovery_time = time.elapsed();
    let mut roots = BTreeSet::new();
    for declaration in &inventory {
        if exclusion(declaration).is_none() {
            roots.insert(declaration.name.as_str());
        }
    }
    let roots: Vec<_> = roots.into_iter().collect();
    let mut outcomes: BTreeMap<_, _> = roots
        .iter()
        .map(|name| (*name, ("blocked", "capture has not completed".to_string())))
        .collect();
    report(output, &inventory, &outcomes);
    if let Some(root) = roots
        .iter()
        .find(|root| options.references.contains_key(**root))
    {
        let error =
            format!("selected header declaration `{root}` has an external metadata binding");
        blocked(output, &inventory, &mut outcomes, "selection", &error);
        return Err(error.into());
    }
    let mut capture_inputs = inputs;
    capture_inputs.extend(definitions);
    let capture_start = std::time::Instant::now();
    let captured = windows_clang2::capture_report(
        capture_inputs,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        &roots,
    )
    .inspect_err(|error| {
        blocked(output, &inventory, &mut outcomes, "capture", error);
    })?;
    let capture_time = capture_start.elapsed();
    for (root, reason) in &captured.rejected {
        outcomes.insert(root.as_str(), ("rejected", format!("capture: {reason}")));
    }
    report(output, &inventory, &outcomes);
    let Some(snapshot) = &captured.snapshot else {
        return Err("all selected roots have failed macro probes".into());
    };
    let resolution_start = std::time::Instant::now();
    let assessment = snapshot.assess().inspect_err(|error| {
        blocked(output, &inventory, &mut outcomes, "resolution", error);
    })?;
    let resolution_time = resolution_start.elapsed();
    for (root, reason) in &assessment.rejected {
        outcomes.insert(
            root.as_str(),
            ("rejected", format!("native availability: {reason}")),
        );
    }
    report(output, &inventory, &outcomes);
    let Some(resolved) = &assessment.resolved else {
        return Err("all selected roots have unavailable native evidence".into());
    };
    let projection_start = std::time::Instant::now();
    let mut supported = vec![];
    let projection = resolved.projection(options).inspect_err(|error| {
        blocked(
            output,
            &inventory,
            &mut outcomes,
            "projection policy",
            error,
        );
    })?;
    for root in &roots {
        if outcomes[root].0 == "rejected" {
            continue;
        }
        let rejected = match projection.project_roots(&[root]) {
            Ok(plan) if plan.omitted().is_empty() => None,
            Ok(plan) => Some(format!("projection omissions: {:?}", plan.omitted())),
            Err(error) => Some(error.to_string()),
        };
        if let Some(reason) = rejected {
            eprintln!("clang2 {name}: rejected {root}: {reason}");
            outcomes.insert(root, ("rejected", format!("projection: {reason}")));
        } else {
            supported.push(*root);
            outcomes.insert(
                root,
                ("blocked", "combined projection has not completed".into()),
            );
        }
    }
    report(output, &inventory, &outcomes);
    if supported.is_empty() {
        return Err(format!(
            "no header roots can be projected; see {}",
            output.join("inventory.tsv").display()
        )
        .into());
    }
    let plan = projection.project_roots(&supported).inspect_err(|error| {
        blocked(
            output,
            &inventory,
            &mut outcomes,
            "combined projection",
            error,
        );
    })?;
    let projection_time = projection_start.elapsed();
    let output_start = std::time::Instant::now();
    assert!(
        plan.omitted().is_empty(),
        "omitted header roots: {:?}",
        plan.omitted()
    );
    for root in &supported {
        outcomes.insert(
            root,
            ("blocked", "projection output has not completed".into()),
        );
    }
    report(output, &inventory, &outcomes);
    let winmd = output.join("audio.winmd");
    let mut reader = windows_rdl::reader();
    let mut partitions = BTreeMap::new();
    let mut owners = String::from("output\theader\n");
    let headers = plan.rdl_by_header().inspect_err(|error| {
        blocked(output, &inventory, &mut outcomes, "source ownership", error);
    })?;
    for (header, text) in headers {
        let name = std::path::Path::new(&header)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let name = format!("{name}.rdl");
        if let Some(previous) = partitions.insert(name.to_ascii_lowercase(), header.clone()) {
            let error = format!("source headers collide on output filename: {previous}, {header}");
            blocked(
                output,
                &inventory,
                &mut outcomes,
                "source ownership",
                &error,
            );
            return Err(error.into());
        }
        let file = rdl.join(&name);
        std::fs::write(&file, text)?;
        reader.input(file);
        owners.push_str(&format!("{name}\t{header}\n"));
    }
    std::fs::write(output.join("headers.tsv"), owners)?;
    compile_metadata(reader, &winmd, options, rdl_only).inspect_err(|error| {
        blocked(output, &inventory, &mut outcomes, "metadata", error);
    })?;
    if !rdl_only {
        windows_bindgen::bindgen([
            "--in",
            "default",
            winmd.to_str().unwrap(),
            "--out",
            output.join("src/bindings.rs").to_str().unwrap(),
            "--flat",
            "--minimal",
            "--filter",
            "Win32Audio",
            "Windows.Win32.PROPVARIANT",
            "Windows.Win32.IPropertyStore",
            "Windows.Win32.PROPERTYKEY",
            "Windows.Win32.PropVariantClear",
            "Windows.Win32.VT_UI4",
            "Windows.Win32.CoCreateInstance",
            "Windows.Win32.CoTaskMemFree",
            "Windows.Win32.StringFromIID",
            "Windows.Win32.E_FAIL",
        ]);
        std::fs::write(
            output.join("Cargo.toml"),
            "[workspace]\n[package]\nname = \"clang2-audio-smoke\"\nversion = \"0.0.0\"\n\
         edition = \"2024\"\n[dependencies]\n\
         windows-core = { path = \"../../../crates/libs/core\" }\n",
        )
        .unwrap();
        std::fs::write(output.join("src/main.rs"), include_str!("audio_smoke.rs")).unwrap();
    }
    for root in &supported {
        outcomes.insert(root, ("emitted", String::new()));
    }
    report(output, &inventory, &outcomes);
    println!(
        "clang2 {name}: stages discovery {:.2}s, capture {:.2}s ({} parses), \
         resolution {:.2}s, projection {:.2}s, output {:.2}s; {} type pairs, \
         {} availability edges, {} unavailable groups",
        discovery_time.as_secs_f32(),
        capture_time.as_secs_f32(),
        captured.parses,
        resolution_time.as_secs_f32(),
        projection_time.as_secs_f32(),
        output_start.elapsed().as_secs_f32(),
        resolved.report().type_pairs,
        assessment.dependency_edges,
        assessment.unavailable_groups,
    );
    println!(
        "clang2 {name}: {} discovered rows, {} selected names, {} emitted, {} rejected; \
         {} groups, {} observations, {} declaration pairs; generated in {:.2}s at {}",
        inventory.len(),
        roots.len(),
        supported.len(),
        roots.len() - supported.len(),
        resolved.group_count(),
        resolved.report().observations,
        resolved.report().declaration_pairs,
        time.elapsed().as_secs_f32(),
        output.display()
    );
    if supported.len() != roots.len() {
        return Err(format!(
            "header coverage is incomplete; see {}",
            output.join("inventory.tsv").display()
        )
        .into());
    }
    Ok(())
}

fn compile_metadata(
    mut reader: windows_rdl::Reader,
    output: &std::path::Path,
    options: &ProjectionOptions,
    rdl_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    reader
        .input_text(include_str!("../../../../metadata/metadata.rdl"))
        .input_text(include_str!("../../../libs/clang2/metadata.rdl"));
    if !options.references.is_empty() || !options.string_references.is_empty() {
        reader.reference_default();
    }
    reader.output(output).write()?;
    if rdl_only {
        std::fs::remove_file(output)?;
    }
    Ok(())
}

fn clear_outputs(output: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(output.join("rdl"))?;
    for file in [
        "audio.rdl",
        "audio.winmd",
        "src/bindings.rs",
        "inventory.tsv",
        "headers.tsv",
    ] {
        let file = output.join(file);
        if file.exists() {
            std::fs::remove_file(file)?;
        }
    }
    for entry in std::fs::read_dir(output.join("rdl"))? {
        let entry = entry?;
        if entry.file_type()?.is_file()
            && entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "rdl")
        {
            std::fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}

fn exclusion(declaration: &DeclarationInfo) -> Option<&'static str> {
    if declaration.record_member {
        Some("record-member declaration captured through its owner")
    } else if declaration.macro_alias.is_some() {
        Some("preprocessing alias of a native type or function")
    } else if declaration.macro_attribute {
        Some("declaration-attribute preprocessing helper")
    } else if declaration.macro_declaration {
        Some("declaration-fragment preprocessing helper")
    } else if declaration.function_macro {
        Some("function-like preprocessing helper")
    } else if declaration.empty_macro {
        Some("empty preprocessing marker")
    } else if declaration.inline {
        Some("inline helper without an exported entry point")
    } else {
        None
    }
}

fn report(
    output: &std::path::Path,
    inventory: &[DeclarationInfo],
    outcomes: &BTreeMap<&str, (&str, String)>,
) {
    let mut text = String::from("header\tline\tkind\tname\tstatus\treason\n");
    for declaration in inventory {
        let (status, reason) = if let Some(reason) = exclusion(declaration) {
            ("excluded", reason)
        } else {
            let (status, reason) = &outcomes[declaration.name.as_str()];
            (*status, reason.as_str())
        };
        let reason = if let Some(target) = &declaration.macro_alias {
            format!("{reason}: {target}")
        } else {
            reason.into()
        };
        text.push_str(&format!(
            "{}\t{}\t{}\t{}\t{status}\t{}\n",
            declaration.header,
            declaration.line,
            declaration.kind,
            declaration.name,
            reason.replace(['\t', '\r', '\n'], " ")
        ));
    }
    std::fs::write(output.join("inventory.tsv"), text).unwrap();
}

fn blocked(
    output: &std::path::Path,
    inventory: &[DeclarationInfo],
    outcomes: &mut BTreeMap<&str, (&str, String)>,
    stage: &str,
    error: &dyn std::fmt::Display,
) {
    for (status, reason) in outcomes.values_mut() {
        if *status == "blocked" {
            *reason = format!("{stage}: {error}");
        }
    }
    report(output, inventory, outcomes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_metadata_gate_preserves_rdl_only_output() {
        let directory =
            std::env::temp_dir().join(format!("clang2-metadata-gate-{}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        let output = directory.join("test.winmd");
        let reader = || {
            let mut reader = windows_rdl::reader();
            reader.input_text(
                "#[win32] mod Test {
                    struct Packet { count: i32, }
                    #[library(\"test.dll\")]
                    extern \"C\" fn Use(#[in] #[len_param(1)] data: *const i32, count: u32);
                }",
            );
            reader
        };
        compile_metadata(reader(), &output, &header_options(), true).unwrap();
        assert!(!output.exists());
        compile_metadata(reader(), &output, &header_options(), false).unwrap();
        let index = windows_metadata::reader::Index::read(&output).unwrap();
        assert_eq!(
            index.expect("Test", "Packet").fields().next().unwrap().ty(),
            windows_metadata::Type::I32
        );
        let windows_metadata::reader::Item::Fn(function) = index.expect_item("Test", "Use") else {
            panic!()
        };
        assert_eq!(
            function.params().next().unwrap().buffer_relationship(),
            Some(windows_metadata::reader::BufferRelationship::ElementsParam(
                1
            ))
        );
        std::fs::remove_file(output).unwrap();
        std::fs::remove_dir(directory).unwrap();
    }

    #[test]
    fn generic_metadata_gate_cannot_resolve_missing_types_from_default_metadata() {
        let directory =
            std::env::temp_dir().join(format!("clang2-metadata-references-{}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        let output = directory.join("test.winmd");
        let reader = || {
            let mut reader = windows_rdl::reader();
            reader.input_text(
                "#[win32] mod Test { struct Packet { point: Windows::Foundation::Point, } }",
            );
            reader
        };
        let mut options = header_options();
        let error = compile_metadata(reader(), &output, &options, true).unwrap_err();
        assert!(error.to_string().contains("type not found"), "{error}");
        assert!(!output.exists());
        options.references.insert(
            "NativePoint".into(),
            TypeReference {
                namespace: "Windows.Foundation".into(),
                name: "Point".into(),
                kind: ReferenceKind::Value,
            },
        );
        compile_metadata(reader(), &output, &options, true).unwrap();
        assert!(!output.exists());
        std::fs::remove_dir(directory).unwrap();
    }

    #[test]
    fn exact_imports_preserve_precedence_targets_and_provenance() {
        use windows_rdl::implib::{Import, ImportKind, ImportTarget};
        let candidate = Import {
            symbol: "_Native@4".into(),
            dll: "first.dll".into(),
            target: ImportTarget::Name("Export".into()),
            kind: ImportKind::Code,
        };
        let mut options = header_options();
        let mut provenance = String::new();
        append_imports(
            &mut options,
            "first.lib",
            vec![candidate.clone(), candidate.clone()],
            &mut provenance,
        )
        .unwrap();
        let mut later = candidate.clone();
        later.dll = "later.dll".into();
        later.target = ImportTarget::Ordinal(7);
        let mut ordinal = later.clone();
        ordinal.symbol = "Ordinal".into();
        let mut data = candidate;
        data.symbol = "Data".into();
        data.kind = ImportKind::Data;
        append_imports(
            &mut options,
            "later.lib",
            vec![later, ordinal, data],
            &mut provenance,
        )
        .unwrap();
        assert_eq!(
            options.imports["_Native@4"],
            FunctionImport {
                library: "first.dll".into(),
                target: windows_clang2::ImportTarget::Name("Export".into()),
            }
        );
        assert_eq!(
            options.imports["Ordinal"].target,
            windows_clang2::ImportTarget::Ordinal(7)
        );
        assert!(!options.imports.contains_key("Native"));
        assert!(!options.imports.contains_key("Data"));
        assert_eq!(
            provenance,
            "first.lib\t_Native@4\tfirst.dll\tExport\ttrue\n\
             first.lib\t_Native@4\tfirst.dll\tExport\tfalse\n\
             later.lib\t_Native@4\tlater.dll\t#7\tfalse\n\
             later.lib\tOrdinal\tlater.dll\t#7\ttrue\n"
        );
    }

    #[test]
    fn conflicting_imports_reject_even_in_a_shadowed_archive() {
        use windows_rdl::implib::{Import, ImportKind, ImportTarget};
        let candidate = Import {
            symbol: "Native".into(),
            dll: "first.dll".into(),
            target: ImportTarget::Name("Native".into()),
            kind: ImportKind::Code,
        };
        let mut options = header_options();
        let mut provenance = String::new();
        append_imports(
            &mut options,
            "first.lib",
            vec![candidate.clone()],
            &mut provenance,
        )
        .unwrap();
        let mut conflicting = candidate.clone();
        conflicting.dll = "conflict.dll".into();
        let error = append_imports(
            &mut options,
            "later.lib",
            vec![candidate, conflicting],
            &mut provenance,
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("conflicting DLL imports within `later.lib`"),
            "{error}"
        );
    }

    #[test]
    fn header_selection_is_explicit_and_bounded() {
        assert!(validate_headers(&["shellscalingapi.h", "tlhelp32.h"]).is_ok());
        for headers in [
            &[][..],
            &[""],
            &["missing.h"],
            &["shellscalingapi.h", "shellscalingapi.h"],
        ] {
            assert!(validate_headers(headers).is_err());
        }
        let options = header_options();
        assert!(options.references.is_empty());
        assert!(options.string_references.is_empty());
        assert!(options.library.is_none());
    }

    #[test]
    fn output_cleanup_removes_stale_results_before_capture() {
        let output = std::env::temp_dir().join(format!("clang2-cleanup-{}", std::process::id()));
        std::fs::create_dir_all(output.join("rdl")).unwrap();
        std::fs::write(output.join("rdl/previous.rdl"), "stale").unwrap();
        std::fs::write(output.join("rdl/keep.txt"), "keep").unwrap();
        std::fs::write(output.join("inventory.tsv"), "stale").unwrap();
        clear_outputs(&output).unwrap();
        assert!(!output.join("rdl/previous.rdl").exists());
        assert!(!output.join("inventory.tsv").exists());
        assert_eq!(
            std::fs::read_to_string(output.join("rdl/keep.txt")).unwrap(),
            "keep"
        );
        std::fs::remove_file(output.join("rdl/keep.txt")).unwrap();
        std::fs::remove_dir(output.join("rdl")).unwrap();
        std::fs::remove_dir(output).unwrap();
    }

    #[test]
    fn inventory_reports_rejections_and_exclusions_without_hiding_data() {
        let declaration = DeclarationInfo {
            header: "header.h".into(),
            line: 1,
            name: "Data".into(),
            kind: "VarDecl".into(),
            definition: false,
            inline: false,
            initializer: false,
            function_macro: false,
            empty_macro: false,
            record_member: false,
            macro_alias: None,
            macro_attribute: false,
            macro_declaration: false,
        };
        assert!(exclusion(&declaration).is_none());
        let mut inventory = vec![declaration.clone()];
        for (name, kind) in [
            ("Marker", "empty"),
            ("Helper", "function"),
            ("Inline", "inline"),
        ] {
            let mut item = declaration.clone();
            item.name = name.into();
            item.empty_macro = kind == "empty";
            item.function_macro = kind == "function";
            item.inline = kind == "inline";
            assert!(exclusion(&item).is_some());
            inventory.push(item);
        }
        let mut reserved = declaration.clone();
        reserved.kind = "macro definition".into();
        reserved.name = "__CONFIG".into();
        assert!(exclusion(&reserved).is_none());
        inventory.push(reserved);
        let mut member = declaration.clone();
        member.record_member = true;
        assert_eq!(
            exclusion(&member),
            Some("record-member declaration captured through its owner")
        );
        let mut alias = declaration.clone();
        alias.kind = "macro definition".into();
        alias.name = "Alias".into();
        alias.macro_alias = Some("NativeFunction".into());
        let mut attribute = declaration.clone();
        attribute.kind = "macro definition".into();
        attribute.name = "Attribute".into();
        attribute.macro_attribute = true;
        let mut fragment = declaration.clone();
        fragment.kind = "macro definition".into();
        fragment.name = "Fragment".into();
        fragment.macro_declaration = true;
        let mut rejected = declaration;
        rejected.name = "Unsupported".into();
        inventory.push(rejected);
        inventory.push(alias);
        inventory.push(attribute);
        inventory.push(fragment);
        let mut outcomes = BTreeMap::from([
            ("Data", ("blocked", String::new())),
            ("__CONFIG", ("blocked", String::new())),
            (
                "Unsupported",
                ("rejected", "projection: missing\ninitializer".into()),
            ),
        ]);
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output = std::env::temp_dir().join(format!(
            "clang2-audio-report-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir(&output).unwrap();
        blocked(
            &output,
            &inventory,
            &mut outcomes,
            "resolution",
            &"native disagreement",
        );
        let text = std::fs::read_to_string(output.join("inventory.tsv")).unwrap();
        let rows: Vec<_> = text
            .lines()
            .skip(1)
            .map(|line| line.split('\t').collect::<Vec<_>>())
            .collect();
        assert_eq!(rows.len(), inventory.len());
        assert!(rows.iter().all(|row| row.len() == 6));
        assert_eq!(
            &rows[0][4..],
            ["blocked", "resolution: native disagreement"]
        );
        assert!(rows[1..4].iter().all(|row| row[4] == "excluded"));
        assert_eq!(
            &rows[4][4..],
            ["blocked", "resolution: native disagreement"]
        );
        assert_eq!(
            &rows[5][4..],
            ["rejected", "projection: missing initializer"]
        );
        assert_eq!(
            &rows[6][4..],
            [
                "excluded",
                "preprocessing alias of a native type or function: NativeFunction"
            ]
        );
        assert_eq!(
            &rows[7][4..],
            ["excluded", "declaration-attribute preprocessing helper"]
        );
        assert_eq!(
            &rows[8][4..],
            ["excluded", "declaration-fragment preprocessing helper"]
        );
        std::fs::remove_file(output.join("inventory.tsv")).unwrap();
        std::fs::remove_dir(output).unwrap();
    }
}
