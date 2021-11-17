fn main() {
    // This struct presents unique challenges to the type reader as it is both arch-specific
    // and one of those definitions has nested types. This combination is tricky because
    // traditional scope resolution is insufficient.
    windows::core::build_legacy! {
        Windows::Win32::System::Diagnostics::Debug::KNONVOLATILE_CONTEXT_POINTERS,
    };
}
