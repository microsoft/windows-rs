#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IAsyncAction,
    IAsyncAction_Vtbl,
    0x5a648006_843a_4da9_865b_9d26e5dfad7b
);
impl windows_core::RuntimeType for IAsyncAction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IAsyncAction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IAsyncAction, windows::Foundation::IAsyncInfo);
impl IAsyncAction {
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::AsyncActionCompletedHandler>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn Completed(
        &self,
    ) -> windows_core::Result<windows::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<windows::Foundation::AsyncStatus> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
unsafe impl Send for IAsyncAction {}
unsafe impl Sync for IAsyncAction {}
impl windows_core::RuntimeName for IAsyncAction {
    const NAME: &'static str = "Windows.Foundation.IAsyncAction";
}
pub trait IAsyncAction_Impl: windows::Foundation::IAsyncInfo_Impl {
    fn SetCompleted(
        &self,
        handler: windows_core::Ref<'_, windows::Foundation::AsyncActionCompletedHandler>,
    ) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<windows::Foundation::AsyncActionCompletedHandler>;
    fn GetResults(&self) -> windows_core::Result<()>;
}
impl IAsyncAction_Vtbl {
    pub const fn new<Identity: IAsyncAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCompleted<Identity: IAsyncAction_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncAction_Impl::SetCompleted(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Completed<Identity: IAsyncAction_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncAction_Impl::Completed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResults<Identity: IAsyncAction_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncAction_Impl::GetResults(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncAction, OFFSET>(),
            SetCompleted: SetCompleted::<Identity, OFFSET>,
            Completed: Completed::<Identity, OFFSET>,
            GetResults: GetResults::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncAction as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IAsyncAction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
