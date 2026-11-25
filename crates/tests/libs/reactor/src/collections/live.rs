use super::*;

pub(super) fn run(source: Source) -> Result<()> {
    App::run_component::<Fixture>(source)
}

struct Fixture {
    window: native::Window,
    combo: native::ComboBox,
    suggest: native::AutoSuggestBox,
    breadcrumb: native::BreadcrumbBar,
    radio: native::RadioButtons,
    sources: Vec<ComObject<ViewSource>>,
    source: Source,
    revision: u32,
    state: &'static str,
}

#[derive(Clone, Copy)]
enum Message {
    Select,
    Suggest,
    Replace,
    Clear,
    Verify,
}

impl Fixture {
    fn new(source: Source) -> Result<Self> {
        let root = native::XamlReader::Load(
            "<StackPanel xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
             xmlns:x='http://schemas.microsoft.com/winfx/2006/xaml' Margin='20' Spacing='12'>\
             <ComboBox x:Name='combo' AutomationProperties.AutomationId='native-combo' Width='300'/>\
             <AutoSuggestBox x:Name='suggest' AutomationProperties.AutomationId='native-suggest' \
             Width='300'/>\
             <BreadcrumbBar x:Name='breadcrumb' \
             AutomationProperties.AutomationId='native-breadcrumb'/>\
             <RadioButtons x:Name='radio' AutomationProperties.AutomationId='native-radio'/>\
             </StackPanel>",
        )?
        .cast::<native::FrameworkElement>()?;
        let combo = root.FindName("combo")?.cast()?;
        let suggest = root.FindName("suggest")?.cast()?;
        let breadcrumb = root.FindName("breadcrumb")?.cast()?;
        let radio = root.FindName("radio")?.cast()?;
        let window = native::Window::new()?;
        window.SetTitle("Native collection consumers")?;
        window.SetContent(&root)?;
        let mut fixture = Self {
            window,
            combo,
            suggest,
            breadcrumb,
            radio,
            sources: Vec::new(),
            source,
            revision: 0,
            state: "Mounted",
        };
        fixture.populate()?;
        fixture.window.Activate()?;
        Ok(fixture)
    }

    fn populate(&mut self) -> Result<()> {
        let mut vectors = Vec::new();
        for name in ["Combo", "Suggest", "Crumb", "Radio"] {
            let source =
                source_values((0..3).map(|index| format!("{name} {} {index}", self.revision)))?;
            let vector = match self.source {
                Source::ReadOnly => ComObject::new(ReadOnlyVector(source.clone())).into_interface(),
                Source::Mutable | Source::Shared | Source::Trusted => {
                    let values = source
                        .values
                        .iter()
                        .map(|value| value.resolve().map(Some))
                        .collect::<Result<Vec<_>>>()?;
                    match self.source {
                        Source::Shared => IVector::from_read_only(values)?,
                        Source::Trusted => {
                            // source_values creates only immutable stock string boxes.
                            unsafe { IVector::from_read_only_unchecked(values)? }
                        }
                        Source::Mutable => IVector::from_marshaled(values)?,
                        Source::ReadOnly => unreachable!(),
                    }
                }
            };
            self.sources.push(source);
            vectors.push(vector);
        }
        self.combo
            .cast::<native::IItemsControl>()?
            .SetItemsSource(&vectors[0])?;
        self.suggest
            .cast::<native::IItemsControl>()?
            .SetItemsSource(&vectors[1])?;
        self.breadcrumb.SetItemsSource(&vectors[2])?;
        self.radio.SetItemsSource(&vectors[3])?;
        Ok(())
    }

    fn apply(&mut self, message: Message) -> Result<()> {
        match message {
            Message::Select => {
                self.combo
                    .cast::<native::ISelector>()?
                    .SetSelectedIndex(1)?;
                self.radio.SetSelectedIndex(1)?;
                self.state = "Selected";
            }
            Message::Suggest => {
                self.window.Activate()?;
                assert!(
                    self.suggest
                        .cast::<native::IUIElement>()?
                        .Focus(native::FocusState::Programmatic)?
                );
                self.suggest.SetText("Suggest")?;
                self.suggest.SetIsSuggestionListOpen(true)?;
                self.state = "Suggestions";
            }
            Message::Replace => {
                self.revision += 1;
                self.populate()?;
                self.state = "Replaced";
            }
            Message::Clear => {
                self.combo
                    .cast::<native::IItemsControl>()?
                    .SetItemsSource(None)?;
                self.suggest
                    .cast::<native::IItemsControl>()?
                    .SetItemsSource(None)?;
                self.breadcrumb.SetItemsSource(None)?;
                self.radio.SetItemsSource(None)?;
                self.state = "Cleared";
            }
            Message::Verify => {
                if self.source == Source::ReadOnly {
                    for source in &self.sources {
                        assert!(source.counts.indexed.load(Ordering::Relaxed) > 0);
                    }
                }
                self.state = "Verified";
            }
        }
        for source in &self.sources {
            assert_eq!(source.counts.mutations.load(Ordering::Relaxed), 0);
        }
        Ok(())
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.window.Close().unwrap();
    }
}

impl Component for Fixture {
    type Input = Source;
    type Message = Message;

    fn create(source: &Source, _: &ComponentContext<Self>) -> Self {
        Self::new(*source).unwrap()
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        self.apply(message).unwrap();
    }

    fn view(&self, _: &Source, context: &mut ViewContext<Self>) -> View {
        let fixture = match self.source {
            Source::Mutable => "collection-live-mutable",
            Source::ReadOnly => "collection-live-read-only",
            Source::Shared => "collection-live-shared",
            Source::Trusted => "collection-live-trusted",
        };
        context.window_title(format!("Reactor UI fixture: {fixture}"));
        context.window_visuals(WindowVisuals::new().client_size(380.0, 400.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                TextBlock::new().automation_id("state").text(self.state),
                Button::new()
                    .automation_id("select")
                    .content("Select")
                    .on_click(context.message(Message::Select)),
                Button::new()
                    .automation_id("suggest")
                    .content("Suggestions")
                    .on_click(context.message(Message::Suggest)),
                Button::new()
                    .automation_id("replace")
                    .content("Replace")
                    .on_click(context.message(Message::Replace)),
                Button::new()
                    .automation_id("clear")
                    .content("Clear")
                    .on_click(context.message(Message::Clear)),
                Button::new()
                    .automation_id("verify")
                    .content("Verify")
                    .on_click(context.message(Message::Verify)),
            ))
            .into()
    }
}
