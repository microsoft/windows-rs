use crate::Result;

pub fn from_process() -> Result<Vec<String>> {
    let mut result = vec![];
    from_iter(&mut result, std::env::args().skip(1))?;
    Ok(result)
}

fn from_string(result: &mut Vec<String>, value: &str) -> Result<()> {
    let value = if let Some((value, _)) = value.split_once('#') {
        value
    } else {
        value
    };

    from_iter(result, value.split_whitespace().map(|arg| arg.to_string()))?;
    Ok(())
}

fn from_iter<T: Iterator<Item = String>>(result: &mut Vec<String>, args: T) -> Result<()> {
    let mut expand = false;

    for arg in args {
        if arg.starts_with('-') {
            expand = false;
        }
        if expand {
            for args in crate::read_file_lines(&arg)? {
                from_string(result, &args)?;
            }
        } else if arg == "--etc" {
            expand = true;
        } else {
            result.push(arg);
        }
    }

    Ok(())
}
