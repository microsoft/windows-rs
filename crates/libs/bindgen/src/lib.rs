mod args;
mod error;
mod metadata;
mod rdl;
mod rust;
mod tokens;
mod tree;
mod winmd;

pub use error::{Error, Result};
use tree::Tree;

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
    Config,
}

pub fn bindgen<I, S>(args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let time = std::time::Instant::now();
    let args = args::expand(args)?;

    let mut kind = ArgKind::None;
    let mut output = None;
    let mut input = Vec::<&str>::new();
    let mut include = Vec::<&str>::new();
    let mut exclude = Vec::<&str>::new();
    let mut config = std::collections::BTreeMap::<&str, &str>::new();
    let mut format = false;

    for arg in &args {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "-i" | "--in" => kind = ArgKind::Input,
                "-o" | "--out" => kind = ArgKind::Output,
                "-f" | "--filter" => kind = ArgKind::Filter,
                "--config" => kind = ArgKind::Config,
                "--format" => format = true,
                _ => return Err(Error::new(&format!("invalid option `{arg}`"))),
            },
            ArgKind::Output => {
                if output.is_none() {
                    output = Some(arg.as_str());
                } else {
                    return Err(Error::new("too many outputs"));
                }
            }
            ArgKind::Input => input.push(arg.as_str()),
            ArgKind::Filter => {
                if let Some(rest) = arg.strip_prefix('!') {
                    exclude.push(rest);
                } else {
                    include.push(arg.as_str());
                }
            }
            ArgKind::Config => {
                if let Some((key, value)) = arg.split_once('=') {
                    config.insert(key, value);
                } else {
                    config.insert(arg, "");
                }
            }
        }
    }

    if format {
        if output.is_some() || !include.is_empty() || !exclude.is_empty() {
            return Err(Error::new("`--format` cannot be combined with `--out` or `--filter`"));
        }

        let input = filter_input(&input, &["rdl"])?;

        if input.is_empty() {
            return Err(Error::new("no .rdl inputs"));
        }

        for path in &input {
            read_file_text(path).and_then(|source| rdl::File::parse_str(&source)).and_then(|file| write_to_file(path, file.fmt())).map_err(|err| err.with_path(path))?;
        }

        return Ok(String::new());
    }

    let Some(output) = output else {
        return Err(Error::new("no output"));
    };

    // This isn't strictly necessary but avoids a common newbie pitfall where all metadata
    // would be generated when building a component for a specific API.
    if include.is_empty() {
        return Err(Error::new("at least one `--filter` must be specified"));
    }

    let output = canonicalize(output)?;

    let input = read_input(&input)?;
    let reader = metadata::Reader::new(&input);
    let filter = metadata::Filter::new(&include, &exclude);

    winmd::verify(&reader, &filter)?;

    match extension(&output) {
        "rdl" => rdl::from_reader(&reader, &filter, config, &output)?,
        "winmd" => winmd::from_reader(&reader, &filter, config, &output)?,
        "rs" => rust::from_reader(&reader, &filter, config, &output)?,
        _ => return Err(Error::new("output extension must be one of winmd/rdl/rs")),
    }

    let elapsed = time.elapsed().as_secs_f32();

    if elapsed > 0.1 {
        Ok(format!("  Finished writing `{}` in {:.2}s", output, time.elapsed().as_secs_f32()))
    } else {
        Ok(format!("  Finished writing `{}`", output,))
    }
}

fn filter_input(input: &[&str], extensions: &[&str]) -> Result<Vec<String>> {
    fn try_push(path: &str, extensions: &[&str], results: &mut Vec<String>) -> Result<()> {
        // First canonicalize input so that the extension check below will match the case of the path.
        let path = canonicalize(path)?;

        if extensions.contains(&extension(&path)) {
            results.push(path);
        }

        Ok(())
    }

    let mut results = vec![];

    for input in input {
        let path = std::path::Path::new(input);

        if !path.exists() {
            return Err(Error::new("failed to read input").with_path(input));
        }

        if path.is_dir() {
            for entry in path.read_dir().map_err(|_| Error::new("failed to read directory").with_path(input))?.flatten() {
                let path = entry.path();

                if path.is_file() {
                    try_push(&path.to_string_lossy(), extensions, &mut results)?;
                }
            }
        } else {
            try_push(&path.to_string_lossy(), extensions, &mut results)?;
        }
    }
    Ok(results)
}

fn read_input(input: &[&str]) -> Result<Vec<metadata::File>> {
    let input = filter_input(input, &["winmd", "rdl"])?;
    let mut results = vec![];

    if cfg!(feature = "metadata") {
        results.push(metadata::File::new(std::include_bytes!("../default/Windows.winmd").to_vec()).unwrap());
        results.push(metadata::File::new(std::include_bytes!("../default/Windows.Win32.winmd").to_vec()).unwrap());
        results.push(metadata::File::new(std::include_bytes!("../default/Windows.Wdk.winmd").to_vec()).unwrap());
    } else if input.is_empty() {
        return Err(Error::new("no inputs"));
    }

    for input in &input {
        let file = if extension(input) == "winmd" { read_winmd_file(input)? } else { read_rdl_file(input)? };

        results.push(file);
    }

    Ok(results)
}

fn read_file_text(path: &str) -> Result<String> {
    std::fs::read_to_string(path).map_err(|_| Error::new("failed to read text file"))
}

fn read_file_bytes(path: &str) -> Result<Vec<u8>> {
    std::fs::read(path).map_err(|_| Error::new("failed to read binary file"))
}

fn read_file_lines(path: &str) -> Result<Vec<String>> {
    use std::io::BufRead;
    fn error(path: &str) -> Error {
        Error::new("failed to read lines").with_path(path)
    }
    let file = std::io::BufReader::new(std::fs::File::open(path).map_err(|_| error(path))?);
    let mut lines = vec![];
    for line in file.lines() {
        lines.push(line.map_err(|_| error(path))?);
    }
    Ok(lines)
}

fn read_rdl_file(path: &str) -> Result<metadata::File> {
    read_file_text(path)
        .and_then(|source| rdl::File::parse_str(&source))
        .and_then(|file| file.into_winmd())
        .map(|bytes| {
            // TODO: Write bytes to file if you need to debug the intermediate .winmd file like so:
            _ = write_to_file("temp.winmd", &bytes);

            // Unwrapping here is fine since `rdl_to_winmd` should have produced a valid winmd
            metadata::File::new(bytes).unwrap()
        })
        .map_err(|err| err.with_path(path))
}

fn read_winmd_file(path: &str) -> Result<metadata::File> {
    read_file_bytes(path).and_then(|bytes| metadata::File::new(bytes).ok_or_else(|| Error::new("failed to read .winmd format").with_path(path)))
}

fn write_to_file<C: AsRef<[u8]>>(path: &str, contents: C) -> Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).map_err(|_| Error::new("failed to create directory").with_path(path))?;
    }

    std::fs::write(path, contents).map_err(|_| Error::new("failed to write file").with_path(path))
}

fn canonicalize(value: &str) -> Result<String> {
    let temp = !std::path::Path::new(value).exists();

    // `std::fs::canonicalize` only works if the file exists so we temporarily create it here.
    if temp {
        write_to_file(value, "")?;
    }

    let path = std::fs::canonicalize(value).map_err(|_| Error::new("failed to find path").with_path(value))?;

    if temp {
        std::fs::remove_file(value).map_err(|_| Error::new("failed to remove temporary file").with_path(value))?;
    }

    let path = path.to_string_lossy().trim_start_matches(r"\\?\").to_string();

    match path.rsplit_once('.') {
        Some((file, extension)) => Ok(format!("{file}.{}", extension.to_lowercase())),
        _ => Ok(path),
    }
}

fn extension(path: &str) -> &str {
    path.rsplit_once('.').map_or("", |(_, extension)| extension)
}

fn directory(path: &str) -> &str {
    path.rsplit_once(&['/', '\\']).map_or("", |(directory, _)| directory)
}

fn trim_tick(name: &str) -> &str {
    if name.as_bytes().iter().rev().nth(1) == Some(&b'`') {
        &name[..name.len() - 2]
    } else {
        name
    }
}
