windows_core::imp::define_interface!(
    ArrayRoundtrip,
    ArrayRoundtrip_Vtbl,
    0xf993673f_d62b_5f56_a513_cf169550c52e
);
impl windows_core::RuntimeType for ArrayRoundtrip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ArrayRoundtrip {
    pub fn new<
        F: Fn(
                &[Option<Tracked>],
                &mut [Option<Tracked>],
                &mut windows_core::Array<Tracked>,
            ) -> windows_core::Result<windows_core::Array<Tracked>>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com =
            windows_core::imp::DelegateBox::<Self, F>::new(&ArrayRoundtripBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke(
        &self,
        input: &[Option<Tracked>],
        fill: &mut [Option<Tracked>],
        receive: &mut windows_core::Array<Tracked>,
    ) -> windows_core::Result<windows_core::Array<Tracked>> {
        if core::mem::needs_drop::<Option<Tracked>>() {
            fill.fill_with(|| unsafe { core::mem::zeroed() });
        }
        receive.clear();
        unsafe {
            let mut result__ = core::ptr::null_mut();
            let mut result_size__ = 0;
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                input.len().try_into().unwrap(),
                input.as_ptr().cast(),
                fill.len().try_into().unwrap(),
                core::mem::transmute_copy(&fill),
                receive.set_abi_len(),
                receive as *mut _ as _,
                &mut result_size__,
                &mut result__,
            )
            .map(|| windows_core::Array::<Tracked>::from_raw_parts(result__.cast(), result_size__))
        }
    }
}
#[repr(C)]
pub struct ArrayRoundtrip_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        input_array_size: u32,
        input: *const Tracked,
        fill_array_size: u32,
        fill: *mut Tracked,
        receive_array_size: *mut u32,
        receive: *mut *mut Tracked,
        result_size__: *mut u32,
        result__: *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct ArrayRoundtripBox<
    F: Fn(
            &[Option<Tracked>],
            &mut [Option<Tracked>],
            &mut windows_core::Array<Tracked>,
        ) -> windows_core::Result<windows_core::Array<Tracked>>
        + Send
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            &[Option<Tracked>],
            &mut [Option<Tracked>],
            &mut windows_core::Array<Tracked>,
        ) -> windows_core::Result<windows_core::Array<Tracked>>
        + Send
        + 'static,
> ArrayRoundtripBox<F>
{
    const VTABLE: ArrayRoundtrip_Vtbl = ArrayRoundtrip_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<ArrayRoundtrip, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<ArrayRoundtrip, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<ArrayRoundtrip, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        input_array_size: u32,
        input: *const Tracked,
        fill_array_size: u32,
        fill: *mut Tracked,
        receive_array_size: *mut u32,
        receive: *mut *mut Tracked,
        result_size__: *mut u32,
        result__: *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<ArrayRoundtrip, F>);
            match (this.invoke)(
                windows_core::imp::array_slice(core::mem::transmute_copy(&input), input_array_size),
                windows_core::imp::array_slice_mut(
                    core::mem::transmute_copy(&fill),
                    fill_array_size,
                ),
                &mut windows_core::imp::array_proxy(
                    core::mem::transmute_copy(&receive),
                    receive_array_size,
                ),
            ) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    result__.write(ok_data__.cast());
                    result_size__.write(ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => {
                    result__.write_bytes(0, 1);
                    result_size__.write(0);
                    err.into()
                }
            }
        }
    }
}
windows_core::imp::define_interface!(
    EmptyArray,
    EmptyArray_Vtbl,
    0x764888f4_0729_5edc_a35b_5f948bf43a06
);
impl windows_core::RuntimeType for EmptyArray {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl EmptyArray {
    pub fn new<F: Fn() -> windows_core::Result<windows_core::Array<u32>> + Send + 'static>(
        invoke: F,
    ) -> Self {
        let com =
            windows_core::imp::DelegateBox::<Self, F>::new(&EmptyArrayBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke(&self) -> windows_core::Result<windows_core::Array<u32>> {
        unsafe {
            let mut result__ = core::ptr::null_mut();
            let mut result_size__ = 0;
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                &mut result_size__,
                &mut result__,
            )
            .map(|| windows_core::Array::<u32>::from_raw_parts(result__.cast(), result_size__))
        }
    }
}
#[repr(C)]
pub struct EmptyArray_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u32,
    ) -> windows_core::HRESULT,
}
struct EmptyArrayBox<F: Fn() -> windows_core::Result<windows_core::Array<u32>> + Send + 'static>(
    core::marker::PhantomData<(fn() -> F,)>,
);
impl<F: Fn() -> windows_core::Result<windows_core::Array<u32>> + Send + 'static> EmptyArrayBox<F> {
    const VTABLE: EmptyArray_Vtbl = EmptyArray_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<EmptyArray, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<EmptyArray, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<EmptyArray, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<EmptyArray, F>);
            match (this.invoke)() {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    result__.write(ok_data__);
                    result_size__.write(ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => {
                    result__.write_bytes(0, 1);
                    result_size__.write(0);
                    err.into()
                }
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericOutputs<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for GenericOutputs<T> {
    type Vtable = GenericOutputs_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for GenericOutputs<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({a3352366-b31c-5ee5-9b2c-6419932f806c}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> GenericOutputs<T> {
    pub fn new<
        F: Fn(
                &mut [<T as windows_core::imp::Type<T>>::Default],
                &mut <T as windows_core::imp::Type<T>>::Default,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &GenericOutputsBox::<T, F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke(
        &self,
        fill: &mut [<T as windows_core::imp::Type<T>>::Default],
        value: &mut <T as windows_core::imp::Type<T>>::Default,
    ) -> windows_core::Result<()> {
        if core::mem::needs_drop::<<T as windows_core::imp::Type<T>>::Default>() {
            fill.fill_with(|| unsafe { core::mem::zeroed() });
        }
        if core::mem::needs_drop::<<T as windows_core::imp::Type<T>>::Default>() {
            *value = unsafe { core::mem::zeroed() };
        }
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                fill.len().try_into().unwrap(),
                core::mem::transmute_copy(&fill),
                value as *mut _ as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct GenericOutputs_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        fill_array_size: u32,
        fill: *mut windows_core::imp::AbiType<T>,
        value: *mut windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
struct GenericOutputsBox<
    T,
    F: Fn(
            &mut [<T as windows_core::imp::Type<T>>::Default],
            &mut <T as windows_core::imp::Type<T>>::Default,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
>(core::marker::PhantomData<(T, fn() -> F)>)
where
    T: windows_core::RuntimeType + 'static;
impl<
    T: windows_core::RuntimeType + 'static,
    F: Fn(
            &mut [<T as windows_core::imp::Type<T>>::Default],
            &mut <T as windows_core::imp::Type<T>>::Default,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> GenericOutputsBox<T, F>
{
    const VTABLE: GenericOutputs_Vtbl<T> = GenericOutputs_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<GenericOutputs<T>, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<GenericOutputs<T>, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<GenericOutputs<T>, F>::Release,
        },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        fill_array_size: u32,
        fill: *mut windows_core::imp::AbiType<T>,
        value: *mut windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<GenericOutputs<T>, F>);
            (this.invoke)(
                windows_core::imp::array_slice_mut(
                    core::mem::transmute_copy(&fill),
                    fill_array_size,
                ),
                core::mem::transmute_copy(&value),
            )
            .into()
        }
    }
}
windows_core::imp::define_interface!(
    InterfaceOutputs,
    InterfaceOutputs_Vtbl,
    0x6b7b2966_3dec_5f46_995f_0350eb967c7d
);
impl windows_core::RuntimeType for InterfaceOutputs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl InterfaceOutputs {
    pub fn new<
        F: Fn(windows_core::OutRef<Tracked>) -> windows_core::Result<Tracked> + Send + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &InterfaceOutputsBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke(&self, value: &mut Option<Tracked>) -> windows_core::Result<Tracked> {
        if core::mem::needs_drop::<Option<Tracked>>() {
            *value = unsafe { core::mem::zeroed() };
        }
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                value as *mut _ as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct InterfaceOutputs_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        value: *mut *mut core::ffi::c_void,
        result__: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct InterfaceOutputsBox<
    F: Fn(windows_core::OutRef<Tracked>) -> windows_core::Result<Tracked> + Send + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::OutRef<Tracked>) -> windows_core::Result<Tracked> + Send + 'static>
    InterfaceOutputsBox<F>
{
    const VTABLE: InterfaceOutputs_Vtbl = InterfaceOutputs_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<InterfaceOutputs, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<InterfaceOutputs, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<InterfaceOutputs, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        value: *mut *mut core::ffi::c_void,
        result__: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<InterfaceOutputs, F>);
            match (this.invoke)(core::mem::transmute_copy(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => {
                    result__.write_bytes(0, 1);
                    err.into()
                }
            }
        }
    }
}
windows_core::imp::define_interface!(
    Tracked,
    Tracked_Vtbl,
    0x1cc94528_d335_506f_b2c7_3ec2a56e820c
);
impl windows_core::RuntimeType for Tracked {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl Tracked {
    pub fn new<F: Fn() -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&TrackedBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct Tracked_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct TrackedBox<F: Fn() -> windows_core::Result<()> + Send + 'static>(
    core::marker::PhantomData<(fn() -> F,)>,
);
impl<F: Fn() -> windows_core::Result<()> + Send + 'static> TrackedBox<F> {
    const VTABLE: Tracked_Vtbl = Tracked_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<Tracked, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<Tracked, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<Tracked, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Tracked, F>);
            (this.invoke)().into()
        }
    }
}
