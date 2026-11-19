#[inline]
pub unsafe fn BorrowOwned(value: *const OwnedValue) -> *const OwnedValue {
    windows_core::link!("bindgen_abi.dll" "system" fn BorrowOwned(value : *const OwnedValue) -> *const OwnedValue);
    unsafe { BorrowOwned(value) }
}
#[inline]
pub unsafe fn GetOwned(status: windows_core::HRESULT) -> windows_core::Result<OwnedValue> {
    windows_core::link!("bindgen_abi.dll" "system" fn GetOwned(status : windows_core::HRESULT, value : *mut OwnedValue) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::MaybeUninit::<OwnedValue>::zeroed();
        GetOwned(status, result__.as_mut_ptr().cast()).map(|| result__.assume_init())
    }
}
#[inline]
pub unsafe fn GetOwnedArray(
    status: windows_core::HRESULT,
) -> windows_core::Result<[OwnedValue; 2]> {
    windows_core::link!("bindgen_abi.dll" "system" fn GetOwnedArray(status : windows_core::HRESULT, value : *mut [OwnedValue; 2]) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::MaybeUninit::<[OwnedValue; 2]>::zeroed();
        GetOwnedArray(status, result__.as_mut_ptr().cast()).map(|| result__.assume_init())
    }
}
#[inline]
pub unsafe fn GetOwnedNestedArray(
    status: windows_core::HRESULT,
) -> windows_core::Result<[[OwnedValue; 2]; 2]> {
    windows_core::link!("bindgen_abi.dll" "system" fn GetOwnedNestedArray(status : windows_core::HRESULT, value : *mut [[OwnedValue; 2]; 2]) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::MaybeUninit::<[[OwnedValue; 2]; 2]>::zeroed();
        GetOwnedNestedArray(status, result__.as_mut_ptr().cast()).map(|| result__.assume_init())
    }
}
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
            + Sync
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
        + Sync
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            &[Option<Tracked>],
            &mut [Option<Tracked>],
            &mut windows_core::Array<Tracked>,
        ) -> windows_core::Result<windows_core::Array<Tracked>>
        + Send
        + Sync
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
            let this = &*(this as *mut *mut core::ffi::c_void
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
    pub fn new<
        F: Fn() -> windows_core::Result<windows_core::Array<u32>> + Send + Sync + 'static,
    >(
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
struct EmptyArrayBox<
    F: Fn() -> windows_core::Result<windows_core::Array<u32>> + Send + Sync + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn() -> windows_core::Result<windows_core::Array<u32>> + Send + Sync + 'static>
    EmptyArrayBox<F>
{
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
            let this = &*(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<EmptyArray, F>);
            match (this.invoke)() {
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
                &mut [<T as windows_core::imp::Type>::Default],
                &mut <T as windows_core::imp::Type>::Default,
            ) -> windows_core::Result<()>
            + Send
            + Sync
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
        fill: &mut [<T as windows_core::imp::Type>::Default],
        value: &mut <T as windows_core::imp::Type>::Default,
    ) -> windows_core::Result<()> {
        if core::mem::needs_drop::<<T as windows_core::imp::Type>::Default>() {
            fill.fill_with(|| unsafe { core::mem::zeroed() });
        }
        if core::mem::needs_drop::<<T as windows_core::imp::Type>::Default>() {
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
            &mut [<T as windows_core::imp::Type>::Default],
            &mut <T as windows_core::imp::Type>::Default,
        ) -> windows_core::Result<()>
        + Send
        + Sync
        + 'static,
>(core::marker::PhantomData<(T, fn() -> F)>)
where
    T: windows_core::RuntimeType + 'static;
impl<
    T: windows_core::RuntimeType + 'static,
    F: Fn(
            &mut [<T as windows_core::imp::Type>::Default],
            &mut <T as windows_core::imp::Type>::Default,
        ) -> windows_core::Result<()>
        + Send
        + Sync
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
            let this = &*(this as *mut *mut core::ffi::c_void
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
windows_core::imp::define_winrt_interface!(GenericResult < T >, GenericResult_Vtbl < T, >, 0xf83d59d6_7deb_5e2e_8219_79ce2ee23f8b, b"Test.GenericResult`1");
impl<T: windows_core::RuntimeType + 'static> GenericResult<T> {
    pub fn Get(&self) -> windows_core::Result<T> {
        unsafe {
            let mut result__ =
                core::mem::MaybeUninit::<<T as windows_core::imp::Type>::Default>::zeroed();
            (windows_core::Interface::vtable(self).Get)(
                windows_core::Interface::as_raw(self),
                result__.as_mut_ptr().cast(),
            )
            .and_then(|| windows_core::imp::Type::from_default_owned(result__.assume_init()))
        }
    }
    pub fn Infallible(&self) -> T {
        unsafe {
            let mut result__ =
                core::mem::MaybeUninit::<<T as windows_core::imp::Type>::Default>::zeroed();
            let hresult__ = (windows_core::Interface::vtable(self).Infallible)(
                windows_core::Interface::as_raw(self),
                result__.as_mut_ptr().cast(),
            );
            debug_assert!(hresult__.0 == 0);
            windows_core::imp::Type::from_default_owned(result__.assume_init()).unwrap()
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for GenericResult<T> {
    const NAME: &'static str = "Test.GenericResult";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
pub trait GenericResult_Impl<T>: windows_core::IUnknownImpl
where
    T: windows_core::RuntimeType + 'static,
{
    fn Get(&self) -> windows_core::Result<T>;
    fn Infallible(&self) -> T;
}
impl<T: windows_core::RuntimeType + 'static> GenericResult_Vtbl<T> {
    pub const fn new<Identity: GenericResult_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<
            T: windows_core::RuntimeType + 'static,
            Identity: GenericResult_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::imp::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match GenericResult_Impl::Get(this) {
                    Ok(ok__) => {
                        core::ptr::write(result__.cast(), ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Infallible<
            T: windows_core::RuntimeType + 'static,
            Identity: GenericResult_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::imp::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let ok__ = GenericResult_Impl::Infallible(this);
                core::ptr::write(result__.cast(), ok__);
                windows_core::HRESULT(0)
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<T, Identity, OFFSET>,
            Infallible: Infallible::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<GenericResult<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct GenericResult_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Get: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub Infallible: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericRoundtrip<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface
    for GenericRoundtrip<T>
{
    type Vtable = GenericRoundtrip_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for GenericRoundtrip<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({a812a876-8e1b-571c-9d41-496d6548d463}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> GenericRoundtrip<T> {
    pub fn new<
        F: Fn(
                windows_core::Ref<T>,
                &mut <T as windows_core::imp::Type>::Default,
            ) -> windows_core::Result<T>
            + Send
            + Sync
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &GenericRoundtripBox::<T, F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0>(
        &self,
        input: P0,
        output: &mut <T as windows_core::imp::Type>::Default,
    ) -> windows_core::Result<T>
    where
        P0: windows_core::Param<T>,
    {
        if core::mem::needs_drop::<<T as windows_core::imp::Type>::Default>() {
            *output = unsafe { core::mem::zeroed() };
        }
        unsafe {
            let mut result__ =
                core::mem::MaybeUninit::<<T as windows_core::imp::Type>::Default>::zeroed();
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                input.param().abi(),
                output as *mut _ as _,
                result__.as_mut_ptr().cast(),
            )
            .and_then(|| windows_core::imp::Type::from_default_owned(result__.assume_init()))
        }
    }
}
#[repr(C)]
pub struct GenericRoundtrip_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        input: windows_core::imp::AbiType<T>,
        output: *mut windows_core::imp::AbiType<T>,
        result__: *mut windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
struct GenericRoundtripBox<
    T,
    F: Fn(
            windows_core::Ref<T>,
            &mut <T as windows_core::imp::Type>::Default,
        ) -> windows_core::Result<T>
        + Send
        + Sync
        + 'static,
>(core::marker::PhantomData<(T, fn() -> F)>)
where
    T: windows_core::RuntimeType + 'static;
impl<
    T: windows_core::RuntimeType + 'static,
    F: Fn(
            windows_core::Ref<T>,
            &mut <T as windows_core::imp::Type>::Default,
        ) -> windows_core::Result<T>
        + Send
        + Sync
        + 'static,
> GenericRoundtripBox<T, F>
{
    const VTABLE: GenericRoundtrip_Vtbl<T> = GenericRoundtrip_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<GenericRoundtrip<T>, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<GenericRoundtrip<T>, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<GenericRoundtrip<T>, F>::Release,
        },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        input: windows_core::imp::AbiType<T>,
        output: *mut windows_core::imp::AbiType<T>,
        result__: *mut windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &*(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<GenericRoundtrip<T>, F>);
            match (this.invoke)(
                core::mem::transmute_copy(&input),
                core::mem::transmute_copy(&output),
            ) {
                Ok(ok__) => {
                    core::ptr::write(result__.cast(), ok__);
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
        F: Fn(windows_core::OutRef<Tracked>) -> windows_core::Result<Tracked> + Send + Sync + 'static,
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
            .and_then(|| windows_core::imp::from_abi(result__))
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
    F: Fn(windows_core::OutRef<Tracked>) -> windows_core::Result<Tracked> + Send + Sync + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::OutRef<Tracked>) -> windows_core::Result<Tracked> + Send + Sync + 'static>
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
            let this = &*(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<InterfaceOutputs, F>);
            match (this.invoke)(core::mem::transmute_copy(&value)) {
                Ok(ok__) => {
                    core::ptr::write(result__.cast(), ok__);
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
    NativeResult,
    NativeResult_Vtbl,
    0x176f5c04_39ba_5f30_a468_c991c44f766c
);
windows_core::imp::interface_hierarchy!(NativeResult, windows_core::IUnknown);
impl NativeResult {
    pub unsafe fn Get(&self) -> windows_core::Result<OwnedValue> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::<OwnedValue>::zeroed();
            (windows_core::Interface::vtable(self).Get)(
                windows_core::Interface::as_raw(self),
                result__.as_mut_ptr().cast(),
            )
            .map(|| result__.assume_init())
        }
    }
    pub unsafe fn GetArray(&self) -> windows_core::Result<[OwnedValue; 2]> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::<[OwnedValue; 2]>::zeroed();
            (windows_core::Interface::vtable(self).GetArray)(
                windows_core::Interface::as_raw(self),
                result__.as_mut_ptr().cast(),
            )
            .map(|| result__.assume_init())
        }
    }
    pub unsafe fn GetNestedArray(&self) -> windows_core::Result<[[OwnedValue; 2]; 2]> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::<[[OwnedValue; 2]; 2]>::zeroed();
            (windows_core::Interface::vtable(self).GetNestedArray)(
                windows_core::Interface::as_raw(self),
                result__.as_mut_ptr().cast(),
            )
            .map(|| result__.assume_init())
        }
    }
}
#[repr(C)]
pub struct NativeResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Get:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut OwnedValue) -> windows_core::HRESULT,
    pub GetArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut [OwnedValue; 2],
    ) -> windows_core::HRESULT,
    pub GetNestedArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut [[OwnedValue; 2]; 2],
    ) -> windows_core::HRESULT,
}
pub trait NativeResult_Impl: windows_core::IUnknownImpl {
    fn Get(&self) -> windows_core::Result<OwnedValue>;
    fn GetArray(&self) -> windows_core::Result<[OwnedValue; 2]>;
    fn GetNestedArray(&self) -> windows_core::Result<[[OwnedValue; 2]; 2]>;
}
impl NativeResult_Vtbl {
    pub const fn new<Identity: NativeResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: NativeResult_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut OwnedValue,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match NativeResult_Impl::Get(this) {
                    Ok(ok__) => {
                        value.cast::<OwnedValue>().write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        value.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetArray<Identity: NativeResult_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut [OwnedValue; 2],
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match NativeResult_Impl::GetArray(this) {
                    Ok(ok__) => {
                        value.cast::<[OwnedValue; 2]>().write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        value.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetNestedArray<
            Identity: NativeResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut [[OwnedValue; 2]; 2],
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match NativeResult_Impl::GetNestedArray(this) {
                    Ok(ok__) => {
                        value.cast::<[[OwnedValue; 2]; 2]>().write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        value.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            GetArray: GetArray::<Identity, OFFSET>,
            GetNestedArray: GetNestedArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<NativeResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for NativeResult {}
windows_core::imp::define_winrt_interface!(
    OwnedResult,
    OwnedResult_Vtbl,
    0x3bf50ee9_dfbf_55af_9ba7_a3b94c787d3b,
    b"Test.OwnedResult"
);
windows_core::imp::interface_hierarchy!(
    OwnedResult,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl OwnedResult {
    pub fn Get(&self) -> windows_core::Result<OwnedValue> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::<OwnedValue>::zeroed();
            (windows_core::Interface::vtable(self).Get)(
                windows_core::Interface::as_raw(self),
                result__.as_mut_ptr().cast(),
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn Infallible(&self) -> OwnedValue {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::<OwnedValue>::zeroed();
            let hresult__ = (windows_core::Interface::vtable(self).Infallible)(
                windows_core::Interface::as_raw(self),
                result__.as_mut_ptr().cast(),
            );
            debug_assert!(hresult__.0 == 0);
            result__.assume_init()
        }
    }
    pub fn GetMany(&self) -> windows_core::Result<windows_core::Array<OwnedValue>> {
        unsafe {
            let mut result__ = core::ptr::null_mut();
            let mut result_size__ = 0;
            (windows_core::Interface::vtable(self).GetMany)(
                windows_core::Interface::as_raw(self),
                &mut result_size__,
                &mut result__,
            )
            .map(|| {
                windows_core::Array::<OwnedValue>::from_raw_parts(result__.cast(), result_size__)
            })
        }
    }
    pub fn InfallibleMany(&self) -> windows_core::Array<OwnedValue> {
        unsafe {
            let mut result__ = core::ptr::null_mut();
            let mut result_size__ = 0;
            let hresult__ = (windows_core::Interface::vtable(self).InfallibleMany)(
                windows_core::Interface::as_raw(self),
                &mut result_size__,
                &mut result__,
            );
            debug_assert!(hresult__.0 == 0);
            windows_core::Array::<OwnedValue>::from_raw_parts(result__.cast(), result_size__)
        }
    }
}
impl windows_core::RuntimeName for OwnedResult {
    const NAME: &'static str = "Test.OwnedResult";
}
pub trait OwnedResult_Impl: windows_core::IUnknownImpl {
    fn Get(&self) -> windows_core::Result<OwnedValue>;
    fn Infallible(&self) -> OwnedValue;
    fn GetMany(&self) -> windows_core::Result<windows_core::Array<OwnedValue>>;
    fn InfallibleMany(&self) -> windows_core::Array<OwnedValue>;
}
impl OwnedResult_Vtbl {
    pub const fn new<Identity: OwnedResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: OwnedResult_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut OwnedValue,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match OwnedResult_Impl::Get(this) {
                    Ok(ok__) => {
                        core::ptr::write(result__.cast(), ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Infallible<Identity: OwnedResult_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut OwnedValue,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let ok__ = OwnedResult_Impl::Infallible(this);
                core::ptr::write(result__.cast(), ok__);
                windows_core::HRESULT(0)
            }
        }
        unsafe extern "system" fn GetMany<Identity: OwnedResult_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut OwnedValue,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match OwnedResult_Impl::GetMany(this) {
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
        unsafe extern "system" fn InfallibleMany<
            Identity: OwnedResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut OwnedValue,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let ok__ = OwnedResult_Impl::InfallibleMany(this);
                let (ok_data__, ok_data_len__) = ok__.into_abi();
                result__.write(ok_data__.cast());
                result_size__.write(ok_data_len__);
                windows_core::HRESULT(0)
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            Infallible: Infallible::<Identity, OFFSET>,
            GetMany: GetMany::<Identity, OFFSET>,
            InfallibleMany: InfallibleMany::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<OwnedResult as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct OwnedResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Get:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut OwnedValue) -> windows_core::HRESULT,
    pub Infallible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut OwnedValue) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut OwnedValue,
    ) -> windows_core::HRESULT,
    pub InfallibleMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut OwnedValue,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    OwnedRoundtrip,
    OwnedRoundtrip_Vtbl,
    0x95157be5_5f47_5813_81ee_d63666d4de8a
);
impl windows_core::RuntimeType for OwnedRoundtrip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl OwnedRoundtrip {
    pub fn new<
        F: Fn(&OwnedValue, &mut OwnedValue) -> windows_core::Result<OwnedValue>
            + Send
            + Sync
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com =
            windows_core::imp::DelegateBox::<Self, F>::new(&OwnedRoundtripBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke(
        &self,
        input: &OwnedValue,
        output: &mut OwnedValue,
    ) -> windows_core::Result<OwnedValue> {
        if core::mem::needs_drop::<OwnedValue>() {
            *output = unsafe { core::mem::zeroed() };
        }
        unsafe {
            let mut result__ = core::mem::MaybeUninit::<OwnedValue>::zeroed();
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(input),
                output as *mut _ as _,
                result__.as_mut_ptr().cast(),
            )
            .map(|| result__.assume_init())
        }
    }
}
#[repr(C)]
pub struct OwnedRoundtrip_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        input: core::mem::ManuallyDrop<OwnedValue>,
        output: *mut OwnedValue,
        result__: *mut OwnedValue,
    ) -> windows_core::HRESULT,
}
struct OwnedRoundtripBox<
    F: Fn(&OwnedValue, &mut OwnedValue) -> windows_core::Result<OwnedValue> + Send + Sync + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(&OwnedValue, &mut OwnedValue) -> windows_core::Result<OwnedValue> + Send + Sync + 'static,
> OwnedRoundtripBox<F>
{
    const VTABLE: OwnedRoundtrip_Vtbl = OwnedRoundtrip_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<OwnedRoundtrip, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<OwnedRoundtrip, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<OwnedRoundtrip, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        input: core::mem::ManuallyDrop<OwnedValue>,
        output: *mut OwnedValue,
        result__: *mut OwnedValue,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &*(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<OwnedRoundtrip, F>);
            match (this.invoke)(
                &*(&raw const input).cast(),
                core::mem::transmute_copy(&output),
            ) {
                Ok(ok__) => {
                    core::ptr::write(result__.cast(), ok__);
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
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OwnedValue {
    pub text: windows_core::HSTRING,
    pub enabled: bool,
    pub number: f64,
    pub object: Option<Tracked>,
}
impl windows_core::imp::TypeKind for OwnedValue {
    type TypeKind = windows_core::imp::CloneType;
}
impl windows_core::RuntimeType for OwnedValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Test.OwnedValue;string;b1;f8;delegate({1cc94528-d335-506f-b2c7-3ec2a56e820c}))",
    );
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.OwnedValue");
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
    pub fn new<F: Fn() -> windows_core::Result<()> + Send + Sync + 'static>(invoke: F) -> Self {
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
struct TrackedBox<F: Fn() -> windows_core::Result<()> + Send + Sync + 'static>(
    core::marker::PhantomData<(fn() -> F,)>,
);
impl<F: Fn() -> windows_core::Result<()> + Send + Sync + 'static> TrackedBox<F> {
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
            let this = &*(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Tracked, F>);
            (this.invoke)().into()
        }
    }
}
