use super::*;
use std::collections::BTreeSet;
use std::process::Command;

const BASELINE: &str = "6e68e56081340eaa932f4dbc92ec54c56ed51473";
type Constants = BTreeMap<String, BTreeSet<String>>;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let directory = Path::new("target\\win32-2");
    let candidate = Path::new("metadata\\win32");
    if !candidate.is_dir() {
        return Err("working-tree RDL is missing; run tool-win32-2 --partial first".into());
    }
    let result = Command::new("git")
        .args([
            "--no-pager",
            "grep",
            "-n",
            "-E",
            "^[[:space:]]*const ",
            BASELINE,
            "--",
            "metadata\\win32",
        ])
        .output()?;
    if !result.status.success() {
        return Err(format!(
            "baseline scan failed: {}",
            String::from_utf8_lossy(&result.stderr)
        )
        .into());
    }
    let mut baseline = Constants::new();
    for line in std::str::from_utf8(&result.stdout)?.lines() {
        let parts: Vec<_> = line.splitn(4, ':').collect();
        let [_, header, _, declaration] = parts.as_slice() else {
            return Err(format!("invalid git grep result: {line}").into());
        };
        add(&mut baseline, header, declaration)?;
    }
    let mut current = Constants::new();
    for entry in std::fs::read_dir(candidate)? {
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "rdl") {
            for line in std::fs::read_to_string(&path)?.lines() {
                add(&mut current, &path.to_string_lossy(), line)?;
            }
        }
    }
    std::fs::create_dir_all(directory)?;
    std::fs::write(
        directory.join("constant-coverage.csv"),
        report(&baseline, &current),
    )?;
    let present = baseline
        .keys()
        .filter(|name| current.contains_key(*name))
        .count();
    let missing = baseline.len() - present;
    let added = current.len() - present;
    println!("Baseline: {BASELINE}");
    println!(
        "Constants: {} baseline, {} current; {present} present, {missing} missing, {added} new",
        baseline.len(),
        current.len()
    );
    println!(
        "Name presence only, not type/value/architecture parity. See target\\win32-2\\constant-coverage.csv"
    );
    Ok(())
}

fn report(baseline: &Constants, current: &Constants) -> String {
    let names: BTreeSet<_> = baseline.keys().chain(current.keys()).collect();
    let mut csv = String::from("Name,Status,BaselineHeaders,CurrentHeaders\n");
    for name in names {
        let old = baseline.get(name);
        let new = current.get(name);
        let status = match (old, new) {
            (Some(_), Some(_)) => "present",
            (Some(_), None) => "missing",
            (None, Some(_)) => "new",
            (None, None) => unreachable!(),
        };
        let headers = |value: Option<&BTreeSet<String>>| {
            value
                .into_iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>()
                .join(";")
        };
        let row = [name.clone(), status.to_string(), headers(old), headers(new)];
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
            report(&baseline, &current),
            concat!(
                "Name,Status,BaselineHeaders,CurrentHeaders\n",
                "\"Missing\",\"missing\",\"old.rdl\",\"\"\n",
                "\"Moved\",\"present\",\"old.rdl\",\"new.rdl\"\n",
                "\"missing\",\"new\",\"\",\"new.rdl\"\n",
            )
        );
    }
}
