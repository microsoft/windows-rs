#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtension(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppExtension {
    type Vtable = IAppExtension_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExtension {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8450902c_15ed_4faf_93ea_2237bbf8cbd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtension_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IAppExtension2 {
    type Vtable = IAppExtension2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExtension2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab3b15f0_14f9_4b9f_9419_a349a242ef38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtension2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionCatalog(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppExtensionCatalog {
    type Vtable = IAppExtensionCatalog_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExtensionCatalog {
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
    pub RequestRemovePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IAppExtensionCatalogStatics {
    type Vtable = IAppExtensionCatalogStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExtensionCatalogStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c36668a_5f18_4f0b_9ce5_cab61d196f11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionCatalogStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appextensionname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageInstalledEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppExtensionPackageInstalledEventArgs {
    type Vtable = IAppExtensionPackageInstalledEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExtensionPackageInstalledEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39e59234_3351_4a8d_9745_e7d3dd45bc48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageInstalledEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppExtensionPackageStatusChangedEventArgs {
    type Vtable = IAppExtensionPackageStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExtensionPackageStatusChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ce17433_1153_44fd_87b1_8ae1050303df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageStatusChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageUninstallingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppExtensionPackageUninstallingEventArgs {
    type Vtable = IAppExtensionPackageUninstallingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExtensionPackageUninstallingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60f160c5_171e_40ff_ae98_ab2c20dd4d75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageUninstallingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppExtensionPackageUpdatedEventArgs {
    type Vtable = IAppExtensionPackageUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExtensionPackageUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a83c43f_797e_44b5_ba24_a4c8b5a543d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppExtensionPackageUpdatingEventArgs {
    type Vtable = IAppExtensionPackageUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExtensionPackageUpdatingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed59329_1a65_4800_a700_b321009e306a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageUpdatingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtension(::windows::core::IUnknown);
impl AppExtension {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppInfo(&self) -> ::windows::core::Result<super::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetExtensionPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IPropertySet>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetExtensionPropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetPublicFolderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPublicFolderAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppExtension2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppUserModelId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppExtension {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtension;{8450902c-15ed-4faf-93ea-2237bbf8cbd6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppExtension {
    type Vtable = IAppExtension_Vtbl;
}
unsafe impl ::windows::core::Interface for AppExtension {
    const IID: ::windows::core::GUID = <IAppExtension as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppExtension {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtension";
}
::windows::core::interface_hierarchy!(AppExtension, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestRemovePackageAsync(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestRemovePackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(packagefullname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageInstalled(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageInstalledEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageInstalled)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageInstalled(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageInstalled)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUpdating(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageUpdating)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUpdating(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageUpdating)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUpdated(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUninstalling(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUninstallingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageUninstalling)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUninstalling(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageUninstalling)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageStatusChanged(&self, handler: &super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageStatusChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageStatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Open(appextensionname: &::windows::core::HSTRING) -> ::windows::core::Result<AppExtensionCatalog> {
        Self::IAppExtensionCatalogStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Open)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appextensionname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppExtensionCatalogStatics<R, F: FnOnce(&IAppExtensionCatalogStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppExtensionCatalog, IAppExtensionCatalogStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppExtensionCatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppExtensionCatalog {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionCatalog;{97872032-8426-4ad1-9084-92e88c2da200})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppExtensionCatalog {
    type Vtable = IAppExtensionCatalog_Vtbl;
}
unsafe impl ::windows::core::Interface for AppExtensionCatalog {
    const IID: ::windows::core::GUID = <IAppExtensionCatalog as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionCatalog";
}
::windows::core::interface_hierarchy!(AppExtensionCatalog, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageInstalledEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageInstalledEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppExtensionName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Extensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageInstalledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppExtensionPackageInstalledEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs;{39e59234-3351-4a8d-9745-e7d3dd45bc48})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppExtensionPackageInstalledEventArgs {
    type Vtable = IAppExtensionPackageInstalledEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppExtensionPackageInstalledEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageInstalledEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageInstalledEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs";
}
::windows::core::interface_hierarchy!(AppExtensionPackageInstalledEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageInstalledEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageInstalledEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageStatusChangedEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageStatusChangedEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppExtensionName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppExtensionPackageStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs;{1ce17433-1153-44fd-87b1-8ae1050303df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppExtensionPackageStatusChangedEventArgs {
    type Vtable = IAppExtensionPackageStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppExtensionPackageStatusChangedEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageStatusChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs";
}
::windows::core::interface_hierarchy!(AppExtensionPackageStatusChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageStatusChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageUninstallingEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageUninstallingEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppExtensionName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppExtensionPackageUninstallingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs;{60f160c5-171e-40ff-ae98-ab2c20dd4d75})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppExtensionPackageUninstallingEventArgs {
    type Vtable = IAppExtensionPackageUninstallingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppExtensionPackageUninstallingEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageUninstallingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs";
}
::windows::core::interface_hierarchy!(AppExtensionPackageUninstallingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageUninstallingEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUninstallingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageUpdatedEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageUpdatedEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppExtensionName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Extensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppExtensionPackageUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs;{3a83c43f-797e-44b5-ba24-a4c8b5a543d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppExtensionPackageUpdatedEventArgs {
    type Vtable = IAppExtensionPackageUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppExtensionPackageUpdatedEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageUpdatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs";
}
::windows::core::interface_hierarchy!(AppExtensionPackageUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUpdatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_AppExtensions\"`*"]
#[repr(transparent)]
pub struct AppExtensionPackageUpdatingEventArgs(::windows::core::IUnknown);
impl AppExtensionPackageUpdatingEventArgs {
    pub fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppExtensionName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppExtensionPackageUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs;{7ed59329-1a65-4800-a700-b321009e306a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppExtensionPackageUpdatingEventArgs {
    type Vtable = IAppExtensionPackageUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppExtensionPackageUpdatingEventArgs {
    const IID: ::windows::core::GUID = <IAppExtensionPackageUpdatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppExtensionPackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs";
}
::windows::core::interface_hierarchy!(AppExtensionPackageUpdatingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExtensionPackageUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUpdatingEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
