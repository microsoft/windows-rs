#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Explicit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Explicit,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IExplicit
);
windows_core::imp::required_hierarchy!(Explicit, IClose);
impl windows_core::RuntimeType for Explicit {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IExplicit>();
}
unsafe impl windows_core::Interface for Explicit {
    type Vtable = <IExplicit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExplicit as windows_core::Interface>::IID;
}
impl core::ops::Deref for Explicit {
    type Target = IExplicit;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Explicit {
    const NAME: &'static str = "Test.Explicit";
}
windows_core::imp::define_interface!(IClose, IClose_Vtbl, 0x6640a4a8_5f60_5b12_a007_5df289b65a90);
impl windows_core::RuntimeType for IClose {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IClose, windows_core::IUnknown, windows_core::IInspectable);
impl IClose {
    pub fn Close(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
impl windows_core::RuntimeName for IClose {
    const NAME: &'static str = "Test.IClose";
}
pub trait IClose_Impl: windows_core::IUnknownImpl {
    fn Close(&self) -> windows_core::Result<()>;
}
impl IClose_Vtbl {
    pub const fn new<Identity: IClose_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IClose_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClose_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClose, OFFSET>(),
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClose as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IClose_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IExplicit,
    IExplicit_Vtbl,
    0x3a3ba593_09db_59a3_bd75_a5183fbfe31e
);
impl windows_core::RuntimeType for IExplicit {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IExplicit,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IExplicit {
    const NAME: &'static str = "Test.IExplicit";
}
pub trait IExplicit_Impl: windows_core::IUnknownImpl {}
impl IExplicit_Vtbl {
    pub const fn new<Identity: IExplicit_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IExplicit, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExplicit as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IExplicit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IGeneric<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IGeneric<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IGeneric<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IGeneric<T> {
    type Vtable = IGeneric_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IGeneric<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({e532daa1-6342-5b0f-ae70-f3974d52d36f}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IGenericBase<T>>
    for IGeneric<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IGeneric<T> {
    pub fn Derived(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Derived)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IGeneric<T> {
    const NAME: &'static str = "Test.IGeneric";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
#[repr(C)]
pub struct IGeneric_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Derived: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IGenericBase<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IGenericBase<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IGenericBase<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IGenericBase<T> {
    type Vtable = IGenericBase_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IGenericBase<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({66c93412-67e7-5d09-9595-53ed77c36338}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IGenericBase<T> {
    const NAME: &'static str = "Test.IGenericBase";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
#[repr(C)]
pub struct IGenericBase_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(
    IProducer,
    IProducer_Vtbl,
    0xf8305d4c_4562_571c_a8b5_b44cb4fe4dd9
);
impl windows_core::RuntimeType for IProducer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IProducer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IProducer {
    pub fn GetClose(&self) -> windows_core::Result<IClose> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClose)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IProducer {
    const NAME: &'static str = "Test.IProducer";
}
pub trait IProducer_Impl: windows_core::IUnknownImpl {
    fn GetClose(&self) -> windows_core::Result<IClose>;
}
impl IProducer_Vtbl {
    pub const fn new<Identity: IProducer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClose<Identity: IProducer_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProducer_Impl::GetClose(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProducer, OFFSET>(),
            GetClose: GetClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProducer as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IProducer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetClose: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStream,
    IStream_Vtbl,
    0x5e60052b_a650_589f_a12b_8116ca318d97
);
impl windows_core::RuntimeType for IStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IStream,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IStream {
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
impl windows_core::RuntimeName for IStream {
    const NAME: &'static str = "Test.IStream";
}
pub trait IStream_Impl: IClose_Impl {
    fn Read(&self) -> windows_core::Result<i32>;
}
impl IStream_Vtbl {
    pub const fn new<Identity: IStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IStream_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStream_Impl::Read(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStream, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStream as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUnrelated,
    IUnrelated_Vtbl,
    0xbcfbf36a_f3ac_52b3_a5c5_6d2b554c04d3
);
impl windows_core::RuntimeType for IUnrelated {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IUnrelated,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IUnrelated {
    pub fn Value(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IUnrelated {
    const NAME: &'static str = "Test.IUnrelated";
}
pub trait IUnrelated_Impl: windows_core::IUnknownImpl {
    fn Value(&self) -> windows_core::Result<i32>;
}
impl IUnrelated_Vtbl {
    pub const fn new<Identity: IUnrelated_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: IUnrelated_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUnrelated_Impl::Value(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUnrelated, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnrelated as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IUnrelated_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWhole, IWhole_Vtbl, 0xa49fce2e_2175_5a27_9117_5eff983bb763);
impl windows_core::RuntimeType for IWhole {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IWhole, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWhole, IWholeBase, IWholeMiddle);
impl IWhole {
    pub fn Derived(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Derived)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
impl windows_core::RuntimeName for IWhole {
    const NAME: &'static str = "Test.IWhole";
}
#[repr(C)]
pub struct IWhole_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Derived: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWholeBase,
    IWholeBase_Vtbl,
    0x06b44107_967f_543c_a642_b55c6856de98
);
impl windows_core::RuntimeType for IWholeBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IWholeBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IWholeBase {
    const NAME: &'static str = "Test.IWholeBase";
}
#[repr(C)]
pub struct IWholeBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IWholeMiddle,
    IWholeMiddle_Vtbl,
    0x1953dc42_9bf3_5505_9bce_2bc9fdd39e58
);
impl windows_core::RuntimeType for IWholeMiddle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IWholeMiddle,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IWholeMiddle {
    const NAME: &'static str = "Test.IWholeMiddle";
}
#[repr(C)]
pub struct IWholeMiddle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Producer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Producer,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IProducer
);
impl windows_core::RuntimeType for Producer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IProducer>();
}
unsafe impl windows_core::Interface for Producer {
    type Vtable = <IProducer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProducer as windows_core::Interface>::IID;
}
impl core::ops::Deref for Producer {
    type Target = IProducer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Producer {
    const NAME: &'static str = "Test.Producer";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Stream,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IStream
);
impl windows_core::RuntimeType for Stream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IStream>();
}
unsafe impl windows_core::Interface for Stream {
    type Vtable = <IStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStream as windows_core::Interface>::IID;
}
impl core::ops::Deref for Stream {
    type Target = IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Stream {
    const NAME: &'static str = "Test.Stream";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unrelated(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Unrelated,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IUnrelated
);
impl windows_core::RuntimeType for Unrelated {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUnrelated>();
}
unsafe impl windows_core::Interface for Unrelated {
    type Vtable = <IUnrelated as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUnrelated as windows_core::Interface>::IID;
}
impl core::ops::Deref for Unrelated {
    type Target = IUnrelated;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Unrelated {
    const NAME: &'static str = "Test.Unrelated";
}
