#[inline]
pub unsafe fn Output(count: u32, values: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("test.dll" "system" fn Output(count : u32, values : *mut u32) -> windows_core::HRESULT);
    unsafe { Output(count, values as _) }
}
windows_core::imp::define_interface!(Buffers, Buffers_Vtbl);
impl Buffers {
    pub unsafe fn Input(&self, values: &[u32]) {
        unsafe {
            (windows_core::Interface::vtable(self).Input)(
                windows_core::Interface::as_raw(self),
                values.len().try_into().unwrap(),
                values.as_ptr(),
            );
        }
    }
    pub unsafe fn Output(&self, count: u32, values: *mut u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Output)(
                windows_core::Interface::as_raw(self),
                count,
                values as _,
            )
        }
    }
    pub unsafe fn Explicit(&self, values: *mut u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Explicit)(
                windows_core::Interface::as_raw(self),
                values as _,
            )
        }
    }
    pub unsafe fn Arithmetic(&self, count: u32, values: *mut u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Arithmetic)(
                windows_core::Interface::as_raw(self),
                count,
                values as _,
            )
        }
    }
    pub unsafe fn Returned(&self, count: u32, values: *mut *mut u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Returned)(
                windows_core::Interface::as_raw(self),
                count,
                values as _,
            )
        }
    }
    pub unsafe fn OptionalCount(
        &self,
        count: Option<*const u32>,
        values: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).OptionalCount)(
                windows_core::Interface::as_raw(self),
                count.unwrap_or(core::mem::zeroed()) as _,
                values as _,
            )
        }
    }
    pub unsafe fn UnknownDirection(&self, count: u32, values: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnknownDirection)(
                windows_core::Interface::as_raw(self),
                count,
                values as _,
            );
        }
    }
    pub unsafe fn Residual(&self, count: u32, values: *mut u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Residual)(
                windows_core::Interface::as_raw(self),
                count,
                values as _,
            )
        }
    }
    pub unsafe fn ResidualInput(&self, count: u32, values: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).ResidualInput)(
                windows_core::Interface::as_raw(self),
                count,
                values,
            );
        }
    }
    pub unsafe fn ResidualCount(&self, count: u32, values: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).ResidualCount)(
                windows_core::Interface::as_raw(self),
                count,
                values,
            );
        }
    }
    pub unsafe fn Scalar(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scalar)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Query<T>(&self) -> windows_core::Result<T>
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
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GuidBuffer(
        &self,
        filters: &[windows_core::GUID],
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GuidBuffer)(
                windows_core::Interface::as_raw(self),
                filters.len().try_into().unwrap(),
                filters.as_ptr(),
                result as _,
            )
        }
    }
    pub unsafe fn OutputBuffer(
        &self,
        riid: *const windows_core::GUID,
        count: u32,
        results: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).OutputBuffer)(
                windows_core::Interface::as_raw(self),
                riid,
                count,
                results as _,
            )
        }
    }
    pub unsafe fn ResidualQuery(
        &self,
        filters: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ResidualQuery)(
                windows_core::Interface::as_raw(self),
                filters,
                result as _,
            )
        }
    }
    pub unsafe fn ResidualOutput(
        &self,
        riid: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ResidualOutput)(
                windows_core::Interface::as_raw(self),
                riid,
                result as _,
            )
        }
    }
}
#[repr(C)]
pub struct Buffers_Vtbl {
    pub Input: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32),
    pub Output:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Explicit:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Arithmetic:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Returned: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut u32,
    ) -> windows_core::HRESULT,
    pub OptionalCount: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub UnknownDirection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32),
    pub Residual:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub ResidualInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32),
    pub ResidualCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32),
    pub Scalar:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GuidBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OutputBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ResidualQuery: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ResidualOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait Buffers_Impl {
    fn Input(&self, count: u32, values: *const u32);
    fn Output(&self, count: u32, values: *mut u32) -> windows_core::Result<()>;
    fn Explicit(&self, values: *mut u32) -> windows_core::Result<()>;
    fn Arithmetic(&self, count: u32, values: *mut u32) -> windows_core::Result<()>;
    fn Returned(&self, count: u32, values: *mut *mut u32) -> windows_core::Result<()>;
    fn OptionalCount(&self, count: *const u32, values: *mut u32) -> windows_core::Result<()>;
    fn UnknownDirection(&self, count: u32, values: *const u32);
    fn Residual(&self, count: u32, values: *mut u32) -> windows_core::Result<()>;
    fn ResidualInput(&self, count: u32, values: *const u32);
    fn ResidualCount(&self, count: u32, values: *const u32);
    fn Scalar(&self) -> windows_core::Result<u32>;
    fn Query(
        &self,
        riid: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GuidBuffer(
        &self,
        count: u32,
        filters: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn OutputBuffer(
        &self,
        riid: *const windows_core::GUID,
        count: u32,
        results: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn ResidualQuery(
        &self,
        filters: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn ResidualOutput(
        &self,
        riid: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
}
impl Buffers_Vtbl {
    pub const fn new<Identity: Buffers_Impl>() -> Self {
        unsafe extern "system" fn Input<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Input(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn Output<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Output(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                )
                .into()
            }
        }
        unsafe extern "system" fn Explicit<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Explicit(this, core::mem::transmute_copy(&values)).into()
            }
        }
        unsafe extern "system" fn Arithmetic<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Arithmetic(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                )
                .into()
            }
        }
        unsafe extern "system" fn Returned<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *mut *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Returned(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                )
                .into()
            }
        }
        unsafe extern "system" fn OptionalCount<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: *const u32,
            values: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::OptionalCount(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                )
                .into()
            }
        }
        unsafe extern "system" fn UnknownDirection<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::UnknownDirection(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn Residual<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Residual(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                )
                .into()
            }
        }
        unsafe extern "system" fn ResidualInput<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::ResidualInput(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn ResidualCount<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::ResidualCount(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn Scalar<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match Buffers_Impl::Scalar(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Query<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Query(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn GuidBuffer<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            count: u32,
            filters: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::GuidBuffer(
                    this,
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&filters),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn OutputBuffer<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            count: u32,
            results: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::OutputBuffer(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&results),
                )
                .into()
            }
        }
        unsafe extern "system" fn ResidualQuery<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            filters: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::ResidualQuery(
                    this,
                    core::mem::transmute_copy(&filters),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn ResidualOutput<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::ResidualOutput(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        Self {
            Input: Input::<Identity>,
            Output: Output::<Identity>,
            Explicit: Explicit::<Identity>,
            Arithmetic: Arithmetic::<Identity>,
            Returned: Returned::<Identity>,
            OptionalCount: OptionalCount::<Identity>,
            UnknownDirection: UnknownDirection::<Identity>,
            Residual: Residual::<Identity>,
            ResidualInput: ResidualInput::<Identity>,
            ResidualCount: ResidualCount::<Identity>,
            Scalar: Scalar::<Identity>,
            Query: Query::<Identity>,
            GuidBuffer: GuidBuffer::<Identity>,
            OutputBuffer: OutputBuffer::<Identity>,
            ResidualQuery: ResidualQuery::<Identity>,
            ResidualOutput: ResidualOutput::<Identity>,
        }
    }
}
struct Buffers_ImplVtbl<T: Buffers_Impl>(core::marker::PhantomData<T>);
impl<T: Buffers_Impl> Buffers_ImplVtbl<T> {
    const VTABLE: Buffers_Vtbl = Buffers_Vtbl::new::<T>();
}
impl Buffers {
    pub fn new<'a, T: Buffers_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &Buffers_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ObjectContracts, ObjectContracts_Vtbl);
impl ObjectContracts {
    pub unsafe fn Required<P0>(&self, value: P0)
    where
        P0: windows_core::Param<Tracked>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Required)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            );
        }
    }
    pub unsafe fn Optional<P0>(&self, value: P0)
    where
        P0: windows_core::Param<Tracked>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Optional)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            );
        }
    }
    pub unsafe fn Unspecified(&self, value: &Option<Tracked>) {
        unsafe {
            (windows_core::Interface::vtable(self).Unspecified)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            );
        }
    }
}
#[repr(C)]
pub struct ObjectContracts_Vtbl {
    pub Required: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub Optional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub Unspecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
pub trait ObjectContracts_Impl {
    fn Required(&self, value: windows_core::Ref<Tracked>);
    fn Optional(&self, value: windows_core::Ref<Tracked>);
    fn Unspecified(&self, value: windows_core::Ref<Tracked>);
}
impl ObjectContracts_Vtbl {
    pub const fn new<Identity: ObjectContracts_Impl>() -> Self {
        unsafe extern "system" fn Required<Identity: ObjectContracts_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ObjectContracts_Impl::Required(this, core::mem::transmute_copy(&value));
            }
        }
        unsafe extern "system" fn Optional<Identity: ObjectContracts_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ObjectContracts_Impl::Optional(this, core::mem::transmute_copy(&value));
            }
        }
        unsafe extern "system" fn Unspecified<Identity: ObjectContracts_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ObjectContracts_Impl::Unspecified(this, core::mem::transmute_copy(&value));
            }
        }
        Self {
            Required: Required::<Identity>,
            Optional: Optional::<Identity>,
            Unspecified: Unspecified::<Identity>,
        }
    }
}
struct ObjectContracts_ImplVtbl<T: ObjectContracts_Impl>(core::marker::PhantomData<T>);
impl<T: ObjectContracts_Impl> ObjectContracts_ImplVtbl<T> {
    const VTABLE: ObjectContracts_Vtbl = ObjectContracts_Vtbl::new::<T>();
}
impl ObjectContracts {
    pub fn new<'a, T: ObjectContracts_Impl>(
        this: &'a T,
    ) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &ObjectContracts_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(
    Tracked,
    Tracked_Vtbl,
    0x6911d859_eae4_4a56_9076_76fc4227c710
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
