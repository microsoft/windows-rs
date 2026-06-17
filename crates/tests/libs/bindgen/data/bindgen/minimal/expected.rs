pub mod Test {
    windows_core::link!("test.dll" "system" fn GetValue() -> u32);
    windows_core::link!("test.dll" "system" fn SetValue(value : u32));
    pub type Color = i32;
    pub type HANDLE = *mut core::ffi::c_void;
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x29b2ee6f_e8bf_5d03_8e01_81e8ad109076);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    impl IFoo {
        pub(crate) fn Direct(&self) -> windows_core::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Direct)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        pub(crate) fn get_Name(&self) -> windows_core::Result<String> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).get_Name)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| {
                    let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                    hstring.to_string_lossy()
                })
            }
        }
        pub(crate) fn put_Name(&self, value: &str) -> windows_core::Result<()> {
            unsafe {
                (windows_core::Interface::vtable(self).put_Name)(
                    windows_core::Interface::as_raw(self),
                    core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
                )
                .ok()
            }
        }
    }
    #[repr(C)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Direct:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
        pub get_Name: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
        pub put_Name: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
    }
    windows_core::imp::define_interface!(IFoo2, IFoo2_Vtbl, 0xd5639aca_50ae_5b48_9f64_938ce24b8683);
    impl windows_core::RuntimeType for IFoo2 {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IFoo2,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IFoo2 {
        pub(crate) fn Bar(&self) -> windows_core::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Bar)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeName for IFoo2 {
        const NAME: &'static str = "Test.IFoo2";
    }
    pub trait IFoo2_Impl: windows_core::IUnknownImpl {
        fn Bar(&self) -> windows_core::Result<i32>;
    }
    impl IFoo2_Vtbl {
        pub const fn new<Identity: IFoo2_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn Bar<Identity: IFoo2_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IFoo2_Impl::Bar(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IFoo2, OFFSET>(),
                Bar: Bar::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IFoo2 as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    pub struct IFoo2_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Bar:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    }
    windows_core::imp::define_interface!(
        IFooStatics,
        IFooStatics_Vtbl,
        0xf1c38ac4_8e1a_5dc8_a1e0_b3b3f956d845
    );
    impl windows_core::RuntimeType for IFooStatics {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    #[repr(C)]
    pub struct IFooStatics_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Stat:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    }
    pub const INVALID_HANDLE_VALUE: HANDLE = -1 as _;
}
