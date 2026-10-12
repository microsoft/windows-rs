use super::{ROUNDS, TARGETS, profile};
use std::ffi::c_void;
use std::fs;
use std::mem::size_of;
use std::os::windows::io::AsRawHandle;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use windows_clang2::{Compiler, Target};

const DEADLINE: Duration = Duration::from_secs(20);

#[repr(C)]
#[derive(Default)]
struct BasicLimits {
    process_time: i64,
    job_time: i64,
    flags: u32,
    minimum_working_set: usize,
    maximum_working_set: usize,
    active_processes: u32,
    affinity: usize,
    priority: u32,
    scheduling: u32,
}

#[repr(C)]
#[derive(Default)]
struct Limits {
    basic: BasicLimits,
    io: [u64; 6],
    process_memory: usize,
    job_memory: usize,
    peak_process_memory: usize,
    peak_job_memory: usize,
}

#[repr(C)]
#[derive(Default)]
struct Accounting {
    user_time: i64,
    kernel_time: i64,
    period_user_time: i64,
    period_kernel_time: i64,
    faults: u32,
    total_processes: u32,
    active_processes: u32,
    terminated_processes: u32,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateJobObjectW(attributes: *const c_void, name: *const u16) -> *mut c_void;
    fn SetInformationJobObject(
        job: *mut c_void,
        class: i32,
        information: *const c_void,
        length: u32,
    ) -> i32;
    fn QueryInformationJobObject(
        job: *mut c_void,
        class: i32,
        information: *mut c_void,
        length: u32,
        returned: *mut u32,
    ) -> i32;
    fn AssignProcessToJobObject(job: *mut c_void, process: *mut c_void) -> i32;
    fn TerminateJobObject(job: *mut c_void, code: u32) -> i32;
    fn CloseHandle(handle: *mut c_void) -> i32;
}

struct Job(*mut c_void);

impl Job {
    fn new() -> Self {
        let job = Self(unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) });
        assert!(!job.0.is_null(), "{}", std::io::Error::last_os_error());
        let mut limits = Limits::default();
        // Kill-on-close contains only assigned workers and their non-breakaway descendants.
        limits.basic.flags = 0x2000;
        assert_ne!(
            unsafe {
                SetInformationJobObject(
                    job.0,
                    9,
                    (&raw const limits).cast(),
                    size_of::<Limits>().try_into().unwrap(),
                )
            },
            0,
            "{}",
            std::io::Error::last_os_error()
        );
        job
    }

    fn assign(&self, child: &mut Child) {
        if unsafe { AssignProcessToJobObject(self.0, child.as_raw_handle()) } == 0 {
            let error = std::io::Error::last_os_error();
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("assign owned PID {}: {error}", child.id());
        }
    }

    fn query<T: Default>(&self, class: i32) -> T {
        let mut value = T::default();
        assert_ne!(
            unsafe {
                QueryInformationJobObject(
                    self.0,
                    class,
                    (&raw mut value).cast(),
                    size_of::<T>().try_into().unwrap(),
                    std::ptr::null_mut(),
                )
            },
            0,
            "{}",
            std::io::Error::last_os_error()
        );
        value
    }

    fn terminate(&self, children: &mut [Child]) {
        assert_ne!(unsafe { TerminateJobObject(self.0, 1) }, 0);
        for child in children {
            child.wait().unwrap();
        }
        let started = Instant::now();
        while self.query::<Accounting>(1).active_processes != 0 {
            assert!(started.elapsed() < Duration::from_secs(5));
            std::thread::sleep(Duration::from_millis(5));
        }
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

fn wait_file(path: &Path) {
    let started = Instant::now();
    while !path.is_file() {
        assert!(
            started.elapsed() < DEADLINE,
            "waiting for {}",
            path.display()
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn workload(compiler: &Compiler, target: Target, directory: &Path) {
    let directory = directory.join(format!("{target:?}"));
    fs::create_dir_all(&directory).unwrap();
    for round in 0..ROUNDS {
        for records in [false, true] {
            let mut input = profile(target);
            let (name, golden) = if records {
                ("records", include_str!("../../input/sdk_records.rdl"))
            } else {
                input.required = [
                    "CHAR",
                    "SHORT",
                    "LONG",
                    "ULONG",
                    "LONGLONG",
                    "ULONGLONG",
                    "WCHAR",
                    "PVOID",
                ]
                .map(String::from)
                .into();
                input.roots = vec![
                    input.roots[0]
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join("um\\winnt.h"),
                    input.roots[0].parent().unwrap().join("minwindef.h"),
                ];
                ("scalars", include_str!("../../input/winnt.rdl"))
            };
            let snapshot = compiler.capture(&input).unwrap();
            let rdl = snapshot.rdl("Test").unwrap();
            assert_eq!(rdl, golden.replace("\r\n", "\n"));
            let stem = format!("{name}-{round}");
            fs::write(directory.join(format!("{stem}.rdl")), &rdl).unwrap();
            let output = directory.join(format!("{stem}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
                .output(&output)
                .write()
                .unwrap();
            windows_metadata::reader::Index::read(&output)
                .unwrap()
                .validate_native_contracts(None)
                .unwrap();
        }
    }
    fs::write(directory.join("done"), "").unwrap();
}

fn worker_command(directory: &Path, mode: &str) -> Command {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", "execution::worker", "--ignored", "--nocapture"])
        .env("WINDOWS_CLANG2_EXECUTION_DIRECTORY", directory)
        .env("WINDOWS_CLANG2_EXECUTION_MODE", mode);
    command
}

#[test]
#[ignore = "owned subprocess for the bounded execution-model gate"]
fn worker() {
    let directory = PathBuf::from(std::env::var_os("WINDOWS_CLANG2_EXECUTION_DIRECTORY").unwrap());
    let mode = std::env::var("WINDOWS_CLANG2_EXECUTION_MODE").unwrap();
    if mode == "descendant" {
        fs::write(
            directory.join("descendant-ready"),
            std::process::id().to_string(),
        )
        .unwrap();
        loop {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    let started = Instant::now();
    let compiler = Compiler::new().unwrap();
    let ready = directory.join(format!("ready-{mode}"));
    let pending = directory.join(format!("ready-{mode}.pending"));
    fs::write(&pending, started.elapsed().as_secs_f64().to_string()).unwrap();
    fs::rename(pending, ready).unwrap();
    // The parent assigns the job before releasing this gate, including before child creation.
    wait_file(&directory.join("go"));
    let stage = directory.join("stage");
    if mode == "shared" {
        std::thread::scope(|scope| {
            let tasks = TARGETS.map(|target| {
                let compiler = &compiler;
                let stage = &stage;
                scope.spawn(move || workload(compiler, target, stage))
            });
            for task in tasks {
                task.join().unwrap();
            }
        });
    } else {
        let target = match mode.as_str() {
            "X86" => Target::X86,
            "X64" => Target::X64,
            "Arm64" => Target::Arm64,
            _ => panic!("unknown worker {mode}"),
        };
        if target == Target::X86 && directory.join("fault").exists() {
            for peer in ["X64", "Arm64"] {
                wait_file(&stage.join(peer).join("done"));
            }
            fs::write(directory.join("fault-reached"), "").unwrap();
            if fs::read_to_string(directory.join("fault")).unwrap() == "fail" {
                std::process::exit(7);
            }
            let mut descendant = worker_command(&directory, "descendant").spawn().unwrap();
            wait_file(&directory.join("descendant-ready"));
            descendant.wait().unwrap();
            panic!("hung descendant exited");
        }
        workload(&compiler, target, &stage);
    }
}

#[derive(Debug)]
struct Measurement {
    startup: f64,
    work: f64,
    wall: f64,
    peak_private_mib: f64,
    library_initialization: Vec<f64>,
}

fn verify_complete(stage: &Path) {
    for target in TARGETS {
        let directory = stage.join(format!("{target:?}"));
        assert!(directory.join("done").is_file());
        for round in 0..ROUNDS {
            for name in ["records", "scalars"] {
                for extension in ["rdl", "winmd"] {
                    assert!(
                        directory
                            .join(format!("{name}-{round}.{extension}"))
                            .is_file()
                    );
                }
            }
        }
    }
}

fn batch(directory: &Path, isolated: bool, fault: Option<&str>) -> Result<Measurement, String> {
    fs::create_dir_all(directory).unwrap();
    if let Some(fault) = fault {
        fs::write(directory.join("fault"), fault).unwrap();
    }
    let job = Job::new();
    let started = Instant::now();
    let modes = if isolated {
        vec!["X86", "X64", "Arm64"]
    } else {
        vec!["shared"]
    };
    let mut children = Vec::new();
    for mode in &modes {
        let file = fs::File::create(directory.join(format!("{mode}.log"))).unwrap();
        let mut child = worker_command(directory, mode)
            .stdout(Stdio::from(file.try_clone().unwrap()))
            .stderr(Stdio::from(file))
            .spawn()
            .unwrap();
        job.assign(&mut child);
        children.push(child);
    }
    let mut released = None;
    let mut library_initialization = Vec::new();
    let error = loop {
        if children.iter_mut().any(|child| {
            child
                .try_wait()
                .unwrap()
                .is_some_and(|status| !status.success())
        }) {
            break "failed worker";
        }
        if started.elapsed() >= DEADLINE {
            break "batch deadline";
        }
        if released.is_none()
            && modes
                .iter()
                .all(|mode| directory.join(format!("ready-{mode}")).is_file())
        {
            library_initialization = modes
                .iter()
                .map(|mode| {
                    fs::read_to_string(directory.join(format!("ready-{mode}")))
                        .unwrap()
                        .parse()
                        .unwrap()
                })
                .collect();
            fs::write(directory.join("go"), "").unwrap();
            released = Some(Instant::now());
        }
        if children
            .iter_mut()
            .all(|child| child.try_wait().unwrap().is_some())
        {
            let completed = Instant::now();
            assert_eq!(job.query::<Accounting>(1).active_processes, 0);
            let stage = directory.join("stage");
            verify_complete(&stage);
            fs::rename(stage, directory.join("published")).unwrap();
            let released = released.unwrap();
            return Ok(Measurement {
                startup: released.duration_since(started).as_secs_f64(),
                work: completed.duration_since(released).as_secs_f64(),
                wall: started.elapsed().as_secs_f64(),
                peak_private_mib: job.query::<Limits>(9).peak_job_memory as f64 / 1048576.0,
                library_initialization,
            });
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    job.terminate(&mut children);
    if directory.join("stage").exists() {
        fs::remove_dir_all(directory.join("stage")).unwrap();
    }
    Err(error.into())
}

fn run_directory() -> PathBuf {
    let run = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    Path::new(env!("OUT_DIR"))
        .join("execution-model")
        .join(run.to_string())
}

#[test]
fn fixed_sdk_execution_models_have_identical_target_outputs() {
    let directory = run_directory();
    println!("execution artifacts {}", directory.display());
    for repetition in 0..3 {
        // Reverse the order in the middle pair to limit cache/order bias.
        let order = if repetition == 1 {
            [true, false]
        } else {
            [false, true]
        };
        for isolated in order {
            let model = if isolated { "isolated" } else { "shared" };
            let root = directory.join(repetition.to_string()).join(model);
            let measurement = batch(&root, isolated, None).unwrap();
            println!("{repetition} {model}: {measurement:?}");
            fs::write(root.join("measurement.txt"), format!("{measurement:#?}")).unwrap();
            assert!(measurement.startup > 0.0);
            assert!(measurement.work > 0.0);
            assert!(measurement.wall >= measurement.startup + measurement.work);
            assert!(measurement.peak_private_mib > 0.0);
            assert_eq!(
                measurement.library_initialization.len(),
                if isolated { 3 } else { 1 }
            );
        }
        for target in TARGETS {
            for round in 0..ROUNDS {
                for name in ["records", "scalars"] {
                    for extension in ["rdl", "winmd"] {
                        let relative =
                            PathBuf::from(format!("{target:?}\\{name}-{round}.{extension}"));
                        let read = |model| {
                            fs::read(
                                directory
                                    .join(repetition.to_string())
                                    .join(model)
                                    .join("published")
                                    .join(&relative),
                            )
                            .unwrap()
                        };
                        assert_eq!(read("shared"), read("isolated"), "{}", relative.display());
                    }
                }
            }
        }
    }
}

#[test]
fn failed_and_hung_workers_are_contained_without_partial_publication() {
    let directory = run_directory();
    println!("containment artifacts {}", directory.display());
    for fault in ["fail", "hang"] {
        let root = directory.join(fault);
        let published = root.join("published");
        fs::create_dir_all(&published).unwrap();
        fs::write(published.join("previous"), "unchanged").unwrap();
        let started = Instant::now();
        let error = batch(&root, true, Some(fault)).unwrap_err();
        assert_eq!(
            error,
            if fault == "fail" {
                "failed worker"
            } else {
                "batch deadline"
            }
        );
        assert!(started.elapsed() < DEADLINE + Duration::from_secs(5));
        assert!(root.join("fault-reached").is_file());
        assert!(!root.join("stage").exists());
        assert_eq!(
            fs::read_to_string(published.join("previous")).unwrap(),
            "unchanged"
        );
        assert_eq!(fs::read_dir(&published).unwrap().count(), 1);
        if fault == "hang" {
            assert!(root.join("descendant-ready").is_file());
        }
        println!("{fault}: {error}; tree reaped; previous publication unchanged");
    }
}
