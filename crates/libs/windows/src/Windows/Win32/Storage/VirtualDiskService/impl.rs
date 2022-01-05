pub trait IEnumVdsObjectImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IVdsAdminImpl: Sized {
    fn RegisterProvider();
    fn UnregisterProvider();
}
pub trait IVdsAdviseSinkImpl: Sized {
    fn OnNotify();
}
pub trait IVdsAsyncImpl: Sized {
    fn Cancel();
    fn Wait();
    fn QueryStatus();
}
pub trait IVdsControllerImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn GetPortProperties();
    fn FlushCache();
    fn InvalidateCache();
    fn Reset();
    fn QueryAssociatedLuns();
    fn SetStatus();
}
pub trait IVdsControllerControllerPortImpl: Sized {
    fn QueryControllerPorts();
}
pub trait IVdsControllerPortImpl: Sized {
    fn GetProperties();
    fn GetController();
    fn QueryAssociatedLuns();
    fn Reset();
    fn SetStatus();
}
pub trait IVdsDriveImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn QueryExtents();
    fn SetFlags();
    fn ClearFlags();
    fn SetStatus();
}
pub trait IVdsDrive2Impl: Sized {
    fn GetProperties2();
}
pub trait IVdsHwProviderImpl: Sized {
    fn QuerySubSystems();
    fn Reenumerate();
    fn Refresh();
}
pub trait IVdsHwProviderPrivateImpl: Sized {
    fn QueryIfCreatedLun();
}
pub trait IVdsHwProviderPrivateMpioImpl: Sized {
    fn SetAllPathStatusesFromHbaPort();
}
pub trait IVdsHwProviderStoragePoolsImpl: Sized {
    fn QueryStoragePools();
    fn CreateLunInStoragePool();
    fn QueryMaxLunCreateSizeInStoragePool();
}
pub trait IVdsHwProviderTypeImpl: Sized {
    fn GetProviderType();
}
pub trait IVdsHwProviderType2Impl: Sized {
    fn GetProviderType2();
}
pub trait IVdsIscsiPortalImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn QueryAssociatedPortalGroups();
    fn SetStatus();
    fn SetIpsecTunnelAddress();
    fn GetIpsecSecurity();
    fn SetIpsecSecurity();
}
pub trait IVdsIscsiPortalGroupImpl: Sized {
    fn GetProperties();
    fn GetTarget();
    fn QueryAssociatedPortals();
    fn AddPortal();
    fn RemovePortal();
    fn Delete();
}
pub trait IVdsIscsiTargetImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn QueryPortalGroups();
    fn QueryAssociatedLuns();
    fn CreatePortalGroup();
    fn Delete();
    fn SetFriendlyName();
    fn SetSharedSecret();
    fn RememberInitiatorSharedSecret();
    fn GetConnectedInitiators();
}
pub trait IVdsLunImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn GetIdentificationData();
    fn QueryActiveControllers();
    fn Extend();
    fn Shrink();
    fn QueryPlexes();
    fn AddPlex();
    fn RemovePlex();
    fn Recover();
    fn SetMask();
    fn Delete();
    fn AssociateControllers();
    fn QueryHints();
    fn ApplyHints();
    fn SetStatus();
    fn QueryMaxLunExtendSize();
}
pub trait IVdsLun2Impl: Sized {
    fn QueryHints2();
    fn ApplyHints2();
}
pub trait IVdsLunControllerPortsImpl: Sized {
    fn AssociateControllerPorts();
    fn QueryActiveControllerPorts();
}
pub trait IVdsLunIscsiImpl: Sized {
    fn AssociateTargets();
    fn QueryAssociatedTargets();
}
pub trait IVdsLunMpioImpl: Sized {
    fn GetPathInfo();
    fn GetLoadBalancePolicy();
    fn SetLoadBalancePolicy();
    fn GetSupportedLbPolicies();
}
pub trait IVdsLunNamingImpl: Sized {
    fn SetFriendlyName();
}
pub trait IVdsLunNumberImpl: Sized {
    fn GetLunNumber();
}
pub trait IVdsLunPlexImpl: Sized {
    fn GetProperties();
    fn GetLun();
    fn QueryExtents();
    fn QueryHints();
    fn ApplyHints();
}
pub trait IVdsMaintenanceImpl: Sized {
    fn StartMaintenance();
    fn StopMaintenance();
    fn PulseMaintenance();
}
pub trait IVdsProviderImpl: Sized {
    fn GetProperties();
}
pub trait IVdsProviderPrivateImpl: Sized {
    fn GetObject();
    fn OnLoad();
    fn OnUnload();
}
pub trait IVdsProviderSupportImpl: Sized {
    fn GetVersionSupport();
}
pub trait IVdsStoragePoolImpl: Sized {
    fn GetProvider();
    fn GetProperties();
    fn GetAttributes();
    fn QueryDriveExtents();
    fn QueryAllocatedLuns();
    fn QueryAllocatedStoragePools();
}
pub trait IVdsSubSystemImpl: Sized {
    fn GetProperties();
    fn GetProvider();
    fn QueryControllers();
    fn QueryLuns();
    fn QueryDrives();
    fn GetDrive();
    fn Reenumerate();
    fn SetControllerStatus();
    fn CreateLun();
    fn ReplaceDrive();
    fn SetStatus();
    fn QueryMaxLunCreateSize();
}
pub trait IVdsSubSystem2Impl: Sized {
    fn GetProperties2();
    fn GetDrive2();
    fn CreateLun2();
    fn QueryMaxLunCreateSize2();
}
pub trait IVdsSubSystemInterconnectImpl: Sized {
    fn GetSupportedInterconnects();
}
pub trait IVdsSubSystemIscsiImpl: Sized {
    fn QueryTargets();
    fn QueryPortals();
    fn CreateTarget();
    fn SetIpsecGroupPresharedKey();
}
pub trait IVdsSubSystemNamingImpl: Sized {
    fn SetFriendlyName();
}
