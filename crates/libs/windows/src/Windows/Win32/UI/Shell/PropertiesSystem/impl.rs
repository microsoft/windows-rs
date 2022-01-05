pub trait ICreateObjectImpl: Sized {
    fn CreateObject();
}
pub trait IDelayedPropertyStoreFactoryImpl: Sized + IPropertyStoreFactoryImpl {
    fn GetDelayedPropertyStore();
}
pub trait IInitializeWithFileImpl: Sized {
    fn Initialize();
}
pub trait IInitializeWithStreamImpl: Sized {
    fn Initialize();
}
pub trait INamedPropertyStoreImpl: Sized {
    fn GetNamedValue();
    fn SetNamedValue();
    fn GetNameCount();
    fn GetNameAt();
}
pub trait IObjectWithPropertyKeyImpl: Sized {
    fn SetPropertyKey();
    fn GetPropertyKey();
}
pub trait IPersistSerializedPropStorageImpl: Sized {
    fn SetFlags();
    fn SetPropertyStorage();
    fn GetPropertyStorage();
}
pub trait IPersistSerializedPropStorage2Impl: Sized + IPersistSerializedPropStorageImpl {
    fn GetPropertyStorageSize();
    fn GetPropertyStorageBuffer();
}
pub trait IPropertyChangeImpl: Sized + IObjectWithPropertyKeyImpl {
    fn ApplyToPropVariant();
}
pub trait IPropertyChangeArrayImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn Append();
    fn AppendOrReplace();
    fn RemoveAt();
    fn IsKeyInArray();
}
pub trait IPropertyDescriptionImpl: Sized {
    fn GetPropertyKey();
    fn GetCanonicalName();
    fn GetPropertyType();
    fn GetDisplayName();
    fn GetEditInvitation();
    fn GetTypeFlags();
    fn GetViewFlags();
    fn GetDefaultColumnWidth();
    fn GetDisplayType();
    fn GetColumnState();
    fn GetGroupingRange();
    fn GetRelativeDescriptionType();
    fn GetRelativeDescription();
    fn GetSortDescription();
    fn GetSortDescriptionLabel();
    fn GetAggregationType();
    fn GetConditionType();
    fn GetEnumTypeList();
    fn CoerceToCanonicalValue();
    fn FormatForDisplay();
    fn IsValueCanonical();
}
pub trait IPropertyDescription2Impl: Sized + IPropertyDescriptionImpl {
    fn GetImageReferenceForValue();
}
pub trait IPropertyDescriptionAliasInfoImpl: Sized + IPropertyDescriptionImpl {
    fn GetSortByAlias();
    fn GetAdditionalSortByAliases();
}
pub trait IPropertyDescriptionListImpl: Sized {
    fn GetCount();
    fn GetAt();
}
pub trait IPropertyDescriptionRelatedPropertyInfoImpl: Sized + IPropertyDescriptionImpl {
    fn GetRelatedProperty();
}
pub trait IPropertyDescriptionSearchInfoImpl: Sized + IPropertyDescriptionImpl {
    fn GetSearchInfoFlags();
    fn GetColumnIndexType();
    fn GetProjectionString();
    fn GetMaxSize();
}
pub trait IPropertyEnumTypeImpl: Sized {
    fn GetEnumType();
    fn GetValue();
    fn GetRangeMinValue();
    fn GetRangeSetValue();
    fn GetDisplayText();
}
pub trait IPropertyEnumType2Impl: Sized + IPropertyEnumTypeImpl {
    fn GetImageReference();
}
pub trait IPropertyEnumTypeListImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetConditionAt();
    fn FindMatchingIndex();
}
pub trait IPropertyStoreImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetValue();
    fn SetValue();
    fn Commit();
}
pub trait IPropertyStoreCacheImpl: Sized + IPropertyStoreImpl {
    fn GetState();
    fn GetValueAndState();
    fn SetState();
    fn SetValueAndState();
}
pub trait IPropertyStoreCapabilitiesImpl: Sized {
    fn IsPropertyWritable();
}
pub trait IPropertyStoreFactoryImpl: Sized {
    fn GetPropertyStore();
    fn GetPropertyStoreForKeys();
}
pub trait IPropertySystemImpl: Sized {
    fn GetPropertyDescription();
    fn GetPropertyDescriptionByName();
    fn GetPropertyDescriptionListFromString();
    fn EnumeratePropertyDescriptions();
    fn FormatForDisplay();
    fn FormatForDisplayAlloc();
    fn RegisterPropertySchema();
    fn UnregisterPropertySchema();
    fn RefreshPropertySchema();
}
pub trait IPropertySystemChangeNotifyImpl: Sized {
    fn SchemaRefreshed();
}
pub trait IPropertyUIImpl: Sized {
    fn ParsePropertyName();
    fn GetCannonicalName();
    fn GetDisplayName();
    fn GetPropertyDescription();
    fn GetDefaultWidth();
    fn GetFlags();
    fn FormatForDisplay();
    fn GetHelpInfo();
}
