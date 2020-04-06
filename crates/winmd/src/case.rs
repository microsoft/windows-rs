pub fn to_snake(preamble: &str, camel: &str) -> String {
    let mut snake = String::with_capacity(preamble.len() + camel.len());
    snake.push_str(preamble);

    // TODO: deal with anomalies like "UI" and "CreateUint8Array". Not sure
    // whether we should just hard code the well-known ones or somehow use
    // heuristics to do this generically. It should handle conversions like
    // this:
    // 
    // "Windows" -> "windows"
    // "UI" -> "ui"
    // "ApplicationModel" -> "application_model"
    // "CreateUInt8Array" -> "create_u8_array"
    //
    // The last one is a stretch goal, but may be too expensive.

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
