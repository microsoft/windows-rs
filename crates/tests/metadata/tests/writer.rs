#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test_metadata.winmd");
    {
        use metadata::writer::*;

        let mut tables = Tables::new();
        tables.module.push(Module::new("test.winmd"));
        tables.type_def.push(TypeDef::module());

        let mut stringable = TypeDef::winrt_interface("IStringable", "Windows.Foundation");
        stringable.method_list.push(MethodDef::new("ToString"));
        tables.type_def.push(stringable);

        let mut closable = TypeDef::winrt_interface("IClosable", "Windows.Foundation");
        closable.method_list.push(MethodDef::new("Close"));
        tables.type_def.push(closable);

        pe::write(temp_file.to_str().unwrap(), tables);
    }
    {
        use metadata::reader::*;
        let files = vec![File::new(temp_file.to_str().unwrap()).unwrap()];
        let reader = &Reader::new(&files);
    }
}
