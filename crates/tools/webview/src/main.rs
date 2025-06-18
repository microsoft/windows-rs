use windows_bindgen::*;
use windows_idl as idl;
mod fmt;
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

    // let value_type = output.TypeRef("System", "Enum");

    // output.TypeDef(
    //     "WebView2",
    //     &item.name,
    //     writer::TypeDefOrRef::TypeRef(value_type),
    //     TypeAttributes::Public
    //         | TypeAttributes::SequentialLayout
    //         | TypeAttributes::Sealed
    //         | TypeAttributes::WindowsRuntime,
    // );
}

fn write_struct(item: &idl::Struct, output: &mut writer::File) {
    let value_type = output.TypeRef("System", "ValueType");

    output.TypeDef(
        "WebView2",
        &item.name,
        writer::TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public
            | TypeAttributes::SequentialLayout
            | TypeAttributes::Sealed,
    );
}

fn write_interface(item: &idl::Interface, output: &mut writer::File) {
    output.TypeDef(
        "WebView2",
        &item.name,
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public
            | TypeAttributes::Interface
            | TypeAttributes::Abstract,
    );
}

fn write_library(item: &idl::Library, output: &mut writer::File) {
    for item in &item.items {
        write_item(item, output);
    }
}
