#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[inline]
pub unsafe fn SwDeviceClose<P0>(hswdevice: P0)
where
    P0: ::windows_core::IntoParam<HSWDEVICE>,
{
    ::windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceClose(hswdevice : HSWDEVICE) -> ());
    SwDeviceClose(hswdevice.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SwDeviceCreate<P0, P1>(pszenumeratorname: P0, pszparentdeviceinstance: P1, pcreateinfo: *const SW_DEVICE_CREATE_INFO, pproperties: ::core::option::Option<&[super::super::Properties::DEVPROPERTY]>, pcallback: SW_DEVICE_CREATE_CALLBACK, pcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<HSWDEVICE>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceCreate(pszenumeratorname : ::windows_core::PCWSTR, pszparentdeviceinstance : ::windows_core::PCWSTR, pcreateinfo : *const SW_DEVICE_CREATE_INFO, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY, pcallback : SW_DEVICE_CREATE_CALLBACK, pcontext : *const ::core::ffi::c_void, phswdevice : *mut HSWDEVICE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    SwDeviceCreate(pszenumeratorname.into_param().abi(), pszparentdeviceinstance.into_param().abi(), pcreateinfo, pproperties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pproperties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcallback, ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[inline]
pub unsafe fn SwDeviceGetLifetime<P0>(hswdevice: P0) -> ::windows_core::Result<SW_DEVICE_LIFETIME>
where
    P0: ::windows_core::IntoParam<HSWDEVICE>,
{
    ::windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceGetLifetime(hswdevice : HSWDEVICE, plifetime : *mut SW_DEVICE_LIFETIME) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    SwDeviceGetLifetime(hswdevice.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDeviceInterfacePropertySet<P0, P1>(hswdevice: P0, pszdeviceinterfaceid: P1, pproperties: &[super::super::Properties::DEVPROPERTY]) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HSWDEVICE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceInterfacePropertySet(hswdevice : HSWDEVICE, pszdeviceinterfaceid : ::windows_core::PCWSTR, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY) -> ::windows_core::HRESULT);
    SwDeviceInterfacePropertySet(hswdevice.into_param().abi(), pszdeviceinterfaceid.into_param().abi(), pproperties.len() as _, ::core::mem::transmute(pproperties.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SwDeviceInterfaceRegister<P0, P1, P2>(hswdevice: P0, pinterfaceclassguid: *const ::windows_core::GUID, pszreferencestring: P1, pproperties: ::core::option::Option<&[super::super::Properties::DEVPROPERTY]>, fenabled: P2) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<HSWDEVICE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceInterfaceRegister(hswdevice : HSWDEVICE, pinterfaceclassguid : *const ::windows_core::GUID, pszreferencestring : ::windows_core::PCWSTR, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY, fenabled : super::super::super::Foundation:: BOOL, ppszdeviceinterfaceid : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    SwDeviceInterfaceRegister(hswdevice.into_param().abi(), pinterfaceclassguid, pszreferencestring.into_param().abi(), pproperties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pproperties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), fenabled.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwDeviceInterfaceSetState<P0, P1, P2>(hswdevice: P0, pszdeviceinterfaceid: P1, fenabled: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HSWDEVICE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceInterfaceSetState(hswdevice : HSWDEVICE, pszdeviceinterfaceid : ::windows_core::PCWSTR, fenabled : super::super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    SwDeviceInterfaceSetState(hswdevice.into_param().abi(), pszdeviceinterfaceid.into_param().abi(), fenabled.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDevicePropertySet<P0>(hswdevice: P0, pproperties: &[super::super::Properties::DEVPROPERTY]) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HSWDEVICE>,
{
    ::windows_targets::link!("cfgmgr32.dll" "system" fn SwDevicePropertySet(hswdevice : HSWDEVICE, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY) -> ::windows_core::HRESULT);
    SwDevicePropertySet(hswdevice.into_param().abi(), pproperties.len() as _, ::core::mem::transmute(pproperties.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[inline]
pub unsafe fn SwDeviceSetLifetime<P0>(hswdevice: P0, lifetime: SW_DEVICE_LIFETIME) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HSWDEVICE>,
{
    ::windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceSetLifetime(hswdevice : HSWDEVICE, lifetime : SW_DEVICE_LIFETIME) -> ::windows_core::HRESULT);
    SwDeviceSetLifetime(hswdevice.into_param().abi(), lifetime).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[inline]
pub unsafe fn SwMemFree(pmem: *const ::core::ffi::c_void) {
    ::windows_targets::link!("cfgmgr32.dll" "system" fn SwMemFree(pmem : *const ::core::ffi::c_void) -> ());
    SwMemFree(pmem)
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPAddressFamilyControl(::windows_core::IUnknown);
impl IUPnPAddressFamilyControl {
    pub unsafe fn SetAddressFamily(&self, dwflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAddressFamily)(::windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetAddressFamily(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAddressFamily)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPAddressFamilyControl, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPAddressFamilyControl {
    type Vtable = IUPnPAddressFamilyControl_Vtbl;
}
impl ::core::clone::Clone for IUPnPAddressFamilyControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPAddressFamilyControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3bf6178_694e_459f_a5a6_191ea0ffa1c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAddressFamilyControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows_core::HRESULT,
    pub GetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPAsyncResult(::windows_core::IUnknown);
impl IUPnPAsyncResult {
    pub unsafe fn AsyncOperationComplete(&self, ullrequestid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncOperationComplete)(::windows_core::Interface::as_raw(self), ullrequestid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPAsyncResult, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPAsyncResult {
    type Vtable = IUPnPAsyncResult_Vtbl;
}
impl ::core::clone::Clone for IUPnPAsyncResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPAsyncResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d65fd08_d13e_4274_9c8b_dd8d028c8644);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAsyncResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPDescriptionDocument(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDescriptionDocument {
    pub unsafe fn ReadyState(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReadyState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Load<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn LoadAsync<P0, P1>(&self, bstrurl: P0, punkcallback: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).LoadAsync)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi(), punkcallback.into_param().abi()).ok()
    }
    pub unsafe fn LoadResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoadResult)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootDevice(&self) -> ::windows_core::Result<IUPnPDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RootDevice)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceByUDN<P0>(&self, bstrudn: P0) -> ::windows_core::Result<IUPnPDevice>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceByUDN)(::windows_core::Interface::as_raw(self), bstrudn.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IUPnPDescriptionDocument, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
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
unsafe impl ::windows_core::Interface for IUPnPDescriptionDocument {
    type Vtable = IUPnPDescriptionDocument_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPDescriptionDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IUPnPDescriptionDocument {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11d1c1b2_7daa_4c9e_9595_7f82ed206d1e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocument_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub ReadyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LoadResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootDevice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceByUDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppuddevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceByUDN: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDescriptionDocumentCallback(::windows_core::IUnknown);
impl IUPnPDescriptionDocumentCallback {
    pub unsafe fn LoadComplete(&self, hrloadresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadComplete)(::windows_core::Interface::as_raw(self), hrloadresult).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPDescriptionDocumentCallback, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPDescriptionDocumentCallback {
    type Vtable = IUPnPDescriptionDocumentCallback_Vtbl;
}
impl ::core::clone::Clone for IUPnPDescriptionDocumentCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPDescriptionDocumentCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77394c69_5486_40d6_9bc3_4991983e02da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocumentCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub LoadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrloadresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPDevice(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDevice {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRootDevice(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRootDevice)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootDevice(&self) -> ::windows_core::Result<IUPnPDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RootDevice)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParentDevice(&self) -> ::windows_core::Result<IUPnPDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ParentDevice)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasChildren(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HasChildren)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Children(&self) -> ::windows_core::Result<IUPnPDevices> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Children)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UniqueDeviceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UniqueDeviceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FriendlyName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PresentationURL(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PresentationURL)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ManufacturerName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ManufacturerName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ManufacturerURL(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ManufacturerURL)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ModelName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ModelName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ModelNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ModelNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ModelURL(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ModelURL)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UPC(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UPC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SerialNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SerialNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IconURL<P0>(&self, bstrencodingformat: P0, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IconURL)(::windows_core::Interface::as_raw(self), bstrencodingformat.into_param().abi(), lsizex, lsizey, lbitdepth, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Services(&self) -> ::windows_core::Result<IUPnPServices> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Services)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IUPnPDevice, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
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
unsafe impl ::windows_core::Interface for IUPnPDevice {
    type Vtable = IUPnPDevice_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IUPnPDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d44d0d1_98c9_4889_acd1_f9d674bf2221);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevice_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRootDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRootDevice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RootDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootDevice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ParentDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParentDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChildren: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudchildren: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    pub UniqueDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PresentationURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ManufacturerURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ModelURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub UPC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IconURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrencodingformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppusservices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceControl(::windows_core::IUnknown);
impl IUPnPDeviceControl {
    pub unsafe fn Initialize<P0, P1, P2>(&self, bstrxmldesc: P0, bstrdeviceidentifier: P1, bstrinitstring: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), bstrxmldesc.into_param().abi(), bstrdeviceidentifier.into_param().abi(), bstrinitstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetServiceObject<P0, P1>(&self, bstrudn: P0, bstrserviceid: P1) -> ::windows_core::Result<super::super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceObject)(::windows_core::Interface::as_raw(self), bstrudn.into_param().abi(), bstrserviceid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPDeviceControl, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPDeviceControl {
    type Vtable = IUPnPDeviceControl_Vtbl;
}
impl ::core::clone::Clone for IUPnPDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPDeviceControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810ba_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetServiceObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrserviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdispservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetServiceObject: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceControlHttpHeaders(::windows_core::IUnknown);
impl IUPnPDeviceControlHttpHeaders {
    pub unsafe fn GetAdditionalResponseHeaders(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAdditionalResponseHeaders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPDeviceControlHttpHeaders, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPDeviceControlHttpHeaders {
    type Vtable = IUPnPDeviceControlHttpHeaders_Vtbl;
}
impl ::core::clone::Clone for IUPnPDeviceControlHttpHeaders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPDeviceControlHttpHeaders {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810bb_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControlHttpHeaders_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetAdditionalResponseHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccess(::windows_core::IUnknown);
impl IUPnPDeviceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentURL)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPDeviceDocumentAccess, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPDeviceDocumentAccess {
    type Vtable = IUPnPDeviceDocumentAccess_Vtbl;
}
impl ::core::clone::Clone for IUPnPDeviceDocumentAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPDeviceDocumentAccess {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7772804_3287_418e_9072_cf2b47238981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccess_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDocumentURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocument: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccessEx(::windows_core::IUnknown);
impl IUPnPDeviceDocumentAccessEx {
    pub unsafe fn GetDocument(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocument)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPDeviceDocumentAccessEx, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPDeviceDocumentAccessEx {
    type Vtable = IUPnPDeviceDocumentAccessEx_Vtbl;
}
impl ::core::clone::Clone for IUPnPDeviceDocumentAccessEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPDeviceDocumentAccessEx {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4bc4050_6178_4bd1_a4b8_6398321f3247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccessEx_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocument: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPDeviceFinder(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinder {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindByType<P0>(&self, bstrtypeuri: P0, dwflags: u32) -> ::windows_core::Result<IUPnPDevices>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindByType)(::windows_core::Interface::as_raw(self), bstrtypeuri.into_param().abi(), dwflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAsyncFind<P0, P1>(&self, bstrtypeuri: P0, dwflags: u32, punkdevicefindercallback: P1) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateAsyncFind)(::windows_core::Interface::as_raw(self), bstrtypeuri.into_param().abi(), dwflags, punkdevicefindercallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartAsyncFind(&self, lfinddata: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartAsyncFind)(::windows_core::Interface::as_raw(self), lfinddata).ok()
    }
    pub unsafe fn CancelAsyncFind(&self, lfinddata: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAsyncFind)(::windows_core::Interface::as_raw(self), lfinddata).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindByUDN<P0>(&self, bstrudn: P0) -> ::windows_core::Result<IUPnPDevice>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindByUDN)(::windows_core::Interface::as_raw(self), bstrudn.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IUPnPDeviceFinder, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
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
unsafe impl ::windows_core::Interface for IUPnPDeviceFinder {
    type Vtable = IUPnPDeviceFinder_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPDeviceFinder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IUPnPDeviceFinder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadda3d55_6f72_4319_bff9_18600a539b10);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinder_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FindByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypeuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: u32, pdevices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindByType: usize,
    pub CreateAsyncFind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypeuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows_core::HRESULT,
    pub StartAsyncFind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT,
    pub CancelAsyncFind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FindByUDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindByUDN: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface(::windows_core::IUnknown);
impl IUPnPDeviceFinderAddCallbackWithInterface {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceAddedWithInterface<P0>(&self, lfinddata: i32, pdevice: P0, pguidinterface: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUPnPDevice>,
    {
        (::windows_core::Interface::vtable(self).DeviceAddedWithInterface)(::windows_core::Interface::as_raw(self), lfinddata, pdevice.into_param().abi(), pguidinterface).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPDeviceFinderAddCallbackWithInterface, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPDeviceFinderAddCallbackWithInterface {
    type Vtable = IUPnPDeviceFinderAddCallbackWithInterface_Vtbl;
}
impl ::core::clone::Clone for IUPnPDeviceFinderAddCallbackWithInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPDeviceFinderAddCallbackWithInterface {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x983dfc0b_1796_44df_8975_ca545b620ee5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAddedWithInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void, pguidinterface: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAddedWithInterface: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceFinderCallback(::windows_core::IUnknown);
impl IUPnPDeviceFinderCallback {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceAdded<P0>(&self, lfinddata: i32, pdevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUPnPDevice>,
    {
        (::windows_core::Interface::vtable(self).DeviceAdded)(::windows_core::Interface::as_raw(self), lfinddata, pdevice.into_param().abi()).ok()
    }
    pub unsafe fn DeviceRemoved<P0>(&self, lfinddata: i32, bstrudn: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeviceRemoved)(::windows_core::Interface::as_raw(self), lfinddata, bstrudn.into_param().abi()).ok()
    }
    pub unsafe fn SearchComplete(&self, lfinddata: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SearchComplete)(::windows_core::Interface::as_raw(self), lfinddata).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPDeviceFinderCallback, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPDeviceFinderCallback {
    type Vtable = IUPnPDeviceFinderCallback_Vtbl;
}
impl ::core::clone::Clone for IUPnPDeviceFinderCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPDeviceFinderCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x415a984a_88b3_49f3_92af_0508bedf0d6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAdded: usize,
    pub DeviceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPDeviceProvider(::windows_core::IUnknown);
impl IUPnPDeviceProvider {
    pub unsafe fn Start<P0>(&self, bstrinitstring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), bstrinitstring.into_param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPDeviceProvider, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPDeviceProvider {
    type Vtable = IUPnPDeviceProvider_Vtbl;
}
impl ::core::clone::Clone for IUPnPDeviceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPDeviceProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b8_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPDevices(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDevices {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, bstrudn: P0) -> ::windows_core::Result<IUPnPDevice>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), bstrudn.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IUPnPDevices, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
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
unsafe impl ::windows_core::Interface for IUPnPDevices {
    type Vtable = IUPnPDevices_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IUPnPDevices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdbc0c73_bda3_4c66_ac4f_f2d96fdad68c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevices_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPEventSink(::windows_core::IUnknown);
impl IUPnPEventSink {
    pub unsafe fn OnStateChanged(&self, rgdispidchanges: &[i32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStateChanged)(::windows_core::Interface::as_raw(self), rgdispidchanges.len() as _, ::core::mem::transmute(rgdispidchanges.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OnStateChangedSafe(&self, varsadispidchanges: super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStateChangedSafe)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varsadispidchanges)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPEventSink, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPEventSink {
    type Vtable = IUPnPEventSink_Vtbl;
}
impl ::core::clone::Clone for IUPnPEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPEventSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b4_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OnStateChangedSafe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsadispidchanges: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OnStateChangedSafe: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPEventSource(::windows_core::IUnknown);
impl IUPnPEventSource {
    pub unsafe fn Advise<P0>(&self, pessubscriber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUPnPEventSink>,
    {
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), pessubscriber.into_param().abi()).ok()
    }
    pub unsafe fn Unadvise<P0>(&self, pessubscriber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUPnPEventSink>,
    {
        (::windows_core::Interface::vtable(self).Unadvise)(::windows_core::Interface::as_raw(self), pessubscriber.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPEventSource, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPEventSource {
    type Vtable = IUPnPEventSource_Vtbl;
}
impl ::core::clone::Clone for IUPnPEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPEventSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b5_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSource_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPHttpHeaderControl(::windows_core::IUnknown);
impl IUPnPHttpHeaderControl {
    pub unsafe fn AddRequestHeaders<P0>(&self, bstrhttpheaders: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).AddRequestHeaders)(::windows_core::Interface::as_raw(self), bstrhttpheaders.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPHttpHeaderControl, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPHttpHeaderControl {
    type Vtable = IUPnPHttpHeaderControl_Vtbl;
}
impl ::core::clone::Clone for IUPnPHttpHeaderControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPHttpHeaderControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0405af4f_8b5c_447c_80f2_b75984a31f3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPHttpHeaderControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddRequestHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhttpheaders: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPRegistrar(::windows_core::IUnknown);
impl IUPnPRegistrar {
    pub unsafe fn RegisterDevice<P0, P1, P2, P3, P4>(&self, bstrxmldesc: P0, bstrprogiddevicecontrolclass: P1, bstrinitstring: P2, bstrcontainerid: P3, bstrresourcepath: P4, nlifetime: i32) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterDevice)(::windows_core::Interface::as_raw(self), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), nlifetime, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterRunningDevice<P0, P1, P2, P3>(&self, bstrxmldesc: P0, punkdevicecontrol: P1, bstrinitstring: P2, bstrresourcepath: P3, nlifetime: i32) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterRunningDevice)(::windows_core::Interface::as_raw(self), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), nlifetime, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterDeviceProvider<P0, P1, P2, P3>(&self, bstrprovidername: P0, bstrprogidproviderclass: P1, bstrinitstring: P2, bstrcontainerid: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterDeviceProvider)(::windows_core::Interface::as_raw(self), bstrprovidername.into_param().abi(), bstrprogidproviderclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi()).ok()
    }
    pub unsafe fn GetUniqueDeviceName<P0, P1>(&self, bstrdeviceidentifier: P0, bstrtemplateudn: P1) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUniqueDeviceName)(::windows_core::Interface::as_raw(self), bstrdeviceidentifier.into_param().abi(), bstrtemplateudn.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDevice<P0, P1>(&self, bstrdeviceidentifier: P0, fpermanent: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).UnregisterDevice)(::windows_core::Interface::as_raw(self), bstrdeviceidentifier.into_param().abi(), fpermanent.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterDeviceProvider<P0>(&self, bstrprovidername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterDeviceProvider)(::windows_core::Interface::as_raw(self), bstrprovidername.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPRegistrar, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPRegistrar {
    type Vtable = IUPnPRegistrar_Vtbl;
}
impl ::core::clone::Clone for IUPnPRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPRegistrar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b6_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRegistrar_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RegisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogiddevicecontrolclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcontainerid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RegisterRunningDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RegisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogidproviderclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcontainerid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetUniqueDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtemplateudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrudn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterDevice: usize,
    pub UnregisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPRemoteEndpointInfo(::windows_core::IUnknown);
impl IUPnPRemoteEndpointInfo {
    pub unsafe fn GetDwordValue<P0>(&self, bstrvaluename: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDwordValue)(::windows_core::Interface::as_raw(self), bstrvaluename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStringValue<P0>(&self, bstrvaluename: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStringValue)(::windows_core::Interface::as_raw(self), bstrvaluename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGuidValue<P0>(&self, bstrvaluename: P0) -> ::windows_core::Result<::windows_core::GUID>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGuidValue)(::windows_core::Interface::as_raw(self), bstrvaluename.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPRemoteEndpointInfo, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPRemoteEndpointInfo {
    type Vtable = IUPnPRemoteEndpointInfo_Vtbl;
}
impl ::core::clone::Clone for IUPnPRemoteEndpointInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPRemoteEndpointInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc92eb863_0269_4aff_9c72_75321bba2952);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRemoteEndpointInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDwordValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwvalue: *mut u32) -> ::windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetGuidValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pguidvalue: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPReregistrar(::windows_core::IUnknown);
impl IUPnPReregistrar {
    pub unsafe fn ReregisterDevice<P0, P1, P2, P3, P4, P5>(&self, bstrdeviceidentifier: P0, bstrxmldesc: P1, bstrprogiddevicecontrolclass: P2, bstrinitstring: P3, bstrcontainerid: P4, bstrresourcepath: P5, nlifetime: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
        P5: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ReregisterDevice)(::windows_core::Interface::as_raw(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), nlifetime).ok()
    }
    pub unsafe fn ReregisterRunningDevice<P0, P1, P2, P3, P4>(&self, bstrdeviceidentifier: P0, bstrxmldesc: P1, punkdevicecontrol: P2, bstrinitstring: P3, bstrresourcepath: P4, nlifetime: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ReregisterRunningDevice)(::windows_core::Interface::as_raw(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), nlifetime).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPReregistrar, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPReregistrar {
    type Vtable = IUPnPReregistrar_Vtbl;
}
impl ::core::clone::Clone for IUPnPReregistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPReregistrar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b7_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPReregistrar_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ReregisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogiddevicecontrolclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcontainerid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32) -> ::windows_core::HRESULT,
    pub ReregisterRunningDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPService(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPService {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn QueryStateVariable<P0>(&self, bstrvariablename: P0) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryStateVariable)(::windows_core::Interface::as_raw(self), bstrvariablename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn InvokeAction<P0>(&self, bstractionname: P0, vinactionargs: super::super::super::System::Variant::VARIANT, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).InvokeAction)(::windows_core::Interface::as_raw(self), bstractionname.into_param().abi(), ::core::mem::transmute(vinactionargs), pvoutactionargs, pvretval).ok()
    }
    pub unsafe fn ServiceTypeIdentifier(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServiceTypeIdentifier)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddCallback<P0>(&self, punkcallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).AddCallback)(::windows_core::Interface::as_raw(self), punkcallback.into_param().abi()).ok()
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastTransportStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastTransportStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IUPnPService, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
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
unsafe impl ::windows_core::Interface for IUPnPService {
    type Vtable = IUPnPService_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IUPnPService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa295019c_dc65_47dd_90dc_7fe918a1ab44);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPService_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub QueryStateVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvariablename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    QueryStateVariable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub InvokeAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstractionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vinactionargs: super::super::super::System::Variant::VARIANT, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    InvokeAction: usize,
    pub ServiceTypeIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AddCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LastTransportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPServiceAsync(::windows_core::IUnknown);
impl IUPnPServiceAsync {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn BeginInvokeAction<P0, P1>(&self, bstractionname: P0, vinactionargs: super::super::super::System::Variant::VARIANT, pasyncresult: P1) -> ::windows_core::Result<u64>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<IUPnPAsyncResult>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginInvokeAction)(::windows_core::Interface::as_raw(self), bstractionname.into_param().abi(), ::core::mem::transmute(vinactionargs), pasyncresult.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndInvokeAction)(::windows_core::Interface::as_raw(self), ullrequestid, pvoutactionargs, pvretval).ok()
    }
    pub unsafe fn BeginQueryStateVariable<P0, P1>(&self, bstrvariablename: P0, pasyncresult: P1) -> ::windows_core::Result<u64>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<IUPnPAsyncResult>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginQueryStateVariable)(::windows_core::Interface::as_raw(self), bstrvariablename.into_param().abi(), pasyncresult.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndQueryStateVariable)(::windows_core::Interface::as_raw(self), ullrequestid, pvalue).ok()
    }
    pub unsafe fn BeginSubscribeToEvents<P0, P1>(&self, punkcallback: P0, pasyncresult: P1) -> ::windows_core::Result<u64>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IUPnPAsyncResult>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginSubscribeToEvents)(::windows_core::Interface::as_raw(self), punkcallback.into_param().abi(), pasyncresult.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndSubscribeToEvents(&self, ullrequestid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndSubscribeToEvents)(::windows_core::Interface::as_raw(self), ullrequestid).ok()
    }
    pub unsafe fn BeginSCPDDownload<P0>(&self, pasyncresult: P0) -> ::windows_core::Result<u64>
    where
        P0: ::windows_core::IntoParam<IUPnPAsyncResult>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginSCPDDownload)(::windows_core::Interface::as_raw(self), pasyncresult.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndSCPDDownload(&self, ullrequestid: u64) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndSCPDDownload)(::windows_core::Interface::as_raw(self), ullrequestid, &mut result__).from_abi(result__)
    }
    pub unsafe fn CancelAsyncOperation(&self, ullrequestid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAsyncOperation)(::windows_core::Interface::as_raw(self), ullrequestid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPServiceAsync, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPServiceAsync {
    type Vtable = IUPnPServiceAsync_Vtbl;
}
impl ::core::clone::Clone for IUPnPServiceAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPServiceAsync {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x098bdaf5_5ec1_49e7_a260_b3a11dd8680c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceAsync_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub BeginInvokeAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstractionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vinactionargs: super::super::super::System::Variant::VARIANT, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    BeginInvokeAction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EndInvokeAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EndInvokeAction: usize,
    pub BeginQueryStateVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvariablename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EndQueryStateVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EndQueryStateVariable: usize,
    pub BeginSubscribeToEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT,
    pub EndSubscribeToEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT,
    pub BeginSCPDDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT,
    pub EndSCPDDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CancelAsyncOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPServiceCallback(::windows_core::IUnknown);
impl IUPnPServiceCallback {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn StateVariableChanged<P0, P1>(&self, pus: P0, pcwszstatevarname: P1, vavalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUPnPService>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).StateVariableChanged)(::windows_core::Interface::as_raw(self), pus.into_param().abi(), pcwszstatevarname.into_param().abi(), ::core::mem::transmute(vavalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ServiceInstanceDied<P0>(&self, pus: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUPnPService>,
    {
        (::windows_core::Interface::vtable(self).ServiceInstanceDied)(::windows_core::Interface::as_raw(self), pus.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPServiceCallback, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPServiceCallback {
    type Vtable = IUPnPServiceCallback_Vtbl;
}
impl ::core::clone::Clone for IUPnPServiceCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPServiceCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31fadca9_ab73_464b_b67d_5c1d0f83c8b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub StateVariableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void, pcwszstatevarname: ::windows_core::PCWSTR, vavalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    StateVariableChanged: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ServiceInstanceDied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServiceInstanceDied: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPServiceDocumentAccess(::windows_core::IUnknown);
impl IUPnPServiceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentURL)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocument(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocument)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPServiceDocumentAccess, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPServiceDocumentAccess {
    type Vtable = IUPnPServiceDocumentAccess_Vtbl;
}
impl ::core::clone::Clone for IUPnPServiceDocumentAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPServiceDocumentAccess {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21905529_0a5e_4589_825d_7e6d87ea6998);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceDocumentAccess_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDocumentURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdoc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct IUPnPServiceEnumProperty(::windows_core::IUnknown);
impl IUPnPServiceEnumProperty {
    pub unsafe fn SetServiceEnumProperty(&self, dwmask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceEnumProperty)(::windows_core::Interface::as_raw(self), dwmask).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUPnPServiceEnumProperty, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IUPnPServiceEnumProperty {
    type Vtable = IUPnPServiceEnumProperty_Vtbl;
}
impl ::core::clone::Clone for IUPnPServiceEnumProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUPnPServiceEnumProperty {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38873b37_91bb_49f4_b249_2e8efbb8a816);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceEnumProperty_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetServiceEnumProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUPnPServices(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPServices {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, bstrserviceid: P0) -> ::windows_core::Result<IUPnPService>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), bstrserviceid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IUPnPServices, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
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
unsafe impl ::windows_core::Interface for IUPnPServices {
    type Vtable = IUPnPServices_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUPnPServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IUPnPServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f8c8e9e_9a7a_4dc8_bc41_ff31fa374956);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServices_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const ADDRESS_FAMILY_VALUE_NAME: ::windows_core::PCWSTR = ::windows_core::w!("AddressFamily");
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
pub const REMOTE_ADDRESS_VALUE_NAME: ::windows_core::PCWSTR = ::windows_core::w!("RemoteAddress");
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(8i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(4i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(0i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(1i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(2i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(0i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(2i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(1i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ACTION_REQUEST_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220976i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ACTION_SPECIFIC_BASE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220736i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220991i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220972i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220988i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_NOTREGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180494i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_RUNNING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180495i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220969i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180511i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DUPLICATE_SERVICE_ID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180510i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220970i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220223i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ICON_ELEMENT_EXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220987i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ICON_NODE_INCOMPLETE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220986i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ACTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220985i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ARGUMENTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220984i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_DESCRIPTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180509i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_DOCUMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220224i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ICON: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180507i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ROOT_NAMESPACE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180505i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180508i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_VARIABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220973i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180506i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_OUT_OF_SYNC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220983i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_PROTOCOL_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220971i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180512i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220992i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220990i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220989i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_SUFFIX_TOO_LONG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180504i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_TRANSPORT_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220975i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_URLBASE_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180503i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_VALUE_TOO_LONG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180496i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220974i32);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_SERVICE_DELAY_SCPD_AND_SUBSCRIPTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDescriptionDocument: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d8a9b47_3a28_4ce2_8a4b_bd34e45bceeb);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDescriptionDocumentEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33fd0563_d81a_4393_83cc_0195b1da2f91);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDevice: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa32552c5_ba61_457a_b59a_a2561e125e33);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDeviceFinder: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2085f28_feb7_404a_b8e7_e659bdeaaa02);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDeviceFinderEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x181b54fc_380b_4a75_b3f1_4ac45e9605b0);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPDevices: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9e84ffd_ad3c_40a4_b835_0882ebcbaaa8);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPRegistrar: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b9_73b2_11d4_bf42_00b0d0118b56);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPRemoteEndpointInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e5e84e9_4049_4244_b728_2d24227157c7);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPService: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc624ba95_fbcb_4409_8c03_8cceec533ef1);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPnPServices: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0bc4b4a_a406_4efc_932f_b8546b8100cc);
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SW_DEVICE_CAPABILITIES(pub i32);
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
impl ::windows_core::TypeKind for SW_DEVICE_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for SW_DEVICE_LIFETIME {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for HSWDEVICE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SW_DEVICE_CREATE_INFO {
    pub cbSize: u32,
    pub pszInstanceId: ::windows_core::PCWSTR,
    pub pszzHardwareIds: ::windows_core::PCWSTR,
    pub pszzCompatibleIds: ::windows_core::PCWSTR,
    pub pContainerId: *const ::windows_core::GUID,
    pub CapabilityFlags: u32,
    pub pszDeviceDescription: ::windows_core::PCWSTR,
    pub pszDeviceLocation: ::windows_core::PCWSTR,
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
impl ::windows_core::TypeKind for SW_DEVICE_CREATE_INFO {
    type TypeKind = ::windows_core::CopyType;
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
pub type SW_DEVICE_CREATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: ::windows_core::HRESULT, pcontext: *const ::core::ffi::c_void, pszdeviceinstanceid: ::windows_core::PCWSTR) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
