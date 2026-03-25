fn should_panic(rdl: &str) {
    windows_rdl::reader()
        .input_str(rdl)
        .output(".winmd")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: class interface does not support attributes\n --> .rdl:6:9")]
fn attribute_on_class_interface_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    interface IFoo {}
    class MyClass {
        #[default] IFoo,
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: attribute type not found\n --> .rdl:4:5")]
fn unknown_attribute_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    #[Unknown(42)]
    class MyClass {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: value not valid\n --> .rdl:6:11")]
fn wrong_arg_type_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(value: u32); }

    #[Foo("not a u32")]
    class MyClass {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: expected `Color` variant name\n --> .rdl:8:15")]
fn enum_arg_requires_variant_name() {
    should_panic(
        r#"
#[winrt]
mod Test {
    #[repr(i32)]
    enum Color { Red = 0, Green = 1, Blue = 2, }
    attribute PaletteAttribute { fn(value: Color); }

    #[Palette(1)]
    class MyClass {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: enum variant not found\n --> .rdl:8:15")]
fn enum_arg_unknown_variant_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    #[repr(i32)]
    enum Color { Red = 0, Green = 1, Blue = 2, }
    attribute PaletteAttribute { fn(value: Color); }

    #[Palette(Purple)]
    class MyClass {}
}
        "#,
    )
}

#[test]
#[should_panic(
    expected = "error: positional attribute arguments must come before named arguments\n --> .rdl:6:27"
)]
fn positional_after_named_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(value: u32); }

    #[Foo(named_prop = 1, 42)]
    class MyClass {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: no matching attribute constructor found\n --> .rdl:6:5")]
fn no_matching_ctor_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(value: u32); }

    #[Foo(1, 2, 3)]
    class MyClass {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: attribute has no property `unknown`\n --> .rdl:6:5")]
fn unknown_property_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(); version: u32, }

    #[Foo(unknown = 42)]
    class MyClass {}
}
        "#,
    )
}

#[test]
#[should_panic(
    expected = "error: attribute cannot use top-level `name = value` syntax\n --> .rdl:6:5"
)]
fn top_level_name_value_syntax_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(); }

    #[Foo = "bar"]
    class MyClass {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: unexpected `self` parameter\n --> .rdl:4:23")]
fn callback_unexpected_self() {
    should_panic(
        r#"
#[win32]
mod Test {
    extern fn Handler(&self);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: param names must be unique\n --> .rdl:4:31")]
fn callback_param_name_unique() {
    should_panic(
        r#"
#[win32]
mod Test {
    extern fn Handler(a: i32, a: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `output` attribute does not accept arguments\n --> .rdl:4:23")]
fn callback_out_with_args() {
    should_panic(
        r#"
#[win32]
mod Test {
    extern fn Handler(#[output(42)] output: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `optional` attribute does not accept arguments\n --> .rdl:4:23")]
fn callback_opt_with_args() {
    should_panic(
        r#"
#[win32]
mod Test {
    extern fn Handler(#[optional(42)] value: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: callback abi not supported\n --> .rdl:4:12")]
fn abi_not_supported() {
    should_panic(
        r#"
#[win32]
mod Test {
    extern "D" fn Handler();
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: GUID constant requires a value\n --> .rdl:4:11")]
fn guid_const_missing_value() {
    should_panic(
        r#"
#[win32]
mod Test {
    const MY_GUID: GUID;
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: unexpected `self` parameter\n --> .rdl:4:25")]
fn delegate_unexpected_self() {
    should_panic(
        r#"
#[winrt]
mod Test {
    delegate fn Handler(&self);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: param names must be unique\n --> .rdl:4:33")]
fn delegate_param_name_unique() {
    should_panic(
        r#"
#[winrt]
mod Test {
    delegate fn Handler(a: i32, a: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `output` attribute does not accept arguments\n --> .rdl:4:25")]
fn delegate_out_with_args() {
    should_panic(
        r#"
#[winrt]
mod Test {
    delegate fn Handler(#[output(42)] output: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `optional` attribute does not accept arguments\n --> .rdl:4:25")]
fn delegate_opt_with_args() {
    should_panic(
        r#"
#[winrt]
mod Test {
    delegate fn Handler(#[optional(42)] value: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: only type generic parameters are supported\n --> .rdl:4:")]
fn non_type_generic_not_supported() {
    should_panic(
        r#"
#[winrt]
mod Test {
    delegate fn Handler<'a>(a: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `repr` must be an integer type\n --> .rdl:4:5")]
fn repr_must_be_integer() {
    should_panic(
        r#"
#[winrt]
mod Test {
    #[repr(bool)]
    enum AsyncStatus {
        Started = 0,
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `flags` attribute does not accept arguments\n --> .rdl:5:5")]
fn flags_with_args_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    #[repr(u32)]
    #[flags(something)]
    enum VirtualKeyModifiers {
        None = 0,
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: unexpected `self` parameter\n --> .rdl:5:17")]
fn fn_unexpected_self() {
    should_panic(
        r#"
#[winrt]
mod Test {
    #[library("lib")]
    extern fn F(&self);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: param names must be unique\n --> .rdl:5:25")]
fn fn_param_name_unique() {
    should_panic(
        r#"
#[winrt]
mod Test {
    #[library("lib")]
    extern fn F(a: i32, a: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `output` attribute does not accept arguments\n --> .rdl:5:17")]
fn fn_out_with_args() {
    should_panic(
        r#"
#[win32]
mod Test {
    #[library("lib")]
    extern fn F(#[output(42)] output: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `optional` attribute does not accept arguments\n --> .rdl:5:17")]
fn fn_opt_with_args() {
    should_panic(
        r#"
#[win32]
mod Test {
    #[library("lib")]
    extern fn F(#[optional(42)] value: i32);
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `library` name missing\n --> .rdl:4:5")]
fn link_missing_name() {
    should_panic(
        r#"
#[win32]
mod Test {
    #[library]
    extern fn F();
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: function abi not supported\n --> .rdl:5:12")]
fn link_abi_not_supported() {
    should_panic(
        r#"
#[win32]
mod Test {
    #[library("a.dll")]
    extern "invalid" fn F();
}
        "#,
    )
}

#[test]
#[should_panic(
    expected = "error: non-WinRT interface can only inherit from one interface\n --> .rdl:4:28"
)]
fn win32_multiple_required_interfaces() {
    should_panic(
        r#"
#[win32]
mod Test {
    interface IFoo: IBar + IBaz {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `&self` parameter not found\n --> .rdl:5:19")]
fn missing_self_typed_first_param() {
    should_panic(
        r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method(a: i32);
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `&self` parameter not found\n --> .rdl:5:19")]
fn missing_self_wrong_receiver() {
    should_panic(
        r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method(&mut self);
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `&self` parameter not found\n --> .rdl:5:12")]
fn missing_self_no_params() {
    should_panic(
        r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method();
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `output` attribute does not accept arguments\n --> .rdl:5:26")]
fn method_out_with_args() {
    should_panic(
        r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method(&self, #[output(42)] output: i32);
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `optional` attribute does not accept arguments\n --> .rdl:5:26")]
fn method_opt_with_args() {
    should_panic(
        r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method(&self, #[optional(42)] value: i32);
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `special` attribute does not accept arguments\n --> .rdl:5:9")]
fn special_with_args() {
    should_panic(
        r#"
#[win32]
mod Test {
    interface IFoo {
        #[special(42)]
        fn Method(&self);
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: only type generic parameters are supported\n --> .rdl:4:20")]
fn interface_lifetime_generic_errors() {
    should_panic(
        r#"
#[winrt]
mod Test {
    interface IFoo<'a> {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: use namespace not found\n --> .rdl:2:1")]
fn use_glob_invalid_path() {
    should_panic(
        r#"
use NonExistent::*;

#[winrt]
mod Test {
    struct Thing {
        a: NoSuchType,
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: type not found\n --> .rdl:7:12")]
fn use_glob_unresolved_type() {
    should_panic(
        r#"
use Other::*;

#[winrt]
mod Test {
    struct Thing {
        a: NoSuchType,
    }
}

#[winrt]
mod Other {
    struct ExistingThing {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: type not supported\n --> .rdl:5:12")]
fn unsupported_type_errors() {
    should_panic(
        r#"
#[win32]
mod Test {
    struct Foo {
        a: (i32, i32),
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `repr` attribute not found\n --> .rdl:4:5")]
pub fn enum_repr_not_found() {
    should_panic(
        r#"
#[winrt]
mod Test {
    enum AsyncStatus {
        Canceled = 2,
        Completed = 1,
        Error = 3,
        Started = 0,
    }
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: value not valid\n --> .rdl:4:20")]
pub fn const_value_not_valid() {
    should_panic(
        r#"
#[win32]
mod Test {
    const U8: u8 = -1;
}
        "#,
    )
}

#[test]
#[should_panic(
    expected = "error: `winrt` and `win32` attributes are mutually exclusive\n --> .rdl:5:5"
)]
pub fn winrt_win32_exclusive() {
    should_panic(
        r#"
mod Test {
    #[winrt]
    #[win32]
    struct A {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: `winrt` or `win32` attribute required\n --> .rdl:3:5")]
pub fn winrt_win32_required() {
    should_panic(
        r#"
mod Test {
    struct A {}
}
        "#,
    )
}

#[test]
#[should_panic(expected = "error: variadic parameters are not supported for callbacks")]
fn vararg_callback_not_supported() {
    should_panic(
        r#"
#[win32]
mod Test {
    extern "C" fn Handler(a: i32, ...);
}
        "#,
    )
}
