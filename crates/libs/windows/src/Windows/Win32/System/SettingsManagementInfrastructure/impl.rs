pub trait IItemEnumeratorImpl: Sized {
    fn Current();
    fn MoveNext();
    fn Reset();
}
pub trait ISettingsContextImpl: Sized {
    fn Serialize();
    fn Deserialize();
    fn SetUserData();
    fn GetUserData();
    fn GetNamespaces();
    fn GetStoredSettings();
    fn RevertSetting();
}
pub trait ISettingsEngineImpl: Sized {
    fn GetNamespaces();
    fn GetNamespace();
    fn GetErrorDescription();
    fn CreateSettingsIdentity();
    fn GetStoreStatus();
    fn LoadStore();
    fn UnloadStore();
    fn RegisterNamespace();
    fn UnregisterNamespace();
    fn CreateTargetInfo();
    fn GetTargetInfo();
    fn SetTargetInfo();
    fn CreateSettingsContext();
    fn SetSettingsContext();
    fn ApplySettingsContext();
    fn GetSettingsContext();
}
pub trait ISettingsIdentityImpl: Sized {
    fn GetAttribute();
    fn SetAttribute();
    fn GetFlags();
    fn SetFlags();
}
pub trait ISettingsItemImpl: Sized {
    fn GetName();
    fn GetValue();
    fn SetValue();
    fn GetSettingType();
    fn GetDataType();
    fn GetValueRaw();
    fn SetValueRaw();
    fn HasChild();
    fn Children();
    fn GetChild();
    fn GetSettingByPath();
    fn CreateSettingByPath();
    fn RemoveSettingByPath();
    fn GetListKeyInformation();
    fn CreateListElement();
    fn RemoveListElement();
    fn Attributes();
    fn GetAttribute();
    fn GetPath();
    fn GetRestrictionFacets();
    fn GetRestriction();
    fn GetKeyValue();
}
pub trait ISettingsNamespaceImpl: Sized {
    fn GetIdentity();
    fn Settings();
    fn Save();
    fn GetSettingByPath();
    fn CreateSettingByPath();
    fn RemoveSettingByPath();
    fn GetAttribute();
}
pub trait ISettingsResultImpl: Sized {
    fn GetDescription();
    fn GetErrorCode();
    fn GetContextDescription();
    fn GetLine();
    fn GetColumn();
    fn GetSource();
}
pub trait ITargetInfoImpl: Sized {
    fn GetTargetMode();
    fn SetTargetMode();
    fn GetTemporaryStoreLocation();
    fn SetTemporaryStoreLocation();
    fn GetTargetID();
    fn SetTargetID();
    fn GetTargetProcessorArchitecture();
    fn SetTargetProcessorArchitecture();
    fn GetProperty();
    fn SetProperty();
    fn GetEnumerator();
    fn ExpandTarget();
    fn ExpandTargetPath();
    fn SetModulePath();
    fn LoadModule();
    fn SetWow64Context();
    fn TranslateWow64();
    fn SetSchemaHiveLocation();
    fn GetSchemaHiveLocation();
    fn SetSchemaHiveMountName();
    fn GetSchemaHiveMountName();
}
