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

unsafe impl ComInterface for IActivationFactory {
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

unsafe impl GetAbi for IActivationFactory {
    type Abi = RawPtr;

    unsafe fn get_abi(&self) -> RawPtr {
        self.0.get_abi()
    }
}

impl IActivationFactory {
    pub fn activate_instance<I: ComInterface>(&self) -> Result<I> {
        unsafe {
            let mut object = None;
            (self.vtable().6)(self.get_abi(), &mut object)
                .and_some(object)?
                .query()
        }
    }
}
