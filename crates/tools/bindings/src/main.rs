use std::path::{Path, PathBuf};
use windows_bindgen2::{Filter, Layout, Metadata, Request};
use windows_metadata2::{Database, Image};

mod format;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = std::time::Instant::now();
    let database = Database::new([
        Image::new(windows_default::WINRT)?,
        Image::new(windows_default::WIN32)?,
    ])?;
    let requests = REQUESTS
        .iter()
        .map(|name| {
            let path = Path::new("crates/tools/bindings/src").join(name);
            parse_request(&database, &std::fs::read_to_string(path)?)
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    let metadata = Metadata::new(database)?;

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
        let tokens = metadata.generator(request)?.render(Layout::Flat)?;
        write_if_changed(&format::rust(&tokens.to_string())?, output)?;
    }

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
    Ok(())
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
    if let Some((prefix, names)) = path
        .strip_suffix('}')
        .and_then(|path| path.rsplit_once("::{"))
    {
        let parts = path_parts(prefix);
        let (parent, ty) = parts.split_at(parts.len() - 1);
        if parent.is_empty() {
            let namespaces = type_namespaces(database, ty[0]);
            if namespaces.is_empty() {
                return Err(format!("unresolved filter path `{path}`").into());
            }
            for namespace in namespaces {
                for method in names.split(',').map(str::trim) {
                    filter.include_method(namespace, ty[0], method);
                }
            }
        } else if type_exists(database, &parent.join("."), ty[0]) {
            for method in names.split(',').map(str::trim) {
                filter.include_method(parent.join("."), ty[0], method);
            }
        } else {
            let namespace = parts.join(".");
            for name in names.split(',').map(str::trim) {
                filter.include_item(&namespace, name);
            }
        }
        return Ok(());
    }

    let parts = path_parts(path);
    if parts.len() == 1 {
        filter.include_name(parts[0]);
        return Ok(());
    }
    let (namespace, name) = parts.split_at(parts.len() - 1);
    if type_exists(database, &namespace.join("."), name[0])
        || namespace_exists(database, &namespace.join("."))
    {
        filter.include_item(namespace.join("."), name[0]);
        return Ok(());
    }
    let (namespace, ty) = namespace.split_at(namespace.len() - 1);
    if namespace.is_empty() {
        let namespaces = type_namespaces(database, ty[0]);
        if namespaces.is_empty() {
            return Err(format!("unresolved filter path `{path}`").into());
        }
        for namespace in namespaces {
            filter.include_method(namespace, ty[0], name[0]);
        }
    } else if type_exists(database, &namespace.join("."), ty[0]) {
        filter.include_method(namespace.join("."), ty[0], name[0]);
    } else {
        return Err(format!("unresolved filter path `{path}`").into());
    }
    Ok(())
}

fn path_parts(path: &str) -> Vec<&str> {
    path.split([':', '.'])
        .filter(|part| !part.is_empty())
        .collect()
}

fn type_namespaces<'a>(database: &'a Database, ty: &str) -> Vec<&'a str> {
    database
        .type_names()
        .filter_map(|(namespace, name, _)| (name == ty).then_some(namespace))
        .collect()
}

fn type_exists(database: &Database, namespace: &str, name: &str) -> bool {
    !database.type_definitions(namespace, name).is_empty()
}

fn namespace_exists(database: &Database, namespace: &str) -> bool {
    database
        .type_names()
        .any(|(candidate, _, _)| candidate == namespace)
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
