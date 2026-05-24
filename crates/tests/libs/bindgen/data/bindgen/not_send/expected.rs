pub mod Test {
    windows_core::imp::define_interface!(
        Handler,
        Handler_Vtbl,
        0xd8ccb2eb_f4d4_55c6_977f_a23ed0a7d401
    );
    impl windows_core::RuntimeType for Handler {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    impl Handler {
        pub fn new<F: Fn(windows_core::Ref<IFoo>, i32) + 'static>(invoke: F) -> Self {
            let com =
                windows_core::imp::DelegateBox::<Handler, F>::new(&HandlerBox::<F>::VTABLE, invoke);
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        }
        pub fn Invoke<P0>(&self, sender: P0, args: i32) -> windows_core::Result<()>
        where
            P0: windows_core::Param<IFoo>,
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
    #[doc(hidden)]
    pub struct Handler_Vtbl {
        base__: windows_core::IUnknown_Vtbl,
        Invoke: unsafe extern "system" fn(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: i32,
        ) -> windows_core::HRESULT,
    }
    struct HandlerBox<F: Fn(windows_core::Ref<IFoo>, i32) + 'static>(
        core::marker::PhantomData<(fn() -> F,)>,
    );
    impl<F: Fn(windows_core::Ref<IFoo>, i32) + 'static> HandlerBox<F> {
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
            args: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = &mut *(this as *mut *mut core::ffi::c_void
                    as *mut windows_core::imp::DelegateBox<Handler, F>);
                (this.invoke)(core::mem::transmute_copy(&sender), args);
                windows_core::HRESULT(0)
            }
        }
    }
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x528bf755_537d_5766_a441_3d440809af4c);
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
        pub fn Bar(&self) -> windows_core::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Bar)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        pub fn Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
        where
            F: Fn(windows_core::Ref<IFoo>, i32) + 'static,
        {
            let handler = <Handler>::new(handler);
            unsafe {
                let mut result__ = core::mem::zeroed();
                let token__ = (windows_core::Interface::vtable(self).Click)(
                    windows_core::Interface::as_raw(self),
                    windows_core::Interface::as_raw(&handler),
                    &mut result__,
                )
                .map(|| result__)?;
                Ok(windows_core::EventRevoker::new(
                    self.clone(),
                    token__,
                    windows_core::Interface::vtable(self).RemoveClick,
                ))
            }
        }
    }
    impl windows_core::RuntimeName for IFoo {
        const NAME: &'static str = "Test.IFoo";
    }
    pub trait IFoo_Impl: windows_core::IUnknownImpl {
        fn Bar(&self) -> windows_core::Result<i32>;
        fn Click(&self, handler: windows_core::Ref<Handler>) -> windows_core::Result<i64>;
        fn RemoveClick(&self, token: i64) -> windows_core::Result<()>;
    }
    impl IFoo_Vtbl {
        pub const fn new<Identity: IFoo_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn Bar<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IFoo_Impl::Bar(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            unsafe extern "system" fn Click<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                handler: *mut core::ffi::c_void,
                result__: *mut i64,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IFoo_Impl::Click(this, core::mem::transmute_copy(&handler)) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            unsafe extern "system" fn RemoveClick<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                token: i64,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    IFoo_Impl::RemoveClick(this, token).into()
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IFoo, OFFSET>(),
                Bar: Bar::<Identity, OFFSET>,
                Click: Click::<Identity, OFFSET>,
                RemoveClick: RemoveClick::<Identity, OFFSET>,
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
        pub Bar:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
        pub Click: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            *mut i64,
        ) -> windows_core::HRESULT,
        pub RemoveClick:
            unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    }
}
