fn main() -> std::io::Result<()> {
    let tokens = windows_macros::generate! {
        Windows::{
            Foundation::{IReference, IStringable, PropertyValue},
            Win32::{
                Foundation::{CloseHandle, CO_E_NOTINITIALIZED, E_NOINTERFACE, E_POINTER},
                System::{
                    Com::{CoCreateGuid, CoTaskMemAlloc, CoTaskMemFree, IAgileObject},
                    Diagnostics::Debug::{FormatMessageW, GetLastError},
                    LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA},
                    Memory::{GetProcessHeap, HeapAlloc, HeapFree},
                    OleAutomation::{GetErrorInfo, IErrorInfo, SetErrorInfo},
                    Threading::{CreateEventA, SetEvent, WaitForSingleObject},
                    WinRT::{
                        ILanguageExceptionErrorInfo2, IRestrictedErrorInfo, IWeakReference,
                        IWeakReferenceSource,
                    },
                },
            },
        },
    };

    let mut path: std::path::PathBuf = windows_gen::workspace_dir().into();
    path.push("src");
    path.push("bindings.rs");

    std::fs::write(&path, tokens)?;

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}
