pub fn to_snake(camel: &str) -> String {
    let mut result = String::new();
    append_snake(&mut result, camel);
    result
}

pub fn append_snake(result: &mut String, camel: &str) {
    // TODO: also deal with these anomolies:
    //      create_u_int8_array - should be 'create_u8_array'
    //      u_i - should be 'ui'

    if camel == "UI" {
        result.push_str("ui");
    } else {
        for c in camel.chars() {
            if c.is_uppercase() {
                if !result.is_empty() {
                    result.push('_');
                }
                for c in c.to_lowercase() {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }
    }
}
