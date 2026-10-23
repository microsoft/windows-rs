#[cfg(feature = "shared")]
#[inline]
pub unsafe fn Beep(dwfreq: super::DWORD, dwduration: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Beep(dwfreq : super::DWORD, dwduration : super::DWORD) -> windows_core::BOOL);
    unsafe { Beep(dwfreq, dwduration) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DecodePointer(ptr: Option<super::PVOID>) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn DecodePointer(ptr : super::PVOID) -> super::PVOID);
    unsafe { DecodePointer(ptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DecodeRemotePointer(processhandle: super::HANDLE, ptr: Option<super::PVOID>, decodedptr: *mut super::PVOID) -> windows_core::HRESULT {
    windows_core::link!("kernelbase.dll" "system" fn DecodeRemotePointer(processhandle : super::HANDLE, ptr : super::PVOID, decodedptr : *mut super::PVOID) -> windows_core::HRESULT);
    unsafe { DecodeRemotePointer(processhandle, ptr.unwrap_or(core::mem::zeroed()) as _, decodedptr as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DecodeSystemPointer(ptr: Option<super::PVOID>) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn DecodeSystemPointer(ptr : super::PVOID) -> super::PVOID);
    unsafe { DecodeSystemPointer(ptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EncodePointer(ptr: Option<super::PVOID>) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn EncodePointer(ptr : super::PVOID) -> super::PVOID);
    unsafe { EncodePointer(ptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EncodeRemotePointer(processhandle: super::HANDLE, ptr: Option<super::PVOID>, encodedptr: *mut super::PVOID) -> windows_core::HRESULT {
    windows_core::link!("kernelbase.dll" "system" fn EncodeRemotePointer(processhandle : super::HANDLE, ptr : super::PVOID, encodedptr : *mut super::PVOID) -> windows_core::HRESULT);
    unsafe { EncodeRemotePointer(processhandle, ptr.unwrap_or(core::mem::zeroed()) as _, encodedptr as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EncodeSystemPointer(ptr: Option<super::PVOID>) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn EncodeSystemPointer(ptr : super::PVOID) -> super::PVOID);
    unsafe { EncodeSystemPointer(ptr.unwrap_or(core::mem::zeroed()) as _) }
}
