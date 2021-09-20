fn main() {
    windows::build!(
        Windows::Foundation::{IClosable, IStringable},
        Windows::Win32::System::Com::IClassFactory
    );
}
