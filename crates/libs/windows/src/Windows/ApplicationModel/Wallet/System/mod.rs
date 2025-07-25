windows_core::imp::define_interface!(IWalletItemSystemStore, IWalletItemSystemStore_Vtbl, 0x522e2bff_96a2_4a17_8d19_fe1d9f837561);
impl windows_core::RuntimeType for IWalletItemSystemStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ImportItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportItemAsync: usize,
    pub GetAppStatusForItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut WalletItemAppAssociation) -> windows_core::HRESULT,
    pub LaunchAppForItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWalletItemSystemStore2, IWalletItemSystemStore2_Vtbl, 0xf98d3a4e_be00_4fdd_9734_6c113c1ac1cb);
impl windows_core::RuntimeType for IWalletItemSystemStore2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStore2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWalletManagerSystemStatics, IWalletManagerSystemStatics_Vtbl, 0xbee8eb89_2634_4b9a_8b23_ee8903c91fe0);
impl windows_core::RuntimeType for IWalletManagerSystemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletManagerSystemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WalletItemAppAssociation(pub i32);
impl WalletItemAppAssociation {
    pub const None: Self = Self(0i32);
    pub const AppInstalled: Self = Self(1i32);
    pub const AppNotInstalled: Self = Self(2i32);
}
impl windows_core::TypeKind for WalletItemAppAssociation {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WalletItemAppAssociation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.System.WalletItemAppAssociation;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletItemSystemStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WalletItemSystemStore, windows_core::IUnknown, windows_core::IInspectable);
impl WalletItemSystemStore {
    pub fn GetItemsAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsync<P0>(&self, item: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::WalletItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportItemAsync<P0>(&self, stream: P0) -> windows_core::Result<windows_future::IAsyncOperation<super::WalletItem>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportItemAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAppStatusForItem<P0>(&self, item: P0) -> windows_core::Result<WalletItemAppAssociation>
    where
        P0: windows_core::Param<super::WalletItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppStatusForItem)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn LaunchAppForItemAsync<P0>(&self, item: P0) -> windows_core::Result<windows_future::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::WalletItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchAppForItemAsync)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ItemsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveItemsChanged(&self, cookie: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveItemsChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for WalletItemSystemStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletItemSystemStore>();
}
unsafe impl windows_core::Interface for WalletItemSystemStore {
    type Vtable = <IWalletItemSystemStore as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWalletItemSystemStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WalletItemSystemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletItemSystemStore";
}
unsafe impl Send for WalletItemSystemStore {}
unsafe impl Sync for WalletItemSystemStore {}
pub struct WalletManagerSystem;
impl WalletManagerSystem {
    pub fn RequestStoreAsync() -> windows_core::Result<windows_future::IAsyncOperation<WalletItemSystemStore>> {
        Self::IWalletManagerSystemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWalletManagerSystemStatics<R, F: FnOnce(&IWalletManagerSystemStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WalletManagerSystem, IWalletManagerSystemStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WalletManagerSystem {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletManagerSystem";
}
