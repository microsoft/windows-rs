use crate::*;

struct IWeakReferenceSource {}
impl QueryType for IWeakReferenceSource {
    fn type_guid() -> &'static Guid {
        static GUID: Guid = Guid::from_values(
            0x00000038,
            0x0000,
            0x0000,
            &[0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        );
        &GUID
    }
}

pub mod abi {
    use com::interfaces::IUnknown;
    #[com::com_interface("00000037-0000-0000-C000-000000000046")]
    pub trait IWeakReference: IUnknown {
        unsafe fn resolve(
            &self,
            guid: *const crate::Guid,
            ptr: *mut crate::RawPtr,
        ) -> crate::ErrorCode;
    }

    #[com::com_interface("00000038-0000-0000-C000-000000000046")]
    pub trait IWeakReferenceSource: IUnknown {
        unsafe fn get_weak_reference(&self, ptr: *mut crate::RawPtr) -> crate::ErrorCode;
    }
}
