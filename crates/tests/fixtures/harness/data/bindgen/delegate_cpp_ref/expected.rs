#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub const CLASS_E_CLASSNOTAVAILABLE: windows_core::HRESULT =
    windows_core::HRESULT(0x80040111_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D_FEATURE_LEVEL(pub i32);
windows_core::imp::define_interface!(
    IActivationFactory,
    IActivationFactory_Vtbl,
    0x00000035_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(
    IActivationFactory,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IActivationFactory {
    pub unsafe fn ActivateInstance(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateInstance)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IActivationFactory_Impl: windows_core::IUnknownImpl {
    fn ActivateInstance(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl IActivationFactory_Vtbl {
    pub const fn new<Identity: IActivationFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateInstance<
            Identity: IActivationFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            instance: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivationFactory_Impl::ActivateInstance(this) {
                    Ok(ok__) => {
                        instance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivationFactory, OFFSET>(),
            ActivateInstance: ActivateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivationFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActivationFactory {}
pub type PFNGETACTIVATIONFACTORY = Option<
    unsafe extern "system" fn(
        param0: windows_core::Ref<windows_core::HSTRING>,
        param1: windows_core::OutRef<IActivationFactory>,
    ) -> windows_core::HRESULT,
>;
pub type PFN_D3D12_CREATE_DEVICE = Option<
    unsafe extern "system" fn(
        param0: windows_core::Ref<windows_core::IUnknown>,
        param1: D3D_FEATURE_LEVEL,
        param2: *const windows_core::GUID,
        param3: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
>;
pub const S_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
