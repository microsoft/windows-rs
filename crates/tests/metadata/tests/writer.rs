#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test_metadata.winmd");
    {
        use metadata::writer::*;

        let mut tables = Tables::new("test.winmd");

        let mut stringable = TypeDef::new(TypeName::new("Windows.Foundation", "IStringable"));
        stringable.flags.set_public();
        stringable.flags.set_abstract();
        stringable.flags.set_winrt();
        stringable.flags.set_interface();
        stringable.method_list.push(MethodDef::new("ToString"));
        tables.type_def.push(stringable);

        let mut closable = TypeDef::new(TypeName::new("Windows.Foundation", "IClosable"));
        closable.flags.set_public();
        closable.flags.set_abstract();
        closable.flags.set_winrt();
        closable.flags.set_interface();
        closable.method_list.push(MethodDef::new("Close"));
        tables.type_def.push(closable);

        pe::write(temp_file.to_str().unwrap(), tables);
    }
    {
        use metadata::reader::*;

        let files = vec![File::new(temp_file.to_str().unwrap()).unwrap()];
        let reader = &Reader::new(&files);
        let def = reader.get(TypeName::new("Windows.Foundation", "IStringable")).next().unwrap();
        assert_eq!(reader.type_def_kind(def), TypeKind::Interface);
        assert!(reader.type_def_flags(def).winrt());

        let method = reader.type_def_methods(def).next().unwrap();
        assert_eq!(reader.method_def_name(method), "ToString");

        let def = reader.get(TypeName::new("Windows.Foundation", "IClosable")).next().unwrap();
        assert_eq!(reader.type_def_kind(def), TypeKind::Interface);
        assert!(reader.type_def_flags(def).winrt());

        let method = reader.type_def_methods(def).next().unwrap();
        assert_eq!(reader.method_def_name(method), "Close");
    }
}
