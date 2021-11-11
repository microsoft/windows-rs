fn main() {
    windows::core::build!(
        Windows::Foundation::{IClosable, IStringable},
        Windows::Win32::System::Com::IClassFactory
    );
}
