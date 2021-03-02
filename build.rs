fn main() {
    windows_macros::build!(
        windows::foundation::{IReference, IStringable, PropertyValue},
        windows::win32::automation::{BSTR, GetErrorInfo, IErrorInfo, SetErrorInfo},
    );
}
