windows_core::imp::define_interface!(Interface, Interface_Vtbl);
impl Interface {
    pub unsafe fn ResultVoid(&self, value: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ResultVoid)(
                windows_core::Interface::as_raw(self),
                value,
            )
        }
    }
    pub unsafe fn ResultValue(&self, value: u32) -> windows_core::Result<Struct> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultValue)(
                windows_core::Interface::as_raw(self),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn ReturnStruct(&self, value: u32) -> Struct {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReturnStruct)(
                windows_core::Interface::as_raw(self),
                value,
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn ReturnValue(&self, value: u32) -> Struct {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReturnValue)(
                windows_core::Interface::as_raw(self),
                value,
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn ExplicitAfterInputOutput(&self, state: *mut u32) -> windows_core::Result<Struct> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExplicitAfterInputOutput)(
                windows_core::Interface::as_raw(self),
                state as _,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn HeuristicAfterInputOutput(
        &self,
        state: *mut u32,
        result: *mut Struct,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HeuristicAfterInputOutput)(
                windows_core::Interface::as_raw(self),
                state as _,
                result as _,
            )
        }
    }
    pub unsafe fn RetvalInputOutput(&self, result: *mut Struct) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RetvalInputOutput)(
                windows_core::Interface::as_raw(self),
                result as _,
            )
        }
    }
    pub unsafe fn RetvalOptional(&self, result: Option<*mut Struct>) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RetvalOptional)(
                windows_core::Interface::as_raw(self),
                result.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn RetvalReserved(&self, result: Option<*mut Struct>) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RetvalReserved)(
                windows_core::Interface::as_raw(self),
                result.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn RetvalArray(&self, result: *mut Struct) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RetvalArray)(
                windows_core::Interface::as_raw(self),
                result as _,
            )
        }
    }
    pub unsafe fn ExplicitLarge(&self) -> windows_core::Result<Large> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExplicitLarge)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn HeuristicLarge(&self, result: *mut Large) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HeuristicLarge)(
                windows_core::Interface::as_raw(self),
                result as _,
            )
        }
    }
    pub unsafe fn ExplicitVoidPointer(&self) -> windows_core::Result<*mut core::ffi::c_void> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExplicitVoidPointer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub ResultVoid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ResultValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut Struct,
    ) -> windows_core::HRESULT,
    pub ReturnStruct: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut Struct),
    pub ReturnValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut Struct),
    pub ExplicitAfterInputOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut Struct,
    ) -> windows_core::HRESULT,
    pub HeuristicAfterInputOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut Struct,
    ) -> windows_core::HRESULT,
    pub RetvalInputOutput:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Struct) -> windows_core::HRESULT,
    pub RetvalOptional:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Struct) -> windows_core::HRESULT,
    pub RetvalReserved:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Struct) -> windows_core::HRESULT,
    pub RetvalArray:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Struct) -> windows_core::HRESULT,
    pub ExplicitLarge:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Large) -> windows_core::HRESULT,
    pub HeuristicLarge:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Large) -> windows_core::HRESULT,
    pub ExplicitVoidPointer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait Interface_Impl {
    fn ResultVoid(&self, value: u32) -> windows_core::Result<()>;
    fn ResultValue(&self, value: u32) -> windows_core::Result<Struct>;
    fn ReturnStruct(&self, value: u32, result: *mut Struct);
    fn ReturnValue(&self, value: u32, result: *mut Struct);
    fn ExplicitAfterInputOutput(&self, state: *mut u32) -> windows_core::Result<Struct>;
    fn HeuristicAfterInputOutput(
        &self,
        state: *mut u32,
        result: *mut Struct,
    ) -> windows_core::Result<()>;
    fn RetvalInputOutput(&self, result: *mut Struct) -> windows_core::Result<()>;
    fn RetvalOptional(&self, result: *mut Struct) -> windows_core::Result<()>;
    fn RetvalReserved(&self, result: *mut Struct) -> windows_core::Result<()>;
    fn RetvalArray(&self, result: *mut Struct) -> windows_core::Result<()>;
    fn ExplicitLarge(&self) -> windows_core::Result<Large>;
    fn HeuristicLarge(&self, result: *mut Large) -> windows_core::Result<()>;
    fn ExplicitVoidPointer(&self) -> windows_core::Result<*mut core::ffi::c_void>;
}
impl Interface_Vtbl {
    pub const fn new<Identity: Interface_Impl>() -> Self {
        unsafe extern "system" fn ResultVoid<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ResultVoid(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ResultValue<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
            result: *mut Struct,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match Interface_Impl::ResultValue(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        result.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReturnStruct<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
            result: *mut Struct,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ReturnStruct(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&result),
                );
            }
        }
        unsafe extern "system" fn ReturnValue<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            value: u32,
            result: *mut Struct,
        ) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::ReturnValue(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&result),
                );
            }
        }
        unsafe extern "system" fn ExplicitAfterInputOutput<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            state: *mut u32,
            result: *mut Struct,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match Interface_Impl::ExplicitAfterInputOutput(
                    this,
                    core::mem::transmute_copy(&state),
                ) {
                    Ok(ok__) => {
                        result.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HeuristicAfterInputOutput<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            state: *mut u32,
            result: *mut Struct,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::HeuristicAfterInputOutput(
                    this,
                    core::mem::transmute_copy(&state),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        unsafe extern "system" fn RetvalInputOutput<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            result: *mut Struct,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::RetvalInputOutput(this, core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn RetvalOptional<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            result: *mut Struct,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::RetvalOptional(this, core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn RetvalReserved<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            result: *mut Struct,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::RetvalReserved(this, core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn RetvalArray<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            result: *mut Struct,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::RetvalArray(this, core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn ExplicitLarge<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            result: *mut Large,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match Interface_Impl::ExplicitLarge(this) {
                    Ok(ok__) => {
                        result.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HeuristicLarge<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            result: *mut Large,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                Interface_Impl::HeuristicLarge(this, core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn ExplicitVoidPointer<Identity: Interface_Impl>(
            this: *mut core::ffi::c_void,
            result: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match Interface_Impl::ExplicitVoidPointer(this) {
                    Ok(ok__) => {
                        result.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            ResultVoid: ResultVoid::<Identity>,
            ResultValue: ResultValue::<Identity>,
            ReturnStruct: ReturnStruct::<Identity>,
            ReturnValue: ReturnValue::<Identity>,
            ExplicitAfterInputOutput: ExplicitAfterInputOutput::<Identity>,
            HeuristicAfterInputOutput: HeuristicAfterInputOutput::<Identity>,
            RetvalInputOutput: RetvalInputOutput::<Identity>,
            RetvalOptional: RetvalOptional::<Identity>,
            RetvalReserved: RetvalReserved::<Identity>,
            RetvalArray: RetvalArray::<Identity>,
            ExplicitLarge: ExplicitLarge::<Identity>,
            HeuristicLarge: HeuristicLarge::<Identity>,
            ExplicitVoidPointer: ExplicitVoidPointer::<Identity>,
        }
    }
}
struct Interface_ImplVtbl<T: Interface_Impl>(core::marker::PhantomData<T>);
impl<T: Interface_Impl> Interface_ImplVtbl<T> {
    const VTABLE: Interface_Vtbl = Interface_Vtbl::new::<T>();
}
impl Interface {
    pub fn new<'a, T: Interface_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap {
            vtable: &Interface_ImplVtbl::<T>::VTABLE as *const _ as *const _,
            this: this as *const _ as *const _,
        };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Large {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
    pub e: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
