use super::*;
use std::collections::{BTreeMap, BTreeSet};
use windows_clang2::{
    DeclarationInfo, FunctionImport, Input, ProjectionOptions, ReferenceKind, TypeReference,
};

pub fn audio(rdl_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let time = std::time::Instant::now();
    let output = std::path::Path::new(if rdl_only {
        "target/win32-clang2/audio-rdl"
    } else {
        "target/win32-clang2/audio"
    });
    std::fs::create_dir_all(output)?;
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
    let include_dirs = sdk_include_dirs();
    let headers = ["mmdeviceapi.h", "endpointvolume.h"];
    let files: Vec<_> = headers
        .iter()
        .map(|header| resolve(header, &include_dirs, "header", "pinned SDK include"))
        .collect();
    let inputs: Vec<_> = clang_inputs(&headers, &include_dirs, false)
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
    let inventory = windows_clang2::discover(
        inputs.clone(),
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        &files.iter().map(String::as_str).collect::<Vec<_>>(),
    )?;
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
    let mut capture_inputs = inputs;
    // The SDK's definition mode supplies initializers without replacing declaration observations.
    capture_inputs.push(Input::new(
        "clang-win32-audio-values.hpp",
        format!("{PRELUDE}\n#include <initguid.h>\n#include <mmdeviceapi.h>"),
    ));
    let snapshot = windows_clang2::capture(
        capture_inputs,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        &roots,
    )
    .inspect_err(|error| {
        blocked(output, &inventory, &mut outcomes, "capture", error);
    })?;
    let resolved = snapshot.resolve().inspect_err(|error| {
        blocked(output, &inventory, &mut outcomes, "resolution", error);
    })?;
    // Keep candidate definitions distinct from the bundled metadata used for external references.
    let mut options = ProjectionOptions::new("Win32Audio");
    for (native, namespace, name, kind) in [
        (
            "HRESULT",
            "Windows.Foundation",
            "HResult",
            ReferenceKind::Value,
        ),
        ("BOOL", "Windows.Win32", "BOOL", ReferenceKind::Value),
        (
            "IUnknown",
            "Windows.Win32",
            "IUnknown",
            ReferenceKind::Interface,
        ),
        (
            "IPropertyStore",
            "Windows.Win32",
            "IPropertyStore",
            ReferenceKind::Interface,
        ),
        (
            "tagPROPVARIANT",
            "Windows.Win32",
            "PROPVARIANT",
            ReferenceKind::Value,
        ),
        ("LPWSTR", "Windows.Win32", "PWSTR", ReferenceKind::Value),
        ("LPCWSTR", "Windows.Win32", "PCWSTR", ReferenceKind::Value),
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

    let library = resolve(
        "mmdevapi.lib",
        &sdk_lib_dirs(),
        "library",
        "pinned SDK libraries",
    );
    for import in windows_rdl::implib::read(&std::fs::read(library)?)? {
        if import.kind != windows_rdl::implib::ImportKind::Code {
            continue;
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
        if let Some(previous) = options.imports.insert(import.symbol.clone(), value) {
            assert_eq!(previous, options.imports[&import.symbol]);
        }
    }

    let mut supported = vec![];
    let projection = resolved.projection(&options)?;
    for root in &roots {
        let rejected = match projection.project_roots(&[root]) {
            Ok(plan) if plan.omitted().is_empty() => None,
            Ok(plan) => Some(format!("projection omissions: {:?}", plan.omitted())),
            Err(error) => Some(error.to_string()),
        };
        if let Some(reason) = rejected {
            eprintln!("clang2 audio: rejected {root}: {reason}");
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
    let plan = projection.project_roots(&supported).inspect_err(|error| {
        blocked(
            output,
            &inventory,
            &mut outcomes,
            "combined projection",
            error,
        );
    })?;
    assert!(
        plan.omitted().is_empty(),
        "omitted audio roots: {:?}",
        plan.omitted()
    );
    for root in &supported {
        outcomes.insert(
            root,
            ("blocked", "projection output has not completed".into()),
        );
    }
    report(output, &inventory, &outcomes);
    let rdl = output.join("rdl");
    std::fs::create_dir_all(&rdl)?;
    for entry in std::fs::read_dir(&rdl)? {
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
    if !rdl_only {
        reader
            .reference_default()
            .output(&winmd)
            .write()
            .inspect_err(|error| {
                blocked(output, &inventory, &mut outcomes, "metadata", error);
            })?;
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
        "clang2 audio: {} discovered rows, {} selected names, {} emitted, {} rejected; \
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
            "audio header coverage is incomplete; see {}",
            output.join("inventory.tsv").display()
        )
        .into());
    }
    Ok(())
}

fn exclusion(declaration: &DeclarationInfo) -> Option<&'static str> {
    if declaration.function_macro {
        Some("function-like preprocessing helper")
    } else if declaration.empty_macro {
        Some("empty preprocessing marker")
    } else if declaration.kind == "macro definition" && declaration.name.starts_with("__") {
        Some("reserved preprocessing configuration")
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
        assert!(exclusion(&reserved).is_some());
        inventory.push(reserved);
        let mut rejected = declaration;
        rejected.name = "Unsupported".into();
        inventory.push(rejected);
        let mut outcomes = BTreeMap::from([
            ("Data", ("blocked", String::new())),
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
        assert!(rows[1..5].iter().all(|row| row[4] == "excluded"));
        assert_eq!(
            &rows[5][4..],
            ["rejected", "projection: missing initializer"]
        );
        std::fs::remove_file(output.join("inventory.tsv")).unwrap();
        std::fs::remove_dir(output).unwrap();
    }
}
