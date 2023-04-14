use std::collections::*;

/// Returns the libraries and their function and stack sizes used by the gnu and msvc tools to build the umbrella libs.
pub fn libraries() -> BTreeMap<String, BTreeMap<String, CallingConvention>> {
    use metadata::reader::File;
    let mut libraries = BTreeMap::<String, BTreeMap<String, CallingConvention>>::new();

    let files = File::with_default(&[]).unwrap();
    combine(&files, &mut libraries);

    libraries
}

pub enum CallingConvention {
    Stdcall(usize),
    Cdecl,
}

fn combine(files: &[metadata::reader::File], libraries: &mut BTreeMap<String, BTreeMap<String, CallingConvention>>) {
    let reader = &metadata::reader::Reader::new(files);
    for method in reader.namespaces().flat_map(|namespace| reader.namespace_functions(namespace)) {
        let library = reader.method_def_module_name(method);
        let impl_map = reader.method_def_impl_map(method).expect("ImplMap not found");
        let flags = reader.impl_map_flags(impl_map);
        let name = reader.impl_map_import_name(impl_map).to_string();
        if flags.contains(metadata::PInvokeAttributes::CONV_PLATFORM) {
            let params = reader.method_def_size(method);
            libraries.entry(library).or_default().insert(name, CallingConvention::Stdcall(params));
        } else if flags.contains(metadata::PInvokeAttributes::CONV_CDECL) {
            libraries.entry(library).or_default().insert(name, CallingConvention::Cdecl);
        } else {
            unimplemented!();
        }
    }
}
