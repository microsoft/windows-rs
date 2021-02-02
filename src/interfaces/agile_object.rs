use com::sys::GUID;
use com::Interface;
use com::interfaces::IUnknown;

/// A marker interface indicating that an object is free-threaded and may be called from any apartment.
/// `IAgileObject` represents the
/// [IAgileObject](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iagileobject)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IAgileObject(IUnknown);

unsafe impl Interface for IAgileObject {
    type VTable = <IUnknown as Interface>::VTable;
    type Super = IUnknown;

    const IID: GUID = GUID::from_values(
        0x94EA_2B94,
        0xE9CC,
        0x49E0,
        [0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90],
    );
}

impl std::fmt::Debug for IAgileObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
