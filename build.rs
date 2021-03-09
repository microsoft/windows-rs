fn main() {
    windows_macros::build!(
        windows::foundation::{IReference, IStringable, PropertyValue},
        windows::win32::automation::{BSTR, GetErrorInfo, IErrorInfo, SetErrorInfo},
        windows::win32::com::{CoTaskMemAlloc, CoTaskMemFree},
        windows::win32::winrt::{IRestrictedErrorInfo, ILanguageExceptionErrorInfo2},
        windows::win32::debug::{GetLastError, FormatMessageW},
        windows::win32::system_services::{CreateEventW, SetEvent, WaitForSingleObject},
        windows::win32::windows_programming::{CloseHandle},
    );
}
