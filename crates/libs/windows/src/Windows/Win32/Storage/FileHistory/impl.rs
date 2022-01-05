pub trait IFhConfigMgrImpl: Sized {
    fn LoadConfiguration();
    fn CreateDefaultConfiguration();
    fn SaveConfiguration();
    fn AddRemoveExcludeRule();
    fn GetIncludeExcludeRules();
    fn GetLocalPolicy();
    fn SetLocalPolicy();
    fn GetBackupStatus();
    fn SetBackupStatus();
    fn GetDefaultTarget();
    fn ValidateTarget();
    fn ProvisionAndSetNewTarget();
    fn ChangeDefaultTargetRecommendation();
    fn QueryProtectionStatus();
}
pub trait IFhReassociationImpl: Sized {
    fn ValidateTarget();
    fn ScanTargetForConfigurations();
    fn GetConfigurationDetails();
    fn SelectConfiguration();
    fn PerformReassociation();
}
pub trait IFhScopeIteratorImpl: Sized {
    fn MoveToNextItem();
    fn GetItem();
}
pub trait IFhTargetImpl: Sized {
    fn GetStringProperty();
    fn GetNumericalProperty();
}
