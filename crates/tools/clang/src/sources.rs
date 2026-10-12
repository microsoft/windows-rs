use crate::inventory::{Result, escape, git, pin, rdl_files};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::{BufWriter, Write as _};
use std::path::{Path, PathBuf};
use windows_rdl::InventoryEntry;

mod classify;
mod native;
mod profiles;
use native::{Capture, Declaration};
use profiles::{Profile, Selection};

pub fn run() -> Result<()> {
    let selection = Selection::parse(std::env::args().skip(2))?;
    let output = selection.output();
    std::fs::create_dir_all(&output)?;
    let run_path = output.join("run.tsv");
    std::fs::write(&run_path, "key\tvalue\nstatus\tpreparing\n")?;
    helpers::ensure_libclang();
    helpers::assert_libclang_version();
    let profiles = profiles::build(&selection)?;
    let baseline = baseline()?;
    let mut run = String::from("key\tvalue\nstatus\trunning\n");
    for (key, value) in [
        ("revision", git(&["rev-parse", "--verify", "HEAD"])?),
        (
            "object_format",
            git(&["rev-parse", "--show-object-format"])?,
        ),
        ("working_tree", git(&["status", "--short"])?),
        (
            "libclang",
            native::text(unsafe { clang_sys::clang_getClangVersion() }),
        ),
        ("family", selection.family.clone()),
        ("arch", selection.arch.clone()),
        (
            "headers",
            selection
                .headers
                .as_ref()
                .map(|headers| headers.join(","))
                .unwrap_or_default(),
        ),
        ("discovery", "declarations-and-macro-definitions".into()),
        ("macro_effective_state", "not-established".into()),
        ("semantic_agreement", "not-checked".into()),
        ("sal_instrumentation", "none".into()),
        ("toolset_policy", "compiler-discovered-not-pinned".into()),
        (
            "include_environment",
            std::env::var("INCLUDE").unwrap_or_default(),
        ),
        (
            "sdk",
            pin("crates\\tools\\win32\\src\\main.rs", "SDK_VERSION")?,
        ),
        (
            "wdk",
            pin("crates\\tools\\win32\\src\\km.rs", "WDK_VERSION")?,
        ),
        (
            "webview2",
            pin("crates\\tools\\webview\\src\\main.rs", "WEBVIEW2_VERSION")?,
        ),
        (
            "wdk_offreg_policy",
            "separate-sdk-prelude-no-synthetic-typedefs".into(),
        ),
    ] {
        writeln!(run, "{key}\t{}", escape(&value))?;
    }
    std::fs::write(&run_path, &run)?;
    let mut manifest = report(
        &output,
        "profiles.tsv",
        "profile\tfamily\tarchitecture\tstatus\troot_declarations\tall_declarations\ttarget\tpointer_bits\tseconds\tdiagnostic\n",
    )?;
    let mut roots = report(&output, "roots.tsv", "profile\trole\tpath\n")?;
    let mut entries = report(
        &output,
        "declarations.tsv",
        "profile\tid\tarchitecture\tscope\tkind\tname\tqualified\tusr\tcanonical_usr\tparent_usr\talias_usr\theader\tline\tcolumn\toffset\tspelling_header\tspelling_offset\twritten_type\tcanonical_type\tdefinition\tinline\tfunction_macro\tfamily_package\tdisposition\n",
    )?;
    let mut mappings = report(
        &output,
        "mappings.tsv",
        "profile\tpartition\tid\tkind\tname\tarchitecture\tstatus\tnative_id\n",
    )?;
    let mut diagnostics = report(
        &output,
        "diagnostics.tsv",
        "profile\tseverity\tdiagnostic\n",
    )?;
    let mut groups = report(
        &output,
        "groups.tsv",
        "profile\tpartition\tid\tkind\tname\tarchitecture\tmapping_status\tgroup\twork_package\tnative_ids\n",
    )?;
    let mut group_counts = BTreeMap::<_, usize>::new();
    let mut coverage =
        BTreeMap::<(PathBuf, usize, i32), (String, String, Vec<(String, &'static str)>)>::new();
    let mut inputs = report(&output, "inputs.tsv", "profile\tpath\tbytes\tgit_blob\n")?;
    let mut hashes = BTreeMap::new();
    fingerprint(
        "configuration",
        &[
            PathBuf::from("crates\\tools\\helpers\\src\\provision.rs"),
            PathBuf::from("crates\\tools\\win32\\src\\main.rs"),
            PathBuf::from("crates\\tools\\win32\\src\\km.rs"),
            PathBuf::from("crates\\tools\\webview\\src\\main.rs"),
            PathBuf::from("crates\\tools\\clang\\src\\sources.rs"),
            PathBuf::from("crates\\tools\\clang\\src\\inventory.rs"),
            PathBuf::from("crates\\tools\\clang\\src\\sources\\profiles.rs"),
            PathBuf::from("crates\\tools\\clang\\src\\sources\\native.rs"),
            PathBuf::from("crates\\tools\\clang\\src\\sources\\classify.rs"),
            PathBuf::from("Cargo.lock"),
        ],
        &mut hashes,
        &mut inputs,
    )?;
    if let Some(directory) = std::env::var_os("LIBCLANG_PATH") {
        let dll = Path::new(&directory).join("libclang.dll");
        if dll.is_file() {
            fingerprint("compiler", &[dll], &mut hashes, &mut inputs)?;
        }
    }
    let baseline_paths = baseline
        .iter()
        .map(|(path, _)| path.clone())
        .collect::<Vec<_>>();
    fingerprint("baseline", &baseline_paths, &mut hashes, &mut inputs)?;
    let mut failed = 0;
    let mut declarations = 0;
    for profile in &profiles {
        let input = output.join(format!("{}.hpp", profile.name));
        std::fs::write(&input, &profile.source)?;
        std::fs::write(
            output.join(format!("{}.args.tsv", profile.name)),
            format!(
                "position\targument\n{}",
                profile
                    .arguments
                    .iter()
                    .enumerate()
                    .map(|(index, value)| { format!("{index}\t{}\n", escape(value)) })
                    .collect::<String>()
            ),
        )?;
        for (role, paths) in [
            ("explicit", &profile.roots),
            ("directory", &profile.root_dirs),
            ("excluded", &profile.excluded),
        ] {
            for path in paths {
                row(&mut roots, &[&profile.name, role, &path.to_string_lossy()])?;
            }
        }
        roots.flush()?;
        println!("Discovering {}...", profile.name);
        let start = std::time::Instant::now();
        let capture = native::capture(profile, &std::path::absolute(&input)?);
        let elapsed = start.elapsed().as_secs_f64();
        let (capture, error) = match capture {
            Ok(capture) => (Some(capture), String::new()),
            Err(error) => (None, error.to_string()),
        };
        let complete = capture.as_ref().is_some_and(|capture| capture.complete);
        failed += usize::from(!complete);
        if !error.is_empty() {
            row(&mut diagnostics, &[&profile.name, "capture-error", &error])?;
        }
        if let Some(capture) = &capture {
            for (severity, diagnostic) in &capture.diagnostics {
                row(
                    &mut diagnostics,
                    &[&profile.name, &severity.to_string(), diagnostic],
                )?;
            }
            fingerprint(&profile.name, &capture.files, &mut hashes, &mut inputs)?;
            for (id, declaration) in capture.declarations.iter().enumerate() {
                declaration_row(&mut entries, profile, id, declaration)?;
            }
            declarations += capture
                .declarations
                .iter()
                .filter(|item| item.scope == "root")
                .count();
        }
        let names = capture.as_ref().map(classify::name_index);
        for (path, parsed) in &baseline {
            let family = if path.starts_with("metadata\\wdk") {
                "wdk"
            } else {
                "sdk"
            };
            if family != profile.family {
                continue;
            }
            let header = format!("{}.h", path.file_stem().unwrap().to_string_lossy());
            let candidates = capture
                .as_ref()
                .map(|capture| candidate_index(capture, &header));
            let observed = capture.as_ref().is_some_and(|capture| {
                capture
                    .files
                    .iter()
                    .any(|path| path.file_name().is_some_and(|name| name == header.as_str()))
            });
            for entry in parsed {
                if !is_api(entry, parsed)
                    || entry
                        .architecture
                        .is_some_and(|bits| bits & profile.architecture == 0)
                {
                    continue;
                }
                let matches = candidates
                    .as_ref()
                    .and_then(|candidates| candidates.get(entry.name.as_str()))
                    .into_iter()
                    .flatten()
                    .filter(|(_, candidate)| compatible(entry.kind, &candidate.kind))
                    .collect::<Vec<_>>();
                let identities = matches
                    .iter()
                    .map(|(id, declaration)| {
                        if declaration.canonical_usr.is_empty() {
                            format!("location:{id}")
                        } else {
                            declaration.canonical_usr.clone()
                        }
                    })
                    .collect::<BTreeSet<_>>();
                let status = mapping_status(complete, observed, identities.len());
                coverage
                    .entry((path.clone(), entry.id, profile.architecture))
                    .or_insert_with(|| (entry.kind.to_string(), entry.name.clone(), Vec::new()))
                    .2
                    .push((profile.name.clone(), status));
                if status != "single-identity-candidate" {
                    let evidence = names
                        .as_ref()
                        .and_then(|names| names.get(entry.name.as_str()))
                        .map(Vec::as_slice)
                        .unwrap_or_default();
                    let matching = matches
                        .iter()
                        .map(|(id, item)| (*id, *item))
                        .collect::<Vec<_>>();
                    let group = classify::group(entry.kind, &header, status, &matching, evidence);
                    let package = classify::package(group, entry.kind);
                    *group_counts
                        .entry((profile.name.clone(), group, package))
                        .or_default() += 1;
                    row(
                        &mut groups,
                        &[
                            &profile.name,
                            &path.to_string_lossy(),
                            &entry.id.to_string(),
                            entry.kind,
                            &entry.name,
                            &profile.architecture.to_string(),
                            status,
                            group,
                            package,
                            &evidence
                                .iter()
                                .map(|(id, _)| id.to_string())
                                .collect::<Vec<_>>()
                                .join(","),
                        ],
                    )?;
                }
                if matches.is_empty() {
                    mapping_row(&mut mappings, profile, path, entry, status, "")?;
                } else {
                    for (id, _) in matches {
                        mapping_row(&mut mappings, profile, path, entry, status, &id.to_string())?;
                    }
                }
            }
        }
        let count = |root: bool| {
            capture
                .as_ref()
                .filter(|capture| capture.complete)
                .map(|capture| {
                    capture
                        .declarations
                        .iter()
                        .filter(|item| !root || item.scope == "root")
                        .count()
                        .to_string()
                })
                .unwrap_or_default()
        };
        row(
            &mut manifest,
            &[
                &profile.name,
                profile.family,
                &profile.architecture.to_string(),
                if complete { "complete" } else { "blocked" },
                &count(true),
                &count(false),
                &capture
                    .as_ref()
                    .map(|capture| capture.target.clone())
                    .unwrap_or_default(),
                &capture
                    .as_ref()
                    .map(|capture| capture.pointer_bits.to_string())
                    .unwrap_or_default(),
                &format!("{elapsed:.3}"),
                &error,
            ],
        )?;
        manifest.flush()?;
        entries.flush()?;
        mappings.flush()?;
        diagnostics.flush()?;
        inputs.flush()?;
        groups.flush()?;
    }
    for writer in [
        &mut manifest,
        &mut roots,
        &mut entries,
        &mut mappings,
        &mut diagnostics,
        &mut inputs,
        &mut groups,
    ] {
        writer.flush()?;
    }
    let mut summary = report(
        &output,
        "groups-summary.tsv",
        "profile\tgroup\twork_package\tbaseline_entries\n",
    )?;
    for ((profile, group, package), count) in group_counts {
        row(
            &mut summary,
            &[&profile, group, package, &count.to_string()],
        )?;
    }
    summary.flush()?;
    let mut coverage_report = report(
        &output,
        "coverage.tsv",
        "partition\tid\tkind\tname\tarchitecture\tstatus\tprofile_statuses\n",
    )?;
    let mut coverage_counts = BTreeMap::<_, usize>::new();
    for ((path, id, arch), (kind, name, observations)) in coverage {
        let status = classify::coverage_status(observations.iter().map(|(_, status)| *status));
        let family = if path.starts_with("metadata\\wdk") {
            "wdk"
        } else {
            "sdk"
        };
        *coverage_counts.entry((family, arch, status)).or_default() += 1;
        row(
            &mut coverage_report,
            &[
                &path.to_string_lossy(),
                &id.to_string(),
                &kind,
                &name,
                &arch.to_string(),
                status,
                &observations
                    .iter()
                    .map(|(profile, status)| format!("{profile}:{status}"))
                    .collect::<Vec<_>>()
                    .join(","),
            ],
        )?;
    }
    coverage_report.flush()?;
    let mut coverage_summary = report(
        &output,
        "coverage-summary.tsv",
        "family\tarchitecture\tstatus\tbaseline_entries\n",
    )?;
    for ((family, arch, status), count) in coverage_counts {
        row(
            &mut coverage_summary,
            &[family, &arch.to_string(), status, &count.to_string()],
        )?;
    }
    coverage_summary.flush()?;
    run = run.replacen(
        "status\trunning",
        if failed == 0 {
            "status\tcomplete"
        } else {
            "status\tblocked"
        },
        1,
    );
    writeln!(run, "profiles\t{}", profiles.len())?;
    writeln!(run, "blocked_profiles\t{failed}")?;
    writeln!(run, "root_observations\t{declarations}")?;
    std::fs::write(run_path, run)?;
    if failed != 0 {
        return Err(format!(
            "{failed} of {} source profiles blocked; reports saved to {}. No RDL was generated.",
            profiles.len(),
            output.display()
        )
        .into());
    }
    println!(
        "Indexed {declarations} root observations in {}; source matches are candidates, not semantic agreement.",
        output.display()
    );
    Ok(())
}

fn baseline() -> Result<Vec<(PathBuf, Vec<InventoryEntry>)>> {
    rdl_files()?
        .into_iter()
        .map(|path| {
            let parsed = windows_rdl::inventory(&path)?;
            Ok((path, parsed))
        })
        .collect()
}

fn is_api(entry: &InventoryEntry, parsed: &[InventoryEntry]) -> bool {
    entry
        .parent
        .is_some_and(|id| parsed[id].kind == "namespace")
        && !matches!(entry.kind, "namespace" | "use")
}

fn candidate_index<'a>(
    capture: &'a Capture,
    header: &str,
) -> BTreeMap<&'a str, Vec<(usize, &'a Declaration)>> {
    let mut result = BTreeMap::<_, Vec<_>>::new();
    for (id, declaration) in capture.declarations.iter().enumerate() {
        if declaration.scope == "root"
            && declaration
                .header
                .file_name()
                .is_some_and(|name| name == header)
            && !declaration.name.is_empty()
            && !matches!(
                declaration.kind.as_str(),
                "FieldDecl" | "ParmDecl" | "CXXMethod"
            )
        {
            result
                .entry(declaration.name.as_str())
                .or_default()
                .push((id, declaration));
        }
    }
    result
}

fn compatible(rdl: &str, native: &str) -> bool {
    match rdl {
        "struct" | "union" | "interface" | "class" => matches!(
            native,
            "StructDecl" | "UnionDecl" | "ClassDecl" | "TypedefDecl" | "TypeAliasDecl"
        ),
        "enum" => matches!(native, "EnumDecl" | "TypedefDecl" | "TypeAliasDecl"),
        "typedef" | "callback" | "delegate" => matches!(native, "TypedefDecl" | "TypeAliasDecl"),
        "function" => native == "FunctionDecl",
        "constant" => matches!(native, "VarDecl" | "EnumConstantDecl" | "macro-definition"),
        _ => false,
    }
}

fn mapping_status(complete: bool, observed: bool, identities: usize) -> &'static str {
    if !complete {
        "blocked-profile"
    } else if !observed {
        "header-not-observed"
    } else {
        match identities {
            0 => "no-candidate",
            1 => "single-identity-candidate",
            _ => "ambiguous-candidate",
        }
    }
}

fn family(kind: &str) -> &'static str {
    match kind {
        "TypedefDecl" | "TypeAliasDecl" => "C2",
        "StructDecl" | "UnionDecl" | "ClassDecl" | "FieldDecl" => "C3",
        "EnumDecl" | "EnumConstantDecl" | "VarDecl" => "C4",
        "FunctionDecl" | "ParmDecl" => "C5",
        "CXXMethod" | "C++ base class specifier" | "Constructor" | "Destructor" => "C6",
        "macro-definition" => "C8",
        _ => "C0",
    }
}

fn report(output: &Path, name: &str, header: &str) -> Result<BufWriter<std::fs::File>> {
    let mut writer = BufWriter::new(std::fs::File::create(output.join(name))?);
    writer.write_all(header.as_bytes())?;
    Ok(writer)
}

fn row(writer: &mut impl std::io::Write, values: &[&str]) -> Result<()> {
    writeln!(
        writer,
        "{}",
        values
            .iter()
            .map(|value| escape(value))
            .collect::<Vec<_>>()
            .join("\t")
    )?;
    Ok(())
}

fn declaration_row(
    writer: &mut impl std::io::Write,
    profile: &Profile,
    id: usize,
    item: &Declaration,
) -> Result<()> {
    row(
        writer,
        &[
            &profile.name,
            &id.to_string(),
            &profile.architecture.to_string(),
            item.scope,
            &item.kind,
            &item.name,
            &item.qualified,
            &item.usr,
            &item.canonical_usr,
            &item.parent_usr,
            &item.alias_usr,
            &item.header.to_string_lossy(),
            &item.line.to_string(),
            &item.column.to_string(),
            &item.offset.to_string(),
            &item.spelling_header,
            &item.spelling_offset.to_string(),
            &item.written_type,
            &item.canonical_type,
            &item.definition.to_string(),
            &item.inline.to_string(),
            &item.function_macro.to_string(),
            family(&item.kind),
            if item.scope == "root" {
                "unclassified-source"
            } else {
                item.scope
            },
        ],
    )
}

fn mapping_row(
    writer: &mut impl std::io::Write,
    profile: &Profile,
    path: &Path,
    entry: &InventoryEntry,
    status: &str,
    native_id: &str,
) -> Result<()> {
    row(
        writer,
        &[
            &profile.name,
            &path.to_string_lossy(),
            &entry.id.to_string(),
            entry.kind,
            &entry.name,
            &profile.architecture.to_string(),
            status,
            native_id,
        ],
    )
}

fn fingerprint(
    profile: &str,
    paths: &[PathBuf],
    hashes: &mut BTreeMap<PathBuf, String>,
    writer: &mut impl std::io::Write,
) -> Result<()> {
    let missing = paths
        .iter()
        .filter(|path| !hashes.contains_key(*path))
        .collect::<BTreeSet<_>>();
    let missing = missing.into_iter().collect::<Vec<_>>();
    for chunk in missing.chunks(40) {
        let mut arguments = vec!["hash-object", "--no-filters", "--"];
        for path in chunk {
            arguments.push(path.to_str().ok_or("non-UTF-8 input path")?);
        }
        let output = git(&arguments)?;
        let values = output.lines().collect::<Vec<_>>();
        if values.len() != chunk.len() {
            return Err("incomplete header fingerprint set".into());
        }
        for (path, hash) in chunk.iter().zip(values) {
            hashes.insert((*path).clone(), hash.to_string());
        }
    }
    for path in paths {
        row(
            writer,
            &[
                profile,
                &path.to_string_lossy(),
                &std::fs::metadata(path)?.len().to_string(),
                &hashes[path],
            ],
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_and_unavailable_are_not_the_same() {
        assert_eq!(mapping_status(false, false, 0), "blocked-profile");
        assert_eq!(mapping_status(true, false, 0), "header-not-observed");
        assert_eq!(mapping_status(true, true, 0), "no-candidate");
        assert_eq!(mapping_status(true, true, 1), "single-identity-candidate");
        assert_eq!(mapping_status(true, true, 2), "ambiguous-candidate");
        assert!(!compatible("function", "macro-definition"));
        assert!(!compatible("function", "CXXMethod"));
        assert!(compatible("constant", "EnumConstantDecl"));
    }
}
