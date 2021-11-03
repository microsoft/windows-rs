#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Enterprise(::windows::runtime::IInspectable);
impl Enterprise {
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn WorkplaceId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn EnrollmentValidFrom(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn EnrollmentValidTo(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<EnterpriseStatus> {
        let this = self;
        unsafe {
            let mut result__: EnterpriseStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EnterpriseStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Enterprise {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.Enterprise;{96592f8d-856c-4426-a947-b06307718078})");
}
unsafe impl ::windows::runtime::Interface for Enterprise {
    type Vtable = IEnterprise_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2522427277, 34156, 17446, [169, 71, 176, 99, 7, 113, 128, 120]);
}
impl ::windows::runtime::RuntimeName for Enterprise {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.Enterprise";
}
impl ::std::convert::From<Enterprise> for ::windows::runtime::IUnknown {
    fn from(value: Enterprise) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Enterprise> for ::windows::runtime::IUnknown {
    fn from(value: &Enterprise) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Enterprise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Enterprise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Enterprise> for ::windows::runtime::IInspectable {
    fn from(value: Enterprise) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Enterprise> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for Enterprise {}
unsafe impl ::std::marker::Sync for Enterprise {}
#[doc = "*Required features: `Phone_Management_Deployment`*"]
pub struct EnterpriseEnrollmentManager {}
impl EnterpriseEnrollmentManager {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation_Collections`*"]
    pub fn EnrolledEnterprises() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<Enterprise>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Enterprise>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn CurrentEnterprise() -> ::windows::runtime::Result<Enterprise> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Enterprise>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn ValidateEnterprisesAsync() -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn RequestEnrollmentAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(enrollmenttoken: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), enrollmenttoken.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn RequestUnenrollmentAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Enterprise>>(enterprise: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), enterprise.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EnterpriseEnrollmentResult(::windows::runtime::IInspectable);
impl EnterpriseEnrollmentResult {
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn EnrolledEnterprise(&self) -> ::windows::runtime::Result<Enterprise> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Enterprise>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<EnterpriseEnrollmentStatus> {
        let this = self;
        unsafe {
            let mut result__: EnterpriseEnrollmentStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EnterpriseEnrollmentStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EnterpriseEnrollmentResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult;{9ff71ce6-90db-4342-b326-1729aa91301c})");
}
unsafe impl ::windows::runtime::Interface for EnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2683772134, 37083, 17218, [179, 38, 23, 41, 170, 145, 48, 28]);
}
impl ::windows::runtime::RuntimeName for EnterpriseEnrollmentResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult";
}
impl ::std::convert::From<EnterpriseEnrollmentResult> for ::windows::runtime::IUnknown {
    fn from(value: EnterpriseEnrollmentResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EnterpriseEnrollmentResult> for ::windows::runtime::IUnknown {
    fn from(value: &EnterpriseEnrollmentResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EnterpriseEnrollmentResult> for ::windows::runtime::IInspectable {
    fn from(value: EnterpriseEnrollmentResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EnterpriseEnrollmentResult> for ::windows::runtime::IInspectable {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EnterpriseEnrollmentStatus(pub i32);
impl EnterpriseEnrollmentStatus {
    pub const Success: EnterpriseEnrollmentStatus = EnterpriseEnrollmentStatus(0i32);
    pub const CancelledByUser: EnterpriseEnrollmentStatus = EnterpriseEnrollmentStatus(1i32);
    pub const UnknownFailure: EnterpriseEnrollmentStatus = EnterpriseEnrollmentStatus(2i32);
}
impl ::std::convert::From<i32> for EnterpriseEnrollmentStatus {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EnterpriseStatus(pub i32);
impl EnterpriseStatus {
    pub const Enrolled: EnterpriseStatus = EnterpriseStatus(0i32);
    pub const Disabled: EnterpriseStatus = EnterpriseStatus(1i32);
    pub const Revoked: EnterpriseStatus = EnterpriseStatus(2i32);
    pub const Expired: EnterpriseStatus = EnterpriseStatus(3i32);
}
impl ::std::convert::From<i32> for EnterpriseStatus {
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
pub struct IEnterprise(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnterprise {
    type Vtable = IEnterprise_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2522427277, 34156, 17446, [169, 71, 176, 99, 7, 113, 128, 120]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EnterpriseStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnterpriseEnrollmentManager {
    type Vtable = IEnterpriseEnrollmentManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(553251728, 11369, 16856, [136, 230, 228, 179, 136, 64, 38, 203]);
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enrollmenttoken: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enterprise: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2683772134, 37083, 17218, [179, 38, 23, 41, 170, 145, 48, 28]);
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
pub struct IInstallationManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInstallationManagerStatics {
    type Vtable = IInstallationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2459608888, 36169, 17068, [128, 201, 180, 173, 121, 60, 67, 242]);
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sourcelocation: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sourcelocation: ::windows::runtime::RawPtr, instanceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, offerid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, license: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
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
pub struct IInstallationManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInstallationManagerStatics2 {
    type Vtable = IInstallationManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2087464125, 64074, 19598, [171, 151, 217, 89, 69, 47, 25, 229]);
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
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, removaloptions: super::super::super::Management::Deployment::RemovalOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Management_Deployment")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifesturi: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageInstallResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageInstallResult {
    type Vtable = IPackageInstallResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(870903509, 3966, 17523, [150, 124, 125, 110, 28, 14, 125, 225]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Management_Deployment")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Management::Deployment::PackageInstallState) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageInstallResult2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageInstallResult2 {
    type Vtable = IPackageInstallResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1900665097, 16377, 16877, [167, 23, 43, 198, 95, 252, 97, 210]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Phone_Management_Deployment`*"]
pub struct InstallationManager {}
impl InstallationManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`*"]
    pub fn AddPackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(title: Param0, sourcelocation: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), title.into_param().abi(), sourcelocation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
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
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), title.into_param().abi(), sourcelocation.into_param().abi(), instanceid.into_param().abi(), offerid.into_param().abi(), license.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetPendingPackageInstalls() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesForCurrentPublisher() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackages() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Foundation`, `Management_Deployment`*"]
    pub fn RemovePackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(packagefullname: Param0, removaloptions: super::super::super::Management::Deployment::RemovalOptions) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), removaloptions, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
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
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(packagename: Param0, packagepublisher: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>(result__)
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PackageInstallResult(::windows::runtime::IInspectable);
impl PackageInstallResult {
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn ProductId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Management_Deployment")]
    #[doc = "*Required features: `Phone_Management_Deployment`, `Management_Deployment`*"]
    pub fn InstallState(&self) -> ::windows::runtime::Result<super::super::super::Management::Deployment::PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Management::Deployment::PackageInstallState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Management::Deployment::PackageInstallState>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Management_Deployment`*"]
    pub fn ErrorText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPackageInstallResult2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PackageInstallResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.PackageInstallResult;{33e8eed5-0f7e-4473-967c-7d6e1c0e7de1})");
}
unsafe impl ::windows::runtime::Interface for PackageInstallResult {
    type Vtable = IPackageInstallResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(870903509, 3966, 17523, [150, 124, 125, 110, 28, 14, 125, 225]);
}
impl ::windows::runtime::RuntimeName for PackageInstallResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.PackageInstallResult";
}
impl ::std::convert::From<PackageInstallResult> for ::windows::runtime::IUnknown {
    fn from(value: PackageInstallResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PackageInstallResult> for ::windows::runtime::IUnknown {
    fn from(value: &PackageInstallResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PackageInstallResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PackageInstallResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PackageInstallResult> for ::windows::runtime::IInspectable {
    fn from(value: PackageInstallResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PackageInstallResult> for ::windows::runtime::IInspectable {
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
