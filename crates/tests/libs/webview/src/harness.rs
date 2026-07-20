//! The self-test harness: owns the live WebView2 objects and provides the
//! synchronous pump-and-wait helpers fixtures build on.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use windows_core::Result;
use windows_webview::*;
use windows_window::Window;

/// How long a single wait may pump before it is treated as a failure. Generous
/// enough for cold WebView2 start-up, short enough to never hang CI.
const TIMEOUT: Duration = Duration::from_secs(15);

/// Owns the window, environment, controller, and webview for the run, plus the
/// running failure count. The window, controller, and webview are created once
/// and reused across fixtures; [`reset`](Self::reset) returns the page to a
/// blank document between fixtures.
pub struct Harness {
    environment: Environment,
    _window: Window,
    controller: Controller,
    webview: WebView,
    failures: Cell<u32>,
}

impl Harness {
    /// Creates the host window and the live WebView2 objects. Returns an error
    /// if the WebView2 runtime is unavailable, which the caller reports as a
    /// TAP skip.
    pub fn bootstrap(title: &str) -> Result<Self> {
        let window = Window::new(title).size(1024, 768).create()?;
        let environment = Environment::new()?;
        let controller = environment.create_controller(&window)?;
        let (width, height) = window.client_size();
        controller.set_bounds(0, 0, width, height)?;
        let webview = controller.webview()?;

        Ok(Self {
            environment,
            _window: window,
            controller,
            webview,
            failures: Cell::new(0),
        })
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    pub fn controller(&self) -> &Controller {
        &self.controller
    }

    pub fn webview(&self) -> &WebView {
        &self.webview
    }

    pub fn failures(&self) -> u32 {
        self.failures.get()
    }

    /// Records an assertion. A false `condition` increments the failure count
    /// and prints a diagnostic; the enclosing fixture is then reported as
    /// `not ok`.
    pub fn check(&self, name: &str, condition: bool) {
        if !condition {
            self.failures.set(self.failures.get() + 1);
            eprintln!("#   FAILED: {name}");
        }
    }

    /// Pumps the message loop until `predicate` returns true or the timeout
    /// elapses. Returns whether the predicate was satisfied.
    pub fn pump_until(&self, mut predicate: impl FnMut() -> bool, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;
        loop {
            if predicate() {
                return true;
            }
            if Instant::now() >= deadline {
                return false;
            }
            if !windows_window::pump() {
                return false;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    /// [`pump_until`](Self::pump_until) with the default timeout.
    pub fn wait(&self, predicate: impl FnMut() -> bool) -> bool {
        self.pump_until(predicate, TIMEOUT)
    }

    /// Returns the page to a blank document so the next fixture starts clean.
    pub fn reset(&self) {
        self.navigate_html("<!DOCTYPE html><html></html>");
    }

    /// Navigates to an in-memory HTML document and pumps until navigation
    /// completes, returning whether it succeeded within the timeout.
    pub fn navigate_html(&self, html: &str) -> bool {
        self.navigate(|webview| webview.navigate_to_string(html))
    }

    /// Navigates to a URI and pumps until navigation completes, returning
    /// whether it succeeded within the timeout.
    pub fn navigate_uri(&self, uri: &str) -> bool {
        self.navigate(|webview| webview.navigate(uri))
    }

    fn navigate(&self, start: impl FnOnce(&WebView) -> Result<()>) -> bool {
        let outcome: Rc<Cell<Option<bool>>> = Rc::new(Cell::new(None));
        let sink = outcome.clone();
        let Ok(registration) = self
            .webview
            .on_navigation_completed(move |args| sink.set(Some(args.is_success())))
        else {
            return false;
        };

        if start(&self.webview).is_err() {
            return false;
        }
        let completed = self.wait(|| outcome.get().is_some());
        drop(registration);
        completed && outcome.get().unwrap_or(false)
    }

    /// Runs `script` in the page and pumps until it returns, yielding the
    /// JSON-encoded result, or `None` if it did not return within the timeout.
    pub fn execute_script(&self, script: &str) -> Option<Result<String>> {
        let slot: Rc<RefCell<Option<Result<String>>>> = Rc::new(RefCell::new(None));
        let sink = slot.clone();
        if let Err(error) = self
            .webview
            .execute_script(script, move |result| *sink.borrow_mut() = Some(result))
        {
            return Some(Err(error));
        }
        if !self.wait(|| slot.borrow().is_some()) {
            return None;
        }
        slot.borrow_mut().take()
    }
}

impl Drop for Harness {
    /// Closes the controller and pumps briefly so Chromium tears its windows
    /// down in order, avoiding the benign `Failed to unregister class` warning
    /// it logs when the process exits with live browser windows.
    fn drop(&mut self) {
        if self.controller.close().is_ok() {
            let deadline = Instant::now() + Duration::from_millis(250);
            while Instant::now() < deadline && windows_window::pump() {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
