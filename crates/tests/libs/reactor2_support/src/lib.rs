//! Shared deployment and process support for Reactor Cargo tests.

use std::ffi::OsStr;
use std::fmt;
use std::io::{self, Read};
use std::process::{Command, ExitStatus, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

mod automation;

pub use automation::{Automation, AutomationError, Element};

pub enum ProcessError {
    Io(io::Error),
    ReaderPanicked,
    Timeout { stdout: Vec<u8>, stderr: Vec<u8> },
}

impl fmt::Debug for ProcessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(formatter),
            Self::ReaderPanicked => formatter.write_str("child output reader panicked"),
            Self::Timeout { stdout, stderr } => write!(
                formatter,
                "child process timed out\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(stdout),
                String::from_utf8_lossy(stderr)
            ),
        }
    }
}

impl std::error::Error for ProcessError {}

impl From<io::Error> for ProcessError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn run_test_process(
    fixture: &str,
    environment: &[(&str, &str)],
    timeout: Duration,
) -> Result<Output, ProcessError> {
    TestProcess::spawn(fixture, environment)?.wait(timeout)
}

pub struct TestProcess {
    child: std::process::Child,
    stdout_reader: Option<thread::JoinHandle<io::Result<Vec<u8>>>>,
    stderr_reader: Option<thread::JoinHandle<io::Result<Vec<u8>>>>,
}

impl TestProcess {
    pub fn spawn(fixture: &str, environment: &[(&str, &str)]) -> Result<Self, ProcessError> {
        let mut command = Command::new(std::env::current_exe()?);
        command
            .arg(fixture)
            .arg("--exact")
            .arg("--nocapture")
            .arg("--test-threads=1")
            .envs(environment.iter().copied());
        Self::spawn_command(&mut command)
    }

    pub fn spawn_executable(
        executable: impl AsRef<OsStr>,
        arguments: &[&str],
        environment: &[(&str, &str)],
    ) -> Result<Self, ProcessError> {
        let mut command = Command::new(executable);
        command.args(arguments).envs(environment.iter().copied());
        Self::spawn_command(&mut command)
    }

    fn spawn_command(command: &mut Command) -> Result<Self, ProcessError> {
        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn()?;
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        Ok(Self {
            child,
            stdout_reader: Some(thread::spawn(move || read_all(stdout))),
            stderr_reader: Some(thread::spawn(move || read_all(stderr))),
        })
    }

    pub fn id(&self) -> u32 {
        self.child.id()
    }

    pub fn wait(mut self, timeout: Duration) -> Result<Output, ProcessError> {
        let deadline = Instant::now() + timeout;
        let status = loop {
            if let Some(status) = self.child.try_wait()? {
                break status;
            }
            if Instant::now() >= deadline {
                self.child.kill()?;
                let _ = self.child.wait();
                let stdout = self.join_stdout()?;
                let stderr = self.join_stderr()?;
                return Err(ProcessError::Timeout { stdout, stderr });
            }
            thread::sleep(Duration::from_millis(10));
        };

        // DWM and UI Automation can retain desktop objects briefly after the process exits.
        thread::sleep(Duration::from_millis(200));

        Ok(Output {
            status,
            stdout: self.join_stdout()?,
            stderr: self.join_stderr()?,
        })
    }

    fn join_stdout(&mut self) -> Result<Vec<u8>, ProcessError> {
        join_reader(self.stdout_reader.take().unwrap())
    }

    fn join_stderr(&mut self) -> Result<Vec<u8>, ProcessError> {
        join_reader(self.stderr_reader.take().unwrap())
    }
}

impl Drop for TestProcess {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
        if let Some(reader) = self.stdout_reader.take() {
            let _ = reader.join();
        }
        if let Some(reader) = self.stderr_reader.take() {
            let _ = reader.join();
        }
    }
}

pub fn assert_success(output: Output) {
    assert_process_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_test_ran(&stdout, &stderr);
}

pub fn assert_process_success(output: &Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "child process exited with {}\nstdout:\n{}\nstderr:\n{}",
        display_status(output.status),
        stdout,
        stderr
    );
}

fn assert_test_ran(stdout: &str, stderr: &str) {
    assert!(
        stdout.contains("running 1 test") && stdout.contains("1 passed"),
        "child process did not run exactly one test\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

fn read_all(mut reader: impl Read) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn join_reader(reader: thread::JoinHandle<io::Result<Vec<u8>>>) -> Result<Vec<u8>, ProcessError> {
    reader
        .join()
        .map_err(|_| ProcessError::ReaderPanicked)?
        .map_err(ProcessError::Io)
}

fn display_status(status: ExitStatus) -> String {
    status
        .code()
        .map_or_else(|| status.to_string(), |code| code.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_fixture_requires_one_executed_test() {
        assert_test_ran(
            "running 1 test\ntest fixture ... ok\n\ntest result: ok. 1 passed; 0 failed",
            "",
        );
    }

    #[test]
    #[should_panic(expected = "child process did not run exactly one test")]
    fn unknown_exact_fixture_cannot_pass() {
        assert_test_ran(
            "running 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored",
            "",
        );
    }
}
