pub fn to_snake(preamble: &str, camel: &str) -> String {
    let mut snake = String::with_capacity(preamble.len() + camel.len());
    snake.push_str(preamble);

    // TODO: deal with anomalies like "UI" and "CreateUint8Array". Not sure
    // whether we should just hard code the well-known ones or somehow use
    // heuristics to do this generically.

    for c in camel.chars() {
        if c.is_uppercase() {
            if !snake.is_empty() {
                snake.push('_');
            }
            for c in c.to_lowercase() {
                snake.push(c);
            }
        } else {
            snake.push(c);
        }
    }

    snake
}
