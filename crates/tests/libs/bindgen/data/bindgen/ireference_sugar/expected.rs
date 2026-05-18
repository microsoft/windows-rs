pub mod Test {
    windows_core::imp::define_interface!(
        IThing,
        IThing_Vtbl,
        0x1ce9fe8c_117a_5770_bd53_bf306bd9e27c
    );
    impl windows_core::RuntimeType for IThing {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IThing,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IThing {
        pub fn Count(&self) -> windows_core::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Count)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .and_then(|| windows_core::Type::from_abi(result__))
                .and_then(|r__: windows_reference::IReference<i32>| r__.Value())
            }
        }
        pub fn SetCount(&self, value: Option<i32>) -> windows_core::Result<()> {
            let value__ =
                value.map(<windows_reference::IReference<i32> as core::convert::From<_>>::from);
            unsafe {
                (windows_core::Interface::vtable(self).SetCount)(
                    windows_core::Interface::as_raw(self),
                    windows_core::Param::param(value__.as_ref()).abi(),
                )
                .ok()
            }
        }
        pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Name)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .and_then(|| windows_core::Type::from_abi(result__))
                .and_then(|r__: windows_reference::IReference<windows_core::HSTRING>| r__.Value())
            }
        }
        pub fn SetName<P0>(&self, value: Option<P0>) -> windows_core::Result<()>
        where
            P0: core::convert::Into<windows_reference::IReference<windows_core::HSTRING>>,
        {
            let value__ =
                value.map(
                    <P0 as core::convert::Into<
                        windows_reference::IReference<windows_core::HSTRING>,
                    >>::into,
                );
            unsafe {
                (windows_core::Interface::vtable(self).SetName)(
                    windows_core::Interface::as_raw(self),
                    windows_core::Param::param(value__.as_ref()).abi(),
                )
                .ok()
            }
        }
    }
    impl windows_core::RuntimeName for IThing {
        const NAME: &'static str = "Test.IThing";
    }
    pub trait IThing_Impl: windows_core::IUnknownImpl {
        fn Count(&self) -> windows_core::Result<windows_reference::IReference<i32>>;
        fn SetCount(
            &self,
            value: windows_core::Ref<windows_reference::IReference<i32>>,
        ) -> windows_core::Result<()>;
        fn Name(
            &self,
        ) -> windows_core::Result<windows_reference::IReference<windows_core::HSTRING>>;
        fn SetName(
            &self,
            value: windows_core::Ref<windows_reference::IReference<windows_core::HSTRING>>,
        ) -> windows_core::Result<()>;
    }
    impl IThing_Vtbl {
        pub const fn new<Identity: IThing_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn Count<Identity: IThing_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IThing_Impl::Count(this) {
                        Ok(ok__) => {
                            result__.write(core::mem::transmute_copy(&ok__));
                            core::mem::forget(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            unsafe extern "system" fn SetCount<Identity: IThing_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                value: *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    IThing_Impl::SetCount(this, core::mem::transmute_copy(&value)).into()
                }
            }
            unsafe extern "system" fn Name<Identity: IThing_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IThing_Impl::Name(this) {
                        Ok(ok__) => {
                            result__.write(core::mem::transmute_copy(&ok__));
                            core::mem::forget(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            unsafe extern "system" fn SetName<Identity: IThing_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                value: *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    IThing_Impl::SetName(this, core::mem::transmute_copy(&value)).into()
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IThing, OFFSET>(),
                Count: Count::<Identity, OFFSET>,
                SetCount: SetCount::<Identity, OFFSET>,
                Name: Name::<Identity, OFFSET>,
                SetName: SetName::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IThing as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IThing_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Count: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
        pub SetCount: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
        pub Name: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
        pub SetName: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
    }
}
