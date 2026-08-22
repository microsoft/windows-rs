#[cfg(test)]
mod tests {
    use windows_reactor_next::*;

    #[test]
    fn generated_builders_compose_elements() {
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

        assert!(matches!(Element::from(text), Element::TextBlock(_)));
        assert!(matches!(Element::from(stack), Element::StackPanel(_)));
        assert!(matches!(Element::from(button), Element::Button(_)));
        assert!(matches!(Element::from(text_box), Element::TextBox(_)));
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
}
