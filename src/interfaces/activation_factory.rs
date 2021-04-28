use super::*;

/// WinRT classes have a supporting factory object that implements `IActivationFactory` to create a new
/// instance of the WinRT class with some default state. `IActivationFactory` represents the
/// [IActivationFactory](https://docs.microsoft.com/en-us/windows/win32/api/activation/nn-activation-iactivationfactory)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IActivationFactory(Object);

impl IActivationFactory {
    /// Creates an instance of the WinRT class associated with the factory object.
    ///
    /// The `activate_instance` method corresponds to the "default constructor" in languages like C# and C++.
    pub fn activate_instance<I: Interface>(&self) -> Result<I> {
        unsafe {
            let mut object = None;

            // Even though the factory will generally return the WinRT default interface, this isn't guaranteed
            // so a cast is required to convert the `Object` into `I`, or the class type.
            (self.vtable().activate_instance)(self.abi(), &mut object)
                .and_some(object)?
                .cast()
        }
    }
}

#[repr(C)]
pub struct IActivationFactory_vtable {
    pubobject_vtable: Object_vtable,
    pub activate_instance:
        unsafe extern "system" fn(this: RawPtr, object: &mut Option<Object>) -> HRESULT,
}

unsafe impl Interface for IActivationFactory {
    type Vtable = IActivationFactory_vtable;

    const IID: Guid = Guid::from_values(
        0x0000_0035,
        0x0000,
        0x0000,
        [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    );
}

impl std::fmt::Debug for IActivationFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
