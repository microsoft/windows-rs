use windows_rdl::*;

fn run_roundtrip(file: &str) {
    let path = std::path::Path::new("src").join(format!("{file}.rdl"));
    let winmd = path.with_extension("winmd");
    let reference = needs_reference(&path);

    let mut r = reader();
    r.input(path.to_str().unwrap());
    if reference {
        r.input("../../../libs/bindgen/default");
    }
    r.output(winmd.to_str().unwrap()).write().unwrap();

    let mut w = writer();
    w.input(winmd.to_str().unwrap());
    if reference {
        w.input("../../../libs/bindgen/default");
    }
    w.output(path.to_str().unwrap())
        .filter("Test")
        .write()
        .unwrap();
}

fn needs_reference(path: &std::path::Path) -> bool {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
        .contains("Windows::")
}

// generated tests

#[test]
fn roundtrip_arches() {
    run_roundtrip("arches");
}
#[test]
fn roundtrip_array() {
    run_roundtrip("array");
}
#[test]
fn roundtrip_attribute_everywhere() {
    run_roundtrip("attribute-everywhere");
}
#[test]
fn roundtrip_attribute_field() {
    run_roundtrip("attribute-field");
}
#[test]
fn roundtrip_attribute_flags_enum() {
    run_roundtrip("attribute-flags-enum");
}
#[test]
fn roundtrip_attribute_on_class_multi() {
    run_roundtrip("attribute-on-class-multi");
}
#[test]
fn roundtrip_attribute_on_class() {
    run_roundtrip("attribute-on-class");
}
#[test]
fn roundtrip_attribute_on_interface() {
    run_roundtrip("attribute-on-interface");
}
#[test]
fn roundtrip_attribute_on_struct() {
    run_roundtrip("attribute-on-struct");
}
#[test]
fn roundtrip_attribute_zero_args() {
    run_roundtrip("attribute-zero-args");
}
#[test]
fn roundtrip_attribute() {
    run_roundtrip("attribute");
}
#[test]
fn roundtrip_class() {
    run_roundtrip("class");
}
#[test]
fn roundtrip_complex_attribute_refs() {
    run_roundtrip("complex-attribute-refs");
}
#[test]
fn roundtrip_const_ptr() {
    run_roundtrip("const-ptr");
}
#[test]
fn roundtrip_const_usize() {
    run_roundtrip("const-usize");
}
#[test]
fn roundtrip_const() {
    run_roundtrip("const");
}
#[test]
fn roundtrip_const_fields() {
    run_roundtrip("const_fields");
}
#[test]
fn roundtrip_cross_namespace_enum_attribute() {
    run_roundtrip("cross-namespace-enum-attribute");
}
#[test]
fn roundtrip_delegate() {
    run_roundtrip("delegate");
}
#[test]
fn roundtrip_empty_struct_union() {
    run_roundtrip("empty_struct_union");
}
#[test]
fn roundtrip_enum_attribute_on_class() {
    run_roundtrip("enum-attribute-on-class");
}
#[test]
fn roundtrip_enum_flags() {
    run_roundtrip("enum-flags");
}
#[test]
fn roundtrip_enum() {
    run_roundtrip("enum");
}
#[test]
fn roundtrip_enum_values() {
    run_roundtrip("enum_values");
}
#[test]
fn roundtrip_event_interface() {
    run_roundtrip("event-interface");
}
#[test]
fn roundtrip_exclusive_to() {
    run_roundtrip("exclusive-to");
}
#[test]
fn roundtrip_fn() {
    run_roundtrip("fn");
}
#[test]
fn roundtrip_fn_last_error() {
    run_roundtrip("fn_last_error");
}
#[test]
fn roundtrip_generic() {
    run_roundtrip("generic");
}
#[test]
fn roundtrip_guid() {
    run_roundtrip("guid");
}
#[test]
fn roundtrip_in() {
    run_roundtrip("in");
}
#[test]
fn roundtrip_interface_inherits() {
    run_roundtrip("interface-inherits");
}
#[test]
fn roundtrip_interface_requires() {
    run_roundtrip("interface-requires");
}
#[test]
fn roundtrip_kind_explicit() {
    run_roundtrip("kind-explicit");
}
#[test]
fn roundtrip_kind_nested() {
    run_roundtrip("kind-nested");
}
#[test]
fn roundtrip_kind_override() {
    run_roundtrip("kind-override");
}
#[test]
fn roundtrip_kind() {
    run_roundtrip("kind");
}
#[test]
fn roundtrip_metadata_enum_attribute() {
    run_roundtrip("metadata-enum-attribute");
}
#[test]
fn roundtrip_nested() {
    run_roundtrip("nested");
}
#[test]
fn roundtrip_opt() {
    run_roundtrip("opt");
}
#[test]
fn roundtrip_overloads() {
    run_roundtrip("overloads");
}
#[test]
fn roundtrip_packed() {
    run_roundtrip("packed");
}
#[test]
fn roundtrip_param_attributes() {
    run_roundtrip("param-attributes");
}
#[test]
fn roundtrip_path() {
    run_roundtrip("path");
}
#[test]
fn roundtrip_property_get_set() {
    run_roundtrip("property-get-set");
}
#[test]
fn roundtrip_pseudo_types() {
    run_roundtrip("pseudo-types");
}
#[test]
fn roundtrip_return_type_attrs() {
    run_roundtrip("return-type-attrs");
}
#[test]
fn roundtrip_same_name() {
    run_roundtrip("same-name");
}
#[test]
fn roundtrip_signature() {
    run_roundtrip("signature");
}
#[test]
fn roundtrip_special_interface() {
    run_roundtrip("special-interface");
}
#[test]
fn roundtrip_special() {
    run_roundtrip("special");
}
#[test]
fn roundtrip_string() {
    run_roundtrip("string");
}
#[test]
fn roundtrip_struct_generic_field() {
    run_roundtrip("struct-generic-field");
}
#[test]
fn roundtrip_struct() {
    run_roundtrip("struct");
}
#[test]
fn roundtrip_typedef() {
    run_roundtrip("typedef");
}
#[test]
fn roundtrip_union() {
    run_roundtrip("union");
}
#[test]
fn roundtrip_vararg() {
    run_roundtrip("vararg");
}
#[test]
fn roundtrip_win32_property_event() {
    run_roundtrip("win32-property-event");
}
#[test]
fn roundtrip_winrt_event_unordered() {
    run_roundtrip("winrt-event-unordered");
}
#[test]
fn roundtrip_winrt_property_interleaved() {
    run_roundtrip("winrt-property-interleaved");
}
#[test]
fn roundtrip_winrt_property_reversed() {
    run_roundtrip("winrt-property-reversed");
}
#[test]
fn roundtrip_winrt_property_type_mismatch() {
    run_roundtrip("winrt-property-type-mismatch");
}
#[test]
fn roundtrip_winrt_property() {
    run_roundtrip("winrt-property");
}
