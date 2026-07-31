//! Live WebView2 self-test harness.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use windows_core::Result;
use windows_webview::*;
use windows_window::Window;

// Allows cold WebView2 startup without hanging CI indefinitely.
const TIMEOUT: Duration = Duration::from_secs(15);

pub struct Harness {
    environment: Environment,
    _window: Window,
    controller: Controller,
    webview: WebView,
    failures: Cell<u32>,
}

impl Harness {
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

    pub fn check(&self, name: &str, condition: bool) {
        if !condition {
            self.failures.set(self.failures.get() + 1);
            eprintln!("#   FAILED: {name}");
        }
    }

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

    pub fn wait(&self, predicate: impl FnMut() -> bool) -> bool {
        self.pump_until(predicate, TIMEOUT)
    }

    pub fn reset(&self) {
        self.navigate_html("<!DOCTYPE html><html></html>");
    }

    pub fn navigate_html(&self, html: &str) -> bool {
        self.navigate(|webview| webview.navigate_to_string(html))
    }

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
    // Chromium logs a benign class-unregistration warning if browser windows remain alive.
    fn drop(&mut self) {
        if self.controller.close().is_ok() {
            let deadline = Instant::now() + Duration::from_millis(250);
            while Instant::now() < deadline && windows_window::pump() {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
