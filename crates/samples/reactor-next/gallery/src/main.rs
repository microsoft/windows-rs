#![windows_subsystem = "windows"]

use windows_reactor_next::*;

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    NumericInput,
    TextInput,
}

impl Page {
    fn title(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::NumericInput => "Numeric input",
            Self::TextInput => "Text input",
        }
    }
}

struct Gallery {
    amount: f64,
    inputs_enabled: bool,
    name: String,
    page: Page,
    volume: f64,
}

#[derive(Clone)]
enum Message {
    AmountChanged(f64),
    InputsEnabledChanged(bool),
    NameChanged(String),
    Navigate(Page),
    Reset,
    VolumeChanged(f64),
}

#[derive(Clone, PartialEq)]
struct HomeProps {
    amount: f64,
    name: String,
    volume: f64,
}

struct HomePage;

#[derive(Clone, PartialEq)]
struct TextInputProps {
    changed: Callback<String>,
    enabled: bool,
    name: String,
}

struct TextInputPage;

#[derive(Clone, PartialEq)]
struct NumericInputProps {
    amount: f64,
    amount_changed: Callback<f64>,
    enabled: bool,
    volume: f64,
    volume_changed: Callback<f64>,
}

struct NumericInputPage;

fn page_header(title: &str, description: &str) -> View {
    StackPanel::new().spacing(4.0).children((
        TextBlock::new().text(title).font_size(28.0),
        TextBlock::new().text(description),
    ))
}

fn sample_card(title: &str, sample: impl Into<View>, source: &str) -> View {
    StackPanel::new().spacing(8.0).children((
        TextBlock::new().text(title).font_size(14.0),
        Border::new()
            .border_thickness(1.0)
            .corner_radius(8.0)
            .content(
                StackPanel::new().children((
                    Border::new().padding(24.0).content(sample),
                    Border::new()
                        .padding(Thickness::new(12.0, 8.0, 12.0, 8.0))
                        .content(TextBlock::new().text(source).font_size(13.0)),
                )),
            ),
    ))
}

fn page_content(
    title: &str,
    description: &str,
    cards: impl IntoIterator<Item = KeyedView>,
) -> View {
    let children = std::iter::once(KeyedView::new("header", page_header(title, description)))
        .chain(cards)
        .collect::<Vec<_>>();
    ScrollViewer::new().content(
        Border::new()
            .padding(Thickness::new(36.0, 24.0, 36.0, 36.0))
            .content(StackPanel::new().spacing(16.0).keyed_children(children)),
    )
}

impl Component for Gallery {
    type Message = Message;
    type Props = ();

    fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
        Self {
            amount: 42.0,
            inputs_enabled: true,
            name: String::new(),
            page: Page::Home,
            volume: 35.0,
        }
    }

    fn update(&mut self, message: Message, _context: &mut ComponentContext<Self>) {
        match message {
            Message::AmountChanged(value) => self.amount = value,
            Message::InputsEnabledChanged(value) => self.inputs_enabled = value,
            Message::NameChanged(value) => self.name = value,
            Message::Navigate(page) => self.page = page,
            Message::Reset => {
                self.amount = 42.0;
                self.inputs_enabled = true;
                self.name.clear();
                self.volume = 35.0;
            }
            Message::VolumeChanged(value) => self.volume = value,
        }
    }

    fn view(&self, _props: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title(format!("Reactor next gallery - {}", self.page.title()));
        context.window_visuals(
            WindowVisuals::new()
                .theme(WindowTheme::System)
                .backdrop(WindowBackdrop::Mica)
                .client_size(1400.0, 900.0),
        );

        let page = match self.page {
            Page::Home => View::component::<HomePage>(HomeProps {
                amount: self.amount,
                name: self.name.clone(),
                volume: self.volume,
            }),
            Page::NumericInput => View::component::<NumericInputPage>(NumericInputProps {
                amount: self.amount,
                amount_changed: context.callback(Message::AmountChanged),
                enabled: self.inputs_enabled,
                volume: self.volume,
                volume_changed: context.callback(Message::VolumeChanged),
            }),
            Page::TextInput => View::component::<TextInputPage>(TextInputProps {
                changed: context.callback(Message::NameChanged),
                enabled: self.inputs_enabled,
                name: self.name.clone(),
            }),
        };

        SplitView::new()
            .open_pane_length(240.0)
            .display_mode(SplitViewDisplayMode::CompactInline)
            .is_pane_open(true)
            .slots([
                SlotView::new(
                    SplitViewSlot::Pane,
                    StackPanel::new().spacing(8.0).children((
                        TextBlock::new().text("Reactor next gallery"),
                        Button::new()
                            .on_click(context.message(Message::Navigate(Page::Home)))
                            .content(TextBlock::new().text("Home")),
                        Button::new()
                            .on_click(context.message(Message::Navigate(Page::TextInput)))
                            .content(TextBlock::new().text("Text input")),
                        Button::new()
                            .on_click(context.message(Message::Navigate(Page::NumericInput)))
                            .content(TextBlock::new().text("Numeric input")),
                        ToggleSwitch::new()
                            .is_on(self.inputs_enabled)
                            .on_toggled(context.callback(Message::InputsEnabledChanged)),
                        TextBlock::new().text("Inputs enabled"),
                        Button::new()
                            .on_click(context.message(Message::Reset))
                            .content(TextBlock::new().text("Reset")),
                    )),
                ),
                SlotView::new(SplitViewSlot::Content, page),
            ])
    }
}

impl Component for HomePage {
    type Message = ();
    type Props = HomeProps;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        let name = if props.name.is_empty() {
            "not entered".to_string()
        } else {
            props.name.clone()
        };
        page_content(
            "Home",
            "A bounded gallery port for visual composition and durable controlled page data.",
            [KeyedView::new(
                "state",
                sample_card(
                    "Retained application state",
                    StackPanel::new().spacing(8.0).children((
                        TextBlock::new().text(format!("Name: {name}")),
                        TextBlock::new().text(format!("Amount: {:.1}", props.amount)),
                        TextBlock::new().text(format!("Volume: {:.0}%", props.volume)),
                    )),
                    "The gallery shell owns values that must survive page replacement.",
                ),
            )],
        )
    }
}

impl Component for TextInputPage {
    type Message = ();
    type Props = TextInputProps;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        page_content(
            "Text input",
            "A controlled TextBox writes durable data to the gallery shell.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic TextBox",
                        StackPanel::new().spacing(8.0).children((
                            TextBlock::new().text("Name"),
                            TextBox::new()
                                .text(props.name.clone())
                                .placeholder_text("Type a name")
                                .is_enabled(props.enabled)
                                .on_text_changed(props.changed.clone()),
                            TextBlock::new()
                                .text(format!("Characters: {}", props.name.chars().count())),
                        )),
                        "TextBox::new().text(name).on_text_changed(callback)",
                    ),
                ),
                KeyedView::new(
                    "disabled",
                    sample_card(
                        "Disabled TextBox",
                        TextBox::new().text("Read-only content").is_enabled(false),
                        "TextBox::new().text(\"Read-only content\").is_enabled(false)",
                    ),
                ),
            ],
        )
    }
}

impl Component for NumericInputPage {
    type Message = ();
    type Props = NumericInputProps;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        page_content(
            "Numeric input",
            "NumberBox and Slider share controlled parent-owned values.",
            [
                KeyedView::new(
                    "number",
                    sample_card(
                        "Basic NumberBox",
                        StackPanel::new().spacing(8.0).children((
                            TextBlock::new().text("Amount"),
                            NumberBox::new()
                                .minimum(0.0)
                                .maximum(100.0)
                                .value(props.amount)
                                .is_enabled(props.enabled)
                                .on_value_changed(props.amount_changed.clone()),
                            TextBlock::new().text(format!("Amount: {:.1}", props.amount)),
                        )),
                        "NumberBox::new().minimum(0.0).maximum(100.0).value(amount)",
                    ),
                ),
                KeyedView::new(
                    "slider",
                    sample_card(
                        "Slider with progress",
                        StackPanel::new().spacing(8.0).children((
                            TextBlock::new().text("Volume"),
                            Slider::new()
                                .minimum(0.0)
                                .maximum(100.0)
                                .value(props.volume)
                                .is_enabled(props.enabled)
                                .on_value_changed(props.volume_changed.clone()),
                            ProgressBar::new()
                                .minimum(0.0)
                                .maximum(100.0)
                                .value(props.volume)
                                .is_enabled(props.enabled),
                            TextBlock::new().text(format!("Volume: {:.0}%", props.volume)),
                        )),
                        "Slider::new().value(volume).on_value_changed(callback)",
                    ),
                ),
            ],
        )
    }
}

fn main() {
    bootstrap().unwrap();
    App::run_component::<Gallery>(()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn active_property_node(
        pump: &Pump<RecordingRuntime>,
        property: PropertyId,
        value: &PropertyValue,
    ) -> NodeId {
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .rev()
            .find_map(|command| match command {
                Command::SetProperty {
                    node,
                    property: candidate,
                    ..
                } if *candidate == property
                    && pump
                        .runtime()
                        .node(*node)
                        .and_then(|node| node.property(property))
                        == Some(value) =>
                {
                    Some(*node)
                }
                _ => None,
            })
            .unwrap()
    }

    fn active_event_node(pump: &Pump<RecordingRuntime>, event: EventId) -> NodeId {
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .rev()
            .find_map(|command| match command {
                Command::SubscribeEvent {
                    node,
                    event: candidate,
                    ..
                } if *candidate == event && pump.event_revision(*node, event).is_some() => {
                    Some(*node)
                }
                _ => None,
            })
            .unwrap()
    }

    fn click(pump: &mut Pump<RecordingRuntime>, label: &str) {
        let label = active_property_node(
            pump,
            PropertyId::TextBlockText,
            &PropertyValue::Str(label.into()),
        );
        let button = pump
            .runtime()
            .commands()
            .iter()
            .flatten()
            .rev()
            .find_map(|command| match command {
                Command::InsertChild { parent, child, .. }
                    if *child == label
                        && pump.event_revision(*parent, EventId::ButtonClick).is_some() =>
                {
                    Some(*parent)
                }
                _ => None,
            })
            .unwrap();
        queue_event(pump, button, EventId::ButtonClick, EventPayload::Unit);
    }

    fn queue_event(
        pump: &mut Pump<RecordingRuntime>,
        node: NodeId,
        event: EventId,
        payload: EventPayload,
    ) {
        let revision = pump.event_revision(node, event).unwrap();
        pump.queue_event(QueuedEvent::new(node, event, revision, payload));
        assert_eq!(pump.dispatch_events(), Ok(1));
        assert_eq!(pump.dispatch_components(1), Ok(1));
    }

    #[test]
    fn routed_pages_retain_controlled_values() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<Gallery>(())).unwrap();
        let window = pump.window().unwrap();
        assert_eq!(
            pump.runtime().window_title(window),
            Some("Reactor next gallery - Home")
        );
        assert_eq!(
            pump.runtime().window_visuals(window),
            Some(
                WindowVisuals::new()
                    .theme(WindowTheme::System)
                    .backdrop(WindowBackdrop::Mica)
                    .client_size(1400.0, 900.0)
            )
        );
        active_property_node(
            &pump,
            PropertyId::TextBlockFontSize,
            &PropertyValue::F64(28.0),
        );
        active_property_node(
            &pump,
            PropertyId::BorderPadding,
            &PropertyValue::Thickness(Thickness::new(36.0, 24.0, 36.0, 36.0)),
        );
        active_property_node(
            &pump,
            PropertyId::BorderCornerRadius,
            &PropertyValue::CornerRadius(CornerRadius::uniform(8.0)),
        );

        click(&mut pump, "Text input");
        assert_eq!(
            pump.runtime().window_title(window),
            Some("Reactor next gallery - Text input")
        );
        let text_box = active_property_node(
            &pump,
            PropertyId::TextBoxText,
            &PropertyValue::Str(String::new()),
        );
        let text_revision = pump
            .event_revision(text_box, EventId::TextBoxTextChanged)
            .unwrap();
        queue_event(
            &mut pump,
            text_box,
            EventId::TextBoxTextChanged,
            EventPayload::Str("Ada".into()),
        );

        click(&mut pump, "Numeric input");
        assert_eq!(
            pump.runtime().window_title(window),
            Some("Reactor next gallery - Numeric input")
        );
        pump.queue_event(QueuedEvent::new(
            text_box,
            EventId::TextBoxTextChanged,
            text_revision,
            EventPayload::Str("stale".into()),
        ));
        assert_eq!(pump.dispatch_events(), Ok(0));
        assert_eq!(pump.dispatch_components(1), Ok(0));

        let number_box = active_event_node(&pump, EventId::NumberBoxValueChanged);
        queue_event(
            &mut pump,
            number_box,
            EventId::NumberBoxValueChanged,
            EventPayload::F64(64.0),
        );
        let slider = active_event_node(&pump, EventId::SliderValueChanged);
        queue_event(
            &mut pump,
            slider,
            EventId::SliderValueChanged,
            EventPayload::F64(75.0),
        );
        let toggle = active_event_node(&pump, EventId::ToggleSwitchToggled);
        queue_event(
            &mut pump,
            toggle,
            EventId::ToggleSwitchToggled,
            EventPayload::Bool(false),
        );
        assert_eq!(
            pump.runtime()
                .node(number_box)
                .and_then(|node| node.property(PropertyId::NumberBoxIsEnabled)),
            Some(&PropertyValue::Bool(false))
        );
        let slider_enabled = active_property_node(
            &pump,
            PropertyId::SliderIsEnabled,
            &PropertyValue::Bool(false),
        );
        assert!(pump.runtime().node(slider_enabled).is_some());
        let progress_enabled = active_property_node(
            &pump,
            PropertyId::ProgressBarIsEnabled,
            &PropertyValue::Bool(false),
        );
        assert!(pump.runtime().node(progress_enabled).is_some());
        active_property_node(
            &pump,
            PropertyId::ProgressBarValue,
            &PropertyValue::F64(75.0),
        );

        click(&mut pump, "Home");
        assert_eq!(
            pump.runtime().window_title(window),
            Some("Reactor next gallery - Home")
        );
        active_property_node(
            &pump,
            PropertyId::TextBlockText,
            &PropertyValue::Str("Name: Ada".into()),
        );
        active_property_node(
            &pump,
            PropertyId::TextBlockText,
            &PropertyValue::Str("Amount: 64.0".into()),
        );
        active_property_node(
            &pump,
            PropertyId::TextBlockText,
            &PropertyValue::Str("Volume: 75%".into()),
        );

        click(&mut pump, "Text input");
        active_property_node(
            &pump,
            PropertyId::TextBoxText,
            &PropertyValue::Str("Ada".into()),
        );
        active_property_node(
            &pump,
            PropertyId::TextBoxIsEnabled,
            &PropertyValue::Bool(false),
        );

        click(&mut pump, "Numeric input");
        active_property_node(&pump, PropertyId::NumberBoxValue, &PropertyValue::F64(64.0));
        active_property_node(&pump, PropertyId::SliderValue, &PropertyValue::F64(75.0));

        click(&mut pump, "Reset");
        active_property_node(&pump, PropertyId::NumberBoxValue, &PropertyValue::F64(42.0));
        active_property_node(&pump, PropertyId::SliderValue, &PropertyValue::F64(35.0));
        active_property_node(
            &pump,
            PropertyId::NumberBoxIsEnabled,
            &PropertyValue::Bool(true),
        );
    }
}
