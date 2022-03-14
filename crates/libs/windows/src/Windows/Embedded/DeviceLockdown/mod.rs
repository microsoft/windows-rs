#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
pub struct DeviceLockdownProfile {}
impl DeviceLockdownProfile {
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedLockdownProfiles() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedLockdownProfiles)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>(result__)
        })
    }
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
    pub fn GetCurrentLockdownProfile() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentLockdownProfile)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplyLockdownProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(profileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplyLockdownProfileAsync)(::core::mem::transmute_copy(this), profileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
    pub fn GetLockdownProfileInformation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(profileid: Param0) -> ::windows::core::Result<DeviceLockdownProfileInformation> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetLockdownProfileInformation)(::core::mem::transmute_copy(this), profileid.into_param().abi(), &mut result__).from_abi::<DeviceLockdownProfileInformation>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceLockdownProfileStatics<R, F: FnOnce(&IDeviceLockdownProfileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceLockdownProfile, IDeviceLockdownProfileStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for DeviceLockdownProfile {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfile";
}
#[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
#[repr(transparent)]
pub struct DeviceLockdownProfileInformation(::windows::core::IUnknown);
impl DeviceLockdownProfileInformation {
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceLockdownProfileInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceLockdownProfileInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceLockdownProfileInformation {}
impl ::core::fmt::Debug for DeviceLockdownProfileInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceLockdownProfileInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceLockdownProfileInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Embedded.DeviceLockdown.DeviceLockdownProfileInformation;{7980e14e-45b1-4a96-92fc-62756b739678})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DeviceLockdownProfileInformation {
    type Vtable = IDeviceLockdownProfileInformation_Vtbl;
    const IID: ::windows::core::GUID = <IDeviceLockdownProfileInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeviceLockdownProfileInformation {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfileInformation";
}
impl ::core::convert::From<DeviceLockdownProfileInformation> for ::windows::core::IUnknown {
    fn from(value: DeviceLockdownProfileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceLockdownProfileInformation> for ::windows::core::IUnknown {
    fn from(value: &DeviceLockdownProfileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceLockdownProfileInformation> for ::windows::core::IInspectable {
    fn from(value: DeviceLockdownProfileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceLockdownProfileInformation> for ::windows::core::IInspectable {
    fn from(value: &DeviceLockdownProfileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceLockdownProfileInformation {}
unsafe impl ::core::marker::Sync for DeviceLockdownProfileInformation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceLockdownProfileInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceLockdownProfileInformation {
    type Vtable = IDeviceLockdownProfileInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7980e14e_45b1_4a96_92fc_62756b739678);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileInformation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceLockdownProfileStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceLockdownProfileStatics {
    type Vtable = IDeviceLockdownProfileStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x622f6965_f9a8_41a1_a691_88cd80c7a069);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedLockdownProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedLockdownProfiles: usize,
    pub GetCurrentLockdownProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyLockdownProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profileid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyLockdownProfileAsync: usize,
    pub GetLockdownProfileInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profileid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
