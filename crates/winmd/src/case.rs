/// Change a CamelCase string to snake case
///  
/// This prepends the optional preamble to the string in an efficient way.
///
/// # Panics
///
/// This panics in debug mode if called with an empty string
pub(crate) fn to_snake(camel: &str, preamble: Option<MethodKind>) -> String {
    debug_assert!(!camel.is_empty(), "Called `to_snake` with empty string");

    let preamble = match preamble {
        None => "",
        Some(kind) => kind.into(),
    };

    let mut snake = String::with_capacity(preamble.len() + camel.len());
    snake.push_str(preamble);

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
            snake.push('_');
        }

        for c in current.to_lowercase() {
            since_last_underscore += 1;

            snake.push(c);
        }
    }

    // last character as lowercase
    snake.extend(chars.next().unwrap().to_lowercase());

    snake
}

#[derive(Copy, Clone)]
pub(crate) enum MethodKind {
    Set,
    Remove,
}

impl std::convert::From<MethodKind> for &'static str {
    fn from(kind: MethodKind) -> Self {
        match kind {
            MethodKind::Set => "set_",
            MethodKind::Remove => "remove_",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_snake_works() {
        assert_eq!(to_snake("Windows", None), "windows".to_owned());
        assert_eq!(
            to_snake("ApplicationModel", None),
            "application_model".to_owned()
        );
        assert_eq!(to_snake("foo", None), "foo".to_owned());
        assert_eq!(to_snake("UIProgramming", None), "ui_programming".to_owned());
        assert_eq!(
            to_snake("UIProgramming", Some(MethodKind::Set)),
            "set_ui_programming".to_owned()
        );
        assert_eq!(
            to_snake("CreateUInt8Array", None),
            "create_uint8_array".to_owned()
        );
        assert_eq!(
            to_snake("Socks", Some(MethodKind::Remove)),
            "remove_socks".to_owned()
        );
    }
}
