#[inline]
pub unsafe fn FilterAttach<P0, P1, P2>(lpfiltername: P0, lpvolumename: P1, lpinstancename: P2, dwcreatedinstancenamelength: Option<u32>, lpcreatedinstancename: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterAttach(lpfiltername : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR, lpinstancename : windows_core::PCWSTR, dwcreatedinstancenamelength : u32, lpcreatedinstancename : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { FilterAttach(lpfiltername.param().abi(), lpvolumename.param().abi(), lpinstancename.param().abi(), dwcreatedinstancenamelength.unwrap_or(core::mem::zeroed()) as _, lpcreatedinstancename.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FilterAttachAtAltitude<P0, P1, P2, P3>(lpfiltername: P0, lpvolumename: P1, lpaltitude: P2, lpinstancename: P3, dwcreatedinstancenamelength: Option<u32>, lpcreatedinstancename: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterAttachAtAltitude(lpfiltername : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR, lpaltitude : windows_core::PCWSTR, lpinstancename : windows_core::PCWSTR, dwcreatedinstancenamelength : u32, lpcreatedinstancename : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { FilterAttachAtAltitude(lpfiltername.param().abi(), lpvolumename.param().abi(), lpaltitude.param().abi(), lpinstancename.param().abi(), dwcreatedinstancenamelength.unwrap_or(core::mem::zeroed()) as _, lpcreatedinstancename.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterClose(hfilter: super::HFILTER) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterClose(hfilter : super::HFILTER) -> windows_core::HRESULT);
    unsafe { FilterClose(hfilter) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn FilterConnectCommunicationPort<P0>(lpportname: P0, dwoptions: u32, lpcontext: Option<*const core::ffi::c_void>, wsizeofcontext: u16, lpsecurityattributes: Option<*const super::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterConnectCommunicationPort(lpportname : windows_core::PCWSTR, dwoptions : u32, lpcontext : *const core::ffi::c_void, wsizeofcontext : u16, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, hport : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        FilterConnectCommunicationPort(lpportname.param().abi(), dwoptions, lpcontext.unwrap_or(core::mem::zeroed()) as _, wsizeofcontext, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterCreate<P0>(lpfiltername: P0) -> windows_core::Result<super::HFILTER>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterCreate(lpfiltername : windows_core::PCWSTR, hfilter : *mut super::HFILTER) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        FilterCreate(lpfiltername.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn FilterDetach<P0, P1, P2>(lpfiltername: P0, lpvolumename: P1, lpinstancename: P2) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterDetach(lpfiltername : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR, lpinstancename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { FilterDetach(lpfiltername.param().abi(), lpvolumename.param().abi(), lpinstancename.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FilterFindClose(hfilterfind: super::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterFindClose(hfilterfind : super::HANDLE) -> windows_core::HRESULT);
    unsafe { FilterFindClose(hfilterfind) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterFindFirst(dwinformationclass: super::FILTER_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterfind: *mut super::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterFindFirst(dwinformationclass : super::FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpfilterfind : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe { FilterFindFirst(dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _, lpfilterfind as _) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterFindNext(hfilterfind: super::HANDLE, dwinformationclass: super::FILTER_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterFindNext(hfilterfind : super::HANDLE, dwinformationclass : super::FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    unsafe { FilterFindNext(hfilterfind, dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _) }
}
#[inline]
pub unsafe fn FilterGetDosName<P0>(lpvolumename: P0, lpdosname: &mut [u16]) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterGetDosName(lpvolumename : windows_core::PCWSTR, lpdosname : windows_core::PWSTR, dwdosnamebuffersize : u32) -> windows_core::HRESULT);
    unsafe { FilterGetDosName(lpvolumename.param().abi(), core::mem::transmute(lpdosname.as_mut_ptr()), lpdosname.len().try_into().unwrap()) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterGetInformation(hfilter: super::HFILTER, dwinformationclass: super::FILTER_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterGetInformation(hfilter : super::HFILTER, dwinformationclass : super::FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    unsafe { FilterGetInformation(hfilter, dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _) }
}
#[cfg(all(feature = "fltuserstructures", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn FilterGetMessage(hport: super::HANDLE, lpmessagebuffer: *mut super::FILTER_MESSAGE_HEADER, dwmessagebuffersize: u32, lpoverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterGetMessage(hport : super::HANDLE, lpmessagebuffer : *mut super::FILTER_MESSAGE_HEADER, dwmessagebuffersize : u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::HRESULT);
    unsafe { FilterGetMessage(hport, lpmessagebuffer as _, dwmessagebuffersize, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterInstanceClose(hinstance: super::HFILTER_INSTANCE) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterInstanceClose(hinstance : super::HFILTER_INSTANCE) -> windows_core::HRESULT);
    unsafe { FilterInstanceClose(hinstance) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterInstanceCreate<P0, P1, P2>(lpfiltername: P0, lpvolumename: P1, lpinstancename: P2) -> windows_core::Result<super::HFILTER_INSTANCE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterInstanceCreate(lpfiltername : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR, lpinstancename : windows_core::PCWSTR, hinstance : *mut super::HFILTER_INSTANCE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        FilterInstanceCreate(lpfiltername.param().abi(), lpvolumename.param().abi(), lpinstancename.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FilterInstanceFindClose(hfilterinstancefind: super::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterInstanceFindClose(hfilterinstancefind : super::HANDLE) -> windows_core::HRESULT);
    unsafe { FilterInstanceFindClose(hfilterinstancefind) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterInstanceFindFirst<P0>(lpfiltername: P0, dwinformationclass: super::INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterinstancefind: *mut super::HANDLE) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterInstanceFindFirst(lpfiltername : windows_core::PCWSTR, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpfilterinstancefind : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe { FilterInstanceFindFirst(lpfiltername.param().abi(), dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _, lpfilterinstancefind as _) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterInstanceFindNext(hfilterinstancefind: super::HANDLE, dwinformationclass: super::INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterInstanceFindNext(hfilterinstancefind : super::HANDLE, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    unsafe { FilterInstanceFindNext(hfilterinstancefind, dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterInstanceGetInformation(hinstance: super::HFILTER_INSTANCE, dwinformationclass: super::INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterInstanceGetInformation(hinstance : super::HFILTER_INSTANCE, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    unsafe { FilterInstanceGetInformation(hinstance, dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _) }
}
#[inline]
pub unsafe fn FilterLoad<P0>(lpfiltername: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterLoad(lpfiltername : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { FilterLoad(lpfiltername.param().abi()) }
}
#[cfg(all(feature = "bcrypt", feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterReplyMessage(hport: super::HANDLE, lpreplybuffer: *const super::FILTER_REPLY_HEADER, dwreplybuffersize: u32) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterReplyMessage(hport : super::HANDLE, lpreplybuffer : *const super::FILTER_REPLY_HEADER, dwreplybuffersize : u32) -> windows_core::HRESULT);
    unsafe { FilterReplyMessage(hport, lpreplybuffer, dwreplybuffersize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FilterSendMessage(hport: super::HANDLE, lpinbuffer: *const core::ffi::c_void, dwinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, dwoutbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterSendMessage(hport : super::HANDLE, lpinbuffer : *const core::ffi::c_void, dwinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, dwoutbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    unsafe { FilterSendMessage(hport, lpinbuffer, dwinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, dwoutbuffersize, lpbytesreturned as _) }
}
#[inline]
pub unsafe fn FilterUnload<P0>(lpfiltername: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterUnload(lpfiltername : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { FilterUnload(lpfiltername.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FilterVolumeFindClose(hvolumefind: super::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterVolumeFindClose(hvolumefind : super::HANDLE) -> windows_core::HRESULT);
    unsafe { FilterVolumeFindClose(hvolumefind) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterVolumeFindFirst(dwinformationclass: super::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumefind: *mut super::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterVolumeFindFirst(dwinformationclass : super::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpvolumefind : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe { FilterVolumeFindFirst(dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _, lpvolumefind as _) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterVolumeFindNext(hvolumefind: super::HANDLE, dwinformationclass: super::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterVolumeFindNext(hvolumefind : super::HANDLE, dwinformationclass : super::FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    unsafe { FilterVolumeFindNext(hvolumefind, dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FilterVolumeInstanceFindClose(hvolumeinstancefind: super::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindClose(hvolumeinstancefind : super::HANDLE) -> windows_core::HRESULT);
    unsafe { FilterVolumeInstanceFindClose(hvolumeinstancefind) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterVolumeInstanceFindFirst<P0>(lpvolumename: P0, dwinformationclass: super::INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumeinstancefind: *mut super::HANDLE) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindFirst(lpvolumename : windows_core::PCWSTR, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpvolumeinstancefind : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe { FilterVolumeInstanceFindFirst(lpvolumename.param().abi(), dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _, lpvolumeinstancefind as _) }
}
#[cfg(all(feature = "fltuserstructures", feature = "winnt"))]
#[inline]
pub unsafe fn FilterVolumeInstanceFindNext(hvolumeinstancefind: super::HANDLE, dwinformationclass: super::INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindNext(hvolumeinstancefind : super::HANDLE, dwinformationclass : super::INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    unsafe { FilterVolumeInstanceFindNext(hvolumeinstancefind, dwinformationclass, lpbuffer as _, dwbuffersize, lpbytesreturned as _) }
}
pub const FLT_MGR_AFTER_XPSP2: u32 = 1;
pub const FLT_MGR_BASELINE: u32 = 1;
pub const FLT_MGR_LONGHORN: u32 = 1;
pub const FLT_MGR_WIN7: u32 = 1;
pub const FLT_MGR_WIN8: u32 = 1;
pub const FLT_MGR_WINBLUE: u32 = 1;
pub const FLT_PORT_FLAG_SYNC_HANDLE: u32 = 1;
