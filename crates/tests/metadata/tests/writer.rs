#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test_metadata.winmd");
    {
        use metadata::writer::*;

        let mut tables = Tables::new("test.winmd");

        let mut stringable = TypeDef::new(TypeName::new("TestWindows.Foundation", "IStringable"));
        stringable.flags.set_public();
        stringable.flags.set_abstract();
        stringable.flags.set_winrt();
        stringable.flags.set_interface();
        stringable.method_list.push(MethodDef::new("ToString"));
        tables.type_def.push(stringable);

        let mut rect = TypeDef::new(TypeName::new("TestWindows.Foundation", "Rect"));
        rect.flags.set_public();
        rect.flags.set_winrt();
        rect.extends = Some(TypeRef::system_value_type());
        rect.field_list.push(Field::new("Height"));
        tables.type_def.push(rect);

        pe::write(temp_file.to_str().unwrap(), tables);
    }
    {
        use metadata::reader::*;

        let files = vec![File::new(temp_file.to_str().unwrap()).unwrap()];
        let reader = &Reader::new(&files);
        let def = reader.get(TypeName::new("TestWindows.Foundation", "IStringable")).next().unwrap();
        assert_eq!(reader.type_def_kind(def), TypeKind::Interface);
        assert!(reader.type_def_flags(def).winrt());

        let method = reader.type_def_methods(def).next().unwrap();
        assert_eq!(reader.method_def_name(method), "ToString");

        let def = reader.get(TypeName::new("TestWindows.Foundation", "Rect")).next().unwrap();
        assert_eq!(reader.type_def_kind(def), TypeKind::Struct);
        assert!(reader.type_def_flags(def).winrt());

        let field = reader.type_def_fields(def).next().unwrap();
        assert_eq!(reader.field_name(field), "Height");
    }
}
