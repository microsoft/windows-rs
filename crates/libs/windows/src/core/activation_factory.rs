use super::*;

/// WinRT classes have a supporting factory object that implements `IActivationFactory` to create a new
/// instance of the WinRT class with some default state. `IActivationFactory` represents the
/// [IActivationFactory](https://docs.microsoft.com/en-us/windows/win32/api/activation/nn-activation-iactivationfactory)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IActivationFactory(IInspectable);

impl IActivationFactory {
    /// Creates an instance of the WinRT class associated with the factory object.
    ///
    /// The `activate_instance` method corresponds to the "default constructor" in languages like C# and C++.
    pub fn activate_instance<I: Interface>(&self) -> Result<I> {
        unsafe {
            let mut object = None;

            // Even though the factory will generally return the WinRT default interface, this isn't guaranteed
            // so a cast is required to convert the `IInspectable` into `I`, or the class type.
            (self.vtable().ActivateInstance)(core::mem::transmute_copy(self), &mut object).and_some(object)?.cast()
        }
    }
}

#[repr(C)]
pub struct IActivationFactoryVtbl {
    pub base: IInspectableVtbl,
    pub ActivateInstance: unsafe extern "system" fn(this: RawPtr, object: &mut Option<IInspectable>) -> HRESULT,
}

unsafe impl Interface for IActivationFactory {
    type Vtable = IActivationFactoryVtbl;

    const IID: GUID = GUID::from_u128(0x00000035_0000_0000_c000_000000000046);
}

impl core::fmt::Debug for IActivationFactory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IActivationFactory").field(&self.0).finish()
    }
}
