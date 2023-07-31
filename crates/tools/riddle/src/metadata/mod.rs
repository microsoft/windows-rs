pub use windows_metadata::*;

pub fn type_def_invoke_method(reader: &Reader, row: TypeDef) -> MethodDef {
    reader
        .type_def_methods(row)
        .find(|method| reader.method_def_name(*method) == "Invoke")
        .expect("`Invoke` method not found")
}
