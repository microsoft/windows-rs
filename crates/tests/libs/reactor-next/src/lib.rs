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
    fn generated_structural_builders_preserve_content_and_keys() {
        let button = Button::new().content(TextBlock::new().text("button"));
        let stack = StackPanel::new()
            .child("first", TextBlock::new().text("one"))
            .children([KeyedElement::new(2_u64, TextBlock::new().text("two"))]);

        assert!(matches!(
            button.content_element(),
            Some(Element::TextBlock(_))
        ));
        assert_eq!(stack.child_elements().len(), 1);
        assert_eq!(stack.child_elements()[0].key(), &Key::Integer(2));
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
