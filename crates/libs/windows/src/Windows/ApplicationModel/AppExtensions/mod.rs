#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtension(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExtension {
    type Vtable = IAppExtension_Vtbl;
}
impl ::core::clone::Clone for IAppExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppExtension {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8450902c_15ed_4faf_93ea_2237bbf8cbd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtension_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtensionPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtensionPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetPublicFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetPublicFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtension2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExtension2 {
    type Vtable = IAppExtension2_Vtbl;
}
impl ::core::clone::Clone for IAppExtension2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppExtension2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab3b15f0_14f9_4b9f_9419_a349a242ef38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtension2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionCatalog(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExtensionCatalog {
    type Vtable = IAppExtensionCatalog_Vtbl;
}
impl ::core::clone::Clone for IAppExtensionCatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppExtensionCatalog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97872032_8426_4ad1_9084_92e88c2da200);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionCatalog_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestRemovePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRemovePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PackageInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageInstalled: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageInstalled: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub PackageStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionCatalogStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExtensionCatalogStatics {
    type Vtable = IAppExtensionCatalogStatics_Vtbl;
}
impl ::core::clone::Clone for IAppExtensionCatalogStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppExtensionCatalogStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c36668a_5f18_4f0b_9ce5_cab61d196f11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionCatalogStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appextensionname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageInstalledEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExtensionPackageInstalledEventArgs {
    type Vtable = IAppExtensionPackageInstalledEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppExtensionPackageInstalledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppExtensionPackageInstalledEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39e59234_3351_4a8d_9745_e7d3dd45bc48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageInstalledEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExtensionPackageStatusChangedEventArgs {
    type Vtable = IAppExtensionPackageStatusChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppExtensionPackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppExtensionPackageStatusChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ce17433_1153_44fd_87b1_8ae1050303df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageStatusChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageUninstallingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExtensionPackageUninstallingEventArgs {
    type Vtable = IAppExtensionPackageUninstallingEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppExtensionPackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppExtensionPackageUninstallingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60f160c5_171e_40ff_ae98_ab2c20dd4d75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageUninstallingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExtensionPackageUpdatedEventArgs {
    type Vtable = IAppExtensionPackageUpdatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppExtensionPackageUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppExtensionPackageUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a83c43f_797e_44b5_ba24_a4c8b5a543d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExtensionPackageUpdatingEventArgs {
    type Vtable = IAppExtensionPackageUpdatingEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppExtensionPackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppExtensionPackageUpdatingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed59329_1a65_4800_a700_b321009e306a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageUpdatingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtension(::windows::core::IUnknown);
impl AppExtension {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Package>();
            (::windows::core::Interface::vtable(this).Package)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppInfo(&self) -> ::windows::core::Result<super::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::AppInfo>();
            (::windows::core::Interface::vtable(this).AppInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetExtensionPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IPropertySet>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IPropertySet>>();
            (::windows::core::Interface::vtable(this).GetExtensionPropertiesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetPublicFolderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFolder>>();
            (::windows::core::Interface::vtable(this).GetPublicFolderAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppExtension2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppUserModelId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtension {}
impl ::core::fmt::Debug for AppExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtension").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppExtension {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtension;{8450902c-15ed-4faf-93ea-2237bbf8cbd6})");
}
impl ::core::clone::Clone for AppExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppExtension {
    type Vtable = IAppExtension_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppExtension {
    const IID: ::windows::core::GUID = <IAppExtension as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppExtension {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtension";
}
::windows::imp::interface_hierarchy!(AppExtension, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtension {}
unsafe impl ::core::marker::Sync for AppExtension {}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionCatalog(::windows::core::IUnknown);
impl AppExtensionCatalog {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppExtension>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppExtension>>>();
            (::windows::core::Interface::vtable(this).FindAllAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestRemovePackageAsync(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).RequestRemovePackageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageInstalled(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageInstalledEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PackageInstalled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageInstalled(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageInstalled)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUpdating(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PackageUpdating)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUpdating(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageUpdating)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUpdated(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PackageUpdated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageUpdated)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUninstalling(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUninstallingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PackageUninstalling)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUninstalling(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageUninstalling)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageStatusChanged(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageStatusChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PackageStatusChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageStatusChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Open(appextensionname: &::windows::core::HSTRING) -> ::windows::core::Result<AppExtensionCatalog> {
        Self::IAppExtensionCatalogStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppExtensionCatalog>();
            (::windows::core::Interface::vtable(this).Open)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(appextensionname), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppExtensionCatalogStatics<R, F: FnOnce(&IAppExtensionCatalogStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppExtensionCatalog, IAppExtensionCatalogStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AppExtensionCatalog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionCatalog {}
impl ::core::fmt::Debug for AppExtensionCatalog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionCatalog").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppExtensionCatalog {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionCatalog;{97872032-8426-4ad1-9084-92e88c2da200})");
}
impl ::core::clone::Clone for AppExtensionCatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppExtensionCatalog {
    type Vtable = IAppExtensionCatalog_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppExtensionCatalog {
    const IID: ::windows::core::GUID = <IAppExtensionCatalog as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionCatalog";
}
::windows::imp::interface_hierarchy!(AppExtensionCatalog, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageInstalledEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageInstalledEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppExtensionName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Package>();
            (::windows::core::Interface::vtable(this).Package)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AppExtension>>();
            (::windows::core::Interface::vtable(this).Extensions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageInstalledEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageInstalledEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageInstalledEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageInstalledEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppExtensionPackageInstalledEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs;{39e59234-3351-4a8d-9745-e7d3dd45bc48})");
}
impl ::core::clone::Clone for AppExtensionPackageInstalledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppExtensionPackageInstalledEventArgs {
    type Vtable = IAppExtensionPackageInstalledEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppExtensionPackageInstalledEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageInstalledEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageInstalledEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs";
}
::windows::imp::interface_hierarchy!(AppExtensionPackageInstalledEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageInstalledEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageInstalledEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageStatusChangedEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageStatusChangedEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppExtensionName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Package>();
            (::windows::core::Interface::vtable(this).Package)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageStatusChangedEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageStatusChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppExtensionPackageStatusChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs;{1ce17433-1153-44fd-87b1-8ae1050303df})");
}
impl ::core::clone::Clone for AppExtensionPackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppExtensionPackageStatusChangedEventArgs {
    type Vtable = IAppExtensionPackageStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppExtensionPackageStatusChangedEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageStatusChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs";
}
::windows::imp::interface_hierarchy!(AppExtensionPackageStatusChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageStatusChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageUninstallingEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageUninstallingEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppExtensionName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Package>();
            (::windows::core::Interface::vtable(this).Package)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageUninstallingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageUninstallingEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageUninstallingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageUninstallingEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppExtensionPackageUninstallingEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs;{60f160c5-171e-40ff-ae98-ab2c20dd4d75})");
}
impl ::core::clone::Clone for AppExtensionPackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppExtensionPackageUninstallingEventArgs {
    type Vtable = IAppExtensionPackageUninstallingEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppExtensionPackageUninstallingEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageUninstallingEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs";
}
::windows::imp::interface_hierarchy!(AppExtensionPackageUninstallingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageUninstallingEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUninstallingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageUpdatedEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageUpdatedEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppExtensionName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Package>();
            (::windows::core::Interface::vtable(this).Package)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AppExtension>>();
            (::windows::core::Interface::vtable(this).Extensions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageUpdatedEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageUpdatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppExtensionPackageUpdatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs;{3a83c43f-797e-44b5-ba24-a4c8b5a543d7})");
}
impl ::core::clone::Clone for AppExtensionPackageUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppExtensionPackageUpdatedEventArgs {
    type Vtable = IAppExtensionPackageUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppExtensionPackageUpdatedEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageUpdatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageUpdatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs";
}
::windows::imp::interface_hierarchy!(AppExtensionPackageUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUpdatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageUpdatingEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageUpdatingEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppExtensionName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Package>();
            (::windows::core::Interface::vtable(this).Package)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageUpdatingEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageUpdatingEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppExtensionPackageUpdatingEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs;{7ed59329-1a65-4800-a700-b321009e306a})");
}
impl ::core::clone::Clone for AppExtensionPackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppExtensionPackageUpdatingEventArgs {
    type Vtable = IAppExtensionPackageUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppExtensionPackageUpdatingEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageUpdatingEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs";
}
::windows::imp::interface_hierarchy!(AppExtensionPackageUpdatingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUpdatingEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
