pub fn to_snake(camel: &str) -> String {
    debug_assert!(!camel.is_empty());
    let mut snake = String::with_capacity(camel.len());
    
    if camel == "WinRT" {
        return "winrt".to_string();
    }

    let mut since_last_underscore = 0;
    let mut chars = camel.chars();

    // first character as lowercased
    for c in chars.next().unwrap().to_lowercase() {
        since_last_underscore += 1;
        snake.push(c);
    }

    // zip together iterator of previous characters and next characters
    for (previous, next) in camel.chars().zip(camel.chars().skip(2)) {
        // safe to unwrap since the iterator of next chars produced something
        let current = chars.next().unwrap();

        // If the current character isn't uppercase we can just push it and move on
        if !current.is_uppercase() {
            since_last_underscore += 1;
            snake.push(current);
            continue;
        }

        if previous.is_lowercase() || next.is_lowercase() && since_last_underscore > 1 {
            since_last_underscore = 0;

            if previous != '_' {
                snake.push('_');
            }
        }

        for c in current.to_lowercase() {
            since_last_underscore += 1;

            snake.push(c);
        }
    }

    if let Some(last) = chars.next() {
        snake.extend(last.to_lowercase());
    }

    snake
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_snake_works() {
        assert_eq!(to_snake("Windows"), "windows".to_owned());

        assert_eq!(to_snake("ApplicationModel"), "application_model".to_owned());

        assert_eq!(
            method_to_snake("foo", MethodKind::Normal),
            "foo".to_owned()
        );

        assert_eq!(
            method_to_snake("UIProgramming", MethodKind::Normal),
            "ui_programming".to_owned()
        );

        assert_eq!(
            method_to_snake("UIProgramming", MethodKind::Set),
            "set_ui_programming".to_owned()
        );

        assert_eq!(
            method_to_snake("CreateUInt8Array", MethodKind::Normal),
            "create_uint8_array".to_owned()
        );

        assert_eq!(
            method_to_snake("Socks", MethodKind::Remove),
            "remove_socks".to_owned()
        );

        assert_eq!(to_snake("appointmentId"), "appointment_id".to_owned());

        assert_eq!(method_to_snake("a", MethodKind::Normal), "a".to_owned());

        assert_eq!(
            method_to_snake("CreateField_Default", MethodKind::Normal),
            "create_field_default".to_owned()
        );

        assert!(to_snake("WinRT") == "winrt");
    }
}
