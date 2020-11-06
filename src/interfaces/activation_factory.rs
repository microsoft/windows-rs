use crate::*;

#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct IActivationFactory(Object);

#[repr(C)]
pub struct IActivationFactory_vtable(
    pub IUnknown_QueryInterface,
    pub IUnknown_AddRef,
    pub IUnknown_Release,
    pub Object_GetIids,
    pub Object_GetRuntimeClassName,
    pub Object_GetTrustLevel,
    pub extern "system" fn(this: RawPtr, object: &mut Option<Object>) -> ErrorCode, // ActivateInstance
);

unsafe impl Interface for IActivationFactory {
    type Vtable = IActivationFactory_vtable;

    const IID: Guid = {
        Guid::from_values(
            0x0000_0035,
            0x0000,
            0x0000,
            [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        )
    };
}

impl std::fmt::Debug for IActivationFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl IActivationFactory {
    pub fn activate_instance<I: Interface>(&self) -> Result<I> {
        unsafe {
            let mut object = None;
            (self.vtable().6)(self.get_abi(), &mut object)
                .and_some(object)?
                .cast_ok()
        }
    }
}
