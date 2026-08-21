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
        let _: View =
            StackPanel::new().children([TextBlock::new().text("one").into(), Button::new().into()]);
        let _: View = StackPanel::new().keyed_children([
            KeyedView::new("first", TextBlock::new().text("one")),
            KeyedView::new(2_u64, TextBlock::new().text("two")),
        ]);
        let repeater = ItemsRepeater::new()
            .item("first", TextBlock::new().text("one"))
            .items([KeyedElement::new(2_u64, TextBlock::new().text("two"))]);
        let _: View = ScrollViewer::new().content(repeater.clone());
        let _: View = NavigationView::new().slots([
            SlotView::new(NavigationViewSlot::Content, TextBlock::new()),
            SlotView::new(NavigationViewSlot::Header, Button::new()),
        ]);

        assert_eq!(repeater.item_elements()[0].key(), &Key::from(2_u64));
    }

    #[test]
    fn generated_callbacks_use_latest_payload_shape() {
        let value = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let captured = std::rc::Rc::clone(&value);
        let text_box = TextBox::new().on_text_changed(move |text| {
            *captured.borrow_mut() = text;
        });

        text_box
            .on_text_changed_callback()
            .unwrap()
            .call("updated".to_string());

        assert_eq!(&*value.borrow(), "updated");
    }
}
