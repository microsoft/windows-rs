#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnterprise(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnterprise {
    type Vtable = IEnterprise_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnterprise {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96592f8d_856c_4426_a947_b06307718078);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterprise_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WorkplaceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnrollmentValidFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnrollmentValidFrom: usize,
    #[cfg(feature = "Foundation")]
    pub EnrollmentValidTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnrollmentValidTo: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnterpriseStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnterpriseEnrollmentManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnterpriseEnrollmentManager {
    type Vtable = IEnterpriseEnrollmentManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnterpriseEnrollmentManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20f9f390_2c69_41d8_88e6_e4b3884026cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EnrolledEnterprises: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnrolledEnterprises: usize,
    pub CurrentEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ValidateEnterprisesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidateEnterprisesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestEnrollmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enrollmenttoken: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestEnrollmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUnenrollmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enterprise: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnenrollmentAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnterpriseEnrollmentResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnterpriseEnrollmentResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ff71ce6_90db_4342_b326_1729aa91301c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnrolledEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnterpriseEnrollmentStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInstallationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInstallationManagerStatics {
    type Vtable = IInstallationManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInstallationManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x929aa738_8d49_42ac_80c9_b4ad793c43f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AddPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sourcelocation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddPackagePreloadedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sourcelocation: *mut ::core::ffi::c_void, instanceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, offerid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, license: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackagePreloadedAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPendingPackageInstalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPendingPackageInstalls: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesForCurrentPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesForCurrentPublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInstallationManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInstallationManagerStatics2 {
    type Vtable = IInstallationManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInstallationManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c6c2cbd_fa4a_4c8e_ab97_d959452f19e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))]
    pub RemovePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, removaloptions: super::super::super::Management::Deployment::RemovalOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Management_Deployment")))]
    RemovePackageAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub RegisterPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    RegisterPackageAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, packagepublisher: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPackageInstallResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageInstallResult {
    type Vtable = IPackageInstallResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPackageInstallResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33e8eed5_0f7e_4473_967c_7d6e1c0e7de1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Management::Deployment::PackageInstallState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    InstallState: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPackageInstallResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageInstallResult2 {
    type Vtable = IPackageInstallResult2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPackageInstallResult2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7149d909_3ff9_41ed_a717_2bc65ffc61d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Enterprise(::windows_core::IUnknown);
impl Enterprise {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WorkplaceId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WorkplaceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnrollmentValidFrom(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnrollmentValidFrom)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnrollmentValidTo(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnrollmentValidTo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<EnterpriseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for Enterprise {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.Enterprise;{96592f8d-856c-4426-a947-b06307718078})");
}
unsafe impl ::windows_core::Interface for Enterprise {
    type Vtable = IEnterprise_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Enterprise {
    const IID: ::windows_core::GUID = <IEnterprise as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Enterprise {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.Enterprise";
}
::windows_core::imp::interface_hierarchy!(Enterprise, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Enterprise {}
unsafe impl ::core::marker::Sync for Enterprise {}
pub struct EnterpriseEnrollmentManager;
impl EnterpriseEnrollmentManager {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnrolledEnterprises() -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<Enterprise>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnrolledEnterprises)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CurrentEnterprise() -> ::windows_core::Result<Enterprise> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentEnterprise)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ValidateEnterprisesAsync() -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValidateEnterprisesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestEnrollmentAsync(enrollmenttoken: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestEnrollmentAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(enrollmenttoken), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUnenrollmentAsync<P0>(enterprise: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Enterprise>,
    {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestUnenrollmentAsync)(::windows_core::Interface::as_raw(this), enterprise.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnterpriseEnrollmentManager<R, F: FnOnce(&IEnterpriseEnrollmentManager) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<EnterpriseEnrollmentManager, IEnterpriseEnrollmentManager> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for EnterpriseEnrollmentManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct EnterpriseEnrollmentResult(::windows_core::IUnknown);
impl EnterpriseEnrollmentResult {
    pub fn EnrolledEnterprise(&self) -> ::windows_core::Result<Enterprise> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnrolledEnterprise)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<EnterpriseEnrollmentStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for EnterpriseEnrollmentResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult;{9ff71ce6-90db-4342-b326-1729aa91301c})");
}
unsafe impl ::windows_core::Interface for EnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EnterpriseEnrollmentResult {
    const IID: ::windows_core::GUID = <IEnterpriseEnrollmentResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EnterpriseEnrollmentResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult";
}
::windows_core::imp::interface_hierarchy!(EnterpriseEnrollmentResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
pub struct InstallationManager;
impl InstallationManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AddPackageAsync<P0>(title: &::windows_core::HSTRING, sourcelocation: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddPackageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(title), sourcelocation.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AddPackagePreloadedAsync<P0, P1>(title: &::windows_core::HSTRING, sourcelocation: P0, instanceid: &::windows_core::HSTRING, offerid: &::windows_core::HSTRING, license: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddPackagePreloadedAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(title), sourcelocation.into_param().abi(), ::core::mem::transmute_copy(instanceid), ::core::mem::transmute_copy(offerid), license.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPendingPackageInstalls() -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPendingPackageInstalls)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesForCurrentPublisher() -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesForCurrentPublisher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackages() -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindPackages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Management_Deployment\"`"]
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))]
    pub fn RemovePackageAsync(packagefullname: &::windows_core::HSTRING, removaloptions: super::super::super::Management::Deployment::RemovalOptions) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemovePackageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), removaloptions, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Management_Deployment\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn RegisterPackageAsync<P0, P1>(manifesturi: P0, dependencypackageuris: P1, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
        P1: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Uri>>,
    {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPackageAsync)(::windows_core::Interface::as_raw(this), manifesturi.into_param().abi(), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisher(packagename: &::windows_core::HSTRING, packagepublisher: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByNamePublisher)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInstallationManagerStatics<R, F: FnOnce(&IInstallationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InstallationManager, IInstallationManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInstallationManagerStatics2<R, F: FnOnce(&IInstallationManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InstallationManager, IInstallationManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for InstallationManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.InstallationManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PackageInstallResult(::windows_core::IUnknown);
impl PackageInstallResult {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Management_Deployment\"`"]
    #[cfg(feature = "Management_Deployment")]
    pub fn InstallState(&self) -> ::windows_core::Result<super::super::super::Management::Deployment::PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstallState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPackageInstallResult2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PackageInstallResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.PackageInstallResult;{33e8eed5-0f7e-4473-967c-7d6e1c0e7de1})");
}
unsafe impl ::windows_core::Interface for PackageInstallResult {
    type Vtable = IPackageInstallResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PackageInstallResult {
    const IID: ::windows_core::GUID = <IPackageInstallResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PackageInstallResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.PackageInstallResult";
}
::windows_core::imp::interface_hierarchy!(PackageInstallResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::TypeKind for EnterpriseEnrollmentStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EnterpriseEnrollmentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseEnrollmentStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EnterpriseEnrollmentStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Management.Deployment.EnterpriseEnrollmentStatus;i4)");
}
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
impl ::windows_core::TypeKind for EnterpriseStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EnterpriseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EnterpriseStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Management.Deployment.EnterpriseStatus;i4)");
}
