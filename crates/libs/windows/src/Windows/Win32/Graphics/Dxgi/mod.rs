#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn CreateDXGIFactory<T>() -> ::windows::core::Result<T>
where
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "dxgi.dll""system" fn CreateDXGIFactory ( riid : *const :: windows::core::GUID , ppfactory : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    CreateDXGIFactory(&<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn CreateDXGIFactory1<T>() -> ::windows::core::Result<T>
where
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "dxgi.dll""system" fn CreateDXGIFactory1 ( riid : *const :: windows::core::GUID , ppfactory : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    CreateDXGIFactory1(&<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn CreateDXGIFactory2<T>(flags: u32) -> ::windows::core::Result<T>
where
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "dxgi.dll""system" fn CreateDXGIFactory2 ( flags : u32 , riid : *const :: windows::core::GUID , ppfactory : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    CreateDXGIFactory2(flags, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn DXGIDeclareAdapterRemovalSupport() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "dxgi.dll""system" fn DXGIDeclareAdapterRemovalSupport ( ) -> :: windows::core::HRESULT );
    DXGIDeclareAdapterRemovalSupport().ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn DXGIGetDebugInterface1<T>(flags: u32) -> ::windows::core::Result<T>
where
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "dxgi.dll""system" fn DXGIGetDebugInterface1 ( flags : u32 , riid : *const :: windows::core::GUID , pdebug : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    DXGIGetDebugInterface1(flags, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter(::windows::core::IUnknown);
impl IDXGIAdapter {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).EnumOutputs)(::windows::core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).CheckInterfaceSupport)(::windows::core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIAdapter, ::windows::core::IUnknown, IDXGIObject);
impl ::core::cmp::PartialEq for IDXGIAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter {}
impl ::core::fmt::Debug for IDXGIAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter {
    type Vtable = IDXGIAdapter_Vtbl;
}
impl ::core::clone::Clone for IDXGIAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIAdapter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, output: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc: usize,
    pub CheckInterfaceSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacename: *const ::windows::core::GUID, pumdversion: *mut i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter1(::windows::core::IUnknown);
impl IDXGIAdapter1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.EnumOutputs)(::windows::core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).base__.CheckInterfaceSupport)(::windows::core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc1)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIAdapter1, ::windows::core::IUnknown, IDXGIObject, IDXGIAdapter);
impl ::core::cmp::PartialEq for IDXGIAdapter1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter1 {}
impl ::core::fmt::Debug for IDXGIAdapter1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter1 {
    type Vtable = IDXGIAdapter1_Vtbl;
}
impl ::core::clone::Clone for IDXGIAdapter1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIAdapter1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29038f61_3839_4626_91fd_086879011a05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter1_Vtbl {
    pub base__: IDXGIAdapter_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter2(::windows::core::IUnknown);
impl IDXGIAdapter2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumOutputs)(::windows::core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).base__.base__.CheckInterfaceSupport)(::windows::core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDesc1)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc2)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIAdapter2, ::windows::core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1);
impl ::core::cmp::PartialEq for IDXGIAdapter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter2 {}
impl ::core::fmt::Debug for IDXGIAdapter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter2 {
    type Vtable = IDXGIAdapter2_Vtbl;
}
impl ::core::clone::Clone for IDXGIAdapter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIAdapter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aa1ae0a_fa0e_4b84_8644_e05ff8e5acb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter2_Vtbl {
    pub base__: IDXGIAdapter1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc2: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter3(::windows::core::IUnknown);
impl IDXGIAdapter3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumOutputs)(::windows::core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.CheckInterfaceSupport)(::windows::core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDesc1)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDesc2)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).RegisterHardwareContentProtectionTeardownStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).UnregisterHardwareContentProtectionTeardownStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryVideoMemoryInfo)(::windows::core::Interface::as_raw(self), nodeindex, memorysegmentgroup, pvideomemoryinfo).ok()
    }
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVideoMemoryReservation)(::windows::core::Interface::as_raw(self), nodeindex, memorysegmentgroup, reservation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).RegisterVideoMemoryBudgetChangeNotificationEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).UnregisterVideoMemoryBudgetChangeNotification)(::windows::core::Interface::as_raw(self), dwcookie)
    }
}
::windows::imp::interface_hierarchy!(IDXGIAdapter3, ::windows::core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1, IDXGIAdapter2);
impl ::core::cmp::PartialEq for IDXGIAdapter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter3 {}
impl ::core::fmt::Debug for IDXGIAdapter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter3 {
    type Vtable = IDXGIAdapter3_Vtbl;
}
impl ::core::clone::Clone for IDXGIAdapter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIAdapter3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x645967a4_1392_4310_a798_8053ce3e93fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter3_Vtbl {
    pub base__: IDXGIAdapter2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterHardwareContentProtectionTeardownStatusEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterHardwareContentProtectionTeardownStatusEvent: usize,
    pub UnregisterHardwareContentProtectionTeardownStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
    pub QueryVideoMemoryInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::HRESULT,
    pub SetVideoMemoryReservation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterVideoMemoryBudgetChangeNotificationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterVideoMemoryBudgetChangeNotificationEvent: usize,
    pub UnregisterVideoMemoryBudgetChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter4(::windows::core::IUnknown);
impl IDXGIAdapter4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EnumOutputs)(::windows::core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.CheckInterfaceSupport)(::windows::core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDesc1)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDesc2)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.RegisterHardwareContentProtectionTeardownStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.UnregisterHardwareContentProtectionTeardownStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.QueryVideoMemoryInfo)(::windows::core::Interface::as_raw(self), nodeindex, memorysegmentgroup, pvideomemoryinfo).ok()
    }
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetVideoMemoryReservation)(::windows::core::Interface::as_raw(self), nodeindex, memorysegmentgroup, reservation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.RegisterVideoMemoryBudgetChangeNotificationEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.UnregisterVideoMemoryBudgetChangeNotification)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc3(&self, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc3)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIAdapter4, ::windows::core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1, IDXGIAdapter2, IDXGIAdapter3);
impl ::core::cmp::PartialEq for IDXGIAdapter4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter4 {}
impl ::core::fmt::Debug for IDXGIAdapter4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter4 {
    type Vtable = IDXGIAdapter4_Vtbl;
}
impl ::core::clone::Clone for IDXGIAdapter4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIAdapter4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c8d99d1_4fbf_4181_a82c_af66bf7bd24e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter4_Vtbl {
    pub base__: IDXGIAdapter3_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc3: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDebug(::windows::core::IUnknown);
impl IDXGIDebug {
    pub unsafe fn ReportLiveObjects(&self, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportLiveObjects)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(apiid), flags).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIDebug, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDXGIDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDebug {}
impl ::core::fmt::Debug for IDXGIDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDebug {
    type Vtable = IDXGIDebug_Vtbl;
}
impl ::core::clone::Clone for IDXGIDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDebug {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x119e7452_de9e_40fe_8806_88f90c12b441);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReportLiveObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDebug1(::windows::core::IUnknown);
impl IDXGIDebug1 {
    pub unsafe fn ReportLiveObjects(&self, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ReportLiveObjects)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(apiid), flags).ok()
    }
    pub unsafe fn EnableLeakTrackingForThread(&self) {
        (::windows::core::Interface::vtable(self).EnableLeakTrackingForThread)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn DisableLeakTrackingForThread(&self) {
        (::windows::core::Interface::vtable(self).DisableLeakTrackingForThread)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLeakTrackingEnabledForThread(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsLeakTrackingEnabledForThread)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IDXGIDebug1, ::windows::core::IUnknown, IDXGIDebug);
impl ::core::cmp::PartialEq for IDXGIDebug1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDebug1 {}
impl ::core::fmt::Debug for IDXGIDebug1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDebug1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDebug1 {
    type Vtable = IDXGIDebug1_Vtbl;
}
impl ::core::clone::Clone for IDXGIDebug1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDebug1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5a05f0c_16f2_4adf_9f4d_a8c4d58ac550);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug1_Vtbl {
    pub base__: IDXGIDebug_Vtbl,
    pub EnableLeakTrackingForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub DisableLeakTrackingForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub IsLeakTrackingEnabledForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLeakTrackingEnabledForThread: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDecodeSwapChain(::windows::core::IUnknown);
impl IDXGIDecodeSwapChain {
    pub unsafe fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).PresentBuffer)(::windows::core::Interface::as_raw(self), buffertopresent, syncinterval, flags)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSourceRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSourceRect)(::windows::core::Interface::as_raw(self), prect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTargetRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetRect)(::windows::core::Interface::as_raw(self), prect).ok()
    }
    pub unsafe fn SetDestSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDestSize)(::windows::core::Interface::as_raw(self), width, height).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSourceRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetSourceRect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTargetRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetTargetRect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDestSize)(::windows::core::Interface::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorSpace)(::windows::core::Interface::as_raw(self), colorspace).ok()
    }
    pub unsafe fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
        (::windows::core::Interface::vtable(self).GetColorSpace)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IDXGIDecodeSwapChain, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDXGIDecodeSwapChain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDecodeSwapChain {}
impl ::core::fmt::Debug for IDXGIDecodeSwapChain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDecodeSwapChain").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDecodeSwapChain {
    type Vtable = IDXGIDecodeSwapChain_Vtbl;
}
impl ::core::clone::Clone for IDXGIDecodeSwapChain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDecodeSwapChain {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2633066b_4514_4c7a_8fd8_12ea98059d18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDecodeSwapChain_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub PresentBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSourceRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetRect: usize,
    pub SetDestSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSourceRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetRect: usize,
    pub GetDestSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT,
    pub SetColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::HRESULT,
    pub GetColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice(::windows::core::IUnknown);
impl IDXGIDevice {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).GetAdapter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateSurface)(::windows::core::Interface::as_raw(self), pdesc, ppsurface.len() as _, usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryResourceResidency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGPUThreadPriority)(::windows::core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetGPUThreadPriority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIDevice, ::windows::core::IUnknown, IDXGIObject);
impl ::core::cmp::PartialEq for IDXGIDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice {}
impl ::core::fmt::Debug for IDXGIDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice {
    type Vtable = IDXGIDevice_Vtbl;
}
impl ::core::clone::Clone for IDXGIDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54ec77fa_1377_44e6_8c32_88fd5f44c84c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSurface: usize,
    pub QueryResourceResidency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT,
    pub GetGPUThreadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice1(::windows::core::IUnknown);
impl IDXGIDevice1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.GetAdapter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CreateSurface)(::windows::core::Interface::as_raw(self), pdesc, ppsurface.len() as _, usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.QueryResourceResidency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetGPUThreadPriority)(::windows::core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.GetGPUThreadPriority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIDevice1, ::windows::core::IUnknown, IDXGIObject, IDXGIDevice);
impl ::core::cmp::PartialEq for IDXGIDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice1 {}
impl ::core::fmt::Debug for IDXGIDevice1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice1 {
    type Vtable = IDXGIDevice1_Vtbl;
}
impl ::core::clone::Clone for IDXGIDevice1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDevice1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77db970f_6276_48ba_ba28_070143b4392c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice1_Vtbl {
    pub base__: IDXGIDevice_Vtbl,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice2(::windows::core::IUnknown);
impl IDXGIDevice2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.GetAdapter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CreateSurface)(::windows::core::Interface::as_raw(self), pdesc, ppsurface.len() as _, usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.QueryResourceResidency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetGPUThreadPriority)(::windows::core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetGPUThreadPriority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OfferResources(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OfferResources)(::windows::core::Interface::as_raw(self), ppresources.len() as _, ::core::mem::transmute(ppresources.as_ptr()), priority).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReclaimResources)(::windows::core::Interface::as_raw(self), numresources, ::core::mem::transmute(ppresources), ::core::mem::transmute(pdiscarded.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).EnqueueSetEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIDevice2, ::windows::core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1);
impl ::core::cmp::PartialEq for IDXGIDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice2 {}
impl ::core::fmt::Debug for IDXGIDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice2 {
    type Vtable = IDXGIDevice2_Vtbl;
}
impl ::core::clone::Clone for IDXGIDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05008617_fbfd_4051_a790_144884b4f6a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice2_Vtbl {
    pub base__: IDXGIDevice1_Vtbl,
    pub OfferResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReclaimResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReclaimResources: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnqueueSetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnqueueSetEvent: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice3(::windows::core::IUnknown);
impl IDXGIDevice3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAdapter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.CreateSurface)(::windows::core::Interface::as_raw(self), pdesc, ppsurface.len() as _, usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.QueryResourceResidency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetGPUThreadPriority)(::windows::core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetGPUThreadPriority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OfferResources(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OfferResources)(::windows::core::Interface::as_raw(self), ppresources.len() as _, ::core::mem::transmute(ppresources.as_ptr()), priority).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ReclaimResources)(::windows::core::Interface::as_raw(self), numresources, ::core::mem::transmute(ppresources), ::core::mem::transmute(pdiscarded.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).base__.EnqueueSetEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi()).ok()
    }
    pub unsafe fn Trim(&self) {
        (::windows::core::Interface::vtable(self).Trim)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IDXGIDevice3, ::windows::core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1, IDXGIDevice2);
impl ::core::cmp::PartialEq for IDXGIDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice3 {}
impl ::core::fmt::Debug for IDXGIDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice3 {
    type Vtable = IDXGIDevice3_Vtbl;
}
impl ::core::clone::Clone for IDXGIDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDevice3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6007896c_3244_4afd_bf18_a6d3beda5023);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice3_Vtbl {
    pub base__: IDXGIDevice2_Vtbl,
    pub Trim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice4(::windows::core::IUnknown);
impl IDXGIDevice4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetAdapter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.CreateSurface)(::windows::core::Interface::as_raw(self), pdesc, ppsurface.len() as _, usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.QueryResourceResidency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetGPUThreadPriority)(::windows::core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetGPUThreadPriority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OfferResources(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.OfferResources)(::windows::core::Interface::as_raw(self), ppresources.len() as _, ::core::mem::transmute(ppresources.as_ptr()), priority).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ReclaimResources)(::windows::core::Interface::as_raw(self), numresources, ::core::mem::transmute(ppresources), ::core::mem::transmute(pdiscarded.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.EnqueueSetEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi()).ok()
    }
    pub unsafe fn Trim(&self) {
        (::windows::core::Interface::vtable(self).base__.Trim)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn OfferResources1(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OfferResources1)(::windows::core::Interface::as_raw(self), ppresources.len() as _, ::core::mem::transmute(ppresources.as_ptr()), priority, flags).ok()
    }
    pub unsafe fn ReclaimResources1(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReclaimResources1)(::windows::core::Interface::as_raw(self), numresources, ::core::mem::transmute(ppresources), presults).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIDevice4, ::windows::core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1, IDXGIDevice2, IDXGIDevice3);
impl ::core::cmp::PartialEq for IDXGIDevice4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice4 {}
impl ::core::fmt::Debug for IDXGIDevice4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice4 {
    type Vtable = IDXGIDevice4_Vtbl;
}
impl ::core::clone::Clone for IDXGIDevice4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDevice4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95b4f95f_d8da_4ca4_9ee6_3b76d5968a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice4_Vtbl {
    pub base__: IDXGIDevice3_Vtbl,
    pub OfferResources1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::HRESULT,
    pub ReclaimResources1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDeviceSubObject(::windows::core::IUnknown);
impl IDXGIDeviceSubObject {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIDeviceSubObject, ::windows::core::IUnknown, IDXGIObject);
impl ::core::cmp::PartialEq for IDXGIDeviceSubObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDeviceSubObject {}
impl ::core::fmt::Debug for IDXGIDeviceSubObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDeviceSubObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDeviceSubObject {
    type Vtable = IDXGIDeviceSubObject_Vtbl;
}
impl ::core::clone::Clone for IDXGIDeviceSubObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDeviceSubObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDisplayControl(::windows::core::IUnknown);
impl IDXGIDisplayControl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsStereoEnabled)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStereoEnabled<P0>(&self, enabled: P0)
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetStereoEnabled)(::windows::core::Interface::as_raw(self), enabled.into_param().abi())
    }
}
::windows::imp::interface_hierarchy!(IDXGIDisplayControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDXGIDisplayControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDisplayControl {}
impl ::core::fmt::Debug for IDXGIDisplayControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDisplayControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDisplayControl {
    type Vtable = IDXGIDisplayControl_Vtbl;
}
impl ::core::clone::Clone for IDXGIDisplayControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIDisplayControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea9dbf1a_c88e_4486_854a_98aa0138f30c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDisplayControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStereoEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStereoEnabled: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory(::windows::core::IUnknown);
impl IDXGIFactory {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).EnumAdapters)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).MakeWindowAssociation)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetWindowAssociation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateSwapChain)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).CreateSoftwareAdapter)(::windows::core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIFactory, ::windows::core::IUnknown, IDXGIObject);
impl ::core::cmp::PartialEq for IDXGIFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory {}
impl ::core::fmt::Debug for IDXGIFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory {
    type Vtable = IDXGIFactory_Vtbl;
}
impl ::core::clone::Clone for IDXGIFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b7166ec_21c7_44ae_b21a_c9ae321ae369);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumAdapters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MakeWindowAssociation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MakeWindowAssociation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindowAssociation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindowAssociation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSoftwareAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: super::super::Foundation::HINSTANCE, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSoftwareAdapter: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory1(::windows::core::IUnknown);
impl IDXGIFactory1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.EnumAdapters)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.MakeWindowAssociation)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.GetWindowAssociation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.CreateSwapChain)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.CreateSoftwareAdapter)(::windows::core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter1>();
        (::windows::core::Interface::vtable(self).EnumAdapters1)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsCurrent)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IDXGIFactory1, ::windows::core::IUnknown, IDXGIObject, IDXGIFactory);
impl ::core::cmp::PartialEq for IDXGIFactory1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory1 {}
impl ::core::fmt::Debug for IDXGIFactory1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory1 {
    type Vtable = IDXGIFactory1_Vtbl;
}
impl ::core::clone::Clone for IDXGIFactory1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIFactory1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x770aae78_f26f_4dba_a829_253c83d1b387);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory1_Vtbl {
    pub base__: IDXGIFactory_Vtbl,
    pub EnumAdapters1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCurrent: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory2(::windows::core::IUnknown);
impl IDXGIFactory2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumAdapters)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.MakeWindowAssociation)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.base__.GetWindowAssociation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CreateSwapChain)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateSoftwareAdapter)(::windows::core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter1>();
        (::windows::core::Interface::vtable(self).base__.EnumAdapters1)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsCurrent)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsWindowedStereoEnabled)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).CreateSwapChainForHwnd)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).CreateSwapChainForCoreWindow)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::LUID>();
        (::windows::core::Interface::vtable(self).GetSharedResourceAdapterLuid)(::windows::core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).RegisterStereoStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).RegisterStereoStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).UnregisterStereoStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).RegisterOcclusionStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).RegisterOcclusionStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).UnregisterOcclusionStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).CreateSwapChainForComposition)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIFactory2, ::windows::core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1);
impl ::core::cmp::PartialEq for IDXGIFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory2 {}
impl ::core::fmt::Debug for IDXGIFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory2 {
    type Vtable = IDXGIFactory2_Vtbl;
}
impl ::core::clone::Clone for IDXGIFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50c83a1c_e072_4c48_87b0_3630fa36a6d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory2_Vtbl {
    pub base__: IDXGIFactory1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWindowedStereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWindowedStereoEnabled: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForHwnd: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForCoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForCoreWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSharedResourceAdapterLuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSharedResourceAdapterLuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterStereoStatusWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterStereoStatusEvent: usize,
    pub UnregisterStereoStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterOcclusionStatusWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterOcclusionStatusEvent: usize,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForComposition: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory3(::windows::core::IUnknown);
impl IDXGIFactory3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumAdapters)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.MakeWindowAssociation)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetWindowAssociation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.CreateSwapChain)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter1>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumAdapters1)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.IsCurrent)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsWindowedStereoEnabled)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.CreateSwapChainForHwnd)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.CreateSwapChainForCoreWindow)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::LUID>();
        (::windows::core::Interface::vtable(self).base__.GetSharedResourceAdapterLuid)(::windows::core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.RegisterStereoStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.RegisterStereoStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.UnregisterStereoStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.RegisterOcclusionStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.RegisterOcclusionStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.UnregisterOcclusionStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.CreateSwapChainForComposition)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetCreationFlags)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IDXGIFactory3, ::windows::core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2);
impl ::core::cmp::PartialEq for IDXGIFactory3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory3 {}
impl ::core::fmt::Debug for IDXGIFactory3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory3 {
    type Vtable = IDXGIFactory3_Vtbl;
}
impl ::core::clone::Clone for IDXGIFactory3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIFactory3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25483823_cd46_4c7d_86ca_47aa95b837bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory3_Vtbl {
    pub base__: IDXGIFactory2_Vtbl,
    pub GetCreationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory4(::windows::core::IUnknown);
impl IDXGIFactory4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EnumAdapters)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.MakeWindowAssociation)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetWindowAssociation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.CreateSwapChain)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumAdapters1)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.IsCurrent)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.IsWindowedStereoEnabled)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateSwapChainForHwnd)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateSwapChainForCoreWindow)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::LUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSharedResourceAdapterLuid)(::windows::core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.RegisterStereoStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.RegisterStereoStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.base__.UnregisterStereoStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.RegisterOcclusionStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.RegisterOcclusionStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.base__.UnregisterOcclusionStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateSwapChainForComposition)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetCreationFlags)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).EnumAdapterByLuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(adapterluid), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).EnumWarpAdapter)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIFactory4, ::windows::core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3);
impl ::core::cmp::PartialEq for IDXGIFactory4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory4 {}
impl ::core::fmt::Debug for IDXGIFactory4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory4 {
    type Vtable = IDXGIFactory4_Vtbl;
}
impl ::core::clone::Clone for IDXGIFactory4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIFactory4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc6ea02_ef36_464f_bf0c_21ca39e5168a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory4_Vtbl {
    pub base__: IDXGIFactory3_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumAdapterByLuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumAdapterByLuid: usize,
    pub EnumWarpAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory5(::windows::core::IUnknown);
impl IDXGIFactory5 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.EnumAdapters)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.MakeWindowAssociation)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetWindowAssociation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSwapChain)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EnumAdapters1)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.IsCurrent)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.IsWindowedStereoEnabled)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.CreateSwapChainForHwnd)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.CreateSwapChainForCoreWindow)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::LUID>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetSharedResourceAdapterLuid)(::windows::core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.RegisterStereoStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.RegisterStereoStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.UnregisterStereoStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.RegisterOcclusionStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.RegisterOcclusionStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.UnregisterOcclusionStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.CreateSwapChainForComposition)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.base__.GetCreationFlags)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.EnumAdapterByLuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(adapterluid), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.EnumWarpAdapter)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckFeatureSupport)(::windows::core::Interface::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIFactory5, ::windows::core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4);
impl ::core::cmp::PartialEq for IDXGIFactory5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory5 {}
impl ::core::fmt::Debug for IDXGIFactory5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory5 {
    type Vtable = IDXGIFactory5_Vtbl;
}
impl ::core::clone::Clone for IDXGIFactory5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIFactory5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7632e1f5_ee65_4dca_87fd_84cd75f8838d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory5_Vtbl {
    pub base__: IDXGIFactory4_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory6(::windows::core::IUnknown);
impl IDXGIFactory6 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.EnumAdapters)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.MakeWindowAssociation)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetWindowAssociation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.CreateSwapChain)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.EnumAdapters1)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.IsCurrent)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.IsWindowedStereoEnabled)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.CreateSwapChainForHwnd)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.CreateSwapChainForCoreWindow)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::LUID>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetSharedResourceAdapterLuid)(::windows::core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RegisterStereoStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RegisterStereoStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.UnregisterStereoStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RegisterOcclusionStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RegisterOcclusionStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.UnregisterOcclusionStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.CreateSwapChainForComposition)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetCreationFlags)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.EnumAdapterByLuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(adapterluid), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.EnumWarpAdapter)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CheckFeatureSupport)(::windows::core::Interface::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn EnumAdapterByGpuPreference<T>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).EnumAdapterByGpuPreference)(::windows::core::Interface::as_raw(self), adapter, gpupreference, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIFactory6, ::windows::core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4, IDXGIFactory5);
impl ::core::cmp::PartialEq for IDXGIFactory6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory6 {}
impl ::core::fmt::Debug for IDXGIFactory6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory6 {
    type Vtable = IDXGIFactory6_Vtbl;
}
impl ::core::clone::Clone for IDXGIFactory6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIFactory6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1b6694f_ff09_44a9_b03c_77900a0a1d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory6_Vtbl {
    pub base__: IDXGIFactory5_Vtbl,
    pub EnumAdapterByGpuPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory7(::windows::core::IUnknown);
impl IDXGIFactory7 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.EnumAdapters)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.MakeWindowAssociation)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetWindowAssociation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSwapChain)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::windows::core::zeroed::<IDXGIAdapter1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.EnumAdapters1)(::windows::core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.IsCurrent)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.IsWindowedStereoEnabled)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSwapChainForHwnd)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSwapChainForCoreWindow)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::LUID>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetSharedResourceAdapterLuid)(::windows::core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RegisterStereoStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RegisterStereoStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.UnregisterStereoStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RegisterOcclusionStatusWindow)(::windows::core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RegisterOcclusionStatusEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.UnregisterOcclusionStatus)(::windows::core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSwapChainForComposition)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetCreationFlags)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumAdapterByLuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(adapterluid), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumWarpAdapter)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CheckFeatureSupport)(::windows::core::Interface::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn EnumAdapterByGpuPreference<T>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.EnumAdapterByGpuPreference)(::windows::core::Interface::as_raw(self), adapter, gpupreference, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterAdaptersChangedEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).RegisterAdaptersChangedEvent)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterAdaptersChangedEvent)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIFactory7, ::windows::core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4, IDXGIFactory5, IDXGIFactory6);
impl ::core::cmp::PartialEq for IDXGIFactory7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory7 {}
impl ::core::fmt::Debug for IDXGIFactory7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory7").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory7 {
    type Vtable = IDXGIFactory7_Vtbl;
}
impl ::core::clone::Clone for IDXGIFactory7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIFactory7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4966eed_76db_44da_84c1_ee9a7afb20a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory7_Vtbl {
    pub base__: IDXGIFactory6_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterAdaptersChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterAdaptersChangedEvent: usize,
    pub UnregisterAdaptersChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactoryMedia(::windows::core::IUnknown);
impl IDXGIFactoryMedia {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCompositionSurfaceHandle<P0, P1, P2>(&self, pdevice: P0, hsurface: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
        P2: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGISwapChain1>();
        (::windows::core::Interface::vtable(self).CreateSwapChainForCompositionSurfaceHandle)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), hsurface.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDecodeSwapChainForCompositionSurfaceHandle<P0, P1, P2, P3>(&self, pdevice: P0, hsurface: P1, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: P2, prestricttooutput: P3) -> ::windows::core::Result<IDXGIDecodeSwapChain>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
        P2: ::windows::core::IntoParam<IDXGIResource>,
        P3: ::windows::core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIDecodeSwapChain>();
        (::windows::core::Interface::vtable(self).CreateDecodeSwapChainForCompositionSurfaceHandle)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), hsurface.into_param().abi(), pdesc, pyuvdecodebuffers.into_param().abi(), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIFactoryMedia, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDXGIFactoryMedia {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactoryMedia {}
impl ::core::fmt::Debug for IDXGIFactoryMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactoryMedia").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactoryMedia {
    type Vtable = IDXGIFactoryMedia_Vtbl;
}
impl ::core::clone::Clone for IDXGIFactoryMedia {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIFactoryMedia {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41e7d1f2_a591_4f7b_a2e5_fa9c843e1c12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactoryMedia_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForCompositionSurfaceHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDecodeSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut ::core::ffi::c_void, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDecodeSwapChainForCompositionSurfaceHandle: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIInfoQueue(::windows::core::IUnknown);
impl IDXGIInfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, producer: ::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMessageCountLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), messagecountlimit).ok()
    }
    pub unsafe fn ClearStoredMessages(&self, producer: ::windows::core::GUID) {
        (::windows::core::Interface::vtable(self).ClearStoredMessages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetMessage(&self, producer: ::windows::core::GUID, messageindex: u64, pmessage: ::core::option::Option<*mut DXGI_INFO_QUEUE_MESSAGE>, pmessagebytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMessage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), messageindex, ::core::mem::transmute(pmessage.unwrap_or(::std::ptr::null_mut())), pmessagebytelength).ok()
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: ::windows::core::GUID) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetNumStoredMessages(&self, producer: ::windows::core::GUID) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumStoredMessages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: ::windows::core::GUID) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetMessageCountLimit(&self, producer: ::windows::core::GUID) -> u64 {
        (::windows::core::Interface::vtable(self).GetMessageCountLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self, producer: ::windows::core::GUID) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumMessagesAllowedByStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self, producer: ::windows::core::GUID) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumMessagesDeniedByStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn AddStorageFilterEntries(&self, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStorageFilterEntries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), pfilter).ok()
    }
    pub unsafe fn GetStorageFilter(&self, producer: ::windows::core::GUID, pfilter: ::core::option::Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), ::core::mem::transmute(pfilter.unwrap_or(::std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearStorageFilter(&self, producer: ::windows::core::GUID) {
        (::windows::core::Interface::vtable(self).ClearStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn PushEmptyStorageFilter(&self, producer: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushEmptyStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushDenyAllStorageFilter(&self, producer: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushDenyAllStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushCopyOfStorageFilter(&self, producer: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushCopyOfStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushStorageFilter(&self, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), pfilter).ok()
    }
    pub unsafe fn PopStorageFilter(&self, producer: ::windows::core::GUID) {
        (::windows::core::Interface::vtable(self).PopStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetStorageFilterStackSize(&self, producer: ::windows::core::GUID) -> u32 {
        (::windows::core::Interface::vtable(self).GetStorageFilterStackSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRetrievalFilterEntries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), pfilter).ok()
    }
    pub unsafe fn GetRetrievalFilter(&self, producer: ::windows::core::GUID, pfilter: ::core::option::Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRetrievalFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), ::core::mem::transmute(pfilter.unwrap_or(::std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearRetrievalFilter(&self, producer: ::windows::core::GUID) {
        (::windows::core::Interface::vtable(self).ClearRetrievalFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self, producer: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushEmptyRetrievalFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushDenyAllRetrievalFilter(&self, producer: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushDenyAllRetrievalFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self, producer: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushCopyOfRetrievalFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushRetrievalFilter(&self, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushRetrievalFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), pfilter).ok()
    }
    pub unsafe fn PopRetrievalFilter(&self, producer: ::windows::core::GUID) {
        (::windows::core::Interface::vtable(self).PopRetrievalFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self, producer: ::windows::core::GUID) -> u32 {
        (::windows::core::Interface::vtable(self).GetRetrievalFilterStackSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn AddMessage<P0>(&self, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).AddMessage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), category, severity, id, pdescription.into_param().abi()).ok()
    }
    pub unsafe fn AddApplicationMessage<P0>(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).AddApplicationMessage)(::windows::core::Interface::as_raw(self), severity, pdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnCategory<P0>(&self, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBreakOnCategory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), category, benable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnSeverity<P0>(&self, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBreakOnSeverity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), severity, benable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnID<P0>(&self, producer: ::windows::core::GUID, id: i32, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBreakOnID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), id, benable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnCategory(&self, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetBreakOnCategory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), category)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnSeverity(&self, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetBreakOnSeverity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), severity)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnID(&self, producer: ::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetBreakOnID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), id)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuteDebugOutput<P0>(&self, producer: ::windows::core::GUID, bmute: P0)
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMuteDebugOutput)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer), bmute.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMuteDebugOutput(&self, producer: ::windows::core::GUID) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetMuteDebugOutput)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
}
::windows::imp::interface_hierarchy!(IDXGIInfoQueue, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDXGIInfoQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIInfoQueue {}
impl ::core::fmt::Debug for IDXGIInfoQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIInfoQueue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIInfoQueue {
    type Vtable = IDXGIInfoQueue_Vtbl;
}
impl ::core::clone::Clone for IDXGIInfoQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIInfoQueue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd67441c7_672a_476f_9e82_cd55b44949ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIInfoQueue_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT,
    pub GetNumStoredMessagesAllowedByRetrievalFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushDenyAllStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushDenyAllRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32,
    pub AddMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, bmute: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMuteDebugOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMuteDebugOutput: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIKeyedMutex(::windows::core::IUnknown);
impl IDXGIKeyedMutex {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquireSync)(::windows::core::Interface::as_raw(self), key, dwmilliseconds).ok()
    }
    pub unsafe fn ReleaseSync(&self, key: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseSync)(::windows::core::Interface::as_raw(self), key).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIKeyedMutex, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl ::core::cmp::PartialEq for IDXGIKeyedMutex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIKeyedMutex {}
impl ::core::fmt::Debug for IDXGIKeyedMutex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIKeyedMutex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIKeyedMutex {
    type Vtable = IDXGIKeyedMutex_Vtbl;
}
impl ::core::clone::Clone for IDXGIKeyedMutex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIKeyedMutex {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d8e1289_d7b3_465f_8126_250e349af85d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIKeyedMutex_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub AcquireSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: u64, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub ReleaseSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIObject(::windows::core::IUnknown);
impl IDXGIObject {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDXGIObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIObject {}
impl ::core::fmt::Debug for IDXGIObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIObject {
    type Vtable = IDXGIObject_Vtbl;
}
impl ::core::clone::Clone for IDXGIObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput(::windows::core::IUnknown);
impl IDXGIOutput {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayModeList)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).FindClosestMatchingMode)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForVBlank)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).TakeOwnership)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).ReleaseOwnership)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGammaControlCapabilities)(::windows::core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).SetDisplaySurface)(::windows::core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).GetDisplaySurfaceData)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIOutput, ::windows::core::IUnknown, IDXGIObject);
impl ::core::cmp::PartialEq for IDXGIOutput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput {}
impl ::core::fmt::Debug for IDXGIOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput {
    type Vtable = IDXGIOutput_Vtbl;
}
impl ::core::clone::Clone for IDXGIOutput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIOutput {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae02eedb_c735_4690_8d52_5a8dc20213aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDisplayModeList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDisplayModeList: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub FindClosestMatchingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    FindClosestMatchingMode: usize,
    pub WaitForVBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TakeOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TakeOwnership: usize,
    pub ReleaseOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetGammaControlCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetGammaControlCapabilities: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetGammaControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetGammaControl: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetGammaControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetGammaControl: usize,
    pub SetDisplaySurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscanoutsurface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDisplaySurfaceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput1(::windows::core::IUnknown);
impl IDXGIOutput1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDisplayModeList)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.FindClosestMatchingMode)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.WaitForVBlank)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.TakeOwnership)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base__.ReleaseOwnership)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetGammaControlCapabilities)(::windows::core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDisplaySurface)(::windows::core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.GetDisplaySurfaceData)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayModeList1)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).FindClosestMatchingMode1)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGIResource>,
    {
        (::windows::core::Interface::vtable(self).GetDisplaySurfaceData1)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutputDuplication>();
        (::windows::core::Interface::vtable(self).DuplicateOutput)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIOutput1, ::windows::core::IUnknown, IDXGIObject, IDXGIOutput);
impl ::core::cmp::PartialEq for IDXGIOutput1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput1 {}
impl ::core::fmt::Debug for IDXGIOutput1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput1 {
    type Vtable = IDXGIOutput1_Vtbl;
}
impl ::core::clone::Clone for IDXGIOutput1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIOutput1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00cddea8_939b_4b83_a340_a685226666cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput1_Vtbl {
    pub base__: IDXGIOutput_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDisplayModeList1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDisplayModeList1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub FindClosestMatchingMode1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    FindClosestMatchingMode1: usize,
    pub GetDisplaySurfaceData1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DuplicateOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput2(::windows::core::IUnknown);
impl IDXGIOutput2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDisplayModeList)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.FindClosestMatchingMode)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.WaitForVBlank)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.TakeOwnership)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base__.base__.ReleaseOwnership)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetGammaControlCapabilities)(::windows::core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetDisplaySurface)(::windows::core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetDisplaySurfaceData)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDisplayModeList1)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.FindClosestMatchingMode1)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGIResource>,
    {
        (::windows::core::Interface::vtable(self).base__.GetDisplaySurfaceData1)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutputDuplication>();
        (::windows::core::Interface::vtable(self).base__.DuplicateOutput)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).SupportsOverlays)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IDXGIOutput2, ::windows::core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1);
impl ::core::cmp::PartialEq for IDXGIOutput2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput2 {}
impl ::core::fmt::Debug for IDXGIOutput2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput2 {
    type Vtable = IDXGIOutput2_Vtbl;
}
impl ::core::clone::Clone for IDXGIOutput2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIOutput2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x595e39d1_2724_4663_99b1_da969de28364);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput2_Vtbl {
    pub base__: IDXGIOutput1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsOverlays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsOverlays: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput3(::windows::core::IUnknown);
impl IDXGIOutput3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDisplayModeList)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.FindClosestMatchingMode)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.WaitForVBlank)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.TakeOwnership)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ReleaseOwnership)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetGammaControlCapabilities)(::windows::core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetDisplaySurface)(::windows::core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDisplaySurfaceData)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDisplayModeList1)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.FindClosestMatchingMode1)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGIResource>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetDisplaySurfaceData1)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutputDuplication>();
        (::windows::core::Interface::vtable(self).base__.base__.DuplicateOutput)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.SupportsOverlays)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CheckOverlaySupport)(::windows::core::Interface::as_raw(self), enumformat, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIOutput3, ::windows::core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2);
impl ::core::cmp::PartialEq for IDXGIOutput3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput3 {}
impl ::core::fmt::Debug for IDXGIOutput3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput3 {
    type Vtable = IDXGIOutput3_Vtbl;
}
impl ::core::clone::Clone for IDXGIOutput3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIOutput3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a6bb301_7e7e_41f4_a8e0_5b32f7f99b18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput3_Vtbl {
    pub base__: IDXGIOutput2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlaySupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlaySupport: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput4(::windows::core::IUnknown);
impl IDXGIOutput4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDisplayModeList)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.FindClosestMatchingMode)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.WaitForVBlank)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.TakeOwnership)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ReleaseOwnership)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetGammaControlCapabilities)(::windows::core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetDisplaySurface)(::windows::core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDisplaySurfaceData)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDisplayModeList1)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.FindClosestMatchingMode1)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGIResource>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDisplaySurfaceData1)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutputDuplication>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.DuplicateOutput)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.SupportsOverlays)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.CheckOverlaySupport)(::windows::core::Interface::as_raw(self), enumformat, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<P0>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CheckOverlayColorSpaceSupport)(::windows::core::Interface::as_raw(self), format, colorspace, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIOutput4, ::windows::core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3);
impl ::core::cmp::PartialEq for IDXGIOutput4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput4 {}
impl ::core::fmt::Debug for IDXGIOutput4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput4 {
    type Vtable = IDXGIOutput4_Vtbl;
}
impl ::core::clone::Clone for IDXGIOutput4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIOutput4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc7dca35_2196_414d_9f53_617884032a60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput4_Vtbl {
    pub base__: IDXGIOutput3_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlayColorSpaceSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlayColorSpaceSupport: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput5(::windows::core::IUnknown);
impl IDXGIOutput5 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDisplayModeList)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.FindClosestMatchingMode)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.WaitForVBlank)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.TakeOwnership)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ReleaseOwnership)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetGammaControlCapabilities)(::windows::core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetDisplaySurface)(::windows::core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDisplaySurfaceData)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDisplayModeList1)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.FindClosestMatchingMode1)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGIResource>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDisplaySurfaceData1)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutputDuplication>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.DuplicateOutput)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SupportsOverlays)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.CheckOverlaySupport)(::windows::core::Interface::as_raw(self), enumformat, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<P0>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.CheckOverlayColorSpaceSupport)(::windows::core::Interface::as_raw(self), format, colorspace, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DuplicateOutput1<P0>(&self, pdevice: P0, flags: u32, psupportedformats: &[Common::DXGI_FORMAT]) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutputDuplication>();
        (::windows::core::Interface::vtable(self).DuplicateOutput1)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), flags, psupportedformats.len() as _, ::core::mem::transmute(psupportedformats.as_ptr()), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIOutput5, ::windows::core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3, IDXGIOutput4);
impl ::core::cmp::PartialEq for IDXGIOutput5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput5 {}
impl ::core::fmt::Debug for IDXGIOutput5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput5 {
    type Vtable = IDXGIOutput5_Vtbl;
}
impl ::core::clone::Clone for IDXGIOutput5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIOutput5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80a07424_ab52_42eb_833c_0c42fd282d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput5_Vtbl {
    pub base__: IDXGIOutput4_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub DuplicateOutput1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    DuplicateOutput1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput6(::windows::core::IUnknown);
impl IDXGIOutput6 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetDisplayModeList)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.FindClosestMatchingMode)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.WaitForVBlank)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.TakeOwnership)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ReleaseOwnership)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetGammaControlCapabilities)(::windows::core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetGammaControl)(::windows::core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetDisplaySurface)(::windows::core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGISurface>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetDisplaySurfaceData)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDisplayModeList1)(::windows::core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.FindClosestMatchingMode1)(::windows::core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDXGIResource>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDisplaySurfaceData1)(::windows::core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutputDuplication>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.DuplicateOutput)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SupportsOverlays)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.CheckOverlaySupport)(::windows::core::Interface::as_raw(self), enumformat, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<P0>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.CheckOverlayColorSpaceSupport)(::windows::core::Interface::as_raw(self), format, colorspace, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DuplicateOutput1<P0>(&self, pdevice: P0, flags: u32, psupportedformats: &[Common::DXGI_FORMAT]) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutputDuplication>();
        (::windows::core::Interface::vtable(self).base__.DuplicateOutput1)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), flags, psupportedformats.len() as _, ::core::mem::transmute(psupportedformats.as_ptr()), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc1)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckHardwareCompositionSupport(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CheckHardwareCompositionSupport)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIOutput6, ::windows::core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3, IDXGIOutput4, IDXGIOutput5);
impl ::core::cmp::PartialEq for IDXGIOutput6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput6 {}
impl ::core::fmt::Debug for IDXGIOutput6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput6 {
    type Vtable = IDXGIOutput6_Vtbl;
}
impl ::core::clone::Clone for IDXGIOutput6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIOutput6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x068346e8_aaec_4b84_add7_137f513f77a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput6_Vtbl {
    pub base__: IDXGIOutput5_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc1: usize,
    pub CheckHardwareCompositionSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutputDuplication(::windows::core::IUnknown);
impl IDXGIOutputDuplication {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), pdesc)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::core::option::Option<IDXGIResource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquireNextFrame)(::windows::core::Interface::as_raw(self), timeoutinmilliseconds, pframeinfo, ::core::mem::transmute(ppdesktopresource)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameDirtyRects)(::windows::core::Interface::as_raw(self), dirtyrectsbuffersize, pdirtyrectsbuffer, pdirtyrectsbuffersizerequired).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameMoveRects)(::windows::core::Interface::as_raw(self), moverectsbuffersize, pmoverectbuffer, pmoverectsbuffersizerequired).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFramePointerShape)(::windows::core::Interface::as_raw(self), pointershapebuffersize, ppointershapebuffer, ppointershapebuffersizerequired, ppointershapeinfo).ok()
    }
    pub unsafe fn MapDesktopSurface(&self) -> ::windows::core::Result<DXGI_MAPPED_RECT> {
        let mut result__ = ::windows::core::zeroed::<DXGI_MAPPED_RECT>();
        (::windows::core::Interface::vtable(self).MapDesktopSurface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnMapDesktopSurface(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnMapDesktopSurface)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseFrame(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseFrame)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGIOutputDuplication, ::windows::core::IUnknown, IDXGIObject);
impl ::core::cmp::PartialEq for IDXGIOutputDuplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutputDuplication {}
impl ::core::fmt::Debug for IDXGIOutputDuplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutputDuplication").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutputDuplication {
    type Vtable = IDXGIOutputDuplication_Vtbl;
}
impl ::core::clone::Clone for IDXGIOutputDuplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIOutputDuplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x191cfac3_a341_470d_b26e_a864f428319c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutputDuplication_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AcquireNextFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AcquireNextFrame: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFrameDirtyRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFrameDirtyRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFrameMoveRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFrameMoveRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFramePointerShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFramePointerShape: usize,
    pub MapDesktopSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows::core::HRESULT,
    pub UnMapDesktopSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIResource(::windows::core::IUnknown);
impl IDXGIResource {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HANDLE>();
        (::windows::core::Interface::vtable(self).GetSharedHandle)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUsage(&self) -> ::windows::core::Result<DXGI_USAGE> {
        let mut result__ = ::windows::core::zeroed::<DXGI_USAGE>();
        (::windows::core::Interface::vtable(self).GetUsage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEvictionPriority)(::windows::core::Interface::as_raw(self), evictionpriority).ok()
    }
    pub unsafe fn GetEvictionPriority(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetEvictionPriority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIResource, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl ::core::cmp::PartialEq for IDXGIResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIResource {}
impl ::core::fmt::Debug for IDXGIResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIResource {
    type Vtable = IDXGIResource_Vtbl;
}
impl ::core::clone::Clone for IDXGIResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x035f3ab4_482e_4e50_b41f_8a7f8bd8960b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSharedHandle: usize,
    pub GetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusage: *mut DXGI_USAGE) -> ::windows::core::HRESULT,
    pub SetEvictionPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::HRESULT,
    pub GetEvictionPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevictionpriority: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIResource1(::windows::core::IUnknown);
impl IDXGIResource1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HANDLE>();
        (::windows::core::Interface::vtable(self).base__.GetSharedHandle)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUsage(&self) -> ::windows::core::Result<DXGI_USAGE> {
        let mut result__ = ::windows::core::zeroed::<DXGI_USAGE>();
        (::windows::core::Interface::vtable(self).base__.GetUsage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEvictionPriority)(::windows::core::Interface::as_raw(self), evictionpriority).ok()
    }
    pub unsafe fn GetEvictionPriority(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetEvictionPriority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSubresourceSurface(&self, index: u32) -> ::windows::core::Result<IDXGISurface2> {
        let mut result__ = ::windows::core::zeroed::<IDXGISurface2>();
        (::windows::core::Interface::vtable(self).CreateSubresourceSurface)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0>(&self, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwaccess: u32, lpname: P0) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HANDLE>();
        (::windows::core::Interface::vtable(self).CreateSharedHandle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), dwaccess, lpname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGIResource1, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGIResource);
impl ::core::cmp::PartialEq for IDXGIResource1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIResource1 {}
impl ::core::fmt::Debug for IDXGIResource1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIResource1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIResource1 {
    type Vtable = IDXGIResource1_Vtbl;
}
impl ::core::clone::Clone for IDXGIResource1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGIResource1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30961379_4609_4a41_998e_54fe567ee0c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource1_Vtbl {
    pub base__: IDXGIResource_Vtbl,
    pub CreateSubresourceSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub CreateSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: ::windows::core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    CreateSharedHandle: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISurface(::windows::core::IUnknown);
impl IDXGISurface {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Map)(::windows::core::Interface::as_raw(self), plockedrect, mapflags).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unmap)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGISurface, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl ::core::cmp::PartialEq for IDXGISurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISurface {}
impl ::core::fmt::Debug for IDXGISurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISurface {
    type Vtable = IDXGISurface_Vtbl;
}
impl ::core::clone::Clone for IDXGISurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGISurface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISurface1(::windows::core::IUnknown);
impl IDXGISurface1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Map)(::windows::core::Interface::as_raw(self), plockedrect, mapflags).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Unmap)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDC<P0>(&self, discard: P0) -> ::windows::core::Result<super::Gdi::HDC>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<super::Gdi::HDC>();
        (::windows::core::Interface::vtable(self).GetDC)(::windows::core::Interface::as_raw(self), discard.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseDC(&self, pdirtyrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseDC)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdirtyrect.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGISurface1, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISurface);
impl ::core::cmp::PartialEq for IDXGISurface1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISurface1 {}
impl ::core::fmt::Debug for IDXGISurface1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISurface1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISurface1 {
    type Vtable = IDXGISurface1_Vtbl;
}
impl ::core::clone::Clone for IDXGISurface1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGISurface1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ae63092_6327_4c1b_80ae_bfe12ea32b86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface1_Vtbl {
    pub base__: IDXGISurface_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseDC: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISurface2(::windows::core::IUnknown);
impl IDXGISurface2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Map)(::windows::core::Interface::as_raw(self), plockedrect, mapflags).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Unmap)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDC<P0>(&self, discard: P0) -> ::windows::core::Result<super::Gdi::HDC>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<super::Gdi::HDC>();
        (::windows::core::Interface::vtable(self).base__.GetDC)(::windows::core::Interface::as_raw(self), discard.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseDC(&self, pdirtyrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ReleaseDC)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdirtyrect.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetResource<T>(&self, psubresourceindex: *mut u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetResource)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__, psubresourceindex).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGISurface2, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISurface, IDXGISurface1);
impl ::core::cmp::PartialEq for IDXGISurface2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISurface2 {}
impl ::core::fmt::Debug for IDXGISurface2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISurface2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISurface2 {
    type Vtable = IDXGISurface2_Vtbl;
}
impl ::core::clone::Clone for IDXGISurface2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGISurface2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaba496dd_b617_4cb8_a866_bc44d7eb1fa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface2_Vtbl {
    pub base__: IDXGISurface1_Vtbl,
    pub GetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain(::windows::core::IUnknown);
impl IDXGISwapChain {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Present)(::windows::core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), buffer, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        (::windows::core::Interface::vtable(self).SetFullscreenState)(::windows::core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFullscreenState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResizeBuffers)(::windows::core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResizeTarget)(::windows::core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).GetContainingOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLastPresentCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGISwapChain, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl ::core::cmp::PartialEq for IDXGISwapChain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain {}
impl ::core::fmt::Debug for IDXGISwapChain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain {
    type Vtable = IDXGISwapChain_Vtbl;
}
impl ::core::clone::Clone for IDXGISwapChain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGISwapChain {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: u32, riid: *const ::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFullscreenState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFullscreenState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFullscreenState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFullscreenState: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeTarget: usize,
    pub GetContainingOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    pub GetLastPresentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain1(::windows::core::IUnknown);
impl IDXGISwapChain1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.Present)(::windows::core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetBuffer)(::windows::core::Interface::as_raw(self), buffer, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        (::windows::core::Interface::vtable(self).base__.SetFullscreenState)(::windows::core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFullscreenState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResizeBuffers)(::windows::core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResizeTarget)(::windows::core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.GetContainingOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetLastPresentCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc1)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFullscreenDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetHwnd)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetCoreWindow)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Present1)(::windows::core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsTemporaryMonoSupported)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).GetRestrictToOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackgroundColor)(::windows::core::Interface::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__ = ::windows::core::zeroed::<DXGI_RGBA>();
        (::windows::core::Interface::vtable(self).GetBackgroundColor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRotation)(::windows::core::Interface::as_raw(self), rotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::windows::core::zeroed::<Common::DXGI_MODE_ROTATION>();
        (::windows::core::Interface::vtable(self).GetRotation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDXGISwapChain1, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain);
impl ::core::cmp::PartialEq for IDXGISwapChain1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain1 {}
impl ::core::fmt::Debug for IDXGISwapChain1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain1 {
    type Vtable = IDXGISwapChain1_Vtbl;
}
impl ::core::clone::Clone for IDXGISwapChain1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGISwapChain1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x790a45f7_0d42_4876_983a_0a55cfe6f4aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain1_Vtbl {
    pub base__: IDXGISwapChain_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetFullscreenDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetFullscreenDesc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHwnd: usize,
    pub GetCoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Present1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Present1: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTemporaryMonoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTemporaryMonoSupported: usize,
    pub GetRestrictToOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprestricttooutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *const DXGI_RGBA) -> ::windows::core::HRESULT,
    pub GetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetRotation: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRotation: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain2(::windows::core::IUnknown);
impl IDXGISwapChain2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Present)(::windows::core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetBuffer)(::windows::core::Interface::as_raw(self), buffer, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetFullscreenState)(::windows::core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFullscreenState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ResizeBuffers)(::windows::core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ResizeTarget)(::windows::core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.base__.GetContainingOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetLastPresentCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDesc1)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFullscreenDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.GetHwnd)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetCoreWindow)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.Present1)(::windows::core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsTemporaryMonoSupported)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.GetRestrictToOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBackgroundColor)(::windows::core::Interface::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__ = ::windows::core::zeroed::<DXGI_RGBA>();
        (::windows::core::Interface::vtable(self).base__.GetBackgroundColor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRotation)(::windows::core::Interface::as_raw(self), rotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::windows::core::zeroed::<Common::DXGI_MODE_ROTATION>();
        (::windows::core::Interface::vtable(self).base__.GetRotation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSourceSize)(::windows::core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSourceSize)(::windows::core::Interface::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).GetFrameLatencyWaitableObject)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixTransform)(::windows::core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMatrixTransform)(::windows::core::Interface::as_raw(self), pmatrix).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGISwapChain2, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1);
impl ::core::cmp::PartialEq for IDXGISwapChain2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain2 {}
impl ::core::fmt::Debug for IDXGISwapChain2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain2 {
    type Vtable = IDXGISwapChain2_Vtbl;
}
impl ::core::clone::Clone for IDXGISwapChain2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGISwapChain2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8be2ac4_199f_4946_b331_79599fb98de7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain2_Vtbl {
    pub base__: IDXGISwapChain1_Vtbl,
    pub SetSourceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT,
    pub GetSourceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFrameLatencyWaitableObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFrameLatencyWaitableObject: usize,
    pub SetMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT,
    pub GetMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain3(::windows::core::IUnknown);
impl IDXGISwapChain3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Present)(::windows::core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetBuffer)(::windows::core::Interface::as_raw(self), buffer, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetFullscreenState)(::windows::core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFullscreenState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ResizeBuffers)(::windows::core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ResizeTarget)(::windows::core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetContainingOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetLastPresentCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDesc1)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFullscreenDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.base__.GetHwnd)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetCoreWindow)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Present1)(::windows::core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.IsTemporaryMonoSupported)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.base__.GetRestrictToOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetBackgroundColor)(::windows::core::Interface::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__ = ::windows::core::zeroed::<DXGI_RGBA>();
        (::windows::core::Interface::vtable(self).base__.base__.GetBackgroundColor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetRotation)(::windows::core::Interface::as_raw(self), rotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::windows::core::zeroed::<Common::DXGI_MODE_ROTATION>();
        (::windows::core::Interface::vtable(self).base__.base__.GetRotation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSourceSize)(::windows::core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSourceSize)(::windows::core::Interface::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).base__.GetFrameLatencyWaitableObject)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMatrixTransform)(::windows::core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMatrixTransform)(::windows::core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetCurrentBackBufferIndex)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CheckColorSpaceSupport)(::windows::core::Interface::as_raw(self), colorspace, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorSpace1)(::windows::core::Interface::as_raw(self), colorspace).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResizeBuffers1)(::windows::core::Interface::as_raw(self), buffercount, width, height, format, swapchainflags, pcreationnodemask, ::core::mem::transmute(pppresentqueue)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGISwapChain3, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1, IDXGISwapChain2);
impl ::core::cmp::PartialEq for IDXGISwapChain3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain3 {}
impl ::core::fmt::Debug for IDXGISwapChain3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain3 {
    type Vtable = IDXGISwapChain3_Vtbl;
}
impl ::core::clone::Clone for IDXGISwapChain3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGISwapChain3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94d99bdb_f1f8_4ab0_b236_7da0170edab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain3_Vtbl {
    pub base__: IDXGISwapChain2_Vtbl,
    pub GetCurrentBackBufferIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckColorSpaceSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckColorSpaceSupport: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain4(::windows::core::IUnknown);
impl IDXGISwapChain4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Present)(::windows::core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetBuffer)(::windows::core::Interface::as_raw(self), buffer, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<IDXGIOutput>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetFullscreenState)(::windows::core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFullscreenState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ResizeBuffers)(::windows::core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ResizeTarget)(::windows::core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetContainingOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetLastPresentCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDesc1)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFullscreenDesc)(::windows::core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetHwnd)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetCoreWindow)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Present1)(::windows::core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.base__.base__.IsTemporaryMonoSupported)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::windows::core::zeroed::<IDXGIOutput>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetRestrictToOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetBackgroundColor)(::windows::core::Interface::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__ = ::windows::core::zeroed::<DXGI_RGBA>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetBackgroundColor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetRotation)(::windows::core::Interface::as_raw(self), rotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::windows::core::zeroed::<Common::DXGI_MODE_ROTATION>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetRotation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetSourceSize)(::windows::core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSourceSize)(::windows::core::Interface::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMaximumFrameLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).base__.base__.GetFrameLatencyWaitableObject)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetMatrixTransform)(::windows::core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetMatrixTransform)(::windows::core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetCurrentBackBufferIndex)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.CheckColorSpaceSupport)(::windows::core::Interface::as_raw(self), colorspace, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetColorSpace1)(::windows::core::Interface::as_raw(self), colorspace).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResizeBuffers1)(::windows::core::Interface::as_raw(self), buffercount, width, height, format, swapchainflags, pcreationnodemask, ::core::mem::transmute(pppresentqueue)).ok()
    }
    pub unsafe fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, pmetadata: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHDRMetaData)(::windows::core::Interface::as_raw(self), r#type, pmetadata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pmetadata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGISwapChain4, ::windows::core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3);
impl ::core::cmp::PartialEq for IDXGISwapChain4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain4 {}
impl ::core::fmt::Debug for IDXGISwapChain4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain4 {
    type Vtable = IDXGISwapChain4_Vtbl;
}
impl ::core::clone::Clone for IDXGISwapChain4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGISwapChain4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d585d5a_bd4a_489e_b1f4_3dbcb6452ffb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain4_Vtbl {
    pub base__: IDXGISwapChain3_Vtbl,
    pub SetHDRMetaData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChainMedia(::windows::core::IUnknown);
impl IDXGISwapChainMedia {
    pub unsafe fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameStatisticsMedia)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn SetPresentDuration(&self, duration: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPresentDuration)(::windows::core::Interface::as_raw(self), duration).ok()
    }
    pub unsafe fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckPresentDurationSupport)(::windows::core::Interface::as_raw(self), desiredpresentduration, pclosestsmallerpresentduration, pclosestlargerpresentduration).ok()
    }
}
::windows::imp::interface_hierarchy!(IDXGISwapChainMedia, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDXGISwapChainMedia {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChainMedia {}
impl ::core::fmt::Debug for IDXGISwapChainMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChainMedia").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChainMedia {
    type Vtable = IDXGISwapChainMedia_Vtbl;
}
impl ::core::clone::Clone for IDXGISwapChainMedia {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGISwapChainMedia {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd95b90b_f05f_4f6a_bd65_25bfb264bd84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChainMedia_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFrameStatisticsMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows::core::HRESULT,
    pub SetPresentDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: u32) -> ::windows::core::HRESULT,
    pub CheckPresentDurationSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGraphicsAnalysis(::windows::core::IUnknown);
impl IDXGraphicsAnalysis {
    pub unsafe fn BeginCapture(&self) {
        (::windows::core::Interface::vtable(self).BeginCapture)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn EndCapture(&self) {
        (::windows::core::Interface::vtable(self).EndCapture)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IDXGraphicsAnalysis, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDXGraphicsAnalysis {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGraphicsAnalysis {}
impl ::core::fmt::Debug for IDXGraphicsAnalysis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGraphicsAnalysis").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGraphicsAnalysis {
    type Vtable = IDXGraphicsAnalysis_Vtbl;
}
impl ::core::clone::Clone for IDXGraphicsAnalysis {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDXGraphicsAnalysis {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f251514_9d4d_4902_9d60_18988ab7d4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGraphicsAnalysis_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub EndCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_ALL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe48ae283_da80_490b_87e6_43e9a9cfda08);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_APP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06cd6e01_4219_4ebd_8709_27ed23360c62);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_DX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35cdd7fc_13b2_421d_a5d7_7e4451287d64);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_DXGI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25cddaa4_b1c6_47e1_ac3e_98875b5a2e2a);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_SCALING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_STEREO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_ACCESS_DENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270485i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_ACCESS_LOST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270490i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270474i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CACHE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270477i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CACHE_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270476i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CACHE_HASH_COLLISION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270475i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270486i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DEVICE_HUNG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270522i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DEVICE_REMOVED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270523i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DEVICE_RESET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270521i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270496i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270479i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270517i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270516i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270480i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_INVALID_CALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270527i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270491i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_MORE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270525i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270484i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NONEXCLUSIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270495i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NON_COMPOSITED_UI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270478i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NOT_CURRENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270482i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270494i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270526i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270493i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270492i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270487i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270483i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_SESSION_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270488i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270524i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_WAIT_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270489i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_WAS_STILL_DRAWING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270518i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAP_DISCARD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAP_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAP_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_VALID: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_RESTART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_TEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_USE_DURATION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_ADAPTER_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG_NONE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG_REMOTE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG_SOFTWARE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(2u32);
impl ::core::marker::Copy for DXGI_ADAPTER_FLAG {}
impl ::core::clone::Clone for DXGI_ADAPTER_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_ADAPTER_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_ADAPTER_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_ADAPTER_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_ADAPTER_FLAG").field(&self.0).finish()
    }
}
impl DXGI_ADAPTER_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_ADAPTER_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_ADAPTER_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_ADAPTER_FLAG3(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_NONE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_REMOTE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(4294967295u32);
impl ::core::marker::Copy for DXGI_ADAPTER_FLAG3 {}
impl ::core::clone::Clone for DXGI_ADAPTER_FLAG3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_ADAPTER_FLAG3 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_ADAPTER_FLAG3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_ADAPTER_FLAG3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_ADAPTER_FLAG3").field(&self.0).finish()
    }
}
impl DXGI_ADAPTER_FLAG3 {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_ADAPTER_FLAG3 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_ADAPTER_FLAG3 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_COMPUTE_PREEMPTION_GRANULARITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(4i32);
impl ::core::marker::Copy for DXGI_COMPUTE_PREEMPTION_GRANULARITY {}
impl ::core::clone::Clone for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_COMPUTE_PREEMPTION_GRANULARITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_DEBUG_RLO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(7i32);
impl ::core::marker::Copy for DXGI_DEBUG_RLO_FLAGS {}
impl ::core::clone::Clone for DXGI_DEBUG_RLO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_DEBUG_RLO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_DEBUG_RLO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_DEBUG_RLO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_DEBUG_RLO_FLAGS").field(&self.0).finish()
    }
}
impl DXGI_DEBUG_RLO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_FEATURE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: DXGI_FEATURE = DXGI_FEATURE(0i32);
impl ::core::marker::Copy for DXGI_FEATURE {}
impl ::core::clone::Clone for DXGI_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_FEATURE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FEATURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_FRAME_PRESENTATION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(3i32);
impl ::core::marker::Copy for DXGI_FRAME_PRESENTATION_MODE {}
impl ::core::clone::Clone for DXGI_FRAME_PRESENTATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_FRAME_PRESENTATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_FRAME_PRESENTATION_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_FRAME_PRESENTATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FRAME_PRESENTATION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_GPU_PREFERENCE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(2i32);
impl ::core::marker::Copy for DXGI_GPU_PREFERENCE {}
impl ::core::clone::Clone for DXGI_GPU_PREFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_GPU_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_GPU_PREFERENCE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_GPU_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_GPU_PREFERENCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_GRAPHICS_PREEMPTION_GRANULARITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(4i32);
impl ::core::marker::Copy for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {}
impl ::core::clone::Clone for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_GRAPHICS_PREEMPTION_GRANULARITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(4i32);
impl ::core::marker::Copy for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {}
impl ::core::clone::Clone for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_HDR_METADATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HDR_METADATA_TYPE_NONE: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HDR_METADATA_TYPE_HDR10: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(2i32);
impl ::core::marker::Copy for DXGI_HDR_METADATA_TYPE {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_HDR_METADATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_HDR_METADATA_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_HDR_METADATA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_INFO_QUEUE_MESSAGE_CATEGORY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(10i32);
impl ::core::marker::Copy for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_INFO_QUEUE_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_INFO_QUEUE_MESSAGE_SEVERITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(4i32);
impl ::core::marker::Copy for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_INFO_QUEUE_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_MEMORY_SEGMENT_GROUP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = DXGI_MEMORY_SEGMENT_GROUP(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = DXGI_MEMORY_SEGMENT_GROUP(1i32);
impl ::core::marker::Copy for DXGI_MEMORY_SEGMENT_GROUP {}
impl ::core::clone::Clone for DXGI_MEMORY_SEGMENT_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_MEMORY_SEGMENT_GROUP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_MEMORY_SEGMENT_GROUP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_MEMORY_SEGMENT_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MEMORY_SEGMENT_GROUP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(4i32);
impl ::core::marker::Copy for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {}
impl ::core::clone::Clone for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_Message_Id(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_InvalidOutputWindow: DXGI_Message_Id = DXGI_Message_Id(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferWidthInferred: DXGI_Message_Id = DXGI_Message_Id(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferHeightInferred: DXGI_Message_Id = DXGI_Message_Id(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_NoScanoutFlagChanged: DXGI_Message_Id = DXGI_Message_Id(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_MaxBufferCountExceeded: DXGI_Message_Id = DXGI_Message_Id(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_TooFewBuffers: DXGI_Message_Id = DXGI_Message_Id(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_NoOutputWindow: DXGI_Message_Id = DXGI_Message_Id(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Destruction_OtherMethodsCalled: DXGI_Message_Id = DXGI_Message_Id(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetDesc_pDescIsNULL: DXGI_Message_Id = DXGI_Message_Id(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_ppSurfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_NoAllocatedBuffers: DXGI_Message_Id = DXGI_Message_Id(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferMustBeZero: DXGI_Message_Id = DXGI_Message_Id(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferOOB: DXGI_Message_Id = DXGI_Message_Id(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_ppOutputIsNULL: DXGI_Message_Id = DXGI_Message_Id(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_SyncIntervalOOB: DXGI_Message_Id = DXGI_Message_Id(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidNonPreRotatedFlag: DXGI_Message_Id = DXGI_Message_Id(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_NoAllocatedBuffers: DXGI_Message_Id = DXGI_Message_Id(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_GetDXGIAdapterFailed: DXGI_Message_Id = DXGI_Message_Id(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOB: DXGI_Message_Id = DXGI_Message_Id(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_UnreleasedReferences: DXGI_Message_Id = DXGI_Message_Id(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidSwapChainFlag: DXGI_Message_Id = DXGI_Message_Id(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidNonPreRotatedFlag: DXGI_Message_Id = DXGI_Message_Id(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_RefreshRateDivideByZero: DXGI_Message_Id = DXGI_Message_Id(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_InvalidTarget: DXGI_Message_Id = DXGI_Message_Id(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = DXGI_Message_Id(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetLastPresentCount_pLastPresentCountIsNULL: DXGI_Message_Id = DXGI_Message_Id(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_RemoteNotSupported: DXGI_Message_Id = DXGI_Message_Id(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_FailedToAcquireFullscreenMutex: DXGI_Message_Id = DXGI_Message_Id(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ppAdapterInterfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_EnumAdapters_ppAdapterInterfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ppSwapChainIsNULL: DXGI_Message_Id = DXGI_Message_Id(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDescIsNULL: DXGI_Message_Id = DXGI_Message_Id(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnknownSwapEffect: DXGI_Message_Id = DXGI_Message_Id(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedFlagAndWindowed: DXGI_Message_Id = DXGI_Message_Id(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NullDeviceInterface: DXGI_Message_Id = DXGI_Message_Id(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_phWndIsNULL: DXGI_Message_Id = DXGI_Message_Id(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_InvalidSurface: DXGI_Message_Id = DXGI_Message_Id(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_FlagsSetToZero: DXGI_Message_Id = DXGI_Message_Id(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_DiscardAndReadFlagSet: DXGI_Message_Id = DXGI_Message_Id(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_DiscardButNotWriteFlagSet: DXGI_Message_Id = DXGI_Message_Id(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess: DXGI_Message_Id = DXGI_Message_Id(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_ReadFlagSetButCPUAccessIsDynamic: DXGI_Message_Id = DXGI_Message_Id(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_DiscardFlagSetButCPUAccessIsNotDynamic: DXGI_Message_Id = DXGI_Message_Id(44i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_pNumModesIsNULL: DXGI_Message_Id = DXGI_Message_Id(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasInvalidWidthOrHeight: DXGI_Message_Id = DXGI_Message_Id(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetCammaControlCapabilities_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_pDeviceIsNULL: DXGI_Message_Id = DXGI_Message_Id(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(49i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_pDestinationIsNULL: DXGI_Message_Id = DXGI_Message_Id(50i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_MapOfDestinationFailed: DXGI_Message_Id = DXGI_Message_Id(51i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(52i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = DXGI_Message_Id(53i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetGammaControl_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(54i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(55i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoGammaControls: DXGI_Message_Id = DXGI_Message_Id(56i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_IDXGIResourceNotSupportedBypPrimary: DXGI_Message_Id = DXGI_Message_Id(57i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_pPrimaryIsInvalid: DXGI_Message_Id = DXGI_Message_Id(58i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(59i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(60i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(61i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(62i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDevice_CreateSurface_InvalidParametersWithpSharedResource: DXGI_Message_Id = DXGI_Message_Id(63i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIObject_GetPrivateData_puiDataSizeIsNULL: DXGI_Message_Id = DXGI_Message_Id(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidOutputWindow: DXGI_Message_Id = DXGI_Message_Id(65i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Release_SwapChainIsFullscreen: DXGI_Message_Id = DXGI_Message_Id(66i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_InvalidTargetSurfaceFormat: DXGI_Message_Id = DXGI_Message_Id(67i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ModuleIsNULL: DXGI_Message_Id = DXGI_Message_Id(68i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(69i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_pModeToMatchOrpClosestMatchIsNULL: DXGI_Message_Id = DXGI_Message_Id(70i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasRefreshRateDenominatorZero: DXGI_Message_Id = DXGI_Message_Id(71i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_UnknownFormatIsInvalidForConfiguration: DXGI_Message_Id = DXGI_Message_Id(72i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScanlineOrdering: DXGI_Message_Id = DXGI_Message_Id(73i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScaling: DXGI_Message_Id = DXGI_Message_Id(74i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeFormatAndDeviceCombination: DXGI_Message_Id = DXGI_Message_Id(75i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_Creation_CalledFromDllMain: DXGI_Message_Id = DXGI_Message_Id(76i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_OutputNotOwnedBySwapChainDevice: DXGI_Message_Id = DXGI_Message_Id(77i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidWindowStyle: DXGI_Message_Id = DXGI_Message_Id(78i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_UnsupportedStatistics: DXGI_Message_Id = DXGI_Message_Id(79i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_SwapchainAdapterDoesNotControlOutput: DXGI_Message_Id = DXGI_Message_Id(80i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetOrGetGammaControl_pArrayIsNULL: DXGI_Message_Id = DXGI_Message_Id(81i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FullscreenInvalidForChildWindows: DXGI_Message_Id = DXGI_Message_Id(82i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_Release_CalledFromDllMain: DXGI_Message_Id = DXGI_Message_Id(83i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(84i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = DXGI_Message_Id(85i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = DXGI_Message_Id(86i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_pHdcIsNULL: DXGI_Message_Id = DXGI_Message_Id(87i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_SurfaceNotTexture2D: DXGI_Message_Id = DXGI_Message_Id(88i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_GDICompatibleFlagNotSet: DXGI_Message_Id = DXGI_Message_Id(89i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(90i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess2: DXGI_Message_Id = DXGI_Message_Id(91i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_GetDCNotCalled: DXGI_Message_Id = DXGI_Message_Id(92i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_InvalidRectangleDimensions: DXGI_Message_Id = DXGI_Message_Id(93i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(94i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(95i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(96i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDeviceHasMismatchedDXGIFactory: DXGI_Message_Id = DXGI_Message_Id(97i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_NonOptimalFSConfiguration: DXGI_Message_Id = DXGI_Message_Id(98i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSequentialNotSupportedOnD3D10: DXGI_Message_Id = DXGI_Message_Id(99i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_BufferCountOOBForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(100i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFormatForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(101i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultiSamplingNotSupportedForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(102i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOBForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(103i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidFormatForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(104i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationBeforeStandardPresentation: DXGI_Message_Id = DXGI_Message_Id(105i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenPartialPresentIsInvalid: DXGI_Message_Id = DXGI_Message_Id(106i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidPresentTestOrDoNotSequenceFlag: DXGI_Message_Id = DXGI_Message_Id(107i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollInfoWithNoDirtyRectsSpecified: DXGI_Message_Id = DXGI_Message_Id(108i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyScrollRect: DXGI_Message_Id = DXGI_Message_Id(109i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBounds: DXGI_Message_Id = DXGI_Message_Id(110i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBoundsWithOffset: DXGI_Message_Id = DXGI_Message_Id(111i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyDirtyRect: DXGI_Message_Id = DXGI_Message_Id(112i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_DirtyRectOutOfBackbufferBounds: DXGI_Message_Id = DXGI_Message_Id(113i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnsupportedBufferUsageFlags: DXGI_Message_Id = DXGI_Message_Id(114i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_DoNotSequenceFlagSetButPreviousBufferIsUndefined: DXGI_Message_Id = DXGI_Message_Id(115i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_UnsupportedFlags: DXGI_Message_Id = DXGI_Message_Id(116i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_FlipModelChainMustResizeOrCreateOnFSTransition: DXGI_Message_Id = DXGI_Message_Id(117i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pRestrictToOutputFromOtherIDXGIFactory: DXGI_Message_Id = DXGI_Message_Id(118i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictOutputNotSupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(119i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagSetButInvalidpRestrictToOutput: DXGI_Message_Id = DXGI_Message_Id(120i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagdWithFullscreen: DXGI_Message_Id = DXGI_Message_Id(121i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictOutputFlagWithStaleSwapChain: DXGI_Message_Id = DXGI_Message_Id(122i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_OtherFlagsCausingInvalidPresentTestFlag: DXGI_Message_Id = DXGI_Message_Id(123i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(124i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(125i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(126i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(127i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_StereoDisabled: DXGI_Message_Id = DXGI_Message_Id(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_UnregisterStatus_CookieNotFound: DXGI_Message_Id = DXGI_Message_Id(129i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFSOrOverlay: DXGI_Message_Id = DXGI_Message_Id(130i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFlipSequential: DXGI_Message_Id = DXGI_Message_Id(131i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentWithRDPDriver: DXGI_Message_Id = DXGI_Message_Id(132i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity: DXGI_Message_Id = DXGI_Message_Id(133i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_WidthOrHeightIsZero: DXGI_Message_Id = DXGI_Message_Id(134i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_OnlyFlipSequentialSupported: DXGI_Message_Id = DXGI_Message_Id(135i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(136i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnWindows7: DXGI_Message_Id = DXGI_Message_Id(137i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSTransitionWithCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(138i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_InvalidWithCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(139i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_WidthOrHeightIsZero: DXGI_Message_Id = DXGI_Message_Id(140i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(141i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingUnrecognized: DXGI_Message_Id = DXGI_Message_Id(142i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = DXGI_Message_Id(143i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(144i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestartIsFullscreenOnly: DXGI_Message_Id = DXGI_Message_Id(145i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedWindowlessPresentationRequiresDisplayOnly: DXGI_Message_Id = DXGI_Message_Id(146i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(147i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetBackgroundColor_OutOfRange: DXGI_Message_Id = DXGI_Message_Id(148i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = DXGI_Message_Id(149i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(150i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapchain_Present_ScrollUnsupported: DXGI_Message_Id = DXGI_Message_Id(151i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(152i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(153i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapchain_Present_FullscreenRotation: DXGI_Message_Id = DXGI_Message_Id(154i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithMSAABuffers: DXGI_Message_Id = DXGI_Message_Id(155i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(156i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_InvalidRotation: DXGI_Message_Id = DXGI_Message_Id(157i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(158i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetHwnd_WrongType: DXGI_Message_Id = DXGI_Message_Id(159i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetCompositionSurface_WrongType: DXGI_Message_Id = DXGI_Message_Id(160i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetCoreWindow_WrongType: DXGI_Message_Id = DXGI_Message_Id(161i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFullscreenDesc_NonHwnd: DXGI_Message_Id = DXGI_Message_Id(162i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_CoreWindow: DXGI_Message_Id = DXGI_Message_Id(163i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_UnsupportedOnWindows7: DXGI_Message_Id = DXGI_Message_Id(164i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsNULL: DXGI_Message_Id = DXGI_Message_Id(165i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FSUnsupportedForModernApps: DXGI_Message_Id = DXGI_Message_Id(166i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_ModernApp: DXGI_Message_Id = DXGI_Message_Id(167i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_ModernApp: DXGI_Message_Id = DXGI_Message_Id(168i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_pNewTargetParametersIsNULL: DXGI_Message_Id = DXGI_Message_Id(169i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_ModernApp: DXGI_Message_Id = DXGI_Message_Id(170i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_ModernApp: DXGI_Message_Id = DXGI_Message_Id(171i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsInvalid: DXGI_Message_Id = DXGI_Message_Id(172i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCompositionSurface_InvalidHandle: DXGI_Message_Id = DXGI_Message_Id(173i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_ModernApp: DXGI_Message_Id = DXGI_Message_Id(174i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneRequiresWindows8OrNewer: DXGI_Message_Id = DXGI_Message_Id(175i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoAndPreferRight: DXGI_Message_Id = DXGI_Message_Id(176i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithDoNotSequence: DXGI_Message_Id = DXGI_Message_Id(177i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithoutStereo: DXGI_Message_Id = DXGI_Message_Id(178i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoUnsupported: DXGI_Message_Id = DXGI_Message_Id(179i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_ArraySizeMismatch: DXGI_Message_Id = DXGI_Message_Id(180i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithSwapEffectDiscard: DXGI_Message_Id = DXGI_Message_Id(181i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaUnrecognized: DXGI_Message_Id = DXGI_Message_Id(182i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsWindowlessOnly: DXGI_Message_Id = DXGI_Message_Id(183i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(184i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictToOutputAdapterMismatch: DXGI_Message_Id = DXGI_Message_Id(185i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyOnLegacy: DXGI_Message_Id = DXGI_Message_Id(186i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyOnLegacy: DXGI_Message_Id = DXGI_Message_Id(187i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIResource1_CreateSubresourceSurface_InvalidIndex: DXGI_Message_Id = DXGI_Message_Id(188i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(189i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForCoreWindow_InvalidSwapEffect: DXGI_Message_Id = DXGI_Message_Id(190i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIResource1_CreateSharedHandle_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(191i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusWindow_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(192i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusEvent_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(193i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_DuplicateOutput_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(194i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDisplayControl_IsStereoEnabled_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(195i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidAlphaMode: DXGI_Message_Id = DXGI_Message_Id(196i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidResource: DXGI_Message_Id = DXGI_Message_Id(197i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidLUID: DXGI_Message_Id = DXGI_Message_Id(198i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(199i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_2DOnly: DXGI_Message_Id = DXGI_Message_Id(200i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_StagingOnly: DXGI_Message_Id = DXGI_Message_Id(201i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NeedCPUAccessWrite: DXGI_Message_Id = DXGI_Message_Id(202i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NoShared: DXGI_Message_Id = DXGI_Message_Id(203i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_OnlyMipLevels1: DXGI_Message_Id = DXGI_Message_Id(204i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_MappedOrOfferedResource: DXGI_Message_Id = DXGI_Message_Id(205i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSUnsupportedForModernApps: DXGI_Message_Id = DXGI_Message_Id(206i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FailedToGoFSButNonPreRotated: DXGI_Message_Id = DXGI_Message_Id(207i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainOrRegisterOcclusionStatus_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = DXGI_Message_Id(208i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = DXGI_Message_Id(209i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(210i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreNotFullscreen: DXGI_Message_Id = DXGI_Message_Id(211i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_Waitable: DXGI_Message_Id = DXGI_Message_Id(212i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveWaitableFlag: DXGI_Message_Id = DXGI_Message_Id(213i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFrameLatencyWaitableObject_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(214i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(215i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_pMaxLatencyIsNULL: DXGI_Message_Id = DXGI_Message_Id(216i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(217i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_MaxLatencyIsOutOfBounds: DXGI_Message_Id = DXGI_Message_Id(218i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ForegroundIsCoreWindowOnly: DXGI_Message_Id = DXGI_Message_Id(219i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_ForegroundUnsupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(220i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(221i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidAlphaMode: DXGI_Message_Id = DXGI_Message_Id(222i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveForegroundFlag: DXGI_Message_Id = DXGI_Message_Id(223i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = DXGI_Message_Id(224i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(225i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeFinite: DXGI_Message_Id = DXGI_Message_Id(226i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeTranslateAndOrScale: DXGI_Message_Id = DXGI_Message_Id(227i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = DXGI_Message_Id(228i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(229i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_DXGIGetDebugInterface1_NULL_ppDebug: DXGI_Message_Id = DXGI_Message_Id(230i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_DXGIGetDebugInterface1_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(231i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_Decode: DXGI_Message_Id = DXGI_Message_Id(232i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Decode: DXGI_Message_Id = DXGI_Message_Id(233i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_FlipModel: DXGI_Message_Id = DXGI_Message_Id(234i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(235i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_WidthHeight: DXGI_Message_Id = DXGI_Message_Id(236i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_NullPointers: DXGI_Message_Id = DXGI_Message_Id(237i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(238i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetColorSpace_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(239i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetSourceRect_InvalidRect: DXGI_Message_Id = DXGI_Message_Id(240i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetTargetRect_InvalidRect: DXGI_Message_Id = DXGI_Message_Id(241i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetDestSize_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(242i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetSourceRect_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(243i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetTargetRect_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(244i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetDestSize_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(245i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_PresentBuffer_YUV: DXGI_Message_Id = DXGI_Message_Id(246i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(247i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(248i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(249i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(250i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentation_YUV: DXGI_Message_Id = DXGI_Message_Id(251i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveFlag_YUV: DXGI_Message_Id = DXGI_Message_Id(252i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Alignment_YUV: DXGI_Message_Id = DXGI_Message_Id(253i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ShaderInputUnsupported_YUV: DXGI_Message_Id = DXGI_Message_Id(254i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(255i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs2_InvalidEnumOutputs2Flag: DXGI_Message_Id = DXGI_Message_Id(257i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_FSUnsupportedForFlipDiscard: DXGI_Message_Id = DXGI_Message_Id(258i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(259i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(260i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_CheckColorSpaceSupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(261i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetColorSpace1_InvalidColorSpace: DXGI_Message_Id = DXGI_Message_Id(262i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidHwProtect: DXGI_Message_Id = DXGI_Message_Id(263i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_HwProtectUnsupported: DXGI_Message_Id = DXGI_Message_Id(264i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtect: DXGI_Message_Id = DXGI_Message_Id(265i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_HwProtectUnsupported: DXGI_Message_Id = DXGI_Message_Id(266i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_D3D12Only: DXGI_Message_Id = DXGI_Message_Id(267i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_FlipModel: DXGI_Message_Id = DXGI_Message_Id(268i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_NodeMaskAndQueueRequired: DXGI_Message_Id = DXGI_Message_Id(269i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreateSwapChain_InvalidHwProtectGdiFlag: DXGI_Message_Id = DXGI_Message_Id(270i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtectGdiFlag: DXGI_Message_Id = DXGI_Message_Id(271i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_10BitFormatNotSupported: DXGI_Message_Id = DXGI_Message_Id(272i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSwapEffectRequired: DXGI_Message_Id = DXGI_Message_Id(273i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidDevice: DXGI_Message_Id = DXGI_Message_Id(274i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_Unsupported: DXGI_Message_Id = DXGI_Message_Id(275i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidQueue: DXGI_Message_Id = DXGI_Message_Id(276i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_ResizeBuffers1_InvalidQueue: DXGI_Message_Id = DXGI_Message_Id(277i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForHwnd_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(278i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(279i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(280i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidType: DXGI_Message_Id = DXGI_Message_Id(281i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenAllowTearingIsInvalid: DXGI_Message_Id = DXGI_Message_Id(282i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresPresentIntervalZero: DXGI_Message_Id = DXGI_Message_Id(283i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresCreationFlag: DXGI_Message_Id = DXGI_Message_Id(284i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveAllowTearingFlag: DXGI_Message_Id = DXGI_Message_Id(285i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AllowTearingFlagIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(286i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidFeature: DXGI_Message_Id = DXGI_Message_Id(287i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(288i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput6_CheckHardwareCompositionSupport_NullPointer: DXGI_Message_Id = DXGI_Message_Id(289i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_PerMonitorDpiShimApplied: DXGI_Message_Id = DXGI_Message_Id(290i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput_PerMonitorDpiShimApplied: DXGI_Message_Id = DXGI_Message_Id(291i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput1_PerMonitorDpiRequired: DXGI_Message_Id = DXGI_Message_Id(292i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory7_UnregisterAdaptersChangedEvent_CookieNotFound: DXGI_Message_Id = DXGI_Message_Id(293i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_LegacyBltModelSwapEffect: DXGI_Message_Id = DXGI_Message_Id(294i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain4_SetHDRMetaData_MetadataUnchanged: DXGI_Message_Id = DXGI_Message_Id(295i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_11On12_Released_Resource: DXGI_Message_Id = DXGI_Message_Id(296i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultipleSwapchainRefToSurface_DeferredDtr: DXGI_Message_Id = DXGI_Message_Id(297i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_NoOpBehavior: DXGI_Message_Id = DXGI_Message_Id(298i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow: DXGI_Message_Id = DXGI_Message_Id(1000i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_DISCARD_BufferCount: DXGI_Message_Id = DXGI_Message_Id(1001i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_SetFullscreenState_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1002i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeBuffers_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1003i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeTarget_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1004i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerIndex: DXGI_Message_Id = DXGI_Message_Id(1005i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleLayerIndex: DXGI_Message_Id = DXGI_Message_Id(1006i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerFlag: DXGI_Message_Id = DXGI_Message_Id(1007i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidRotation: DXGI_Message_Id = DXGI_Message_Id(1008i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidBlend: DXGI_Message_Id = DXGI_Message_Id(1009i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidResource: DXGI_Message_Id = DXGI_Message_Id(1010i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidMultiPlaneOverlayResource: DXGI_Message_Id = DXGI_Message_Id(1011i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForPrimary: DXGI_Message_Id = DXGI_Message_Id(1012i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForOverlay: DXGI_Message_Id = DXGI_Message_Id(1013i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSubResourceIndex: DXGI_Message_Id = DXGI_Message_Id(1014i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSourceRect: DXGI_Message_Id = DXGI_Message_Id(1015i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidDestinationRect: DXGI_Message_Id = DXGI_Message_Id(1016i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleResource: DXGI_Message_Id = DXGI_Message_Id(1017i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_NotSharedResource: DXGI_Message_Id = DXGI_Message_Id(1018i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidFlag: DXGI_Message_Id = DXGI_Message_Id(1019i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidInterval: DXGI_Message_Id = DXGI_Message_Id(1020i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_MSAA_NotSupported: DXGI_Message_Id = DXGI_Message_Id(1021i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_ScalingAspectRatioStretch_Supported_ModernApp: DXGI_Message_Id = DXGI_Message_Id(1022i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_GetFrameStatistics_NotAvailable_ModernApp: DXGI_Message_Id = DXGI_Message_Id(1023i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_ReplaceInterval0With1: DXGI_Message_Id = DXGI_Message_Id(1024i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FailedRegisterWithCompositor: DXGI_Message_Id = DXGI_Message_Id(1025i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow_AtRendering: DXGI_Message_Id = DXGI_Message_Id(1026i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_SEQUENTIAL_BufferCount: DXGI_Message_Id = DXGI_Message_Id(1027i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_Modern_CoreWindow_Only: DXGI_Message_Id = DXGI_Message_Id(1028i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present1_RequiresOverlays: DXGI_Message_Id = DXGI_Message_Id(1029i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_SetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(1030i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_GetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(1031i32);
impl ::core::marker::Copy for DXGI_Message_Id {}
impl ::core::clone::Clone for DXGI_Message_Id {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_Message_Id {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_Message_Id {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_Message_Id {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_Message_Id").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_OFFER_RESOURCE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS = DXGI_OFFER_RESOURCE_FLAGS(1i32);
impl ::core::marker::Copy for DXGI_OFFER_RESOURCE_FLAGS {}
impl ::core::clone::Clone for DXGI_OFFER_RESOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OFFER_RESOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_OFFER_RESOURCE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OFFER_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OFFER_RESOURCE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_OFFER_RESOURCE_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(3i32);
impl ::core::marker::Copy for DXGI_OFFER_RESOURCE_PRIORITY {}
impl ::core::clone::Clone for DXGI_OFFER_RESOURCE_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OFFER_RESOURCE_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_OFFER_RESOURCE_PRIORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OFFER_RESOURCE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OFFER_RESOURCE_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_OUTDUPL_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = DXGI_OUTDUPL_FLAG(1i32);
impl ::core::marker::Copy for DXGI_OUTDUPL_FLAG {}
impl ::core::clone::Clone for DXGI_OUTDUPL_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OUTDUPL_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_OUTDUPL_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OUTDUPL_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(4i32);
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {}
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OUTDUPL_POINTER_SHAPE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(1i32);
impl ::core::marker::Copy for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {}
impl ::core::clone::Clone for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_OVERLAY_SUPPORT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: DXGI_OVERLAY_SUPPORT_FLAG = DXGI_OVERLAY_SUPPORT_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: DXGI_OVERLAY_SUPPORT_FLAG = DXGI_OVERLAY_SUPPORT_FLAG(2i32);
impl ::core::marker::Copy for DXGI_OVERLAY_SUPPORT_FLAG {}
impl ::core::clone::Clone for DXGI_OVERLAY_SUPPORT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OVERLAY_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_OVERLAY_SUPPORT_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OVERLAY_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OVERLAY_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_RECLAIM_RESOURCE_RESULTS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(2i32);
impl ::core::marker::Copy for DXGI_RECLAIM_RESOURCE_RESULTS {}
impl ::core::clone::Clone for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_RECLAIM_RESOURCE_RESULTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RECLAIM_RESOURCE_RESULTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_RESIDENCY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESIDENCY_FULLY_RESIDENT: DXGI_RESIDENCY = DXGI_RESIDENCY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: DXGI_RESIDENCY = DXGI_RESIDENCY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: DXGI_RESIDENCY = DXGI_RESIDENCY(3i32);
impl ::core::marker::Copy for DXGI_RESIDENCY {}
impl ::core::clone::Clone for DXGI_RESIDENCY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_RESIDENCY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_RESIDENCY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_RESIDENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RESIDENCY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_RESOURCE_PRIORITY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(671088640u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_LOW: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(1342177280u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_NORMAL: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(2013265920u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_HIGH: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(2684354560u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(3355443200u32);
impl ::core::marker::Copy for DXGI_RESOURCE_PRIORITY {}
impl ::core::clone::Clone for DXGI_RESOURCE_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_RESOURCE_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_RESOURCE_PRIORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_RESOURCE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RESOURCE_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_SCALING(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = DXGI_SCALING(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SCALING_NONE: DXGI_SCALING = DXGI_SCALING(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = DXGI_SCALING(2i32);
impl ::core::marker::Copy for DXGI_SCALING {}
impl ::core::clone::Clone for DXGI_SCALING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_SCALING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_SCALING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_SCALING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SCALING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(2i32);
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {}
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_SWAP_CHAIN_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(512i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1024i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2048i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(4096i32);
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_FLAG {}
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_SWAP_CHAIN_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_SWAP_CHAIN_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_CHAIN_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_SWAP_EFFECT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(4i32);
impl ::core::marker::Copy for DXGI_SWAP_EFFECT {}
impl ::core::clone::Clone for DXGI_SWAP_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_SWAP_EFFECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_SWAP_EFFECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_SWAP_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_EFFECT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_USAGE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_SHADER_INPUT: DXGI_USAGE = DXGI_USAGE(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: DXGI_USAGE = DXGI_USAGE(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_BACK_BUFFER: DXGI_USAGE = DXGI_USAGE(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_SHARED: DXGI_USAGE = DXGI_USAGE(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_READ_ONLY: DXGI_USAGE = DXGI_USAGE(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_DISCARD_ON_PRESENT: DXGI_USAGE = DXGI_USAGE(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_UNORDERED_ACCESS: DXGI_USAGE = DXGI_USAGE(1024u32);
impl ::core::marker::Copy for DXGI_USAGE {}
impl ::core::clone::Clone for DXGI_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DXGI_USAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DXGI_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_USAGE").field(&self.0).finish()
    }
}
impl DXGI_USAGE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_USAGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_USAGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_USAGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_USAGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC").field("Description", &self.Description).field("VendorId", &self.VendorId).field("DeviceId", &self.DeviceId).field("SubSysId", &self.SubSysId).field("Revision", &self.Revision).field("DedicatedVideoMemory", &self.DedicatedVideoMemory).field("DedicatedSystemMemory", &self.DedicatedSystemMemory).field("SharedSystemMemory", &self.SharedSystemMemory).field("AdapterLuid", &self.AdapterLuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_ADAPTER_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC1").field("Description", &self.Description).field("VendorId", &self.VendorId).field("DeviceId", &self.DeviceId).field("SubSysId", &self.SubSysId).field("Revision", &self.Revision).field("DedicatedVideoMemory", &self.DedicatedVideoMemory).field("DedicatedSystemMemory", &self.DedicatedSystemMemory).field("SharedSystemMemory", &self.SharedSystemMemory).field("AdapterLuid", &self.AdapterLuid).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_ADAPTER_DESC1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC2")
            .field("Description", &self.Description)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("SubSysId", &self.SubSysId)
            .field("Revision", &self.Revision)
            .field("DedicatedVideoMemory", &self.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.SharedSystemMemory)
            .field("AdapterLuid", &self.AdapterLuid)
            .field("Flags", &self.Flags)
            .field("GraphicsPreemptionGranularity", &self.GraphicsPreemptionGranularity)
            .field("ComputePreemptionGranularity", &self.ComputePreemptionGranularity)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_ADAPTER_DESC2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC2 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid && self.Flags == other.Flags && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC3")
            .field("Description", &self.Description)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("SubSysId", &self.SubSysId)
            .field("Revision", &self.Revision)
            .field("DedicatedVideoMemory", &self.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.SharedSystemMemory)
            .field("AdapterLuid", &self.AdapterLuid)
            .field("Flags", &self.Flags)
            .field("GraphicsPreemptionGranularity", &self.GraphicsPreemptionGranularity)
            .field("ComputePreemptionGranularity", &self.ComputePreemptionGranularity)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_ADAPTER_DESC3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC3 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid && self.Flags == other.Flags && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
impl ::core::marker::Copy for DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::core::clone::Clone for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_DECODE_SWAP_CHAIN_DESC").field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_DECODE_SWAP_CHAIN_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::core::default::Default for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [f32; 16],
    pub WhitePoints: [f32; 32],
}
impl ::core::marker::Copy for DXGI_DISPLAY_COLOR_SPACE {}
impl ::core::clone::Clone for DXGI_DISPLAY_COLOR_SPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_DISPLAY_COLOR_SPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_DISPLAY_COLOR_SPACE").field("PrimaryCoordinates", &self.PrimaryCoordinates).field("WhitePoints", &self.WhitePoints).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_DISPLAY_COLOR_SPACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_DISPLAY_COLOR_SPACE {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryCoordinates == other.PrimaryCoordinates && self.WhitePoints == other.WhitePoints
    }
}
impl ::core::cmp::Eq for DXGI_DISPLAY_COLOR_SPACE {}
impl ::core::default::Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_FRAME_STATISTICS").field("PresentCount", &self.PresentCount).field("PresentRefreshCount", &self.PresentRefreshCount).field("SyncRefreshCount", &self.SyncRefreshCount).field("SyncQPCTime", &self.SyncQPCTime).field("SyncGPUTime", &self.SyncGPUTime).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_FRAME_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount && self.PresentRefreshCount == other.PresentRefreshCount && self.SyncRefreshCount == other.SyncRefreshCount && self.SyncQPCTime == other.SyncQPCTime && self.SyncGPUTime == other.SyncGPUTime
    }
}
impl ::core::cmp::Eq for DXGI_FRAME_STATISTICS {}
impl ::core::default::Default for DXGI_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS_MEDIA {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS_MEDIA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_FRAME_STATISTICS_MEDIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_FRAME_STATISTICS_MEDIA").field("PresentCount", &self.PresentCount).field("PresentRefreshCount", &self.PresentRefreshCount).field("SyncRefreshCount", &self.SyncRefreshCount).field("SyncQPCTime", &self.SyncQPCTime).field("SyncGPUTime", &self.SyncGPUTime).field("CompositionMode", &self.CompositionMode).field("ApprovedPresentDuration", &self.ApprovedPresentDuration).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_FRAME_STATISTICS_MEDIA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_FRAME_STATISTICS_MEDIA {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount && self.PresentRefreshCount == other.PresentRefreshCount && self.SyncRefreshCount == other.SyncRefreshCount && self.SyncQPCTime == other.SyncQPCTime && self.SyncGPUTime == other.SyncGPUTime && self.CompositionMode == other.CompositionMode && self.ApprovedPresentDuration == other.ApprovedPresentDuration
    }
}
impl ::core::cmp::Eq for DXGI_FRAME_STATISTICS_MEDIA {}
impl ::core::default::Default for DXGI_FRAME_STATISTICS_MEDIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10 {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_HDR10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_HDR_METADATA_HDR10").field("RedPrimary", &self.RedPrimary).field("GreenPrimary", &self.GreenPrimary).field("BluePrimary", &self.BluePrimary).field("WhitePoint", &self.WhitePoint).field("MaxMasteringLuminance", &self.MaxMasteringLuminance).field("MinMasteringLuminance", &self.MinMasteringLuminance).field("MaxContentLightLevel", &self.MaxContentLightLevel).field("MaxFrameAverageLightLevel", &self.MaxFrameAverageLightLevel).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_HDR_METADATA_HDR10 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_HDR_METADATA_HDR10 {
    fn eq(&self, other: &Self) -> bool {
        self.RedPrimary == other.RedPrimary && self.GreenPrimary == other.GreenPrimary && self.BluePrimary == other.BluePrimary && self.WhitePoint == other.WhitePoint && self.MaxMasteringLuminance == other.MaxMasteringLuminance && self.MinMasteringLuminance == other.MinMasteringLuminance && self.MaxContentLightLevel == other.MaxContentLightLevel && self.MaxFrameAverageLightLevel == other.MaxFrameAverageLightLevel
    }
}
impl ::core::cmp::Eq for DXGI_HDR_METADATA_HDR10 {}
impl ::core::default::Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10PLUS {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10PLUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_HDR10PLUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_HDR_METADATA_HDR10PLUS").field("Data", &self.Data).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_HDR_METADATA_HDR10PLUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_HDR_METADATA_HDR10PLUS {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DXGI_HDR_METADATA_HDR10PLUS {}
impl ::core::default::Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_FILTER").field("AllowList", &self.AllowList).field("DenyList", &self.DenyList).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_INFO_QUEUE_FILTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.AllowList == other.AllowList && self.DenyList == other.DenyList
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_FILTER {}
impl ::core::default::Default for DXGI_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut i32,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_FILTER_DESC").field("NumCategories", &self.NumCategories).field("pCategoryList", &self.pCategoryList).field("NumSeverities", &self.NumSeverities).field("pSeverityList", &self.pSeverityList).field("NumIDs", &self.NumIDs).field("pIDList", &self.pIDList).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_INFO_QUEUE_FILTER_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumCategories == other.NumCategories && self.pCategoryList == other.pCategoryList && self.NumSeverities == other.NumSeverities && self.pSeverityList == other.pSeverityList && self.NumIDs == other.NumIDs && self.pIDList == other.pIDList
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::core::default::Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: ::windows::core::GUID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: i32,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_MESSAGE {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_MESSAGE").field("Producer", &self.Producer).field("Category", &self.Category).field("Severity", &self.Severity).field("ID", &self.ID).field("pDescription", &self.pDescription).field("DescriptionByteLength", &self.DescriptionByteLength).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_INFO_QUEUE_MESSAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Producer == other.Producer && self.Category == other.Category && self.Severity == other.Severity && self.ID == other.ID && self.pDescription == other.pDescription && self.DescriptionByteLength == other.DescriptionByteLength
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_MESSAGE {}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl ::core::marker::Copy for DXGI_MAPPED_RECT {}
impl ::core::clone::Clone for DXGI_MAPPED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_MAPPED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MAPPED_RECT").field("Pitch", &self.Pitch).field("pBits", &self.pBits).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_MAPPED_RECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_MAPPED_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Pitch == other.Pitch && self.pBits == other.pBits
    }
}
impl ::core::cmp::Eq for DXGI_MAPPED_RECT {}
impl ::core::default::Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl ::core::marker::Copy for DXGI_MATRIX_3X2_F {}
impl ::core::clone::Clone for DXGI_MATRIX_3X2_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_MATRIX_3X2_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MATRIX_3X2_F").field("_11", &self._11).field("_12", &self._12).field("_21", &self._21).field("_22", &self._22).field("_31", &self._31).field("_32", &self._32).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_MATRIX_3X2_F {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_MATRIX_3X2_F {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._21 == other._21 && self._22 == other._22 && self._31 == other._31 && self._32 == other._32
    }
}
impl ::core::cmp::Eq for DXGI_MATRIX_3X2_F {}
impl ::core::default::Default for DXGI_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub Format: Common::DXGI_FORMAT,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Stereo: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_MODE_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_MODE_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_MODE_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MODE_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).field("Stereo", &self.Stereo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::TypeKind for DXGI_MODE_DESC1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_MODE_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.RefreshRate == other.RefreshRate && self.Format == other.Format && self.ScanlineOrdering == other.ScanlineOrdering && self.Scaling == other.Scaling && self.Stereo == other.Stereo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_MODE_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_MODE_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: Common::DXGI_MODE_DESC,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_OUTDUPL_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_OUTDUPL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_OUTDUPL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_DESC").field("ModeDesc", &self.ModeDesc).field("Rotation", &self.Rotation).field("DesktopImageInSystemMemory", &self.DesktopImageInSystemMemory).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::TypeKind for DXGI_OUTDUPL_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ModeDesc == other.ModeDesc && self.Rotation == other.Rotation && self.DesktopImageInSystemMemory == other.DesktopImageInSystemMemory
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_OUTDUPL_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_OUTDUPL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: super::super::Foundation::BOOL,
    pub ProtectedContentMaskedOut: super::super::Foundation::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_FRAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_FRAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_FRAME_INFO")
            .field("LastPresentTime", &self.LastPresentTime)
            .field("LastMouseUpdateTime", &self.LastMouseUpdateTime)
            .field("AccumulatedFrames", &self.AccumulatedFrames)
            .field("RectsCoalesced", &self.RectsCoalesced)
            .field("ProtectedContentMaskedOut", &self.ProtectedContentMaskedOut)
            .field("PointerPosition", &self.PointerPosition)
            .field("TotalMetadataBufferSize", &self.TotalMetadataBufferSize)
            .field("PointerShapeBufferSize", &self.PointerShapeBufferSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_OUTDUPL_FRAME_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastPresentTime == other.LastPresentTime && self.LastMouseUpdateTime == other.LastMouseUpdateTime && self.AccumulatedFrames == other.AccumulatedFrames && self.RectsCoalesced == other.RectsCoalesced && self.ProtectedContentMaskedOut == other.ProtectedContentMaskedOut && self.PointerPosition == other.PointerPosition && self.TotalMetadataBufferSize == other.TotalMetadataBufferSize && self.PointerShapeBufferSize == other.PointerShapeBufferSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::super::Foundation::POINT,
    pub DestinationRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_MOVE_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_MOVE_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_MOVE_RECT").field("SourcePoint", &self.SourcePoint).field("DestinationRect", &self.DestinationRect).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_OUTDUPL_MOVE_RECT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_MOVE_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.SourcePoint == other.SourcePoint && self.DestinationRect == other.DestinationRect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_MOVE_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::super::Foundation::POINT,
    pub Visible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_POINTER_POSITION").field("Position", &self.Position).field("Visible", &self.Visible).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_OUTDUPL_POINTER_POSITION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Visible == other.Visible
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_POINTER_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_POINTER_SHAPE_INFO").field("Type", &self.Type).field("Width", &self.Width).field("Height", &self.Height).field("Pitch", &self.Pitch).field("HotSpot", &self.HotSpot).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Width == other.Width && self.Height == other.Height && self.Pitch == other.Pitch && self.HotSpot == other.HotSpot
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DXGI_OUTPUT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTPUT_DESC").field("DeviceName", &self.DeviceName).field("DesktopCoordinates", &self.DesktopCoordinates).field("AttachedToDesktop", &self.AttachedToDesktop).field("Rotation", &self.Rotation).field("Monitor", &self.Monitor).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DXGI_OUTPUT_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DXGI_OUTPUT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DesktopCoordinates == other.DesktopCoordinates && self.AttachedToDesktop == other.AttachedToDesktop && self.Rotation == other.Rotation && self.Monitor == other.Monitor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: Common::DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DXGI_OUTPUT_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTPUT_DESC1")
            .field("DeviceName", &self.DeviceName)
            .field("DesktopCoordinates", &self.DesktopCoordinates)
            .field("AttachedToDesktop", &self.AttachedToDesktop)
            .field("Rotation", &self.Rotation)
            .field("Monitor", &self.Monitor)
            .field("BitsPerColor", &self.BitsPerColor)
            .field("ColorSpace", &self.ColorSpace)
            .field("RedPrimary", &self.RedPrimary)
            .field("GreenPrimary", &self.GreenPrimary)
            .field("BluePrimary", &self.BluePrimary)
            .field("WhitePoint", &self.WhitePoint)
            .field("MinLuminance", &self.MinLuminance)
            .field("MaxLuminance", &self.MaxLuminance)
            .field("MaxFullFrameLuminance", &self.MaxFullFrameLuminance)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DXGI_OUTPUT_DESC1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DXGI_OUTPUT_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DesktopCoordinates == other.DesktopCoordinates && self.AttachedToDesktop == other.AttachedToDesktop && self.Rotation == other.Rotation && self.Monitor == other.Monitor && self.BitsPerColor == other.BitsPerColor && self.ColorSpace == other.ColorSpace && self.RedPrimary == other.RedPrimary && self.GreenPrimary == other.GreenPrimary && self.BluePrimary == other.BluePrimary && self.WhitePoint == other.WhitePoint && self.MinLuminance == other.MinLuminance && self.MaxLuminance == other.MaxLuminance && self.MaxFullFrameLuminance == other.MaxFullFrameLuminance
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::super::Foundation::RECT,
    pub pScrollRect: *mut super::super::Foundation::RECT,
    pub pScrollOffset: *mut super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_PRESENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_PRESENT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_PRESENT_PARAMETERS").field("DirtyRectsCount", &self.DirtyRectsCount).field("pDirtyRects", &self.pDirtyRects).field("pScrollRect", &self.pScrollRect).field("pScrollOffset", &self.pScrollOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_PRESENT_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_PRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.DirtyRectsCount == other.DirtyRectsCount && self.pDirtyRects == other.pDirtyRects && self.pScrollRect == other.pScrollRect && self.pScrollOffset == other.pScrollOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
impl ::core::marker::Copy for DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::core::clone::Clone for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_QUERY_VIDEO_MEMORY_INFO").field("Budget", &self.Budget).field("CurrentUsage", &self.CurrentUsage).field("AvailableForReservation", &self.AvailableForReservation).field("CurrentReservation", &self.CurrentReservation).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_QUERY_VIDEO_MEMORY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Budget == other.Budget && self.CurrentUsage == other.CurrentUsage && self.AvailableForReservation == other.AvailableForReservation && self.CurrentReservation == other.CurrentReservation
    }
}
impl ::core::cmp::Eq for DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::core::default::Default for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for DXGI_RGBA {}
impl ::core::clone::Clone for DXGI_RGBA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_RGBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_RGBA").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::windows::core::TypeKind for DXGI_RGBA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for DXGI_RGBA {}
impl ::core::default::Default for DXGI_RGBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_SHARED_RESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_SHARED_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_SHARED_RESOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SHARED_RESOURCE").field("Handle", &self.Handle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXGI_SHARED_RESOURCE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_SHARED_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_SHARED_RESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_SURFACE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_SURFACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DXGI_SURFACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SURFACE_DESC").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::TypeKind for DXGI_SURFACE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DXGI_SURFACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.SampleDesc == other.SampleDesc
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DXGI_SURFACE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DXGI_SURFACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: Common::DXGI_MODE_DESC,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub OutputWindow: super::super::Foundation::HWND,
    pub Windowed: super::super::Foundation::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_DESC").field("BufferDesc", &self.BufferDesc).field("SampleDesc", &self.SampleDesc).field("BufferUsage", &self.BufferUsage).field("BufferCount", &self.BufferCount).field("OutputWindow", &self.OutputWindow).field("Windowed", &self.Windowed).field("SwapEffect", &self.SwapEffect).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::TypeKind for DXGI_SWAP_CHAIN_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.BufferDesc == other.BufferDesc && self.SampleDesc == other.SampleDesc && self.BufferUsage == other.BufferUsage && self.BufferCount == other.BufferCount && self.OutputWindow == other.OutputWindow && self.Windowed == other.Windowed && self.SwapEffect == other.SwapEffect && self.Flags == other.Flags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub Stereo: super::super::Foundation::BOOL,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: Common::DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("Stereo", &self.Stereo).field("SampleDesc", &self.SampleDesc).field("BufferUsage", &self.BufferUsage).field("BufferCount", &self.BufferCount).field("Scaling", &self.Scaling).field("SwapEffect", &self.SwapEffect).field("AlphaMode", &self.AlphaMode).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::TypeKind for DXGI_SWAP_CHAIN_DESC1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.Stereo == other.Stereo && self.SampleDesc == other.SampleDesc && self.BufferUsage == other.BufferUsage && self.BufferCount == other.BufferCount && self.Scaling == other.Scaling && self.SwapEffect == other.SwapEffect && self.AlphaMode == other.AlphaMode && self.Flags == other.Flags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_SWAP_CHAIN_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Windowed: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_FULLSCREEN_DESC").field("RefreshRate", &self.RefreshRate).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).field("Windowed", &self.Windowed).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::TypeKind for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.RefreshRate == other.RefreshRate && self.ScanlineOrdering == other.ScanlineOrdering && self.Scaling == other.Scaling && self.Windowed == other.Windowed
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
