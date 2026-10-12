windows_core::imp::define_interface!(IItems, IItems_Vtbl, 0x8dfef4bd_8c6e_5d6d_9614_0aca51e6943b);
impl windows_core::RuntimeType for IItems {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IItems");
}
windows_core::imp::interface_hierarchy!(IItems, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IItems {
    const NAME: &'static str = "Test.IItems";
}
pub trait IItems_Impl: windows_core::IUnknownImpl {}
impl IItems_Vtbl {
    pub const fn new<Identity: IItems_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IItems, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IItems as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IItems_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(Item, Item_Vtbl, 0x2d93fccb_fadf_52bd_a462_4bb301d551af);
impl windows_core::RuntimeType for Item {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Item");
}
windows_core::imp::interface_hierarchy!(Item, windows_core::IUnknown, windows_core::IInspectable);
impl Item {
    pub fn Read(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for Item {
    const NAME: &'static str = "Test.Item";
}
pub trait Item_Impl: windows_core::IUnknownImpl {
    fn Read(&self) -> windows_core::Result<i32>;
}
impl Item_Vtbl {
    pub const fn new<Identity: Item_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: Item_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Item_Impl::Read(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, Item, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Item as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct Item_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Items(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Items,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IItems
);
windows_core::imp::required_hierarchy!(Items, windows_collections::IIterable<Item>);
impl Items {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<Item>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<Item>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Items {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IItems>();
}
unsafe impl windows_core::Interface for Items {
    type Vtable = <IItems as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IItems as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Items {
    const NAME: &'static str = "Test.Items";
}
impl IntoIterator for Items {
    type Item = Item;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &Items {
    type Item = Item;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        windows_collections::BufferedIterator::new(self.First().unwrap())
    }
}
