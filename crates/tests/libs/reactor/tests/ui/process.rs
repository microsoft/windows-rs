use super::*;
use std::fs::{self, File};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

static SERIAL: Mutex<()> = Mutex::new(());
static NEXT_LOG: AtomicU32 = AtomicU32::new(0);

struct Host {
    child: Child,
    directory: PathBuf,
}

impl Host {
    fn spawn(fixture: &str) -> Result<Self> {
        let directory = loop {
            let directory = std::env::temp_dir().join(format!(
                "reactor-ui-{}-{}",
                std::process::id(),
                NEXT_LOG.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&directory) {
                Ok(()) => break directory,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error.into()),
            }
        };
        let child = Command::new(env!("CARGO_BIN_EXE_test-reactor"))
            .arg(fixture)
            .stdin(Stdio::null())
            .stdout(File::create(directory.join("stdout.txt"))?)
            .stderr(File::create(directory.join("stderr.txt"))?)
            .spawn()?;
        eprintln!("fixture {fixture}: pid {}", child.id());
        Ok(Self { child, directory })
    }

    fn wait(&mut self, deadline: Instant) -> Result<ExitStatus> {
        loop {
            if let Some(status) = self.child.try_wait()? {
                if !status.success() {
                    return Err(format!("fixture exited with {status}").into());
                }
                return Ok(status);
            }
            if Instant::now() >= deadline {
                return Err("fixture timed out".into());
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn stop(&mut self) -> Result<()> {
        if self.child.try_wait()?.is_none() {
            if let Err(error) = self.child.kill()
                && self.child.try_wait()?.is_none()
            {
                return Err(error.into());
            }
            self.child.wait()?;
        }
        Ok(())
    }

    fn output(&self) -> Result<String> {
        let stdout = fs::read(self.directory.join("stdout.txt"))?;
        let stderr = fs::read(self.directory.join("stderr.txt"))?;
        Ok(format!(
            "logs: {}\nstdout:\n{}\nstderr:\n{}",
            self.directory.display(),
            String::from_utf8_lossy(&stdout),
            String::from_utf8_lossy(&stderr),
        ))
    }

    fn remove_logs(&self) -> Result<()> {
        fs::remove_file(self.directory.join("stdout.txt"))?;
        fs::remove_file(self.directory.join("stderr.txt"))?;
        fs::remove_dir(&self.directory)?;
        Ok(())
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        if let Err(error) = self.stop() {
            eprintln!("failed to stop fixture {}: {error}", self.child.id());
        }
    }
}

pub fn run(fixture: &'static str, test: fn(&Automation) -> Result<()>) -> Result<()> {
    // Serialize this suite even when Cargo uses its default test-thread count.
    let _serial = SERIAL.lock().unwrap();
    let mut host = Host::spawn(fixture)?;
    match drive(&mut host, fixture, test) {
        Ok(()) => host.remove_logs(),
        Err(error) => {
            if let Err(cleanup) = host.stop() {
                eprintln!("fixture cleanup also failed: {cleanup}");
            }
            Err(format!("{fixture}: {error}\n{}", host.output()?).into())
        }
    }
}

fn drive(
    host: &mut Host,
    fixture: &'static str,
    test: fn(&Automation) -> Result<()>,
) -> Result<()> {
    let process = host.child.id();
    let cancelled = Arc::new(AtomicBool::new(false));
    let driver_cancelled = cancelled.clone();
    let driver = std::thread::spawn(move || {
        let _com = Com::new()?;
        let ui = Automation::new(process, fixture, driver_cancelled)?;
        test(&ui)?;
        ui.window_handle()
    });
    let deadline = Instant::now() + Duration::from_secs(30);
    let result: Result<()> = (|| {
        while !driver.is_finished() {
            if let Some(status) = host.child.try_wait()? {
                return Err(format!("fixture exited before the UI test finished: {status}").into());
            }
            if Instant::now() >= deadline {
                return Err("UI test timed out (including UI Automation calls)".into());
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    })();
    if let Err(error) = result {
        cancelled.store(true, Ordering::Relaxed);
        host.stop()?;
        // Killing the provider normally releases an outstanding UIA call. Never block on join.
        let cleanup = Instant::now() + Duration::from_secs(5);
        while !driver.is_finished() && Instant::now() < cleanup {
            std::thread::sleep(Duration::from_millis(10));
        }
        if driver.is_finished() {
            match driver.join() {
                Ok(Ok(_)) => {}
                Ok(Err(error)) => eprintln!("UI worker also failed: {error}"),
                Err(_) => eprintln!("UI worker also panicked"),
            }
        } else {
            eprintln!("UI Automation worker did not finish after its provider was terminated");
        }
        return Err(error);
    }
    let window = driver.join().map_err(|panic| {
        let message = panic
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| panic.downcast_ref::<&str>().copied())
            .unwrap_or("non-string panic");
        format!("UI assertion panicked: {message}")
    })??;
    unsafe {
        let window = window as HWND;
        let mut owner = 0;
        GetWindowThreadProcessId(window, &mut owner);
        if owner != process {
            return Err("refusing to close another process's window".into());
        }
        PostMessageW(window, WM_CLOSE as u32, 0, 0).ok()?;
    }
    host.wait(deadline)?;
    Ok(())
}

#[test]
fn child_panic_is_reported() -> Result<()> {
    let mut host = Host::spawn("panic")?;
    let error = host
        .wait(Instant::now() + Duration::from_secs(10))
        .unwrap_err();
    assert!(error.to_string().contains("fixture exited with"), "{error}");
    let output = host.output()?;
    assert!(output.contains("UI fixture panic probe stdout"));
    assert!(output.contains("UI fixture panic probe stderr"));
    host.remove_logs()
}

#[test]
fn hung_child_is_terminated() -> Result<()> {
    let mut host = Host::spawn("hang")?;
    let startup = Instant::now() + Duration::from_secs(10);
    while !fs::read_to_string(host.directory.join("stderr.txt"))?.contains("UI fixture hang probe")
    {
        assert!(
            host.child.try_wait()?.is_none() && Instant::now() < startup,
            "hang probe did not start: {}",
            host.output()?
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    let error = host
        .wait(Instant::now() + Duration::from_millis(100))
        .unwrap_err();
    assert_eq!(error.to_string(), "fixture timed out");
    host.stop()?;
    assert!(host.child.try_wait()?.is_some());
    assert!(host.output()?.contains("UI fixture hang probe"));
    host.remove_logs()
}

#[test]
fn early_successful_exit_is_rejected() -> Result<()> {
    let serial = SERIAL.lock().unwrap();
    let mut host = Host::spawn("exit")?;
    let result = drive(&mut host, "exit", |ui| ui.value("input", ""));
    host.stop()?;
    drop(serial);
    let error = result.unwrap_err();
    assert!(
        error
            .to_string()
            .contains("fixture exited before the UI test finished: exit code: 0"),
        "{error}"
    );
    host.remove_logs()
}
