use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub fn worker(test: &str) -> bool {
    if std::env::var_os("WINDOWS_CLANG2_LAYOUT_WORKER").is_some() {
        return false;
    }
    let native = PathBuf::from(std::env::var_os("WINDOWS_CLANG2_LAYOUT_NATIVE").unwrap());
    let log = Path::new(env!("OUT_DIR")).join(format!("{test}_worker.log"));
    let file = std::fs::File::create(&log).unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test, "--nocapture"])
        .env("LIBCLANG_PATH", &native)
        .env("WINDOWS_CLANG2_LAYOUT_WORKER", "1")
        .stdout(Stdio::from(file.try_clone().unwrap()))
        .stderr(Stdio::from(file))
        .spawn()
        .unwrap();
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            assert!(
                status.success(),
                "{}",
                std::fs::read_to_string(&log).unwrap()
            );
            return true;
        }
        if started.elapsed() >= Duration::from_secs(20) {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("native worker deadline; {}", log.display());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
