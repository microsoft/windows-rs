use super::*;

pub(super) struct AccessibilityFixture {
    notify: ElementRef<Button>,
    panel: ElementRef<Grid>,
    status: u32,
    region: u32,
    quiet: u32,
    silenced: bool,
    notified: Option<Result<(), IntegrationError>>,
    panel_notified: Option<Result<(), IntegrationError>>,
}

#[derive(Clone)]
pub(super) enum Message {
    Status,
    Region,
    Quiet,
    Silence,
    Notify,
    NotifyPanel,
    Notified(Result<(), IntegrationError>),
    PanelNotified(Result<(), IntegrationError>),
}

impl Component for AccessibilityFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            notify: ElementRef::new(),
            panel: ElementRef::new(),
            status: 0,
            region: 0,
            quiet: 0,
            silenced: false,
            notified: None,
            panel_notified: None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Status => self.status += 1,
            Message::Region => self.region += 1,
            Message::Quiet => self.quiet += 1,
            Message::Silence => self.silenced = true,
            Message::Notify => {
                let completed = context.callback(Message::Notified);
                assert!(
                    self.notify.request_notification_result(
                        AutomationNotification::new("Marked done")
                            .kind(AutomationNotificationKind::ActionCompleted)
                            .processing(AutomationNotificationProcessing::ImportantMostRecent)
                            .activity_id("fixture"),
                        move |result| completed.call(result),
                    )
                );
            }
            Message::NotifyPanel => {
                let completed = context.callback(Message::PanelNotified);
                assert!(self.panel.request_notification_result(
                    AutomationNotification::new("Panel"),
                    move |result| completed.call(result),
                ));
            }
            Message::Notified(result) => self.notified = Some(result),
            Message::PanelNotified(result) => self.panel_notified = Some(result),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: accessibility");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 600.0));
        let live = if self.silenced {
            AutomationLiveSetting::Off
        } else {
            AutomationLiveSetting::Polite
        };
        let button = |id: &str, message| {
            Button::new()
                .automation_id(id)
                .content(id.to_string())
                .on_click(context.message(message))
        };
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                button("change-status", Message::Status),
                button("change-region", Message::Region),
                button("change-quiet", Message::Quiet),
                button("silence", Message::Silence),
                button("notify-panel", Message::NotifyPanel),
                Button::new()
                    .automation_id("notify")
                    .content("notify")
                    .automation_help_text("Announces the result")
                    .automation_full_description("Raises a UI Automation notification")
                    .element_ref(&self.notify)
                    .on_click(context.message(Message::Notify)),
                // A layout change on a live region must not announce it.
                TextBlock::new()
                    .automation_id("status")
                    .automation_live_setting(live)
                    .margin(f64::from(self.quiet))
                    .text(format!("Status {}", self.status)),
                Button::new()
                    .automation_id("region")
                    .automation_live_setting(AutomationLiveSetting::Assertive)
                    .content(format!("Region {}", self.region)),
                TextBlock::new()
                    .automation_id("quiet")
                    .text(format!("Quiet {}", self.quiet)),
                StackPanel::new().children(
                    self.silenced
                        .then(|| {
                            TextBlock::new()
                                .automation_id("added")
                                .automation_live_setting(AutomationLiveSetting::Polite)
                                .text("Added")
                                .into()
                        })
                        .into_iter()
                        .collect::<Vec<View>>(),
                ),
                Grid::new()
                    .element_ref(&self.panel)
                    .children((TextBlock::new().text("Panel"),)),
                TextBlock::new().automation_id("result").text(format!(
                    "Notified {:?}; panel {:?}",
                    self.notified, self.panel_notified
                )),
            ))
            .into()
    }
}
