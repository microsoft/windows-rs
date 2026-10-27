#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn Beep(dwfreq : super::DWORD, dwduration : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DecodePointer(ptr : super::PVOID) -> super::PVOID);
#[cfg(feature = "winnt")]
windows_link::link!("kernelbase.dll" "system" fn DecodeRemotePointer(processhandle : super::HANDLE, ptr : super::PVOID, decodedptr : *mut super::PVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DecodeSystemPointer(ptr : super::PVOID) -> super::PVOID);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EncodePointer(ptr : super::PVOID) -> super::PVOID);
#[cfg(feature = "winnt")]
windows_link::link!("kernelbase.dll" "system" fn EncodeRemotePointer(processhandle : super::HANDLE, ptr : super::PVOID, encodedptr : *mut super::PVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EncodeSystemPointer(ptr : super::PVOID) -> super::PVOID);
