windows_link::link!("fltlib.dll" "system" fn FilterAttach(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR, dwcreatedinstancenamelength : u32, lpcreatedinstancename : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterAttachAtAltitude(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpaltitude : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR, dwcreatedinstancenamelength : u32, lpcreatedinstancename : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterClose(hfilter : super::fltuserstructures::HFILTER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterConnectCommunicationPort(lpportname : windows_sys::core::PCWSTR, dwoptions : u32, lpcontext : *const core::ffi::c_void, wsizeofcontext : u16, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, hport : *mut super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterCreate(lpfiltername : windows_sys::core::PCWSTR, hfilter : *mut super::fltuserstructures::HFILTER) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterDetach(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterFindClose(hfilterfind : super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterFindFirst(dwinformationclass : super::fltuserstructures::FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpfilterfind : *mut super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterFindNext(hfilterfind : super::winnt::HANDLE, dwinformationclass : super::fltuserstructures::FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterGetDosName(lpvolumename : windows_sys::core::PCWSTR, lpdosname : windows_sys::core::PWSTR, dwdosnamebuffersize : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterGetInformation(hfilter : super::fltuserstructures::HFILTER, dwinformationclass : super::fltuserstructures::FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterGetMessage(hport : super::winnt::HANDLE, lpmessagebuffer : *mut super::fltuserstructures::FILTER_MESSAGE_HEADER, dwmessagebuffersize : u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceClose(hinstance : super::fltuserstructures::HFILTER_INSTANCE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceCreate(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR, hinstance : *mut super::fltuserstructures::HFILTER_INSTANCE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceFindClose(hfilterinstancefind : super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceFindFirst(lpfiltername : windows_sys::core::PCWSTR, dwinformationclass : super::fltuserstructures::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpfilterinstancefind : *mut super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceFindNext(hfilterinstancefind : super::winnt::HANDLE, dwinformationclass : super::fltuserstructures::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceGetInformation(hinstance : super::fltuserstructures::HFILTER_INSTANCE, dwinformationclass : super::fltuserstructures::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterLoad(lpfiltername : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterReplyMessage(hport : super::winnt::HANDLE, lpreplybuffer : *const super::fltuserstructures::FILTER_REPLY_HEADER, dwreplybuffersize : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterSendMessage(hport : super::winnt::HANDLE, lpinbuffer : *const core::ffi::c_void, dwinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, dwoutbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterUnload(lpfiltername : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeFindClose(hvolumefind : super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeFindFirst(dwinformationclass : super::fltuserstructures::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpvolumefind : *mut super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeFindNext(hvolumefind : super::winnt::HANDLE, dwinformationclass : super::fltuserstructures::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindClose(hvolumeinstancefind : super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindFirst(lpvolumename : windows_sys::core::PCWSTR, dwinformationclass : super::fltuserstructures::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpvolumeinstancefind : *mut super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_fltuserstructures", feature = "Win32_winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindNext(hvolumeinstancefind : super::winnt::HANDLE, dwinformationclass : super::fltuserstructures::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_sys::core::HRESULT);
pub const FLT_MGR_AFTER_XPSP2: u32 = 1;
pub const FLT_MGR_BASELINE: u32 = 1;
pub const FLT_MGR_LONGHORN: u32 = 1;
pub const FLT_MGR_WIN7: u32 = 1;
pub const FLT_MGR_WIN8: u32 = 1;
pub const FLT_MGR_WINBLUE: u32 = 1;
pub const FLT_PORT_FLAG_SYNC_HANDLE: u32 = 1;
