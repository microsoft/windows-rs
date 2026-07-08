// ICU's `UChar` is `char16_t` (and there are `char32_t` users too). These
// C++ character types must map to fixed-width unsigned integers (U16/U32),
// exactly like `WCHAR`, rather than reaching the unhandled-CXTypeKind panic.
struct Text {
    char16_t First;
    char32_t Wide;
};
