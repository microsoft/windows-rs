::windows_core::imp::com_interface!(IAppExtension, IAppExtension_Vtbl, 0x8450902c_15ed_4faf_93ea_2237bbf8cbd6);
#[repr(C)]
pub struct IAppExtension_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AppInfo: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtensionPropertiesAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtensionPropertiesAsync: usize,
    #[cfg(feature = "Storage")]
    pub GetPublicFolderAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GetPublicFolderAsync: usize,
}
::windows_core::imp::com_interface!(IAppExtension2, IAppExtension2_Vtbl, 0xab3b15f0_14f9_4b9f_9419_a349a242ef38);
#[repr(C)]
pub struct IAppExtension2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppUserModelId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAppExtensionCatalog, IAppExtensionCatalog_Vtbl, 0x97872032_8426_4ad1_9084_92e88c2da200);
#[repr(C)]
pub struct IAppExtensionCatalog_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    pub RequestRemovePackageAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PackageInstalled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageInstalled: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PackageUpdating: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageUpdating: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PackageUpdated: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageUpdated: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PackageUninstalling: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageUninstalling: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PackageStatusChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageStatusChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAppExtensionCatalogStatics, IAppExtensionCatalogStatics_Vtbl, 0x3c36668a_5f18_4f0b_9ce5_cab61d196f11);
#[repr(C)]
pub struct IAppExtensionCatalogStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Open: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAppExtensionPackageInstalledEventArgs, IAppExtensionPackageInstalledEventArgs_Vtbl, 0x39e59234_3351_4a8d_9745_e7d3dd45bc48);
#[repr(C)]
pub struct IAppExtensionPackageInstalledEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
::windows_core::imp::com_interface!(IAppExtensionPackageStatusChangedEventArgs, IAppExtensionPackageStatusChangedEventArgs_Vtbl, 0x1ce17433_1153_44fd_87b1_8ae1050303df);
#[repr(C)]
pub struct IAppExtensionPackageStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAppExtensionPackageUninstallingEventArgs, IAppExtensionPackageUninstallingEventArgs_Vtbl, 0x60f160c5_171e_40ff_ae98_ab2c20dd4d75);
#[repr(C)]
pub struct IAppExtensionPackageUninstallingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAppExtensionPackageUpdatedEventArgs, IAppExtensionPackageUpdatedEventArgs_Vtbl, 0x3a83c43f_797e_44b5_ba24_a4c8b5a543d7);
#[repr(C)]
pub struct IAppExtensionPackageUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
::windows_core::imp::com_interface!(IAppExtensionPackageUpdatingEventArgs, IAppExtensionPackageUpdatingEventArgs_Vtbl, 0x7ed59329_1a65_4800_a700_b321009e306a);
#[repr(C)]
pub struct IAppExtensionPackageUpdatingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppExtension(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AppExtension, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl AppExtension {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppInfo(&self) -> ::windows_core::Result<super::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppInfo)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetExtensionPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IPropertySet>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetExtensionPropertiesAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn GetPublicFolderAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPublicFolderAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppExtension2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppUserModelId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for AppExtension {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppExtension {
    type Vtable = IAppExtension_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtension as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtension {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtension";
}
unsafe impl ::core::marker::Send for AppExtension {}
unsafe impl ::core::marker::Sync for AppExtension {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppExtensionCatalog(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AppExtensionCatalog, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl AppExtensionCatalog {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppExtension>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestRemovePackageAsync(&self, packagefullname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestRemovePackageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn PackageInstalled<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageInstalledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageInstalled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageInstalled(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageInstalled)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageUpdating<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageUpdating)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageUpdating(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageUpdating)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageUpdated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageUninstalling<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUninstallingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageUninstalling)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageUninstalling(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageUninstalling)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageStatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageStatusChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Open(appextensionname: &::windows_core::HSTRING) -> ::windows_core::Result<AppExtensionCatalog> {
        Self::IAppExtensionCatalogStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Open)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(appextensionname), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppExtensionCatalogStatics<R, F: FnOnce(&IAppExtensionCatalogStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppExtensionCatalog, IAppExtensionCatalogStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppExtensionCatalog {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppExtensionCatalog {
    type Vtable = IAppExtensionCatalog_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionCatalog as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionCatalog";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppExtensionPackageInstalledEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AppExtensionPackageInstalledEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl AppExtensionPackageInstalledEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Extensions)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for AppExtensionPackageInstalledEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppExtensionPackageInstalledEventArgs {
    type Vtable = IAppExtensionPackageInstalledEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageInstalledEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageInstalledEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs";
}
unsafe impl ::core::marker::Send for AppExtensionPackageInstalledEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageInstalledEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppExtensionPackageStatusChangedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AppExtensionPackageStatusChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl AppExtensionPackageStatusChangedEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for AppExtensionPackageStatusChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppExtensionPackageStatusChangedEventArgs {
    type Vtable = IAppExtensionPackageStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageStatusChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs";
}
unsafe impl ::core::marker::Send for AppExtensionPackageStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppExtensionPackageUninstallingEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AppExtensionPackageUninstallingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl AppExtensionPackageUninstallingEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for AppExtensionPackageUninstallingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppExtensionPackageUninstallingEventArgs {
    type Vtable = IAppExtensionPackageUninstallingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageUninstallingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs";
}
unsafe impl ::core::marker::Send for AppExtensionPackageUninstallingEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUninstallingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppExtensionPackageUpdatedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AppExtensionPackageUpdatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl AppExtensionPackageUpdatedEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Extensions)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for AppExtensionPackageUpdatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppExtensionPackageUpdatedEventArgs {
    type Vtable = IAppExtensionPackageUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageUpdatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs";
}
unsafe impl ::core::marker::Send for AppExtensionPackageUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUpdatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppExtensionPackageUpdatingEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AppExtensionPackageUpdatingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl AppExtensionPackageUpdatingEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for AppExtensionPackageUpdatingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppExtensionPackageUpdatingEventArgs {
    type Vtable = IAppExtensionPackageUpdatingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageUpdatingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs";
}
unsafe impl ::core::marker::Send for AppExtensionPackageUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUpdatingEventArgs {}
