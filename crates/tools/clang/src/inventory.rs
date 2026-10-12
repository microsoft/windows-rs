use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use windows_rdl::InventoryEntry;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const OUTPUT: &str = "target\\clang-inventory";
const CONFIGURATION: &[&str] = &[
    "crates\\tools\\helpers\\src\\provision.rs",
    "crates\\tools\\win32\\src\\main.rs",
    "crates\\tools\\win32\\src\\km.rs",
    "crates\\tools\\webview\\src\\main.rs",
    "Cargo.lock",
];

pub fn run() -> Result<()> {
    let mut files = rdl_files()?;
    files.push(PathBuf::from("metadata\\metadata.rdl"));
    files.sort();

    let mut baseline = String::from("key\tvalue\n");
    for (key, value) in [
        ("schema", "1".to_string()),
        ("revision", git(&["rev-parse", "--verify", "HEAD"])?),
        (
            "object_format",
            git(&["rev-parse", "--show-object-format"])?,
        ),
        ("working_tree", git(&["status", "--short"])?),
        ("coverage", "existing-rdl-only".to_string()),
        ("header_discovery", "not-run".to_string()),
        ("source_mapping", "unresolved".to_string()),
        ("libclang", helpers::LIBCLANG_VERSION.to_string()),
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
    ] {
        writeln!(baseline, "{key}\t{}", escape(&value))?;
    }

    let mut entries = String::from(
        "partition\tid\tparent\tnamespace\tkind\tname\tshape\tattributes\tarchitecture\tline\tfamily_package\tsource_status\theader_hint\n",
    );
    let mut counts = BTreeMap::<&str, usize>::new();
    let mut declarations = 0;
    for path in &files {
        let vocabulary = path == Path::new("metadata\\metadata.rdl");
        let parsed = windows_rdl::inventory(path)?;
        let header_hint = if vocabulary {
            String::new()
        } else {
            format!(
                "{}.h",
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or("invalid RDL partition name")?
            )
        };
        let mut packages = Vec::new();
        for entry in &parsed {
            let package = if vocabulary {
                "C1"
            } else {
                package(entry, &packages)
            };
            packages.push(package);
            *counts.entry(entry.kind).or_default() += 1;
            let declaration = entry.parent.is_some_and(|parent| {
                parsed[parent].kind == "namespace" && entry.kind != "namespace"
            });
            declarations += usize::from(declaration && !vocabulary);
            let status = if vocabulary {
                "vocabulary"
            } else if matches!(entry.kind, "namespace" | "use") {
                "not-applicable"
            } else {
                "unresolved"
            };
            let columns = [
                path.to_string_lossy().into_owned(),
                entry.id.to_string(),
                entry.parent.map(|id| id.to_string()).unwrap_or_default(),
                entry.namespace.clone(),
                entry.kind.to_string(),
                entry.name.clone(),
                entry.shape.clone(),
                entry.attributes.clone(),
                entry
                    .architecture
                    .map(|bits| bits.to_string())
                    .unwrap_or_default(),
                entry.line.to_string(),
                package.to_string(),
                status.to_string(),
                if vocabulary || status == "not-applicable" {
                    String::new()
                } else {
                    header_hint.clone()
                },
            ];
            writeln!(
                entries,
                "{}",
                columns
                    .iter()
                    .map(|column| escape(column))
                    .collect::<Vec<_>>()
                    .join("\t")
            )?;
        }
    }
    let mut summary = String::from("kind\tentries\n");
    for (kind, count) in &counts {
        writeln!(summary, "{kind}\t{count}")?;
    }
    writeln!(baseline, "rdl_files\t{}", files.len())?;
    writeln!(baseline, "api_declarations\t{declarations}")?;

    let mut manifest = String::from("path\trole\tbytes\tgit_blob\n");
    let mut inputs = files.clone();
    inputs.extend(CONFIGURATION.iter().map(PathBuf::from));
    for chunk in inputs.chunks(40) {
        let paths = chunk
            .iter()
            .map(|path| path.to_str().ok_or("non-UTF-8 inventory path"))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut arguments = vec!["hash-object", "--no-filters", "--"];
        arguments.extend(paths);
        let hashes = git(&arguments)?;
        let hashes = hashes.lines().collect::<Vec<_>>();
        if hashes.len() != chunk.len() {
            return Err("git returned an incomplete inventory input fingerprint set".into());
        }
        for (path, hash) in chunk.iter().zip(hashes) {
            writeln!(
                manifest,
                "{}\t{}\t{}\t{hash}",
                escape(&path.to_string_lossy()),
                if path.extension().is_some_and(|extension| extension == "rdl") {
                    "rdl"
                } else {
                    "configuration"
                },
                std::fs::metadata(path)?.len(),
            )?;
        }
    }

    std::fs::create_dir_all(OUTPUT)?;
    for (name, contents) in [
        ("entries.tsv", entries),
        ("summary.tsv", summary),
        ("inputs.tsv", manifest),
        ("baseline.tsv", baseline),
    ] {
        std::fs::write(Path::new(OUTPUT).join(name), contents)?;
    }
    println!(
        "Inventoried {declarations} API declarations from {} RDL files in {OUTPUT}; header mappings remain unresolved.",
        files.len()
    );
    Ok(())
}

pub(crate) fn rdl_files() -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for directory in ["metadata\\win32", "metadata\\wdk"] {
        files.extend(rdl_directory(Path::new(directory))?);
    }
    files.sort();
    Ok(files)
}

fn rdl_directory(directory: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_file() && path.extension().is_some_and(|extension| extension == "rdl") {
            files.push(path);
        }
    }
    if files.is_empty() {
        return Err(format!("no RDL inputs in `{}`", directory.display()).into());
    }
    Ok(files)
}

pub(crate) fn git(arguments: &[&str]) -> Result<String> {
    let output = Command::new("git").args(arguments).output()?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            arguments.join(" "),
            String::from_utf8(output.stderr)?
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?.trim_end().to_string())
}

pub(crate) fn pin(path: &str, name: &str) -> Result<String> {
    let source = std::fs::read_to_string(path)?;
    let file = syn::parse_file(&source)?;
    for item in file.items {
        if let syn::Item::Const(item) = item
            && item.ident == name
        {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(value),
                ..
            }) = *item.expr
            {
                return Ok(value.value());
            }
            return Err(format!("pin `{name}` in `{path}` is not a string literal").into());
        }
    }
    Err(format!("pin `{name}` is missing from `{path}`").into())
}

fn package(entry: &InventoryEntry, parents: &[&'static str]) -> &'static str {
    match entry.kind {
        "namespace" | "use" => "C0",
        "attribute" => "C1",
        "typedef" => "C2",
        "struct" | "union" => "C3",
        "enum" | "constant" => "C4",
        "function" | "callback" | "delegate" => "C5",
        "interface" | "class" => "C6",
        _ => entry.parent.map_or("C0", |parent| parents[parent]),
    }
}

pub(crate) fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tsv_fields_preserve_control_characters_and_backslashes() {
        assert_eq!(escape("a\\b\tc\r\nd"), "a\\\\b\\tc\\r\\nd");
        assert_eq!(escape("\\t"), "\\\\t");
    }

    #[test]
    fn directories_can_contain_non_rdl_entries() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\tests\\libs\\rdl");
        assert!(!rdl_directory(&root).unwrap().is_empty());
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
        assert!(
            rdl_directory(&root)
                .unwrap_err()
                .to_string()
                .contains("no RDL inputs")
        );
    }
}
