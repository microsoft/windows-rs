use std::path::Path;
use std::process::Command;
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn native_reference_metadata() {
    generate("native-reference-metadata");
}

#[test]
#[ignore = "requires the pinned Clang executable and three MSVC Rust targets"]
fn native_reference_consumers() {
    let (root, scratch, winmd) = generate("native-reference-consumers");
    let host = if cfg!(target_arch = "aarch64") {
        Target::Arm64
    } else {
        Target::X64
    };
    let compiler = root
        .join("target\\tool-clang\\native")
        .join(helpers::LIBCLANG_VERSION)
        .join(host.triple())
        .join("llvm\\bin\\clang.exe");
    for target in TARGETS {
        let object = scratch.join(format!("{}.obj", target.triple()));
        checked(
            Command::new(&compiler)
                .args([
                    "-target",
                    target.triple(),
                    "-std=c++17",
                    "-fno-exceptions",
                    "-fno-rtti",
                    "-c",
                ])
                .arg(test_clang2::input_dir().join("native_references.cpp"))
                .arg("-o")
                .arg(&object),
        );
        for sys in [true, false] {
            let consumer = scratch.join(format!("{}-{sys}", target.triple()));
            std::fs::create_dir_all(consumer.join("src")).unwrap();
            let mut builder = windows_bindgen::builder();
            builder
                .input(&winmd)
                .flat()
                .output(consumer.join("src\\bindings.rs"));
            if sys {
                builder.filter("Fixture").sys().extern_fns();
            } else {
                builder.filter("Fixture.IReferences");
            }
            builder.write();
            std::fs::copy(
                test_clang2::input_dir().join("native_references_consumer.rs"),
                consumer.join("src\\main.rs"),
            )
            .unwrap();
            let manifest = consumer.join("Cargo.toml");
            std::fs::write(
                &manifest,
                format!(
                    "[package]\nname = \"native-reference-consumer\"\nversion = \"0.0.0\"\n\
                     edition = \"2024\"\n[workspace]\n[dependencies]\n\
                     windows-core = {{ path = '{}' }}\n",
                    root.join("crates\\libs\\core").display()
                ),
            )
            .unwrap();
            let mut cargo = Command::new("cargo");
            cargo
                .args(["rustc", "--quiet", "--offline", "--manifest-path"])
                .arg(&manifest)
                .args(["--target", target.triple(), "--target-dir"])
                .arg(scratch.join("build"))
                .args(["--", "--check-cfg=cfg(raw_bindings)"]);
            if sys {
                cargo.arg("--cfg=raw_bindings");
            }
            checked(cargo.arg(format!("-Clink-arg={}", object.display())));
            if target == host || host == Target::X64 && target == Target::X86 {
                checked(&mut Command::new(
                    scratch
                        .join("build")
                        .join(target.triple())
                        .join("debug\\native-reference-consumer.exe"),
                ));
            }
        }
    }
}

fn generate(name: &str) -> (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf) {
    helpers::ensure_libclang();
    let input = test_clang2::fixture("native_references");
    let imports = projection::Imports::from_iter(
        [
            "ApplyReference",
            "InvokeReference",
            "InvokeAnonymous",
            "ReferenceObject",
            "InvokeObject",
        ]
        .map(|name| {
            (
                name.into(),
                projection::Import {
                    library: "reference.dll".into(),
                    name: None,
                },
            )
        }),
    );
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output = projection::project_with_options(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &projection::Options {
            imports: Some(&imports),
            ..Default::default()
        },
    )
    .unwrap();
    let rdl = &output.partitions["native_references"].rdl;
    let golden = test_clang2::input_dir().join("native_references.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&golden, rdl).unwrap();
    }
    assert_eq!(
        *rdl,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target").join(name);
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = scratch.join("roundtrip.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(rewritten).unwrap(), *rdl);
    (root, scratch, winmd)
}

fn checked(command: &mut Command) {
    let result = command.output().unwrap();
    assert!(
        result.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}
