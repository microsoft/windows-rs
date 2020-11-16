use crate::*;

/// WinRT classes have a supporting factory object that implements `IActivationFactory` to create a new
/// instance of the WinRT class with some default state. `IActivationFactory` implements the
/// [IActivationFactory](https://docs.microsoft.com/en-us/windows/win32/api/activation/nn-activation-iactivationfactory)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct IActivationFactory(Object);

#[repr(C)]
pub struct IActivationFactory_vtable(
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    extern "system" fn(this: RawPtr, object: &mut Option<Object>) -> ErrorCode, // ActivateInstance
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
                .cast()
        }
    }
}
