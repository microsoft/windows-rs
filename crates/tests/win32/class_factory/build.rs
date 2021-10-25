fn main() {
    windows::runtime::build!(
        Windows::Foundation::{IClosable, IStringable},
        Windows::Win32::System::Com::IClassFactory
    );
}
