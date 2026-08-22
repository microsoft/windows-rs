use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::*;
use crate::core::*;
use crate::native::*;

const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
const MAX_PENDING_WINDOW_OPENS: usize = 64;

#[cfg(feature = "test")]
static LIVE_CLOSED_TASK_FINISHED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

thread_local! {
    static HOST: RefCell<Option<LiveHost>> = const { RefCell::new(None) };
    static SCHEDULER_FAULT: RefCell<Option<windows_core::Error>> = const { RefCell::new(None) };
}

#[cfg(feature = "test")]
thread_local! {
    static LIVE_TEST_DISPATCHES: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_TEST_REARM: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static LIVE_COMPONENT_CREATES: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_COMPONENT_EFFECT_SETUPS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_COMPONENT_EFFECT_CLEANUPS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_COMPONENT_BACKGROUND: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static LIVE_CLOSED_TASK_DELIVERED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static LIVE_CLOSE_SENDER: RefCell<Option<LocalSender<LiveClosingMessage>>> = const { RefCell::new(None) };
    static LIVE_RUNTIME_OPEN_SETUPS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_RUNTIME_OPEN_CLEANUPS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_RANGE_EVENTS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_TOGGLE_EVENTS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_PRIMARY_EVENTS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_PRIMARY_NATIVE_PAYLOAD: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static LIVE_SECONDARY_EVENTS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_SECONDARY_NATIVE_PAYLOAD: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
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
    fn primary_window_for_test(&self) -> Option<Window> {
        self.primary()?.live_window().ok()
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
    fn live_set_root_text(&self, _value: &str) -> Result<(), RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_root_text(&self) -> Result<String, RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
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
    fn take_live_native_apply_times(&mut self) -> Vec<f64> {
        Vec::new()
    }
    #[cfg(feature = "test")]
    fn live_rejection_then_retry(&self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_component_update(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_component_message_result(&self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_component_text(&self) -> Result<String, RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_closing_task(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_dense_reorder(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_fragment_anchor(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_grid(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_split_view(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_range_feedback(&mut self, _control: LiveRangeControl) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_range_restore(&mut self, _control: LiveRangeControl) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_range_value(&self) -> Result<f64, RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_range_clear_is_idempotent(&mut self, _control: LiveRangeControl) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_named_slots(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_progress_bar(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_toggle_switch(&mut self) -> bool {
        false
    }
}

struct ComponentLoop {
    pump: Pump<WinUiRuntime>,
    root: Option<View>,
}

#[cfg(feature = "test")]
struct LiveTestComponent {
    messages: u8,
    text: String,
}

#[cfg(feature = "test")]
enum LiveTestMessage {
    Background,
    Native(String),
}

#[cfg(feature = "test")]
#[derive(Clone, Copy)]
enum LiveRangeControl {
    NumberBox,
    Slider,
}

#[cfg(feature = "test")]
impl LiveRangeControl {
    fn name(self) -> &'static str {
        match self {
            Self::NumberBox => "NumberBox",
            Self::Slider => "Slider",
        }
    }
}

#[cfg(feature = "test")]
impl Component for LiveTestComponent {
    type Props = String;
    type Message = LiveTestMessage;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        LIVE_COMPONENT_CREATES.with(|count| count.set(count.get().saturating_add(1)));
        context.spawn_background(|_| LiveTestMessage::Background);
        Self {
            messages: 0,
            text: props.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.text.clone_from(props);
    }

    fn update(&mut self, message: Self::Message, _context: &mut ComponentContext<Self>) {
        match message {
            LiveTestMessage::Background => {
                LIVE_COMPONENT_BACKGROUND.with(|completed| completed.set(true));
            }
            LiveTestMessage::Native(message) => {
                self.messages = self.messages.saturating_add(1);
                self.text = if self.messages == 65 {
                    message
                } else {
                    "pending".to_string()
                };
            }
        }
    }

    fn view(&self, _props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        context.use_effect("live-test", (), || {
            LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.set(count.get().saturating_add(1)));
            Some(Box::new(|| {
                LIVE_COMPONENT_EFFECT_CLEANUPS
                    .with(|count| count.set(count.get().saturating_add(1)));
            }))
        });
        let sender = context.sender();
        View::native(TextBox::new().text(self.text.clone()).on_text_changed(
            move |value: String| {
                for _ in 0..65 {
                    sender.send(LiveTestMessage::Native(value.clone()));
                }
            },
        ))
    }
}

#[cfg(feature = "test")]
struct LiveClosingTask;

#[cfg(feature = "test")]
struct LiveRuntimeOpened;

#[cfg(feature = "test")]
#[derive(Clone)]
enum LiveClosingMessage {
    Background,
    Close,
}

#[cfg(feature = "test")]
impl Component for LiveClosingTask {
    type Props = ();
    type Message = LiveClosingMessage;

    fn create(_props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        LIVE_CLOSED_TASK_FINISHED.store(false, std::sync::atomic::Ordering::Release);
        LIVE_CLOSE_SENDER.with(|sender| *sender.borrow_mut() = Some(context.sender()));
        context.spawn_background(|_| {
            std::thread::sleep(std::time::Duration::from_millis(300));
            LIVE_CLOSED_TASK_FINISHED.store(true, std::sync::atomic::Ordering::Release);
            LiveClosingMessage::Background
        });
        Self
    }

    fn changed(&mut self, _props: &Self::Props, _context: &mut ComponentContext<Self>) {}

    fn update(&mut self, message: LiveClosingMessage, context: &mut ComponentContext<Self>) {
        match message {
            LiveClosingMessage::Background => {
                LIVE_CLOSED_TASK_DELIVERED.with(|delivered| delivered.set(true));
            }
            LiveClosingMessage::Close => {
                let opened = context.open_window(View::component::<LiveRuntimeOpened>(()));
                let closed = context.window().request_close();
                if !opened || !closed {
                    eprintln!("live component could not replace its window");
                    std::process::exit(1);
                }
            }
        }
    }

    fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text("closing"))
    }
}

#[cfg(feature = "test")]
impl Component for LiveRuntimeOpened {
    type Props = ();
    type Message = ();

    fn create(_props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        if !context.window().request_close() {
            eprintln!("runtime-open fixture could not request close");
            std::process::exit(1);
        }
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, _props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        context.window_title("Runtime opened");
        context.use_effect("runtime-open", (), || {
            LIVE_RUNTIME_OPEN_SETUPS.with(|count| count.set(count.get().saturating_add(1)));
            Some(Box::new(|| {
                LIVE_RUNTIME_OPEN_CLEANUPS.with(|count| count.set(count.get().saturating_add(1)));
            }))
        });
        TextBlock::new().text("runtime opened").into()
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
    fn live_set_root_text(&self, value: &str) -> Result<(), RuntimeError> {
        let root = self
            .pump
            .root_native()
            .ok_or(RuntimeError::UnsupportedKind)?;
        self.pump.runtime().live_set_text(root, value)
    }

    #[cfg(feature = "test")]
    fn live_root_text(&self) -> Result<String, RuntimeError> {
        let root = self
            .pump
            .root_native()
            .ok_or(RuntimeError::UnsupportedKind)?;
        self.pump.runtime().live_text(root)
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
    fn take_live_native_apply_times(&mut self) -> Vec<f64> {
        self.pump.runtime_mut().take_live_native_apply_times()
    }

    #[cfg(feature = "test")]
    fn live_rejection_then_retry(&self) -> bool {
        self.pump.runtime().live_reject_next_enqueue();
        self.pump.runtime().schedule_dispatch() == Err(RuntimeError::DispatcherRejected)
            && self.pump.runtime().schedule_dispatch().is_ok()
    }

    #[cfg(feature = "test")]
    fn live_component_update(&mut self) -> bool {
        LIVE_COMPONENT_CREATES.with(|count| count.set(0));
        LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.set(0));
        LIVE_COMPONENT_EFFECT_CLEANUPS.with(|count| count.set(0));
        LIVE_COMPONENT_BACKGROUND.with(|completed| completed.set(false));
        if self
            .pump
            .update_view(View::component::<LiveTestComponent>(
                "component".to_string(),
            ))
            .is_err()
        {
            return false;
        }
        let Some(native) = self.pump.root_native() else {
            return false;
        };
        LIVE_COMPONENT_CREATES.with(|count| count.get() == 1)
            && LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.get() == 1)
            && LIVE_COMPONENT_EFFECT_CLEANUPS.with(|count| count.get() == 0)
            && self.pump.runtime().live_text(native).as_deref() == Ok("component")
            && self.pump.runtime().live_set_text(native, "message").is_ok()
    }

    #[cfg(feature = "test")]
    fn live_component_message_result(&self) -> bool {
        let Some(native) = self.pump.root_native() else {
            return false;
        };
        LIVE_COMPONENT_CREATES.with(|count| count.get() == 1)
            && LIVE_COMPONENT_BACKGROUND.with(std::cell::Cell::get)
            && self.pump.runtime().live_text(native).as_deref() == Ok("message")
    }

    #[cfg(feature = "test")]
    fn live_component_text(&self) -> Result<String, RuntimeError> {
        let native = self
            .pump
            .root_native()
            .ok_or(RuntimeError::UnsupportedKind)?;
        self.pump.runtime().live_text(native)
    }

    #[cfg(feature = "test")]
    fn live_closing_task(&mut self) -> bool {
        LIVE_CLOSED_TASK_DELIVERED.with(|delivered| delivered.set(false));
        let mounted = self
            .pump
            .update_view(View::component::<LiveClosingTask>(()))
            .is_ok();
        mounted
            && LIVE_CLOSE_SENDER.with(|sender| {
                sender
                    .borrow()
                    .as_ref()
                    .is_some_and(|sender| sender.send(LiveClosingMessage::Close))
            })
    }

    #[cfg(feature = "test")]
    fn live_dense_reorder(&mut self) -> bool {
        let labels = (0..512).map(|index| index.to_string()).collect::<Vec<_>>();
        let view =
            |labels: &[String]| {
                View::native(StackPanel::new().native_children(
                    labels.iter().map(|label| {
                        KeyedElement::new(label.clone(), TextBlock::new().text(label))
                    }),
                ))
            };
        if self.pump.update_view(view(&labels)).is_err() {
            return false;
        }
        let reversed = labels.into_iter().rev().collect::<Vec<_>>();
        self.pump.update_view(view(&reversed)).is_ok()
    }

    #[cfg(feature = "test")]
    fn live_fragment_anchor(&mut self) -> bool {
        let fragment = |reverse: bool| {
            let children = if reverse {
                [
                    KeyedView::new("b", View::native(TextBlock::new().text("B"))),
                    KeyedView::new("a", View::native(TextBlock::new().text("A"))),
                ]
            } else {
                [
                    KeyedView::new("a", View::native(TextBlock::new().text("A"))),
                    KeyedView::new("b", View::native(TextBlock::new().text("B"))),
                ]
            };
            StackPanel::new().children((View::empty(), View::keyed_fragment(children)))
        };

        self.pump.update_view(View::empty()).is_ok()
            && self.pump.update_view(fragment(false)).is_ok()
            && self.pump.update_view(fragment(true)).is_ok()
    }

    #[cfg(feature = "test")]
    fn live_grid(&mut self) -> bool {
        let populated = Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .columns([GridLength::Pixel(120.0), GridLength::STAR])
            .children((TextBlock::new()
                .text("grid")
                .grid_row(1)
                .grid_column(2)
                .grid_row_span(3)
                .grid_column_span(4),));
        if self.pump.update_view(populated).is_err() {
            return false;
        }
        let Some(grid) = self.pump.root_native() else {
            return false;
        };
        let child = match self.pump.live_native_children(grid) {
            Ok([child]) => *child,
            _ => return false,
        };
        if self.pump.runtime().live_grid_matches(grid, child, true) != Ok(true) {
            return false;
        }
        if self
            .pump
            .update_view(Grid::new().children((TextBlock::new().text("grid"),)))
            .is_err()
        {
            return false;
        }
        if self.pump.runtime().live_grid_matches(grid, child, false) != Ok(true) {
            return false;
        }

        let repeater = || {
            ItemsRepeater::new()
                .item("row", TextBlock::new().text("row"))
                .grid_row(1)
                .grid_column(2)
                .grid_row_span(3)
                .grid_column_span(4)
        };
        if self
            .pump
            .update_view(
                Grid::new()
                    .rows([GridLength::Auto, GridLength::STAR])
                    .columns([GridLength::Pixel(120.0), GridLength::STAR])
                    .children((repeater(),)),
            )
            .is_err()
        {
            return false;
        }
        let collection = match self.pump.live_native_children(grid) {
            Ok([collection]) => *collection,
            _ => return false,
        };
        if self
            .pump
            .runtime()
            .live_grid_matches(grid, collection, true)
            != Ok(true)
        {
            return false;
        }
        if self
            .pump
            .update_view(
                Grid::new()
                    .children((ItemsRepeater::new().item("row", TextBlock::new().text("row")),)),
            )
            .is_err()
        {
            return false;
        }
        self.pump
            .runtime()
            .live_grid_matches(grid, collection, false)
            == Ok(true)
    }

    #[cfg(feature = "test")]
    fn live_split_view(&mut self) -> bool {
        let view = SplitView::new()
            .open_pane_length(280.0)
            .compact_pane_length(48.0)
            .display_mode(SplitViewDisplayMode::CompactInline)
            .is_pane_open(true)
            .slots([
                SlotView::new(
                    SplitViewSlot::Pane,
                    StackPanel::new().children((TextBlock::new().text("Pane"),)),
                ),
                SlotView::new(
                    SplitViewSlot::Content,
                    Grid::new().children((TextBlock::new().text("Content"),)),
                ),
            ]);
        if self.pump.update_view(view).is_err() {
            return false;
        }
        self.pump
            .root_native()
            .is_some_and(|root| self.pump.runtime().live_split_view_matches(root) == Ok(true))
    }

    #[cfg(feature = "test")]
    fn live_range_feedback(&mut self, control: LiveRangeControl) -> bool {
        LIVE_RANGE_EVENTS.with(|count| count.set(0));
        let view = |maximum, value| match control {
            LiveRangeControl::NumberBox => View::native(
                NumberBox::new()
                    .minimum(0.0)
                    .maximum(maximum)
                    .value(value)
                    .on_value_changed(record_live_range_event),
            ),
            LiveRangeControl::Slider => View::native(
                Slider::new()
                    .minimum(0.0)
                    .maximum(maximum)
                    .value(value)
                    .on_value_changed(record_live_range_event),
            ),
        };
        self.pump.update_view(view(10.0, 7.0)).is_ok()
            && self.pump.update_view(view(5.0, 7.0)).is_ok()
    }

    #[cfg(feature = "test")]
    fn live_range_restore(&mut self, control: LiveRangeControl) -> bool {
        let view = match control {
            LiveRangeControl::NumberBox => View::native(
                NumberBox::new()
                    .minimum(0.0)
                    .maximum(10.0)
                    .value(7.0)
                    .on_value_changed(record_live_range_event),
            ),
            LiveRangeControl::Slider => View::native(
                Slider::new()
                    .minimum(0.0)
                    .maximum(10.0)
                    .value(7.0)
                    .on_value_changed(record_live_range_event),
            ),
        };
        self.pump.update_view(view).is_ok()
    }

    #[cfg(feature = "test")]
    fn live_range_value(&self) -> Result<f64, RuntimeError> {
        let native = self
            .pump
            .root_native()
            .ok_or(RuntimeError::UnsupportedKind)?;
        self.pump.runtime().live_range_value(native)
    }

    #[cfg(feature = "test")]
    fn live_range_clear_is_idempotent(&mut self, control: LiveRangeControl) -> bool {
        let view = match control {
            LiveRangeControl::NumberBox => View::native(
                NumberBox::new()
                    .minimum(0.0)
                    .maximum(10.0)
                    .on_value_changed(record_live_range_event),
            ),
            LiveRangeControl::Slider => View::native(
                Slider::new()
                    .minimum(0.0)
                    .maximum(10.0)
                    .on_value_changed(record_live_range_event),
            ),
        };
        if self.pump.update_view(view.clone()).is_err() || self.pump.dispatch_events().is_err() {
            return false;
        }
        let commands = self.pump.runtime().live_applied_commands();
        self.pump.update_view(view).is_ok()
            && self.pump.runtime().live_applied_commands() == commands
    }

    #[cfg(feature = "test")]
    fn live_named_slots(&mut self) -> bool {
        let view = |content: Option<&str>, header: &str| {
            let mut slots = vec![SlotView::new(
                NavigationViewSlot::Header,
                View::native(TextBlock::new().text(header)),
            )];
            if let Some(content) = content {
                slots.push(SlotView::new(
                    NavigationViewSlot::Content,
                    View::native(TextBlock::new().text(content)),
                ));
            }
            NavigationView::new().slots(slots)
        };

        self.pump
            .update_view(view(Some("content 1"), "header 1"))
            .is_ok()
            && self
                .pump
                .update_view(view(Some("content 2"), "header 1"))
                .is_ok()
            && self.pump.update_view(view(None, "header 2")).is_ok()
    }

    #[cfg(feature = "test")]
    fn live_progress_bar(&mut self) -> bool {
        let view = |value, error, paused| {
            View::native(
                ProgressBar::new()
                    .minimum(0.0)
                    .maximum(100.0)
                    .value(value)
                    .is_indeterminate(false)
                    .show_error(error)
                    .show_paused(paused)
                    .is_enabled(true),
            )
        };
        if let Err(error) = self.pump.update_view(view(25.0, false, false)) {
            eprintln!("ProgressBar initial update failed: {error:?}");
            return false;
        }
        if let Err(error) = self.pump.update_view(view(75.0, true, true)) {
            eprintln!("ProgressBar second update failed: {error:?}");
            return false;
        }
        true
    }

    #[cfg(feature = "test")]
    fn live_toggle_switch(&mut self) -> bool {
        LIVE_TOGGLE_EVENTS.with(|count| count.set(0));
        let view = |is_on| {
            View::native(
                ToggleSwitch::new()
                    .is_on(is_on)
                    .is_enabled(true)
                    .on_toggled(record_live_toggle_event),
            )
        };
        self.pump.update_view(view(false)).is_ok()
            && self.pump.update_view(view(true)).is_ok()
            && self.pump.update_view(view(false)).is_ok()
            && LIVE_TOGGLE_EVENTS.with(std::cell::Cell::get) == 0
            && self
                .pump
                .root_native()
                .and_then(|node| self.pump.runtime().live_toggle_value(node).ok())
                == Some(false)
    }
}

#[cfg(feature = "test")]
fn record_live_range_event(_: f64) {
    LIVE_RANGE_EVENTS.with(|count| count.set(count.get().saturating_add(1)));
}

#[cfg(feature = "test")]
fn record_live_toggle_event(_: bool) {
    LIVE_TOGGLE_EVENTS.with(|count| count.set(count.get().saturating_add(1)));
}

pub fn bootstrap() -> windows_core::Result<()> {
    bootstrap_runtime()
}

#[cfg(feature = "test")]
pub fn bring_live_virtual_index(index: usize) -> Result<(), RuntimeError> {
    HOST.with(|host| {
        host.borrow()
            .as_ref()
            .and_then(LiveHost::primary)
            .ok_or(RuntimeError::UnsupportedKind)?
            .live_bring_virtual_index(index)
    })
}

#[cfg(feature = "test")]
pub fn live_virtual_shell_counts() -> Result<(usize, usize), RuntimeError> {
    HOST.with(|host| {
        host.borrow()
            .as_ref()
            .and_then(LiveHost::primary)
            .ok_or(RuntimeError::UnsupportedKind)?
            .live_virtual_shell_counts()
    })
}

#[cfg(feature = "test")]
pub fn take_live_performance_times() -> (Vec<f64>, Vec<f64>) {
    let dispatch = LIVE_DISPATCH_TIMES_US.with(|times| std::mem::take(&mut *times.borrow_mut()));
    let native = HOST.with(|host| {
        host.borrow_mut()
            .as_mut()
            .and_then(LiveHost::primary_mut)
            .map_or_else(Vec::new, LivePump::take_live_native_apply_times)
    });
    (dispatch, native)
}

pub struct App;

impl App {
    pub fn run(root: View) -> windows_core::Result<()> {
        Self::run_with(move |application| {
            vec![Box::new(ComponentLoop {
                pump: Pump::new(WinUiRuntime::with_application(application)),
                root: Some(root),
            })]
        })
    }

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
                    }) as Box<dyn LivePump>
                })
                .collect()
        })
    }

    pub fn run_component<C: Component>(props: C::Props) -> windows_core::Result<()> {
        Self::run(View::component::<C>(props))
    }

    fn run_with(
        create_pumps: impl FnOnce(Application) -> Vec<Box<dyn LivePump>> + 'static,
    ) -> windows_core::Result<()> {
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
                                    eprintln!(
                                        "windows-reactor-next additional window fault: {error}"
                                    );
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
    let rejected = matches!(
        error,
        PumpError::DuplicateEffectKey(_)
            | PumpError::DuplicateElementRef
            | PumpError::DuplicateKey(_)
            | PumpError::DuplicateWindowTitle
            | PumpError::StructureUnsupported
    );
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
        eprintln!("windows-reactor-next rejected a runtime window: {error:?}");
    } else {
        eprintln!("windows-reactor-next runtime window fault: {error:?}");
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
        #[cfg(feature = "test")]
        LIVE_TEST_DISPATCHES.with(|count| count.set(count.get().saturating_add(1)));
        let mut retry = false;
        let mut fault = None;
        #[cfg(feature = "test")]
        let dispatch_started = std::time::Instant::now();
        match live.dispatch_events() {
            Ok(()) => retry = live.native_work_pending(),
            Err(error) => {
                let error = pump_error(error);
                eprintln!("windows-reactor-next fault: {error}");
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
                PumpDiagnostic::VirtualRowRootCount {
                    collection,
                    key,
                    actual,
                } => {
                    eprintln!(
                        "windows-reactor-next warning: virtual row {key:?} in \
                         {collection:?} has {actual} native roots; shell left empty"
                    );
                }
            }
        }
        let closed = host
            .borrow()
            .as_ref()
            .is_some_and(|host| host.closed_in_flight.contains(&token));
        #[cfg(feature = "test")]
        if !closed
            && LIVE_TEST_REARM.with(|rearm| rearm.replace(false))
            && let Err(error) = live.schedule_dispatch()
        {
            fault = Some(runtime_error(error));
            live.shutdown();
            exit_ui_thread();
        }
        if !closed
            && retry
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
            #[cfg(feature = "test")]
            finish_live_closed_test();
            #[cfg(not(feature = "test"))]
            exit_ui_thread();
        }
    });
    let queued = DispatcherQueue::GetForCurrentThread().and_then(|dispatcher| {
        dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::High, &drop_window)
    });
    if !matches!(queued, Ok(true)) {
        eprintln!("windows-reactor-next could not finalize a closed window");
        std::process::abort();
    }
}

#[cfg(feature = "test")]
pub fn record_live_primary_event(value: String) {
    LIVE_PRIMARY_EVENTS.with(|count| count.set(count.get().saturating_add(1)));
    if value == "native" {
        LIVE_PRIMARY_NATIVE_PAYLOAD.with(|observed| observed.set(true));
    }
}

#[cfg(feature = "test")]
pub fn record_live_secondary_event(value: String) {
    LIVE_SECONDARY_EVENTS.with(|count| count.set(count.get().saturating_add(1)));
    if value == "secondary-native" {
        LIVE_SECONDARY_NATIVE_PAYLOAD.with(|observed| observed.set(true));
    }
}

#[cfg(feature = "test")]
pub fn schedule_live_controlled_repair_test(initial_success: bool) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(150));
        let edit_dispatcher = dispatcher.clone();
        let edit = DispatcherQueueHandler::new(move || {
            LIVE_PRIMARY_EVENTS.with(|count| count.set(0));
            LIVE_SECONDARY_EVENTS.with(|count| count.set(0));
            LIVE_PRIMARY_NATIVE_PAYLOAD.with(|observed| observed.set(false));
            LIVE_SECONDARY_NATIVE_PAYLOAD.with(|observed| observed.set(false));
            let edited = HOST.with(|host| {
                host.borrow().as_ref().map(|host| {
                    host.primary()
                        .ok_or(RuntimeError::MissingApplication)
                        .and_then(|pump| pump.live_set_root_text("native"))
                })
            });
            if !matches!(edited, Some(Ok(()))) {
                eprintln!("controlled repair fixture could not edit primary root: {edited:?}");
                std::process::exit(1);
            }
            let verify_dispatcher = edit_dispatcher.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                if queue_live_repair_verification(verify_dispatcher, initial_success, 8).is_err() {
                    std::process::exit(1);
                }
            });
        });
        if !matches!(
            dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &edit),
            Ok(true)
        ) {
            std::process::exit(1);
        }
    });
    Ok(())
}

#[cfg(feature = "test")]
fn queue_live_repair_verification(
    dispatcher: DispatcherQueue,
    initial_success: bool,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        let text = HOST.with(|host| {
            host.borrow().as_ref().map(|host| {
                (
                    host.primary().map(LivePump::live_root_text),
                    host.secondary().map(LivePump::live_root_text),
                )
            })
        });
        let repaired = matches!(
            text.as_ref(),
            Some((Some(Ok(primary)), Some(Ok(secondary))))
                if primary == "fixed" && secondary == "second"
        );
        let routed = LIVE_PRIMARY_EVENTS.with(|count| count.get() > 0)
            && LIVE_SECONDARY_EVENTS.with(|count| count.get() == 0)
            && LIVE_PRIMARY_NATIVE_PAYLOAD.with(std::cell::Cell::get);
        if initial_success && repaired && routed {
            if schedule_live_secondary_repair_test(initial_success).is_err() {
                std::process::exit(1);
            }
            return;
        }
        if attempts == 0 {
            eprintln!(
                "primary controlled repair fixture failed: resources={initial_success}, \
                 text={text:?}, routed={routed}"
            );
            std::process::exit(1);
        }
        if queue_live_repair_verification(next_dispatcher.clone(), initial_success, attempts - 1)
            .is_err()
        {
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected controlled repair verification",
        ))
    }
}

#[cfg(feature = "test")]
fn schedule_live_secondary_repair_test(initial_success: bool) -> windows_core::Result<()> {
    let edited = HOST.with(|host| {
        host.borrow().as_ref().map(|host| {
            host.secondary()
                .ok_or(RuntimeError::MissingApplication)
                .and_then(|pump| pump.live_set_root_text("secondary-native"))
        })
    });
    if !matches!(edited, Some(Ok(()))) {
        return Err(windows_core::Error::new(
            E_FAIL,
            format!("controlled repair fixture could not edit secondary root: {edited:?}"),
        ));
    }
    queue_live_secondary_repair_verification(
        DispatcherQueue::GetForCurrentThread()?,
        initial_success,
        LIVE_PRIMARY_EVENTS.with(std::cell::Cell::get),
        8,
    )
}

#[cfg(feature = "test")]
fn queue_live_secondary_repair_verification(
    dispatcher: DispatcherQueue,
    initial_success: bool,
    primary_events: u8,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        let text = HOST.with(|host| {
            host.borrow().as_ref().map(|host| {
                (
                    host.primary().map(LivePump::live_root_text),
                    host.secondary().map(LivePump::live_root_text),
                )
            })
        });
        let repaired = matches!(
            text.as_ref(),
            Some((Some(Ok(primary)), Some(Ok(secondary))))
                if primary == "fixed" && secondary == "second"
        );
        let routed = LIVE_PRIMARY_EVENTS.with(|count| count.get() == primary_events)
            && LIVE_SECONDARY_EVENTS.with(|count| count.get() > 0)
            && LIVE_SECONDARY_NATIVE_PAYLOAD.with(std::cell::Cell::get);
        if initial_success && repaired && routed {
            if schedule_live_scheduler_reentrancy_test().is_err() {
                std::process::exit(1);
            }
            return;
        }
        if attempts == 0
            || queue_live_secondary_repair_verification(
                next_dispatcher.clone(),
                initial_success,
                primary_events,
                attempts - 1,
            )
            .is_err()
        {
            eprintln!(
                "secondary controlled repair fixture failed: resources={initial_success}, \
                 text={text:?}, routed={routed}"
            );
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected secondary controlled repair verification",
        ))
    }
}

#[cfg(feature = "test")]
fn schedule_live_scheduler_reentrancy_test() -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let verify_dispatcher = dispatcher.clone();
    let start = DispatcherQueueHandler::new(move || {
        LIVE_TEST_DISPATCHES.with(|count| count.set(0));
        LIVE_TEST_REARM.with(|rearm| rearm.set(true));
        let scheduled = HOST.with(|host| {
            host.borrow()
                .as_ref()
                .and_then(LiveHost::primary)
                .is_some_and(LivePump::live_rejection_then_retry)
        });
        if !scheduled || queue_live_reentrancy_verification(verify_dispatcher.clone(), 8).is_err() {
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &start)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected reentrancy fixture",
        ))
    }
}

#[cfg(feature = "test")]
fn queue_live_reentrancy_verification(
    dispatcher: DispatcherQueue,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        if LIVE_TEST_DISPATCHES.with(|count| count.get() >= 2) {
            finish_live_backend_test();
            return;
        }
        if attempts == 0
            || queue_live_reentrancy_verification(next_dispatcher.clone(), attempts - 1).is_err()
        {
            eprintln!("scheduler reentrancy fixture did not observe a rearmed dispatch");
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected reentrancy verification",
        ))
    }
}

#[cfg(feature = "test")]
fn finish_live_backend_test() {
    LIVE_RUNTIME_OPEN_SETUPS.with(|count| count.set(0));
    LIVE_RUNTIME_OPEN_CLEANUPS.with(|count| count.set(0));
    let prepared = HOST.with(|host| {
        host.borrow_mut()
            .as_mut()
            .and_then(LiveHost::secondary_mut)
            .is_some_and(LivePump::live_closing_task)
    });
    if !prepared {
        eprintln!("live backend fixture could not start a secondary background task");
        std::process::exit(1);
    }
    let dispatcher = match DispatcherQueue::GetForCurrentThread() {
        Ok(dispatcher) => dispatcher,
        Err(error) => {
            eprintln!("window closure fixture has no dispatcher: {error}");
            std::process::exit(1);
        }
    };
    if queue_live_secondary_close_verification(dispatcher, 8).is_err() {
        std::process::exit(1);
    }
}

#[cfg(feature = "test")]
fn queue_live_secondary_close_verification(
    dispatcher: DispatcherQueue,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        let closed = HOST.with(|host| {
            host.borrow().as_ref().is_some_and(|host| {
                host.windows.len() == 1
                    && host.pending_opens == 0
                    && host
                        .primary()
                        .is_some_and(|pump| pump.live_root_text().as_deref() == Ok("fixed"))
            })
        });
        let runtime_opened = LIVE_RUNTIME_OPEN_SETUPS.with(|count| count.get() == 1)
            && LIVE_RUNTIME_OPEN_CLEANUPS.with(|count| count.get() == 1);
        if closed && runtime_opened {
            continue_live_backend_test();
            return;
        }
        if attempts == 0
            || queue_live_secondary_close_verification(next_dispatcher.clone(), attempts - 1)
                .is_err()
        {
            eprintln!(
                "live backend fixture did not complete runtime open/close: \
                 closed={closed}, runtime_opened={runtime_opened}"
            );
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected window closure verification",
        ))
    }
}

#[cfg(feature = "test")]
fn continue_live_backend_test() {
    let Some(mut live) = HOST.with(|host| host.borrow_mut().take()) else {
        eprintln!("live backend fixture lost its host");
        std::process::exit(1);
    };
    if !live.primary_mut().is_some_and(LivePump::live_dense_reorder) {
        eprintln!("live backend fixture did not apply a dense keyed reorder");
        std::process::exit(1);
    }
    if !live
        .primary_mut()
        .is_some_and(LivePump::live_fragment_anchor)
    {
        eprintln!("live backend fixture did not apply empty and fragment transitions");
        std::process::exit(1);
    }
    let control = LiveRangeControl::NumberBox;
    if !live
        .primary_mut()
        .is_some_and(|pump| pump.live_range_feedback(control))
    {
        eprintln!("live backend fixture did not apply NumberBox feedback updates");
        std::process::exit(1);
    }
    let dispatcher = match DispatcherQueue::GetForCurrentThread() {
        Ok(dispatcher) => dispatcher,
        Err(error) => {
            eprintln!("component scheduler fixture has no dispatcher: {error}");
            std::process::exit(1);
        }
    };
    HOST.with(|host| *host.borrow_mut() = Some(live));
    if queue_live_range_verification(dispatcher, control, 8).is_err() {
        std::process::exit(1);
    }
}

#[cfg(feature = "test")]
fn queue_live_range_verification(
    dispatcher: DispatcherQueue,
    control: LiveRangeControl,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        let name = control.name();
        let events = LIVE_RANGE_EVENTS.with(std::cell::Cell::get);
        let value = HOST.with(|host| {
            host.borrow()
                .as_ref()
                .and_then(LiveHost::primary)
                .map(LivePump::live_range_value)
        });
        if events != 0 {
            eprintln!("{name} delivered {events} programmatic feedback events: value={value:?}");
            std::process::exit(1);
        }
        if attempts == 0 {
            if !matches!(value, Some(Ok(value)) if value == 5.0) {
                eprintln!("{name} did not report its tightened-bound value: {value:?}");
                std::process::exit(1);
            }
            let restored = HOST.with(|host| {
                host.borrow_mut()
                    .as_mut()
                    .and_then(LiveHost::primary_mut)
                    .is_some_and(|pump| pump.live_range_restore(control))
            });
            if !restored {
                eprintln!("{name} did not apply its relaxed-bound update");
                std::process::exit(1);
            }
            if queue_live_range_restore_verification(next_dispatcher.clone(), control, 8).is_err() {
                std::process::exit(1);
            }
            return;
        }
        if queue_live_range_verification(next_dispatcher.clone(), control, attempts - 1).is_err() {
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected range feedback verification",
        ))
    }
}

#[cfg(feature = "test")]
fn queue_live_range_restore_verification(
    dispatcher: DispatcherQueue,
    control: LiveRangeControl,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        let name = control.name();
        let events = LIVE_RANGE_EVENTS.with(std::cell::Cell::get);
        let value = HOST.with(|host| {
            host.borrow()
                .as_ref()
                .and_then(LiveHost::primary)
                .map(LivePump::live_range_value)
        });
        if events != 0 {
            eprintln!("{name} delivered {events} programmatic feedback events: value={value:?}");
            std::process::exit(1);
        }
        if matches!(value, Some(Ok(value)) if value == 7.0) {
            match control {
                LiveRangeControl::NumberBox => {
                    let clear_passed = HOST.with(|host| {
                        host.borrow_mut()
                            .as_mut()
                            .and_then(LiveHost::primary_mut)
                            .is_some_and(|pump| pump.live_range_clear_is_idempotent(control))
                    });
                    if !clear_passed {
                        eprintln!("NumberBox clear feedback was not idempotent");
                        std::process::exit(1);
                    }
                    let slider = LiveRangeControl::Slider;
                    let started = HOST.with(|host| {
                        host.borrow_mut()
                            .as_mut()
                            .and_then(LiveHost::primary_mut)
                            .is_some_and(|pump| pump.live_range_feedback(slider))
                    });
                    if !started {
                        eprintln!("live backend fixture did not apply Slider feedback updates");
                        std::process::exit(1);
                    }
                    if queue_live_range_verification(next_dispatcher.clone(), slider, 8).is_err() {
                        std::process::exit(1);
                    }
                }
                LiveRangeControl::Slider => {
                    let clear_passed = HOST.with(|host| {
                        host.borrow_mut()
                            .as_mut()
                            .and_then(LiveHost::primary_mut)
                            .is_some_and(|pump| pump.live_range_clear_is_idempotent(control))
                    });
                    if !clear_passed {
                        eprintln!("Slider clear feedback was not idempotent");
                        std::process::exit(1);
                    }
                    let slots_passed = HOST.with(|host| {
                        host.borrow_mut()
                            .as_mut()
                            .and_then(LiveHost::primary_mut)
                            .is_some_and(LivePump::live_named_slots)
                    });
                    if !slots_passed {
                        eprintln!("live backend fixture did not update NavigationView named slots");
                        std::process::exit(1);
                    }
                    let progress_bar_passed = HOST.with(|host| {
                        host.borrow_mut()
                            .as_mut()
                            .and_then(LiveHost::primary_mut)
                            .is_some_and(LivePump::live_progress_bar)
                    });
                    if !progress_bar_passed {
                        eprintln!("live backend fixture did not update ProgressBar properties");
                        std::process::exit(1);
                    }
                    let toggle_passed = HOST.with(|host| {
                        host.borrow_mut()
                            .as_mut()
                            .and_then(LiveHost::primary_mut)
                            .is_some_and(LivePump::live_toggle_switch)
                    });
                    if !toggle_passed {
                        eprintln!(
                            "live backend fixture did not suppress ToggleSwitch setter feedback"
                        );
                        std::process::exit(1);
                    }
                    let grid_passed = HOST.with(|host| {
                        host.borrow_mut()
                            .as_mut()
                            .and_then(LiveHost::primary_mut)
                            .is_some_and(LivePump::live_grid)
                    });
                    if !grid_passed {
                        eprintln!("live backend fixture did not apply or clear Grid state");
                        std::process::exit(1);
                    }
                    let split_view_passed = HOST.with(|host| {
                        host.borrow_mut()
                            .as_mut()
                            .and_then(LiveHost::primary_mut)
                            .is_some_and(LivePump::live_split_view)
                    });
                    if !split_view_passed {
                        eprintln!("live backend fixture did not apply SplitView state");
                        std::process::exit(1);
                    }
                    let prepared = HOST.with(|host| {
                        host.borrow_mut()
                            .as_mut()
                            .and_then(LiveHost::primary_mut)
                            .is_some_and(LivePump::live_component_update)
                    });
                    if !prepared {
                        eprintln!(
                            "live backend fixture did not apply a component structural update"
                        );
                        std::process::exit(1);
                    }
                    if queue_live_component_verification(next_dispatcher.clone(), 8).is_err() {
                        std::process::exit(1);
                    }
                }
            }
            return;
        }
        if attempts == 0
            || queue_live_range_restore_verification(next_dispatcher.clone(), control, attempts - 1)
                .is_err()
        {
            eprintln!("{name} did not restore its desired value after relaxing bounds: {value:?}");
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected range restore verification",
        ))
    }
}

#[cfg(feature = "test")]
fn queue_live_component_verification(
    dispatcher: DispatcherQueue,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        let passed = HOST.with(|host| {
            host.borrow()
                .as_ref()
                .and_then(LiveHost::primary)
                .is_some_and(LivePump::live_component_message_result)
        });
        if passed {
            finish_live_component_test();
            return;
        }
        if attempts == 0
            || queue_live_component_verification(next_dispatcher.clone(), attempts - 1).is_err()
        {
            let state = HOST.with(|host| {
                host.borrow().as_ref().map(|host| {
                    (
                        host.windows.len(),
                        host.primary().map(LivePump::live_component_text),
                        host.primary().map(LivePump::native_work_pending),
                    )
                })
            });
            eprintln!(
                "component scheduler fixture did not drain and rearm its message backlog: \
                 state={state:?}, dispatches={}",
                LIVE_TEST_DISPATCHES.with(|count| count.get()),
            );
            eprintln!(
                "component counts: creates={}, setups={}, cleanups={}",
                LIVE_COMPONENT_CREATES.with(|count| count.get()),
                LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.get()),
                LIVE_COMPONENT_EFFECT_CLEANUPS.with(|count| count.get()),
            );
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected component scheduler verification",
        ))
    }
}

#[cfg(feature = "test")]
fn finish_live_component_test() {
    let dispatcher = match DispatcherQueue::GetForCurrentThread() {
        Ok(dispatcher) => dispatcher,
        Err(error) => {
            eprintln!("closed-task fixture has no dispatcher: {error}");
            std::process::exit(1);
        }
    };
    std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while !LIVE_CLOSED_TASK_FINISHED.load(std::sync::atomic::Ordering::Acquire) {
            if std::time::Instant::now() >= deadline {
                eprintln!("closed-task fixture did not finish its worker");
                std::process::exit(1);
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let verify = DispatcherQueueHandler::new(move || {
            if LIVE_CLOSED_TASK_DELIVERED.with(std::cell::Cell::get) {
                eprintln!("closed secondary window received a background completion");
                std::process::exit(1);
            }
            let window = HOST.with(|host| {
                host.borrow()
                    .as_ref()
                    .and_then(LiveHost::primary_window_for_test)
            });
            if window.is_none_or(|window| window.Close().is_err()) {
                eprintln!("component scheduler fixture could not close its primary window");
                std::process::exit(1);
            }
        });
        if !matches!(
            dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify),
            Ok(true)
        ) {
            eprintln!("dispatcher rejected closed-task verification");
            std::process::exit(1);
        }
    });
}

#[cfg(feature = "test")]
fn finish_live_closed_test() {
    if LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.get()) != 1
        || LIVE_COMPONENT_EFFECT_CLEANUPS.with(|count| count.get()) != 1
        || live_test_cleanup_count() != 1
    {
        eprintln!("live component effect setup or cleanup count was incorrect");
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn pump_error(error: PumpError) -> windows_core::Error {
    match error {
        PumpError::NativeApplyFailed(error) => {
            eprintln!(
                "windows-reactor-next fatal native command failure at {}: {:?}",
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
