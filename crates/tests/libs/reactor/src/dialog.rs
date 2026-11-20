use super::*;

#[allow(dead_code, nonstandard_style, clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\callback_bindings.rs"));
}

pub(super) struct ShutdownApplication {
    queued: bool,
    closed: bool,
}

#[derive(Clone, Copy)]
pub(super) enum ShutdownMessage {
    Open,
    Queued,
    Closed,
    Exit,
}

impl Application for ShutdownApplication {
    type Input = ();
    type Message = ShutdownMessage;

    fn create(_: &(), context: &ApplicationContext<Self>) -> Self {
        context.show_window("main");
        Self {
            queued: false,
            closed: false,
        }
    }

    fn update(&mut self, message: ShutdownMessage, context: &ApplicationContext<Self>) {
        match message {
            ShutdownMessage::Open => context.show_window("dialog"),
            ShutdownMessage::Queued => self.queued = true,
            ShutdownMessage::Closed => self.closed = true,
            ShutdownMessage::Exit => context.exit(),
        }
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        ApplicationView::new()
            .window::<ShutdownController>("main", (self.queued, self.closed, context.forward()))
            .window::<QueuedWindow>("dialog", context.forward())
    }
}

struct ShutdownController(bool);

impl Component for ShutdownController {
    type Input = (bool, bool, Callback<ShutdownMessage>);
    type Message = ();

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.0 = true;
    }

    fn view(
        &self,
        (queued, closed, callback): &Self::Input,
        context: &mut ViewContext<Self>,
    ) -> View {
        context.window_title("Reactor UI fixture: dialog-application");
        let closed = *closed;
        let completion = context.completion();
        context.use_effect("closed", closed, move || {
            if closed {
                // Dialog completion queues Normal-priority work. Observe a later dispatcher turn
                // while this window is still alive, rather than exiting with pending work.
                let dispatcher = bindings::DispatcherQueue::GetForCurrentThread().unwrap();
                assert!(
                    dispatcher
                        .TryEnqueueWithPriority(
                            bindings::DispatcherQueuePriority::Low,
                            &bindings::DispatcherQueueHandler::new(move || {
                                assert!(completion.clone().complete(()));
                            }),
                        )
                        .unwrap()
                );
            }
            None
        });
        let open = callback.clone();
        let exit = callback.clone();
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Button::new()
                    .automation_id("open-window")
                    .content("Open dialog window")
                    .on_click(move || open.call(ShutdownMessage::Open)),
                Button::new()
                    .automation_id("exit-application")
                    .content("Exit application")
                    .on_click(move || exit.call(ShutdownMessage::Exit)),
                TextBlock::new()
                    .automation_id("shutdown-state")
                    .text(format!(
                        "Queued {queued}; closed {closed}; drained {}",
                        self.0
                    )),
            ))
            .into()
    }
}

struct QueuedWindow {
    queued: bool,
    callback: Callback<ShutdownMessage>,
}

#[derive(Clone, Copy)]
enum QueuedMessage {
    Queue,
    Close,
}

impl Drop for QueuedWindow {
    fn drop(&mut self) {
        self.callback.call(ShutdownMessage::Closed);
    }
}

impl Component for QueuedWindow {
    type Input = Callback<ShutdownMessage>;
    type Message = QueuedMessage;

    fn create(callback: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self {
            queued: false,
            callback: callback.clone(),
        }
    }

    fn update(&mut self, message: QueuedMessage, context: &ComponentContext<Self>) {
        match message {
            QueuedMessage::Queue => self.queued = true,
            QueuedMessage::Close => assert!(context.close_window()),
        }
    }

    fn view(&self, _: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let queued = self.queued;
        let callback = self.callback.clone();
        context.use_effect("queued", queued, move || {
            if queued {
                // Both owners are already loaded. Publish only after the second open request
                // has been applied behind the visible first dialog.
                callback.call(ShutdownMessage::Queued);
            }
            None
        });
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                TitleBar::new()
                    .title("Dialog shutdown")
                    .content(Border::new()),
                Border::new().grid_row(1).content_dialog(
                    ContentDialog::new()
                        .title("Queued dialog")
                        .is_open(self.queued)
                        .content(
                            TextBlock::new()
                                .automation_id("queued-dialog-content")
                                .text("This dialog must not open during shutdown."),
                        ),
                ),
            ))
            .content_dialog(
                ContentDialog::new()
                    .title("Visible dialog")
                    .is_open(true)
                    .content(
                        StackPanel::new().spacing(12.0).children((
                            Button::new()
                                .automation_id("queue-dialog")
                                .content("Queue second dialog")
                                .on_click(context.message(QueuedMessage::Queue)),
                            Button::new()
                                .automation_id("close-dialog-window")
                                .content("Close window")
                                .on_click(context.message(QueuedMessage::Close)),
                        )),
                    ),
            )
    }
}

pub(super) struct ShutdownFixture;

impl Component for ShutdownFixture {
    type Input = ();
    type Message = ContentDialogResult;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: ContentDialogResult, context: &ComponentContext<Self>) {
        _ = context.close_window();
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: dialog-shutdown");
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((TitleBar::new()
                .title("Reactor UI fixture: dialog-shutdown")
                .content(Border::new()),))
            .content_dialog(
                ContentDialog::new()
                    .title("Test")
                    .content(
                        TextBlock::new()
                            .automation_id("dialog-content")
                            .text("Closing the main window causes an access violation."),
                    )
                    .primary_button_text("OK")
                    .close_button_text("Close")
                    .on_closed(context.forward())
                    .is_open(true),
            )
    }
}

pub(super) struct DialogFixture {
    open: bool,
    closed: u32,
    result: ContentDialogResult,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Open,
    Closed(ContentDialogResult),
    CloseWithPendingOpen,
    Armed,
    OpenAfterClose,
}

impl Component for DialogFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            open: true,
            closed: 0,
            result: ContentDialogResult::None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Open => self.open = true,
            Message::Closed(result) => {
                self.result = result;
                self.open = false;
                self.closed += 1;
            }
            Message::CloseWithPendingOpen => {
                let completion = context.completion();
                assert!(context.run_window(move |window| {
                    let window = window.as_raw() as usize;
                    let dispatcher = bindings::DispatcherQueue::GetForCurrentThread().unwrap();
                    assert!(
                        dispatcher
                            .TryEnqueueWithPriority(
                                bindings::DispatcherQueuePriority::Normal,
                                &bindings::DispatcherQueueHandler::new(move || {
                                    // Queue outside Reactor's window-work drain so this update
                                    // precedes the host disposal queued by synchronous closure.
                                    assert!(completion.clone().complete(Message::OpenAfterClose));
                                    unsafe {
                                        bindings::SendMessageW(
                                            window as _,
                                            bindings::WM_CLOSE as u32,
                                            0,
                                            0,
                                        );
                                    }
                                    eprintln!("native close returned");
                                }),
                            )
                            .unwrap()
                    );
                    Message::Armed
                }));
            }
            Message::Armed => {}
            Message::OpenAfterClose => {
                eprintln!("queued dialog open delivered");
                self.open = true;
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: dialog");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 400.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Button::new()
                    .automation_id("open")
                    .content("Open dialog")
                    .on_click(context.message(Message::Open))
                    .content_dialog(
                        ContentDialog::new()
                            .title("Lifecycle")
                            .primary_button_text("Accept")
                            .close_button_text("Close")
                            .is_open(self.open)
                            .on_closed(context.callback(Message::Closed))
                            .content(
                                TextBlock::new()
                                    .automation_id("dialog-content")
                                    .text("Requested at mount"),
                            ),
                    ),
                TextBlock::new()
                    .automation_id("closed")
                    .text(format!("Closed {}; result {:?}", self.closed, self.result)),
                Button::new()
                    .automation_id("close-with-pending-open")
                    .content("Close with a queued dialog open")
                    .on_click(context.message(Message::CloseWithPendingOpen)),
            ))
            .into()
    }
}
