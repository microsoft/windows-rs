use super::*;

pub enum CallingConvention {
    Stdcall(usize),
    Cdecl,
}


/// Returns the libraries and their function and stack sizes used by the gnu and msvc tools to build the umbrella libs.
pub fn libraries() -> BTreeMap<String, BTreeMap<String, CallingConvention>> {
    let mut libraries = BTreeMap::<String, BTreeMap<String, CallingConvention>>::new();

    let reader = Reader::new(expand_input(&["default"]));
    combine_libraries(reader, &mut libraries);
    libraries
}

fn combine_libraries(
    reader: &metadata::Reader,
    libraries: &mut BTreeMap<String, BTreeMap<String, CallingConvention>>,
) {
    for types in reader.values() {
        for ty in types.values() {

        let Type::CppFn(item) = ty else {
            continue;
        };

        // Windows libs are always produced with lower case module names.
        let library = item.method.module_name().to_lowercase();
        let impl_map = item.method.impl_map().expect("ImplMap not found");
        let flags = impl_map.flags();
        let name = impl_map.import_name().to_string();

        // TODO: don't include these in metadata to begin with
        if name.starts_with('#') || library == "forceinline" {
            continue;
        }

        if flags.contains(PInvokeAttributes::CallConvPlatformapi) {
            let params = item.method.signature(&[]).size();
            libraries
                .entry(library)
                .or_default()
                .insert(name, CallingConvention::Stdcall(params));
        } else if flags.contains(PInvokeAttributes::CallConvCdecl) {
            libraries
                .entry(library)
                .or_default()
                .insert(name, CallingConvention::Cdecl);
        } else {
            unimplemented!();
        }
    }}
}