use super::*;
use windows_clang2::{Snapshot, ValueKind};
use windows_metadata::reader::{self, HasAttributes};
use windows_rdl::implib::{Import, ImportKind};

pub const BASELINE: &str = "6e68e56081340eaa932f4dbc92ec54c56ed51473";
const SOURCE: &str = "metadata/win32";

#[derive(Debug, Eq, PartialEq)]
struct Export {
    library: String,
    name: String,
}

pub struct Evidence(BTreeMap<(Target, String), Export>);

impl Evidence {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::parse(&std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src\\named-exports.txt"),
        )?)
        .map_err(Into::into)
    }

    fn parse(text: &str) -> Result<Self, String> {
        let mut lines = text.lines();
        if lines.next() != Some(format!("# {BASELINE}:{SOURCE}").as_str()) {
            return Err("named-export evidence does not identify the frozen baseline".into());
        }
        let mut entries = BTreeMap::new();
        for line in lines {
            let columns: Vec<_> = line.split('\t').collect();
            let [arches, source, library, name] = columns.as_slice() else {
                return Err(format!("invalid named-export evidence: {line}"));
            };
            let arches: u8 = arches
                .parse()
                .map_err(|_| format!("invalid target mask: {line}"))?;
            if arches == 0
                || arches & !7 != 0
                || [source, library, name]
                    .iter()
                    .any(|value| value.is_empty() || value.contains(char::is_whitespace))
                || name.starts_with('#')
            {
                return Err(format!("invalid named-export evidence: {line}"));
            }
            for (target, bit) in [(Target::X86, 1), (Target::X64, 2), (Target::Arm64, 4)] {
                if arches & bit == 0 {
                    continue;
                }
                let export = Export {
                    library: library.to_ascii_lowercase(),
                    name: (*name).into(),
                };
                if entries.insert((target, (*source).into()), export).is_some() {
                    return Err(format!("duplicate named-export evidence: {line}"));
                }
            }
        }
        if entries.is_empty() {
            return Err("named-export evidence is empty".into());
        }
        Ok(Self(entries))
    }

    #[cfg(test)]
    pub fn names(&self, target: Target, library: &str) -> Vec<String> {
        self.0
            .iter()
            .filter(|((t, _), export)| *t == target && export.library.eq_ignore_ascii_case(library))
            .map(|((_, name), _)| name.clone())
            .collect()
    }

    pub fn resolve<'a>(
        &'a self,
        target: Target,
        import: &Import,
        native: &BTreeMap<String, String>,
    ) -> Result<Option<&'a str>, String> {
        if import.machine != import_target(target).1 || import.kind != ImportKind::Code {
            return Err(format!(
                "{target:?}: invalid native import evidence: {import:?}"
            ));
        }
        let Some(source) = native.get(&import.symbol) else {
            return Ok(None);
        };
        let Some(export) = self.0.get(&(target, source.clone())) else {
            return Ok(None);
        };
        if !import.dll.eq_ignore_ascii_case(&export.library) {
            return Err(format!(
                "{target:?} `{source}`: SDK provider `{}` conflicts with baseline named provider `{}`",
                import.dll, export.library
            ));
        }
        Ok(Some(&export.name))
    }
}

pub fn native_symbols(
    snapshots: &[Snapshot],
    target: Target,
) -> Result<BTreeMap<String, String>, String> {
    let mut result = BTreeMap::new();
    for snapshot in snapshots
        .iter()
        .filter(|snapshot| snapshot.target == target)
    {
        for value in &snapshot.values {
            for observation in &value.observations {
                if let ValueKind::Function {
                    alias: None,
                    mangling,
                    stdcall_candidate,
                    ..
                } = &observation.kind
                {
                    for mangling in std::iter::once(mangling).chain(
                        stdcall_candidate
                            .iter()
                            .map(|candidate| &candidate.mangling),
                    ) {
                        if let Some(previous) = result.insert(mangling.clone(), value.name.clone())
                            && previous != value.name
                        {
                            return Err(format!(
                                "{target:?} native symbol `{mangling}` has conflicting names `{previous}` and `{}`",
                                value.name
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(result)
}

pub fn refresh() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let directory = Path::new("target\\win32-2").join(format!("named-exports-{BASELINE}"));
    std::fs::create_dir_all(&directory)?;
    let output = Command::new("git")
        .args([
            "archive",
            "--format=tar",
            BASELINE,
            "metadata\\win32",
            "metadata\\metadata.rdl",
            "crates\\libs\\default\\Windows.winmd",
        ])
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "cannot read frozen metadata; fetch {BASELINE}: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let mut extract = Command::new("tar")
        .args(["-xf", "-", "-C"])
        .arg(&directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    extract.stdin.take().unwrap().write_all(&output.stdout)?;
    let output = extract.wait_with_output()?;
    if !output.status.success() {
        return Err(format!(
            "cannot extract frozen RDL: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let winmd = directory.join("baseline.winmd");
    windows_rdl::reader()
        .input(directory.join("metadata\\metadata.rdl"))
        .input(directory.join("metadata\\win32"))
        .reference(directory.join("crates\\libs\\default\\Windows.winmd"))
        .output(&winmd)
        .write()?;
    let index = reader::Index::read(&winmd).ok_or("invalid frozen metadata")?;
    let mut entries = BTreeMap::<(String, String, String), i32>::new();
    for (namespace, _, item) in index.iter_items() {
        if namespace != "Windows.Win32" && !namespace.starts_with("Windows.Win32.") {
            continue;
        }
        let reader::Item::Fn(method) = item else {
            continue;
        };
        let Some(import) = method.impl_map() else {
            continue;
        };
        let name = import.import_name();
        if name.starts_with('#') {
            continue;
        }
        let arches = method.arches();
        *entries
            .entry((
                method.name().into(),
                import.import_scope().name().to_ascii_lowercase(),
                name.into(),
            ))
            .or_default() |= if arches == 0 { 7 } else { arches };
    }
    let mut text = format!("# {BASELINE}:{SOURCE}\n");
    for ((source, library, name), arches) in entries {
        text.push_str(&format!("{arches}\t{source}\t{library}\t{name}\n"));
    }
    Evidence::parse(&text)?;
    std::fs::write(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src\\named-exports.txt"),
        text,
    )?;
    std::fs::remove_dir_all(directory)?;
    Ok(())
}

fn approved_corrections(text: &str) -> Result<BTreeSet<(Target, String)>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(format!("# Baseline: {BASELINE}").as_str())
        || lines.next() != Some(format!("# SDK: {SDK_VERSION}").as_str())
    {
        return Err("availability corrections do not identify the frozen baseline and SDK".into());
    }
    let mut corrections = BTreeSet::new();
    for line in lines.filter(|line| !line.starts_with('#')) {
        let columns: Vec<_> = line.split_whitespace().collect();
        let ["X86", name] = columns.as_slice() else {
            return Err(format!("invalid approved availability correction: {line}"));
        };
        if !corrections.insert((Target::X86, (*name).into())) {
            return Err(format!(
                "duplicate approved availability correction: {line}"
            ));
        }
    }
    if corrections.is_empty() {
        return Err("approved availability corrections are empty".into());
    }
    Ok(corrections)
}

fn validate_corrections(
    corrections: &BTreeSet<(Target, String)>,
    baseline: &Evidence,
    current: &BTreeMap<(Target, String), Export>,
) -> Result<(), String> {
    for key @ (target, name) in corrections {
        if !baseline.0.contains_key(key) {
            return Err(format!(
                "approved correction `{name}` on {target:?} is absent from baseline"
            ));
        }
        if current.contains_key(key) {
            return Err(format!(
                "stale approved correction `{name}` on {target:?}: function is present"
            ));
        }
    }
    Ok(())
}

pub fn compare() -> Result<(), Box<dyn std::error::Error>> {
    let baseline = Evidence::load()?;
    let corrections = approved_corrections(include_str!("approved-availability.txt"))?;
    let directory = Path::new("target\\win32-2");
    std::fs::create_dir_all(directory)?;
    let winmd = directory.join("function-coverage.winmd");
    windows_rdl::reader()
        .input("metadata\\metadata.rdl")
        .input("metadata\\win32")
        .output(&winmd)
        .write()?;
    let index = reader::Index::read(&winmd).ok_or("invalid current metadata")?;
    let mut current = BTreeMap::new();
    for (_, source, item) in index.iter_items() {
        let reader::Item::Fn(method) = item else {
            continue;
        };
        let Some(import) = method.impl_map() else {
            continue;
        };
        for (target, bit) in [(Target::X86, 1), (Target::X64, 2), (Target::Arm64, 4)] {
            if method.arches() != 0 && method.arches() & bit == 0 {
                continue;
            }
            let export = Export {
                library: import.import_scope().name().to_ascii_lowercase(),
                name: import.import_name().into(),
            };
            if current
                .insert((target, source.to_string()), export)
                .is_some()
            {
                return Err(format!("duplicate current function `{source}` on {target:?}").into());
            }
        }
    }
    validate_corrections(&corrections, &baseline, &current)?;
    let keys: BTreeSet<_> = baseline.0.keys().chain(current.keys()).collect();
    let mut rows = String::from(
        "Target,Name,Status,BaselineLibrary,BaselineExport,CurrentLibrary,CurrentExport,Acceptance\n",
    );
    let mut totals = BTreeMap::<&str, usize>::new();
    let mut missing = BTreeSet::new();
    let mut unapproved_names = BTreeSet::new();
    let mut unapproved_pairs = 0;
    let current_names: BTreeSet<_> = current.keys().map(|key| &key.1).collect();
    for key in keys {
        let old = baseline.0.get(key);
        let new = current.get(key);
        let status = match (old, new) {
            (Some(a), Some(b)) if a == b => "same-linkage",
            (Some(_), Some(_)) => "changed-linkage",
            (Some(_), None) => {
                missing.insert(&key.1);
                "missing"
            }
            (None, Some(_)) => "new",
            _ => unreachable!(),
        };
        let acceptance = if status != "missing" {
            ""
        } else if corrections.contains(key) {
            "approved-availability"
        } else {
            unapproved_pairs += 1;
            unapproved_names.insert(&key.1);
            "unapproved"
        };
        *totals.entry(status).or_default() += 1;
        let row = [
            format!("{:?}", key.0),
            key.1.clone(),
            status.into(),
            old.map_or("", |v| &v.library).into(),
            old.map_or("", |v| &v.name).into(),
            new.map_or("", |v| &v.library).into(),
            new.map_or("", |v| &v.name).into(),
            acceptance.into(),
        ];
        rows.push_str(
            &row.iter()
                .map(|v| format!("\"{}\"", v.replace('"', "\"\"")))
                .collect::<Vec<_>>()
                .join(","),
        );
        rows.push('\n');
    }
    std::fs::write(directory.join("function-coverage.csv"), rows)?;
    println!("Baseline: {BASELINE}\nFunction target pairs: {totals:?}");
    let absent = missing.difference(&current_names).count();
    println!(
        "{absent} names absent; {} names missing on at least one baseline target. Presence/linkage only, not signature or behavior parity. See target\\win32-2\\function-coverage.csv",
        missing.len()
    );
    println!(
        "{} approved availability corrections; {unapproved_pairs} unapproved missing target pairs across {} names. Changed linkage and additions are not approved by this accounting.",
        corrections.len(),
        unapproved_names.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_rdl::implib::ImportName;

    #[test]
    fn reviewed_availability_corrections() {
        let corrections = approved_corrections(include_str!("approved-availability.txt")).unwrap();
        assert_eq!(corrections.len(), 88);
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..\\..\\tests\\libs\\clang2\\input\\sdk_import_availability.txt");
        let observed = std::fs::read_to_string(fixtures).unwrap();
        let expected: BTreeSet<_> = observed
            .lines()
            .filter_map(|line| {
                let fields: Vec<_> = line.split_whitespace().collect();
                match fields.as_slice() {
                    ["6", name, _] => Some((Target::X86, (*name).into())),
                    _ => None,
                }
            })
            .collect();
        assert_eq!(corrections, expected);
        validate_corrections(&corrections, &Evidence::load().unwrap(), &BTreeMap::new()).unwrap();
        for name in [
            "RtlCaptureContext2",
            "Ndr64AsyncClientCall",
            "Ndr64DcomAsyncClientCall",
            "NdrClientCall3",
        ] {
            assert!(!corrections.contains(&(Target::X86, name.into())));
        }
    }

    #[test]
    fn invalid_availability_corrections() {
        let header = format!("# Baseline: {BASELINE}\n# SDK: {SDK_VERSION}\n");
        for rows in [
            "",
            "X64 F\n",
            "Arm64 F\n",
            "X86\n",
            "X86 F extra\n",
            "X86 F\nX86 F\n",
        ] {
            assert!(approved_corrections(&format!("{header}{rows}")).is_err());
        }
        assert!(approved_corrections("# Baseline: other\n# SDK: other\nX86 F\n").is_err());
        assert!(
            approved_corrections(&format!("# Baseline: {BASELINE}\n# SDK: other\nX86 F\n"))
                .is_err()
        );
        assert!(
            approved_corrections(&format!("# Baseline: other\n# SDK: {SDK_VERSION}\nX86 F\n"))
                .is_err()
        );
    }

    #[test]
    fn stale_availability_corrections() {
        let baseline =
            Evidence::parse(&format!("# {BASELINE}:{SOURCE}\n7\tF\texample.dll\tF\n")).unwrap();
        let corrections = [(Target::X86, "F".into())].into();
        let mut current = BTreeMap::from([(
            (Target::X64, "F".into()),
            Export {
                library: "example.dll".into(),
                name: "F".into(),
            },
        )]);
        validate_corrections(&corrections, &baseline, &current).unwrap();
        assert!(
            validate_corrections(&[(Target::X86, "Other".into())].into(), &baseline, &current)
                .unwrap_err()
                .contains("absent from baseline")
        );
        current.insert(
            (Target::X86, "F".into()),
            Export {
                library: "changed.dll".into(),
                name: "Changed".into(),
            },
        );
        assert!(
            validate_corrections(&corrections, &baseline, &current)
                .unwrap_err()
                .contains("function is present")
        );
    }

    #[test]
    fn exact_evidence() {
        let evidence = Evidence::parse(&format!(
            "# {BASELINE}:{SOURCE}\n3\tFunction\tprovider.dll\tPublicExport\n"
        ))
        .unwrap();
        let import = Import {
            machine: 0x14c,
            kind: ImportKind::Code,
            symbol: "_Function@8".into(),
            dll: "PROVIDER.dll".into(),
            name: ImportName::Ordinal(9),
        };
        let native = [(import.symbol.clone(), "Function".into())].into();
        assert_eq!(
            evidence.resolve(Target::X86, &import, &native).unwrap(),
            Some("PublicExport")
        );
        assert_eq!(
            evidence
                .resolve(Target::X86, &import, &BTreeMap::new())
                .unwrap(),
            None
        );
        for symbol in ["_Function", "_Function@4", "Function"] {
            let changed = Import {
                symbol: symbol.into(),
                ..import.clone()
            };
            assert_eq!(
                evidence.resolve(Target::X86, &changed, &native).unwrap(),
                None
            );
        }
        for changed in [
            Import {
                machine: 0x8664,
                ..import.clone()
            },
            Import {
                kind: ImportKind::Data,
                ..import.clone()
            },
            Import {
                dll: "other.dll".into(),
                ..import.clone()
            },
        ] {
            assert!(evidence.resolve(Target::X86, &changed, &native).is_err());
        }
        let arm = Import {
            machine: 0xaa64,
            ..import
        };
        assert_eq!(
            evidence.resolve(Target::Arm64, &arm, &native).unwrap(),
            None
        );
    }

    #[test]
    fn malformed_evidence() {
        for rows in [
            "",
            "8\tF\tx.dll\tF\n",
            "0\tF\tx.dll\tF\n",
            "7\tF\tx.dll\t#2\n",
            "7\tF\tx.dll\tF\n7\tF\tx.dll\tF\n",
        ] {
            assert!(Evidence::parse(&format!("# {BASELINE}:{SOURCE}\n{rows}")).is_err());
        }
        assert!(Evidence::parse("wrong baseline\n").is_err());
    }
}
