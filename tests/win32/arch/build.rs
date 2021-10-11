fn main() {
    windows::build! {
        Windows::Win32::System::SystemServices::KNONVOLATILE_CONTEXT_POINTERS,
    };
}
