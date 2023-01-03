#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[inline]
pub unsafe fn SwDeviceClose<P0>(hswdevice: P0)
where
    P0: ::std::convert::Into<HSWDEVICE>,
{
    ::windows::core::link ! ( "cfgmgr32.dll""system" fn SwDeviceClose ( hswdevice : HSWDEVICE ) -> ( ) );
    SwDeviceClose(hswdevice.into())
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SwDeviceCreate<P0, P1>(pszenumeratorname: P0, pszparentdeviceinstance: P1, pcreateinfo: *const SW_DEVICE_CREATE_INFO, pproperties: ::core::option::Option<&[super::super::Properties::DEVPROPERTY]>, pcallback: SW_DEVICE_CREATE_CALLBACK, pcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<isize>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "cfgmgr32.dll""system" fn SwDeviceCreate ( pszenumeratorname : :: windows::core::PCWSTR , pszparentdeviceinstance : :: windows::core::PCWSTR , pcreateinfo : *const SW_DEVICE_CREATE_INFO , cpropertycount : u32 , pproperties : *const super::super::Properties:: DEVPROPERTY , pcallback : SW_DEVICE_CREATE_CALLBACK , pcontext : *const ::core::ffi::c_void , phswdevice : *mut isize ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SwDeviceCreate(pszenumeratorname.into().abi(), pszparentdeviceinstance.into().abi(), pcreateinfo, pproperties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pproperties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcallback, ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[inline]
pub unsafe fn SwDeviceGetLifetime<P0>(hswdevice: P0) -> ::windows::core::Result<SW_DEVICE_LIFETIME>
where
    P0: ::std::convert::Into<HSWDEVICE>,
{
    ::windows::core::link ! ( "cfgmgr32.dll""system" fn SwDeviceGetLifetime ( hswdevice : HSWDEVICE , plifetime : *mut SW_DEVICE_LIFETIME ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SwDeviceGetLifetime(hswdevice.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDeviceInterfacePropertySet<P0, P1>(hswdevice: P0, pszdeviceinterfaceid: P1, pproperties: &[super::super::Properties::DEVPROPERTY]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HSWDEVICE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "cfgmgr32.dll""system" fn SwDeviceInterfacePropertySet ( hswdevice : HSWDEVICE , pszdeviceinterfaceid : :: windows::core::PCWSTR , cpropertycount : u32 , pproperties : *const super::super::Properties:: DEVPROPERTY ) -> :: windows::core::HRESULT );
    SwDeviceInterfacePropertySet(hswdevice.into(), pszdeviceinterfaceid.into().abi(), pproperties.len() as _, ::core::mem::transmute(pproperties.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SwDeviceInterfaceRegister<P0, P1, P2>(hswdevice: P0, pinterfaceclassguid: *const ::windows::core::GUID, pszreferencestring: P1, pproperties: ::core::option::Option<&[super::super::Properties::DEVPROPERTY]>, fenabled: P2) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<HSWDEVICE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "cfgmgr32.dll""system" fn SwDeviceInterfaceRegister ( hswdevice : HSWDEVICE , pinterfaceclassguid : *const :: windows::core::GUID , pszreferencestring : :: windows::core::PCWSTR , cpropertycount : u32 , pproperties : *const super::super::Properties:: DEVPROPERTY , fenabled : super::super::super::Foundation:: BOOL , ppszdeviceinterfaceid : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SwDeviceInterfaceRegister(hswdevice.into(), pinterfaceclassguid, pszreferencestring.into().abi(), pproperties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pproperties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), fenabled.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwDeviceInterfaceSetState<P0, P1, P2>(hswdevice: P0, pszdeviceinterfaceid: P1, fenabled: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HSWDEVICE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "cfgmgr32.dll""system" fn SwDeviceInterfaceSetState ( hswdevice : HSWDEVICE , pszdeviceinterfaceid : :: windows::core::PCWSTR , fenabled : super::super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    SwDeviceInterfaceSetState(hswdevice.into(), pszdeviceinterfaceid.into().abi(), fenabled.into()).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDevicePropertySet<P0>(hswdevice: P0, pproperties: &[super::super::Properties::DEVPROPERTY]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HSWDEVICE>,
{
    ::windows::core::link ! ( "cfgmgr32.dll""system" fn SwDevicePropertySet ( hswdevice : HSWDEVICE , cpropertycount : u32 , pproperties : *const super::super::Properties:: DEVPROPERTY ) -> :: windows::core::HRESULT );
    SwDevicePropertySet(hswdevice.into(), pproperties.len() as _, ::core::mem::transmute(pproperties.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[inline]
pub unsafe fn SwDeviceSetLifetime<P0>(hswdevice: P0, lifetime: SW_DEVICE_LIFETIME) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HSWDEVICE>,
{
    ::windows::core::link ! ( "cfgmgr32.dll""system" fn SwDeviceSetLifetime ( hswdevice : HSWDEVICE , lifetime : SW_DEVICE_LIFETIME ) -> :: windows::core::HRESULT );
    SwDeviceSetLifetime(hswdevice.into(), lifetime).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[inline]
pub unsafe fn SwMemFree(pmem: *const ::core::ffi::c_void) {
    ::windows::core::link ! ( "cfgmgr32.dll""system" fn SwMemFree ( pmem : *const ::core::ffi::c_void ) -> ( ) );
    SwMemFree(pmem)
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPAddressFamilyControl(::windows::core::IUnknown);
impl IUPnPAddressFamilyControl {
    pub unsafe fn SetAddressFamily(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAddressFamily)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetAddressFamily(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAddressFamily)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUPnPAddressFamilyControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPAddressFamilyControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPAddressFamilyControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPAddressFamilyControl {}
impl ::core::fmt::Debug for IUPnPAddressFamilyControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPAddressFamilyControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPAddressFamilyControl {
    type Vtable = IUPnPAddressFamilyControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPAddressFamilyControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3bf6178_694e_459f_a5a6_191ea0ffa1c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAddressFamilyControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT,
    pub GetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPAsyncResult(::windows::core::IUnknown);
impl IUPnPAsyncResult {
    pub unsafe fn AsyncOperationComplete(&self, ullrequestid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AsyncOperationComplete)(::windows::core::Vtable::as_raw(self), ullrequestid).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPAsyncResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPAsyncResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPAsyncResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPAsyncResult {}
impl ::core::fmt::Debug for IUPnPAsyncResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPAsyncResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPAsyncResult {
    type Vtable = IUPnPAsyncResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPAsyncResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d65fd08_d13e_4274_9c8b_dd8d028c8644);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAsyncResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPDescriptionDocument(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDescriptionDocument {
    pub unsafe fn ReadyState(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReadyState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Load(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn LoadAsync<P0>(&self, bstrurl: &::windows::core::BSTR, punkcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).LoadAsync)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), punkcallback.into().abi()).ok()
    }
    pub unsafe fn LoadResult(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadResult)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Abort)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootDevice(&self) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RootDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceByUDN(&self, bstrudn: &::windows::core::BSTR) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DeviceByUDN)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrudn), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IUPnPDescriptionDocument, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPDescriptionDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUPnPDescriptionDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUPnPDescriptionDocument {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUPnPDescriptionDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDescriptionDocument").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IUPnPDescriptionDocument {
    type Vtable = IUPnPDescriptionDocument_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IUPnPDescriptionDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d1c1b2_7daa_4c9e_9595_7f82ed206d1e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocument_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub ReadyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootDevice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceByUDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: *mut ::core::ffi::c_void, ppuddevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceByUDN: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDescriptionDocumentCallback(::windows::core::IUnknown);
impl IUPnPDescriptionDocumentCallback {
    pub unsafe fn LoadComplete(&self, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).LoadComplete)(::windows::core::Vtable::as_raw(self), hrloadresult).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPDescriptionDocumentCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPDescriptionDocumentCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDescriptionDocumentCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDescriptionDocumentCallback {}
impl ::core::fmt::Debug for IUPnPDescriptionDocumentCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDescriptionDocumentCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPDescriptionDocumentCallback {
    type Vtable = IUPnPDescriptionDocumentCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPDescriptionDocumentCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77394c69_5486_40d6_9bc3_4991983e02da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocumentCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LoadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPDevice(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDevice {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRootDevice(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsRootDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootDevice(&self) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RootDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParentDevice(&self) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ParentDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasChildren(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HasChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Children(&self) -> ::windows::core::Result<IUPnPDevices> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Children)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UniqueDeviceName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UniqueDeviceName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FriendlyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PresentationURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PresentationURL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ManufacturerName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ManufacturerName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ManufacturerURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ManufacturerURL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ModelName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ModelName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ModelNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ModelURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ModelURL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UPC(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UPC)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SerialNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IconURL(&self, bstrencodingformat: &::windows::core::BSTR, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IconURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrencodingformat), lsizex, lsizey, lbitdepth, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Services(&self) -> ::windows::core::Result<IUPnPServices> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Services)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IUPnPDevice, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUPnPDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUPnPDevice {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUPnPDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IUPnPDevice {
    type Vtable = IUPnPDevice_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IUPnPDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d44d0d1_98c9_4889_acd1_f9d674bf2221);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevice_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRootDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRootDevice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RootDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootDevice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ParentDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParentDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChildren: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudchildren: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    pub UniqueDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PresentationURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ManufacturerURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModelURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UPC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IconURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrencodingformat: *mut ::core::ffi::c_void, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppusservices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceControl(::windows::core::IUnknown);
impl IUPnPDeviceControl {
    pub unsafe fn Initialize(&self, bstrxmldesc: &::windows::core::BSTR, bstrdeviceidentifier: &::windows::core::BSTR, bstrinitstring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrxmldesc), ::core::mem::transmute_copy(bstrdeviceidentifier), ::core::mem::transmute_copy(bstrinitstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetServiceObject(&self, bstrudn: &::windows::core::BSTR, bstrserviceid: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetServiceObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrudn), ::core::mem::transmute_copy(bstrserviceid), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUPnPDeviceControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceControl {}
impl ::core::fmt::Debug for IUPnPDeviceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPDeviceControl {
    type Vtable = IUPnPDeviceControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPDeviceControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810ba_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: *mut ::core::ffi::c_void, bstrdeviceidentifier: *mut ::core::ffi::c_void, bstrinitstring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetServiceObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: *mut ::core::ffi::c_void, bstrserviceid: *mut ::core::ffi::c_void, ppdispservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetServiceObject: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceControlHttpHeaders(::windows::core::IUnknown);
impl IUPnPDeviceControlHttpHeaders {
    pub unsafe fn GetAdditionalResponseHeaders(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAdditionalResponseHeaders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUPnPDeviceControlHttpHeaders, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPDeviceControlHttpHeaders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceControlHttpHeaders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceControlHttpHeaders {}
impl ::core::fmt::Debug for IUPnPDeviceControlHttpHeaders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceControlHttpHeaders").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPDeviceControlHttpHeaders {
    type Vtable = IUPnPDeviceControlHttpHeaders_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPDeviceControlHttpHeaders {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810bb_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControlHttpHeaders_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAdditionalResponseHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccess(::windows::core::IUnknown);
impl IUPnPDeviceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentURL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUPnPDeviceDocumentAccess, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPDeviceDocumentAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceDocumentAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceDocumentAccess {}
impl ::core::fmt::Debug for IUPnPDeviceDocumentAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceDocumentAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPDeviceDocumentAccess {
    type Vtable = IUPnPDeviceDocumentAccess_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPDeviceDocumentAccess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7772804_3287_418e_9072_cf2b47238981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccess_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDocumentURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocument: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccessEx(::windows::core::IUnknown);
impl IUPnPDeviceDocumentAccessEx {
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocument)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUPnPDeviceDocumentAccessEx, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPDeviceDocumentAccessEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceDocumentAccessEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceDocumentAccessEx {}
impl ::core::fmt::Debug for IUPnPDeviceDocumentAccessEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceDocumentAccessEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPDeviceDocumentAccessEx {
    type Vtable = IUPnPDeviceDocumentAccessEx_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPDeviceDocumentAccessEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4bc4050_6178_4bd1_a4b8_6398321f3247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccessEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocument: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPDeviceFinder(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinder {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindByType(&self, bstrtypeuri: &::windows::core::BSTR, dwflags: u32) -> ::windows::core::Result<IUPnPDevices> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindByType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtypeuri), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAsyncFind<P0>(&self, bstrtypeuri: &::windows::core::BSTR, dwflags: u32, punkdevicefindercallback: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAsyncFind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtypeuri), dwflags, punkdevicefindercallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StartAsyncFind(&self, lfinddata: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartAsyncFind)(::windows::core::Vtable::as_raw(self), lfinddata).ok()
    }
    pub unsafe fn CancelAsyncFind(&self, lfinddata: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelAsyncFind)(::windows::core::Vtable::as_raw(self), lfinddata).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindByUDN(&self, bstrudn: &::windows::core::BSTR) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindByUDN)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrudn), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IUPnPDeviceFinder, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPDeviceFinder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUPnPDeviceFinder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUPnPDeviceFinder {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUPnPDeviceFinder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceFinder").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IUPnPDeviceFinder {
    type Vtable = IUPnPDeviceFinder_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IUPnPDeviceFinder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadda3d55_6f72_4319_bff9_18600a539b10);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinder_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FindByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypeuri: *mut ::core::ffi::c_void, dwflags: u32, pdevices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindByType: usize,
    pub CreateAsyncFind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypeuri: *mut ::core::ffi::c_void, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows::core::HRESULT,
    pub StartAsyncFind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT,
    pub CancelAsyncFind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FindByUDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: *mut ::core::ffi::c_void, pdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindByUDN: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface(::windows::core::IUnknown);
impl IUPnPDeviceFinderAddCallbackWithInterface {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceAddedWithInterface<P0>(&self, lfinddata: i32, pdevice: P0, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUPnPDevice>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceAddedWithInterface)(::windows::core::Vtable::as_raw(self), lfinddata, pdevice.into().abi(), pguidinterface).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPDeviceFinderAddCallbackWithInterface, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPDeviceFinderAddCallbackWithInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceFinderAddCallbackWithInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceFinderAddCallbackWithInterface {}
impl ::core::fmt::Debug for IUPnPDeviceFinderAddCallbackWithInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceFinderAddCallbackWithInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPDeviceFinderAddCallbackWithInterface {
    type Vtable = IUPnPDeviceFinderAddCallbackWithInterface_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPDeviceFinderAddCallbackWithInterface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x983dfc0b_1796_44df_8975_ca545b620ee5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAddedWithInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAddedWithInterface: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceFinderCallback(::windows::core::IUnknown);
impl IUPnPDeviceFinderCallback {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceAdded<P0>(&self, lfinddata: i32, pdevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUPnPDevice>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceAdded)(::windows::core::Vtable::as_raw(self), lfinddata, pdevice.into().abi()).ok()
    }
    pub unsafe fn DeviceRemoved(&self, lfinddata: i32, bstrudn: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeviceRemoved)(::windows::core::Vtable::as_raw(self), lfinddata, ::core::mem::transmute_copy(bstrudn)).ok()
    }
    pub unsafe fn SearchComplete(&self, lfinddata: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SearchComplete)(::windows::core::Vtable::as_raw(self), lfinddata).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPDeviceFinderCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPDeviceFinderCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceFinderCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceFinderCallback {}
impl ::core::fmt::Debug for IUPnPDeviceFinderCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceFinderCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPDeviceFinderCallback {
    type Vtable = IUPnPDeviceFinderCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPDeviceFinderCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x415a984a_88b3_49f3_92af_0508bedf0d6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAdded: usize,
    pub DeviceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceProvider(::windows::core::IUnknown);
impl IUPnPDeviceProvider {
    pub unsafe fn Start(&self, bstrinitstring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Start)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrinitstring)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPDeviceProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPDeviceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceProvider {}
impl ::core::fmt::Debug for IUPnPDeviceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPDeviceProvider {
    type Vtable = IUPnPDeviceProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPDeviceProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b8_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinitstring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPDevices(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDevices {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, bstrudn: &::windows::core::BSTR) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrudn), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IUPnPDevices, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUPnPDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUPnPDevices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUPnPDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDevices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IUPnPDevices {
    type Vtable = IUPnPDevices_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IUPnPDevices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdbc0c73_bda3_4c66_ac4f_f2d96fdad68c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevices_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPEventSink(::windows::core::IUnknown);
impl IUPnPEventSink {
    pub unsafe fn OnStateChanged(&self, rgdispidchanges: &[i32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnStateChanged)(::windows::core::Vtable::as_raw(self), rgdispidchanges.len() as _, ::core::mem::transmute(rgdispidchanges.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OnStateChangedSafe(&self, varsadispidchanges: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnStateChangedSafe)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varsadispidchanges)).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPEventSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPEventSink {}
impl ::core::fmt::Debug for IUPnPEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPEventSink {
    type Vtable = IUPnPEventSink_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b4_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OnStateChangedSafe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsadispidchanges: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OnStateChangedSafe: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPEventSource(::windows::core::IUnknown);
impl IUPnPEventSource {
    pub unsafe fn Advise<P0>(&self, pessubscriber: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUPnPEventSink>>,
    {
        (::windows::core::Vtable::vtable(self).Advise)(::windows::core::Vtable::as_raw(self), pessubscriber.into().abi()).ok()
    }
    pub unsafe fn Unadvise<P0>(&self, pessubscriber: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUPnPEventSink>>,
    {
        (::windows::core::Vtable::vtable(self).Unadvise)(::windows::core::Vtable::as_raw(self), pessubscriber.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPEventSource, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPEventSource {}
impl ::core::fmt::Debug for IUPnPEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPEventSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPEventSource {
    type Vtable = IUPnPEventSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPEventSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b5_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPHttpHeaderControl(::windows::core::IUnknown);
impl IUPnPHttpHeaderControl {
    pub unsafe fn AddRequestHeaders(&self, bstrhttpheaders: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddRequestHeaders)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrhttpheaders)).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPHttpHeaderControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPHttpHeaderControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPHttpHeaderControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPHttpHeaderControl {}
impl ::core::fmt::Debug for IUPnPHttpHeaderControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPHttpHeaderControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPHttpHeaderControl {
    type Vtable = IUPnPHttpHeaderControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPHttpHeaderControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0405af4f_8b5c_447c_80f2_b75984a31f3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPHttpHeaderControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddRequestHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhttpheaders: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPRegistrar(::windows::core::IUnknown);
impl IUPnPRegistrar {
    pub unsafe fn RegisterDevice(&self, bstrxmldesc: &::windows::core::BSTR, bstrprogiddevicecontrolclass: &::windows::core::BSTR, bstrinitstring: &::windows::core::BSTR, bstrcontainerid: &::windows::core::BSTR, bstrresourcepath: &::windows::core::BSTR, nlifetime: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrxmldesc), ::core::mem::transmute_copy(bstrprogiddevicecontrolclass), ::core::mem::transmute_copy(bstrinitstring), ::core::mem::transmute_copy(bstrcontainerid), ::core::mem::transmute_copy(bstrresourcepath), nlifetime, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterRunningDevice<P0>(&self, bstrxmldesc: &::windows::core::BSTR, punkdevicecontrol: P0, bstrinitstring: &::windows::core::BSTR, bstrresourcepath: &::windows::core::BSTR, nlifetime: i32) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterRunningDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrxmldesc), punkdevicecontrol.into().abi(), ::core::mem::transmute_copy(bstrinitstring), ::core::mem::transmute_copy(bstrresourcepath), nlifetime, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterDeviceProvider(&self, bstrprovidername: &::windows::core::BSTR, bstrprogidproviderclass: &::windows::core::BSTR, bstrinitstring: &::windows::core::BSTR, bstrcontainerid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RegisterDeviceProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprovidername), ::core::mem::transmute_copy(bstrprogidproviderclass), ::core::mem::transmute_copy(bstrinitstring), ::core::mem::transmute_copy(bstrcontainerid)).ok()
    }
    pub unsafe fn GetUniqueDeviceName(&self, bstrdeviceidentifier: &::windows::core::BSTR, bstrtemplateudn: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUniqueDeviceName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdeviceidentifier), ::core::mem::transmute_copy(bstrtemplateudn), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDevice<P0>(&self, bstrdeviceidentifier: &::windows::core::BSTR, fpermanent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).UnregisterDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdeviceidentifier), fpermanent.into()).ok()
    }
    pub unsafe fn UnregisterDeviceProvider(&self, bstrprovidername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterDeviceProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprovidername)).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPRegistrar, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPRegistrar {}
impl ::core::fmt::Debug for IUPnPRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPRegistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPRegistrar {
    type Vtable = IUPnPRegistrar_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPRegistrar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b6_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRegistrar_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: *mut ::core::ffi::c_void, bstrprogiddevicecontrolclass: *mut ::core::ffi::c_void, bstrinitstring: *mut ::core::ffi::c_void, bstrcontainerid: *mut ::core::ffi::c_void, bstrresourcepath: *mut ::core::ffi::c_void, nlifetime: i32, pbstrdeviceidentifier: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterRunningDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: *mut ::core::ffi::c_void, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: *mut ::core::ffi::c_void, bstrresourcepath: *mut ::core::ffi::c_void, nlifetime: i32, pbstrdeviceidentifier: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprovidername: *mut ::core::ffi::c_void, bstrprogidproviderclass: *mut ::core::ffi::c_void, bstrinitstring: *mut ::core::ffi::c_void, bstrcontainerid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUniqueDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: *mut ::core::ffi::c_void, bstrtemplateudn: *mut ::core::ffi::c_void, pbstrudn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: *mut ::core::ffi::c_void, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterDevice: usize,
    pub UnregisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprovidername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPRemoteEndpointInfo(::windows::core::IUnknown);
impl IUPnPRemoteEndpointInfo {
    pub unsafe fn GetDwordValue(&self, bstrvaluename: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDwordValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvaluename), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringValue(&self, bstrvaluename: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStringValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvaluename), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGuidValue(&self, bstrvaluename: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGuidValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvaluename), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUPnPRemoteEndpointInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPRemoteEndpointInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPRemoteEndpointInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPRemoteEndpointInfo {}
impl ::core::fmt::Debug for IUPnPRemoteEndpointInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPRemoteEndpointInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPRemoteEndpointInfo {
    type Vtable = IUPnPRemoteEndpointInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPRemoteEndpointInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc92eb863_0269_4aff_9c72_75321bba2952);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRemoteEndpointInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDwordValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: *mut ::core::ffi::c_void, pdwvalue: *mut u32) -> ::windows::core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: *mut ::core::ffi::c_void, pbstrvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGuidValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: *mut ::core::ffi::c_void, pguidvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPReregistrar(::windows::core::IUnknown);
impl IUPnPReregistrar {
    pub unsafe fn ReregisterDevice(&self, bstrdeviceidentifier: &::windows::core::BSTR, bstrxmldesc: &::windows::core::BSTR, bstrprogiddevicecontrolclass: &::windows::core::BSTR, bstrinitstring: &::windows::core::BSTR, bstrcontainerid: &::windows::core::BSTR, bstrresourcepath: &::windows::core::BSTR, nlifetime: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReregisterDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdeviceidentifier), ::core::mem::transmute_copy(bstrxmldesc), ::core::mem::transmute_copy(bstrprogiddevicecontrolclass), ::core::mem::transmute_copy(bstrinitstring), ::core::mem::transmute_copy(bstrcontainerid), ::core::mem::transmute_copy(bstrresourcepath), nlifetime).ok()
    }
    pub unsafe fn ReregisterRunningDevice<P0>(&self, bstrdeviceidentifier: &::windows::core::BSTR, bstrxmldesc: &::windows::core::BSTR, punkdevicecontrol: P0, bstrinitstring: &::windows::core::BSTR, bstrresourcepath: &::windows::core::BSTR, nlifetime: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).ReregisterRunningDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdeviceidentifier), ::core::mem::transmute_copy(bstrxmldesc), punkdevicecontrol.into().abi(), ::core::mem::transmute_copy(bstrinitstring), ::core::mem::transmute_copy(bstrresourcepath), nlifetime).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPReregistrar, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPReregistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPReregistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPReregistrar {}
impl ::core::fmt::Debug for IUPnPReregistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPReregistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPReregistrar {
    type Vtable = IUPnPReregistrar_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPReregistrar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b7_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPReregistrar_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReregisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: *mut ::core::ffi::c_void, bstrxmldesc: *mut ::core::ffi::c_void, bstrprogiddevicecontrolclass: *mut ::core::ffi::c_void, bstrinitstring: *mut ::core::ffi::c_void, bstrcontainerid: *mut ::core::ffi::c_void, bstrresourcepath: *mut ::core::ffi::c_void, nlifetime: i32) -> ::windows::core::HRESULT,
    pub ReregisterRunningDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: *mut ::core::ffi::c_void, bstrxmldesc: *mut ::core::ffi::c_void, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: *mut ::core::ffi::c_void, bstrresourcepath: *mut ::core::ffi::c_void, nlifetime: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPService(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPService {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QueryStateVariable(&self, bstrvariablename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueryStateVariable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvariablename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InvokeAction(&self, bstractionname: &::windows::core::BSTR, vinactionargs: super::super::super::System::Com::VARIANT, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InvokeAction)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstractionname), ::core::mem::transmute(vinactionargs), pvoutactionargs, pvretval).ok()
    }
    pub unsafe fn ServiceTypeIdentifier(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServiceTypeIdentifier)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddCallback<P0>(&self, punkcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).AddCallback)(::windows::core::Vtable::as_raw(self), punkcallback.into().abi()).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastTransportStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastTransportStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IUPnPService, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUPnPService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUPnPService {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUPnPService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPService").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IUPnPService {
    type Vtable = IUPnPService_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IUPnPService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa295019c_dc65_47dd_90dc_7fe918a1ab44);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPService_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QueryStateVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvariablename: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QueryStateVariable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InvokeAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstractionname: *mut ::core::ffi::c_void, vinactionargs: super::super::super::System::Com::VARIANT, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InvokeAction: usize,
    pub ServiceTypeIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastTransportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPServiceAsync(::windows::core::IUnknown);
impl IUPnPServiceAsync {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginInvokeAction<P0>(&self, bstractionname: &::windows::core::BSTR, vinactionargs: super::super::super::System::Com::VARIANT, pasyncresult: P0) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUPnPAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginInvokeAction)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstractionname), ::core::mem::transmute(vinactionargs), pasyncresult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndInvokeAction)(::windows::core::Vtable::as_raw(self), ullrequestid, pvoutactionargs, pvretval).ok()
    }
    pub unsafe fn BeginQueryStateVariable<P0>(&self, bstrvariablename: &::windows::core::BSTR, pasyncresult: P0) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUPnPAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginQueryStateVariable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvariablename), pasyncresult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndQueryStateVariable)(::windows::core::Vtable::as_raw(self), ullrequestid, pvalue).ok()
    }
    pub unsafe fn BeginSubscribeToEvents<P0, P1>(&self, punkcallback: P0, pasyncresult: P1) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUPnPAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginSubscribeToEvents)(::windows::core::Vtable::as_raw(self), punkcallback.into().abi(), pasyncresult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndSubscribeToEvents(&self, ullrequestid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndSubscribeToEvents)(::windows::core::Vtable::as_raw(self), ullrequestid).ok()
    }
    pub unsafe fn BeginSCPDDownload<P0>(&self, pasyncresult: P0) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUPnPAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginSCPDDownload)(::windows::core::Vtable::as_raw(self), pasyncresult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndSCPDDownload(&self, ullrequestid: u64) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndSCPDDownload)(::windows::core::Vtable::as_raw(self), ullrequestid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CancelAsyncOperation(&self, ullrequestid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelAsyncOperation)(::windows::core::Vtable::as_raw(self), ullrequestid).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPServiceAsync, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPServiceAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceAsync {}
impl ::core::fmt::Debug for IUPnPServiceAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPServiceAsync {
    type Vtable = IUPnPServiceAsync_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPServiceAsync {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x098bdaf5_5ec1_49e7_a260_b3a11dd8680c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceAsync_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginInvokeAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstractionname: *mut ::core::ffi::c_void, vinactionargs: super::super::super::System::Com::VARIANT, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginInvokeAction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EndInvokeAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EndInvokeAction: usize,
    pub BeginQueryStateVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvariablename: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EndQueryStateVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EndQueryStateVariable: usize,
    pub BeginSubscribeToEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows::core::HRESULT,
    pub EndSubscribeToEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT,
    pub BeginSCPDDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows::core::HRESULT,
    pub EndSCPDDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelAsyncOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPServiceCallback(::windows::core::IUnknown);
impl IUPnPServiceCallback {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StateVariableChanged<P0, P1>(&self, pus: P0, pcwszstatevarname: P1, vavalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUPnPService>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).StateVariableChanged)(::windows::core::Vtable::as_raw(self), pus.into().abi(), pcwszstatevarname.into().abi(), ::core::mem::transmute(vavalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ServiceInstanceDied<P0>(&self, pus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUPnPService>>,
    {
        (::windows::core::Vtable::vtable(self).ServiceInstanceDied)(::windows::core::Vtable::as_raw(self), pus.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPServiceCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPServiceCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceCallback {}
impl ::core::fmt::Debug for IUPnPServiceCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPServiceCallback {
    type Vtable = IUPnPServiceCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPServiceCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31fadca9_ab73_464b_b67d_5c1d0f83c8b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StateVariableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void, pcwszstatevarname: ::windows::core::PCWSTR, vavalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StateVariableChanged: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ServiceInstanceDied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServiceInstanceDied: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPServiceDocumentAccess(::windows::core::IUnknown);
impl IUPnPServiceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentURL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocument)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUPnPServiceDocumentAccess, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPServiceDocumentAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceDocumentAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceDocumentAccess {}
impl ::core::fmt::Debug for IUPnPServiceDocumentAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceDocumentAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPServiceDocumentAccess {
    type Vtable = IUPnPServiceDocumentAccess_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPServiceDocumentAccess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21905529_0a5e_4589_825d_7e6d87ea6998);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceDocumentAccess_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDocumentURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdoc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPServiceEnumProperty(::windows::core::IUnknown);
impl IUPnPServiceEnumProperty {
    pub unsafe fn SetServiceEnumProperty(&self, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetServiceEnumProperty)(::windows::core::Vtable::as_raw(self), dwmask).ok()
    }
}
::windows::core::interface_hierarchy!(IUPnPServiceEnumProperty, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUPnPServiceEnumProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceEnumProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceEnumProperty {}
impl ::core::fmt::Debug for IUPnPServiceEnumProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceEnumProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUPnPServiceEnumProperty {
    type Vtable = IUPnPServiceEnumProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for IUPnPServiceEnumProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38873b37_91bb_49f4_b249_2e8efbb8a816);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceEnumProperty_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetServiceEnumProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPServices(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPServices {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, bstrserviceid: &::windows::core::BSTR) -> ::windows::core::Result<IUPnPService> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrserviceid), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IUPnPServices, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUPnPServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUPnPServices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUPnPServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IUPnPServices {
    type Vtable = IUPnPServices_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IUPnPServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f8c8e9e_9a7a_4dc8_bc41_ff31fa374956);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServices_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserviceid: *mut ::core::ffi::c_void, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const ADDRESS_FAMILY_VALUE_NAME: ::windows::core::PCWSTR = ::windows::w!("AddressFamily");
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_INVALID_ACTION: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_INVALID_ARG: u32 = 402u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const REMOTE_ADDRESS_VALUE_NAME: ::windows::core::PCWSTR = ::windows::w!("RemoteAddress");
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ACTION_REQUEST_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220976i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ACTION_SPECIFIC_BASE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220736i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220972i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220988i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_NOTREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180494i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180495i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220969i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180511i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DUPLICATE_SERVICE_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180510i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220970i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220223i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ICON_ELEMENT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220987i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ICON_NODE_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220986i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ACTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220985i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ARGUMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220984i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_DESCRIPTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180509i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220224i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ICON: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180507i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ROOT_NAMESPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180505i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_SERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180508i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_VARIABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220973i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180506i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_OUT_OF_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220983i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_PROTOCOL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220971i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180512i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220989i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_SUFFIX_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180504i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_TRANSPORT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220975i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_URLBASE_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180503i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_VALUE_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180496i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220974i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_SERVICE_DELAY_SCPD_AND_SUBSCRIPTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDescriptionDocument: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d8a9b47_3a28_4ce2_8a4b_bd34e45bceeb);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDescriptionDocumentEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33fd0563_d81a_4393_83cc_0195b1da2f91);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDevice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa32552c5_ba61_457a_b59a_a2561e125e33);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDeviceFinder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2085f28_feb7_404a_b8e7_e659bdeaaa02);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDeviceFinderEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x181b54fc_380b_4a75_b3f1_4ac45e9605b0);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDevices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9e84ffd_ad3c_40a4_b835_0882ebcbaaa8);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPRegistrar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b9_73b2_11d4_bf42_00b0d0118b56);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPRemoteEndpointInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e5e84e9_4049_4244_b728_2d24227157c7);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc624ba95_fbcb_4409_8c03_8cceec533ef1);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0bc4b4a_a406_4efc_932f_b8546b8100cc);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SW_DEVICE_CAPABILITIES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(0i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(1i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(2i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(4i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(8i32);
impl ::core::marker::Copy for SW_DEVICE_CAPABILITIES {}
impl ::core::clone::Clone for SW_DEVICE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SW_DEVICE_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SW_DEVICE_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for SW_DEVICE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SW_DEVICE_CAPABILITIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SW_DEVICE_LIFETIME(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(0i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(1i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(2i32);
impl ::core::marker::Copy for SW_DEVICE_LIFETIME {}
impl ::core::clone::Clone for SW_DEVICE_LIFETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SW_DEVICE_LIFETIME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SW_DEVICE_LIFETIME {
    type Abi = Self;
}
impl ::core::fmt::Debug for SW_DEVICE_LIFETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SW_DEVICE_LIFETIME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HSWDEVICE(pub isize);
impl HSWDEVICE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HSWDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSWDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSWDEVICE {}
impl ::core::fmt::Debug for HSWDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSWDEVICE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HSWDEVICE>> for HSWDEVICE {
    fn from(optional: ::core::option::Option<HSWDEVICE>) -> HSWDEVICE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HSWDEVICE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SW_DEVICE_CREATE_INFO {
    pub cbSize: u32,
    pub pszInstanceId: ::windows::core::PCWSTR,
    pub pszzHardwareIds: ::windows::core::PCWSTR,
    pub pszzCompatibleIds: ::windows::core::PCWSTR,
    pub pContainerId: *const ::windows::core::GUID,
    pub CapabilityFlags: u32,
    pub pszDeviceDescription: ::windows::core::PCWSTR,
    pub pszDeviceLocation: ::windows::core::PCWSTR,
    pub pSecurityDescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SW_DEVICE_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SW_DEVICE_CREATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for SW_DEVICE_CREATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SW_DEVICE_CREATE_INFO").field("cbSize", &self.cbSize).field("pszInstanceId", &self.pszInstanceId).field("pszzHardwareIds", &self.pszzHardwareIds).field("pszzCompatibleIds", &self.pszzCompatibleIds).field("pContainerId", &self.pContainerId).field("CapabilityFlags", &self.CapabilityFlags).field("pszDeviceDescription", &self.pszDeviceDescription).field("pszDeviceLocation", &self.pszDeviceLocation).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for SW_DEVICE_CREATE_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for SW_DEVICE_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszInstanceId == other.pszInstanceId && self.pszzHardwareIds == other.pszzHardwareIds && self.pszzCompatibleIds == other.pszzCompatibleIds && self.pContainerId == other.pContainerId && self.CapabilityFlags == other.CapabilityFlags && self.pszDeviceDescription == other.pszDeviceDescription && self.pszDeviceLocation == other.pszDeviceLocation && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for SW_DEVICE_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for SW_DEVICE_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub type SW_DEVICE_CREATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: ::windows::core::HRESULT, pcontext: *const ::core::ffi::c_void, pszdeviceinstanceid: ::windows::core::PCWSTR) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
