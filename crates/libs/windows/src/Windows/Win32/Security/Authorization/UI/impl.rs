pub trait IEffectivePermissionImpl: Sized {
    fn GetEffectivePermission();
}
pub trait IEffectivePermission2Impl: Sized {
    fn ComputeEffectivePermissionWithSecondarySecurity();
}
pub trait ISecurityInformationImpl: Sized {
    fn GetObjectInformation();
    fn GetSecurity();
    fn SetSecurity();
    fn GetAccessRights();
    fn MapGeneric();
    fn GetInheritTypes();
    fn PropertySheetPageCallback();
}
pub trait ISecurityInformation2Impl: Sized {
    fn IsDaclCanonical();
    fn LookupSids();
}
pub trait ISecurityInformation3Impl: Sized {
    fn GetFullResourceName();
    fn OpenElevatedEditor();
}
pub trait ISecurityInformation4Impl: Sized {
    fn GetSecondarySecurity();
}
pub trait ISecurityObjectTypeInfoImpl: Sized {
    fn GetInheritSource();
}
