#[cfg(feature = "Win32_System_Com")]
pub trait IDiskQuotaControlImpl: Sized + IConnectionPointContainerImpl {
    fn Initialize();
    fn SetQuotaState();
    fn GetQuotaState();
    fn SetQuotaLogFlags();
    fn GetQuotaLogFlags();
    fn SetDefaultQuotaThreshold();
    fn GetDefaultQuotaThreshold();
    fn GetDefaultQuotaThresholdText();
    fn SetDefaultQuotaLimit();
    fn GetDefaultQuotaLimit();
    fn GetDefaultQuotaLimitText();
    fn AddUserSid();
    fn AddUserName();
    fn DeleteUser();
    fn FindUserSid();
    fn FindUserName();
    fn CreateEnumUsers();
    fn CreateUserBatch();
    fn InvalidateSidNameCache();
    fn GiveUserNameResolutionPriority();
    fn ShutdownNameResolution();
}
pub trait IDiskQuotaEventsImpl: Sized {
    fn OnUserNameChanged();
}
pub trait IDiskQuotaUserImpl: Sized {
    fn GetID();
    fn GetName();
    fn GetSidLength();
    fn GetSid();
    fn GetQuotaThreshold();
    fn GetQuotaThresholdText();
    fn GetQuotaLimit();
    fn GetQuotaLimitText();
    fn GetQuotaUsed();
    fn GetQuotaUsedText();
    fn GetQuotaInformation();
    fn SetQuotaThreshold();
    fn SetQuotaLimit();
    fn Invalidate();
    fn GetAccountStatus();
}
pub trait IDiskQuotaUserBatchImpl: Sized {
    fn Add();
    fn Remove();
    fn RemoveAll();
    fn FlushToDisk();
}
pub trait IEnumDiskQuotaUsersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
