#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItemSystemStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWalletItemSystemStore {
    type Vtable = IWalletItemSystemStoreVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x522e2bff_96a2_4a17_8d19_fe1d9f837561);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStoreVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut WalletItemAppAssociation) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItemSystemStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWalletItemSystemStore2 {
    type Vtable = IWalletItemSystemStore2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf98d3a4e_be00_4fdd_9734_6c113c1ac1cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStore2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletManagerSystemStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWalletManagerSystemStatics {
    type Vtable = IWalletManagerSystemStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee8eb89_2634_4b9a_8b23_ee8903c91fe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletManagerSystemStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: 'ApplicationModel_Wallet_System'*"]
#[repr(transparent)]
pub struct WalletItemAppAssociation(pub i32);
impl WalletItemAppAssociation {
    pub const None: Self = Self(0i32);
    pub const AppInstalled: Self = Self(1i32);
    pub const AppNotInstalled: Self = Self(2i32);
}
impl ::core::marker::Copy for WalletItemAppAssociation {}
impl ::core::clone::Clone for WalletItemAppAssociation {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WalletItemAppAssociation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WalletItemAppAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletItemAppAssociation {}
impl ::core::fmt::Debug for WalletItemAppAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItemAppAssociation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletItemAppAssociation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.System.WalletItemAppAssociation;i4)");
}
impl ::windows::core::DefaultType for WalletItemAppAssociation {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Wallet_System'*"]
#[repr(transparent)]
pub struct WalletItemSystemStore(::windows::core::IUnknown);
impl WalletItemSystemStore {
    #[doc = "*Required features: 'ApplicationModel_Wallet_System', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetItemsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::WalletItem>>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Wallet_System', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::WalletItem>>(&self, item: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Wallet_System', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ImportItemAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, stream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::WalletItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::WalletItem>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Wallet_System'*"]
    pub fn GetAppStatusForItem<'a, Param0: ::windows::core::IntoParam<'a, super::WalletItem>>(&self, item: Param0) -> ::windows::core::Result<WalletItemAppAssociation> {
        let this = self;
        unsafe {
            let mut result__: WalletItemAppAssociation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<WalletItemAppAssociation>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Wallet_System', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAppForItemAsync<'a, Param0: ::windows::core::IntoParam<'a, super::WalletItem>>(&self, item: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Wallet_System', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Wallet_System', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WalletItemSystemStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletItemSystemStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletItemSystemStore {}
impl ::core::fmt::Debug for WalletItemSystemStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItemSystemStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletItemSystemStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.System.WalletItemSystemStore;{522e2bff-96a2-4a17-8d19-fe1d9f837561})");
}
unsafe impl ::windows::core::Interface for WalletItemSystemStore {
    type Vtable = IWalletItemSystemStoreVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x522e2bff_96a2_4a17_8d19_fe1d9f837561);
}
impl ::windows::core::RuntimeName for WalletItemSystemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletItemSystemStore";
}
impl ::core::convert::From<WalletItemSystemStore> for ::windows::core::IUnknown {
    fn from(value: WalletItemSystemStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletItemSystemStore> for ::windows::core::IUnknown {
    fn from(value: &WalletItemSystemStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WalletItemSystemStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WalletItemSystemStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WalletItemSystemStore> for ::windows::core::IInspectable {
    fn from(value: WalletItemSystemStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletItemSystemStore> for ::windows::core::IInspectable {
    fn from(value: &WalletItemSystemStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WalletItemSystemStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WalletItemSystemStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WalletItemSystemStore {}
unsafe impl ::core::marker::Sync for WalletItemSystemStore {}
#[doc = "*Required features: 'ApplicationModel_Wallet_System'*"]
pub struct WalletManagerSystem {}
impl WalletManagerSystem {
    #[doc = "*Required features: 'ApplicationModel_Wallet_System', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WalletItemSystemStore>> {
        Self::IWalletManagerSystemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WalletItemSystemStore>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWalletManagerSystemStatics<R, F: FnOnce(&IWalletManagerSystemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WalletManagerSystem, IWalletManagerSystemStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WalletManagerSystem {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletManagerSystem";
}
