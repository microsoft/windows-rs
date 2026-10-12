#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Handler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for Handler<T> {
    type Vtable = Handler_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for Handler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({81da47bc-7402-5e33-aa32-c87d0d4ef213}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> Handler<T> {
    pub fn new<F: Fn(windows_core::Ref<T>, i32) -> windows_core::Result<()> + Send + 'static>(
        invoke: F,
    ) -> Self {
        let com =
            windows_core::imp::DelegateBox::<Self, F>::new(&HandlerBox::<T, F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct Handler_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: windows_core::imp::AbiType<T>,
        value: i32,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
struct HandlerBox<T, F: Fn(windows_core::Ref<T>, i32) -> windows_core::Result<()> + Send + 'static>(
    core::marker::PhantomData<(T, fn() -> F)>,
)
where
    T: windows_core::RuntimeType + 'static;
impl<
    T: windows_core::RuntimeType + 'static,
    F: Fn(windows_core::Ref<T>, i32) -> windows_core::Result<()> + Send + 'static,
> HandlerBox<T, F>
{
    const VTABLE: Handler_Vtbl<T> = Handler_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<Handler<T>, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<Handler<T>, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<Handler<T>, F>::Release,
        },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: windows_core::imp::AbiType<T>,
        value: i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Handler<T>, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), value).into()
        }
    }
}
windows_core::imp::define_interface!(
    ISource,
    ISource_Vtbl,
    0xc1f11ade_373b_528b_9be6_7b1e709e331a
);
impl windows_core::RuntimeType for ISource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.ISource");
}
windows_core::imp::interface_hierarchy!(
    ISource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ISource {
    pub fn Changed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, i32) + Send + 'static,
    {
        let handler = <Handler<Self>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Changed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveChanged,
            ))
        }
    }
}
impl windows_core::RuntimeName for ISource {
    const NAME: &'static str = "Test.ISource";
}
pub trait ISource_Impl: windows_core::IUnknownImpl {
    fn Changed(&self, handler: windows_core::Ref<Handler<ISource>>) -> windows_core::Result<i64>;
    fn RemoveChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl ISource_Vtbl {
    pub const fn new<Identity: ISource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Changed<Identity: ISource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISource_Impl::Changed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveChanged<Identity: ISource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISource_Impl::RemoveChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISource, OFFSET>(),
            Changed: Changed::<Identity, OFFSET>,
            RemoveChanged: RemoveChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISource as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ISource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Changed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
