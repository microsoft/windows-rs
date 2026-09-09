//! Live WebView2 self-test harness.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use windows_core::{Error, HRESULT, Result, StaApartment};
use windows_webview::*;
use windows_window::Window;

// Allows cold WebView2 startup without hanging CI indefinitely.
const TIMEOUT: Duration = Duration::from_secs(15);

pub struct Harness {
    webview: WebView,
    controller: Controller,
    environment: Environment,
    _window: Window,
    failures: Cell<u32>,
    _apartment: StaApartment,
}

impl Harness {
    pub fn bootstrap(title: &str) -> Result<Self> {
        let apartment = windows_core::init_sta()?;
        let window = Window::new(title).size(1024, 768).create()?;
        let environment = Self::complete(Environment::create).ok_or_else(Self::timeout_error)??;
        let controller = Self::complete(|handler| {
            environment.create_controller_for_hwnd(window.hwnd(), handler)
        })
        .ok_or_else(Self::timeout_error)??;
        let (width, height) = window.client_size();
        controller.set_bounds(0, 0, width, height)?;
        let webview = controller.webview()?;

        Ok(Self {
            webview,
            controller,
            environment,
            _window: window,
            failures: Cell::new(0),
            _apartment: apartment,
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

    pub fn pump_until(&self, predicate: impl FnMut() -> bool, timeout: Duration) -> bool {
        Self::drive_until(predicate, timeout)
    }

    pub fn wait(&self, predicate: impl FnMut() -> bool) -> bool {
        self.pump_until(predicate, TIMEOUT)
    }

    pub fn complete<T: 'static>(
        start: impl FnOnce(Box<dyn FnOnce(Result<T>)>) -> Result<()>,
    ) -> Option<Result<T>> {
        let slot = Rc::new(RefCell::new(None));
        let sink = Rc::clone(&slot);
        if let Err(error) = start(Box::new(move |result| *sink.borrow_mut() = Some(result))) {
            return Some(Err(error));
        }
        if !Self::drive_until(|| slot.borrow().is_some(), TIMEOUT) {
            return None;
        }
        slot.borrow_mut().take()
    }

    pub fn reset(&self) {
        self.navigate_html("<!DOCTYPE html><html></html>");
    }

    fn drive_until(mut predicate: impl FnMut() -> bool, timeout: Duration) -> bool {
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

    fn timeout_error() -> Error {
        Error::new(
            HRESULT(0x8000_4005_u32 as i32),
            "WebView2 startup timed out",
        )
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
