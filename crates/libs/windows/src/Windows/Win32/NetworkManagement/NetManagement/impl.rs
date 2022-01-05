pub trait IEnumNetCfgBindingInterfaceImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumNetCfgBindingPathImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumNetCfgComponentImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait INetCfgImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn Apply();
    fn Cancel();
    fn EnumComponents();
    fn FindComponent();
    fn QueryNetCfgClass();
}
pub trait INetCfgBindingInterfaceImpl: Sized {
    fn GetName();
    fn GetUpperComponent();
    fn GetLowerComponent();
}
pub trait INetCfgBindingPathImpl: Sized {
    fn IsSamePathAs();
    fn IsSubPathOf();
    fn IsEnabled();
    fn Enable();
    fn GetPathToken();
    fn GetOwner();
    fn GetDepth();
    fn EnumBindingInterfaces();
}
pub trait INetCfgClassImpl: Sized {
    fn FindComponent();
    fn EnumComponents();
}
pub trait INetCfgClassSetupImpl: Sized {
    fn SelectAndInstall();
    fn Install();
    fn DeInstall();
}
pub trait INetCfgClassSetup2Impl: Sized + INetCfgClassSetupImpl {
    fn UpdateNonEnumeratedComponent();
}
pub trait INetCfgComponentImpl: Sized {
    fn GetDisplayName();
    fn SetDisplayName();
    fn GetHelpText();
    fn GetId();
    fn GetCharacteristics();
    fn GetInstanceGuid();
    fn GetPnpDevNodeId();
    fn GetClassGuid();
    fn GetBindName();
    fn GetDeviceStatus();
    fn OpenParamKey();
    fn RaisePropertyUi();
}
pub trait INetCfgComponentBindingsImpl: Sized {
    fn BindTo();
    fn UnbindFrom();
    fn SupportsBindingInterface();
    fn IsBoundTo();
    fn IsBindableTo();
    fn EnumBindingPaths();
    fn MoveBefore();
    fn MoveAfter();
}
pub trait INetCfgComponentControlImpl: Sized {
    fn Initialize();
    fn ApplyRegistryChanges();
    fn ApplyPnpChanges();
    fn CancelChanges();
}
pub trait INetCfgComponentNotifyBindingImpl: Sized {
    fn QueryBindingPath();
    fn NotifyBindingPath();
}
pub trait INetCfgComponentNotifyGlobalImpl: Sized {
    fn GetSupportedNotifications();
    fn SysQueryBindingPath();
    fn SysNotifyBindingPath();
    fn SysNotifyComponent();
}
pub trait INetCfgComponentPropertyUiImpl: Sized {
    fn QueryPropertyUi();
    fn SetContext();
    fn MergePropPages();
    fn ValidateProperties();
    fn ApplyProperties();
    fn CancelProperties();
}
pub trait INetCfgComponentSetupImpl: Sized {
    fn Install();
    fn Upgrade();
    fn ReadAnswerFile();
    fn Removing();
}
pub trait INetCfgComponentSysPrepImpl: Sized {
    fn SaveAdapterParameters();
    fn RestoreAdapterParameters();
}
pub trait INetCfgComponentUpperEdgeImpl: Sized {
    fn GetInterfaceIdsForAdapter();
    fn AddInterfacesToAdapter();
    fn RemoveInterfacesFromAdapter();
}
pub trait INetCfgLockImpl: Sized {
    fn AcquireWriteLock();
    fn ReleaseWriteLock();
    fn IsWriteLocked();
}
pub trait INetCfgPnpReconfigCallbackImpl: Sized {
    fn SendPnpReconfig();
}
pub trait INetCfgSysPrepImpl: Sized {
    fn HrSetupSetFirstDword();
    fn HrSetupSetFirstString();
    fn HrSetupSetFirstStringAsBool();
    fn HrSetupSetFirstMultiSzField();
}
pub trait INetLanConnectionUiInfoImpl: Sized {
    fn GetDeviceGuid();
}
pub trait INetRasConnectionIpUiInfoImpl: Sized {
    fn GetUiInfo();
}
pub trait IProvisioningDomainImpl: Sized {
    fn Add();
    fn Query();
}
pub trait IProvisioningProfileWirelessImpl: Sized {
    fn CreateProfile();
}
