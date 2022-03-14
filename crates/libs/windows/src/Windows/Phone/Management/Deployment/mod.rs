#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
pub struct Enterprise(::windows::core::IUnknown);
impl Enterprise {
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
    pub fn WorkplaceId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WorkplaceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnrollmentValidFrom(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnrollmentValidFrom)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnrollmentValidTo(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnrollmentValidTo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<EnterpriseStatus> {
        let this = self;
        unsafe {
            let mut result__: EnterpriseStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnterpriseStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for Enterprise {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Enterprise {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Enterprise {}
impl ::core::fmt::Debug for Enterprise {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Enterprise").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Enterprise {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.Enterprise;{96592f8d-856c-4426-a947-b06307718078})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Enterprise {
    type Vtable = IEnterprise_Vtbl;
    const IID: ::windows::core::GUID = <IEnterprise as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Enterprise {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.Enterprise";
}
impl ::core::convert::From<Enterprise> for ::windows::core::IUnknown {
    fn from(value: Enterprise) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Enterprise> for ::windows::core::IUnknown {
    fn from(value: &Enterprise) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Enterprise {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Enterprise {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Enterprise> for ::windows::core::IInspectable {
    fn from(value: Enterprise) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Enterprise> for ::windows::core::IInspectable {
    fn from(value: &Enterprise) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Enterprise {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Enterprise {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Enterprise {}
unsafe impl ::core::marker::Sync for Enterprise {}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
pub struct EnterpriseEnrollmentManager {}
impl EnterpriseEnrollmentManager {
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnrolledEnterprises() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Enterprise>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnrolledEnterprises)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Enterprise>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
    pub fn CurrentEnterprise() -> ::windows::core::Result<Enterprise> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentEnterprise)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Enterprise>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValidateEnterprisesAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValidateEnterprisesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestEnrollmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(enrollmenttoken: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestEnrollmentAsync)(::core::mem::transmute_copy(this), enrollmenttoken.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUnenrollmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Enterprise>>(enterprise: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestUnenrollmentAsync)(::core::mem::transmute_copy(this), enterprise.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnterpriseEnrollmentManager<R, F: FnOnce(&IEnterpriseEnrollmentManager) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EnterpriseEnrollmentManager, IEnterpriseEnrollmentManager> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for EnterpriseEnrollmentManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentManager";
}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
pub struct EnterpriseEnrollmentResult(::windows::core::IUnknown);
impl EnterpriseEnrollmentResult {
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
    pub fn EnrolledEnterprise(&self) -> ::windows::core::Result<Enterprise> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnrolledEnterprise)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Enterprise>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<EnterpriseEnrollmentStatus> {
        let this = self;
        unsafe {
            let mut result__: EnterpriseEnrollmentStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnterpriseEnrollmentStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for EnterpriseEnrollmentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnterpriseEnrollmentResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnterpriseEnrollmentResult {}
impl ::core::fmt::Debug for EnterpriseEnrollmentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseEnrollmentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnterpriseEnrollmentResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult;{9ff71ce6-90db-4342-b326-1729aa91301c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_Vtbl;
    const IID: ::windows::core::GUID = <IEnterpriseEnrollmentResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EnterpriseEnrollmentResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult";
}
impl ::core::convert::From<EnterpriseEnrollmentResult> for ::windows::core::IUnknown {
    fn from(value: EnterpriseEnrollmentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseEnrollmentResult> for ::windows::core::IUnknown {
    fn from(value: &EnterpriseEnrollmentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnterpriseEnrollmentResult> for ::windows::core::IInspectable {
    fn from(value: EnterpriseEnrollmentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseEnrollmentResult> for ::windows::core::IInspectable {
    fn from(value: &EnterpriseEnrollmentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EnterpriseEnrollmentStatus(pub i32);
impl EnterpriseEnrollmentStatus {
    pub const Success: Self = Self(0i32);
    pub const CancelledByUser: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for EnterpriseEnrollmentStatus {}
impl ::core::clone::Clone for EnterpriseEnrollmentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnterpriseEnrollmentStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EnterpriseEnrollmentStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnterpriseEnrollmentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseEnrollmentStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnterpriseEnrollmentStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Management.Deployment.EnterpriseEnrollmentStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EnterpriseStatus(pub i32);
impl EnterpriseStatus {
    pub const Enrolled: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Expired: Self = Self(3i32);
}
impl ::core::marker::Copy for EnterpriseStatus {}
impl ::core::clone::Clone for EnterpriseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnterpriseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EnterpriseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnterpriseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnterpriseStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Management.Deployment.EnterpriseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterprise(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnterprise {
    type Vtable = IEnterprise_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96592f8d_856c_4426_a947_b06307718078);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterprise_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WorkplaceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnrollmentValidFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnrollmentValidFrom: usize,
    #[cfg(feature = "Foundation")]
    pub EnrollmentValidTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnrollmentValidTo: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnterpriseStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseEnrollmentManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnterpriseEnrollmentManager {
    type Vtable = IEnterpriseEnrollmentManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20f9f390_2c69_41d8_88e6_e4b3884026cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EnrolledEnterprises: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnrolledEnterprises: usize,
    pub CurrentEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ValidateEnterprisesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidateEnterprisesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestEnrollmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enrollmenttoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestEnrollmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUnenrollmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enterprise: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnenrollmentAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseEnrollmentResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ff71ce6_90db_4342_b326_1729aa91301c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EnrolledEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnterpriseEnrollmentStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstallationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInstallationManagerStatics {
    type Vtable = IInstallationManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x929aa738_8d49_42ac_80c9_b4ad793c43f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AddPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sourcelocation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddPackagePreloadedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sourcelocation: ::windows::core::RawPtr, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, offerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, license: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackagePreloadedAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPendingPackageInstalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPendingPackageInstalls: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesForCurrentPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesForCurrentPublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstallationManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInstallationManagerStatics2 {
    type Vtable = IInstallationManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c6c2cbd_fa4a_4c8e_ab97_d959452f19e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))]
    pub RemovePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, removaloptions: super::super::super::Management::Deployment::RemovalOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Management_Deployment")))]
    RemovePackageAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub RegisterPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    RegisterPackageAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageInstallResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageInstallResult {
    type Vtable = IPackageInstallResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33e8eed5_0f7e_4473_967c_7d6e1c0e7de1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Management::Deployment::PackageInstallState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    InstallState: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageInstallResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageInstallResult2 {
    type Vtable = IPackageInstallResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7149d909_3ff9_41ed_a717_2bc65ffc61d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
pub struct InstallationManager {}
impl InstallationManager {
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(title: Param0, sourcelocation: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddPackageAsync)(::core::mem::transmute_copy(this), title.into_param().abi(), sourcelocation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddPackagePreloadedAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(title: Param0, sourcelocation: Param1, instanceid: Param2, offerid: Param3, license: Param4) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddPackagePreloadedAsync)(::core::mem::transmute_copy(this), title.into_param().abi(), sourcelocation.into_param().abi(), instanceid.into_param().abi(), offerid.into_param().abi(), license.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPendingPackageInstalls() -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPendingPackageInstalls)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesForCurrentPublisher() -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindPackagesForCurrentPublisher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackages() -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindPackages)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation\"`, `\"Management_Deployment\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))]
    pub fn RemovePackageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagefullname: Param0, removaloptions: super::super::super::Management::Deployment::RemovalOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemovePackageAsync)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), removaloptions, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Foundation_Collections\"`, `\"Management_Deployment\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn RegisterPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Uri>>>(manifesturi: Param0, dependencypackageuris: Param1, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPackageAsync)(::core::mem::transmute_copy(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagename: Param0, packagepublisher: Param1) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindPackagesByNamePublisher)(::core::mem::transmute_copy(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInstallationManagerStatics<R, F: FnOnce(&IInstallationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InstallationManager, IInstallationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IInstallationManagerStatics2<R, F: FnOnce(&IInstallationManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InstallationManager, IInstallationManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for InstallationManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.InstallationManager";
}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageInstallResult(::windows::core::IUnknown);
impl PackageInstallResult {
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`, `\"Management_Deployment\"`*"]
    #[cfg(feature = "Management_Deployment")]
    pub fn InstallState(&self) -> ::windows::core::Result<super::super::super::Management::Deployment::PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Management::Deployment::PackageInstallState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Management::Deployment::PackageInstallState>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackageInstallResult2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageInstallResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageInstallResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageInstallResult {}
impl ::core::fmt::Debug for PackageInstallResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageInstallResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageInstallResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.PackageInstallResult;{33e8eed5-0f7e-4473-967c-7d6e1c0e7de1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageInstallResult {
    type Vtable = IPackageInstallResult_Vtbl;
    const IID: ::windows::core::GUID = <IPackageInstallResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageInstallResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.PackageInstallResult";
}
impl ::core::convert::From<PackageInstallResult> for ::windows::core::IUnknown {
    fn from(value: PackageInstallResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageInstallResult> for ::windows::core::IUnknown {
    fn from(value: &PackageInstallResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageInstallResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageInstallResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageInstallResult> for ::windows::core::IInspectable {
    fn from(value: PackageInstallResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageInstallResult> for ::windows::core::IInspectable {
    fn from(value: &PackageInstallResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageInstallResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageInstallResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
