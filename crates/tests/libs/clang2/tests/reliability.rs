use std::ffi::{CStr, CString};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};
use windows_clang2::{Compiler, Input, Stage, Target};

const ROUNDS: usize = 8;
const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];
const CAPTURE_CASES: &[&str] = &[
    "serial",
    "reload-serial",
    "shared",
    "independent",
    "cold-independent",
    "mixed-shared",
    "mixed-independent",
    "native-serial",
];
const NATIVE_CASES: &[&str] = &[
    "native-serial",
    "native-shared",
    "native-independent",
    "native-cold",
    "native-warm",
];

fn profile(target: Target) -> Input {
    let mut input = Input::new("records.cpp", include_str!("../input/winnt.h"), target);
    input.required.extend(["POINT".into(), "RECT".into()]);
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap();
    let sdk = std::env::var_os("WINDOWS_CLANG2_SDK_INCLUDE").map_or_else(||
        PathBuf::from(std::env::var_os("USERPROFILE").unwrap())
            .join(".nuget\\packages\\microsoft.windows.sdk.cpp\\10.0.28000.2270\\c\\Include\\10.0.28000.0"), PathBuf::from);
    let resource = std::env::var_os("CLANG_RESOURCE_DIR").map_or_else(
        || repository.join("target\\tool-clang\\clang-resource\\22.1.8"),
        PathBuf::from,
    );
    input
        .arguments
        .extend(["-resource-dir".into(), resource.to_str().unwrap().into()]);
    for directory in ["ucrt", "um", "shared"] {
        input
            .arguments
            .push(format!("-I{}", sdk.join(directory).display()));
    }
    input.roots.push(sdk.join("shared\\windef.h"));
    input
}

fn triple(target: Target) -> &'static str {
    match target {
        Target::X86 => "i686-pc-windows-msvc",
        Target::X64 => "x86_64-pc-windows-msvc",
        Target::Arm64 => "aarch64-pc-windows-msvc",
    }
}

fn capture(compiler: &Compiler, target: Target, round: usize) {
    println!("capture-start {target:?} {round}");
    let snapshot = compiler.capture(&profile(target)).unwrap();
    println!(
        "capture-end {target:?} {round} {} {:?}",
        snapshot.compiler, snapshot.arguments
    );
    assert_eq!(
        snapshot.rdl("Test").unwrap(),
        include_str!("../input/sdk_records.rdl").replace("\r\n", "\n")
    );
    println!("output-end {target:?} {round}");
}

fn short_sources(compiler: Option<&Compiler>) {
    for round in 0..8 {
        let owned = compiler.is_none().then(|| Compiler::new().unwrap());
        let compiler = compiler.or(owned.as_ref()).unwrap();
        for (source, failed) in [
            ("typedef int Value;", None),
            ("typedef int Value; broken source", Some(Stage::Source)),
            ("struct Value { int number; };", None),
            ("struct Value {};", Some(Stage::Capture)),
            (
                "struct Value { unsigned member : 3; };",
                Some(Stage::Capture),
            ),
            (
                "struct Value { union { int a; float b; }; };",
                Some(Stage::Capture),
            ),
            (
                "struct Value { struct Nested { int member; } nested; };",
                Some(Stage::Capture),
            ),
            (
                "struct Base { int member; }; struct Value : Base { int other; };",
                Some(Stage::Capture),
            ),
            (
                "struct Value { int member; void Method(); };",
                Some(Stage::Capture),
            ),
            ("class Value { int member; };", Some(Stage::Capture)),
            ("struct Value { int &member; };", Some(Stage::Capture)),
            ("struct Value { int data[]; };", Some(Stage::Capture)),
            (
                "struct Value { int member __attribute__((annotate(\"contract\"))); };",
                Some(Stage::Capture),
            ),
            (
                "struct __declspec(align(16)) Value { int member; };",
                Some(Stage::Capture),
            ),
            (
                "#pragma pack(push, 1)\nstruct Value { char a; int b; };\n#pragma pack(pop)",
                Some(Stage::Representation),
            ),
            (
                "struct Value { int data[0]; };",
                Some(Stage::Representation),
            ),
        ] {
            let mut input = Input::new("records.cpp", source, Target::X64);
            input.required.insert("Value".into());
            println!("short-start {round} {source}");
            let result = compiler.capture(&input);
            if let Some(stage) = failed {
                if stage == Stage::Representation {
                    assert_eq!(result.unwrap().rdl("Test").unwrap_err().stage, stage);
                } else {
                    assert_eq!(result.unwrap_err().stage, stage);
                }
            } else {
                result.unwrap().rdl("Test").unwrap();
            }
            println!("short-end {round}");
        }
    }
}

fn capture_case(mode: &str) {
    let compiler = (!mode.ends_with("independent") && mode != "reload-serial")
        .then(|| Arc::new(Compiler::new().unwrap()));
    if mode == "serial" || mode == "reload-serial" {
        for round in 0..ROUNDS {
            for target in TARGETS {
                if let Some(compiler) = &compiler {
                    capture(compiler, target, round);
                } else {
                    capture(&Compiler::new().unwrap(), target, round);
                }
            }
        }
        return;
    }
    let barrier = Barrier::new(TARGETS.len());
    std::thread::scope(|scope| {
        let mut tasks = Vec::new();
        for target in TARGETS {
            let compiler = compiler.as_ref().map(Arc::clone);
            let barrier = &barrier;
            tasks.push(scope.spawn(move || {
                for round in 0..ROUNDS {
                    let owned = mode
                        .ends_with("independent")
                        .then(|| Compiler::new().unwrap());
                    if mode == "cold-independent" && round == 0 {
                        barrier.wait();
                    }
                    capture(
                        compiler.as_deref().or(owned.as_ref()).unwrap(),
                        target,
                        round,
                    );
                }
            }));
        }
        if mode.starts_with("mixed-") {
            let compiler = compiler.as_ref().map(Arc::clone);
            tasks.push(scope.spawn(move || {
                short_sources(if mode.ends_with("independent") {
                    None
                } else {
                    compiler.as_deref()
                });
            }));
        }
        for task in tasks {
            task.join().unwrap();
        }
    });
}

struct NativeSession {
    index: clang_sys::CXIndex,
    unit: clang_sys::CXTranslationUnit,
    previous: Option<Arc<clang_sys::SharedLibrary>>,
}

impl Drop for NativeSession {
    fn drop(&mut self) {
        unsafe {
            if !self.unit.is_null() {
                clang_sys::clang_disposeTranslationUnit(self.unit);
            }
            if !self.index.is_null() {
                clang_sys::clang_disposeIndex(self.index);
            }
        }
        clang_sys::set_library(self.previous.take());
    }
}

fn native_parse(
    library: Arc<clang_sys::SharedLibrary>,
    target: Target,
    round: usize,
    barrier: Option<&Barrier>,
) {
    use clang_sys::*;
    let input = profile(target);
    let mut arguments = vec![
        "-x".into(),
        "c++".into(),
        "-std=c++20".into(),
        format!("--target={}", triple(target)),
        "-fms-extensions".into(),
        "-fms-compatibility".into(),
    ];
    arguments.extend(input.arguments);
    let arguments = arguments
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect::<Vec<_>>();
    let pointers = arguments.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();
    let name = CString::new(input.name).unwrap();
    let source = CString::new(input.source).unwrap();
    let mut unsaved = CXUnsavedFile {
        Filename: name.as_ptr(),
        Contents: source.as_ptr(),
        Length: source.as_bytes().len().try_into().unwrap(),
    };
    let mut session = NativeSession {
        index: std::ptr::null_mut(),
        unit: std::ptr::null_mut(),
        previous: set_library(Some(library)),
    };
    if let Some(barrier) = barrier {
        barrier.wait();
    }
    println!("native-index-start {target:?} {round}");
    session.index = unsafe { clang_createIndex(0, 0) };
    println!("native-index-end {target:?} {round}");
    assert!(!session.index.is_null());
    println!("native-parse-start {target:?} {round}");
    let status = unsafe {
        clang_parseTranslationUnit2(
            session.index,
            name.as_ptr(),
            pointers.as_ptr(),
            pointers.len().try_into().unwrap(),
            &raw mut unsaved,
            1,
            CXTranslationUnit_DetailedPreprocessingRecord,
            &raw mut session.unit,
        )
    };
    println!("native-parse-end {target:?} {round} {status}");
    assert_eq!(status, CXError_Success);
    assert!(!session.unit.is_null());
    for index in 0..unsafe { clang_getNumDiagnostics(session.unit) } {
        let diagnostic = unsafe { clang_getDiagnostic(session.unit, index) };
        let severity = unsafe { clang_getDiagnosticSeverity(diagnostic) };
        let message =
            unsafe { clang_formatDiagnostic(diagnostic, clang_defaultDiagnosticDisplayOptions()) };
        let text = unsafe { CStr::from_ptr(clang_getCString(message)) }
            .to_string_lossy()
            .into_owned();
        unsafe {
            clang_disposeString(message);
            clang_disposeDiagnostic(diagnostic);
        }
        assert!(severity < CXDiagnostic_Error, "{text}");
    }
    drop(session);
    println!("native-dispose-end {target:?} {round}");
}

#[test]
#[ignore = "subprocess worker for the deadline-controlled parser reliability matrix"]
fn parser_worker() {
    let mode = std::env::var("WINDOWS_CLANG2_REPRO_MODE").unwrap();
    assert!(CAPTURE_CASES.contains(&mode.as_str()) || NATIVE_CASES.contains(&mode.as_str()));
    println!(
        "mode {mode}; rounds {ROUNDS}; source {:?}",
        include_str!("../input/winnt.h")
    );
    for target in TARGETS {
        let input = profile(target);
        println!(
            "profile {target:?}; name {:?}; args {:?}; roots {:?}",
            input.name, input.arguments, input.roots
        );
    }
    if mode.starts_with("native-") {
        let library =
            (mode != "native-independent").then(|| Arc::new(clang_sys::load_manually().unwrap()));
        if let Some(library) = &library {
            println!("native-library {:?}", library.path());
        }
        if mode == "native-warm" {
            let mut session = NativeSession {
                index: std::ptr::null_mut(),
                unit: std::ptr::null_mut(),
                previous: clang_sys::set_library(library.as_ref().map(Arc::clone)),
            };
            session.index = unsafe { clang_sys::clang_createIndex(0, 0) };
            assert!(!session.index.is_null());
            drop(session);
        }
        if mode == "native-serial" {
            for round in 0..ROUNDS {
                for target in TARGETS {
                    native_parse(Arc::clone(library.as_ref().unwrap()), target, round, None);
                }
            }
        } else {
            let barrier = Barrier::new(TARGETS.len());
            std::thread::scope(|scope| {
                let tasks = TARGETS
                    .into_iter()
                    .map(|target| {
                        let library = library.as_ref().map(Arc::clone);
                        let barrier = &barrier;
                        let mode = &mode;
                        scope.spawn(move || {
                            for round in 0..ROUNDS {
                                let library = library.as_ref().map_or_else(
                                    || Arc::new(clang_sys::load_manually().unwrap()),
                                    Arc::clone,
                                );
                                native_parse(
                                    library,
                                    target,
                                    round,
                                    (matches!(mode.as_str(), "native-cold" | "native-warm")
                                        && round == 0)
                                        .then_some(barrier),
                                );
                            }
                        })
                    })
                    .collect::<Vec<_>>();
                for task in tasks {
                    task.join().unwrap();
                }
            });
        }
    } else {
        capture_case(&mode);
    }
}

#[test]
fn fixed_parser_matrix_has_no_stalls_or_lifecycle_failures() {
    run_matrix(CAPTURE_CASES);
}

#[test]
#[ignore = "direct native controls bypass the wrapper's LLVM registration synchronization"]
fn direct_native_registration_matrix() {
    let cases = (0..8)
        .flat_map(|_| ["native-cold", "native-warm"])
        .collect::<Vec<_>>();
    run_matrix(&cases);
}

fn run_matrix(cases: &[&str]) {
    let run = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = Path::new(env!("OUT_DIR"))
        .join("parser-reliability")
        .join(run.to_string());
    std::fs::create_dir_all(&directory).unwrap();
    let mut failures = Vec::new();
    for (case, mode) in cases.iter().enumerate() {
        let log = directory.join(format!("{case}-{mode}.log"));
        let file = File::create(&log).unwrap();
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "parser_worker", "--ignored", "--nocapture"])
            .env("WINDOWS_CLANG2_REPRO_MODE", mode)
            .stdout(Stdio::from(file.try_clone().unwrap()))
            .stderr(Stdio::from(file))
            .spawn()
            .unwrap();
        let started = Instant::now();
        loop {
            if let Some(status) = child.try_wait().unwrap() {
                println!(
                    "{mode}: {status}; {:.2}s; {}",
                    started.elapsed().as_secs_f64(),
                    log.display()
                );
                if !status.success() {
                    failures.push(format!("{mode}: {status}"));
                }
                break;
            }
            if started.elapsed() >= Duration::from_secs(20) {
                child.kill().unwrap();
                child.wait().unwrap();
                println!("{mode}: DEADLINE; log {}", log.display());
                failures.push(format!("{mode}: 20s deadline exceeded"));
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }
    assert!(
        failures.is_empty(),
        "parser reliability failures: {failures:?}; logs {}",
        directory.display()
    );
}
