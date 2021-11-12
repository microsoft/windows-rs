#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub struct DeviceLockdownProfile {}
impl DeviceLockdownProfile {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedLockdownProfiles() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>(result__)
        })
    }
    pub fn GetCurrentLockdownProfile() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ApplyLockdownProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(profileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), profileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn GetLockdownProfileInformation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(profileid: Param0) -> ::windows::core::Result<DeviceLockdownProfileInformation> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), profileid.into_param().abi(), &mut result__).from_abi::<DeviceLockdownProfileInformation>(result__)
        })
    }
    pub fn IDeviceLockdownProfileStatics<R, F: FnOnce(&IDeviceLockdownProfileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceLockdownProfile, IDeviceLockdownProfileStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for DeviceLockdownProfile {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfile";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceLockdownProfileInformation(pub ::windows::core::IInspectable);
impl DeviceLockdownProfileInformation {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceLockdownProfileInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Embedded.DeviceLockdown.DeviceLockdownProfileInformation;{7980e14e-45b1-4a96-92fc-62756b739678})");
}
unsafe impl ::windows::core::Interface for DeviceLockdownProfileInformation {
    type Vtable = IDeviceLockdownProfileInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7980e14e_45b1_4a96_92fc_62756b739678);
}
impl ::windows::core::RuntimeName for DeviceLockdownProfileInformation {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfileInformation";
}
impl ::core::convert::From<DeviceLockdownProfileInformation> for ::windows::core::IUnknown {
    fn from(value: DeviceLockdownProfileInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceLockdownProfileInformation> for ::windows::core::IUnknown {
    fn from(value: &DeviceLockdownProfileInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceLockdownProfileInformation> for ::windows::core::IInspectable {
    fn from(value: DeviceLockdownProfileInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceLockdownProfileInformation> for ::windows::core::IInspectable {
    fn from(value: &DeviceLockdownProfileInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceLockdownProfileInformation {}
unsafe impl ::core::marker::Sync for DeviceLockdownProfileInformation {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceLockdownProfileInformation {
    type Vtable = IDeviceLockdownProfileInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7980e14e_45b1_4a96_92fc_62756b739678);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceLockdownProfileStatics {
    type Vtable = IDeviceLockdownProfileStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x622f6965_f9a8_41a1_a691_88cd80c7a069);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profileid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profileid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
