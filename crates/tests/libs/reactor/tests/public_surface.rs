use windows_reactor::*;

#[test]
fn generated_builders_convert_to_views() {
    let optional_text = Some("value");
    let optional_enabled = Some(true);
    let text = TextBlock::new()
        .text("hello")
        .font_size(28.0)
        .text_wrapping(TextWrapping::Wrap);
    let stack = StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(8.0);
    let button = Button::new().is_enabled(false);
    let text_box = TextBox::new()
        .text_optional(optional_text)
        .placeholder_text("hint")
        .is_enabled(optional_enabled);
    let number_box = NumberBox::new().value(None);
    let slider = Slider::new().value(Some(10.0));
    let toggle = ToggleSwitch::new().is_on(None);
    let grid = Grid::new()
        .rows_optional(Some([GridLength::Auto, GridLength::STAR]))
        .columns_optional(None::<[GridLength; 0]>);
    let border = Border::new()
        .padding(Thickness::uniform(24.0))
        .border_thickness(1.0)
        .corner_radius(CornerRadius::uniform(8.0))
        .background(ThemeBrush::CardBackground)
        .border_brush(ThemeBrush::CardStroke);

    let _: View = text.into();
    let _: View = stack.into();
    let _: View = button.into();
    let _: View = text_box.into();
    let _: View = number_box.into();
    let _: View = slider.into();
    let _: View = toggle.into();
    let _: View = grid.into();
    let _: View = border.into();
}

#[test]
fn generated_structural_capabilities_compose_views() {
    let _: View = Button::new().content(TextBlock::new().text("button"));
    let _: View = Border::new().content(TextBlock::new().text("card"));
    let _: View = StackPanel::new().keyed_children([
        ("first", TextBlock::new().text("one")),
        ("second", TextBlock::new().text("two")),
    ]);
    let repeater = ItemsRepeater::new()
        .item("first", TextBlock::new().text("one"))
        .items([(2_u64, View::component::<TestComponent>("two".to_string()))]);
    let _: View = ScrollViewer::new().content(repeater);
    let _: View = NavigationView::new().slots([
        SlotView::new(NavigationViewSlot::Content, TextBlock::new()),
        SlotView::new(NavigationViewSlot::Header, Button::new()),
    ]);
    let _: View = NavigationView::new().collection_slot(
        NavigationViewSlot::MenuItems,
        [
            (
                "first",
                NavigationViewItem::new().slot(NavigationViewItemSlot::Content, "one"),
            ),
            (
                "second",
                NavigationViewItem::new().slot(NavigationViewItemSlot::Content, "two"),
            ),
        ],
    );
    let _: SlotView<NavigationViewSlot> = SlotView::collection(
        NavigationViewSlot::MenuItems,
        [
            (
                "first",
                NavigationViewItem::new().slot(NavigationViewItemSlot::Content, "one"),
            ),
            (
                "second",
                NavigationViewItem::new().slot(NavigationViewItemSlot::Content, "two"),
            ),
        ],
    );
    let _: View = View::keyed_fragment([
        ("first", TextBlock::new().text("one")),
        ("second", TextBlock::new().text("two")),
    ]);
    let _: KeyedView = ("key", TextBlock::new().text("value")).into();
    let _: View = TitleBar::new()
        .preferred_height(WindowTitleBarHeight::Tall)
        .slots(std::iter::empty::<SlotView<TitleBarSlot>>());
    let _: View = TitleBar::new().slots(std::iter::empty::<SlotView<TitleBarSlot>>());
}

struct TestComponent;

impl Component for TestComponent {
    type Message = ();
    type Input = String;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(input.clone()).into()
    }
}

#[test]
fn positional_children_accept_heterogeneous_tuples_without_leaf_conversions() {
    let _: View = StackPanel::new().children((
        TextBlock::new().text("one"),
        TextBox::new().placeholder_text("two"),
        Button::new().content(TextBlock::new().text("button")),
    ));
}

#[test]
fn positional_children_accept_fixed_arrays() {
    let _: View = StackPanel::new().children([
        TextBlock::new().text("first"),
        TextBlock::new().text("second"),
    ]);
}

struct WindowVisualComponent;

impl Component for WindowVisualComponent {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_visuals(
            WindowVisuals::new()
                .theme(WindowTheme::Dark)
                .backdrop(WindowBackdrop::Mica)
                .client_size(1400.0, 900.0),
        );
        TextBlock::new().into()
    }
}

#[test]
fn window_visual_environment_is_public() {
    let _: View = View::component::<WindowVisualComponent>(());
}
