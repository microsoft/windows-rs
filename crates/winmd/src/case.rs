/// Change a CamelCase string to snake case
///  
/// This prepends the `preamble` to the string in an efficient way.
/// The preamble must already be in snake case
pub fn to_snake(preamble: &str, camel: &str) -> String {
    match (preamble, camel) {
        ("", "") => return String::new(),
        (_, "") => return preamble.to_owned(),
        _ => {}
    }

    let mut snake = String::with_capacity(preamble.len() + camel.len());
    snake.push_str(preamble);
    // add '_' if preamble was present
    if !snake.is_empty() {
        snake.push('_');
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_snake_works() {
        assert_eq!(to_snake("", "Windows"), "windows".to_owned());
        assert_eq!(
            to_snake("", "ApplicationModel"),
            "application_model".to_owned()
        );
        assert_eq!(to_snake("foo", ""), "foo".to_owned());
        assert_eq!(to_snake("", "UIProgramming"), "ui_programming".to_owned());
        assert_eq!(
            to_snake("awesome", "UIProgramming"),
            "awesome_ui_programming".to_owned()
        );
        assert_eq!(
            to_snake("", "CreateUInt8Array"),
            "create_uint8_array".to_owned()
        );
        assert_eq!(to_snake("awesome", "Socks"), "awesome_socks".to_owned());
    }
}
