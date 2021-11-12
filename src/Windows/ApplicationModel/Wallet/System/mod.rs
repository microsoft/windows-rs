#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IWalletItemSystemStore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWalletItemSystemStore {
    type Vtable = IWalletItemSystemStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x522e2bff_96a2_4a17_8d19_fe1d9f837561);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: ::windows::core::RawPtr, result__: *mut WalletItemAppAssociation) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWalletItemSystemStore2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWalletItemSystemStore2 {
    type Vtable = IWalletItemSystemStore2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf98d3a4e_be00_4fdd_9734_6c113c1ac1cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStore2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWalletManagerSystemStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWalletManagerSystemStatics {
    type Vtable = IWalletManagerSystemStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee8eb89_2634_4b9a_8b23_ee8903c91fe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletManagerSystemStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WalletItemAppAssociation(pub i32);
impl WalletItemAppAssociation {
    pub const None: WalletItemAppAssociation = WalletItemAppAssociation(0i32);
    pub const AppInstalled: WalletItemAppAssociation = WalletItemAppAssociation(1i32);
    pub const AppNotInstalled: WalletItemAppAssociation = WalletItemAppAssociation(2i32);
}
impl ::core::convert::From<i32> for WalletItemAppAssociation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WalletItemAppAssociation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WalletItemAppAssociation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.System.WalletItemAppAssociation;i4)");
}
impl ::windows::core::DefaultType for WalletItemAppAssociation {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WalletItemSystemStore(pub ::windows::core::IInspectable);
impl WalletItemSystemStore {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetItemsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::WalletItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::WalletItem>>(&self, item: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ImportItemAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, stream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::WalletItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::WalletItem>>(result__)
        }
    }
    pub fn GetAppStatusForItem<'a, Param0: ::windows::core::IntoParam<'a, super::WalletItem>>(&self, item: Param0) -> ::windows::core::Result<WalletItemAppAssociation> {
        let this = self;
        unsafe {
            let mut result__: WalletItemAppAssociation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<WalletItemAppAssociation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchAppForItemAsync<'a, Param0: ::windows::core::IntoParam<'a, super::WalletItem>>(&self, item: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ItemsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for WalletItemSystemStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.System.WalletItemSystemStore;{522e2bff-96a2-4a17-8d19-fe1d9f837561})");
}
unsafe impl ::windows::core::Interface for WalletItemSystemStore {
    type Vtable = IWalletItemSystemStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x522e2bff_96a2_4a17_8d19_fe1d9f837561);
}
impl ::windows::core::RuntimeName for WalletItemSystemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletItemSystemStore";
}
impl ::core::convert::From<WalletItemSystemStore> for ::windows::core::IUnknown {
    fn from(value: WalletItemSystemStore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WalletItemSystemStore> for ::windows::core::IUnknown {
    fn from(value: &WalletItemSystemStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WalletItemSystemStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WalletItemSystemStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WalletItemSystemStore> for ::windows::core::IInspectable {
    fn from(value: WalletItemSystemStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WalletItemSystemStore> for ::windows::core::IInspectable {
    fn from(value: &WalletItemSystemStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WalletItemSystemStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WalletItemSystemStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WalletItemSystemStore {}
unsafe impl ::core::marker::Sync for WalletItemSystemStore {}
pub struct WalletManagerSystem {}
impl WalletManagerSystem {
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WalletItemSystemStore>> {
        Self::IWalletManagerSystemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WalletItemSystemStore>>(result__)
        })
    }
    pub fn IWalletManagerSystemStatics<R, F: FnOnce(&IWalletManagerSystemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WalletManagerSystem, IWalletManagerSystemStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WalletManagerSystem {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletManagerSystem";
}
