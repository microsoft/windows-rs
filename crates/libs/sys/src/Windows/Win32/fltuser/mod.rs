windows_link::link!("fltlib.dll" "system" fn FilterAttach(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR, dwcreatedinstancenamelength : u32, lpcreatedinstancename : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterAttachAtAltitude(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpaltitude : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR, dwcreatedinstancenamelength : u32, lpcreatedinstancename : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterClose(hfilter : super::HFILTER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterConnectCommunicationPort(lpportname : windows_sys::core::PCWSTR, dwoptions : u32, lpcontext : *const core::ffi::c_void, wsizeofcontext : u16, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, hport : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterCreate(lpfiltername : windows_sys::core::PCWSTR, hfilter : *mut super::HFILTER) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterDetach(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterFindClose(hfilterfind : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterFindFirst(dwinformationclass : super::FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpfilterfind : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterFindNext(hfilterfind : super::HANDLE, dwinformationclass : super::FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterGetDosName(lpvolumename : windows_sys::core::PCWSTR, lpdosname : windows_sys::core::PWSTR, dwdosnamebuffersize : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterGetInformation(hfilter : super::HFILTER, dwinformationclass : super::FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterGetMessage(hport : super::HANDLE, lpmessagebuffer : *mut super::FILTER_MESSAGE_HEADER, dwmessagebuffersize : u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceClose(hinstance : super::HFILTER_INSTANCE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceCreate(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR, hinstance : *mut super::HFILTER_INSTANCE) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceFindClose(hfilterinstancefind : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceFindFirst(lpfiltername : windows_sys::core::PCWSTR, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpfilterinstancefind : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceFindNext(hfilterinstancefind : super::HANDLE, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceGetInformation(hinstance : super::HFILTER_INSTANCE, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterLoad(lpfiltername : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "bcrypt", feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterReplyMessage(hport : super::HANDLE, lpreplybuffer : *const super::FILTER_REPLY_HEADER, dwreplybuffersize : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterSendMessage(hport : super::HANDLE, lpinbuffer : *const core::ffi::c_void, dwinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, dwoutbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterUnload(lpfiltername : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeFindClose(hvolumefind : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeFindFirst(dwinformationclass : super::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpvolumefind : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeFindNext(hvolumefind : super::HANDLE, dwinformationclass : super::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindClose(hvolumeinstancefind : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindFirst(lpvolumename : windows_sys::core::PCWSTR, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpvolumeinstancefind : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindNext(hvolumeinstancefind : super::HANDLE, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
pub const FLT_MGR_AFTER_XPSP2: u32 = 1;
pub const FLT_MGR_BASELINE: u32 = 1;
pub const FLT_MGR_LONGHORN: u32 = 1;
pub const FLT_MGR_WIN7: u32 = 1;
pub const FLT_MGR_WIN8: u32 = 1;
pub const FLT_MGR_WINBLUE: u32 = 1;
pub const FLT_PORT_FLAG_SYNC_HANDLE: u32 = 1;
