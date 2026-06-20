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
        let com = windows_core::imp::DelegateBox::<Handler<T>, F>::new(
            &HandlerBox::<T, F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0, args: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args,
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
        sender: windows_core::AbiType<T>,
        args: i32,
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
        sender: windows_core::AbiType<T>,
        args: i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Handler<T>, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), args).into()
        }
    }
}
