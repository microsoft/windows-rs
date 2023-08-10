use metadata::RowReader;
use std::collections::BTreeMap;

pub enum CallingConvention {
    Stdcall(usize),
    Cdecl,
}

pub fn default_metadata() -> Vec<metadata::File> {
    vec![
        metadata::File::new(
            std::include_bytes!("../../../libs/metadata/default/Windows.winmd").to_vec(),
        )
        .expect("invalid winmd"),
        metadata::File::new(
            std::include_bytes!("../../../libs/metadata/default/Windows.Win32.winmd").to_vec(),
        )
        .expect("invalid winmd"),
        metadata::File::new(
            std::include_bytes!("../../../libs/metadata/default/Windows.Wdk.winmd").to_vec(),
        )
        .expect("invalid winmd"),
    ]
}

/// Returns the libraries and their function and stack sizes used by the gnu and msvc tools to build the umbrella libs.
pub fn libraries() -> BTreeMap<String, BTreeMap<String, CallingConvention>> {
    let mut libraries = BTreeMap::<String, BTreeMap<String, CallingConvention>>::new();

    let files = default_metadata();
    let reader = &metadata::Reader::new(&files);
    combine_libraries(reader, &mut libraries);

    // StgConvertPropertyToVariant was removed https://github.com/microsoft/win32metadata/issues/1566
    // It is very unlikely that anybody is calling that function, but this just ensures that the libs
    // are stable and we don't break the `windows-targets` crate compatibility until the next major
    // release of that crate.

    let compat = [
        metadata::File::new(std::include_bytes!("../Windows.Win32.49.winmd").to_vec())
            .expect("invalid winmd"),
    ];

    let reader = &metadata::Reader::new(&compat);
    combine_libraries(reader, &mut libraries);

    libraries
}

fn combine_libraries(
    reader: &metadata::Reader,
    libraries: &mut BTreeMap<String, BTreeMap<String, CallingConvention>>,
) {
    for item in reader.items(&Default::default()) {
        let metadata::Item::Fn(method, _) = item else {
            continue;
        };

        let library = reader.method_def_module_name(method);
        let impl_map = reader
            .method_def_impl_map(method)
            .expect("ImplMap not found");
        let flags = reader.impl_map_flags(impl_map);
        let name = reader.impl_map_import_name(impl_map).to_string();

        // TODO: don't include these in metadata to begin with
        if name.starts_with('#') || library == "forceinline" {
            continue;
        }

        if flags.contains(metadata::PInvokeAttributes::CallConvPlatformapi) {
            let params = reader.method_def_size(method);
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
