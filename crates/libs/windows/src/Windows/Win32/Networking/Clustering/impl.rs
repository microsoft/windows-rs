pub trait IGetClusterDataInfoImpl: Sized {
    fn GetClusterName();
    fn GetClusterHandle();
    fn GetObjectCount();
}
pub trait IGetClusterGroupInfoImpl: Sized {
    fn GetGroupHandle();
}
pub trait IGetClusterNetInterfaceInfoImpl: Sized {
    fn GetNetInterfaceHandle();
}
pub trait IGetClusterNetworkInfoImpl: Sized {
    fn GetNetworkHandle();
}
pub trait IGetClusterNodeInfoImpl: Sized {
    fn GetNodeHandle();
}
pub trait IGetClusterObjectInfoImpl: Sized {
    fn GetObjectName();
    fn GetObjectType();
}
pub trait IGetClusterResourceInfoImpl: Sized {
    fn GetResourceHandle();
    fn GetResourceTypeName();
    fn GetResourceNetworkName();
}
pub trait IGetClusterUIInfoImpl: Sized {
    fn GetClusterName();
    fn GetLocale();
    fn GetFont();
    fn GetIcon();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusApplicationImpl: Sized + IDispatchImpl {
    fn DomainNames();
    fn ClusterNames();
    fn OpenCluster();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusCryptoKeysImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn AddItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusDiskImpl: Sized + IDispatchImpl {
    fn Signature();
    fn ScsiAddress();
    fn DiskNumber();
    fn Partitions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusDisksImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetInterfaceImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Name();
    fn Handle();
    fn State();
    fn Cluster();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetInterfacesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetworkImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Handle();
    fn Name();
    fn SetName();
    fn NetworkID();
    fn State();
    fn NetInterfaces();
    fn Cluster();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetworkNetInterfacesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetworksImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNodeImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Name();
    fn Handle();
    fn NodeID();
    fn State();
    fn Pause();
    fn Resume();
    fn Evict();
    fn ResourceGroups();
    fn Cluster();
    fn NetInterfaces();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNodeNetInterfacesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNodesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPartitionImpl: Sized + IDispatchImpl {
    fn Flags();
    fn DeviceName();
    fn VolumeLabel();
    fn SerialNumber();
    fn MaximumComponentLength();
    fn FileSystemFlags();
    fn FileSystem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPartitionExImpl: Sized + ISClusPartitionImpl + IDispatchImpl {
    fn TotalSize();
    fn FreeSpace();
    fn DeviceNumber();
    fn PartitionNumber();
    fn VolumeGuid();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPartitionsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertiesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn UseDefaultValue();
    fn SaveChanges();
    fn ReadOnly();
    fn Private();
    fn Common();
    fn Modified();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Length();
    fn ValueCount();
    fn Values();
    fn Value();
    fn SetValue();
    fn Type();
    fn SetType();
    fn Format();
    fn SetFormat();
    fn ReadOnly();
    fn Private();
    fn Common();
    fn Modified();
    fn UseDefaultValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyValueImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Type();
    fn SetType();
    fn Format();
    fn SetFormat();
    fn Length();
    fn DataCount();
    fn Data();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyValueDataImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn CreateItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyValuesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn CreateItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusRefObjectImpl: Sized + IDispatchImpl {
    fn Handle();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusRegistryKeysImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn AddItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResDependenciesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
    fn AddItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResDependentsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
    fn AddItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Handle();
    fn Name();
    fn SetName();
    fn State();
    fn OwnerNode();
    fn Resources();
    fn PreferredOwnerNodes();
    fn Delete();
    fn Online();
    fn Move();
    fn Offline();
    fn Cluster();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupPreferredOwnerNodesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn InsertItem();
    fn RemoveItem();
    fn Modified();
    fn SaveChanges();
    fn AddItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupResourcesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResPossibleOwnerNodesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn AddItem();
    fn RemoveItem();
    fn Modified();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypeImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Name();
    fn Delete();
    fn Cluster();
    fn Resources();
    fn PossibleOwnerNodes();
    fn AvailableDisks();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypePossibleOwnerNodesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypeResourcesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResourceImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Handle();
    fn Name();
    fn SetName();
    fn State();
    fn CoreFlag();
    fn BecomeQuorumResource();
    fn Delete();
    fn Fail();
    fn Online();
    fn Offline();
    fn ChangeResourceGroup();
    fn AddResourceNode();
    fn RemoveResourceNode();
    fn CanResourceBeDependent();
    fn PossibleOwnerNodes();
    fn Dependencies();
    fn Dependents();
    fn Group();
    fn OwnerNode();
    fn Cluster();
    fn ClassInfo();
    fn Disk();
    fn RegistryKeys();
    fn CryptoKeys();
    fn TypeName();
    fn Type();
    fn MaintenanceMode();
    fn SetMaintenanceMode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResourcesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusScsiAddressImpl: Sized + IDispatchImpl {
    fn PortNumber();
    fn PathId();
    fn TargetId();
    fn Lun();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusVersionImpl: Sized + IDispatchImpl {
    fn Name();
    fn MajorVersion();
    fn MinorVersion();
    fn BuildNumber();
    fn VendorId();
    fn CSDVersion();
    fn ClusterHighestVersion();
    fn ClusterLowestVersion();
    fn Flags();
    fn MixedVersion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusterImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Handle();
    fn Open();
    fn Name();
    fn SetName();
    fn Version();
    fn SetQuorumResource();
    fn QuorumResource();
    fn QuorumLogSize();
    fn SetQuorumLogSize();
    fn QuorumPath();
    fn SetQuorumPath();
    fn Nodes();
    fn ResourceGroups();
    fn Resources();
    fn ResourceTypes();
    fn Networks();
    fn NetInterfaces();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusterNamesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn DomainName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISDomainNamesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
pub trait IWCContextMenuCallbackImpl: Sized {
    fn AddExtensionMenuItem();
}
pub trait IWCPropertySheetCallbackImpl: Sized {
    fn AddPropertySheetPage();
}
pub trait IWCWizard97CallbackImpl: Sized {
    fn AddWizard97Page();
    fn EnableNext();
}
pub trait IWCWizardCallbackImpl: Sized {
    fn AddWizardPage();
    fn EnableNext();
}
pub trait IWEExtendContextMenuImpl: Sized {
    fn AddContextMenuItems();
}
pub trait IWEExtendPropertySheetImpl: Sized {
    fn CreatePropertySheetPages();
}
pub trait IWEExtendWizardImpl: Sized {
    fn CreateWizardPages();
}
pub trait IWEExtendWizard97Impl: Sized {
    fn CreateWizard97Pages();
}
pub trait IWEInvokeCommandImpl: Sized {
    fn InvokeCommand();
}
