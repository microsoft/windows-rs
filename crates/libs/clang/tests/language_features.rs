use windows_clang::{Input, extract};

#[test]
fn preserves_cpp_types_and_declaration_attributes() {
    helpers::ensure_libclang();

    let source = r#"
#define _Analysis_noreturn_ __attribute__((annotate("_Analysis_noreturn_")))
#define DEFINE_ENUM_FLAG_OPERATORS(T)

struct TEXT {
    char16_t first;
    char32_t second;
};

struct GUID {
    unsigned long data1;
};

typedef const GUID& REFGUID;

enum class COLOR : int {
    Red = 1,
};

enum class ACCESS : unsigned int {
    Read = 1,
};
DEFINE_ENUM_FLAG_OPERATORS(ACCESS)

extern "C" __declspec(noreturn) void ExitNow();
extern "C" _Analysis_noreturn_ void FailFast();
extern "C" int UseReferences(REFGUID input, GUID& output, const int& count);
"#;

    let snapshot = extract(
        [Input::new("language_features.hpp", source)],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Test", "test.dll").unwrap();

    assert!(rdl.contains("first: u16"));
    assert!(rdl.contains("second: u32"));
    assert!(rdl.contains("type REFGUID = *const GUID"));
    assert!(rdl.contains("#[repr(i32)]\n    #[scoped]\n    enum COLOR"));
    assert!(rdl.contains("#[repr(u32)]\n    #[flags]\n    #[scoped]\n    enum ACCESS"));
    assert!(rdl.contains("#[noreturn]\n    #[library(\"test.dll\")]\n    extern \"C\" fn ExitNow"));
    assert!(
        rdl.contains("#[noreturn]\n    #[library(\"test.dll\")]\n    extern \"C\" fn FailFast")
    );
    assert!(
        rdl.contains("UseReferences(input: REFGUID, output: *mut GUID, count: *const i32) -> i32")
    );
}
