#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IIterable<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IIterable<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IIterable<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterable<T> {
    type Vtable = IIterable_Vtbl;
    const IID: windows_core::GUID =
        windows_core::GUID::from_u128(0xfaa585ea_6214_4217_afda_7f46de5869b3);
}
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IIterable<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> IIterable<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIterable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IVector<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IVector<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IVector<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>>
    for IVector<T>
{
    const QUERY: bool = true;
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVector<T> {
    type Vtable = IVector_Vtbl;
    const IID: windows_core::GUID =
        windows_core::GUID::from_u128(0x913337e9_11a1_4345_a3a2_4e7f956e222d);
}
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IVector<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> IVector<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(
                windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IndexOf(&self, value: &T, index: &mut u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAt(&self, index: u32, value: &T) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetAt)(
                windows_core::Interface::as_raw(this),
                index,
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt(&self, index: u32, value: &T) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).InsertAt)(
                windows_core::Interface::as_raw(this),
                index,
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAt)(
                windows_core::Interface::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append(&self, value: &T) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Append)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn GetMany(
        &self,
        startIndex: u32,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(
                windows_core::Interface::as_raw(this),
                startIndex,
                items.len().try_into().unwrap(),
                core::mem::transmute_copy(&items),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReplaceAll(
        &self,
        items: &[<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).ReplaceAll)(
                windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVector<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
