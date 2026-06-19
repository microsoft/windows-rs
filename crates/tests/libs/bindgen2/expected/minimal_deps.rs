#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Enum(pub i32);
impl Enum {
    pub const First: Self = Self(0);
    pub const Second: Self = Self(1);
}
impl windows_core::TypeKind for Enum {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Enum {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Test.Enum;i4)");
}
windows_core::imp::define_interface!(
    Interface,
    Interface_Vtbl,
    0x15cc7583_8b29_53d3_b0ee_7091c6f91977
);
impl windows_core::RuntimeType for Interface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    Interface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Interface {
    pub fn Method(&self, value: Struct) -> windows_core::Result<Enum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Method)(
                windows_core::Interface::as_raw(self),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for Interface {
    const NAME: &'static str = "Test.Interface";
}
pub trait Interface_Impl: windows_core::IUnknownImpl {
    fn Method(&self, value: &Struct) -> windows_core::Result<Enum>;
}
impl Interface_Vtbl {
    pub const fn new<Identity: Interface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Method<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: Struct,
            result__: *mut Enum,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Interface_Impl::Method(this, core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, Interface, OFFSET>(),
            Method: Method::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Interface as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Struct,
        *mut Enum,
    ) -> windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
impl windows_core::TypeKind for Struct {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Struct {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Struct;i4;i4)");
}
