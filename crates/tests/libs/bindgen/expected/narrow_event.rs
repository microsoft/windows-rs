windows_core::imp::define_interface!(
    Handler,
    Handler_Vtbl,
    0xeb295785_7560_583c_9290_2858ae1013bb
);
impl windows_core::RuntimeType for Handler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl Handler {
    pub fn new<F: Fn(&Value) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&HandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke(&self, value: Value) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct Handler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        value: Value,
    ) -> windows_core::HRESULT,
}
struct HandlerBox<F: Fn(&Value) -> windows_core::Result<()> + Send + 'static>(
    core::marker::PhantomData<(fn() -> F,)>,
);
impl<F: Fn(&Value) -> windows_core::Result<()> + Send + 'static> HandlerBox<F> {
    const VTABLE: Handler_Vtbl = Handler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<Handler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<Handler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<Handler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        value: Value,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Handler, F>);
            (this.invoke)(core::mem::transmute(&value)).into()
        }
    }
}
windows_core::imp::define_interface!(
    ISource,
    ISource_Vtbl,
    0xfa047a8f_370f_5f8b_b61f_3d4bfca8faa8
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
        F: Fn(&Value) + Send + 'static,
    {
        let handler = <Handler>::new(move |a0| {
            handler(a0);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Value {
    pub Number: i32,
}
impl windows_core::imp::TypeKind for Value {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Value {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Value;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Value");
}
