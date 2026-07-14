#[inline]
pub unsafe fn MultinetGetConnectionPerformanceA(lpnetresource: *const NETRESOURCEA, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32 {
    windows_core::link!("mpr.dll" "system" fn MultinetGetConnectionPerformanceA(lpnetresource : *const NETRESOURCEA, lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT) -> u32);
    unsafe { MultinetGetConnectionPerformanceA(lpnetresource, lpnetconnectinfostruct as _) }
}
#[inline]
pub unsafe fn MultinetGetConnectionPerformanceW(lpnetresource: *const NETRESOURCEW, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32 {
    windows_core::link!("mpr.dll" "system" fn MultinetGetConnectionPerformanceW(lpnetresource : *const NETRESOURCEW, lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT) -> u32);
    unsafe { MultinetGetConnectionPerformanceW(lpnetresource, lpnetconnectinfostruct as _) }
}
#[inline]
pub unsafe fn WNetAddConnection2A<P1, P2>(lpnetresource: *const NETRESOURCEA, lppassword: P1, lpusername: P2, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetAddConnection2A(lpnetresource : *const NETRESOURCEA, lppassword : windows_core::PCSTR, lpusername : windows_core::PCSTR, dwflags : u32) -> u32);
    unsafe { WNetAddConnection2A(lpnetresource, lppassword.param().abi(), lpusername.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn WNetAddConnection2W<P1, P2>(lpnetresource: *const NETRESOURCEW, lppassword: P1, lpusername: P2, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetAddConnection2W(lpnetresource : *const NETRESOURCEW, lppassword : windows_core::PCWSTR, lpusername : windows_core::PCWSTR, dwflags : u32) -> u32);
    unsafe { WNetAddConnection2W(lpnetresource, lppassword.param().abi(), lpusername.param().abi(), dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetAddConnection3A<P2, P3>(hwndowner: Option<super::windef::HWND>, lpnetresource: *const NETRESOURCEA, lppassword: P2, lpusername: P3, dwflags: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetAddConnection3A(hwndowner : super::windef::HWND, lpnetresource : *const NETRESOURCEA, lppassword : windows_core::PCSTR, lpusername : windows_core::PCSTR, dwflags : u32) -> u32);
    unsafe { WNetAddConnection3A(hwndowner.unwrap_or(core::mem::zeroed()) as _, lpnetresource, lppassword.param().abi(), lpusername.param().abi(), dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetAddConnection3W<P2, P3>(hwndowner: Option<super::windef::HWND>, lpnetresource: *const NETRESOURCEW, lppassword: P2, lpusername: P3, dwflags: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetAddConnection3W(hwndowner : super::windef::HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_core::PCWSTR, lpusername : windows_core::PCWSTR, dwflags : u32) -> u32);
    unsafe { WNetAddConnection3W(hwndowner.unwrap_or(core::mem::zeroed()) as _, lpnetresource, lppassword.param().abi(), lpusername.param().abi(), dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetAddConnection4A(hwndowner: Option<super::windef::HWND>, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: &[u8]) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetAddConnection4A(hwndowner : super::windef::HWND, lpnetresource : *const NETRESOURCEA, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32) -> u32);
    unsafe { WNetAddConnection4A(hwndowner.unwrap_or(core::mem::zeroed()) as _, lpnetresource, pauthbuffer, cbauthbuffer, dwflags, lpuseoptions.as_ptr(), lpuseoptions.len().try_into().unwrap()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetAddConnection4W(hwndowner: Option<super::windef::HWND>, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: &[u8]) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetAddConnection4W(hwndowner : super::windef::HWND, lpnetresource : *const NETRESOURCEW, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32) -> u32);
    unsafe { WNetAddConnection4W(hwndowner.unwrap_or(core::mem::zeroed()) as _, lpnetresource, pauthbuffer, cbauthbuffer, dwflags, lpuseoptions.as_ptr(), lpuseoptions.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn WNetAddConnectionA<P0, P1, P2>(lpremotename: P0, lppassword: P1, lplocalname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetAddConnectionA(lpremotename : windows_core::PCSTR, lppassword : windows_core::PCSTR, lplocalname : windows_core::PCSTR) -> u32);
    unsafe { WNetAddConnectionA(lpremotename.param().abi(), lppassword.param().abi(), lplocalname.param().abi()) }
}
#[inline]
pub unsafe fn WNetAddConnectionW<P0, P1, P2>(lpremotename: P0, lppassword: P1, lplocalname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetAddConnectionW(lpremotename : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, lplocalname : windows_core::PCWSTR) -> u32);
    unsafe { WNetAddConnectionW(lpremotename.param().abi(), lppassword.param().abi(), lplocalname.param().abi()) }
}
#[inline]
pub unsafe fn WNetCancelConnection2A<P0>(lpname: P0, dwflags: u32, fforce: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetCancelConnection2A(lpname : windows_core::PCSTR, dwflags : u32, fforce : windows_core::BOOL) -> u32);
    unsafe { WNetCancelConnection2A(lpname.param().abi(), dwflags, fforce.into()) }
}
#[inline]
pub unsafe fn WNetCancelConnection2W<P0>(lpname: P0, dwflags: u32, fforce: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetCancelConnection2W(lpname : windows_core::PCWSTR, dwflags : u32, fforce : windows_core::BOOL) -> u32);
    unsafe { WNetCancelConnection2W(lpname.param().abi(), dwflags, fforce.into()) }
}
#[inline]
pub unsafe fn WNetCancelConnectionA<P0>(lpname: P0, fforce: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetCancelConnectionA(lpname : windows_core::PCSTR, fforce : windows_core::BOOL) -> u32);
    unsafe { WNetCancelConnectionA(lpname.param().abi(), fforce.into()) }
}
#[inline]
pub unsafe fn WNetCancelConnectionW<P0>(lpname: P0, fforce: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetCancelConnectionW(lpname : windows_core::PCWSTR, fforce : windows_core::BOOL) -> u32);
    unsafe { WNetCancelConnectionW(lpname.param().abi(), fforce.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WNetCloseEnum(henum: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetCloseEnum(henum : super::winnt::HANDLE) -> u32);
    unsafe { WNetCloseEnum(henum) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetConnectionDialog(hwnd: super::windef::HWND, dwtype: u32) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetConnectionDialog(hwnd : super::windef::HWND, dwtype : u32) -> u32);
    unsafe { WNetConnectionDialog(hwnd, dwtype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetConnectionDialog1A(lpconndlgstruct : *mut CONNECTDLGSTRUCTA) -> u32);
    unsafe { WNetConnectionDialog1A(lpconndlgstruct as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetConnectionDialog1W(lpconndlgstruct : *mut CONNECTDLGSTRUCTW) -> u32);
    unsafe { WNetConnectionDialog1W(lpconndlgstruct as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetDisconnectDialog(hwnd: Option<super::windef::HWND>, dwtype: u32) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetDisconnectDialog(hwnd : super::windef::HWND, dwtype : u32) -> u32);
    unsafe { WNetDisconnectDialog(hwnd.unwrap_or(core::mem::zeroed()) as _, dwtype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetDisconnectDialog1A(lpconndlgstruct : *const DISCDLGSTRUCTA) -> u32);
    unsafe { WNetDisconnectDialog1A(lpconndlgstruct) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetDisconnectDialog1W(lpconndlgstruct : *const DISCDLGSTRUCTW) -> u32);
    unsafe { WNetDisconnectDialog1W(lpconndlgstruct) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WNetEnumResourceA(henum: super::winnt::HANDLE, lpccount: *mut u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetEnumResourceA(henum : super::winnt::HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
    unsafe { WNetEnumResourceA(henum, lpccount as _, lpbuffer as _, lpbuffersize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WNetEnumResourceW(henum: super::winnt::HANDLE, lpccount: *mut u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetEnumResourceW(henum : super::winnt::HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
    unsafe { WNetEnumResourceW(henum, lpccount as _, lpbuffer as _, lpbuffersize as _) }
}
#[inline]
pub unsafe fn WNetGetConnectionA<P0>(lplocalname: P0, lpremotename: Option<windows_core::PSTR>, lpnlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetGetConnectionA(lplocalname : windows_core::PCSTR, lpremotename : windows_core::PSTR, lpnlength : *mut u32) -> u32);
    unsafe { WNetGetConnectionA(lplocalname.param().abi(), lpremotename.unwrap_or(core::mem::zeroed()) as _, lpnlength as _) }
}
#[inline]
pub unsafe fn WNetGetConnectionW<P0>(lplocalname: P0, lpremotename: Option<windows_core::PWSTR>, lpnlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetGetConnectionW(lplocalname : windows_core::PCWSTR, lpremotename : windows_core::PWSTR, lpnlength : *mut u32) -> u32);
    unsafe { WNetGetConnectionW(lplocalname.param().abi(), lpremotename.unwrap_or(core::mem::zeroed()) as _, lpnlength as _) }
}
#[inline]
pub unsafe fn WNetGetLastErrorA(lperror: *mut u32, lperrorbuf: &mut [u8], lpnamebuf: &mut [u8]) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetGetLastErrorA(lperror : *mut u32, lperrorbuf : windows_core::PSTR, nerrorbufsize : u32, lpnamebuf : windows_core::PSTR, nnamebufsize : u32) -> u32);
    unsafe { WNetGetLastErrorA(lperror as _, core::mem::transmute(lperrorbuf.as_mut_ptr()), lperrorbuf.len().try_into().unwrap(), core::mem::transmute(lpnamebuf.as_mut_ptr()), lpnamebuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn WNetGetLastErrorW(lperror: *mut u32, lperrorbuf: &mut [u16], lpnamebuf: &mut [u16]) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetGetLastErrorW(lperror : *mut u32, lperrorbuf : windows_core::PWSTR, nerrorbufsize : u32, lpnamebuf : windows_core::PWSTR, nnamebufsize : u32) -> u32);
    unsafe { WNetGetLastErrorW(lperror as _, core::mem::transmute(lperrorbuf.as_mut_ptr()), lperrorbuf.len().try_into().unwrap(), core::mem::transmute(lpnamebuf.as_mut_ptr()), lpnamebuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn WNetGetNetworkInformationA<P0>(lpprovider: P0, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetGetNetworkInformationA(lpprovider : windows_core::PCSTR, lpnetinfostruct : *mut NETINFOSTRUCT) -> u32);
    unsafe { WNetGetNetworkInformationA(lpprovider.param().abi(), lpnetinfostruct as _) }
}
#[inline]
pub unsafe fn WNetGetNetworkInformationW<P0>(lpprovider: P0, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetGetNetworkInformationW(lpprovider : windows_core::PCWSTR, lpnetinfostruct : *mut NETINFOSTRUCT) -> u32);
    unsafe { WNetGetNetworkInformationW(lpprovider.param().abi(), lpnetinfostruct as _) }
}
#[inline]
pub unsafe fn WNetGetProviderNameA(dwnettype: u32, lpprovidername: windows_core::PSTR, lpbuffersize: *mut u32) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetGetProviderNameA(dwnettype : u32, lpprovidername : windows_core::PSTR, lpbuffersize : *mut u32) -> u32);
    unsafe { WNetGetProviderNameA(dwnettype, lpprovidername, lpbuffersize as _) }
}
#[inline]
pub unsafe fn WNetGetProviderNameW(dwnettype: u32, lpprovidername: windows_core::PWSTR, lpbuffersize: *mut u32) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetGetProviderNameW(dwnettype : u32, lpprovidername : windows_core::PWSTR, lpbuffersize : *mut u32) -> u32);
    unsafe { WNetGetProviderNameW(dwnettype, lpprovidername, lpbuffersize as _) }
}
#[inline]
pub unsafe fn WNetGetResourceInformationA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut windows_core::PSTR) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetGetResourceInformationA(lpnetresource : *const NETRESOURCEA, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32, lplpsystem : *mut windows_core::PSTR) -> u32);
    unsafe { WNetGetResourceInformationA(lpnetresource, lpbuffer as _, lpcbbuffer as _, lplpsystem as _) }
}
#[inline]
pub unsafe fn WNetGetResourceInformationW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut windows_core::PWSTR) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetGetResourceInformationW(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32, lplpsystem : *mut windows_core::PWSTR) -> u32);
    unsafe { WNetGetResourceInformationW(lpnetresource, lpbuffer as _, lpcbbuffer as _, lplpsystem as _) }
}
#[inline]
pub unsafe fn WNetGetResourceParentA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut core::ffi::c_void, lpcbbuffer: *mut u32) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetGetResourceParentA(lpnetresource : *const NETRESOURCEA, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32) -> u32);
    unsafe { WNetGetResourceParentA(lpnetresource, lpbuffer as _, lpcbbuffer as _) }
}
#[inline]
pub unsafe fn WNetGetResourceParentW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpcbbuffer: *mut u32) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetGetResourceParentW(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32) -> u32);
    unsafe { WNetGetResourceParentW(lpnetresource, lpbuffer as _, lpcbbuffer as _) }
}
#[inline]
pub unsafe fn WNetGetUniversalNameA<P0>(lplocalpath: P0, dwinfolevel: u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetGetUniversalNameA(lplocalpath : windows_core::PCSTR, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
    unsafe { WNetGetUniversalNameA(lplocalpath.param().abi(), dwinfolevel, lpbuffer as _, lpbuffersize as _) }
}
#[inline]
pub unsafe fn WNetGetUniversalNameW<P0>(lplocalpath: P0, dwinfolevel: u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetGetUniversalNameW(lplocalpath : windows_core::PCWSTR, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
    unsafe { WNetGetUniversalNameW(lplocalpath.param().abi(), dwinfolevel, lpbuffer as _, lpbuffersize as _) }
}
#[inline]
pub unsafe fn WNetGetUserA<P0>(lpname: P0, lpusername: windows_core::PSTR, lpnlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetGetUserA(lpname : windows_core::PCSTR, lpusername : windows_core::PSTR, lpnlength : *mut u32) -> u32);
    unsafe { WNetGetUserA(lpname.param().abi(), lpusername, lpnlength as _) }
}
#[inline]
pub unsafe fn WNetGetUserW<P0>(lpname: P0, lpusername: windows_core::PWSTR, lpnlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetGetUserW(lpname : windows_core::PCWSTR, lpusername : windows_core::PWSTR, lpnlength : *mut u32) -> u32);
    unsafe { WNetGetUserW(lpname.param().abi(), lpusername, lpnlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WNetOpenEnumA(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: Option<*const NETRESOURCEA>, lphenum: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetOpenEnumA(dwscope : u32, dwtype : u32, dwusage : u32, lpnetresource : *const NETRESOURCEA, lphenum : *mut super::winnt::HANDLE) -> u32);
    unsafe { WNetOpenEnumA(dwscope, dwtype, dwusage, lpnetresource.unwrap_or(core::mem::zeroed()) as _, lphenum as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WNetOpenEnumW(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: Option<*const NETRESOURCEW>, lphenum: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetOpenEnumW(dwscope : u32, dwtype : u32, dwusage : u32, lpnetresource : *const NETRESOURCEW, lphenum : *mut super::winnt::HANDLE) -> u32);
    unsafe { WNetOpenEnumW(dwscope, dwtype, dwusage, lpnetresource.unwrap_or(core::mem::zeroed()) as _, lphenum as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetUseConnection4A(hwndowner: Option<super::windef::HWND>, lpnetresource: *const NETRESOURCEA, pauthbuffer: Option<*const core::ffi::c_void>, cbauthbuffer: u32, dwflags: u32, lpuseoptions: Option<&[u8]>, lpaccessname: Option<windows_core::PSTR>, lpbuffersize: Option<*mut u32>, lpresult: Option<*mut u32>) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetUseConnection4A(hwndowner : super::windef::HWND, lpnetresource : *const NETRESOURCEA, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32, lpaccessname : windows_core::PSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> u32);
    unsafe { WNetUseConnection4A(hwndowner.unwrap_or(core::mem::zeroed()) as _, lpnetresource, pauthbuffer.unwrap_or(core::mem::zeroed()) as _, cbauthbuffer, dwflags, lpuseoptions.map_or(core::ptr::null(), |slice| slice.as_ptr()), lpuseoptions.map_or(0, |slice| slice.len().try_into().unwrap()), lpaccessname.unwrap_or(core::mem::zeroed()) as _, lpbuffersize.unwrap_or(core::mem::zeroed()) as _, lpresult.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetUseConnection4W(hwndowner: Option<super::windef::HWND>, lpnetresource: *const NETRESOURCEW, pauthbuffer: Option<*const core::ffi::c_void>, cbauthbuffer: u32, dwflags: u32, lpuseoptions: Option<&[u8]>, lpaccessname: Option<windows_core::PWSTR>, lpbuffersize: Option<*mut u32>, lpresult: Option<*mut u32>) -> u32 {
    windows_core::link!("mpr.dll" "system" fn WNetUseConnection4W(hwndowner : super::windef::HWND, lpnetresource : *const NETRESOURCEW, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32, lpaccessname : windows_core::PWSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> u32);
    unsafe { WNetUseConnection4W(hwndowner.unwrap_or(core::mem::zeroed()) as _, lpnetresource, pauthbuffer.unwrap_or(core::mem::zeroed()) as _, cbauthbuffer, dwflags, lpuseoptions.map_or(core::ptr::null(), |slice| slice.as_ptr()), lpuseoptions.map_or(0, |slice| slice.len().try_into().unwrap()), lpaccessname.unwrap_or(core::mem::zeroed()) as _, lpbuffersize.unwrap_or(core::mem::zeroed()) as _, lpresult.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetUseConnectionA<P2, P3>(hwndowner: Option<super::windef::HWND>, lpnetresource: *const NETRESOURCEA, lppassword: P2, lpuserid: P3, dwflags: u32, lpaccessname: Option<windows_core::PSTR>, lpbuffersize: Option<*mut u32>, lpresult: Option<*mut u32>) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetUseConnectionA(hwndowner : super::windef::HWND, lpnetresource : *const NETRESOURCEA, lppassword : windows_core::PCSTR, lpuserid : windows_core::PCSTR, dwflags : u32, lpaccessname : windows_core::PSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> u32);
    unsafe { WNetUseConnectionA(hwndowner.unwrap_or(core::mem::zeroed()) as _, lpnetresource, lppassword.param().abi(), lpuserid.param().abi(), dwflags, lpaccessname.unwrap_or(core::mem::zeroed()) as _, lpbuffersize.unwrap_or(core::mem::zeroed()) as _, lpresult.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WNetUseConnectionW<P2, P3>(hwndowner: Option<super::windef::HWND>, lpnetresource: *const NETRESOURCEW, lppassword: P2, lpuserid: P3, dwflags: u32, lpaccessname: Option<windows_core::PWSTR>, lpbuffersize: Option<*mut u32>, lpresult: Option<*mut u32>) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mpr.dll" "system" fn WNetUseConnectionW(hwndowner : super::windef::HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_core::PCWSTR, lpuserid : windows_core::PCWSTR, dwflags : u32, lpaccessname : windows_core::PWSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> u32);
    unsafe { WNetUseConnectionW(hwndowner.unwrap_or(core::mem::zeroed()) as _, lpnetresource, lppassword.param().abi(), lpuserid.param().abi(), dwflags, lpaccessname.unwrap_or(core::mem::zeroed()) as _, lpbuffersize.unwrap_or(core::mem::zeroed()) as _, lpresult.unwrap_or(core::mem::zeroed()) as _) }
}
pub const CONNDLG_CONN_POINT: u32 = 2;
pub const CONNDLG_HIDE_BOX: u32 = 8;
pub const CONNDLG_NOT_PERSIST: u32 = 32;
pub const CONNDLG_PERSIST: u32 = 16;
pub const CONNDLG_RO_PATH: u32 = 1;
pub const CONNDLG_USE_MRU: u32 = 4;
#[cfg(feature = "windef")]
pub type CONNECTDLGSTRUCT = CONNECTDLGSTRUCTA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONNECTDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::windef::HWND,
    pub lpConnRes: LPNETRESOURCEA,
    pub dwFlags: u32,
    pub dwDevNum: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONNECTDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::windef::HWND,
    pub lpConnRes: LPNETRESOURCEW,
    pub dwFlags: u32,
    pub dwDevNum: u32,
}
pub const CONNECT_CMD_SAVECRED: u32 = 4096;
pub const CONNECT_COMMANDLINE: u32 = 2048;
pub const CONNECT_CRED_RESET: u32 = 8192;
pub const CONNECT_CURRENT_MEDIA: u32 = 512;
pub const CONNECT_DEFERRED: u32 = 1024;
pub const CONNECT_GLOBAL_MAPPING: u32 = 262144;
pub const CONNECT_INTERACTIVE: u32 = 8;
pub const CONNECT_LOCALDRIVE: u32 = 256;
pub const CONNECT_NEED_DRIVE: u32 = 32;
pub const CONNECT_PROMPT: u32 = 16;
pub const CONNECT_REDIRECT: u32 = 128;
pub const CONNECT_REFCOUNT: u32 = 64;
pub const CONNECT_REQUIRE_INTEGRITY: u32 = 16384;
pub const CONNECT_REQUIRE_PRIVACY: u32 = 32768;
pub const CONNECT_RESERVED: u32 = 4278190080;
pub const CONNECT_TEMPORARY: u32 = 4;
pub const CONNECT_UPDATE_PROFILE: u32 = 1;
pub const CONNECT_UPDATE_RECENT: u32 = 2;
pub const CONNECT_WRITE_THROUGH_SEMANTICS: u32 = 65536;
#[cfg(feature = "windef")]
pub type DISCDLGSTRUCT = DISCDLGSTRUCTA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISCDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::windef::HWND,
    pub lpLocalName: windows_core::PSTR,
    pub lpRemoteName: windows_core::PSTR,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISCDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::windef::HWND,
    pub lpLocalName: windows_core::PWSTR,
    pub lpRemoteName: windows_core::PWSTR,
    pub dwFlags: u32,
}
pub const DISC_NO_FORCE: u32 = 64;
pub const DISC_UPDATE_PROFILE: u32 = 1;
#[cfg(feature = "windef")]
pub type LPCONNECTDLGSTRUCT = LPCONNECTDLGSTRUCTA;
#[cfg(feature = "windef")]
pub type LPCONNECTDLGSTRUCTA = *mut CONNECTDLGSTRUCTA;
#[cfg(feature = "windef")]
pub type LPCONNECTDLGSTRUCTW = *mut CONNECTDLGSTRUCTW;
#[cfg(feature = "windef")]
pub type LPDISCDLGSTRUCT = LPDISCDLGSTRUCTA;
#[cfg(feature = "windef")]
pub type LPDISCDLGSTRUCTA = *mut DISCDLGSTRUCTA;
#[cfg(feature = "windef")]
pub type LPDISCDLGSTRUCTW = *mut DISCDLGSTRUCTW;
pub type LPNETCONNECTINFOSTRUCT = *mut NETCONNECTINFOSTRUCT;
pub type LPNETINFOSTRUCT = *mut NETINFOSTRUCT;
pub type LPNETRESOURCE = LPNETRESOURCEA;
pub type LPNETRESOURCEA = *mut NETRESOURCEA;
pub type LPNETRESOURCEW = *mut NETRESOURCEW;
pub type LPREMOTE_NAME_INFO = LPREMOTE_NAME_INFOA;
pub type LPREMOTE_NAME_INFOA = *mut REMOTE_NAME_INFOA;
pub type LPREMOTE_NAME_INFOW = *mut REMOTE_NAME_INFOW;
pub type LPUNIVERSAL_NAME_INFO = LPUNIVERSAL_NAME_INFOA;
pub type LPUNIVERSAL_NAME_INFOA = *mut UNIVERSAL_NAME_INFOA;
pub type LPUNIVERSAL_NAME_INFOW = *mut UNIVERSAL_NAME_INFOW;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETCONNECTINFOSTRUCT {
    pub cbStructure: u32,
    pub dwFlags: u32,
    pub dwSpeed: u32,
    pub dwDelay: u32,
    pub dwOptDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETINFOSTRUCT {
    pub cbStructure: u32,
    pub dwProviderVersion: u32,
    pub dwStatus: u32,
    pub dwCharacteristics: u32,
    pub dwHandle: usize,
    pub wNetType: u16,
    pub dwPrinters: u32,
    pub dwDrives: u32,
}
pub const NETINFO_DISKRED: u32 = 4;
pub const NETINFO_DLL16: u32 = 1;
pub const NETINFO_PRINTERRED: u32 = 8;
pub const NETPROPERTY_PERSISTENT: u32 = 1;
pub type NETRESOURCE = NETRESOURCEA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETRESOURCEA {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: windows_core::PSTR,
    pub lpRemoteName: windows_core::PSTR,
    pub lpComment: windows_core::PSTR,
    pub lpProvider: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETRESOURCEW {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: windows_core::PWSTR,
    pub lpRemoteName: windows_core::PWSTR,
    pub lpComment: windows_core::PWSTR,
    pub lpProvider: windows_core::PWSTR,
}
pub type REMOTE_NAME_INFO = REMOTE_NAME_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REMOTE_NAME_INFOA {
    pub lpUniversalName: windows_core::PSTR,
    pub lpConnectionName: windows_core::PSTR,
    pub lpRemainingPath: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REMOTE_NAME_INFOW {
    pub lpUniversalName: windows_core::PWSTR,
    pub lpConnectionName: windows_core::PWSTR,
    pub lpRemainingPath: windows_core::PWSTR,
}
pub const REMOTE_NAME_INFO_LEVEL: u32 = 2;
pub const RESOURCEDISPLAYTYPE_DIRECTORY: u32 = 9;
pub const RESOURCEDISPLAYTYPE_DOMAIN: u32 = 1;
pub const RESOURCEDISPLAYTYPE_FILE: u32 = 4;
pub const RESOURCEDISPLAYTYPE_GENERIC: u32 = 0;
pub const RESOURCEDISPLAYTYPE_GROUP: u32 = 5;
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: u32 = 11;
pub const RESOURCEDISPLAYTYPE_NETWORK: u32 = 6;
pub const RESOURCEDISPLAYTYPE_ROOT: u32 = 7;
pub const RESOURCEDISPLAYTYPE_SERVER: u32 = 2;
pub const RESOURCEDISPLAYTYPE_SHARE: u32 = 3;
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: u32 = 8;
pub const RESOURCEDISPLAYTYPE_TREE: u32 = 10;
pub const RESOURCETYPE_ANY: u32 = 0;
pub const RESOURCETYPE_DISK: u32 = 1;
pub const RESOURCETYPE_PRINT: u32 = 2;
pub const RESOURCETYPE_RESERVED: u32 = 8;
pub const RESOURCETYPE_UNKNOWN: u32 = 4294967295;
pub const RESOURCEUSAGE_ALL: u32 = 19;
pub const RESOURCEUSAGE_ATTACHED: u32 = 16;
pub const RESOURCEUSAGE_CONNECTABLE: u32 = 1;
pub const RESOURCEUSAGE_CONTAINER: u32 = 2;
pub const RESOURCEUSAGE_NOLOCALDEVICE: u32 = 4;
pub const RESOURCEUSAGE_RESERVED: u32 = 2147483648;
pub const RESOURCEUSAGE_SIBLING: u32 = 8;
pub const RESOURCE_CONNECTED: u32 = 1;
pub const RESOURCE_CONTEXT: u32 = 5;
pub const RESOURCE_GLOBALNET: u32 = 2;
pub const RESOURCE_RECENT: u32 = 4;
pub const RESOURCE_REMEMBERED: u32 = 3;
pub type UNIVERSAL_NAME_INFO = UNIVERSAL_NAME_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNIVERSAL_NAME_INFOA {
    pub lpUniversalName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNIVERSAL_NAME_INFOW {
    pub lpUniversalName: windows_core::PWSTR,
}
pub const UNIVERSAL_NAME_INFO_LEVEL: u32 = 1;
pub const WNCON_DYNAMIC: u32 = 8;
pub const WNCON_FORNETCARD: u32 = 1;
pub const WNCON_NOTROUTED: u32 = 2;
pub const WNCON_SLOWLINK: u32 = 4;
pub const WNFMT_ABBREVIATED: u32 = 2;
pub const WNFMT_CONNECTION: u32 = 32;
pub const WNFMT_INENUM: u32 = 16;
pub const WNFMT_MULTILINE: u32 = 1;
pub const WN_ACCESS_DENIED: u32 = 5;
pub const WN_ALREADY_CONNECTED: u32 = 85;
pub const WN_BAD_DEV_TYPE: u32 = 66;
pub const WN_BAD_HANDLE: u32 = 6;
pub const WN_BAD_LEVEL: u32 = 124;
pub const WN_BAD_LOCALNAME: u32 = 1200;
pub const WN_BAD_NETNAME: u32 = 67;
pub const WN_BAD_PASSWORD: u32 = 86;
pub const WN_BAD_POINTER: u32 = 487;
pub const WN_BAD_PROFILE: u32 = 1206;
pub const WN_BAD_PROVIDER: u32 = 1204;
pub const WN_BAD_USER: u32 = 2202;
pub const WN_BAD_VALUE: u32 = 87;
pub const WN_CANCEL: u32 = 1223;
pub const WN_CANNOT_OPEN_PROFILE: u32 = 1205;
pub const WN_CONNECTED_OTHER_PASSWORD: u32 = 2108;
pub const WN_CONNECTED_OTHER_PASSWORD_DEFAULT: u32 = 2109;
pub const WN_CONNECTION_CLOSED: u32 = 1201;
pub const WN_DEVICE_ALREADY_REMEMBERED: u32 = 1202;
pub const WN_DEVICE_ERROR: u32 = 31;
pub const WN_DEVICE_IN_USE: u32 = 2404;
pub const WN_EXTENDED_ERROR: u32 = 1208;
pub const WN_FUNCTION_BUSY: u32 = 170;
pub const WN_MORE_DATA: u32 = 234;
pub const WN_NET_ERROR: u32 = 59;
pub const WN_NOT_AUTHENTICATED: u32 = 1244;
pub const WN_NOT_CONNECTED: u32 = 2250;
pub const WN_NOT_CONTAINER: u32 = 1207;
pub const WN_NOT_INITIALIZING: u32 = 1247;
pub const WN_NOT_LOGGED_ON: u32 = 1245;
pub const WN_NOT_SUPPORTED: u32 = 50;
pub const WN_NOT_VALIDATED: u32 = 1311;
pub const WN_NO_ERROR: u32 = 0;
pub const WN_NO_MORE_DEVICES: u32 = 1248;
pub const WN_NO_MORE_ENTRIES: u32 = 259;
pub const WN_NO_NETWORK: u32 = 1222;
pub const WN_NO_NET_OR_BAD_PATH: u32 = 1203;
pub const WN_OPEN_FILES: u32 = 2401;
pub const WN_OUT_OF_MEMORY: u32 = 8;
pub const WN_RETRY: u32 = 1237;
pub const WN_SUCCESS: u32 = 0;
pub const WN_WINDOWS_ERROR: u32 = 59;
