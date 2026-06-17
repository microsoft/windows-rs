pub mod Test {
    windows_core::imp::define_interface!(
        Handler,
        Handler_Vtbl,
        0x150c069b_c999_5fa7_ac0d_9b1c08de0960
    );
    impl windows_core::RuntimeType for Handler {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    impl Handler {
        pub fn new<
            F: Fn(windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()>
                + Send
                + 'static,
        >(
            invoke: F,
        ) -> Self {
            let com =
                windows_core::imp::DelegateBox::<Handler, F>::new(&HandlerBox::<F>::VTABLE, invoke);
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        }
        pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
        where
            P0: windows_core::Param<windows_core::IInspectable>,
        {
            unsafe {
                (windows_core::Interface::vtable(self).Invoke)(
                    windows_core::Interface::as_raw(self),
                    sender.param().abi(),
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
            sender: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
    }
    struct HandlerBox<
        F: Fn(windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()>
            + Send
            + 'static,
    >(core::marker::PhantomData<(fn() -> F,)>);
    impl<
        F: Fn(windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()>
            + Send
            + 'static,
    > HandlerBox<F>
    {
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
            sender: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = &mut *(this as *mut *mut core::ffi::c_void
                    as *mut windows_core::imp::DelegateBox<Handler, F>);
                (this.invoke)(core::mem::transmute_copy(&sender)).into()
            }
        }
    }
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x4d1da584_31ea_5f8d_8360_8133f5a5fb71);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IFoo");
    }
    windows_core::imp::interface_hierarchy!(
        IFoo,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IFoo {
        pub fn Keep(&self) -> windows_core::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Keep)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeName for IFoo {
        const NAME: &'static str = "Test.IFoo";
    }
    #[repr(C)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        Value: usize,
        SetValue: usize,
        Changed: usize,
        RemoveChanged: usize,
        pub Keep:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    }
}
