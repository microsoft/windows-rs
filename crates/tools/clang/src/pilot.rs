use crate::inventory::{Result, escape, git, pin};
use crate::sources::{native, profiles};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use windows_clang2::{Compiler, Import, Input, Target};

const MANIFEST: &str = include_str!("pilot.tsv");
const SDK: &str = "crates\\tools\\win32\\src\\main.rs";
const MSVC_COMPATIBILITY: &str = "19.44.35228";
const HEADERS: &[&str] = &[
    "tlhelp32.h",
    "shellscalingapi.h",
    "unknwnbase.h",
    "objidlbase.h",
];

#[derive(Debug)]
struct Required<'a> {
    header: &'a str,
    kind: &'a str,
    name: &'a str,
    disposition: &'a str,
}

fn manifest(text: &str) -> Result<Vec<Required<'_>>> {
    let mut lines = text.lines();
    if lines.next() != Some("header\tkind\tname\tdisposition") {
        return Err("invalid pilot manifest header".into());
    }
    let mut keys = BTreeSet::new();
    let mut result = Vec::new();
    for line in lines {
        let columns = line.split('\t').collect::<Vec<_>>();
        let [header, kind, name, disposition] = columns.as_slice() else {
            return Err(format!("invalid pilot manifest row `{line}`").into());
        };
        if !HEADERS.contains(header)
            || name.is_empty()
            || !matches!(
                *disposition,
                "required" | "unicode-alias" | "header-guard" | "inline-helper" | "owned-member"
            )
            || !keys.insert((*header, *kind, *name))
        {
            return Err(format!("invalid or duplicate pilot root `{line}`").into());
        }
        result.push(Required {
            header,
            kind,
            name,
            disposition,
        });
    }
    if result.is_empty() {
        return Err("empty pilot manifest".into());
    }
    Ok(result)
}

pub fn run(mut arguments: impl Iterator<Item = String>) -> Result<()> {
    if let Some(option) = arguments.next() {
        if option != "--worker" {
            return Err("usage: tool-clang pilot".into());
        }
        let architecture = arguments
            .next()
            .ok_or("missing pilot worker architecture")?;
        let output = PathBuf::from(arguments.next().ok_or("missing pilot worker output")?);
        if arguments.next().is_some() {
            return Err("unexpected pilot worker argument".into());
        }
        return worker(&architecture, &output);
    }
    manifest(MANIFEST)?;
    let executable = std::env::current_exe()?;
    let provisioning = Command::new(&executable).arg("native-layout").output()?;
    if !provisioning.status.success() {
        return Err(format!(
            "pilot compiler provisioning failed:\n{}\n{}",
            String::from_utf8_lossy(&provisioning.stdout),
            String::from_utf8_lossy(&provisioning.stderr)
        )
        .into());
    }
    let provisioning = String::from_utf8(provisioning.stdout)?;
    let parser = PathBuf::from(
        provisioning
            .lines()
            .last()
            .ok_or("native-layout did not report its parser directory")?,
    );
    // Provision before spawning workers, which only inspect cached inputs.
    profiles::build(&profiles::Selection {
        family: "sdk".into(),
        arch: "all".into(),
        headers: Some(vec!["tlhelp32.h".into(), "shellscalingapi.h".into()]),
    })?;
    let sdk_version = pin(SDK, "SDK_VERSION")?;
    for architecture in ["x86", "x64", "arm64"] {
        helpers::nuget_package(
            &format!("microsoft.windows.sdk.cpp.{architecture}"),
            &sdk_version,
        );
    }
    let run = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let output = std::path::absolute(Path::new("target\\clang-pilot").join(run.to_string()))?;
    std::fs::create_dir_all(&output)?;
    std::fs::write(output.join("scope.tsv"), MANIFEST)?;
    let mut provenance = String::from("key\tvalue\n");
    for (key, value) in [
        ("status", "diagnostic-baseline".into()),
        ("proof_start", "2026-10-12".into()),
        ("decision_due", "2026-10-23".into()),
        ("revision", git(&["rev-parse", "--verify", "HEAD"])?),
        ("working_tree", git(&["status", "--short"])?),
        ("sdk", pin(SDK, "SDK_VERSION")?),
        ("msvc_compatibility", MSVC_COMPATIBILITY.into()),
        ("compiler_directory", parser.to_string_lossy().into_owned()),
        ("promotion", "disabled".into()),
    ] {
        writeln!(provenance, "{key}\t{}", escape(&value))?;
    }
    std::fs::write(output.join("run.tsv"), provenance)?;
    let mut statuses = String::from("architecture\tstatus\n");
    let mut failed = Vec::new();
    for architecture in ["x86", "x64", "arm64"] {
        let directory = output.join(architecture);
        std::fs::create_dir(&directory)?;
        let stdout = std::fs::File::create(directory.join("stdout.log"))?;
        let stderr = std::fs::File::create(directory.join("stderr.log"))?;
        let mut child = Command::new(&executable)
            .args(["pilot", "--worker", architecture])
            .arg(&directory)
            .env("LIBCLANG_PATH", &parser)
            .env(
                "WINDOWS_CLANG2_LAYOUT_NATIVE",
                parser.join("windows_clang_layout.dll"),
            )
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()?;
        let start = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait()? {
                break status.to_string();
            }
            if start.elapsed() > Duration::from_secs(60) {
                child.kill()?;
                child.wait()?;
                break "deadline exceeded".into();
            }
            std::thread::sleep(Duration::from_millis(50));
        };
        writeln!(statuses, "{architecture}\t{}", escape(&status))?;
        std::fs::write(output.join("workers.tsv"), &statuses)?;
        if !directory.join("gates.tsv").is_file() || !directory.join("capture.tsv").is_file() {
            failed.push(architecture);
        }
    }
    println!("Pilot reports: {}", output.display());
    if !failed.is_empty() {
        return Err(format!(
            "pilot baseline failed for {}; see worker stderr.log files",
            failed.join(", ")
        )
        .into());
    }
    Err("pilot is incomplete: 0/2 accepted partitions; see gates.tsv and capture.tsv".into())
}

fn build_profiles(architecture: &str) -> Result<Vec<profiles::Profile>> {
    let base = profiles::build(&profiles::Selection {
        family: "sdk".into(),
        arch: architecture.into(),
        headers: Some(vec!["tlhelp32.h".into(), "shellscalingapi.h".into()]),
    })?
    .pop()
    .ok_or("missing pilot SDK profile")?;
    let sdk_um = base.roots[0].parent().ok_or("SDK root has no directory")?;
    let roots = HEADERS
        .iter()
        .map(|header| {
            let path = sdk_um.join(header);
            if !path.is_file() {
                return Err(format!("missing pilot header `{}`", path.display()).into());
            }
            Ok(path)
        })
        .collect::<Result<Vec<_>>>()?;
    let mut result = Vec::new();
    for unicode in [false, true] {
        for (label, selected) in [
            ("tlhelp32", vec![0]),
            ("shellscalingapi", vec![1]),
            ("com", vec![2, 3]),
            ("combined", vec![0, 1, 2, 3]),
            ("reversed", vec![3, 2, 1, 0]),
        ] {
            let mut source = pin(SDK, "PRELUDE")?;
            for index in &selected {
                writeln!(source, "\n#include <{}>", HEADERS[*index])?;
            }
            let mut arguments = base.arguments.clone();
            arguments.push(format!("-fms-compatibility-version={MSVC_COMPATIBILITY}"));
            if unicode {
                arguments.extend(["-DUNICODE".into(), "-D_UNICODE".into()]);
            }
            result.push(profiles::Profile {
                name: format!("{label}-{}", if unicode { "unicode" } else { "ansi" }),
                family: "sdk",
                architecture: base.architecture,
                source,
                arguments,
                roots: selected
                    .into_iter()
                    .map(|index| roots[index].clone())
                    .collect(),
                root_dirs: Vec::new(),
                excluded: Vec::new(),
            });
        }
    }
    Ok(result)
}

fn root_header(declaration: &native::Declaration) -> Result<&str> {
    declaration
        .header
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "invalid compiler root filename".into())
}

fn declaration_context(declaration: &native::Declaration) -> bool {
    declaration.kind == "LinkageSpec"
        || (declaration.kind == "UnexposedDecl" && declaration.name.is_empty())
}

fn check_scope(
    profile: &profiles::Profile,
    captured: &native::Capture,
    scope: &[Required<'_>],
) -> Result<BTreeSet<String>> {
    if !captured.complete {
        return Err(format!("{}: source diagnostics prevent discovery", profile.name).into());
    }
    let unicode = profile
        .arguments
        .iter()
        .any(|argument| argument == "-DUNICODE");
    let headers = profile
        .roots
        .iter()
        .filter_map(|path| path.file_name().and_then(|name| name.to_str()))
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    let mut required = BTreeSet::new();
    for declaration in captured
        .declarations
        .iter()
        .filter(|decl| decl.scope == "root")
    {
        let header = root_header(declaration)?;
        let matched = scope.iter().find(|entry| {
            (entry.header, entry.kind, entry.name)
                == (
                    header,
                    declaration.kind.as_str(),
                    declaration.qualified.as_str(),
                )
        });
        if let Some(entry) = matched {
            if entry.disposition == "inline-helper" && !declaration.inline {
                return Err(
                    format!("{}: helper `{}` is not inline", profile.name, entry.name).into(),
                );
            }
            if entry.disposition == "unicode-alias" && !unicode {
                return Err(format!(
                    "{}: unexpected Unicode alias `{}`",
                    profile.name, entry.name
                )
                .into());
            }
            seen.insert((entry.header, entry.kind, entry.name));
            if matches!(entry.disposition, "required" | "unicode-alias") {
                required.insert(entry.name.to_string());
            }
        } else if matches!(header, "tlhelp32.h" | "shellscalingapi.h")
            && !declaration_context(declaration)
            && !matches!(
                declaration.kind.as_str(),
                "FieldDecl" | "ParmDecl" | "EnumConstantDecl"
            )
        {
            return Err(format!(
                "{}: unclassified root {} `{}` in {header}",
                profile.name, declaration.kind, declaration.name
            )
            .into());
        }
    }
    for entry in scope.iter().filter(|entry| headers.contains(entry.header)) {
        if entry.disposition == "unicode-alias" && !unicode {
            continue;
        }
        if !seen.contains(&(entry.header, entry.kind, entry.name)) {
            return Err(format!(
                "{}: missing scoped {} `{}` in {}",
                profile.name, entry.kind, entry.name, entry.header
            )
            .into());
        }
    }
    Ok(required)
}

fn worker(architecture: &str, output: &Path) -> Result<()> {
    let target = match architecture {
        "x86" => Target::X86,
        "x64" => Target::X64,
        "arm64" => Target::Arm64,
        _ => return Err("invalid pilot worker architecture".into()),
    };
    let bridge = std::env::var_os("WINDOWS_CLANG2_LAYOUT_NATIVE")
        .ok_or("pilot requires the matched compiler bridge")?;
    let compiler = Compiler::with_layout_bridge(bridge)?;
    let scope = manifest(MANIFEST)?;
    let profiles = build_profiles(architecture)?;
    let imports = imports(architecture, output)?;
    let vocabulary = output.join("vocabulary.winmd");
    windows_rdl::reader()
        .input("metadata\\metadata.rdl")
        .output(&vocabulary)
        .write()?;
    let mut discovery = String::from("profile\theader\tkind\tname\tline\tdisposition\n");
    let mut captures = String::from("profile\tstatus\tdiagnostic\n");
    let mut inputs = String::from("profile\trole\tpath\n");
    let mut compiler_facts = String::from("profile\tcompiler\ttarget\tpointer_bits\n");
    let mut snapshots = BTreeMap::new();
    let mut counts = BTreeMap::<_, usize>::new();
    for entry in &scope {
        if entry.disposition == "required" {
            *counts.entry((entry.header, entry.kind)).or_default() += 1;
        }
    }
    let mut families = String::from("header\tkind\trequired_roots\n");
    for ((header, kind), count) in counts {
        writeln!(families, "{header}\t{kind}\t{count}")?;
    }
    std::fs::write(output.join("families.tsv"), families)?;
    for profile in &profiles {
        let path = output.join(format!("{}.hpp", profile.name));
        for root in &profile.roots {
            writeln!(
                inputs,
                "{}\troot\t{}",
                profile.name,
                escape(&root.to_string_lossy())
            )?;
        }
        std::fs::write(output.join("inputs.tsv"), &inputs)?;
        std::fs::write(&path, &profile.source)?;
        std::fs::write(
            output.join(format!("{}.args.tsv", profile.name)),
            format!(
                "position\targument\n{}",
                profile
                    .arguments
                    .iter()
                    .enumerate()
                    .map(|(index, argument)| format!("{index}\t{}\n", escape(argument)))
                    .collect::<String>()
            ),
        )?;
        let captured = native::capture(profile, &path)?;
        writeln!(
            compiler_facts,
            "{}\t{}\t{}\t{}",
            profile.name,
            escape(&native::text(unsafe { clang_sys::clang_getClangVersion() })),
            escape(&captured.target),
            captured.pointer_bits
        )?;
        std::fs::write(output.join("compiler.tsv"), &compiler_facts)?;
        let mut diagnostics = String::from("severity\tdiagnostic\n");
        for (severity, diagnostic) in &captured.diagnostics {
            writeln!(diagnostics, "{severity}\t{}", escape(diagnostic))?;
        }
        std::fs::write(
            output.join(format!("{}.diagnostics.tsv", profile.name)),
            diagnostics,
        )?;
        for declaration in captured
            .declarations
            .iter()
            .filter(|decl| decl.scope == "root")
        {
            let header = root_header(declaration)?;
            let disposition = scope
                .iter()
                .find(|entry| {
                    (entry.header, entry.kind, entry.name)
                        == (
                            header,
                            declaration.kind.as_str(),
                            declaration.qualified.as_str(),
                        )
                })
                .map_or(
                    if matches!(header, "unknwnbase.h" | "objidlbase.h") {
                        "outside-selected-com-closure"
                    } else if declaration_context(declaration) {
                        "declaration-context"
                    } else if matches!(
                        declaration.kind.as_str(),
                        "FieldDecl" | "ParmDecl" | "EnumConstantDecl"
                    ) {
                        "member"
                    } else {
                        "unclassified"
                    },
                    |entry| entry.disposition,
                );
            writeln!(
                discovery,
                "{}\t{}\t{}\t{}\t{}\t{disposition}",
                profile.name,
                escape(header),
                escape(&declaration.kind),
                escape(&declaration.qualified),
                declaration.line
            )?;
        }
        std::fs::write(output.join("discovery.tsv"), &discovery)?;
        let required = check_scope(profile, &captured, &scope)?;
        let mut input = Input::new(path.to_string_lossy(), &profile.source, target);
        input.arguments.clone_from(&profile.arguments);
        input.roots.clone_from(&profile.roots);
        input.required = required;
        let emitted = (|| -> Result<()> {
            let snapshot = compiler.capture(&input)?;
            if let Some(probe) = &snapshot.macro_probe {
                std::fs::write(
                    output.join(format!("{}.macros.hpp", profile.name)),
                    &probe.source,
                )?;
                std::fs::write(
                    output.join(format!("{}.macros.args.tsv", profile.name)),
                    format!(
                        "position\targument\n{}",
                        probe
                            .arguments
                            .iter()
                            .enumerate()
                            .map(|(position, argument)| format!(
                                "{position}\t{}\n",
                                escape(argument)
                            ))
                            .collect::<String>()
                    ),
                )?;
            }
            let mut macro_report = String::from("name\tkind\tdetail\theader\tline\n");
            for evidence in snapshot.macros() {
                let (kind, detail) = match &evidence.kind {
                    windows_clang2::MacroKind::Integer {
                        written_type,
                        value,
                        ..
                    } => ("integer", format!("{written_type}: {value:?}")),
                    windows_clang2::MacroKind::Alias {
                        qualified_name,
                        kind,
                        ..
                    } => ("alias", format!("{kind}: {qualified_name}")),
                };
                writeln!(
                    macro_report,
                    "{}\t{kind}\t{}\t{}\t{}",
                    escape(&evidence.name),
                    escape(&detail),
                    escape(&evidence.expansion.file),
                    evidence.expansion.line
                )?;
            }
            std::fs::write(
                output.join(format!("{}.macros.tsv", profile.name)),
                macro_report,
            )?;
            let mut interface_report = String::from("interface\tguid\tsize\tinherited\tpacking\n");
            for (declaration, interface) in snapshot.interfaces() {
                writeln!(
                    interface_report,
                    "{}\t{}\t{}\t{}\t{:?}",
                    escape(&declaration.qualified_name),
                    interface.guid,
                    interface.vtable_size,
                    interface.inherited,
                    interface.packing
                )?;
            }
            std::fs::write(
                output.join(format!("{}.interfaces.tsv", profile.name)),
                interface_report,
            )?;
            let mut method_report = String::from("interface\tmethod\tslot\tconvention\n");
            for (declaration, method, function) in snapshot.methods() {
                writeln!(
                    method_report,
                    "{}\t{}\t{}\t{:?}",
                    escape(&declaration.qualified_name),
                    escape(&method.name),
                    method.slot,
                    function.convention
                )?;
            }
            std::fs::write(
                output.join(format!("{}.methods.tsv", profile.name)),
                method_report,
            )?;
            let mut helper_report = String::from("interface\tname\tkind\n");
            for (declaration, helper) in snapshot.interface_helpers() {
                writeln!(
                    helper_report,
                    "{}\t{}\t{}",
                    escape(&declaration.qualified_name),
                    escape(&helper.qualified_name),
                    escape(&helper.kind)
                )?;
            }
            std::fs::write(
                output.join(format!("{}.helpers.tsv", profile.name)),
                helper_report,
            )?;
            snapshots.insert(profile.name.clone(), snapshot.clone());
            let rdl = snapshot.rdl_with_imports("Windows.Win32", &imports)?;
            let binary = output.join(format!("{}.winmd", profile.name));
            windows_rdl::reader()
                .reference(&vocabulary)
                .input_text(&rdl)
                .output(&binary)
                .write()?;
            let index = windows_metadata::reader::Index::read(&binary)
                .ok_or_else(|| format!("cannot read candidate WinMD: {}", binary.display()))?;
            index.validate_native_contracts(None)?;
            index.validate_native_slots(None)?;
            std::fs::write(output.join(format!("{}.rdl", profile.name)), rdl)?;
            Ok(())
        })();
        match emitted {
            Ok(()) => {
                writeln!(captures, "{}\tcompiled\t", profile.name)?;
            }
            Err(error) => {
                writeln!(
                    captures,
                    "{}\trejected\t{}",
                    profile.name,
                    escape(&error.to_string())
                )?;
            }
        }
        std::fs::write(output.join("capture.tsv"), &captures)?;
    }
    let mut agreement = String::from("left\tright\tdeclarations\tmacros\tstatus\tdiagnostic\n");
    let mut agreement_passed = true;
    for encoding in ["ansi", "unicode"] {
        for (left, right) in [
            ("tlhelp32", "combined"),
            ("shellscalingapi", "combined"),
            ("com", "combined"),
            ("combined", "reversed"),
        ] {
            let left = format!("{left}-{encoding}");
            let right = format!("{right}-{encoding}");
            if let (Some(a), Some(b)) = (snapshots.get(&left), snapshots.get(&right)) {
                let result = a.agree(b).and_then(|()| {
                    if left.starts_with("combined-") {
                        b.agree(a)
                    } else {
                        Ok(())
                    }
                });
                let (status, diagnostic) = match result {
                    Ok(()) => ("passed", String::new()),
                    Err(error) => {
                        agreement_passed = false;
                        ("rejected", error.to_string())
                    }
                };
                writeln!(
                    agreement,
                    "{left}\t{right}\t{}\t{}\t{status}\t{}",
                    a.evidence().count(),
                    a.macros().count(),
                    escape(&diagnostic)
                )?;
            } else {
                agreement_passed = false;
                writeln!(
                    agreement,
                    "{left}\t{right}\tunknown\tunknown\tunavailable\tcapture unavailable"
                )?;
            }
        }
    }
    std::fs::write(output.join("agreement.tsv"), agreement)?;
    std::fs::write(
        output.join("gates.tsv"),
        format!(
            "gate\tstatus\nsource-scope\tpassed\ncapture\tsee-capture.tsv\n\
         semantic-winmd\tstructural-checks-for-compiled-profiles\nsource-contracts\tnot-implemented\n\
         native-imports\tnot-implemented\ncross-input-agreement\t{}\n\
         architecture-agreement\tnot-implemented\nsemantic-differences\tnot-classified\n\
         accepted-partitions\t0/2\npromotion\tdisabled\n",
            if agreement_passed {
                "passed"
            } else {
                "see-agreement.tsv"
            }
        ),
    )?;
    Err(
        "pilot worker is incomplete; reports are diagnostic artifacts, not accepted metadata"
            .into(),
    )
}

fn imports(architecture: &str, output: &Path) -> Result<BTreeMap<String, Import>> {
    use windows_rdl::implib::{ImportKind, ImportTarget};
    let machine = match architecture {
        "x86" => 0x14c,
        "x64" => 0x8664,
        "arm64" => 0xaa64,
        _ => return Err("invalid import architecture".into()),
    };
    let sdk_version = pin(SDK, "SDK_VERSION")?;
    let libraries = helpers::nuget_package(
        &format!("microsoft.windows.sdk.cpp.{architecture}"),
        &sdk_version,
    )
    .join("c\\um")
    .join(architecture);
    let mut result = BTreeMap::<String, Import>::new();
    let mut report =
        String::from("archive\tmachine\tkind\tsymbol\tdll\texport\tarchitecture_match\n");
    for name in ["kernel32.lib", "shcore.lib"] {
        let path = libraries.join(name);
        let bytes = std::fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        for contract in windows_rdl::implib::read_contracts(&bytes)
            .map_err(|error| format!("{}: {error}", path.display()))?
        {
            let export = match &contract.target {
                ImportTarget::Name(name) => name.clone(),
                ImportTarget::Ordinal(ordinal) => format!("#{ordinal}"),
            };
            writeln!(
                report,
                "{}\t{:04x}\t{:?}\t{}\t{}\t{}\t{}",
                escape(&path.to_string_lossy()),
                contract.machine,
                contract.kind,
                escape(&contract.symbol),
                escape(&contract.dll),
                escape(&export),
                contract.machine == machine,
            )?;
            if contract.machine != machine {
                continue;
            }
            if contract.kind == ImportKind::Code && matches!(contract.target, ImportTarget::Name(_))
            {
                let imported = Import {
                    dll: contract.dll,
                    export,
                };
                if let Some(previous) = result.get(&contract.symbol) {
                    if previous.dll != imported.dll || previous.export != imported.export {
                        return Err(format!(
                            "conflicting native import contracts for `{}`",
                            contract.symbol
                        )
                        .into());
                    }
                } else {
                    result.insert(contract.symbol, imported);
                }
            }
        }
    }
    std::fs::write(output.join("imports.tsv"), report)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_source_manifest() {
        let scope = manifest(MANIFEST).unwrap();
        let required = scope.iter().filter(|entry| entry.disposition == "required");
        let mut counts = BTreeMap::new();
        for entry in required {
            *counts.entry(entry.header).or_insert(0) += 1;
        }
        assert_eq!(counts["tlhelp32.h"], 57);
        assert_eq!(counts["shellscalingapi.h"], 20);
        assert_eq!(counts["unknwnbase.h"], 2);
        assert_eq!(counts["objidlbase.h"], 2);
        assert_eq!(
            scope
                .iter()
                .filter(|entry| entry.disposition == "unicode-alias")
                .count(),
            10
        );
        assert_eq!(
            scope
                .iter()
                .filter(|entry| entry.disposition == "owned-member")
                .count(),
            5
        );
        assert!(manifest("").is_err());
        assert!(
            manifest("header\tkind\tname\tdisposition\nother.h\tClassDecl\tX\trequired").is_err()
        );
        assert!(
            manifest(&format!(
                "{MANIFEST}tlhelp32.h\tStructDecl\ttagHEAPLIST32\trequired\n"
            ))
            .is_err()
        );
    }

    #[test]
    fn pilot_options_do_not_expand_scope() {
        assert!(run(["--headers", "other.h"].into_iter().map(str::to_string)).is_err());
        assert!(
            run(["--worker", "mips", "unused"]
                .into_iter()
                .map(str::to_string))
            .is_err()
        );
        assert!(run(["--worker"].into_iter().map(str::to_string)).is_err());
    }

    #[test]
    fn scope_requires_clean_complete_original_roots() {
        let profile = profiles::Profile {
            name: "control".into(),
            family: "sdk",
            architecture: 2,
            source: String::new(),
            arguments: Vec::new(),
            roots: vec![PathBuf::from("tlhelp32.h")],
            root_dirs: Vec::new(),
            excluded: Vec::new(),
        };
        let scope = manifest(
            "header\tkind\tname\tdisposition\ntlhelp32.h\tFunctionDecl\tRequired\trequired",
        )
        .unwrap();
        let mut capture = native::Capture {
            declarations: Vec::new(),
            files: Vec::new(),
            diagnostics: Vec::new(),
            complete: false,
            target: "x86_64-pc-windows-msvc".into(),
            pointer_bits: 64,
        };
        assert!(
            check_scope(&profile, &capture, &scope)
                .unwrap_err()
                .to_string()
                .contains("diagnostics")
        );
        capture.complete = true;
        assert!(
            check_scope(&profile, &capture, &scope)
                .unwrap_err()
                .to_string()
                .contains("missing scoped")
        );
        capture.declarations.push(native::Declaration {
            scope: "root",
            kind: "FunctionDecl".into(),
            name: "Required".into(),
            qualified: "Required".into(),
            usr: "required".into(),
            canonical_usr: "required".into(),
            parent_usr: String::new(),
            alias_usr: String::new(),
            header: PathBuf::from("tlhelp32.h"),
            line: 1,
            column: 1,
            offset: 0,
            spelling_header: "tlhelp32.h".into(),
            spelling_offset: 0,
            written_type: String::new(),
            canonical_type: String::new(),
            definition: false,
            inline: false,
            function_macro: false,
        });
        assert_eq!(
            check_scope(&profile, &capture, &scope).unwrap(),
            BTreeSet::from(["Required".into()])
        );
        capture.declarations[0].qualified = "Other::Required".into();
        assert!(
            check_scope(&profile, &capture, &scope)
                .unwrap_err()
                .to_string()
                .contains("unclassified root")
        );
        capture.declarations[0].qualified = "Required".into();
        capture.declarations[0].kind = "VarDecl".into();
        assert!(
            check_scope(&profile, &capture, &scope)
                .unwrap_err()
                .to_string()
                .contains("unclassified root")
        );
    }
}
