windows_core::imp::define_interface!(
    Handler,
    Handler_Vtbl,
    0x9df939b9_b0e1_58dc_89ec_fc84175d6ef4
);
impl windows_core::RuntimeType for Handler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct Handler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        value: i32,
    ) -> windows_core::HRESULT,
}
struct HandlerBox<F: Fn(i32) + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(i32) + 'static> HandlerBox<F> {
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
        value: i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Handler, F>);
            (this.invoke)(value);
            windows_core::HRESULT(0)
        }
    }
}
windows_core::imp::define_interface!(
    ISource,
    ISource_Vtbl,
    0xcbd42b36_9482_5b72_a9e9_22257c7e3f0e
);
impl windows_core::RuntimeType for ISource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ISource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ISource {
    pub fn Changed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(i32) + 'static,
    {
        let handler: Handler = {
            let com = windows_core::imp::DelegateBox::<Handler, F>::new(
                &HandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Changed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveChanged,
            ))
        }
    }
}
impl windows_core::RuntimeName for ISource {
    const NAME: &'static str = "Test.ISource";
}
pub trait ISource_Impl: windows_core::IUnknownImpl {
    fn Changed(&self, handler: windows_core::Ref<Handler>) -> windows_core::Result<i64>;
    fn RemoveChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl ISource_Vtbl {
    pub const fn new<Identity: ISource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Changed<Identity: ISource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISource_Impl::Changed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveChanged<Identity: ISource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISource_Impl::RemoveChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISource, OFFSET>(),
            Changed: Changed::<Identity, OFFSET>,
            RemoveChanged: RemoveChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISource as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ISource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Changed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
