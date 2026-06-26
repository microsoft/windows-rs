use std::cell::{Cell, RefCell};
use std::io::Write;
use std::rc::Rc;

use windows_core::*;

use crate::bindings::Color;
use crate::bindings::DispatcherQueue;
use crate::bindings::IInvokeProvider;
use crate::bindings::{
    Border, Button, CheckBox, ColumnDefinition, ComboBox, Grid, PasswordBox, RadioButtons,
    RowDefinition, Slider, TextBlock, TextBox, ToggleSwitch,
};
use crate::bindings::{ButtonAutomationPeer, PatternInterface};
use crate::bindings::{
    CLSCTX_INPROC_SERVER, CoCreateInstance, HWND, ITaskbarList3, TBPF_ERROR, TBPF_NOPROGRESS,
    TBPF_NORMAL, TaskbarList,
};
use crate::bindings::{
    DependencyObject, FrameworkElement, GridLength, GridUnitType, HorizontalAlignment, Thickness,
    UIElement, VerticalAlignment, Window,
};
use crate::bindings::{SolidColorBrush, VisualTreeHelper};

use windows_reactor::Component;
use windows_reactor::RenderHost;
use windows_reactor::WinUIBackend;
use windows_reactor::WinUIDispatcher;

use crate::exec::YieldLow;

fn replace_opt<T>(cell: &RefCell<Option<T>>, new: Option<T>) -> Option<T> {
    std::mem::replace(&mut *cell.borrow_mut(), new)
}

#[derive(Clone)]
pub struct Harness {
    inner: Rc<HarnessInner>,
}

struct HarnessInner {
    window: Window,
    content_area: Border,
    subtitle: TextBlock,
    segments: RefCell<Vec<Border>>,
    total: Cell<usize>,
    failures: Cell<u32>,
    dispatcher: DispatcherQueue,
    host: RefCell<Option<RenderHost<WinUIBackend, WinUIDispatcher>>>,
    taskbar: RefCell<Option<ITaskbarList3>>,
    hwnd: Cell<HWND>,
}

impl Harness {
    pub fn new(window_title: &str) -> Result<Self> {
        let window = Window::new()?;
        window.SetTitle(window_title)?;

        let dispatcher = DispatcherQueue::GetForCurrentThread()?;

        let content_area = Border::new()?;
        let subtitle = TextBlock::new()?;
        subtitle.SetFontSize(12.0)?;
        subtitle.SetForeground(&solid_brush(255, 255, 255, 255)?)?;
        subtitle
            .cast::<crate::bindings::IFrameworkElement>()?
            .SetVerticalAlignment(VerticalAlignment::Center)?;
        subtitle
            .cast::<crate::bindings::IUIElement>()?
            .SetIsHitTestVisible(false)?;

        let inner = HarnessInner {
            window,
            content_area,
            subtitle,
            segments: RefCell::new(Vec::new()),
            total: Cell::new(0),
            failures: Cell::new(0),
            dispatcher,
            host: RefCell::new(None),
            taskbar: RefCell::new(None),
            hwnd: Cell::new(core::ptr::null_mut()),
        };
        Ok(Self {
            inner: Rc::new(inner),
        })
    }

    pub fn setup_titlebar(&self, total: usize) -> Result<()> {
        let inner = &*self.inner;
        inner.total.set(total);
        inner.segments.borrow_mut().clear();

        let segment_bar = Grid::new()?;
        segment_bar
            .cast::<crate::bindings::IUIElement>()?
            .SetIsHitTestVisible(false)?;
        let cols = segment_bar.ColumnDefinitions()?;
        let star_one = GridLength {
            value: 1.0,
            grid_unit_type: GridUnitType::Star,
        };
        for i in 0..total {
            let cd = ColumnDefinition::new()?;
            cd.SetWidth(star_one)?;
            cols.Append(&cd)?;
            let seg = Border::new()?;
            seg.SetBackground(&solid_brush(30, 200, 200, 200)?)?;
            Grid::SetColumn(&seg, i as i32)?;
            let seg_ui: UIElement = seg.cast()?;
            segment_bar
                .cast::<crate::bindings::IPanel>()?
                .Children()?
                .Append(&seg_ui)?;
            inner.segments.borrow_mut().push(seg);
        }

        let pill = Border::new()?;
        pill.SetBackground(&solid_brush(180, 0, 0, 0)?)?;
        pill.SetCornerRadius(crate::bindings::CornerRadius {
            top_left: 4.0,
            top_right: 4.0,
            bottom_right: 4.0,
            bottom_left: 4.0,
        })?;
        pill.SetPadding(Thickness {
            left: 8.0,
            top: 2.0,
            right: 8.0,
            bottom: 2.0,
        })?;
        pill.cast::<crate::bindings::IFrameworkElement>()?
            .SetHorizontalAlignment(HorizontalAlignment::Left)?;
        pill.cast::<crate::bindings::IFrameworkElement>()?
            .SetVerticalAlignment(VerticalAlignment::Center)?;
        pill.cast::<crate::bindings::IFrameworkElement>()?
            .SetMargin(Thickness {
                left: 12.0,
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
            })?;
        pill.cast::<crate::bindings::IUIElement>()?
            .SetIsHitTestVisible(false)?;
        let subtitle_ui: UIElement = inner.subtitle.cast()?;
        pill.SetChild(&subtitle_ui)?;

        let titlebar_area = Grid::new()?;
        titlebar_area
            .cast::<crate::bindings::IFrameworkElement>()?
            .SetHeight(48.0)?;
        let bar_ui: UIElement = segment_bar.cast()?;
        let pill_ui: UIElement = pill.cast()?;
        titlebar_area
            .cast::<crate::bindings::IPanel>()?
            .Children()?
            .Append(&bar_ui)?;
        titlebar_area
            .cast::<crate::bindings::IPanel>()?
            .Children()?
            .Append(&pill_ui)?;

        let root = Grid::new()?;
        let rows = root.RowDefinitions()?;
        let auto = GridLength {
            value: 0.0,
            grid_unit_type: GridUnitType::Auto,
        };
        let row0 = RowDefinition::new()?;
        row0.SetHeight(auto)?;
        rows.Append(&row0)?;
        let row1 = RowDefinition::new()?;
        row1.SetHeight(star_one)?;
        rows.Append(&row1)?;

        let titlebar_ui: UIElement = titlebar_area.cast()?;
        Grid::SetRow(&titlebar_ui.cast::<FrameworkElement>()?, 0)?;
        root.cast::<crate::bindings::IPanel>()?
            .Children()?
            .Append(&titlebar_ui)?;

        let content_ui: UIElement = inner.content_area.cast()?;
        Grid::SetRow(&content_ui.cast::<FrameworkElement>()?, 1)?;
        root.cast::<crate::bindings::IPanel>()?
            .Children()?
            .Append(&content_ui)?;

        let root_ui: UIElement = root.cast()?;
        inner.window.SetContent(&root_ui)?;
        inner.window.SetExtendsContentIntoTitleBar(true)?;

        inner
            .window
            .SetTitleBar(&titlebar_area.cast::<UIElement>()?)?;

        if let Ok(hwnd) = window_hwnd(&inner.window) {
            inner.hwnd.set(hwnd);
            unsafe {
                let mut raw = core::ptr::null_mut();
                if CoCreateInstance(
                    &TaskbarList,
                    core::ptr::null_mut(),
                    CLSCTX_INPROC_SERVER,
                    &ITaskbarList3::IID,
                    &mut raw,
                )
                .is_ok()
                    && !raw.is_null()
                {
                    let tb = ITaskbarList3::from_raw(raw);
                    if tb.HrInit().is_ok() {
                        let _ = tb.SetProgressState(hwnd, TBPF_NORMAL);
                        let _ = tb.SetProgressValue(hwnd, 0, total as u64);
                        *inner.taskbar.borrow_mut() = Some(tb);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn activate(&self) -> Result<()> {
        self.inner.window.Activate()
    }

    pub fn update_progress(&self, current: usize, fixture: &str) -> Result<()> {
        let inner = &*self.inner;
        let total = inner.total.get();
        let label = format!("{current}/{total} \u{2014} {fixture}");
        inner.subtitle.SetText(&label)?;
        if let Some(tb) = inner.taskbar.borrow().as_ref() {
            unsafe {
                let _ = tb.SetProgressValue(inner.hwnd.get(), current as u64, total as u64);
            }
        }
        Ok(())
    }

    pub fn mark_fixture_result(&self, index: usize, passed: bool) -> Result<()> {
        let inner = &*self.inner;
        let segs = inner.segments.borrow();
        let Some(seg) = segs.get(index) else {
            return Ok(());
        };
        let brush = if passed {
            solid_brush(255, 76, 175, 80)?
        } else {
            solid_brush(255, 244, 67, 54)?
        };
        seg.SetBackground(&brush)?;
        Ok(())
    }

    pub fn finalize_taskbar(&self) {
        let inner = &*self.inner;
        if let Some(tb) = inner.taskbar.borrow().as_ref() {
            unsafe {
                let total = inner.total.get() as u64;
                let _ = tb.SetProgressValue(inner.hwnd.get(), total, total);
                let state = if inner.failures.get() > 0 {
                    TBPF_ERROR
                } else {
                    TBPF_NOPROGRESS
                };
                let _ = tb.SetProgressState(inner.hwnd.get(), state);
            }
        }
    }

    pub fn mount(&self, root: Box<dyn Component>) {
        let inner = &*self.inner;

        replace_opt(&inner.host, None);

        let dispatcher = WinUIDispatcher::for_current_thread()
            .expect("Harness::mount must be called on the UI thread");
        let host = RenderHost::new(WinUIBackend::new(), root, dispatcher);

        let host_for_post = host.clone_inner();
        let content = inner.content_area.clone();
        let last_attached: Rc<Cell<Option<windows_reactor::ControlId>>> = Rc::new(Cell::new(None));
        host.set_post_render(move |new_id| {
            if last_attached.get() == new_id {
                return;
            }
            match new_id {
                Some(rid) => {
                    if let Some(ui) = host_for_post.with_backend(|b| b.get_ui_element(rid)) {
                        let ui: UIElement = ui.cast().unwrap();
                        let _ = content
                            .cast::<crate::bindings::IBorder>()
                            .unwrap()
                            .SetChild(&ui);
                        last_attached.set(Some(rid));
                    }
                }
                None => {
                    last_attached.set(None);
                }
            }
        });
        host.kick();

        replace_opt(&inner.host, Some(host));
    }

    pub async fn render(&self) {
        const MAX_YIELDS: u32 = 10;
        for _ in 0..MAX_YIELDS {
            if self.is_idle() {
                break;
            }
            YieldLow::new(self.inner.dispatcher.clone()).await;
        }

        if let Ok(content) = self.inner.window.Content() {
            let _ = content.UpdateLayout();
        }

        for _ in 0..3 {
            YieldLow::new(self.inner.dispatcher.clone()).await;
        }
    }

    /// Pump the dispatcher in a bounded loop until `pred` returns `true`,
    /// running a full `render()` between each check. Returns `true` if the
    /// predicate held within the budget. On timeout, emits a TAP `#`
    /// diagnostic naming `label` and dumping the visual tree, then returns
    /// `false`. The caller decides whether timing out is fatal (typically
    /// by feeding the return value into a `check`/`check_with`).
    ///
    /// This is the scalable companion to `render()` for control surfaces
    /// (RadioButtons, ComboBox, ItemsRepeater-backed lists, etc.) whose
    /// items materialize across multiple dispatcher turns: instead of
    /// hard-coding extra `render().await` calls per fixture, wait on the
    /// observable condition you actually need.
    pub async fn render_until<F>(&self, label: &str, mut pred: F) -> bool
    where
        F: FnMut(&Harness) -> bool,
    {
        // 30 iterations × (10 low-priority yields per `render` + UpdateLayout
        // + 3 trailing yields) is enough budget for every ItemsRepeater-backed
        // control we've shipped fixtures for to materialize its items on a
        // loaded CI agent, while still capping a stuck test at well under a
        // second. Raise this only if a new control type genuinely needs more.
        const MAX_ITERATIONS: u32 = 30;
        self.render().await;
        for _ in 0..MAX_ITERATIONS {
            if pred(self) {
                return true;
            }
            YieldLow::new(self.inner.dispatcher.clone()).await;
            self.render().await;
        }
        if pred(self) {
            return true;
        }
        self.diag(&format!(
            "render_until: timed out waiting for {label}; visual tree:\n{}",
            self.dump_tree()
        ));
        false
    }

    /// Like [`render_until`](Self::render_until), but does not emit a
    /// diagnostic tree dump on timeout. Use for conditions that are
    /// expected to time out (e.g. known-flaky WinUI programmatic input).
    pub async fn render_until_quiet<F>(&self, _label: &str, mut pred: F) -> bool
    where
        F: FnMut(&Harness) -> bool,
    {
        const MAX_ITERATIONS: u32 = 30;
        self.render().await;
        for _ in 0..MAX_ITERATIONS {
            if pred(self) {
                return true;
            }
            YieldLow::new(self.inner.dispatcher.clone()).await;
            self.render().await;
        }
        pred(self)
    }

    fn is_idle(&self) -> bool {
        self.inner
            .host
            .borrow()
            .as_ref()
            .is_none_or(|h| h.is_idle())
    }

    pub fn check(&self, name: &str, ok: bool) {
        self.check_with(name, ok, || String::from("assertion failed"));
    }

    /// Like `check`, but `diag` is invoked only when the assertion fails and
    /// its returned string is appended to the TAP `not ok` line. Use this
    /// whenever a bare boolean assertion would not give enough context to
    /// diagnose a failure without re-running with a debugger attached.
    pub fn check_with<F>(&self, name: &str, ok: bool, diag: F)
    where
        F: FnOnce() -> String,
    {
        if ok {
            return;
        }
        let d = diag();
        if d.is_empty() {
            println!("not ok {name}");
        } else {
            println!("not ok {name} - {d}");
        }
        self.inner.failures.set(self.inner.failures.get() + 1);
        let _ = std::io::stdout().flush();
    }

    /// Equality assertion that prints `expected=… actual=…` on failure.
    /// Prefer this over `check` whenever the value being compared has a
    /// useful `Debug` representation (numbers, strings, enums, etc.).
    pub fn check_eq<T>(&self, name: &str, expected: T, actual: T)
    where
        T: std::fmt::Debug + PartialEq,
    {
        let ok = expected == actual;
        self.check_with(name, ok, || {
            format!("expected={expected:?} actual={actual:?}")
        });
    }

    /// Emit `ok NAME # SKIP reason` — a TAP-spec skip directive. The test
    /// is counted as passing for plan-count purposes but the harness/CI
    /// understands it as "intentionally not run on this platform / under
    /// these conditions". Use for assertions that cannot be reliably
    /// verified in-process (e.g. WinUI controls whose event chain only
    /// fires for real input).
    #[allow(clippy::unused_self)] // method for API consistency
    pub fn check_skip(&self, name: &str, reason: &str) {
        println!("ok {name} # SKIP {reason}");
        let _ = std::io::stdout().flush();
    }

    /// Begin capturing stderr output. Returns a [`StderrCapture`] that,
    /// when finished, yields the captured bytes as a string. Use this to
    /// detect `windows-reactor:` diagnostic warnings emitted by the
    /// backend's `eprintln!` calls.
    #[allow(clippy::unused_self)]
    pub fn capture_stderr(&self) -> StderrCapture {
        StderrCapture::start()
    }

    /// Assert that the captured stderr contains no `windows-reactor:`
    /// diagnostic lines.
    pub fn check_no_reactor_warnings(&self, name: &str, captured: &str) {
        let warnings: Vec<&str> = captured
            .lines()
            .filter(|l| l.contains("windows-reactor:"))
            .collect();
        self.check_with(name, warnings.is_empty(), || {
            format!("{} warning(s): {}", warnings.len(), warnings.join("; "))
        });
    }

    /// Emit a free-form TAP diagnostic comment. Useful from driver helpers
    /// to record why a side-effect could not be performed (e.g. control not
    /// found in the tree, HRESULT error from the underlying setter).
    #[allow(clippy::unused_self)] // method for API consistency
    pub fn diag(&self, msg: &str) {
        for line in msg.lines() {
            println!("# {line}");
        }
        let _ = std::io::stdout().flush();
    }

    /// Walk the current visual tree and return a compact textual snapshot
    /// — one indented line per node, with the type name and a few useful
    /// properties (text content, selected index, IsChecked, etc.). Intended
    /// for use inside `check_with` diagnostics; never inspected as a string
    /// by the test runner, so the exact format can evolve.
    pub fn dump_tree(&self) -> String {
        let mut out = String::new();
        match self.search_root() {
            Some(root) => dump_node(&root, 0, &mut out),
            None => out.push_str("(empty tree)\n"),
        }
        out
    }

    pub fn failures(&self) -> u32 {
        self.inner.failures.get()
    }

    pub fn window(&self) -> &Window {
        &self.inner.window
    }

    pub fn hwnd(&self) -> HWND {
        self.inner.hwnd.get()
    }

    fn search_root(&self) -> Option<DependencyObject> {
        let inner = &*self.inner;

        match inner
            .content_area
            .cast::<crate::bindings::IBorder>()
            .unwrap()
            .Child()
        {
            Ok(ui) => ui.cast::<DependencyObject>().ok(),
            Err(_) => None,
        }
    }

    pub fn find_text(&self, s: &str) -> Option<TextBlock> {
        self.find_first::<TextBlock>(&|tb| {
            tb.cast::<crate::bindings::ITextBlock>()
                .unwrap()
                .Text()
                .ok()
                .is_some_and(|t| t == s)
        })
    }

    pub fn find_text_containing(&self, s: &str) -> Option<TextBlock> {
        self.find_first::<TextBlock>(&|tb| {
            tb.cast::<crate::bindings::ITextBlock>()
                .unwrap()
                .Text()
                .ok()
                .is_some_and(|t| t.contains(s))
        })
    }

    pub fn find_button(&self, label: &str) -> Option<Button> {
        self.find_first::<Button>(&|btn| {
            let Ok(content) = btn
                .cast::<crate::bindings::IContentControl>()
                .unwrap()
                .Content()
            else {
                return false;
            };
            if let Ok(tb) = content.cast::<TextBlock>()
                && let Ok(t) = tb.cast::<crate::bindings::ITextBlock>().unwrap().Text()
                && t == label
            {
                return true;
            }
            content_string(&content).is_some_and(|s| s == label)
        })
    }

    pub fn count_controls<T: Interface>(&self) -> usize {
        let mut out = Vec::new();
        if let Some(root) = self.search_root() {
            collect_in_tree::<T>(&root, &|_| true, &mut out);
        }
        out.len()
    }

    pub fn find_all<T: Interface>(&self, pred: &dyn Fn(&T) -> bool) -> Vec<T> {
        let mut out = Vec::new();
        if let Some(root) = self.search_root() {
            collect_in_tree::<T>(&root, pred, &mut out);
        }
        out
    }

    fn find_first<T: Interface>(&self, pred: &dyn Fn(&T) -> bool) -> Option<T> {
        let root = self.search_root()?;
        find_in_tree::<T>(&root, pred)
    }

    pub fn click_button(&self, label: &str) -> Result<()> {
        let Some(btn) = self.find_button(label) else {
            return Ok(());
        };
        if !btn
            .cast::<crate::bindings::IControl>()?
            .IsEnabled()
            .unwrap_or(false)
        {
            return Ok(());
        }
        let peer = ButtonAutomationPeer::CreateInstanceWithOwner(&btn)?;
        let pat = peer
            .cast::<crate::bindings::IAutomationPeer>()?
            .GetPattern(PatternInterface::Invoke)?;
        let invoke: IInvokeProvider = pat.cast()?;
        invoke.Invoke()?;
        Ok(())
    }

    /// Set the underlying WinUI value on the first CheckBox in the tree.
    /// Fires the Checked/Unchecked event the reactor has wired, which in
    /// turn drives any `on_checked` callback.
    pub fn set_checkbox_value(&self, checked: bool) -> Result<()> {
        let Some(cb) = self.find_first::<CheckBox>(&|_| true) else {
            self.diag("set_checkbox_value: no CheckBox found in visual tree");
            return Err(Error::empty());
        };
        self.report_hresult("set_checkbox_value", || {
            cb.cast::<crate::bindings::IToggleButton>()?
                .SetIsChecked(Some(checked))?;
            Ok(())
        })
    }

    /// Set the underlying WinUI value on the first ToggleSwitch in the tree.
    pub fn set_toggle_switch_value(&self, on: bool) -> Result<()> {
        let Some(ts) = self.find_first::<ToggleSwitch>(&|_| true) else {
            self.diag("set_toggle_switch_value: no ToggleSwitch found in visual tree");
            return Err(Error::empty());
        };
        self.report_hresult("set_toggle_switch_value", || {
            ts.cast::<crate::bindings::IToggleSwitch>()?.SetIsOn(on)?;
            Ok(())
        })
    }

    /// Set the underlying WinUI value on the first Slider in the tree.
    pub fn set_slider_value(&self, value: f64) -> Result<()> {
        let Some(s) = self.find_first::<Slider>(&|_| true) else {
            self.diag("set_slider_value: no Slider found in visual tree");
            return Err(Error::empty());
        };
        self.report_hresult("set_slider_value", || {
            s.cast::<crate::bindings::IRangeBase>()?.SetValue(value)?;
            Ok(())
        })
    }

    /// Set the underlying WinUI text on the first TextBox in the tree.
    pub fn set_text_field_value(&self, text: &str) -> Result<()> {
        let Some(tb) = self.find_first::<TextBox>(&|_| true) else {
            self.diag("set_text_field_value: no TextBox found in visual tree");
            return Err(Error::empty());
        };
        self.report_hresult("set_text_field_value", || {
            tb.cast::<crate::bindings::ITextBox>()?.SetText(text)?;
            Ok(())
        })
    }

    /// Set the password value on the first PasswordBox in the tree, which
    /// fires the WinUI `PasswordChanged` event the reactor has wired.
    pub fn set_password_box_value(&self, text: &str) -> Result<()> {
        let Some(pb) = self.find_first::<PasswordBox>(&|_| true) else {
            self.diag("set_password_box_value: no PasswordBox found in visual tree");
            return Err(Error::empty());
        };
        self.report_hresult("set_password_box_value", || {
            pb.cast::<crate::bindings::IPasswordBox>()?
                .SetPassword(text)?;
            Ok(())
        })
    }

    /// Drive a selection change on the first `RadioButtons` in the tree by
    /// invoking the Nth child `RadioButton` through its UI Automation peer.
    ///
    /// **Known limitation:** WinUI's `RadioButtons` control does not
    /// reliably fire `SelectionChanged` for *any* in-process programmatic
    /// input. We have empirically tried three driving paths and observed
    /// the same failure mode on a loaded CI agent for all three:
    ///
    /// * `RadioButtons::SetSelectedIndex(n)` on the host — sets the
    ///   wrapper's `SelectedIndex` but children stay `IsChecked=false`
    ///   and `SelectionChanged` does not fire.
    /// * `IToggleButton::SetIsChecked(true)` on the Nth child — wrapper
    ///   reaches `SelectedIndex=n` but `SelectionChanged` still drops.
    /// * `IInvokeProvider::Invoke()` on the Nth child's automation peer
    ///   (the canonical WinUI "simulate a click" hook) — same outcome.
    ///
    /// The wrapper's `SelectionChanged` only fires reliably for real
    /// input events fed through XAML's input pipeline (e.g. WinAppDriver).
    /// Callers that need to verify the event chain end-to-end should use
    /// `Harness::check_skip` when this driver's effect cannot be observed
    /// within the `render_until` budget — see the
    /// `radio_buttons_change_selection` fixture for the pattern.
    ///
    /// This helper keeps the Automation peer Invoke path because it is
    /// the canonical hook and at least sometimes works; future bindgen
    /// work could switch to `ISelectionItemProvider::Select()` which is
    /// the spec-prescribed pattern for radio buttons but is not in our
    /// current `--minimal` binding set.
    pub fn set_radio_buttons_selected_index(&self, index: i32) -> Result<()> {
        if self.find_first::<RadioButtons>(&|_| true).is_none() {
            self.diag("set_radio_buttons_selected_index: no RadioButtons found in visual tree");
            return Err(Error::empty());
        }
        let buttons = self.find_all::<crate::bindings::RadioButton>(&|_| true);
        let Some(btn) = buttons.get(index as usize) else {
            self.diag(&format!(
                "set_radio_buttons_selected_index: index {index} out of range (found {} RadioButton children); visual tree:\n{}",
                buttons.len(),
                self.dump_tree()
            ));
            return Err(Error::empty());
        };
        let btn = btn.clone();
        let element: UIElement = btn.cast()?;
        let peer = match crate::bindings::FrameworkElementAutomationPeer::FromElement(&element) {
            Ok(p) => p,
            Err(_) => {
                crate::bindings::FrameworkElementAutomationPeer::CreatePeerForElement(&element)?
            }
        };
        let pattern = peer
            .cast::<crate::bindings::IAutomationPeer>()?
            .GetPattern(PatternInterface::Invoke)?;
        let invoke: IInvokeProvider = pattern.cast()?;
        invoke.Invoke()
    }

    /// Set the selected index on the first ComboBox in the tree.
    pub fn set_combo_box_selected_index(&self, index: i32) -> Result<()> {
        let Some(c) = self.find_first::<ComboBox>(&|_| true) else {
            self.diag("set_combo_box_selected_index: no ComboBox found in visual tree");
            return Err(Error::empty());
        };
        self.report_hresult("set_combo_box_selected_index", || {
            c.cast::<crate::bindings::ISelector>()?
                .SetSelectedIndex(index)?;
            Ok(())
        })
    }

    /// Internal helper: invokes `f` and, if it returns an `Err`, emits a TAP
    /// diagnostic comment tagging it with `op` before re-returning the error.
    fn report_hresult<F>(&self, op: &str, f: F) -> Result<()>
    where
        F: FnOnce() -> Result<()>,
    {
        match f() {
            Ok(()) => Ok(()),
            Err(e) => {
                self.diag(&format!("{op}: HRESULT error: {e}"));
                Err(e)
            }
        }
    }
}

fn find_in_tree<T: Interface>(root: &DependencyObject, pred: &dyn Fn(&T) -> bool) -> Option<T> {
    if let Ok(t) = root.cast::<T>()
        && pred(&t)
    {
        return Some(t);
    }
    let count = VisualTreeHelper::GetChildrenCount(root).unwrap_or(0);
    for i in 0..count {
        if let Ok(child) = VisualTreeHelper::GetChild(root, i)
            && let Some(found) = find_in_tree::<T>(&child, pred)
        {
            return Some(found);
        }
    }
    None
}

/// Recursively pretty-print a DependencyObject subtree. Each node is shown
/// on its own line, prefixed with two spaces per depth level, followed by
/// the runtime class name and any easy-to-read properties (text content,
/// selected index, IsChecked). This is best-effort: any HRESULT error
/// during introspection is swallowed so a malformed tree still produces
/// useful output for diagnostics.
fn dump_node(node: &DependencyObject, depth: usize, out: &mut String) {
    for _ in 0..depth {
        out.push_str("  ");
    }
    let class_name = node
        .cast::<IInspectable>()
        .ok()
        .and_then(|i| i.GetRuntimeClassName().ok())
        .map_or_else(|| "<unknown>".into(), |h| h.to_string_lossy());
    out.push_str(&class_name);

    if let Ok(tb) = node.cast::<crate::bindings::ITextBlock>()
        && let Ok(t) = tb.Text()
    {
        out.push_str(&format!(" Text={t:?}"));
    }
    if let Ok(rb) = node.cast::<crate::bindings::IRadioButtons>() {
        if let Ok(idx) = rb.SelectedIndex() {
            out.push_str(&format!(" SelectedIndex={idx}"));
        }
    } else if let Ok(sel) = node.cast::<crate::bindings::ISelector>()
        && let Ok(idx) = sel.SelectedIndex()
    {
        out.push_str(&format!(" SelectedIndex={idx}"));
    }
    if let Ok(tog) = node.cast::<crate::bindings::IToggleButton>()
        && let Ok(v) = tog.IsChecked()
    {
        out.push_str(&format!(" IsChecked={v:?}"));
    }
    out.push('\n');

    let count = VisualTreeHelper::GetChildrenCount(node).unwrap_or(0);
    for i in 0..count {
        if let Ok(child) = VisualTreeHelper::GetChild(node, i) {
            dump_node(&child, depth + 1, out);
        }
    }
}

fn collect_in_tree<T: Interface>(
    root: &DependencyObject,
    pred: &dyn Fn(&T) -> bool,
    out: &mut Vec<T>,
) {
    if let Ok(t) = root.cast::<T>()
        && pred(&t)
    {
        out.push(t);
    }
    let count = VisualTreeHelper::GetChildrenCount(root).unwrap_or(0);
    for i in 0..count {
        if let Ok(child) = VisualTreeHelper::GetChild(root, i) {
            collect_in_tree::<T>(&child, pred, out);
        }
    }
}

fn solid_brush(a: u8, r: u8, g: u8, b: u8) -> Result<SolidColorBrush> {
    let brush = SolidColorBrush::new()?;
    brush.SetColor(Color { a, r, g, b })?;
    Ok(brush)
}

fn content_string(content: &IInspectable) -> Option<String> {
    let pv: windows_reference::IReference<HSTRING> = content.cast().ok()?;
    Some(pv.Value().ok()?.to_string_lossy())
}

interface_decl! {
unsafe trait IWindowNative(IWindowNative_Vtbl, IWindowNative_Impl) : IUnknown = 0xEECDBF0E_BAE9_4CB6_A68E_9598E1CB57BB {
    unsafe fn WindowHandle(&self, hwnd: *mut HWND) -> HRESULT;
}
}

fn window_hwnd(window: &Window) -> Result<HWND> {
    let native: IWindowNative = window.cast()?;
    let mut hwnd: HWND = core::ptr::null_mut();
    unsafe { native.WindowHandle(&mut hwnd as *mut HWND).ok()? };
    Ok(hwnd)
}

// ── Stderr capture ─────────────────────────────────────────────────────

type RawHandle = *mut core::ffi::c_void;
const STD_ERROR_HANDLE: u32 = 0xFFFF_FFF4; // (DWORD)-12

unsafe extern "system" {
    fn GetStdHandle(nStdHandle: u32) -> RawHandle;
    fn SetStdHandle(nStdHandle: u32, hHandle: RawHandle) -> i32;
    fn CreatePipe(
        hReadPipe: *mut RawHandle,
        hWritePipe: *mut RawHandle,
        lpPipeAttributes: *const core::ffi::c_void,
        nSize: u32,
    ) -> i32;
    fn PeekNamedPipe(
        hNamedPipe: RawHandle,
        lpBuffer: *mut core::ffi::c_void,
        nBufferSize: u32,
        lpBytesRead: *mut u32,
        lpTotalBytesAvail: *mut u32,
        lpBytesLeftThisMessage: *mut u32,
    ) -> i32;
    fn ReadFile(
        hFile: RawHandle,
        lpBuffer: *mut u8,
        nNumberOfBytesToRead: u32,
        lpNumberOfBytesRead: *mut u32,
        lpOverlapped: *mut core::ffi::c_void,
    ) -> i32;
    fn CloseHandle(hObject: RawHandle) -> i32;
}

/// Captures stderr output by redirecting the Win32 standard error handle
/// to an anonymous pipe. Rust's `eprintln!` calls `GetStdHandle` on each
/// write, so `SetStdHandle` is the correct redirection mechanism.
pub struct StderrCapture {
    original: RawHandle,
    write_end: RawHandle,
    read_end: RawHandle,
}

impl StderrCapture {
    pub fn start() -> Self {
        unsafe {
            let original = GetStdHandle(STD_ERROR_HANDLE);
            let mut read_end: RawHandle = core::ptr::null_mut();
            let mut write_end: RawHandle = core::ptr::null_mut();
            // Use a 1 MB buffer so panic backtraces (RUST_BACKTRACE=1 on CI)
            // don't fill the pipe and deadlock the write side.
            CreatePipe(
                &mut read_end,
                &mut write_end,
                core::ptr::null(),
                1024 * 1024,
            );
            SetStdHandle(STD_ERROR_HANDLE, write_end);
            Self {
                original,
                write_end,
                read_end,
            }
        }
    }

    pub fn finish(self) -> String {
        unsafe {
            let _ = std::io::stderr().flush();
            SetStdHandle(STD_ERROR_HANDLE, self.original);
            CloseHandle(self.write_end);

            let mut buf = vec![0u8; 65536];
            let mut total = 0usize;
            loop {
                let mut avail: u32 = 0;
                if PeekNamedPipe(
                    self.read_end,
                    core::ptr::null_mut(),
                    0,
                    core::ptr::null_mut(),
                    &mut avail,
                    core::ptr::null_mut(),
                ) == 0
                    || avail == 0
                {
                    break;
                }
                let mut read: u32 = 0;
                let to_read = avail.min((buf.len() - total) as u32);
                ReadFile(
                    self.read_end,
                    buf.as_mut_ptr().add(total),
                    to_read,
                    &mut read,
                    core::ptr::null_mut(),
                );
                total += read as usize;
                if total >= buf.len() {
                    break;
                }
            }
            CloseHandle(self.read_end);
            buf.truncate(total);
            String::from_utf8_lossy(&buf).into_owned()
        }
    }
}
