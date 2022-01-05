#[cfg(feature = "implement_exclusive")]
pub trait IApiInformationStaticsImpl: Sized {
    fn IsTypePresent(&self, typename: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsMethodPresent(&self, typename: &::windows::core::HSTRING, methodname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsMethodPresentWithArity(&self, typename: &::windows::core::HSTRING, methodname: &::windows::core::HSTRING, inputparametercount: u32) -> ::windows::core::Result<bool>;
    fn IsEventPresent(&self, typename: &::windows::core::HSTRING, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsPropertyPresent(&self, typename: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsReadOnlyPropertyPresent(&self, typename: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsWriteablePropertyPresent(&self, typename: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsEnumNamedValuePresent(&self, enumtypename: &::windows::core::HSTRING, valuename: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsApiContractPresentByMajor(&self, contractname: &::windows::core::HSTRING, majorversion: u16) -> ::windows::core::Result<bool>;
    fn IsApiContractPresentByMajorAndMinor(&self, contractname: &::windows::core::HSTRING, majorversion: u16, minorversion: u16) -> ::windows::core::Result<bool>;
}
