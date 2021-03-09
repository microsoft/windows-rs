fn main() {
    windows_macros::build!(
        windows::foundation::{IReference, IStringable, PropertyValue},
        windows::win32::automation::{BSTR, GetErrorInfo, IErrorInfo, SetErrorInfo},
        windows::win32::com::{CoTaskMemAlloc, CoTaskMemFree},
        windows::win32::winrt::{IRestrictedErrorInfo, ILanguageExceptionErrorInfo2},
        windows::win32::debug::{GetLastError, FormatMessageW},
    );
}
