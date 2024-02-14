#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3D11On12CreateDevice<P0>(pdevice: P0, flags: u32, pfeaturelevels: ::core::option::Option<&[super::Direct3D::D3D_FEATURE_LEVEL]>, ppcommandqueues: ::core::option::Option<&[::core::option::Option<::windows_core::IUnknown>]>, nodemask: u32, ppdevice: ::core::option::Option<*mut ::core::option::Option<super::Direct3D11::ID3D11Device>>, ppimmediatecontext: ::core::option::Option<*mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>>, pchosenfeaturelevel: ::core::option::Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("d3d11.dll" "system" fn D3D11On12CreateDevice(pdevice : * mut::core::ffi::c_void, flags : u32, pfeaturelevels : *const super::Direct3D:: D3D_FEATURE_LEVEL, featurelevels : u32, ppcommandqueues : *const * mut::core::ffi::c_void, numqueues : u32, nodemask : u32, ppdevice : *mut * mut::core::ffi::c_void, ppimmediatecontext : *mut * mut::core::ffi::c_void, pchosenfeaturelevel : *mut super::Direct3D:: D3D_FEATURE_LEVEL) -> ::windows_core::HRESULT);
    D3D11On12CreateDevice(
        pdevice.into_param().abi(),
        flags,
        ::core::mem::transmute(pfeaturelevels.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        pfeaturelevels.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        ::core::mem::transmute(ppcommandqueues.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        ppcommandqueues.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        nodemask,
        ::core::mem::transmute(ppdevice.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(ppimmediatecontext.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pchosenfeaturelevel.unwrap_or(::std::ptr::null_mut())),
    )
    .ok()
}
::windows_core::imp::com_interface!(ID3D11On12Device, ID3D11On12Device_Vtbl, 0x85611e73_70a9_490e_9614_a9e302777904);
::windows_core::imp::interface_hierarchy!(ID3D11On12Device, ::windows_core::IUnknown);
impl ID3D11On12Device {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateWrappedResource<P0, T>(&self, presource12: P0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        T: ::windows_core::Interface,
    {
        (::windows_core::Interface::vtable(self).CreateWrappedResource)(::windows_core::Interface::as_raw(self), presource12.into_param().abi(), pflags11, instate, outstate, &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).ReleaseWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len().try_into().unwrap())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).AcquireWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len().try_into().unwrap())
    }
}
unsafe impl ::core::marker::Send for ID3D11On12Device {}
unsafe impl ::core::marker::Sync for ID3D11On12Device {}
#[repr(C)]
pub struct ID3D11On12Device_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateWrappedResource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *const D3D11_RESOURCE_FLAGS, super::Direct3D12::D3D12_RESOURCE_STATES, super::Direct3D12::D3D12_RESOURCE_STATES, *const ::windows_core::GUID, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateWrappedResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub ReleaseWrappedResources: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const *mut ::core::ffi::c_void, u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))]
    ReleaseWrappedResources: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub AcquireWrappedResources: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const *mut ::core::ffi::c_void, u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))]
    AcquireWrappedResources: usize,
}
::windows_core::imp::com_interface!(ID3D11On12Device1, ID3D11On12Device1_Vtbl, 0xbdb64df4_ea2f_4c70_b861_aaab1258bb5d);
::windows_core::imp::interface_hierarchy!(ID3D11On12Device1, ::windows_core::IUnknown, ID3D11On12Device);
impl ID3D11On12Device1 {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateWrappedResource<P0, T>(&self, presource12: P0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        T: ::windows_core::Interface,
    {
        (::windows_core::Interface::vtable(self).base__.CreateWrappedResource)(::windows_core::Interface::as_raw(self), presource12.into_param().abi(), pflags11, instate, outstate, &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).base__.ReleaseWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len().try_into().unwrap())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).base__.AcquireWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len().try_into().unwrap())
    }
    pub unsafe fn GetD3D12Device<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetD3D12Device)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for ID3D11On12Device1 {}
unsafe impl ::core::marker::Sync for ID3D11On12Device1 {}
#[repr(C)]
pub struct ID3D11On12Device1_Vtbl {
    pub base__: ID3D11On12Device_Vtbl,
    pub GetD3D12Device: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ID3D11On12Device2, ID3D11On12Device2_Vtbl, 0xdc90f331_4740_43fa_866e_67f12cb58223);
::windows_core::imp::interface_hierarchy!(ID3D11On12Device2, ::windows_core::IUnknown, ID3D11On12Device, ID3D11On12Device1);
impl ID3D11On12Device2 {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateWrappedResource<P0, T>(&self, presource12: P0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        T: ::windows_core::Interface,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CreateWrappedResource)(::windows_core::Interface::as_raw(self), presource12.into_param().abi(), pflags11, instate, outstate, &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).base__.base__.ReleaseWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len().try_into().unwrap())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).base__.base__.AcquireWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len().try_into().unwrap())
    }
    pub unsafe fn GetD3D12Device<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetD3D12Device)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn UnwrapUnderlyingResource<P0, P1, T>(&self, presource11: P0, pcommandqueue: P1) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::Direct3D11::ID3D11Resource>,
        P1: ::windows_core::IntoParam<super::Direct3D12::ID3D12CommandQueue>,
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).UnwrapUnderlyingResource)(::windows_core::Interface::as_raw(self), presource11.into_param().abi(), pcommandqueue.into_param().abi(), &T::IID, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn ReturnUnderlyingResource<P0>(&self, presource11: P0, numsync: u32, psignalvalues: *const u64, ppfences: *const ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Direct3D11::ID3D11Resource>,
    {
        (::windows_core::Interface::vtable(self).ReturnUnderlyingResource)(::windows_core::Interface::as_raw(self), presource11.into_param().abi(), numsync, psignalvalues, ::core::mem::transmute(ppfences)).ok()
    }
}
unsafe impl ::core::marker::Send for ID3D11On12Device2 {}
unsafe impl ::core::marker::Sync for ID3D11On12Device2 {}
#[repr(C)]
pub struct ID3D11On12Device2_Vtbl {
    pub base__: ID3D11On12Device1_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *const ::windows_core::GUID, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, u32, *const u64, *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))]
    ReturnUnderlyingResource: usize,
}
#[repr(C)]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
impl ::core::marker::Copy for D3D11_RESOURCE_FLAGS {}
impl ::core::clone::Clone for D3D11_RESOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D11_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_RESOURCE_FLAGS").field("BindFlags", &self.BindFlags).field("MiscFlags", &self.MiscFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("StructureByteStride", &self.StructureByteStride).finish()
    }
}
impl ::windows_core::TypeKind for D3D11_RESOURCE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D3D11_RESOURCE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.BindFlags == other.BindFlags && self.MiscFlags == other.MiscFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.StructureByteStride == other.StructureByteStride
    }
}
impl ::core::cmp::Eq for D3D11_RESOURCE_FLAGS {}
impl ::core::default::Default for D3D11_RESOURCE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<::windows_core::IUnknown>, param1: u32, param2: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const ::core::option::Option<::windows_core::IUnknown>, numqueues: u32, param6: u32, param7: *mut ::core::option::Option<super::Direct3D11::ID3D11Device>, param8: *mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>, param9: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
