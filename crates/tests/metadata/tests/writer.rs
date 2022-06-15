#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test_metadata.winmd");
    {
        use metadata::writer::*;

        let mut tables = Tables::new("test.winmd");

        let mut def = TypeDef::new(TypeName::new("TestWindows.Foundation", "IStringable"));
        def.flags.set_public();
        def.flags.set_abstract();
        def.flags.set_winrt();
        def.flags.set_interface();

        let mut method = MethodDef::new("ToString");
        method.param_list.push(Param { name: "param123".to_string(), sequence: 123, ..Default::default() });
        def.method_list.push(method);

        tables.type_def.push(def);

        let mut def = TypeDef::new(TypeName::new("TestWindows.Foundation", "Rect"));
        def.flags.set_public();
        def.flags.set_winrt();
        def.extends = Some(TypeRef::system_value_type());
        def.field_list.push(Field::new("Height"));
        tables.type_def.push(def);

        let mut def = TypeDef::new(TypeName::new("TestWindows.Foundation", "AsyncStatus"));
        def.flags.set_public();
        def.flags.set_winrt();
        def.extends = Some(TypeRef::system_enum());
        tables.type_def.push(def);

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

        let param = reader.method_def_params(method).next().unwrap();
        assert_eq!(reader.param_name(param), "param123");
        assert_eq!(reader.param_sequence(param), 123);

        let def = reader.get(TypeName::new("TestWindows.Foundation", "Rect")).next().unwrap();
        assert_eq!(reader.type_def_kind(def), TypeKind::Struct);
        assert!(reader.type_def_flags(def).winrt());

        let field = reader.type_def_fields(def).next().unwrap();
        assert_eq!(reader.field_name(field), "Height");

        let def = reader.get(TypeName::new("TestWindows.Foundation", "AsyncStatus")).next().unwrap();
        assert_eq!(reader.type_def_kind(def), TypeKind::Enum);
        assert!(reader.type_def_flags(def).winrt());
    }
}
