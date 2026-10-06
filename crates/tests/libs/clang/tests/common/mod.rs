struct FixtureSource {
    name: String,
    source: String,
    input: bool,
}

struct Fixture {
    sources: Vec<FixtureSource>,
    namespace: String,
    library: String,
    args: Vec<String>,
    filters: Vec<String>,
    reference_default: bool,
    references: Vec<String>,
}

struct PreparedFixture {
    fixture: Fixture,
    scratch: std::path::PathBuf,
    references: Vec<std::path::PathBuf>,
}

fn prepare(name: &str, prefix: &str) -> PreparedFixture {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = root.join("input").join(format!("{name}.h"));
    let source = std::fs::read_to_string(&input).unwrap();
    let fixture = parse_fixture(name, &source);
    let scratch = std::path::Path::new(env!("OUT_DIR")).join(format!("{prefix}{name}"));
    if scratch.exists() {
        std::fs::remove_dir_all(&scratch).unwrap();
    }
    std::fs::create_dir_all(&scratch).unwrap();

    let references: Vec<_> = fixture
        .references
        .iter()
        .map(|name| {
            let output = scratch
                .join(format!("reference-{name}"))
                .with_extension("winmd");
            windows_rdl::reader()
                .input(root.join("input").join(name))
                .output(&output)
                .write()
                .unwrap();
            output
        })
        .collect();
    PreparedFixture {
        fixture,
        scratch,
        references,
    }
}

pub fn run(name: &str) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let expected = root.join("expected");
    let expected_error = expected.join(format!("{name}.error")).exists();
    let PreparedFixture {
        fixture,
        scratch,
        references,
    } = prepare(name, "");
    let forward = generate(&fixture, &scratch, &references, false, false);
    let reverse = if fixture.sources.iter().filter(|source| source.input).count() > 1 {
        generate(&fixture, &scratch, &references, true, false)
    } else {
        forward.clone()
    };
    assert_eq!(forward, reverse, "fixture `{name}` depends on input order");

    std::fs::create_dir_all(&expected).unwrap();
    match (expected_error, forward) {
        (false, Ok(actual)) => {
            let mut reader = windows_rdl::reader();
            reader
                .input_text(&actual)
                .reference_default()
                .output(scratch.join(format!("{name}.winmd")));
            for reference in &references {
                reader.reference(reference);
            }
            reader.write().unwrap();
            std::fs::write(expected.join(format!("{name}.rdl")), actual).unwrap();
        }
        (true, Err(error)) => {
            std::fs::write(expected.join(format!("{name}.error")), format!("{error}\n")).unwrap();
        }
        (false, Err(error)) => panic!("fixture `{name}` failed unexpectedly: {error}"),
        (true, Ok(_)) => panic!("fixture `{name}` unexpectedly succeeded"),
    }
}

pub fn compile(name: &str) -> windows_metadata::reader::Index {
    compile_inputs(name, false)
}

pub fn compile_permutations(name: &str) -> windows_metadata::reader::Index {
    compile_inputs(name, true)
}

fn compile_inputs(name: &str, permutations: bool) -> windows_metadata::reader::Index {
    let PreparedFixture {
        mut fixture,
        scratch,
        references,
    } = prepare(name, "projection-");
    fixture.namespace = "Test".to_string();
    let rdl = generate(&fixture, &scratch, &references, false, false).unwrap();
    if permutations {
        for (reverse, rename) in [(true, false), (false, true), (true, true)] {
            assert_eq!(
                rdl,
                generate(&fixture, &scratch, &references, reverse, rename).unwrap(),
                "fixture `{name}` depends on input order or names"
            );
        }
    }
    let output = scratch.join("test.winmd");
    let mut reader = windows_rdl::reader();
    reader.input_text(&rdl).output(&output);
    if fixture.reference_default {
        reader.reference_default();
    }
    for reference in references {
        reader.reference(reference);
    }
    reader.write().unwrap();
    windows_metadata::reader::Index::read(output).unwrap()
}

fn parse_fixture(default_name: &str, source: &str) -> Fixture {
    let mut namespace = "Test".to_string();
    let mut library = "test.dll".to_string();
    let mut args = vec!["-x".to_string(), "c++".to_string()];
    let mut filters = vec![];
    let mut reference_default = false;
    let mut references = vec![];
    let mut sources = vec![];
    let mut current = None;
    let mut default_source = String::new();
    let mut preamble = true;

    for line in source.lines() {
        if let Some(directive) = line.strip_prefix("//!").map(str::trim) {
            let section = directive
                .strip_prefix("input ")
                .map(|value| (value, true))
                .or_else(|| directive.strip_prefix("file ").map(|value| (value, false)));
            if let Some((value, input)) = section {
                if let Some(source) = current.take() {
                    sources.push(source);
                }
                let name = value.trim();
                let path = std::path::Path::new(name);
                assert!(
                    !name.is_empty()
                        && path.file_name().is_some_and(|file_name| file_name == name)
                        && path.extension().is_some_and(|extension| extension == "h"),
                    "fixture source `{name}` must be a .h file"
                );
                current = Some(FixtureSource {
                    name: name.to_string(),
                    source: String::new(),
                    input,
                });
                preamble = false;
                continue;
            }
            if preamble {
                if let Some(value) = directive.strip_prefix("namespace ") {
                    namespace = value.trim().to_string();
                } else if let Some(value) = directive.strip_prefix("library ") {
                    library = value.trim().to_string();
                } else if let Some(value) = directive.strip_prefix("args ") {
                    args = value.split_whitespace().map(str::to_string).collect();
                } else if let Some(value) = directive.strip_prefix("filter ") {
                    filters.push(value.trim().to_string());
                } else if directive == "reference-default" {
                    reference_default = true;
                } else if let Some(value) = directive.strip_prefix("reference ") {
                    let name = value.trim();
                    let path = std::path::Path::new(name);
                    assert!(
                        path.file_name().is_some_and(|file_name| file_name == name)
                            && path.extension().is_some_and(|extension| extension == "rdl"),
                        "fixture reference `{name}` must be a .rdl file"
                    );
                    references.push(name.to_string());
                } else {
                    panic!("unknown fixture directive `{directive}`");
                }
                continue;
            }
        }

        preamble = false;
        let target = current
            .as_mut()
            .map_or(&mut default_source, |input| &mut input.source);
        target.push_str(line);
        target.push('\n');
    }

    if let Some(source) = current {
        sources.push(source);
    } else {
        sources.push(FixtureSource {
            name: format!("{default_name}.h"),
            source: default_source,
            input: true,
        });
    }
    let mut names = std::collections::BTreeSet::new();
    for source in &sources {
        assert!(
            names.insert(&source.name),
            "duplicate fixture source `{}`",
            source.name
        );
        assert!(
            !source.source.trim().is_empty(),
            "fixture source `{}` is empty",
            source.name
        );
    }
    assert!(
        sources.iter().any(|source| source.input),
        "fixture has no translation units"
    );

    Fixture {
        sources,
        namespace,
        library,
        args,
        filters,
        reference_default,
        references,
    }
}

fn generate(
    fixture: &Fixture,
    scratch: &std::path::Path,
    references: &[std::path::PathBuf],
    reverse: bool,
    rename: bool,
) -> Result<String, String> {
    let names: Vec<_> = fixture
        .sources
        .iter()
        .filter(|source| source.input)
        .map(|source| source.name.as_str())
        .collect();
    let renamed = |name: &str| {
        if rename && let Some(index) = names.iter().position(|original| *original == name) {
            names[names.len() - index - 1].to_string()
        } else {
            name.to_string()
        }
    };
    let mut inputs = vec![];
    for source in &fixture.sources {
        let path = scratch.join(renamed(&source.name));
        let text = source
            .source
            .lines()
            .map(|line| {
                if let Some(include) = line.trim_start().strip_prefix("#include")
                    && let Some(include) = include.trim_start().strip_prefix('"')
                    && let Some((name, _)) = include.split_once('"')
                {
                    line.replacen(&format!("\"{name}\""), &format!("\"{}\"", renamed(name)), 1)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(&path, format!("{text}\n")).unwrap();
        if source.input {
            inputs.push(path);
        }
    }
    if reverse {
        inputs.reverse();
    }
    let output = scratch.join(if reverse {
        "reverse.rdl"
    } else {
        "forward.rdl"
    });
    let mut clang = windows_clang::clang();
    clang
        .inputs(&inputs)
        .args(&fixture.args)
        .namespace(&fixture.namespace)
        .library(&fixture.library)
        .output(&output);
    if fixture.reference_default {
        clang.reference_default();
    }
    for filter in &fixture.filters {
        let filter = names
            .iter()
            .find_map(|name| {
                filter
                    .strip_suffix(name)
                    .map(|prefix| format!("{prefix}{}", renamed(name)))
            })
            .unwrap_or_else(|| filter.clone());
        clang.filter(filter);
    }
    for reference in references {
        clang.reference(reference);
    }
    match clang.write() {
        Ok(()) => Ok(std::fs::read_to_string(output).unwrap()),
        Err(error) => {
            let scratch = scratch.to_string_lossy();
            let normalized = error
                .to_string()
                .replace(scratch.as_ref(), "<input>")
                .replace(&scratch.replace('\\', "/"), "<input>");
            Err(normalized)
        }
    }
}
