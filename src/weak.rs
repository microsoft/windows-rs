use crate::*;

struct IWeakReferenceSource {}
impl QueryType for IWeakReferenceSource {
    fn type_guid() -> &'static Guid {
        use com::ComInterface;
        static GUID: Guid = Guid(abi::IWeakReferenceSource::IID);
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
