#[inline]
pub unsafe fn Nullable() -> windows_core::Result<Option<Tracked>> {
    windows_core::link!("test.dll" "system" fn Nullable(value : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        Nullable(&mut result__).map(|| {
            if result__.is_null() {
                None
            } else {
                Some(windows_core::Interface::from_raw(result__))
            }
        })
    }
}
#[inline]
pub unsafe fn NullableVoid() -> Option<Tracked> {
    windows_core::link!("test.dll" "system" fn NullableVoid(value : *mut *mut core::ffi::c_void));
    unsafe {
        let mut result__ = core::mem::zeroed();
        NullableVoid(&mut result__);
        if result__.is_null() {
            None
        } else {
            Some(windows_core::Interface::from_raw(result__))
        }
    }
}
#[inline]
pub unsafe fn Query<T>() -> windows_core::Result<Option<T>>
where
    T: windows_core::Interface,
{
    windows_core::link!("test.dll" "system" fn Query(iid : *const windows_core::GUID, value : *mut RawObject) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        Query(&T::IID, &mut result__).map(|| {
            if result__.is_null() {
                None
            } else {
                Some(windows_core::Interface::from_raw(result__))
            }
        })
    }
}
pub type ObjectAlias = Tracked;
pub type OutputSlot = *mut RawObject;
pub type RawObject = *mut core::ffi::c_void;
windows_core::imp::define_interface!(
    Results,
    Results_Vtbl,
    0x10000000_0000_0000_0000_000000000002
);
windows_core::imp::interface_hierarchy!(Results, windows_core::IUnknown);
impl Results {
    pub unsafe fn Nullable(&self) -> windows_core::Result<Option<ObjectAlias>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Nullable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                if result__.is_null() {
                    None
                } else {
                    Some(windows_core::Interface::from_raw(result__))
                }
            })
        }
    }
    pub unsafe fn Reserved(
        &self,
        reserved: Option<usize>,
    ) -> windows_core::Result<Option<Tracked>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reserved)(
                windows_core::Interface::as_raw(self),
                reserved.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .map(|| {
                if result__.is_null() {
                    None
                } else {
                    Some(windows_core::Interface::from_raw(result__))
                }
            })
        }
    }
    pub unsafe fn Required(&self) -> windows_core::Result<Tracked> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Required)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn UnspecifiedFailure(&self) -> windows_core::Result<Tracked> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnspecifiedFailure)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn NullableVoid(&self) -> Option<Tracked> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NullableVoid)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            if result__.is_null() {
                None
            } else {
                Some(windows_core::Interface::from_raw(result__))
            }
        }
    }
    pub unsafe fn Query<T>(&self) -> windows_core::Result<Option<T>>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).Query)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                &mut result__,
            )
            .map(|| {
                if result__.is_null() {
                    None
                } else {
                    Some(windows_core::Interface::from_raw(result__))
                }
            })
        }
    }
    pub unsafe fn OptionalStorage<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OptionalStorage)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                result__ as *mut _ as *mut _,
            )
            .ok()
        }
    }
    pub unsafe fn InOutQuery(
        &self,
        iid: *const windows_core::GUID,
        value: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).InOutQuery)(
                windows_core::Interface::as_raw(self),
                iid,
                value as _,
            )
        }
    }
    pub unsafe fn Data(&self) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn ReadThenFail(
        &self,
        input: *const *mut core::ffi::c_void,
    ) -> windows_core::Result<Tracked> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadThenFail)(
                windows_core::Interface::as_raw(self),
                input,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn TypedefSlot(&self) -> windows_core::Result<Tracked> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TypedefSlot)(
                windows_core::Interface::as_raw(self),
                core::ptr::from_mut::<*mut core::ffi::c_void>(&mut result__).cast(),
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct Results_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Nullable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Reserved: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        usize,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Required: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub UnspecifiedFailure: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NullableVoid:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub Query: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        OutputSlot,
    ) -> windows_core::HRESULT,
    pub OptionalStorage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InOutQuery: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Data:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
    pub ReadThenFail: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TypedefSlot:
        unsafe extern "system" fn(*mut core::ffi::c_void, TrackedSlot) -> windows_core::HRESULT,
}
pub trait Results_Impl: windows_core::IUnknownImpl {
    fn Nullable(&self) -> windows_core::Result<Option<ObjectAlias>>;
    fn Reserved(&self, reserved: usize) -> windows_core::Result<Option<Tracked>>;
    fn Required(&self) -> windows_core::Result<Tracked>;
    fn UnspecifiedFailure(&self) -> windows_core::Result<Tracked>;
    fn NullableVoid(&self, value: windows_core::OutRef<Tracked>);
    fn Query(&self, iid: *const windows_core::GUID, value: OutputSlot) -> windows_core::Result<()>;
    fn OptionalStorage(
        &self,
        iid: *const windows_core::GUID,
        value: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn InOutQuery(
        &self,
        iid: *const windows_core::GUID,
        value: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn Data(&self) -> windows_core::Result<*mut u8>;
    fn ReadThenFail(&self, input: *const *mut core::ffi::c_void) -> windows_core::Result<Tracked>;
    fn TypedefSlot(&self) -> windows_core::Result<Tracked>;
}
impl Results_Vtbl {
    pub const fn new<Identity: Results_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Nullable<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Results_Impl::Nullable(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        value.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Reserved<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            reserved: usize,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Results_Impl::Reserved(this, core::mem::transmute_copy(&reserved)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Required<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Results_Impl::Required(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        value.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn UnspecifiedFailure<
            Identity: Results_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Results_Impl::UnspecifiedFailure(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NullableVoid<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Results_Impl::NullableVoid(this, core::mem::transmute_copy(&value));
            }
        }
        unsafe extern "system" fn Query<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            iid: *const windows_core::GUID,
            value: OutputSlot,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Results_Impl::Query(
                    this,
                    core::mem::transmute_copy(&iid),
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn OptionalStorage<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            iid: *const windows_core::GUID,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Results_Impl::OptionalStorage(
                    this,
                    core::mem::transmute_copy(&iid),
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn InOutQuery<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            iid: *const windows_core::GUID,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Results_Impl::InOutQuery(
                    this,
                    core::mem::transmute_copy(&iid),
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn Data<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Results_Impl::Data(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        value.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn ReadThenFail<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            input: *const *mut core::ffi::c_void,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Results_Impl::ReadThenFail(this, core::mem::transmute_copy(&input)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        value.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn TypedefSlot<Identity: Results_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: TrackedSlot,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Results_Impl::TypedefSlot(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Nullable: Nullable::<Identity, OFFSET>,
            Reserved: Reserved::<Identity, OFFSET>,
            Required: Required::<Identity, OFFSET>,
            UnspecifiedFailure: UnspecifiedFailure::<Identity, OFFSET>,
            NullableVoid: NullableVoid::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            OptionalStorage: OptionalStorage::<Identity, OFFSET>,
            InOutQuery: InOutQuery::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            ReadThenFail: ReadThenFail::<Identity, OFFSET>,
            TypedefSlot: TypedefSlot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Results as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for Results {}
windows_core::imp::define_interface!(
    Tracked,
    Tracked_Vtbl,
    0x10000000_0000_0000_0000_000000000001
);
windows_core::imp::interface_hierarchy!(Tracked, windows_core::IUnknown);
#[repr(C)]
pub struct Tracked_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait Tracked_Impl: windows_core::IUnknownImpl {}
impl Tracked_Vtbl {
    pub const fn new<Identity: Tracked_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Tracked as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for Tracked {}
pub type TrackedSlot = *mut Option<Tracked>;
