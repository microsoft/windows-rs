#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IMdmAllowPolicyStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMdmAllowPolicyStatics {
    type Vtable = IMdmAllowPolicyStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc39709e7_741c_41f2_a4b6_314c31502586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmAllowPolicyStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMdmPolicyStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMdmPolicyStatics2 {
    type Vtable = IMdmPolicyStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99c7526_03d4_49f9_a993_43efccd265c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmPolicyStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MessagingSyncPolicy) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWorkplaceSettingsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWorkplaceSettingsStatics {
    type Vtable = IWorkplaceSettingsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4676ffd_2d92_4c08_bad4_f6590b54a6d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkplaceSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
pub struct MdmPolicy {}
impl MdmPolicy {
    pub fn IsBrowserAllowed() -> ::windows::core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsCameraAllowed() -> ::windows::core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsMicrosoftAccountAllowed() -> ::windows::core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsStoreAllowed() -> ::windows::core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn GetMessagingSyncPolicy() -> ::windows::core::Result<MessagingSyncPolicy> {
        Self::IMdmPolicyStatics2(|this| unsafe {
            let mut result__: MessagingSyncPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MessagingSyncPolicy>(result__)
        })
    }
    pub fn IMdmAllowPolicyStatics<R, F: FnOnce(&IMdmAllowPolicyStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MdmPolicy, IMdmAllowPolicyStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMdmPolicyStatics2<R, F: FnOnce(&IMdmPolicyStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MdmPolicy, IMdmPolicyStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MdmPolicy {
    const NAME: &'static str = "Windows.Management.Workplace.MdmPolicy";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MessagingSyncPolicy(pub i32);
impl MessagingSyncPolicy {
    pub const Disallowed: MessagingSyncPolicy = MessagingSyncPolicy(0i32);
    pub const Allowed: MessagingSyncPolicy = MessagingSyncPolicy(1i32);
    pub const Required: MessagingSyncPolicy = MessagingSyncPolicy(2i32);
}
impl ::core::convert::From<i32> for MessagingSyncPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MessagingSyncPolicy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MessagingSyncPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Workplace.MessagingSyncPolicy;i4)");
}
impl ::windows::core::DefaultType for MessagingSyncPolicy {
    type DefaultType = Self;
}
pub struct WorkplaceSettings {}
impl WorkplaceSettings {
    pub fn IsMicrosoftAccountOptional() -> ::windows::core::Result<bool> {
        Self::IWorkplaceSettingsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IWorkplaceSettingsStatics<R, F: FnOnce(&IWorkplaceSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WorkplaceSettings, IWorkplaceSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WorkplaceSettings {
    const NAME: &'static str = "Windows.Management.Workplace.WorkplaceSettings";
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct WorkplaceSettingsContract(pub u8);
