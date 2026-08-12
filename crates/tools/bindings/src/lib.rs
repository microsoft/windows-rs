use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use windows_bindgen2::{Filter, Layout, Metadata, Request, format};
use windows_metadata2::{Database, Image};

const REQUESTS: [&str; 17] = [
    "collections.txt",
    "core.txt",
    "cppwinrt.txt",
    "future_impl.txt",
    "future.txt",
    "metadata.txt",
    "numerics.txt",
    "registry.txt",
    "result.txt",
    "strings.txt",
    "version.txt",
    "threading.txt",
    "time.txt",
    "services.txt",
    "canvas.txt",
    "animation.txt",
    "window.txt",
];

#[derive(Clone, Copy)]
enum Projection {
    Default,
    Sys,
    Minimal,
}

struct ToolRequest {
    output: PathBuf,
    filter: Filter,
    projection: Projection,
}

struct Generated {
    output: PathBuf,
    contents: String,
}

#[derive(Default)]
struct Timings {
    database: Duration,
    requests: Duration,
    metadata: Duration,
    selection: Duration,
    rendering: Duration,
    formatting: Duration,
    writing: Duration,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let total = Instant::now();
    let (generated, mut timings) = generate()?;

    let start = Instant::now();
    for generated in generated {
        write_if_changed(&generated.contents, generated.output)?;
    }
    timings.writing = start.elapsed();

    println!(
        "Finished in {:.2}s (database {:.2}s, requests {:.2}s, metadata {:.2}s, \
         selection {:.2}s, rendering {:.2}s, formatting {:.2}s, writing {:.2}s)",
        total.elapsed().as_secs_f32(),
        timings.database.as_secs_f32(),
        timings.requests.as_secs_f32(),
        timings.metadata.as_secs_f32(),
        timings.selection.as_secs_f32(),
        timings.rendering.as_secs_f32(),
        timings.formatting.as_secs_f32(),
        timings.writing.as_secs_f32(),
    );
    Ok(())
}

fn generate() -> Result<(Vec<Generated>, Timings), Box<dyn std::error::Error>> {
    let mut timings = Timings::default();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");

    let start = Instant::now();
    let database = Database::new([
        Image::new(windows_default::WINRT)?,
        Image::new(windows_default::WIN32)?,
    ])?;
    timings.database = start.elapsed();

    let start = Instant::now();
    let requests = REQUESTS
        .iter()
        .map(|name| {
            let path = root.join("crates/tools/bindings/src").join(name);
            let mut request = parse_request(&database, &std::fs::read_to_string(path)?)?;
            request.output = root.join(request.output);
            Ok(request)
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    timings.requests = start.elapsed();

    let start = Instant::now();
    let metadata = Metadata::new(database)?;
    timings.metadata = start.elapsed();

    let mut generated = Vec::with_capacity(requests.len());
    for request in requests {
        let ToolRequest {
            output,
            filter,
            projection,
        } = request;
        let request = match projection {
            Projection::Default => Request::filtered(filter),
            Projection::Sys => Request::filtered(filter).sys(),
            Projection::Minimal => Request::filtered(filter).minimal(),
        };

        let start = Instant::now();
        let generator = metadata.generator(request)?;
        timings.selection += start.elapsed();

        let start = Instant::now();
        let tokens = generator.render(Layout::Flat)?;
        timings.rendering += start.elapsed();

        let start = Instant::now();
        let contents = format(&tokens.to_string())?;
        timings.formatting += start.elapsed();

        generated.push(Generated { output, contents });
    }

    Ok((generated, timings))
}

fn parse_request(
    database: &Database,
    source: &str,
) -> Result<ToolRequest, Box<dyn std::error::Error>> {
    let mut output = None;
    let mut filter = Filter::new();
    let mut projection = Projection::Default;
    let mut filtering = false;

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        if line.starts_with("--") {
            filtering = line == "--filter";
            if let Some(path) = line.strip_prefix("--out ") {
                output = Some(PathBuf::from(path));
            }
            for option in line.split_whitespace() {
                projection = match option {
                    "--sys" => Projection::Sys,
                    "--minimal" => Projection::Minimal,
                    _ => projection,
                };
            }
            continue;
        }
        if filtering {
            include_filter_path(database, &mut filter, line)?;
        }
    }

    Ok(ToolRequest {
        output: output.ok_or("request has no output path")?,
        filter,
        projection,
    })
}

fn include_filter_path(
    database: &Database,
    filter: &mut Filter,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    filter.include_path(database, path)?;
    Ok(())
}

fn write_if_changed(contents: &str, path: PathBuf) -> Result<(), std::io::Error> {
    if std::fs::read_to_string(&path).is_ok_and(|existing| existing == contents) {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, contents)
}

#[cfg(test)]
mod tests {
    #[test]
    fn requests_match_committed_output() {
        let (generated, _) = super::generate().unwrap();
        for generated in generated {
            assert_eq!(
                generated.contents,
                std::fs::read_to_string(&generated.output).unwrap(),
                "{}",
                generated.output.display()
            );
        }
    }
}
