#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkplaceId(&self) -> ::windows::core::Result<i32>;
    fn EnrollmentValidFrom(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn EnrollmentValidTo(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Status(&self) -> ::windows::core::Result<EnterpriseStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnterprise {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.IEnterprise";
}
#[cfg(feature = "implement_exclusive")]
impl IEnterpriseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseImpl, const OFFSET: isize>() -> IEnterpriseVtbl {
        unsafe extern "system" fn Id<Impl: IEnterpriseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IEnterpriseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkplaceId<Impl: IEnterpriseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkplaceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnrollmentValidFrom<Impl: IEnterpriseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnrollmentValidFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnrollmentValidTo<Impl: IEnterpriseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnrollmentValidTo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IEnterpriseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EnterpriseStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnterprise>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Name::<Impl, OFFSET>, WorkplaceId::<Impl, OFFSET>, EnrollmentValidFrom::<Impl, OFFSET>, EnrollmentValidTo::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseEnrollmentManagerImpl: Sized {
    fn EnrolledEnterprises(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Enterprise>>;
    fn CurrentEnterprise(&self) -> ::windows::core::Result<Enterprise>;
    fn ValidateEnterprisesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn RequestEnrollmentAsync(&self, enrollmenttoken: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>>;
    fn RequestUnenrollmentAsync(&self, enterprise: &::core::option::Option<Enterprise>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnterpriseEnrollmentManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.IEnterpriseEnrollmentManager";
}
#[cfg(feature = "implement_exclusive")]
impl IEnterpriseEnrollmentManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseEnrollmentManagerImpl, const OFFSET: isize>() -> IEnterpriseEnrollmentManagerVtbl {
        unsafe extern "system" fn EnrolledEnterprises<Impl: IEnterpriseEnrollmentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnrolledEnterprises() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentEnterprise<Impl: IEnterpriseEnrollmentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentEnterprise() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateEnterprisesAsync<Impl: IEnterpriseEnrollmentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateEnterprisesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestEnrollmentAsync<Impl: IEnterpriseEnrollmentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enrollmenttoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestEnrollmentAsync(&*(&enrollmenttoken as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestUnenrollmentAsync<Impl: IEnterpriseEnrollmentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enterprise: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestUnenrollmentAsync(&*(&enterprise as *const <Enterprise as ::windows::core::Abi>::Abi as *const <Enterprise as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnterpriseEnrollmentManager>, ::windows::core::GetTrustLevel, EnrolledEnterprises::<Impl, OFFSET>, CurrentEnterprise::<Impl, OFFSET>, ValidateEnterprisesAsync::<Impl, OFFSET>, RequestEnrollmentAsync::<Impl, OFFSET>, RequestUnenrollmentAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseEnrollmentResultImpl: Sized {
    fn EnrolledEnterprise(&self) -> ::windows::core::Result<Enterprise>;
    fn Status(&self) -> ::windows::core::Result<EnterpriseEnrollmentStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnterpriseEnrollmentResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.IEnterpriseEnrollmentResult";
}
#[cfg(feature = "implement_exclusive")]
impl IEnterpriseEnrollmentResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseEnrollmentResultImpl, const OFFSET: isize>() -> IEnterpriseEnrollmentResultVtbl {
        unsafe extern "system" fn EnrolledEnterprise<Impl: IEnterpriseEnrollmentResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnrolledEnterprise() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IEnterpriseEnrollmentResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EnterpriseEnrollmentStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnterpriseEnrollmentResult>, ::windows::core::GetTrustLevel, EnrolledEnterprise::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInstallationManagerStaticsImpl: Sized {
    fn AddPackageAsync(&self, title: &::windows::core::HSTRING, sourcelocation: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>;
    fn AddPackagePreloadedAsync(&self, title: &::windows::core::HSTRING, sourcelocation: &::core::option::Option<super::super::super::Foundation::Uri>, instanceid: &::windows::core::HSTRING, offerid: &::windows::core::HSTRING, license: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>;
    fn GetPendingPackageInstalls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>>;
    fn FindPackagesForCurrentPublisher(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>;
    fn FindPackages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInstallationManagerStatics {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.IInstallationManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInstallationManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationManagerStaticsImpl, const OFFSET: isize>() -> IInstallationManagerStaticsVtbl {
        unsafe extern "system" fn AddPackageAsync<Impl: IInstallationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sourcelocation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageAsync(&*(&title as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&sourcelocation as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPackagePreloadedAsync<Impl: IInstallationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sourcelocation: ::windows::core::RawPtr, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, offerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, license: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackagePreloadedAsync(
                &*(&title as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&sourcelocation as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&instanceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&offerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&license as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPendingPackageInstalls<Impl: IInstallationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPendingPackageInstalls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesForCurrentPublisher<Impl: IInstallationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesForCurrentPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackages<Impl: IInstallationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInstallationManagerStatics>, ::windows::core::GetTrustLevel, AddPackageAsync::<Impl, OFFSET>, AddPackagePreloadedAsync::<Impl, OFFSET>, GetPendingPackageInstalls::<Impl, OFFSET>, FindPackagesForCurrentPublisher::<Impl, OFFSET>, FindPackages::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInstallationManagerStatics2Impl: Sized {
    fn RemovePackageAsync(&self, packagefullname: &::windows::core::HSTRING, removaloptions: super::super::super::Management::Deployment::RemovalOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>;
    fn RegisterPackageAsync(&self, manifesturi: &::core::option::Option<super::super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Uri>>, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>;
    fn FindPackagesByNamePublisher(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInstallationManagerStatics2 {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.IInstallationManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInstallationManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationManagerStatics2Impl, const OFFSET: isize>() -> IInstallationManagerStatics2Vtbl {
        unsafe extern "system" fn RemovePackageAsync<Impl: IInstallationManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, removaloptions: super::super::super::Management::Deployment::RemovalOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePackageAsync(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), removaloptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPackageAsync<Impl: IInstallationManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifesturi: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPackageAsync(
                &*(&manifesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                deploymentoptions,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByNamePublisher<Impl: IInstallationManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByNamePublisher(&*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagepublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInstallationManagerStatics2>, ::windows::core::GetTrustLevel, RemovePackageAsync::<Impl, OFFSET>, RegisterPackageAsync::<Impl, OFFSET>, FindPackagesByNamePublisher::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageInstallResultImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallState(&self) -> ::windows::core::Result<super::super::super::Management::Deployment::PackageInstallState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageInstallResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.IPackageInstallResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageInstallResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageInstallResultImpl, const OFFSET: isize>() -> IPackageInstallResultVtbl {
        unsafe extern "system" fn ProductId<Impl: IPackageInstallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallState<Impl: IPackageInstallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Management::Deployment::PackageInstallState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageInstallResult>, ::windows::core::GetTrustLevel, ProductId::<Impl, OFFSET>, InstallState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageInstallResult2Impl: Sized {
    fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageInstallResult2 {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.IPackageInstallResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageInstallResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageInstallResult2Impl, const OFFSET: isize>() -> IPackageInstallResult2Vtbl {
        unsafe extern "system" fn ErrorText<Impl: IPackageInstallResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageInstallResult2>, ::windows::core::GetTrustLevel, ErrorText::<Impl, OFFSET>)
    }
}
