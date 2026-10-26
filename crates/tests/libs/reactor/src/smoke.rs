use super::*;

/// Mounts action and display-only objects. Action controls log their callbacks; the display
/// objects can be updated, removed, and mounted again.
pub(super) struct SmokeFixture {
    generation: u32,
    shown: bool,
    events: Vec<String>,
}

#[derive(Clone)]
pub(super) enum Message {
    Advance,
    Show(bool),
    Event(&'static str),
    Crumb(Rc<str>),
}

impl Component for SmokeFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            generation: 0,
            shown: true,
            events: Vec::new(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Advance => self.generation += 1,
            Message::Show(shown) => self.shown = shown,
            Message::Event(name) => self.events.push(name.to_string()),
            Message::Crumb(item) => self.events.push(format!("crumb {item}")),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: smoke");
        context.window_visuals(WindowVisuals::new().client_size(900.0, 700.0));
        let button = |id: &str, message| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(message))
        };
        let event = |name| context.message(Message::Event(name));
        let actions = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                HyperlinkButton::new()
                    .automation_id("hyperlink")
                    .content("Link")
                    .on_click(event("hyperlink")),
                RepeatButton::new()
                    .automation_id("repeat")
                    .content("Repeat")
                    .on_click(event("repeat")),
                SplitButton::new()
                    .automation_id("split")
                    .content("Split")
                    .on_click(event("split")),
                CommandBar::new().primary_commands((
                    AppBarButton::new()
                        .automation_id("app-bar")
                        .label("Command")
                        .icon(Symbol::Accept)
                        .on_click(event("app-bar")),
                    AppBarSeparator::new(),
                )),
                BreadcrumbBar::new()
                    .automation_id("crumbs")
                    .items_source(["Root", "Leaf"])
                    .on_item_clicked(context.callback(Message::Crumb)),
            ));
        let mut children = vec![
            TitleBar::new()
                .title("Reactor UI fixture: smoke")
                .is_back_button_visible(true)
                .is_back_button_enabled(true)
                .on_back_requested(event("back"))
                .into(),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    button("advance", Message::Advance),
                    button("hide", Message::Show(false)),
                    button("show", Message::Show(true)),
                ))
                .into(),
            TextBlock::new()
                .automation_id("events")
                .text(format!("Events [{}]", self.events.join(", ")))
                .into(),
            actions.into(),
        ];
        if self.shown {
            children.push(self.display());
        }
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children(children)
            .into()
    }
}

impl SmokeFixture {
    fn display(&self) -> View {
        let odd = self.generation % 2 == 1;
        let label = format!("Gen {}", self.generation);
        let text = |id: &str| TextBlock::new().automation_id(id).text(label.as_str());
        let value = if odd { 60.0 } else { 30.0 };
        let color = if odd {
            Color::rgb(200, 40, 40)
        } else {
            Color::rgb(40, 40, 200)
        };
        let shapes = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                Rectangle::new()
                    .width(24.0)
                    .height(24.0)
                    .fill(color)
                    .stroke(color)
                    .stroke_thickness(if odd { 2.0 } else { 1.0 })
                    .radius_x(4.0)
                    .radius_y(4.0),
                Ellipse::new()
                    .width(24.0)
                    .height(24.0)
                    .fill(color)
                    .stroke(color)
                    .stroke_thickness(1.0),
                SymbolIcon::new().symbol(if odd { Symbol::Like } else { Symbol::Home }),
                FontIcon::new()
                    .glyph(if odd { "\u{E734}" } else { "\u{E721}" })
                    .font_size(16.0)
                    .foreground(color),
                PathIcon::new().data(if odd {
                    "M 0,0 L 16,0 L 8,16 Z"
                } else {
                    "M 0,8 L 6,14 L 16,2 Z"
                }),
                BitmapIcon::new().show_as_monochrome(odd),
                ImageIcon::new()
                    .width(16.0)
                    .height(16.0)
                    .source_data(EncodedImage::from_static(image::PNG)),
                AppBarSeparator::new(),
            ));
        let progress = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                ProgressBar::new()
                    .automation_id("progress-bar")
                    .width(120.0)
                    .maximum(100.0)
                    .value(value)
                    .show_paused(odd),
                ProgressRing::new()
                    .automation_id("progress-ring")
                    .width(32.0)
                    .height(32.0)
                    .is_indeterminate(false)
                    .maximum(100.0)
                    .value(value),
                InfoBadge::new()
                    .automation_id("badge")
                    .value(self.generation as i32),
                PersonPicture::new()
                    .automation_id("person")
                    .width(40.0)
                    .height(40.0)
                    .display_name(if odd { "Ada Lovelace" } else { "Alan Turing" }),
            ));
        let containers = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                Viewbox::new()
                    .width(60.0)
                    .height(20.0)
                    .content(text("viewbox-text")),
                ScrollView::new()
                    .width(80.0)
                    .height(40.0)
                    .content(text("scroll-text")),
                VariableSizedWrapGrid::new()
                    .item_width(60.0)
                    .item_height(20.0)
                    .children((text("wrap-text"),)),
                RelativePanel::new().children((text("relative-text"),)),
                RichTextBlock::new()
                    .automation_id("rich")
                    .paragraphs(RichText::single_paragraph([RichTextInline::Run(
                        RichTextRun::plain(label.as_str()),
                    )])),
                Button::new()
                    .automation_id("tipped")
                    .content("Tipped")
                    .tooltip(label.as_str()),
            ));
        let pickers = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                DatePicker::new()
                    .automation_id("date")
                    .header("Date")
                    .year_visible(!odd),
                TimePicker::new()
                    .automation_id("time")
                    .header("Time")
                    .minute_increment(if odd { 15 } else { 5 }),
                ListViewItem::new()
                    .automation_id("list-item")
                    .content(text("list-text")),
                GridView::new().width(100.0).items((GridViewItem::new()
                    .automation_id("grid-item")
                    .content(text("grid-text")),)),
            ));
        StackPanel::new()
            .spacing(8.0)
            .children((shapes, progress, containers, pickers))
            .into()
    }
}
