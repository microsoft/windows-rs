fn main() {
    windows_macros::build!(
        Windows::Foundation::{IReference, IStringable, PropertyValue},
        Windows::Win32::Automation::{BSTR, GetErrorInfo, IErrorInfo, SetErrorInfo},
        Windows::Win32::WinRT::{IRestrictedErrorInfo, ILanguageExceptionErrorInfo2},
        Windows::Win32::Debug::{GetLastError, FormatMessageW},
        Windows::Win32::WindowsProgramming::{CloseHandle},
        Windows::Win32::Com::{CoTaskMemAlloc, CoTaskMemFree, CLSIDFromProgID, CoInitializeEx, CoCreateInstance, COINIT},
        Windows::Win32::SystemServices::{
            CreateEventW, SetEvent, WaitForSingleObject, GetProcessHeap, HeapAlloc, HeapFree, GetProcAddress,
            LoadLibraryA, FreeLibrary,
        },
    );
}
