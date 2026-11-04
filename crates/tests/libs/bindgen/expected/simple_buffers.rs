windows_core::imp::define_interface!(Buffers, Buffers_Vtbl);
impl Buffers {
    pub unsafe fn Input(&self, values: *const u32, count: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Input)(
                windows_core::Interface::as_raw(self),
                values,
                count,
            );
        }
    }
    pub unsafe fn Fixed(&self, values: *const u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Fixed)(
                windows_core::Interface::as_raw(self),
                values,
            );
        }
    }
    pub unsafe fn InOut(&self, values: *mut u32, count: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).InOut)(
                windows_core::Interface::as_raw(self),
                values as _,
                count,
            );
        }
    }
    pub unsafe fn Optional(&self, values: Option<*const u32>, count: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Optional)(
                windows_core::Interface::as_raw(self),
                values.unwrap_or(core::mem::zeroed()) as _,
                count,
            );
        }
    }
    pub unsafe fn Output(
        &self,
        values: *mut u32,
        count: u32,
        written: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Output)(
                windows_core::Interface::as_raw(self),
                values as _,
                count,
                written as _,
            )
        }
    }
    pub unsafe fn Returned(&self, values: *mut *mut u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Returned)(
                windows_core::Interface::as_raw(self),
                values as _,
            )
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
    pub unsafe fn QueryArray(
        &self,
        iid: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).QueryArray)(
                windows_core::Interface::as_raw(self),
                iid,
                result as _,
            )
        }
    }
    pub unsafe fn QueryWithBuffer(
        &self,
        ids: *const windows_core::GUID,
        count: u32,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).QueryWithBuffer)(
                windows_core::Interface::as_raw(self),
                ids,
                count,
                result as _,
            )
        }
    }
    pub unsafe fn BufferResult(&self, value: *mut u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).BufferResult)(
                windows_core::Interface::as_raw(self),
                value as _,
            )
        }
    }
}
#[repr(C)]
pub struct Buffers_Vtbl {
    pub Input: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32),
    pub Fixed: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32),
    pub InOut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32),
    pub Optional: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32),
    pub Output: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Returned:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u32) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub QueryArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub QueryWithBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BufferResult:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait Buffers_Impl {
    fn Input(&self, values: *const u32, count: u32);
    fn Fixed(&self, values: *const u32);
    fn InOut(&self, values: *mut u32, count: u32);
    fn Optional(&self, values: *const u32, count: u32);
    fn Output(&self, values: *mut u32, count: u32, written: *mut u32) -> windows_core::HRESULT;
    fn Returned(&self, values: *mut *mut u32) -> windows_core::HRESULT;
    fn Query(
        &self,
        iid: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn QueryArray(
        &self,
        iid: *const windows_core::GUID,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT;
    fn QueryWithBuffer(
        &self,
        ids: *const windows_core::GUID,
        count: u32,
        result: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT;
    fn BufferResult(&self, value: *mut u32) -> windows_core::HRESULT;
}
impl Buffers_Vtbl {
    pub const fn new<Identity: Buffers_Impl>() -> Self {
        unsafe extern "system" fn Input<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *const u32,
            count: u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Input(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                );
            }
        }
        unsafe extern "system" fn Fixed<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *const u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Fixed(this, core::mem::transmute_copy(&values));
            }
        }
        unsafe extern "system" fn InOut<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *mut u32,
            count: u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::InOut(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                );
            }
        }
        unsafe extern "system" fn Optional<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *const u32,
            count: u32,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Optional(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                );
            }
        }
        unsafe extern "system" fn Output<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *mut u32,
            count: u32,
            written: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Output(
                    this,
                    core::mem::transmute_copy(&values),
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&written),
                )
            }
        }
        unsafe extern "system" fn Returned<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            values: *mut *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Returned(this, core::mem::transmute_copy(&values))
            }
        }
        unsafe extern "system" fn Query<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            iid: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::Query(
                    this,
                    core::mem::transmute_copy(&iid),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn QueryArray<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            iid: *const windows_core::GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::QueryArray(
                    this,
                    core::mem::transmute_copy(&iid),
                    core::mem::transmute_copy(&result),
                )
            }
        }
        unsafe extern "system" fn QueryWithBuffer<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            ids: *const windows_core::GUID,
            count: u32,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::QueryWithBuffer(
                    this,
                    core::mem::transmute_copy(&ids),
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&result),
                )
            }
        }
        unsafe extern "system" fn BufferResult<Identity: Buffers_Impl>(
            this: *mut core::ffi::c_void,
            value: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Buffers_Impl::BufferResult(this, core::mem::transmute_copy(&value))
            }
        }
        Self {
            Input: Input::<Identity>,
            Fixed: Fixed::<Identity>,
            InOut: InOut::<Identity>,
            Optional: Optional::<Identity>,
            Output: Output::<Identity>,
            Returned: Returned::<Identity>,
            Query: Query::<Identity>,
            QueryArray: QueryArray::<Identity>,
            QueryWithBuffer: QueryWithBuffer::<Identity>,
            BufferResult: BufferResult::<Identity>,
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
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
