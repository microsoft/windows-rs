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
            F: Fn(windows_core::Ref<windows_core::IInspectable>) -> windows_result::Result<()>
                + Send
                + 'static,
        >(
            invoke: F,
        ) -> Self {
            let com =
                windows_core::imp::DelegateBox::<Handler, F>::new(&HandlerBox::<F>::VTABLE, invoke);
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        }
        pub fn Invoke<P0>(&self, sender: P0) -> windows_result::Result<()>
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
    #[doc(hidden)]
    pub struct Handler_Vtbl {
        base__: windows_core::IUnknown_Vtbl,
        Invoke: unsafe extern "system" fn(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
    }
    struct HandlerBox<
        F: Fn(windows_core::Ref<windows_core::IInspectable>) -> windows_result::Result<()>
            + Send
            + 'static,
    >(core::marker::PhantomData<(fn() -> F,)>);
    impl<
            F: Fn(windows_core::Ref<windows_core::IInspectable>) -> windows_result::Result<()>
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
    }
    windows_core::imp::interface_hierarchy!(
        IFoo,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IFoo {
        pub fn Keep(&self) -> windows_result::Result<i32> {
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
    pub trait IFoo_Impl: windows_core::IUnknownImpl {
        fn Keep(&self) -> windows_result::Result<i32>;
    }
    impl IFoo_Vtbl {
        pub const fn new<Identity: IFoo_Impl, const OFFSET: isize>() -> Self {
            #[allow(unused_variables)]
            unsafe extern "system" fn Value<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                windows_core::HRESULT(0x80004001_u32 as i32)
            }
            #[allow(unused_variables)]
            unsafe extern "system" fn SetValue<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                value: i32,
            ) -> windows_core::HRESULT {
                windows_core::HRESULT(0x80004001_u32 as i32)
            }
            #[allow(unused_variables)]
            unsafe extern "system" fn Changed<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                handler: *mut core::ffi::c_void,
                result__: *mut i64,
            ) -> windows_core::HRESULT {
                windows_core::HRESULT(0x80004001_u32 as i32)
            }
            #[allow(unused_variables)]
            unsafe extern "system" fn RemoveChanged<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                token: i64,
            ) -> windows_core::HRESULT {
                windows_core::HRESULT(0x80004001_u32 as i32)
            }
            unsafe extern "system" fn Keep<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IFoo_Impl::Keep(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IFoo, OFFSET>(),
                Value: Value::<Identity, OFFSET>,
                SetValue: SetValue::<Identity, OFFSET>,
                Changed: Changed::<Identity, OFFSET>,
                RemoveChanged: RemoveChanged::<Identity, OFFSET>,
                Keep: Keep::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IFoo as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        Value:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
        SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_result::HRESULT,
        Changed: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            *mut i64,
        ) -> windows_result::HRESULT,
        RemoveChanged:
            unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_result::HRESULT,
        pub Keep:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
