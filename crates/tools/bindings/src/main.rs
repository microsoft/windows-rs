// TODO: remove this and windows-sys rather than generated bindings

fn main() -> std::io::Result<()> {
    let tokens = macros::generate! {
        Windows::Foundation::{IReference, IStringable, PropertyValue},
        Windows::Win32::Foundation::{CloseHandle, GetLastError, E_NOINTERFACE, CLASS_E_CLASSNOTAVAILABLE, E_OUTOFMEMORY, S_OK, CO_E_NOTINITIALIZED},
        Windows::Win32::System::Com::{CoCreateGuid, CoTaskMemAlloc, CoTaskMemFree, IAgileObject, GetErrorInfo, IErrorInfo, SetErrorInfo},
        Windows::Win32::System::Diagnostics::Debug::FormatMessageW,
        Windows::Win32::System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA},
        Windows::Win32::System::Memory::{GetProcessHeap, HeapAlloc, HeapFree},
        Windows::Win32::System::Threading::{CreateEventA, SetEvent, WaitForSingleObject},
        Windows::Win32::System::WinRT::{
            ILanguageExceptionErrorInfo2, IRestrictedErrorInfo, IWeakReference,
            IWeakReferenceSource,
        },
    };

    let mut path: std::path::PathBuf = reader::workspace_dir().into();
    path.push("src/core/bindings.rs");

    std::fs::write(&path, tokens)?;

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}
