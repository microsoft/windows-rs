windows_core::imp::define_interface!(
    Delegate,
    Delegate_Vtbl,
    0x153ec0b2_76da_51fe_92a4_066f19c17c4c
);
impl windows_core::RuntimeType for Delegate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl Delegate {
    pub fn new<F: Fn(i32, i32) + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&DelegateBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct Delegate_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: i32,
        args: i32,
    ) -> windows_core::HRESULT,
}
struct DelegateBox<F: Fn(i32, i32) + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(i32, i32) + 'static> DelegateBox<F> {
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
        args: i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Delegate, F>);
            (this.invoke)(sender, args);
            windows_core::HRESULT(0)
        }
    }
}
