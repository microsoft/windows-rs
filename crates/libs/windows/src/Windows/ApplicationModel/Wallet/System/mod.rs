#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IWalletItemSystemStore(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IWalletItemSystemStore {
    type Vtable = IWalletItemSystemStore_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IWalletItemSystemStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x522e2bff_96a2_4a17_8d19_fe1d9f837561);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStore_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetItemsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DeleteAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub ImportItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    ImportItemAsync: usize,
    #[cfg(feature = "deprecated")]
    pub GetAppStatusForItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut WalletItemAppAssociation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetAppStatusForItem: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub LaunchAppForItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    LaunchAppForItemAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IWalletItemSystemStore2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IWalletItemSystemStore2 {
    type Vtable = IWalletItemSystemStore2_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IWalletItemSystemStore2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf98d3a4e_be00_4fdd_9734_6c113c1ac1cb);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStore2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ItemsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ItemsChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveItemsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveItemsChanged: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IWalletManagerSystemStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IWalletManagerSystemStatics {
    type Vtable = IWalletManagerSystemStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IWalletManagerSystemStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee8eb89_2634_4b9a_8b23_ee8903c91fe0);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IWalletManagerSystemStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestStoreAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Wallet_System\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct WalletItemSystemStore(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl WalletItemSystemStore {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetItemsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetItemsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn DeleteAsync(&self, item: &super::WalletItem) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn ImportItemAsync<P0, E0>(&self, stream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::WalletItem>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImportItemAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetAppStatusForItem(&self, item: &super::WalletItem) -> ::windows::core::Result<WalletItemAppAssociation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAppStatusForItem)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn LaunchAppForItemAsync(&self, item: &super::WalletItem) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchAppForItemAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ItemsChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemsChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveItemsChanged(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveItemsChanged)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for WalletItemSystemStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for WalletItemSystemStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for WalletItemSystemStore {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for WalletItemSystemStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItemSystemStore").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for WalletItemSystemStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.System.WalletItemSystemStore;{522e2bff-96a2-4a17-8d19-fe1d9f837561})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for WalletItemSystemStore {
    type Vtable = IWalletItemSystemStore_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for WalletItemSystemStore {
    const IID: ::windows::core::GUID = <IWalletItemSystemStore as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for WalletItemSystemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletItemSystemStore";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(WalletItemSystemStore, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for WalletItemSystemStore {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for WalletItemSystemStore {}
#[doc = "*Required features: `\"ApplicationModel_Wallet_System\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct WalletManagerSystem;
#[cfg(feature = "deprecated")]
impl WalletManagerSystem {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WalletItemSystemStore>> {
        Self::IWalletManagerSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestStoreAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IWalletManagerSystemStatics<R, F: FnOnce(&IWalletManagerSystemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WalletManagerSystem, IWalletManagerSystemStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for WalletManagerSystem {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletManagerSystem";
}
#[doc = "*Required features: `\"ApplicationModel_Wallet_System\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WalletItemAppAssociation(pub i32);
#[cfg(feature = "deprecated")]
impl WalletItemAppAssociation {
    pub const None: Self = Self(0i32);
    pub const AppInstalled: Self = Self(1i32);
    pub const AppNotInstalled: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for WalletItemAppAssociation {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for WalletItemAppAssociation {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for WalletItemAppAssociation {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for WalletItemAppAssociation {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for WalletItemAppAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItemAppAssociation").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for WalletItemAppAssociation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.System.WalletItemAppAssociation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
