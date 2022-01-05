pub trait AsyncIFtpAuthenticationProviderImpl: Sized {
    fn Begin_AuthenticateUser();
    fn Finish_AuthenticateUser();
}
pub trait AsyncIFtpAuthorizationProviderImpl: Sized {
    fn Begin_GetUserAccessPermission();
    fn Finish_GetUserAccessPermission();
}
pub trait AsyncIFtpHomeDirectoryProviderImpl: Sized {
    fn Begin_GetUserHomeDirectoryData();
    fn Finish_GetUserHomeDirectoryData();
}
pub trait AsyncIFtpLogProviderImpl: Sized {
    fn Begin_Log();
    fn Finish_Log();
}
pub trait AsyncIFtpPostprocessProviderImpl: Sized {
    fn Begin_HandlePostprocess();
    fn Finish_HandlePostprocess();
}
pub trait AsyncIFtpPreprocessProviderImpl: Sized {
    fn Begin_HandlePreprocess();
    fn Finish_HandlePreprocess();
}
pub trait AsyncIFtpRoleProviderImpl: Sized {
    fn Begin_IsUserInRole();
    fn Finish_IsUserInRole();
}
pub trait AsyncIMSAdminBaseSinkWImpl: Sized {
    fn Begin_SinkNotify();
    fn Finish_SinkNotify();
    fn Begin_ShutdownNotify();
    fn Finish_ShutdownNotify();
}
pub trait IADMEXTImpl: Sized {
    fn Initialize();
    fn EnumDcomCLSIDs();
    fn Terminate();
}
pub trait IFtpAuthenticationProviderImpl: Sized {
    fn AuthenticateUser();
}
pub trait IFtpAuthorizationProviderImpl: Sized {
    fn GetUserAccessPermission();
}
pub trait IFtpHomeDirectoryProviderImpl: Sized {
    fn GetUserHomeDirectoryData();
}
pub trait IFtpLogProviderImpl: Sized {
    fn Log();
}
pub trait IFtpPostprocessProviderImpl: Sized {
    fn HandlePostprocess();
}
pub trait IFtpPreprocessProviderImpl: Sized {
    fn HandlePreprocess();
}
pub trait IFtpProviderConstructImpl: Sized {
    fn Construct();
}
pub trait IFtpRoleProviderImpl: Sized {
    fn IsUserInRole();
}
pub trait IMSAdminBase2WImpl: Sized + IMSAdminBaseWImpl {
    fn BackupWithPasswd();
    fn RestoreWithPasswd();
    fn Export();
    fn Import();
    fn RestoreHistory();
    fn EnumHistory();
}
pub trait IMSAdminBase3WImpl: Sized + IMSAdminBase2WImpl + IMSAdminBaseWImpl {
    fn GetChildPaths();
}
pub trait IMSAdminBaseSinkWImpl: Sized {
    fn SinkNotify();
    fn ShutdownNotify();
}
pub trait IMSAdminBaseWImpl: Sized {
    fn AddKey();
    fn DeleteKey();
    fn DeleteChildKeys();
    fn EnumKeys();
    fn CopyKey();
    fn RenameKey();
    fn SetData();
    fn GetData();
    fn DeleteData();
    fn EnumData();
    fn GetAllData();
    fn DeleteAllData();
    fn CopyData();
    fn GetDataPaths();
    fn OpenKey();
    fn CloseKey();
    fn ChangePermissions();
    fn SaveData();
    fn GetHandleInfo();
    fn GetSystemChangeNumber();
    fn GetDataSetNumber();
    fn SetLastChangeTime();
    fn GetLastChangeTime();
    fn KeyExchangePhase1();
    fn KeyExchangePhase2();
    fn Backup();
    fn Restore();
    fn EnumBackups();
    fn DeleteBackup();
    fn UnmarshalInterface();
    fn GetServerGuid();
}
pub trait IMSImpExpHelpWImpl: Sized {
    fn EnumeratePathsInFile();
}
