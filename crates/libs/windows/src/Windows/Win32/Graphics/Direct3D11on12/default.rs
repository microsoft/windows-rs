impl ::core::default::Default for D3D11_RESOURCE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_RESOURCE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.BindFlags == other.BindFlags && self.MiscFlags == other.MiscFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.StructureByteStride == other.StructureByteStride
    }
}
impl ::core::cmp::Eq for D3D11_RESOURCE_FLAGS {}
impl ::core::fmt::Debug for D3D11_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_RESOURCE_FLAGS").field("BindFlags", &self.BindFlags).field("MiscFlags", &self.MiscFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("StructureByteStride", &self.StructureByteStride).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11On12Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11On12Device {}
impl ::core::fmt::Debug for ID3D11On12Device {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11On12Device").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11On12Device1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11On12Device1 {}
impl ::core::fmt::Debug for ID3D11On12Device1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11On12Device1").field(&self.0).finish()
    }
}
impl ID3D11On12Device1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateWrappedResource<P0, T>(&self, presource12: P0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateWrappedResource)(::windows::core::Vtable::as_raw(self), presource12.into().abi(), pflags11, instate, outstate, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[super::Direct3D11::ID3D11Resource]) {
        (::windows::core::Vtable::vtable(self).base__.ReleaseWrappedResources)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len() as _)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[super::Direct3D11::ID3D11Resource]) {
        (::windows::core::Vtable::vtable(self).base__.AcquireWrappedResources)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len() as _)
    }
}
impl ::core::cmp::PartialEq for ID3D11On12Device2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11On12Device2 {}
impl ::core::fmt::Debug for ID3D11On12Device2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11On12Device2").field(&self.0).finish()
    }
}
impl ID3D11On12Device2 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateWrappedResource<P0, T>(&self, presource12: P0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateWrappedResource)(::windows::core::Vtable::as_raw(self), presource12.into().abi(), pflags11, instate, outstate, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[super::Direct3D11::ID3D11Resource]) {
        (::windows::core::Vtable::vtable(self).base__.base__.ReleaseWrappedResources)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len() as _)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[super::Direct3D11::ID3D11Resource]) {
        (::windows::core::Vtable::vtable(self).base__.base__.AcquireWrappedResources)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppresources.as_ptr()), ppresources.len() as _)
    }
    pub unsafe fn GetD3D12Device<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetD3D12Device)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
