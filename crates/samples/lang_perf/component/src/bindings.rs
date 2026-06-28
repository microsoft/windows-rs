#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Class, windows_core::IUnknown, windows_core::IInspectable);
impl Class {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Class, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Int32Property(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Int32Property)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInt32Property(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInt32Property)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn StringProperty(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StringProperty)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetStringProperty(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStringProperty)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ObjectProperty(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectProperty)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetObjectProperty<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetObjectProperty)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Next(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Lang(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Lang)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Event<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, i32) + Send + 'static,
    {
        let handler = <Handler>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Event)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveEvent,
            ))
        }
    }
    pub fn Raise(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Raise)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
impl windows_core::RuntimeType for Class {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IClass>();
}
unsafe impl windows_core::Interface for Class {
    type Vtable = <IClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Class {
    const NAME: &'static str = "LangPerf.Class";
}
unsafe impl Send for Class {}
unsafe impl Sync for Class {}
windows_core::imp::define_interface!(
    Handler,
    Handler_Vtbl,
    0x04c9aba3_8fc3_580e_8c33_da8f9c18f656
);
impl windows_core::RuntimeType for Handler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl Handler {
    pub fn new<
        F: Fn(windows_core::Ref<windows_core::IInspectable>, i32) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&HandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0, args: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
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
pub struct Handler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        args: i32,
    ) -> windows_core::HRESULT,
}
struct HandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, i32) -> windows_core::Result<()>
        + Send
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, i32) -> windows_core::Result<()>
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
        args: i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Handler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), args).into()
        }
    }
}
windows_core::imp::define_interface!(IClass, IClass_Vtbl, 0x02dfb266_f3a3_57a8_98c1_eb15d31b4a4d);
impl windows_core::RuntimeType for IClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"LangPerf.IClass");
}
impl windows_core::RuntimeName for IClass {
    const NAME: &'static str = "LangPerf.IClass";
}
pub trait IClass_Impl: windows_core::IUnknownImpl {
    fn Int32Property(&self) -> windows_core::Result<i32>;
    fn SetInt32Property(&self, value: i32) -> windows_core::Result<()>;
    fn StringProperty(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetStringProperty(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ObjectProperty(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn SetObjectProperty(
        &self,
        value: windows_core::Ref<windows_core::IInspectable>,
    ) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<i32>;
    fn Lang(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Event(&self, handler: windows_core::Ref<Handler>) -> windows_core::Result<i64>;
    fn RemoveEvent(&self, token: i64) -> windows_core::Result<()>;
    fn Raise(&self) -> windows_core::Result<()>;
}
impl IClass_Vtbl {
    pub const fn new<Identity: IClass_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Int32Property<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::Int32Property(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInt32Property<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClass_Impl::SetInt32Property(this, value).into()
            }
        }
        unsafe extern "system" fn StringProperty<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::StringProperty(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStringProperty<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClass_Impl::SetStringProperty(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ObjectProperty<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::ObjectProperty(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetObjectProperty<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClass_Impl::SetObjectProperty(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::Next(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Lang<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::Lang(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Event<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::Event(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveEvent<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClass_Impl::RemoveEvent(this, token).into()
            }
        }
        unsafe extern "system" fn Raise<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClass_Impl::Raise(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClass, OFFSET>(),
            Int32Property: Int32Property::<Identity, OFFSET>,
            SetInt32Property: SetInt32Property::<Identity, OFFSET>,
            StringProperty: StringProperty::<Identity, OFFSET>,
            SetStringProperty: SetStringProperty::<Identity, OFFSET>,
            ObjectProperty: ObjectProperty::<Identity, OFFSET>,
            SetObjectProperty: SetObjectProperty::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Lang: Lang::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
            RemoveEvent: RemoveEvent::<Identity, OFFSET>,
            Raise: Raise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClass as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Int32Property:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInt32Property:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StringProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetStringProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ObjectProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetObjectProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Lang: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Event: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveEvent:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Raise: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INonDefault,
    INonDefault_Vtbl,
    0x1471cc2c_0216_5a12_90ad_6bc57677b7f5
);
impl windows_core::RuntimeType for INonDefault {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"LangPerf.INonDefault");
}
windows_core::imp::interface_hierarchy!(
    INonDefault,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl INonDefault {
    pub fn Value(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for INonDefault {
    const NAME: &'static str = "LangPerf.INonDefault";
}
pub trait INonDefault_Impl: windows_core::IUnknownImpl {
    fn Value(&self) -> windows_core::Result<i32>;
}
impl INonDefault_Vtbl {
    pub const fn new<Identity: INonDefault_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: INonDefault_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INonDefault_Impl::Value(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INonDefault, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INonDefault as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct INonDefault_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
