#[cfg(test)]
mod tests {
    use windows_reactor_next::*;

    #[test]
    fn generated_builders_preserve_property_state() {
        let text = TextBlock::new()
            .text("hello")
            .text_wrapping(TextWrapping::Wrap);
        let stack = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0);
        let button = Button::new().is_enabled(false);
        let text_box = TextBox::new()
            .text("value")
            .placeholder_text("hint")
            .is_enabled(true);

        assert_eq!(
            text.text_property().as_set().map(String::as_str),
            Some("hello")
        );
        assert_eq!(
            text.text_wrapping_property().as_set(),
            Some(&TextWrapping::Wrap)
        );
        assert_eq!(
            stack.orientation_property().as_set(),
            Some(&Orientation::Horizontal)
        );
        assert_eq!(stack.spacing_property().as_set(), Some(&8.0));
        assert_eq!(button.is_enabled_property(), &Property::Set(false));
        assert_eq!(
            text_box
                .placeholder_text_property()
                .as_set()
                .map(String::as_str),
            Some("hint")
        );

        assert!(matches!(Element::from(text), Element::TextBlock(_)));
        assert!(matches!(Element::from(stack), Element::StackPanel(_)));
    }

    #[test]
    fn generated_structural_capabilities_compose_views() {
        let _: View = Button::new().content(TextBlock::new().text("button"));
        let _: View = StackPanel::new().keyed_children([
            KeyedView::new("first", TextBlock::new().text("one")),
            KeyedView::new(2_u64, TextBlock::new().text("two")),
        ]);
        let repeater = ItemsRepeater::new()
            .item("first", TextBlock::new().text("one"))
            .items([KeyedView::new(
                2_u64,
                View::component::<TestComponent>("two".to_string()),
            )]);
        let _: View = ScrollViewer::new().content(repeater);
        let _: View = NavigationView::new().slots([
            SlotView::new(NavigationViewSlot::Content, TextBlock::new()),
            SlotView::new(NavigationViewSlot::Header, Button::new()),
        ]);
    }

    struct TestComponent;

    impl Component for TestComponent {
        type Message = ();
        type Props = String;

        fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self
        }

        fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

        fn view(&self, props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
            TextBlock::new().text(props.clone()).into()
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

    #[test]
    fn generated_callbacks_use_latest_payload_shape() {
        let value = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let captured = std::rc::Rc::clone(&value);
        let text_box = TextBox::new().on_text_changed(move |text| {
            *captured.borrow_mut() = text;
        });

        assert!(
            text_box
                .on_text_changed_callback()
                .unwrap()
                .call("updated".to_string())
        );

        assert_eq!(&*value.borrow(), "updated");
    }
}
