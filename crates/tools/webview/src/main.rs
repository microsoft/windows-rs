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

    bindgen([
        "--in",
        "default",
        "crates/tools/webview/WebView2.winmd",
        "--out",
        "crates/libs/webview/src/bindings.rs",
        "--filter",
        "WebView2",
        "--no-deps",
    ])
    .unwrap();
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
}

fn write_interface(item: &idl::Interface, output: &mut writer::File) {
    output.TypeDef(
        "WebView2",
        &item.name,
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );

    for method in &item.methods {
        let flags = MethodAttributes::Public
            | MethodAttributes::HideBySig
            | MethodAttributes::Abstract
            | MethodAttributes::NewSlot
            | MethodAttributes::Virtual;

        let signature = Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: to_type(&method.return_type),
            types: method
                .params
                .iter()
                .map(|param| to_type(&param.ty))
                .collect(),
        };

        output.MethodDef(&method.name, &signature, flags, Default::default());

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
        "HRESULT" => Type::named("Windows.Foundation", "HResult"),
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
            let trim = name.trim_end_matches('*').trim_end_matches("const").trim();

            if trim.len() != name.len() {
                to_type(trim)
            } else {
                Type::named("WebView2", name)
            }
        }
    }
}
