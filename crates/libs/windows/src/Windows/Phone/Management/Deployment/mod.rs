#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterprise(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEnterprise {
    type Vtable = IEnterprise_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnterprise {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96592f8d_856c_4426_a947_b06307718078);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterprise_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IEnterpriseEnrollmentManager {
    type Vtable = IEnterpriseEnrollmentManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnterpriseEnrollmentManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20f9f390_2c69_41d8_88e6_e4b3884026cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EnrolledEnterprises: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnrolledEnterprises: usize,
    pub CurrentEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ValidateEnterprisesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidateEnterprisesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestEnrollmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enrollmenttoken: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestEnrollmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUnenrollmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enterprise: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnenrollmentAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseEnrollmentResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnterpriseEnrollmentResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ff71ce6_90db_4342_b326_1729aa91301c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EnrolledEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnterpriseEnrollmentStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstallationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInstallationManagerStatics {
    type Vtable = IInstallationManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IInstallationManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x929aa738_8d49_42ac_80c9_b4ad793c43f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AddPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::core::ffi::c_void, sourcelocation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddPackagePreloadedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::core::ffi::c_void, sourcelocation: *mut ::core::ffi::c_void, instanceid: *mut ::core::ffi::c_void, offerid: *mut ::core::ffi::c_void, license: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackagePreloadedAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPendingPackageInstalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPendingPackageInstalls: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesForCurrentPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesForCurrentPublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstallationManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInstallationManagerStatics2 {
    type Vtable = IInstallationManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInstallationManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c6c2cbd_fa4a_4c8e_ab97_d959452f19e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))]
    pub RemovePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: *mut ::core::ffi::c_void, removaloptions: super::super::super::Management::Deployment::RemovalOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Management_Deployment")))]
    RemovePackageAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub RegisterPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    RegisterPackageAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: *mut ::core::ffi::c_void, packagepublisher: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageInstallResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageInstallResult {
    type Vtable = IPackageInstallResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageInstallResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33e8eed5_0f7e_4473_967c_7d6e1c0e7de1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Management::Deployment::PackageInstallState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    InstallState: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageInstallResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageInstallResult2 {
    type Vtable = IPackageInstallResult2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageInstallResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7149d909_3ff9_41ed_a717_2bc65ffc61d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
pub struct Enterprise(::windows::core::IUnknown);
impl Enterprise {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn WorkplaceId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WorkplaceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnrollmentValidFrom(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnrollmentValidFrom)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnrollmentValidTo(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnrollmentValidTo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<EnterpriseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for Enterprise {
    type Vtable = IEnterprise_Vtbl;
}
unsafe impl ::windows::core::Interface for Enterprise {
    const IID: ::windows::core::GUID = <IEnterprise as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Enterprise {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.Enterprise";
}
::windows::core::interface_hierarchy!(Enterprise, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Enterprise {}
unsafe impl ::core::marker::Sync for Enterprise {}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
pub struct EnterpriseEnrollmentManager;
impl EnterpriseEnrollmentManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnrolledEnterprises() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Enterprise>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnrolledEnterprises)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CurrentEnterprise() -> ::windows::core::Result<Enterprise> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentEnterprise)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValidateEnterprisesAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValidateEnterprisesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestEnrollmentAsync(enrollmenttoken: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestEnrollmentAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(enrollmenttoken), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUnenrollmentAsync(enterprise: &Enterprise) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestUnenrollmentAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(enterprise), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnterpriseEnrollmentManager<R, F: FnOnce(&IEnterpriseEnrollmentManager) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<EnterpriseEnrollmentManager, IEnterpriseEnrollmentManager> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for EnterpriseEnrollmentManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentManager";
}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
pub struct EnterpriseEnrollmentResult(::windows::core::IUnknown);
impl EnterpriseEnrollmentResult {
    pub fn EnrolledEnterprise(&self) -> ::windows::core::Result<Enterprise> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnrolledEnterprise)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<EnterpriseEnrollmentStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for EnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_Vtbl;
}
unsafe impl ::windows::core::Interface for EnterpriseEnrollmentResult {
    const IID: ::windows::core::GUID = <IEnterpriseEnrollmentResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EnterpriseEnrollmentResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult";
}
::windows::core::interface_hierarchy!(EnterpriseEnrollmentResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
pub struct InstallationManager;
impl InstallationManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddPackageAsync(title: &::windows::core::HSTRING, sourcelocation: &super::super::super::Foundation::Uri) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddPackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(sourcelocation), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddPackagePreloadedAsync(title: &::windows::core::HSTRING, sourcelocation: &super::super::super::Foundation::Uri, instanceid: &::windows::core::HSTRING, offerid: &::windows::core::HSTRING, license: &super::super::super::Foundation::Uri) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddPackagePreloadedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(sourcelocation), ::core::mem::transmute_copy(instanceid), ::core::mem::transmute_copy(offerid), ::core::mem::transmute_copy(license), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPendingPackageInstalls() -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPendingPackageInstalls)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesForCurrentPublisher() -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindPackagesForCurrentPublisher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackages() -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindPackages)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Management_Deployment\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))]
    pub fn RemovePackageAsync(packagefullname: &::windows::core::HSTRING, removaloptions: super::super::super::Management::Deployment::RemovalOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemovePackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(packagefullname), removaloptions, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Management_Deployment\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn RegisterPackageAsync<P0, E0>(manifesturi: &super::super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Uri>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(manifesturi), dependencypackageuris.try_into().map_err(|e| e.into())?.abi(), deploymentoptions, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisher(packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindPackagesByNamePublisher)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInstallationManagerStatics<R, F: FnOnce(&IInstallationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InstallationManager, IInstallationManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInstallationManagerStatics2<R, F: FnOnce(&IInstallationManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InstallationManager, IInstallationManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for InstallationManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.InstallationManager";
}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageInstallResult(::windows::core::IUnknown);
impl PackageInstallResult {
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Deployment\"`*"]
    #[cfg(feature = "Management_Deployment")]
    pub fn InstallState(&self) -> ::windows::core::Result<super::super::super::Management::Deployment::PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstallState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackageInstallResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageInstallResult {
    type Vtable = IPackageInstallResult_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageInstallResult {
    const IID: ::windows::core::GUID = <IPackageInstallResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageInstallResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.PackageInstallResult";
}
::windows::core::interface_hierarchy!(PackageInstallResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
