#[cfg(feature = "Win32_System_Com")]
pub trait ICatalogImpl: Sized + IDispatchImpl {
    fn GetCollection();
    fn Connect();
    fn MajorVersion();
    fn MinorVersion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentUtilImpl: Sized + IDispatchImpl {
    fn InstallComponent();
    fn ImportComponent();
    fn ImportComponentByName();
    fn GetCLSIDs();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPackageUtilImpl: Sized + IDispatchImpl {
    fn InstallPackage();
    fn ExportPackage();
    fn ShutdownPackage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteComponentUtilImpl: Sized + IDispatchImpl {
    fn InstallRemoteComponent();
    fn InstallRemoteComponentByName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRoleAssociationUtilImpl: Sized + IDispatchImpl {
    fn AssociateRole();
    fn AssociateRoleByName();
}
