windows_core::imp::define_interface!(
    Delegate,
    Delegate_Vtbl,
    0x7b568e1d_7e8a_5208_9852_d3e3f1299a20
);
impl windows_core::RuntimeType for Delegate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl Delegate {
    pub fn new<F: Fn(i32) + 'static>(invoke: F) -> Self {
        let com =
            windows_core::imp::DelegateBox::<Delegate, F>::new(&DelegateBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct Delegate_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: i32,
    ) -> windows_core::HRESULT,
}
struct DelegateBox<F: Fn(i32) + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(i32) + 'static> DelegateBox<F> {
    const VTABLE: Delegate_Vtbl = Delegate_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<Delegate, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<Delegate, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<Delegate, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Delegate, F>);
            (this.invoke)(sender);
            windows_core::HRESULT(0)
        }
    }
}
windows_core::imp::define_interface!(
    Interface,
    Interface_Vtbl,
    0xca5a6b51_8bab_5a2e_af33_e182fa13377f
);
impl windows_core::RuntimeType for Interface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    Interface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for Interface {
    const NAME: &'static str = "Test.Interface";
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
