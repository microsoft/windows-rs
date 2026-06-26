windows_core::imp::define_interface!(
    Delegate,
    Delegate_Vtbl,
    0x7b568e1d_7e8a_5208_9852_d3e3f1299a20
);
impl windows_core::RuntimeType for Delegate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl Delegate {
    pub fn new<F: Fn(i32) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&DelegateBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke(&self, sender: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct Delegate_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: i32,
    ) -> windows_core::HRESULT,
}
struct DelegateBox<F: Fn(i32) -> windows_core::Result<()> + Send + 'static>(
    core::marker::PhantomData<(fn() -> F,)>,
);
impl<F: Fn(i32) -> windows_core::Result<()> + Send + 'static> DelegateBox<F> {
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
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Delegate, F>);
            (this.invoke)(sender).into()
        }
    }
}
windows_core::imp::define_interface!(
    Interface,
    Interface_Vtbl,
    0xca5a6b51_8bab_5a2e_af33_e182fa13377f
);
impl windows_core::RuntimeType for Interface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Interface");
}
windows_core::imp::interface_hierarchy!(
    Interface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Interface {
    pub fn Method(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Method)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Changed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(i32) + Send + 'static,
    {
        let handler = <Delegate>::new(move |a0| {
            handler(a0);
            Ok(())
        });
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
impl windows_core::RuntimeName for Interface {
    const NAME: &'static str = "Test.Interface";
}
pub trait Interface_Impl: windows_core::IUnknownImpl {
    fn Method(&self) -> windows_core::Result<i32>;
    fn Changed(&self, handler: windows_core::Ref<Delegate>) -> windows_core::Result<i64>;
    fn RemoveChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl Interface_Vtbl {
    pub const fn new<Identity: Interface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Method<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Interface_Impl::Method(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Changed<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Interface_Impl::Changed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveChanged<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Interface_Impl::RemoveChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, Interface, OFFSET>(),
            Method: Method::<Identity, OFFSET>,
            Changed: Changed::<Identity, OFFSET>,
            RemoveChanged: RemoveChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Interface as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
