#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Enterprise(pub ::windows::runtime::IInspectable);
impl Enterprise {
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn WorkplaceId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn EnrollmentValidFrom(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn EnrollmentValidTo(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<EnterpriseStatus> {
        let this = self;
        unsafe {
            let mut result__: EnterpriseStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnterpriseStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Enterprise {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.Enterprise;{96592f8d-856c-4426-a947-b06307718078})");
}
unsafe impl ::windows::runtime::Interface for Enterprise {
    type Vtable = IEnterprise_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x96592f8d_856c_4426_a947_b06307718078);
}
impl ::windows::runtime::RuntimeName for Enterprise {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.Enterprise";
}
impl ::core::convert::From<Enterprise> for ::windows::runtime::IUnknown {
    fn from(value: Enterprise) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Enterprise> for ::windows::runtime::IUnknown {
    fn from(value: &Enterprise) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Enterprise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Enterprise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Enterprise> for ::windows::runtime::IInspectable {
    fn from(value: Enterprise) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Enterprise> for ::windows::runtime::IInspectable {
    fn from(value: &Enterprise) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Enterprise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Enterprise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Enterprise {}
unsafe impl ::core::marker::Sync for Enterprise {}
#[doc = "*Required features: `Phone_Management_Deployment`*"]
pub struct EnterpriseEnrollmentManager {}
impl EnterpriseEnrollmentManager {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation_Collections`*"]
    pub fn EnrolledEnterprises() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<Enterprise>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Enterprise>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn CurrentEnterprise() -> ::windows::runtime::Result<Enterprise> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Enterprise>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn ValidateEnterprisesAsync() -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn RequestEnrollmentAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(enrollmenttoken: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), enrollmenttoken.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn RequestUnenrollmentAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Enterprise>>(enterprise: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), enterprise.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IEnterpriseEnrollmentManager<R, F: FnOnce(&IEnterpriseEnrollmentManager) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<EnterpriseEnrollmentManager, IEnterpriseEnrollmentManager> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for EnterpriseEnrollmentManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentManager";
}
#[doc = "*Required features: `Phone_Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EnterpriseEnrollmentResult(pub ::windows::runtime::IInspectable);
impl EnterpriseEnrollmentResult {
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn EnrolledEnterprise(&self) -> ::windows::runtime::Result<Enterprise> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Enterprise>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<EnterpriseEnrollmentStatus> {
        let this = self;
        unsafe {
            let mut result__: EnterpriseEnrollmentStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnterpriseEnrollmentStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EnterpriseEnrollmentResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult;{9ff71ce6-90db-4342-b326-1729aa91301c})");
}
unsafe impl ::windows::runtime::Interface for EnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9ff71ce6_90db_4342_b326_1729aa91301c);
}
impl ::windows::runtime::RuntimeName for EnterpriseEnrollmentResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult";
}
impl ::core::convert::From<EnterpriseEnrollmentResult> for ::windows::runtime::IUnknown {
    fn from(value: EnterpriseEnrollmentResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EnterpriseEnrollmentResult> for ::windows::runtime::IUnknown {
    fn from(value: &EnterpriseEnrollmentResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EnterpriseEnrollmentResult> for ::windows::runtime::IInspectable {
    fn from(value: EnterpriseEnrollmentResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EnterpriseEnrollmentResult> for ::windows::runtime::IInspectable {
    fn from(value: &EnterpriseEnrollmentResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Management_Deployment`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EnterpriseEnrollmentStatus(pub i32);
impl EnterpriseEnrollmentStatus {
    pub const Success: EnterpriseEnrollmentStatus = EnterpriseEnrollmentStatus(0i32);
    pub const CancelledByUser: EnterpriseEnrollmentStatus = EnterpriseEnrollmentStatus(1i32);
    pub const UnknownFailure: EnterpriseEnrollmentStatus = EnterpriseEnrollmentStatus(2i32);
}
impl ::core::convert::From<i32> for EnterpriseEnrollmentStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EnterpriseEnrollmentStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EnterpriseEnrollmentStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Management.Deployment.EnterpriseEnrollmentStatus;i4)");
}
impl ::windows::runtime::DefaultType for EnterpriseEnrollmentStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Management_Deployment`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EnterpriseStatus(pub i32);
impl EnterpriseStatus {
    pub const Enrolled: EnterpriseStatus = EnterpriseStatus(0i32);
    pub const Disabled: EnterpriseStatus = EnterpriseStatus(1i32);
    pub const Revoked: EnterpriseStatus = EnterpriseStatus(2i32);
    pub const Expired: EnterpriseStatus = EnterpriseStatus(3i32);
}
impl ::core::convert::From<i32> for EnterpriseStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EnterpriseStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EnterpriseStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Management.Deployment.EnterpriseStatus;i4)");
}
impl ::windows::runtime::DefaultType for EnterpriseStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnterprise(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnterprise {
    type Vtable = IEnterprise_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x96592f8d_856c_4426_a947_b06307718078);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterprise_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EnterpriseStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnterpriseEnrollmentManager {
    type Vtable = IEnterpriseEnrollmentManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x20f9f390_2c69_41d8_88e6_e4b3884026cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enrollmenttoken: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enterprise: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9ff71ce6_90db_4342_b326_1729aa91301c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EnterpriseEnrollmentStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInstallationManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInstallationManagerStatics {
    type Vtable = IInstallationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x929aa738_8d49_42ac_80c9_b4ad793c43f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, sourcelocation: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, sourcelocation: ::windows::runtime::RawPtr, instanceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, offerid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, license: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInstallationManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInstallationManagerStatics2 {
    type Vtable = IInstallationManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7c6c2cbd_fa4a_4c8e_ab97_d959452f19e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, removaloptions: super::super::super::Management::Deployment::RemovalOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Management_Deployment")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifesturi: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageInstallResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageInstallResult {
    type Vtable = IPackageInstallResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x33e8eed5_0f7e_4473_967c_7d6e1c0e7de1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Management_Deployment")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Management::Deployment::PackageInstallState) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageInstallResult2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageInstallResult2 {
    type Vtable = IPackageInstallResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7149d909_3ff9_41ed_a717_2bc65ffc61d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Phone_Management_Deployment`*"]
pub struct InstallationManager {}
impl InstallationManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn AddPackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(title: Param0, sourcelocation: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), title.into_param().abi(), sourcelocation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn AddPackagePreloadedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(
        title: Param0,
        sourcelocation: Param1,
        instanceid: Param2,
        offerid: Param3,
        license: Param4,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), title.into_param().abi(), sourcelocation.into_param().abi(), instanceid.into_param().abi(), offerid.into_param().abi(), license.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetPendingPackageInstalls() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesForCurrentPublisher() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackages() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`, `Management_Deployment`*"]
    pub fn RemovePackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(packagefullname: Param0, removaloptions: super::super::super::Management::Deployment::RemovalOptions) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), removaloptions, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`, `Foundation_Collections`, `Management_Deployment`*"]
    pub fn RegisterPackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Uri>>>(
        manifesturi: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(packagename: Param0, packagepublisher: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>(result__)
        })
    }
    pub fn IInstallationManagerStatics<R, F: FnOnce(&IInstallationManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InstallationManager, IInstallationManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInstallationManagerStatics2<R, F: FnOnce(&IInstallationManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InstallationManager, IInstallationManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for InstallationManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.InstallationManager";
}
#[doc = "*Required features: `Phone_Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PackageInstallResult(pub ::windows::runtime::IInspectable);
impl PackageInstallResult {
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn ProductId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Management_Deployment")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Management_Deployment`*"]
    pub fn InstallState(&self) -> ::windows::runtime::Result<super::super::super::Management::Deployment::PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Management::Deployment::PackageInstallState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Management::Deployment::PackageInstallState>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn ErrorText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPackageInstallResult2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PackageInstallResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.PackageInstallResult;{33e8eed5-0f7e-4473-967c-7d6e1c0e7de1})");
}
unsafe impl ::windows::runtime::Interface for PackageInstallResult {
    type Vtable = IPackageInstallResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x33e8eed5_0f7e_4473_967c_7d6e1c0e7de1);
}
impl ::windows::runtime::RuntimeName for PackageInstallResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.PackageInstallResult";
}
impl ::core::convert::From<PackageInstallResult> for ::windows::runtime::IUnknown {
    fn from(value: PackageInstallResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PackageInstallResult> for ::windows::runtime::IUnknown {
    fn from(value: &PackageInstallResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PackageInstallResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PackageInstallResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PackageInstallResult> for ::windows::runtime::IInspectable {
    fn from(value: PackageInstallResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PackageInstallResult> for ::windows::runtime::IInspectable {
    fn from(value: &PackageInstallResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PackageInstallResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PackageInstallResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
