use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::*;
use crate::core::*;
use crate::native::*;

const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
const MAX_PENDING_WINDOW_OPENS: usize = 64;

#[cfg(feature = "test")]
pub(crate) mod test;

thread_local! {
    static HOST: RefCell<Option<LiveHost>> = const { RefCell::new(None) };
    static SCHEDULER_FAULT: RefCell<Option<windows_core::Error>> = const { RefCell::new(None) };
}

#[cfg(feature = "test")]
thread_local! {
    static LIVE_DISPATCH_TIMES_US: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
}

struct LiveHost {
    _application: Application,
    closed_in_flight: HashSet<WindowToken>,
    fault: Option<windows_core::Error>,
    in_flight: HashSet<WindowToken>,
    pending_opens: usize,
    #[cfg(feature = "test")]
    primary: WindowToken,
    windows: HashMap<WindowToken, Box<dyn LivePump>>,
}

impl LiveHost {
    fn is_empty(&self) -> bool {
        self.windows.is_empty() && self.in_flight.is_empty() && self.pending_opens == 0
    }

    #[cfg(feature = "test")]
    fn primary(&self) -> Option<&dyn LivePump> {
        self.windows.get(&self.primary).map(Box::as_ref)
    }

    #[cfg(feature = "test")]
    fn primary_mut(&mut self) -> Option<&mut (dyn LivePump + '_)> {
        match self.windows.get_mut(&self.primary) {
            Some(pump) => Some(pump.as_mut()),
            None => None,
        }
    }

    #[cfg(feature = "test")]
    fn secondary(&self) -> Option<&dyn LivePump> {
        self.windows
            .iter()
            .find(|(token, _)| **token != self.primary)
            .map(|(_, pump)| pump.as_ref())
    }

    #[cfg(feature = "test")]
    fn secondary_mut(&mut self) -> Option<&mut (dyn LivePump + '_)> {
        for (token, pump) in &mut self.windows {
            if *token != self.primary {
                return Some(pump.as_mut());
            }
        }
        None
    }
}

trait LivePump {
    fn mount(&mut self) -> Result<(), PumpError>;
    fn dispatch_events(&mut self) -> Result<(), PumpError>;
    fn drain_diagnostics(&mut self) -> Vec<PumpDiagnostic>;
    fn native_work_pending(&self) -> bool;
    fn schedule_dispatch(&self) -> Result<(), RuntimeError>;
    fn close_scheduler(&self);
    fn native_window_closed(&mut self);
    fn shutdown(&mut self);
    fn window_token(&self) -> WindowToken;
    #[cfg(feature = "test")]
    fn live_window(&self) -> Result<Window, RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_bring_virtual_index(&self, _index: usize) -> Result<(), RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_virtual_shell_counts(&self) -> Result<(usize, usize), RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_event_subscription_count(&self) -> Result<usize, RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn take_live_native_apply_times(&mut self) -> Vec<f64> {
        Vec::new()
    }
    #[cfg(feature = "test")]
    fn clear_live_native_apply_times(&mut self) {}
    #[cfg(feature = "test")]
    fn live_event_revokers(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_event_delivery_step(&mut self) -> Result<bool, String> {
        Err("live event delivery is unsupported".to_string())
    }
    #[cfg(feature = "test")]
    fn live_content_dialog_lifecycle_step(&mut self) -> Result<bool, String> {
        Err("live ContentDialog lifecycle is unsupported".to_string())
    }
    #[cfg(feature = "test")]
    fn live_controlled_feedback_start(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_controlled_feedback_input(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_controlled_feedback_finish(&mut self) -> bool {
        false
    }
}

struct ComponentLoop {
    pump: Pump<WinUiRuntime>,
    root: Option<View>,
    #[cfg(feature = "test")]
    test: test::LiveTestState,
}

#[cfg(feature = "test")]
impl ComponentLoop {
    fn begin_live_event_stage(
        &mut self,
        view: impl Into<View>,
        name: &str,
        observed: Rc<std::cell::Cell<bool>>,
        apply: impl FnOnce(&WinUiRuntime, NodeId) -> Result<(), RuntimeError>,
    ) -> Result<(), String> {
        self.pump
            .update_view(view.into())
            .map_err(|error| format!("{name} event target update failed: {error:?}"))?;
        let node = self
            .pump
            .root_native()
            .ok_or_else(|| format!("{name} event target is unavailable"))?;
        apply(self.pump.runtime(), node)
            .map_err(|error| format!("{name} native input failed: {error:?}"))?;
        self.test.event_delivery_observed = Some(observed);
        self.test.event_delivery_waits = 0;
        Ok(())
    }

    fn live_event_delivery_step_impl(&mut self) -> Result<bool, String> {
        if let Some(observed) = self.test.event_delivery_observed.take() {
            self.pump
                .dispatch_events()
                .map_err(|error| format!("event dispatch failed: {error:?}"))?;
            if !observed.get() {
                self.test.event_delivery_waits += 1;
                if self.test.event_delivery_waits == 100 {
                    return Err(format!(
                        "event delivery stage {} produced no matching payload",
                        self.test.event_delivery_stage
                    ));
                }
                self.test.event_delivery_observed = Some(observed);
                return Ok(false);
            }
            self.test.event_delivery_stage += 1;
        }

        let observed = Rc::new(std::cell::Cell::new(false));
        let callback = Rc::clone(&observed);
        match self.test.event_delivery_stage {
            0 => self.begin_live_event_stage(
                ToggleSwitch::new()
                    .is_on(false)
                    .on_toggled(move |value| callback.set(value)),
                "bool",
                observed,
                |runtime, node| {
                    runtime.live_write_test_property(
                        node,
                        PropertyId::ToggleSwitchIsOn,
                        &PropertyValue::Bool(true),
                    )
                },
            )?,
            1 => self.begin_live_event_stage(
                PasswordBox::new()
                    .password("initial")
                    .on_password_changed(move |value| callback.set(value == "native")),
                "string",
                observed,
                |runtime, node| {
                    runtime.live_write_test_property(
                        node,
                        PropertyId::PasswordBoxPassword,
                        &PropertyValue::Str("native".to_string()),
                    )
                },
            )?,
            2 => self.begin_live_event_stage(
                Slider::new()
                    .minimum(0.0)
                    .maximum(10.0)
                    .value(1.0)
                    .on_value_changed(move |value| callback.set(value == 4.5)),
                "f64",
                observed,
                |runtime, node| {
                    runtime.live_write_test_property(
                        node,
                        PropertyId::SliderValue,
                        &PropertyValue::F64(4.5),
                    )
                },
            )?,
            3 => self.begin_live_event_stage(
                NumberBox::new()
                    .minimum(0.0)
                    .maximum(10.0)
                    .value(1.0)
                    .on_value_changed(move |value| callback.set(value == Some(4.5))),
                "NumberBox optional f64",
                observed,
                |runtime, node| {
                    runtime.live_write_test_property(
                        node,
                        PropertyId::NumberBoxValue,
                        &PropertyValue::OptionalF64(Some(4.5)),
                    )
                },
            )?,
            4 => {
                let color = Color {
                    a: 255,
                    r: 12,
                    g: 34,
                    b: 56,
                };
                self.begin_live_event_stage(
                    ColorPicker::new()
                        .color(Color::default())
                        .on_color_changed(move |value| callback.set(value == color)),
                    "color",
                    observed,
                    move |runtime, node| {
                        runtime.live_write_test_property(
                            node,
                            PropertyId::ColorPickerColor,
                            &PropertyValue::Color(color),
                        )
                    },
                )?;
            }
            5 => self.begin_live_event_stage(
                ListView::new()
                    .selected_index(None)
                    .on_selection_changed(move |value| callback.set(value == Some(1)))
                    .collection_slot(
                        ListViewSlot::Items,
                        [
                            KeyedView::new(
                                "first",
                                ListViewItem::new()
                                    .tag("first")
                                    .content(TextBlock::new().text("First")),
                            ),
                            KeyedView::new(
                                "second",
                                ListViewItem::new()
                                    .tag("second")
                                    .content(TextBlock::new().text("Second")),
                            ),
                        ],
                    ),
                "selection index",
                observed,
                |runtime, node| {
                    runtime.live_write_test_property(
                        node,
                        PropertyId::ListViewSelectedIndex,
                        &PropertyValue::SelectionIndex(Some(1)),
                    )
                },
            )?,
            6 => {
                let date = DateTime::from_unix_secs(1_700_000_000);
                self.begin_live_event_stage(
                    CalendarDatePicker::new()
                        .on_date_changed(move |value| callback.set(value == Some(date))),
                    "optional date",
                    observed,
                    move |runtime, node| runtime.live_set_test_date(node, date),
                )?;
            }
            7 => {
                let time = TimeSpan::from_hours(14) + TimeSpan::from_minutes(30);
                self.begin_live_event_stage(
                    TimePicker::new()
                        .on_selected_time_changed(move |value| callback.set(value == Some(time))),
                    "optional time",
                    observed,
                    move |runtime, node| runtime.live_set_test_time(node, time),
                )?;
            }
            8 => return Ok(true),
            _ => return Err("event delivery stage is invalid".to_string()),
        }
        Ok(false)
    }

    fn live_content_dialog_lifecycle_step_impl(&mut self) -> Result<bool, String> {
        let view = |first_open, second_open| {
            StackPanel::new().keyed_children([
                KeyedView::new(
                    "first",
                    ContentDialog::new().title("First").is_open(first_open),
                ),
                KeyedView::new(
                    "second",
                    ContentDialog::new().title("Second").is_open(second_open),
                ),
            ])
        };
        let mut wait = |states: &[LiveContentDialogState]| {
            self.test.content_dialog_waits += 1;
            if self.test.content_dialog_waits == 250 {
                Err(format!(
                    "ContentDialog probe stalled at stage {}: {states:?}",
                    self.test.content_dialog_stage
                ))
            } else {
                Ok(false)
            }
        };
        match self.test.content_dialog_stage {
            0 => {
                self.pump
                    .update_view(view(true, false))
                    .map_err(|error| format!("ContentDialog mount failed: {error:?}"))?;
                self.test.content_dialog_stage = 1;
                self.test.content_dialog_waits = 0;
            }
            1 => {
                let states = self.pump.runtime().live_content_dialog_states();
                let [first, second] = states.as_slice() else {
                    return Err("ContentDialog probe expected two dialogs".to_string());
                };
                if !first.pending || second.pending {
                    return wait(&states);
                }
                self.pump
                    .update_view(view(true, true))
                    .and_then(|_| self.pump.update_view(view(false, true)))
                    .and_then(|_| self.pump.update_view(view(true, true)))
                    .map_err(|error| format!("ContentDialog queue setup failed: {error:?}"))?;
                self.test.content_dialog_stage = 2;
                self.test.content_dialog_waits = 0;
            }
            2 => {
                let states = self.pump.runtime().live_content_dialog_states();
                let [first, second] = states.as_slice() else {
                    return Err("ContentDialog probe lost a dialog".to_string());
                };
                if first.pending || !first.queued || !second.pending {
                    return wait(&states);
                }
                self.pump
                    .runtime()
                    .live_hide_content_dialog(second.node)
                    .map_err(|error| format!("second ContentDialog hide failed: {error:?}"))?;
                self.test.content_dialog_stage = 3;
                self.test.content_dialog_waits = 0;
            }
            3 => {
                let states = self.pump.runtime().live_content_dialog_states();
                let [first, second] = states.as_slice() else {
                    return Err("ContentDialog probe lost a dialog".to_string());
                };
                if first.pending && !first.queued && !second.pending {
                    self.pump
                        .update_view(view(false, false))
                        .map_err(|error| format!("ContentDialog cleanup failed: {error:?}"))?;
                    self.test.content_dialog_stage = 4;
                    self.test.content_dialog_waits = 0;
                    return Ok(false);
                }
                return wait(&states);
            }
            4 => {
                let states = self.pump.runtime().live_content_dialog_states();
                let [first, second] = states.as_slice() else {
                    return Err("ContentDialog probe lost a dialog".to_string());
                };
                if !first.desired_open
                    && !first.pending
                    && !first.queued
                    && !second.desired_open
                    && !second.pending
                    && !second.queued
                {
                    return Ok(true);
                }
                return wait(&states);
            }
            _ => return Err("ContentDialog probe stage is invalid".to_string()),
        }
        Ok(false)
    }
}

impl LivePump for ComponentLoop {
    fn mount(&mut self) -> Result<(), PumpError> {
        self.pump
            .mount_view(self.root.take().ok_or(PumpError::AlreadyMounted)?)
            .map(|_| ())
    }

    fn dispatch_events(&mut self) -> Result<(), PumpError> {
        self.pump.dispatch_events()?;
        self.pump.dispatch_components(64)?;
        self.pump.process_imperatives().map(|_| ())
    }

    fn drain_diagnostics(&mut self) -> Vec<PumpDiagnostic> {
        self.pump.drain_diagnostics()
    }

    fn native_work_pending(&self) -> bool {
        self.pump.native_work_pending()
    }

    fn schedule_dispatch(&self) -> Result<(), RuntimeError> {
        self.pump.runtime().schedule_dispatch()
    }

    fn close_scheduler(&self) {
        self.pump.runtime().close_scheduler();
    }

    fn native_window_closed(&mut self) {
        self.pump.native_window_closed();
    }

    fn shutdown(&mut self) {
        self.pump.shutdown();
        self.pump.runtime().close_scheduler();
    }

    fn window_token(&self) -> WindowToken {
        self.pump.window_token()
    }

    #[cfg(feature = "test")]
    fn live_window(&self) -> Result<Window, RuntimeError> {
        self.pump.runtime().live_window()
    }

    #[cfg(feature = "test")]
    fn live_bring_virtual_index(&self, index: usize) -> Result<(), RuntimeError> {
        self.pump.runtime().live_bring_virtual_index(index)
    }

    #[cfg(feature = "test")]
    fn live_virtual_shell_counts(&self) -> Result<(usize, usize), RuntimeError> {
        self.pump.runtime().live_virtual_shell_counts()
    }

    #[cfg(feature = "test")]
    fn live_event_subscription_count(&self) -> Result<usize, RuntimeError> {
        Ok(self.pump.runtime().live_event_subscription_count())
    }

    #[cfg(feature = "test")]
    fn take_live_native_apply_times(&mut self) -> Vec<f64> {
        self.pump.runtime_mut().take_live_native_apply_times()
    }

    #[cfg(feature = "test")]
    fn clear_live_native_apply_times(&mut self) {
        self.pump.runtime_mut().clear_live_native_apply_times();
    }

    #[cfg(feature = "test")]
    fn live_event_revokers(&mut self) -> bool {
        let first = Rc::new(std::cell::Cell::new(0_u8));
        let second = Rc::new(std::cell::Cell::new(0_u8));
        let first_callback = Rc::clone(&first);
        if self
            .pump
            .update_view(
                CheckBox::new()
                    .is_checked(false)
                    .on_is_checked_changed(move |_| first_callback.set(first_callback.get() + 1))
                    .content(TextBlock::new().text("event target")),
            )
            .is_err()
        {
            return false;
        }
        let Some(node) = self.pump.root_native() else {
            return false;
        };
        if !matches!(self.pump.live_native_children(node), [_])
            || self.pump.runtime().live_set_checked(node, true).is_err()
            || self.pump.dispatch_events() != Ok(1)
            || first.get() != 1
        {
            return false;
        }

        let second_callback = Rc::clone(&second);
        if self
            .pump
            .update_view(
                CheckBox::new()
                    .is_checked(true)
                    .on_is_checked_changed(move |_| second_callback.set(second_callback.get() + 1))
                    .content(TextBlock::new().text("event target")),
            )
            .is_err()
            || self.pump.runtime().live_set_checked(node, false).is_err()
            || self.pump.dispatch_events() != Ok(1)
            || first.get() != 1
            || second.get() != 1
        {
            return false;
        }

        self.pump
            .update_view(
                CheckBox::new()
                    .is_checked(false)
                    .content(TextBlock::new().text("event target")),
            )
            .is_ok()
            && self.pump.runtime().live_set_checked(node, true).is_ok()
            && self.pump.dispatch_events() == Ok(0)
            && first.get() == 1
            && second.get() == 1
    }

    #[cfg(feature = "test")]
    fn live_event_delivery_step(&mut self) -> Result<bool, String> {
        self.live_event_delivery_step_impl()
    }

    #[cfg(feature = "test")]
    fn live_content_dialog_lifecycle_step(&mut self) -> Result<bool, String> {
        self.live_content_dialog_lifecycle_step_impl()
    }

    #[cfg(feature = "test")]
    fn live_controlled_feedback_start(&mut self) -> bool {
        true
    }

    #[cfg(feature = "test")]
    fn live_controlled_feedback_input(&mut self) -> bool {
        true
    }

    #[cfg(feature = "test")]
    fn live_controlled_feedback_finish(&mut self) -> bool {
        let text_events = Rc::new(std::cell::Cell::new(0_u8));
        let callback = Rc::clone(&text_events);
        let text_view = |value| {
            let callback = Rc::clone(&callback);
            TextBox::new()
                .text(value)
                .on_text_changed(move |_| callback.set(callback.get() + 1))
        };
        if self.pump.update_view(text_view("first").into()).is_err()
            || self.pump.update_view(text_view("second").into()).is_err()
            || text_events.get() != 0
        {
            eprintln!("controlled TextBox setter echoed to the application");
            return false;
        }

        let number_events = Rc::new(std::cell::Cell::new(0_u8));
        let number_view = |maximum| {
            let events = Rc::clone(&number_events);
            NumberBox::new()
                .minimum(0.0)
                .maximum(maximum)
                .value(7.0)
                .on_value_changed(move |_| events.set(events.get() + 1))
        };
        if self.pump.update_view(number_view(10.0).into()).is_err()
            || self.pump.update_view(number_view(5.0).into()).is_err()
            || number_events.get() != 0
            || self
                .pump
                .root_native()
                .is_none_or(|node| self.pump.runtime().live_range_value(node) != Ok(5.0))
        {
            eprintln!("controlled NumberBox feedback failed");
            return false;
        }

        let slider_events = Rc::new(std::cell::Cell::new(0_u8));
        let slider_view = |maximum| {
            let events = Rc::clone(&slider_events);
            Slider::new()
                .minimum(0.0)
                .maximum(maximum)
                .value(7.0)
                .on_value_changed(move |_| events.set(events.get() + 1))
        };
        if self.pump.update_view(slider_view(10.0).into()).is_err()
            || self.pump.update_view(slider_view(5.0).into()).is_err()
            || slider_events.get() != 0
            || self
                .pump
                .root_native()
                .is_none_or(|node| self.pump.runtime().live_range_value(node) != Ok(5.0))
        {
            eprintln!("controlled Slider feedback failed");
            return false;
        }

        let toggle_events = Rc::new(std::cell::Cell::new(0_u8));
        let check_box = |checked| {
            let events = Rc::clone(&toggle_events);
            CheckBox::new()
                .is_checked(checked)
                .on_is_checked_changed(move |_| events.set(events.get() + 1))
                .content(TextBlock::new().text("Check"))
        };
        if self.pump.update_view(check_box(false)).is_err()
            || self.pump.update_view(check_box(true)).is_err()
            || toggle_events.get() != 0
        {
            eprintln!("controlled CheckBox setter echoed to the application");
            return false;
        }
        let Some(check_box) = self.pump.root_native() else {
            return false;
        };
        if self
            .pump
            .runtime()
            .live_set_checked(check_box, false)
            .is_err()
            || self.pump.dispatch_events() != Ok(1)
            || toggle_events.get() != 1
            || self.pump.runtime().live_checked_value(check_box) != Ok(false)
        {
            eprintln!("CheckBox native feedback failed");
            return false;
        }

        let selected = Rc::new(RefCell::new(None));
        let selected_callback = Rc::clone(&selected);
        let list = ListBox::new()
            .on_selected_tag_changed(move |tag| *selected_callback.borrow_mut() = tag)
            .slots([SlotView::collection(
                ListBoxSlot::Items,
                [
                    KeyedView::new(
                        "one",
                        ListBoxItem::new()
                            .tag("one")
                            .is_selected(true)
                            .content(TextBlock::new().text("One")),
                    ),
                    KeyedView::new(
                        "two",
                        ListBoxItem::new()
                            .tag("two")
                            .is_selected(false)
                            .content(TextBlock::new().text("Two")),
                    ),
                ],
            )]);
        if self.pump.update_view(list).is_err() || self.pump.dispatch_events() != Ok(0) {
            eprintln!("controlled selection feedback failed");
            return false;
        }
        let Some(list_box) = self.pump.root_native() else {
            return false;
        };
        if self
            .pump
            .runtime()
            .live_select_list_box_item(list_box, 1)
            .is_err()
            || self.pump.dispatch_events() != Ok(1)
            || selected.borrow().as_deref() != Some("two")
        {
            eprintln!("ListBox native feedback failed");
            return false;
        }

        let progress = |value| {
            ProgressBar::new()
                .minimum(0.0)
                .maximum(100.0)
                .value(value)
                .is_indeterminate(false)
        };
        if self.pump.update_view(progress(25.0).into()).is_err()
            || self.pump.update_view(progress(75.0).into()).is_err()
        {
            eprintln!("controlled range update failed");
            return false;
        }
        true
    }
}

/// Starts and owns a Reactor application's WinUI message loop.
///
/// Each run method blocks until all application-owned windows have closed.
pub struct App;

impl App {
    /// Runs one window whose root content is `root`.
    ///
    /// This call blocks while the WinUI message loop is running.
    pub fn run(root: View) -> windows_core::Result<()> {
        Self::run_with(move |application| {
            vec![Box::new(ComponentLoop {
                pump: Pump::new(WinUiRuntime::with_application(application)),
                root: Some(root),
                #[cfg(feature = "test")]
                test: Default::default(),
            })]
        })
    }

    /// Runs one independent window and component pump for each root view.
    ///
    /// Unlike placing several views in a fragment, each item creates a separate native window.
    /// This call blocks until every application-owned window has closed.
    ///
    /// Returns an invalid-argument error if `roots` is empty.
    pub fn run_windows<I>(roots: I) -> windows_core::Result<()>
    where
        I: IntoIterator<Item = View>,
    {
        let roots = roots.into_iter().collect::<Vec<_>>();
        if roots.is_empty() {
            return Err(windows_core::Error::new(
                E_INVALIDARG,
                "at least one window is required",
            ));
        }
        Self::run_with(move |application| {
            roots
                .into_iter()
                .map(|root| {
                    Box::new(ComponentLoop {
                        pump: Pump::new(WinUiRuntime::with_application(application.clone())),
                        root: Some(root),
                        #[cfg(feature = "test")]
                        test: Default::default(),
                    }) as Box<dyn LivePump>
                })
                .collect()
        })
    }

    /// Runs one window rooted at component `C`.
    ///
    /// This is equivalent to passing [`View::component`] to [`run`](Self::run), and blocks while
    /// the WinUI message loop is running.
    pub fn run_component<C: Component>(input: C::Input) -> windows_core::Result<()> {
        Self::run(View::component::<C>(input))
    }

    fn run_with(
        create_pumps: impl FnOnce(Application) -> Vec<Box<dyn LivePump>> + 'static,
    ) -> windows_core::Result<()> {
        if !is_packaged_process()? {
            bootstrap_runtime()?;
        }

        initialize_ui_thread()?;
        let create_pumps = Rc::new(RefCell::new(Some(create_pumps)));
        let result = Rc::new(RefCell::new(Ok(())));
        let callback_result = Rc::clone(&result);

        let start = Application::Start(&ApplicationInitializationCallback::new(move |_| {
            let application = Rc::new(RefCell::new(None));
            let launch_application = Rc::clone(&application);
            let launch_result = Rc::clone(&callback_result);
            let launch_create_pumps = Rc::clone(&create_pumps);
            let on_launched = Box::new(move || {
                let launched: windows_core::Result<()> = (|| {
                    let application = launch_application
                        .borrow_mut()
                        .take()
                        .ok_or_else(|| windows_core::Error::new(E_FAIL, "missing application"))?;
                    install_xaml_controls_resources(&application)?;
                    let create_pumps = launch_create_pumps.borrow_mut().take().unwrap();
                    let mut pumps = create_pumps(application.clone()).into_iter();
                    let mut primary_pump = pumps.next().ok_or_else(|| {
                        windows_core::Error::new(E_INVALIDARG, "at least one window is required")
                    })?;
                    let primary = primary_pump.window_token();
                    let pumps = pumps.collect::<Vec<_>>();
                    let mut in_flight = pumps
                        .iter()
                        .map(|pump| pump.window_token())
                        .collect::<HashSet<_>>();
                    assert!(in_flight.insert(primary));
                    HOST.with(|host| {
                        *host.borrow_mut() = Some(LiveHost {
                            _application: application,
                            closed_in_flight: HashSet::new(),
                            fault: None,
                            in_flight,
                            pending_opens: pumps.len() + 1,
                            #[cfg(feature = "test")]
                            primary,
                            windows: HashMap::with_capacity(pumps.len() + 1),
                        });
                    });
                    primary_pump.mount().map_err(pump_error)?;
                    publish_mounted_window(primary_pump);
                    if !pumps.is_empty() {
                        let dispatcher = DispatcherQueue::GetForCurrentThread()?;
                        let pumps = Rc::new(RefCell::new(Some(pumps)));
                        let mount = DispatcherQueueHandler::new(move || {
                            let Some(pumps) = pumps.borrow_mut().take() else {
                                return;
                            };
                            for mut pump in pumps {
                                if let Err(error) = pump.mount() {
                                    let error = pump_error(error);
                                    eprintln!("windows-reactor additional window fault: {error}");
                                    HOST.with(|host| {
                                        if let Some(host) = host.borrow_mut().as_mut() {
                                            host.fault = Some(error);
                                        }
                                    });
                                    exit_ui_thread();
                                    return;
                                }
                                publish_mounted_window(pump);
                            }
                        });
                        if !dispatcher
                            .TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &mount)?
                        {
                            return Err(windows_core::Error::new(
                                E_FAIL,
                                "dispatcher rejected additional window mounting",
                            ));
                        }
                    }
                    Ok(())
                })();
                if let Err(error) = &launched {
                    *launch_result.borrow_mut() = Err(error.clone());
                    exit_ui_thread();
                }
                launched
            });
            match create_application(on_launched) {
                Ok(created) => *application.borrow_mut() = Some(created),
                Err(error) => {
                    *callback_result.borrow_mut() = Err(error);
                    exit_ui_thread();
                }
            }
        }));

        let callback_result = std::mem::replace(&mut *result.borrow_mut(), Ok(()));
        let host = HOST.with(|host| host.borrow_mut().take());
        let host_result = host
            .and_then(|mut host| {
                for pump in host.windows.values_mut() {
                    pump.shutdown();
                }
                host.fault
            })
            .map_or(Ok(()), Err);
        let scheduler_result = SCHEDULER_FAULT
            .with(|fault| fault.borrow_mut().take())
            .map_or(Ok(()), Err);
        start
            .and(callback_result)
            .and(host_result)
            .and(scheduler_result)
    }
}

fn publish_mounted_window(pump: Box<dyn LivePump>) {
    let token = pump.window_token();
    let mut pump = Some(pump);
    let finalize = HOST.with(|host| {
        let mut host = host.borrow_mut();
        let host = host
            .as_mut()
            .expect("missing live host during window mount");
        assert!(host.in_flight.remove(&token));
        host.pending_opens = host.pending_opens.checked_sub(1).unwrap();
        if host.closed_in_flight.remove(&token) {
            Some((pump.take().unwrap(), host.is_empty()))
        } else {
            assert!(host.windows.insert(token, pump.take().unwrap()).is_none());
            None
        }
    });
    if let Some((pump, empty)) = finalize {
        finalize_closed_window(pump, empty);
    }
}

pub(crate) fn open_live_windows(roots: Vec<View>) -> Result<(), RuntimeError> {
    if roots.is_empty() {
        return Ok(());
    }
    let application =
        HOST.with(|host| host.borrow().as_ref().map(|host| host._application.clone()));
    let application = application.ok_or(RuntimeError::MissingApplication)?;
    let pumps = roots
        .into_iter()
        .map(|root| {
            Box::new(ComponentLoop {
                pump: Pump::new(WinUiRuntime::with_application(application.clone())),
                root: Some(root),
                #[cfg(feature = "test")]
                test: Default::default(),
            }) as Box<dyn LivePump>
        })
        .collect::<Vec<_>>();
    let tokens = pumps
        .iter()
        .map(|pump| pump.window_token())
        .collect::<Vec<_>>();
    let registered = HOST.with(|host| {
        let mut host = host.borrow_mut();
        let Some(host) = host.as_mut() else {
            return false;
        };
        if host.pending_opens.saturating_add(pumps.len()) > MAX_PENDING_WINDOW_OPENS {
            return false;
        }
        host.pending_opens += pumps.len();
        for token in &tokens {
            assert!(host.in_flight.insert(*token));
        }
        true
    });
    if !registered {
        return Err(RuntimeError::WindowOpenCapacity);
    }

    let pending = Rc::new(RefCell::new(Some(pumps)));
    let pending_mount = Rc::clone(&pending);
    let mount = DispatcherQueueHandler::new(move || {
        let Some(pumps) = pending_mount.borrow_mut().take() else {
            return;
        };
        for mut pump in pumps {
            match pump.mount() {
                Ok(()) => publish_mounted_window(pump),
                Err(error) => reject_pending_window(pump, error),
            }
        }
    });
    let queued = DispatcherQueue::GetForCurrentThread()
        .map_err(winui_runtime_error)
        .and_then(|dispatcher| {
            dispatcher
                .TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &mount)
                .map_err(winui_runtime_error)
        });
    match queued {
        Ok(true) => Ok(()),
        Ok(false) => {
            rollback_pending_windows(&tokens);
            Err(RuntimeError::DispatcherRejected)
        }
        Err(error) => {
            rollback_pending_windows(&tokens);
            Err(error)
        }
    }
}

fn rollback_pending_windows(tokens: &[WindowToken]) {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        let Some(host) = host.as_mut() else {
            return;
        };
        for token in tokens {
            assert!(host.in_flight.remove(token));
            host.closed_in_flight.remove(token);
        }
        host.pending_opens = host.pending_opens.checked_sub(tokens.len()).unwrap();
    });
}

fn reject_pending_window(mut pump: Box<dyn LivePump>, error: PumpError) {
    let token = pump.window_token();
    let rejected = error.is_declaration_rejection();
    pump.shutdown();
    let empty = HOST.with(|host| {
        let mut host = host.borrow_mut();
        let host = host
            .as_mut()
            .expect("missing live host during window rejection");
        assert!(host.in_flight.remove(&token));
        host.closed_in_flight.remove(&token);
        host.pending_opens = host.pending_opens.checked_sub(1).unwrap();
        if !rejected {
            host.fault = Some(pump_error(error.clone()));
        }
        host.is_empty()
    });
    if rejected {
        eprintln!("windows-reactor rejected a runtime window: {error:?}");
    } else {
        eprintln!("windows-reactor runtime window fault: {error:?}");
        exit_ui_thread();
    }
    if empty {
        exit_ui_thread();
    }
}

pub(crate) fn dispatch_native_events(token: WindowToken) {
    HOST.with(|host| {
        let Some(mut live) = ({
            let mut host = host.borrow_mut();
            let Some(host) = host.as_mut() else {
                return;
            };
            let live = host.windows.remove(&token);
            if live.is_some() {
                host.in_flight.insert(token);
            }
            live
        }) else {
            return;
        };
        let mut rearm = false;
        let mut fault = None;
        #[cfg(feature = "test")]
        let dispatch_started = std::time::Instant::now();
        match live.dispatch_events() {
            Ok(()) => rearm = live.native_work_pending(),
            Err(error) => {
                let error = pump_error(error);
                eprintln!("windows-reactor fault: {error}");
                fault = Some(error);
                live.shutdown();
                exit_ui_thread();
            }
        }
        #[cfg(feature = "test")]
        LIVE_DISPATCH_TIMES_US.with(|times| {
            times
                .borrow_mut()
                .push(dispatch_started.elapsed().as_secs_f64() * 1_000_000.0);
        });
        for diagnostic in live.drain_diagnostics() {
            match diagnostic {
                PumpDiagnostic::WindowOpenRejected { error } => {
                    let message = format!("runtime window open was rejected: {error:?}");
                    #[cfg(feature = "test")]
                    test::record_live_diagnostic(message.clone());
                    eprintln!("windows-reactor warning: {message}");
                }
                PumpDiagnostic::VirtualRowRootCount {
                    collection,
                    key,
                    actual,
                } => {
                    let message = format!(
                        "virtual row {key:?} in {collection:?} has {actual} native roots; \
                         shell left empty"
                    );
                    #[cfg(feature = "test")]
                    test::record_live_diagnostic(message.clone());
                    eprintln!("windows-reactor warning: {message}");
                }
            }
        }
        let closed = host
            .borrow()
            .as_ref()
            .is_some_and(|host| host.closed_in_flight.contains(&token));
        if !closed
            && rearm
            && let Err(error) = live.schedule_dispatch()
        {
            fault = Some(runtime_error(error));
            live.shutdown();
            exit_ui_thread();
        }
        let mut finalize = None;
        if let Some(host) = host.borrow_mut().as_mut() {
            host.in_flight.remove(&token);
            let closed = host.closed_in_flight.remove(&token);
            if let Some(error) = fault {
                host.fault = Some(error);
            } else if closed {
                finalize = Some((live, host.is_empty()));
            } else {
                host.windows.insert(token, live);
            }
        }
        if let Some((live, empty)) = finalize {
            finalize_closed_window(live, empty);
        }
    });
}

pub(crate) fn dispatch_window_closed(token: WindowToken) {
    let (live, empty) = HOST.with(|host| {
        let mut host = host.borrow_mut();
        let Some(host) = host.as_mut() else {
            return (None, false);
        };
        if host.in_flight.contains(&token) {
            host.closed_in_flight.insert(token);
            return (None, false);
        }
        let live = host.windows.remove(&token);
        (live, host.is_empty())
    });
    if let Some(live) = live {
        finalize_closed_window(live, empty);
    }
}

fn finalize_closed_window(mut live: Box<dyn LivePump>, empty: bool) {
    live.close_scheduler();
    live.native_window_closed();
    let pending = Rc::new(RefCell::new(Some(live)));
    let pending_drop = Rc::clone(&pending);
    let drop_window = DispatcherQueueHandler::new(move || {
        drop(pending_drop.borrow_mut().take());
        if empty {
            exit_ui_thread();
        }
    });
    let queued = DispatcherQueue::GetForCurrentThread().and_then(|dispatcher| {
        dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::High, &drop_window)
    });
    if !matches!(queued, Ok(true)) {
        eprintln!("windows-reactor could not finalize a closed window");
        std::process::abort();
    }
}

#[cfg(feature = "test")]
fn queue_live_delayed(
    dispatcher: DispatcherQueue,
    continuation: impl FnOnce() + 'static,
) -> windows_core::Result<()> {
    let timer = dispatcher.CreateTimer()?;
    timer.SetInterval(TimeSpan { duration: 100_000 })?;
    timer.SetIsRepeating(false)?;
    let continuation = Rc::new(RefCell::new(Some(continuation)));
    let revoker = Rc::new(RefCell::new(None));
    let tick_timer = timer.clone();
    let tick_continuation = Rc::clone(&continuation);
    let tick_revoker = Rc::clone(&revoker);
    *revoker.borrow_mut() = Some(timer.Tick(move |_, _| {
        _ = tick_timer.Stop();
        tick_revoker.borrow_mut().take();
        if let Some(continuation) = tick_continuation.borrow_mut().take() {
            continuation();
        }
    })?);
    timer.Start()
}

fn is_packaged_process() -> windows_core::Result<bool> {
    let mut length = 0;
    let rc = unsafe { GetCurrentPackageFullName(&mut length, windows_core::PWSTR::null()) };
    match rc {
        ERROR_INSUFFICIENT_BUFFER => Ok(true),
        APPMODEL_ERROR_NO_PACKAGE => Ok(false),
        _ => Err(windows_core::HRESULT::from(windows_core::WIN32_ERROR(rc as u32)).into()),
    }
}

fn pump_error(error: PumpError) -> windows_core::Error {
    match error {
        PumpError::NativeApplyFailed(error) => {
            eprintln!(
                "windows-reactor fatal native command failure at {}: {:?}",
                error.command, error.error
            );
            std::process::abort();
        }
        error => windows_core::Error::new(E_FAIL, format!("{error:?}")),
    }
}

fn runtime_error(error: RuntimeError) -> windows_core::Error {
    windows_core::Error::new(E_FAIL, format!("{error:?}"))
}

fn winui_runtime_error(error: windows_core::Error) -> RuntimeError {
    RuntimeError::Native(error.code().0)
}

pub(crate) fn fail_native_scheduler(error: RuntimeError) {
    SCHEDULER_FAULT.with(|fault| {
        if fault.borrow().is_none() {
            *fault.borrow_mut() = Some(runtime_error(error));
        }
    });
    exit_ui_thread();
}
