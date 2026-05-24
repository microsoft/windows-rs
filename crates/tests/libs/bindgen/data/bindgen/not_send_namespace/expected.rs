pub mod Test {
    windows_core::imp::define_interface!(
        IWidget,
        IWidget_Vtbl,
        0x492fc421_ed8a_5e68_ab53_0010fb7914aa
    );
    impl windows_core::RuntimeType for IWidget {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IWidget,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IWidget {
        pub fn Name(&self) -> windows_core::Result<String> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Name)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| {
                    let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                    hstring.to_string_lossy()
                })
            }
        }
        pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
        where
            F: Fn(windows_core::Ref<IWidget>) + 'static,
        {
            let handler = <TappedHandler>::new(handler);
            unsafe {
                let mut result__ = core::mem::zeroed();
                let token__ = (windows_core::Interface::vtable(self).Tapped)(
                    windows_core::Interface::as_raw(self),
                    windows_core::Interface::as_raw(&handler),
                    &mut result__,
                )
                .map(|| result__)?;
                Ok(windows_core::EventRevoker::new(
                    self.clone(),
                    token__,
                    windows_core::Interface::vtable(self).RemoveTapped,
                ))
            }
        }
    }
    impl windows_core::RuntimeName for IWidget {
        const NAME: &'static str = "Test.IWidget";
    }
    pub trait IWidget_Impl: windows_core::IUnknownImpl {
        fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
        fn Tapped(&self, handler: windows_core::Ref<TappedHandler>) -> windows_core::Result<i64>;
        fn RemoveTapped(&self, token: i64) -> windows_core::Result<()>;
    }
    impl IWidget_Vtbl {
        pub const fn new<Identity: IWidget_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn Name<Identity: IWidget_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IWidget_Impl::Name(this) {
                        Ok(ok__) => {
                            result__.write(core::mem::transmute_copy(&ok__));
                            core::mem::forget(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            unsafe extern "system" fn Tapped<Identity: IWidget_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                handler: *mut core::ffi::c_void,
                result__: *mut i64,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IWidget_Impl::Tapped(this, core::mem::transmute_copy(&handler)) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            unsafe extern "system" fn RemoveTapped<Identity: IWidget_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                token: i64,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    IWidget_Impl::RemoveTapped(this, token).into()
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidget, OFFSET>(),
                Name: Name::<Identity, OFFSET>,
                Tapped: Tapped::<Identity, OFFSET>,
                RemoveTapped: RemoveTapped::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IWidget as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IWidget_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Name: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
        pub Tapped: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            *mut i64,
        ) -> windows_core::HRESULT,
        pub RemoveTapped:
            unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    }
    windows_core::imp::define_interface!(
        TappedHandler,
        TappedHandler_Vtbl,
        0xba1e1255_54dc_5060_924f_72b4e438bb9b
    );
    impl windows_core::RuntimeType for TappedHandler {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    impl TappedHandler {
        pub fn new<F: Fn(windows_core::Ref<IWidget>) + 'static>(invoke: F) -> Self {
            let com = windows_core::imp::DelegateBox::<TappedHandler, F>::new(
                &TappedHandlerBox::<F>::VTABLE,
                invoke,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        }
        pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
        where
            P0: windows_core::Param<IWidget>,
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
    pub struct TappedHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl,
        Invoke: unsafe extern "system" fn(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
    }
    struct TappedHandlerBox<F: Fn(windows_core::Ref<IWidget>) + 'static>(
        core::marker::PhantomData<(fn() -> F,)>,
    );
    impl<F: Fn(windows_core::Ref<IWidget>) + 'static> TappedHandlerBox<F> {
        const VTABLE: TappedHandler_Vtbl = TappedHandler_Vtbl {
            base__: windows_core::IUnknown_Vtbl {
                QueryInterface: windows_core::imp::DelegateBox::<TappedHandler, F>::QueryInterface,
                AddRef: windows_core::imp::DelegateBox::<TappedHandler, F>::AddRef,
                Release: windows_core::imp::DelegateBox::<TappedHandler, F>::Release,
            },
            Invoke: Self::Invoke,
        };
        unsafe extern "system" fn Invoke(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = &mut *(this as *mut *mut core::ffi::c_void
                    as *mut windows_core::imp::DelegateBox<TappedHandler, F>);
                (this.invoke)(core::mem::transmute_copy(&sender));
                windows_core::HRESULT(0)
            }
        }
    }
}
