windows_core::imp::define_winrt_interface!(
    IItems,
    IItems_Vtbl,
    0x9e338b3a_5eb9_5b77_a540_541e53c3dcf5,
    b"Windows.Test.IItems"
);
windows_core::imp::interface_hierarchy!(IItems, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IItems {
    const NAME: &'static str = "Windows.Test.IItems";
}
pub trait IItems_Impl: windows_core::IUnknownImpl {}
impl IItems_Vtbl {
    pub const fn new<Identity: IItems_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
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
windows_core::imp::define_winrt_interface!(
    Item,
    Item_Vtbl,
    0x767a02ee_2f01_5886_a631_40ddb9957f8d,
    b"Windows.Other.Item"
);
windows_core::imp::interface_hierarchy!(Item, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for Item {
    const NAME: &'static str = "Windows.Other.Item";
}
#[repr(C)]
pub struct Item_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
            .and_then(|| windows_core::imp::from_abi(result__))
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
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for Items {
    const NAME: &'static str = "Windows.Test.Items";
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
