windows_core::imp::define_interface!(
    IReader,
    IReader_Vtbl,
    0x148c0a0d_2b39_566f_bf14_abfefea7e386
);
impl windows_core::RuntimeType for IReader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IReader");
}
windows_core::imp::interface_hierarchy!(
    IReader,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IReader {
    pub fn Read(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadWithOptions(&self, options: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadWithOptions)(
                windows_core::Interface::as_raw(self),
                options,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Read2(&self, options: u32, count: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read2)(
                windows_core::Interface::as_raw(self),
                options,
                count,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IReader {
    const NAME: &'static str = "Test.IReader";
}
pub trait IReader_Impl: windows_core::IUnknownImpl {
    fn Read(&self) -> windows_core::Result<u32>;
    fn ReadWithOptions(&self, options: u32) -> windows_core::Result<u32>;
    fn Read2(&self, options: u32, count: u32) -> windows_core::Result<u32>;
}
impl IReader_Vtbl {
    pub const fn new<Identity: IReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IReader_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReader_Impl::Read(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadWithOptions<Identity: IReader_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            options: u32,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReader_Impl::ReadWithOptions(this, options) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Read2<Identity: IReader_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            options: u32,
            count: u32,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReader_Impl::Read2(this, options, count) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IReader, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
            ReadWithOptions: ReadWithOptions::<Identity, OFFSET>,
            Read2: Read2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReader as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadWithOptions:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Read2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
}
