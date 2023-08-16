use crate::Result;

pub fn expand<I, S>(args: I) -> Result<Vec<String>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut result = vec![];
    from_iter(&mut result, args)?;
    Ok(result)
}

fn from_string(result: &mut Vec<String>, value: &str) -> Result<()> {
    let value = if let Some((value, _)) = value.split_once('#') { value } else { value };

    from_iter(result, value.split_whitespace().map(|arg| arg.to_string()))?;
    Ok(())
}

fn from_iter<I, S>(result: &mut Vec<String>, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut expand = false;

    for arg in args.into_iter().map(|arg| arg.as_ref().to_string()) {
        if arg.starts_with('-') {
            expand = false;
        }
        if expand {
            for args in crate::read_file_lines(&arg)? {
                if !args.starts_with("//") {
                    from_string(result, &args)?;
                }
            }
        } else if arg == "--etc" {
            expand = true;
        } else {
            result.push(arg);
        }
    }

    Ok(())
}
