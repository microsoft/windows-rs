use std::collections::*;
use std::io::*;

/// Formats the token string
pub fn format(namespace: &str, tokens: &mut String) {
    let mut child = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn().expect("Failed to spawn `rustfmt`");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        *tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        // TODO: This doesn't print anything useful
        println!("rustfmt failed for `{namespace}` with status {}\nError:\n{}", output.status, String::from_utf8_lossy(&output.stderr));
    }
}

/// Returns the libraries and their function and stack sizes used by the gnu and msvc tools to build the umbrella libs.
pub fn libraries() -> BTreeMap<String, BTreeMap<String, CallingConvention>> {
    let files = metadata::reader::File::with_default(&[]).unwrap();
    let reader = &metadata::reader::Reader::new(&files);
    let mut libraries = BTreeMap::<String, BTreeMap<String, CallingConvention>>::new();
    let root = reader.tree("Windows", &Default::default());

    for tree in root.flatten() {
        if let Some(apis) = reader.get(metadata::reader::TypeName::new(tree.namespace, "Apis")).next() {
            for method in reader.type_def_methods(apis) {
                let impl_map = reader.method_def_impl_map(method).expect("ImplMap not found");
                let scope = reader.impl_map_scope(impl_map);
                let library = reader.module_ref_name(scope).to_lowercase();
                if library == "forceinline" {
                    continue;
                }
                let flags = reader.impl_map_flags(impl_map);
                if flags.contains(metadata::PInvokeAttributes::CONV_PLATFORM) {
                    let params = reader.method_def_size(method);
                    libraries.entry(library).or_default().insert(reader.method_def_name(method).to_string(), CallingConvention::Stdcall(params));
                } else if flags.contains(metadata::PInvokeAttributes::CONV_CDECL) {
                    libraries.entry(library).or_default().insert(reader.method_def_name(method).to_string(), CallingConvention::Cdecl);
                } else {
                    unimplemented!();
                }
            }
        }
    }

    libraries
}

pub enum CallingConvention {
    Stdcall(usize),
    Cdecl,
}
