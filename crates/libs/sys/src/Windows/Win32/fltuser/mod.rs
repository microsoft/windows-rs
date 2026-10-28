#[cfg(feature = "shared")]
windows_link::link!("fltlib.dll" "system" fn FilterAttach(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR, dwcreatedinstancenamelength : super::DWORD, lpcreatedinstancename : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("fltlib.dll" "system" fn FilterAttachAtAltitude(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpaltitude : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR, dwcreatedinstancenamelength : super::DWORD, lpcreatedinstancename : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "fltuserstructures")]
windows_link::link!("fltlib.dll" "system" fn FilterClose(hfilter : super::HFILTER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterConnectCommunicationPort(lpportname : windows_sys::core::PCWSTR, dwoptions : super::DWORD, lpcontext : super::LPCVOID, wsizeofcontext : super::WORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, hport : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "fltuserstructures")]
windows_link::link!("fltlib.dll" "system" fn FilterCreate(lpfiltername : windows_sys::core::PCWSTR, hfilter : *mut super::HFILTER) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterDetach(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterFindClose(hfilterfind : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared"))]
windows_link::link!("fltlib.dll" "system" fn FilterFindFirst(dwinformationclass : super::FILTER_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD, lpfilterfind : super::LPHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterFindNext(hfilterfind : super::HANDLE, dwinformationclass : super::FILTER_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("fltlib.dll" "system" fn FilterGetDosName(lpvolumename : windows_sys::core::PCWSTR, lpdosname : windows_sys::core::PWSTR, dwdosnamebuffersize : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared"))]
windows_link::link!("fltlib.dll" "system" fn FilterGetInformation(hfilter : super::HFILTER, dwinformationclass : super::FILTER_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterGetMessage(hport : super::HANDLE, lpmessagebuffer : super::PFILTER_MESSAGE_HEADER, dwmessagebuffersize : super::DWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::HRESULT);
#[cfg(feature = "fltuserstructures")]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceClose(hinstance : super::HFILTER_INSTANCE) -> windows_sys::core::HRESULT);
#[cfg(feature = "fltuserstructures")]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceCreate(lpfiltername : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR, lpinstancename : windows_sys::core::PCWSTR, hinstance : *mut super::HFILTER_INSTANCE) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceFindClose(hfilterinstancefind : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceFindFirst(lpfiltername : windows_sys::core::PCWSTR, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD, lpfilterinstancefind : super::LPHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceFindNext(hfilterinstancefind : super::HANDLE, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared"))]
windows_link::link!("fltlib.dll" "system" fn FilterInstanceGetInformation(hinstance : super::HFILTER_INSTANCE, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterLoad(lpfiltername : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "shared", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterReplyMessage(hport : super::HANDLE, lpreplybuffer : super::PFILTER_REPLY_HEADER, dwreplybuffersize : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterSendMessage(hport : super::HANDLE, lpinbuffer : super::LPVOID, dwinbuffersize : super::DWORD, lpoutbuffer : super::LPVOID, dwoutbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD) -> windows_sys::core::HRESULT);
windows_link::link!("fltlib.dll" "system" fn FilterUnload(lpfiltername : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeFindClose(hvolumefind : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeFindFirst(dwinformationclass : super::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD, lpvolumefind : super::PHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeFindNext(hvolumefind : super::HANDLE, dwinformationclass : super::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindClose(hvolumeinstancefind : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindFirst(lpvolumename : windows_sys::core::PCWSTR, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD, lpvolumeinstancefind : super::LPHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "fltuserstructures", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindNext(hvolumeinstancefind : super::HANDLE, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : super::LPVOID, dwbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD) -> windows_sys::core::HRESULT);
pub const FLT_MGR_AFTER_XPSP2: bool = true;
pub const FLT_MGR_BASELINE: bool = true;
pub const FLT_MGR_LONGHORN: bool = true;
pub const FLT_MGR_WIN7: bool = true;
pub const FLT_MGR_WIN8: bool = true;
pub const FLT_MGR_WINBLUE: bool = true;
pub const FLT_PORT_FLAG_SYNC_HANDLE: i32 = 1;
