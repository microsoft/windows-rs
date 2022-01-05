pub trait IGameExplorerImpl: Sized {
    fn AddGame();
    fn RemoveGame();
    fn UpdateGame();
    fn VerifyAccess();
}
pub trait IGameExplorer2Impl: Sized {
    fn InstallGame();
    fn UninstallGame();
    fn CheckAccess();
}
pub trait IGameStatisticsImpl: Sized {
    fn GetMaxCategoryLength();
    fn GetMaxNameLength();
    fn GetMaxValueLength();
    fn GetMaxCategories();
    fn GetMaxStatsPerCategory();
    fn SetCategoryTitle();
    fn GetCategoryTitle();
    fn GetStatistic();
    fn SetStatistic();
    fn Save();
    fn SetLastPlayedCategory();
    fn GetLastPlayedCategory();
}
pub trait IGameStatisticsMgrImpl: Sized {
    fn GetGameStatistics();
    fn RemoveGameStatistics();
}
pub trait IXblIdpAuthManagerImpl: Sized {
    fn SetGamerAccount();
    fn GetGamerAccount();
    fn SetAppViewInitialized();
    fn GetEnvironment();
    fn GetSandbox();
    fn GetTokenAndSignatureWithTokenResult();
}
pub trait IXblIdpAuthTokenResultImpl: Sized {
    fn GetStatus();
    fn GetErrorCode();
    fn GetToken();
    fn GetSignature();
    fn GetSandbox();
    fn GetEnvironment();
    fn GetMsaAccountId();
    fn GetXuid();
    fn GetGamertag();
    fn GetAgeGroup();
    fn GetPrivileges();
    fn GetMsaTarget();
    fn GetMsaPolicy();
    fn GetMsaAppId();
    fn GetRedirect();
    fn GetMessage();
    fn GetHelpId();
    fn GetEnforcementBans();
    fn GetRestrictions();
    fn GetTitleRestrictions();
}
pub trait IXblIdpAuthTokenResult2Impl: Sized {
    fn GetModernGamertag();
    fn GetModernGamertagSuffix();
    fn GetUniqueModernGamertag();
}
