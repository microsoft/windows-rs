#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CopyValue {
    pub value: i32,
}
impl windows_core::TypeKind for CopyValue {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CopyValue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.CopyValue;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.CopyValue");
}
windows_core::imp::define_interface!(
    Handler,
    Handler_Vtbl,
    0xb6e1d103_fd8f_567a_99ae_41bc5e8a6de1
);
impl windows_core::RuntimeType for Handler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl Handler {
    pub fn new<
        F: Fn(
                &windows_core::HSTRING,
                windows_core::Ref<windows_core::IInspectable>,
                Kind,
                &CopyValue,
                &OwnedValue,
                &[i32],
            ) -> windows_core::Result<OwnedValue>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&HandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P1>(
        &self,
        text: &windows_core::HSTRING,
        object: P1,
        kind: Kind,
        copy: CopyValue,
        owned: &OwnedValue,
        values: &[i32],
    ) -> windows_core::Result<OwnedValue>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(text),
                object.param().abi(),
                kind,
                copy,
                core::mem::transmute_copy(owned),
                values.len().try_into().unwrap(),
                values.as_ptr(),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
pub struct Handler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        text: *mut core::ffi::c_void,
        object: *mut core::ffi::c_void,
        kind: Kind,
        copy: CopyValue,
        owned: core::mem::MaybeUninit<OwnedValue>,
        values_array_size: u32,
        values: *const i32,
        result__: *mut core::mem::MaybeUninit<OwnedValue>,
    ) -> windows_core::HRESULT,
}
struct HandlerBox<
    F: Fn(
            &windows_core::HSTRING,
            windows_core::Ref<windows_core::IInspectable>,
            Kind,
            &CopyValue,
            &OwnedValue,
            &[i32],
        ) -> windows_core::Result<OwnedValue>
        + Send
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            &windows_core::HSTRING,
            windows_core::Ref<windows_core::IInspectable>,
            Kind,
            &CopyValue,
            &OwnedValue,
            &[i32],
        ) -> windows_core::Result<OwnedValue>
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
        text: *mut core::ffi::c_void,
        object: *mut core::ffi::c_void,
        kind: Kind,
        copy: CopyValue,
        owned: core::mem::MaybeUninit<OwnedValue>,
        values_array_size: u32,
        values: *const i32,
        result__: *mut core::mem::MaybeUninit<OwnedValue>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Handler, F>);
            match (this.invoke)(
                core::mem::transmute(&text),
                core::mem::transmute_copy(&object),
                kind,
                core::mem::transmute(&copy),
                core::mem::transmute(&owned),
                core::slice::from_raw_parts(
                    core::mem::transmute_copy(&values),
                    values_array_size as usize,
                ),
            ) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Kind(pub i32);
impl Kind {
    pub const First: Self = Self(0);
}
impl windows_core::TypeKind for Kind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Kind {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Test.Kind;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Kind");
}
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OwnedValue {
    pub value: windows_core::HSTRING,
}
impl windows_core::TypeKind for OwnedValue {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for OwnedValue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.OwnedValue;string)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.OwnedValue");
}
