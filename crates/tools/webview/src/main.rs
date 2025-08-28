use windows_bindgen::*;
use windows_idl as idl;
use windows_metadata::*;

fn main() {
    metadata_to_bindings();
}

fn metadata_to_bindings() {
    let input = std::include_str!("../WebView2.idl");
    let input = idl::parse(input).unwrap();

    let mut output = writer::File::new("WebView2");

    for item in &input.items {
        write_item(item, &mut output);
    }

    let output = output.into_stream();
    std::fs::write("crates/tools/webview/WebView2.winmd", output).unwrap();

    // TODO: some methods will be skipped
    _ = bindgen([
        "--in",
        "default",
        "crates/tools/webview/WebView2.winmd",
        "--out",
        "crates/libs/webview/src/bindings.rs",
        "--flat",
        "--filter",
        "WebView2",
        "IStream",
        "VARIANT",
        "POINT",
        "HCURSOR",
        "HICON",
        "IDataObject",
        "POINT",
        "RECT",
        "HANDLE",
        "HWND",
        "--no-deps",
    ]);
}

fn write_item(item: &idl::Item, output: &mut writer::File) {
    match item {
        idl::Item::Enum(item) => write_enum(item, output),
        idl::Item::Interface(item) => write_interface(item, output),
        idl::Item::Struct(item) => write_struct(item, output),
        idl::Item::Library(item) => write_library(item, output),
        // idl::Item::CppQuote(item) => write_cpp_quote(item, output),
        _ => {}
    }
}

fn write_enum(item: &idl::Enum, output: &mut writer::File) {
    let value_type = output.TypeRef("System", "Enum");

    output.TypeDef(
        "WebView2",
        &item.name,
        writer::TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public | TypeAttributes::Sealed,
    );

    output.Field(
        "value__",
        &Type::U32,
        FieldAttributes::Private | FieldAttributes::SpecialName | FieldAttributes::RTSpecialName,
    );

    let mut next = 0;

    for variant in &item.variants {
        let field = output.Field(
            &variant.name,
            &Type::named("WebView2", &item.name),
            FieldAttributes::Public | FieldAttributes::Static | FieldAttributes::Literal,
        );

        let value = if let Some(value) = variant.value {
            value
        } else {
            next
        };

        next = value + 1;

        output.Constant(
            writer::HasConstant::Field(field),
            &Value::U32(value.try_into().unwrap()),
        );
    }
}

fn write_struct(item: &idl::Struct, output: &mut writer::File) {
    let value_type = output.TypeRef("System", "ValueType");

    output.TypeDef(
        "WebView2",
        &item.name,
        writer::TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public | TypeAttributes::SequentialLayout | TypeAttributes::Sealed,
    );

    for field in &item.fields {
        output.Field(
            &field.name,
            &to_type(&field.field_type),
            FieldAttributes::Public,
        );
    }
}

fn write_interface(item: &idl::Interface, output: &mut writer::File) {
    // TODO: detect delegate-like interfaces and project them as such
    let like_delegate = if item.name.ends_with("CompletedHandler") {
        assert_eq!(item.implements.len(), 1);
        assert_eq!(item.implements[0].name, "IUnknown");
        assert_eq!(item.methods.len(), 1);
        assert_eq!(item.methods[0].name, "Invoke");
        // TODO: validate return type is HRESULT
        true
    } else {
        false
    };

    let type_def = if like_delegate {
        let delegate_type = output.TypeRef("System", "MulticastDelegate");

        output.TypeDef(
            "WebView2",
            &item.name,
            writer::TypeDefOrRef::TypeRef(delegate_type),
            TypeAttributes::Public | TypeAttributes::WindowsRuntime,
        )
    } else {
        output.TypeDef(
            "WebView2",
            &item.name,
            writer::TypeDefOrRef::default(),
            TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
        )
    };

    output.InterfaceImpl(type_def, &to_type(&item.implements[0].name));

    let attribute_ref = writer::MemberRefParent::TypeRef(
        output.TypeRef("System.Runtime.InteropServices", "GuidAttribute"),
    );

    let uuid = item.attributes.iter().find(|attribute|attribute.name == "uuid").unwrap();
    assert_eq!(uuid.parameters.len(), 1);
    let guid =  &mut uuid.parameters[0].1.bytes();

    let signature = Signature {
        types: vec![
            Type::U32,
            Type::U16,
            Type::U16,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
        ],
        ..Default::default()
    };

    let ctor = output.MemberRef(".ctor", &signature, attribute_ref);

    // TODO: read the actual guid
    let value = vec![
        (String::new(), Value::U32(guid_u32(guid, true))),
        (String::new(), Value::U16(guid_u16(guid, true))),
        (String::new(), Value::U16(guid_u16(guid, true))),
        (String::new(), Value::U8(guid_u8(guid, false))),
        (String::new(), Value::U8(guid_u8(guid, true))),
        (String::new(), Value::U8(guid_u8(guid, false))),
        (String::new(), Value::U8(guid_u8(guid, false))),
        (String::new(), Value::U8(guid_u8(guid, false))),
        (String::new(), Value::U8(guid_u8(guid, false))),
        (String::new(), Value::U8(guid_u8(guid, false))),
        (String::new(), Value::U8(guid_u8(guid, false))),
    ];

    output.Attribute(
        writer::HasAttribute::TypeDef(type_def),
        writer::AttributeType::MemberRef(ctor),
        &value,
    );

    for method in &item.methods {
        let mut flags = MethodAttributes::Public
            | MethodAttributes::HideBySig
            | MethodAttributes::Abstract
            | MethodAttributes::NewSlot
            | MethodAttributes::Virtual;

        let specialname = if method.attributes.iter().any(|attribute|attribute.name == "propget") {
            flags |= MethodAttributes::SpecialName;
            "propget"
        } else if method.attributes.iter().any(|attribute|attribute.name == "propput") {
            flags |= MethodAttributes::SpecialName;
            "propput"
        } else {
            ""
        };

        let return_type = if like_delegate {
            assert_eq!(method.return_type, "HRESULT");
            Type::Void
        } else {
            to_type(&method.return_type)
        };

        let signature = Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type,
            types: method
                .params
                .iter()
                .map(|param| to_type(&param.ty))
                .collect(),
        };

        let name = match specialname {
            "propget" => format!("get_{}", &method.name),
            "propput" => format!("put_{}", &method.name),
            _ => method.name.clone(),
        };

        output.MethodDef(&name, &signature, flags, Default::default());

        for (sequence, param) in method.params.iter().enumerate() {
            output.Param(
                &param.name,
                (sequence + 1).try_into().unwrap(),
                ParamAttributes::In,
            );
        }
    }
}

fn write_library(item: &idl::Library, output: &mut writer::File) {
    for item in &item.items {
        write_item(item, output);
    }
}

// Just a hack to get this to compile - need to provide actual type resolution
fn to_type(name: &str) -> Type {
    match name {
        "DWORD" | "UINT" | "UINT32" => Type::U32,
        "int" | "INT32" | "INT" => Type::I32,
        "BYTE" => Type::U8,
        "HRESULT" => Type::named("Windows.Win32.Foundation", "HRESULT"),
        "LPWSTR" => Type::named("Windows.Win32.Foundation", "PWSTR"),
        "HANDLE" => Type::named("Windows.Win32.Foundation", "HANDLE"),
        "LPCWSTR" => Type::named("Windows.Win32.Foundation", "PWSTR"),
        "BOOL" => Type::named("Windows.Win32.Foundation", "BOOL"),
        "IStream" => Type::named("Windows.Win32.System.Com", "IStream"),
        "IDataObject" => Type::named("Windows.Win32.System.Com", "IDataObject"),
        "HWND" => Type::named("Windows.Win32.Foundation", "HWND"),
        "IUnknown" => Type::named("Windows.Win32.System.Com", "IUnknown"),
        "RECT" => Type::named("Windows.Win32.Foundation", "RECT"),
        "POINT" => Type::named("Windows.Win32.Foundation", "POINT"),
        "HCURSOR" => Type::named("Windows.Win32.UI.WindowsAndMessaging", "HCURSOR"),
        "VARIANT" => Type::named("Windows.Win32.System.Variant", "VARIANT"),
        "double" => Type::F64,
        "EventRegistrationToken" | "INT64" => Type::I64,
        "UINT64" => Type::U64,
        _ => {
            // TODO: need to count `*` and into PtrMut or PtrConst depending on `const`
            // and possibly other attributes?
            let trim = name.trim_end_matches('*').trim_end_matches("const").trim();

            if trim.len() != name.len() {
                to_type(trim)
            } else {
                Type::named("WebView2", name)
            }
        }
    }
}

// TODO: push common handling to windows-guid

fn guid_u32(bytes: &mut core::str::Bytes, delimiter: bool) -> u32 {
    guid_next(bytes, 8, delimiter)
}

fn guid_u16(bytes: &mut core::str::Bytes, delimiter: bool) -> u16 {
    guid_next(bytes, 4, delimiter) as u16
}

fn guid_u8(bytes: &mut core::str::Bytes, delimiter: bool) -> u8 {
    guid_next(bytes, 2, delimiter) as u8
}

fn guid_next(bytes: &mut core::str::Bytes, len: usize, delimiter: bool) -> u32 {
    let mut value: u32 = 0;

    for _ in 0..len {
        let digit = bytes.next().unwrap();

        match digit {
            b'0'..=b'9' => value = (value << 4) + (digit - b'0') as u32,
            b'A'..=b'F' => value = (value << 4) + (digit - b'A' + 10) as u32,
            b'a'..=b'f' => value = (value << 4) + (digit - b'a' + 10) as u32,
            _ => panic!(),
        }
    }

    if delimiter && bytes.next() != Some(b'-') {
        panic!()
    } else {
        value
    }
}
