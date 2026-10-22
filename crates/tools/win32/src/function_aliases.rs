use super::*;
use std::hash::{Hash, Hasher};
use windows_clang::{Snapshot, ValueKind};

pub fn selection() -> Result<BTreeMap<Target, BTreeSet<String>>, String> {
    let mut selected = BTreeMap::<Target, BTreeSet<String>>::new();
    for line in include_str!("function-aliases.txt")
        .lines()
        .filter(|line| !line.starts_with('#'))
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        let [mask, name] = fields.as_slice() else {
            return Err(format!("invalid function alias selection: {line:?}"));
        };
        let mask: u8 = mask
            .parse()
            .map_err(|_| format!("invalid alias target mask: {line:?}"))?;
        if mask == 0
            || mask & !7 != 0
            || !name.bytes().enumerate().all(|(i, byte)| {
                byte == b'_' || byte.is_ascii_alphabetic() || (i != 0 && byte.is_ascii_digit())
            })
        {
            return Err(format!("invalid function alias selection: {line:?}"));
        }
        for (target, bit) in [(Target::X86, 1), (Target::X64, 2), (Target::Arm64, 4)] {
            if mask & bit != 0 && !selected.entry(target).or_default().insert((*name).into()) {
                return Err(format!("duplicate function alias selection: {line:?}"));
            }
        }
    }
    Ok(selected)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Origin {
    Native,
    Alias(BTreeSet<String>),
}

impl Origin {
    pub fn category(&self, export: &str) -> &'static str {
        match self {
            Self::Native => "native-declaration",
            Self::Alias(_) if export.ends_with('A') => "macro-alias-to-A-export",
            Self::Alias(_) => "macro-alias",
        }
    }

    pub fn references(&self) -> String {
        match self {
            Self::Native => String::new(),
            Self::Alias(names) => names.iter().cloned().collect::<Vec<_>>().join(","),
        }
    }
}

pub type Origins = BTreeMap<(Target, String), Origin>;

pub fn observe(snapshots: &[Snapshot]) -> Origins {
    let mut origins = Origins::new();
    for snapshot in snapshots {
        for value in &snapshot.values {
            for observation in &value.observations {
                let ValueKind::Function { alias, .. } = &observation.kind else {
                    continue;
                };
                let name = value.name.rsplit("::").next().unwrap().to_string();
                let origin = origins
                    .entry((snapshot.target, name))
                    .or_insert_with(|| Origin::Alias(BTreeSet::new()));
                if let Some(alias) = alias {
                    if let Origin::Alias(names) = origin {
                        names.insert(alias.name.clone());
                    }
                } else {
                    *origin = Origin::Native;
                }
            }
        }
    }
    origins
}

fn fingerprint<'a>(partitions: impl Iterator<Item = (&'a str, &'a str)>) -> String {
    let mut hash = std::collections::hash_map::DefaultHasher::new();
    for partition in partitions {
        partition.hash(&mut hash);
    }
    format!("{:016x}", hash.finish())
}

pub fn write(
    directory: &Path,
    origins: &Origins,
    partitions: &BTreeMap<String, projection::Partition>,
) -> Result<(), std::io::Error> {
    let fingerprint = fingerprint(
        partitions
            .iter()
            .map(|(name, part)| (name.as_str(), part.rdl.as_str())),
    );
    let mut text = format!("# RDL fingerprint: {fingerprint}\n");
    for ((target, name), origin) in origins {
        text.push_str(&format!("{target:?}\t{name}\t{}\n", origin.references()));
    }
    std::fs::write(directory.join("function-origins.txt"), text)
}

pub fn read(directory: &Path) -> Result<Origins, Box<dyn std::error::Error>> {
    let mut partitions = BTreeMap::new();
    for entry in std::fs::read_dir("metadata\\win32")? {
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "rdl") {
            partitions.insert(
                path.file_stem().unwrap().to_string_lossy().into_owned(),
                std::fs::read_to_string(path)?,
            );
        }
    }
    let fingerprint = fingerprint(
        partitions
            .iter()
            .map(|(name, rdl)| (name.as_str(), rdl.as_str())),
    );
    let text =
        std::fs::read_to_string(directory.join("function-origins.txt")).map_err(|error| {
            format!("missing function provenance; regenerate with tool-win32: {error}")
        })?;
    parse(&text, &fingerprint).map_err(Into::into)
}

fn parse(text: &str, fingerprint: &str) -> Result<Origins, String> {
    let mut lines = text.lines();
    if lines.next() != Some(format!("# RDL fingerprint: {fingerprint}").as_str()) {
        return Err("stale function provenance; regenerate with tool-win32".into());
    }
    let mut origins = Origins::new();
    for line in lines {
        let fields: Vec<_> = line.split('\t').collect();
        let [target, name, references] = fields.as_slice() else {
            return Err(format!("invalid function provenance: {line:?}"));
        };
        let target = match *target {
            "X86" => Target::X86,
            "X64" => Target::X64,
            "Arm64" => Target::Arm64,
            _ => return Err(format!("invalid function provenance target: {line:?}")),
        };
        let origin = if references.is_empty() {
            Origin::Native
        } else {
            Origin::Alias(references.split(',').map(String::from).collect())
        };
        if name.is_empty() || origins.insert((target, (*name).into()), origin).is_some() {
            return Err(format!(
                "invalid or duplicate function provenance: {line:?}"
            ));
        }
    }
    Ok(origins)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_compatibility_selection() {
        let selected = selection().unwrap();
        for name in [
            "EnumProcesses",
            "GetProcessMemoryInfo",
            "GetWindowLongPtrW",
            "UrlIs",
        ] {
            assert!(selected[&Target::X86].contains(name), "{name}");
        }
        for name in [
            "CreateFile",
            "CreateEvent",
            "AnsiLower",
            "AccFree",
            "AVIFileClose",
        ] {
            assert!(
                selected.values().all(|names| !names.contains(name)),
                "{name}"
            );
        }
        assert!(!selected[&Target::X64].contains("GetWindowLongPtrW"));
        assert!(!selected[&Target::X64].contains("SymGetModuleInfo"));
    }

    #[test]
    fn provenance_categories_and_staleness() {
        let text = "# RDL fingerprint: sample\nX86\tNativeA\t\nX86\tNeutral\tNativeA\nX64\tCompat\tNativeA\n";
        let origins = parse(text, "sample").unwrap();
        assert_eq!(
            origins[&(Target::X86, "NativeA".into())].category("NativeA"),
            "native-declaration"
        );
        assert_eq!(
            origins[&(Target::X86, "Neutral".into())].category("NativeA"),
            "macro-alias-to-A-export"
        );
        assert_eq!(
            origins[&(Target::X64, "Compat".into())].category("Other"),
            "macro-alias"
        );
        assert!(parse(text, "changed").is_err());
        assert!(parse(&format!("{text}X86\tNativeA\t\n"), "sample").is_err());
        assert!(parse("# RDL fingerprint: sample\nOther\tF\t\n", "sample").is_err());
        assert_ne!(
            fingerprint([("a", "one")].into_iter()),
            fingerprint([("a", "two")].into_iter())
        );
    }
}
