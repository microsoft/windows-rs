use regex::Regex;
use std::collections::BTreeMap;
use std::path::Path;

pub enum CallingConvention {
    Stdcall(usize),
    Cdecl,
}

pub fn default_metadata() -> Vec<metadata::File> {
    vec![
        metadata::File::new(
            std::include_bytes!("../../../libs/bindgen/default/Windows.winmd").to_vec(),
        )
        .expect("invalid winmd"),
        metadata::File::new(
            std::include_bytes!("../../../libs/bindgen/default/Windows.Win32.winmd").to_vec(),
        )
        .expect("invalid winmd"),
        metadata::File::new(
            std::include_bytes!("../../../libs/bindgen/default/Windows.Wdk.winmd").to_vec(),
        )
        .expect("invalid winmd"),
    ]
}

/// Returns the libraries and their function and stack sizes used by the gnu and msvc tools to build the umbrella libs.
pub fn libraries() -> BTreeMap<String, BTreeMap<String, CallingConvention>> {
    let mut libraries = BTreeMap::<String, BTreeMap<String, CallingConvention>>::new();

    let files = default_metadata();
    let reader = metadata::Reader::new(files);
    combine_libraries(reader, &mut libraries);
    libraries
}

fn combine_libraries(
    reader: &metadata::Reader,
    libraries: &mut BTreeMap<String, BTreeMap<String, CallingConvention>>,
) {
    for item in reader.items() {
        let metadata::Item::Fn(method, _) = item else {
            continue;
        };

        // Windows libs are always produced with lower case module names.
        let library = method.module_name().to_lowercase();
        let impl_map = method.impl_map().expect("ImplMap not found");
        let flags = impl_map.flags();
        let name = impl_map.import_name().to_string();

        // TODO: don't include these in metadata to begin with
        if name.starts_with('#') || library == "forceinline" {
            continue;
        }

        if flags.contains(metadata::PInvokeAttributes::CallConvPlatformapi) {
            let params = method.signature(&[]).size();
            libraries
                .entry(library)
                .or_default()
                .insert(name, CallingConvention::Stdcall(params));
        } else if flags.contains(metadata::PInvokeAttributes::CallConvCdecl) {
            libraries
                .entry(library)
                .or_default()
                .insert(name, CallingConvention::Cdecl);
        } else {
            unimplemented!();
        }
    }
}

pub fn crates<P: AsRef<Path>>(path: P) -> Vec<(String, String)> {
    let regex = Regex::new(r#"name = "([^"]+)"\sversion = "([^"]+)""#).expect("regex");
    let mut names = find(path, &regex);
    names.sort();
    names
}

fn find<P: AsRef<Path>>(path: P, regex: &Regex) -> Vec<(String, String)> {
    let mut names = vec![];

    if let Ok(files) = std::fs::read_dir(path) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_dir() {
                    names.append(&mut find(file.path(), regex));
                } else if file.file_name() == "Cargo.toml" {
                    let text = std::fs::read_to_string(file.path()).expect("Cargo.toml");
                    let captures = regex.captures(&text).expect("captures");
                    let name = captures.get(1).expect("name");
                    let version = captures.get(2).expect("version");
                    names.push((name.as_str().to_string(), version.as_str().to_string()));
                }
            }
        }
    }

    names
}
