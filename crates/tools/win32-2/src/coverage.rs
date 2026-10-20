use super::*;
use std::collections::BTreeSet;
use std::process::Command;

use named_exports::BASELINE;
const MASTER: &str = "afcb0ab1f8cf7a71ec563f659451748b26e7f5b6";
type Constants = BTreeMap<String, BTreeSet<String>>;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let directory = Path::new("target\\win32-2");
    let candidate = Path::new("metadata\\win32");
    if !candidate.is_dir() {
        return Err("working-tree RDL is missing; run tool-win32-2 --partial first".into());
    }
    let mut baseline = read_git_constants(BASELINE)?;
    let master = read_git_constants(MASTER)?;
    let supplement = Path::new(env!("CARGO_MANIFEST_DIR")).join("src\\baseline");
    for (name, headers) in read_constants(&supplement)? {
        if baseline.insert(name.clone(), headers).is_some() {
            return Err(format!("baseline supplement duplicates frozen constant `{name}`").into());
        }
    }
    let current = read_constants(candidate)?;
    let mut members = Constants::new();
    for entry in std::fs::read_dir(candidate)? {
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "rdl") {
            for name in windows_rdl::enum_member_names(&path, "Windows.Win32")? {
                members
                    .entry(name)
                    .or_default()
                    .insert(path.file_name().unwrap().to_string_lossy().into_owned());
            }
        }
    }
    std::fs::create_dir_all(directory)?;
    println!(
        "Baseline: {BASELINE} + reviewed source supplement ({})",
        supplement.display()
    );
    write_comparison(
        &baseline,
        &current,
        &members,
        &directory.join("constant-coverage.csv"),
    )?;
    println!("Inspected master: {MASTER} (separate control; frozen approvals unchanged)");
    write_comparison(
        &master,
        &current,
        &members,
        &directory.join("constant-coverage-master.csv"),
    )
}

fn read_git_constants(revision: &str) -> Result<Constants, Box<dyn std::error::Error>> {
    let result = Command::new("git")
        .args([
            "--no-pager",
            "grep",
            "-n",
            "-E",
            "^[[:space:]]*const ",
            revision,
            "--",
            "metadata\\win32",
        ])
        .output()?;
    if !result.status.success() {
        return Err(format!(
            "constant control scan failed; fetch {revision}: {}",
            String::from_utf8_lossy(&result.stderr)
        )
        .into());
    }
    let mut constants = Constants::new();
    for line in std::str::from_utf8(&result.stdout)?.lines() {
        let parts: Vec<_> = line.splitn(4, ':').collect();
        let [_, header, _, declaration] = parts.as_slice() else {
            return Err(format!("invalid git grep result: {line}").into());
        };
        add(&mut constants, header, declaration)?;
    }
    Ok(constants)
}

fn write_comparison(
    baseline: &Constants,
    current: &Constants,
    members: &Constants,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, report(baseline, current, members))?;
    let present = baseline
        .keys()
        .filter(|name| current.contains_key(*name))
        .count();
    let enum_members = baseline
        .keys()
        .filter(|name| !current.contains_key(*name) && members.contains_key(*name))
        .count();
    let missing = baseline.len() - present - enum_members;
    let added = current.len() - present;
    println!(
        "Constants: {} baseline, {} current; {present} present, {enum_members} enum members, {missing} missing, {added} new",
        baseline.len(),
        current.len()
    );
    println!(
        "Name presence only, not type/scope/value/architecture parity. See {}",
        path.display()
    );
    Ok(())
}

fn read_constants(directory: &Path) -> Result<Constants, Box<dyn std::error::Error>> {
    let mut constants = Constants::new();
    for entry in std::fs::read_dir(directory)? {
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "rdl") {
            for line in std::fs::read_to_string(&path)?.lines() {
                add(&mut constants, &path.to_string_lossy(), line)?;
            }
        }
    }
    Ok(constants)
}

fn report(baseline: &Constants, current: &Constants, members: &Constants) -> String {
    let names: BTreeSet<_> = baseline.keys().chain(current.keys()).collect();
    let mut csv = String::from("Name,Status,BaselineHeaders,CurrentHeaders\n");
    for name in names {
        let old = baseline.get(name);
        let new = current.get(name);
        let member = members.get(name);
        let status = match (old, new, member) {
            (Some(_), Some(_), _) => "present",
            (Some(_), None, Some(_)) => "enum-member",
            (Some(_), None, None) => "missing",
            (None, Some(_), _) => "new",
            (None, None, _) => unreachable!(),
        };
        let headers = |value: Option<&BTreeSet<String>>| {
            value
                .into_iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>()
                .join(";")
        };
        let row = [
            name.clone(),
            status.to_string(),
            headers(old),
            headers(new.or(member)),
        ];
        csv.push_str(
            &row.iter()
                .map(|value| format!("\"{}\"", value.replace('"', "\"\"")))
                .collect::<Vec<_>>()
                .join(","),
        );
        csv.push('\n');
    }
    csv
}

// The inputs are generated RDL declarations, not arbitrary source text or header macros.
fn add(values: &mut Constants, header: &str, line: &str) -> Result<(), String> {
    let Some(declaration) = line.trim_start().strip_prefix("const ") else {
        return Ok(());
    };
    let (name, _) = declaration
        .split_once(':')
        .ok_or_else(|| format!("invalid constant declaration in {header}: {line}"))?;
    let name = name.trim();
    let name = name.strip_prefix("r#").unwrap_or(name);
    if name.is_empty() || name.chars().any(char::is_whitespace) {
        return Err(format!("invalid constant name in {header}: {line}"));
    }
    values
        .entry(name.into())
        .or_default()
        .insert(header.rsplit(['/', '\\']).next().unwrap().into());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_supplement() {
        let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("src\\baseline");
        assert_eq!(
            read_constants(&directory).unwrap(),
            BTreeMap::from([(
                "WM_CTLCOLOR".into(),
                BTreeSet::from(["windowsx.rdl".into()]),
            )])
        );
    }

    #[test]
    fn constant_names_and_owners() {
        let mut values = Constants::new();
        add(&mut values, "metadata/a.rdl", "  const Value: u32 = 1;").unwrap();
        add(&mut values, "metadata/a.rdl", "  const Value: u64 = 1;").unwrap();
        add(&mut values, "metadata/b.rdl", "const Value: u64 = 1;").unwrap();
        add(&mut values, "metadata/b.rdl", "const value: u32 = 2;").unwrap();
        add(&mut values, "metadata/b.rdl", "// const Absent: u32 = 0;").unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(
            values["Value"],
            BTreeSet::from(["a.rdl".into(), "b.rdl".into()])
        );
        assert!(add(&mut values, "bad.rdl", "const Broken").is_err());
        add(&mut values, "raw.rdl", "const r#type: u32 = 1;").unwrap();
        assert!(values.contains_key("type"));
        assert!(!values.contains_key("r#type"));
    }

    #[test]
    fn moved_ownership_and_presence() {
        let mut baseline = Constants::new();
        let mut current = Constants::new();
        add(&mut baseline, "old.rdl", "const Moved: u32 = 1;").unwrap();
        add(&mut current, "new.rdl", "const Moved: u64 = 2;").unwrap();
        add(&mut baseline, "old.rdl", "const Missing: u32 = 0;").unwrap();
        add(&mut current, "new.rdl", "const missing: u32 = 0;").unwrap();
        assert_eq!(
            report(&baseline, &current, &Constants::new()),
            concat!(
                "Name,Status,BaselineHeaders,CurrentHeaders\n",
                "\"Missing\",\"missing\",\"old.rdl\",\"\"\n",
                "\"Moved\",\"present\",\"old.rdl\",\"new.rdl\"\n",
                "\"missing\",\"new\",\"\",\"new.rdl\"\n",
            )
        );
    }

    #[test]
    fn enum_members_have_distinct_accounting() {
        let baseline = ["Both", "Member", "Missing"]
            .into_iter()
            .map(|name| (name.into(), BTreeSet::from(["old.rdl".into()])))
            .collect();
        let current = BTreeMap::from([("Both".into(), BTreeSet::from(["constant.rdl".into()]))]);
        let members = ["Both", "Member", "NotInBaseline"]
            .into_iter()
            .map(|name| (name.into(), BTreeSet::from(["enum.rdl".into()])))
            .collect();
        assert_eq!(
            report(&baseline, &current, &members),
            concat!(
                "Name,Status,BaselineHeaders,CurrentHeaders\n",
                "\"Both\",\"present\",\"old.rdl\",\"constant.rdl\"\n",
                "\"Member\",\"enum-member\",\"old.rdl\",\"enum.rdl\"\n",
                "\"Missing\",\"missing\",\"old.rdl\",\"\"\n",
            )
        );
    }
}
