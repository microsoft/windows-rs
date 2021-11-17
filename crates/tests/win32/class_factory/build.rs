fn main() {
    windows::core::build_legacy!(
        Windows::Foundation::{IClosable, IStringable},
        Windows::Win32::System::Com::IClassFactory
    );
}
