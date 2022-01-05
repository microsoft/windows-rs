pub trait IContactImpl: Sized {
    fn GetContactID();
    fn GetPath();
    fn CommitChanges();
}
pub trait IContactAggregationAggregateImpl: Sized {
    fn Save();
    fn GetComponentItems();
    fn Link();
    fn Groups();
    fn AntiLink();
    fn SetAntiLink();
    fn FavoriteOrder();
    fn SetFavoriteOrder();
    fn Id();
}
pub trait IContactAggregationAggregateCollectionImpl: Sized {
    fn FindFirst();
    fn FindFirstByAntiLinkId();
    fn FindNext();
    fn Count();
}
pub trait IContactAggregationContactImpl: Sized {
    fn Delete();
    fn Save();
    fn MoveToAggregate();
    fn Unlink();
    fn AccountId();
    fn SetAccountId();
    fn AggregateId();
    fn Id();
    fn IsMe();
    fn IsExternal();
    fn NetworkSourceId();
    fn SetNetworkSourceId();
    fn NetworkSourceIdString();
    fn SetNetworkSourceIdString();
    fn RemoteObjectId();
    fn SetRemoteObjectId();
    fn SyncIdentityHash();
    fn SetSyncIdentityHash();
}
pub trait IContactAggregationContactCollectionImpl: Sized {
    fn FindFirst();
    fn FindNext();
    fn FindFirstByIdentityHash();
    fn Count();
    fn FindFirstByRemoteId();
}
pub trait IContactAggregationGroupImpl: Sized {
    fn Delete();
    fn Save();
    fn Add();
    fn Remove();
    fn Members();
    fn GlobalObjectId();
    fn SetGlobalObjectId();
    fn Id();
    fn Name();
    fn SetName();
}
pub trait IContactAggregationGroupCollectionImpl: Sized {
    fn FindFirst();
    fn FindFirstByGlobalObjectId();
    fn FindNext();
    fn Count();
}
pub trait IContactAggregationLinkImpl: Sized {
    fn Delete();
    fn Save();
    fn AccountId();
    fn SetAccountId();
    fn Id();
    fn IsLinkResolved();
    fn SetIsLinkResolved();
    fn NetworkSourceIdString();
    fn SetNetworkSourceIdString();
    fn RemoteObjectId();
    fn SetRemoteObjectId();
    fn ServerPerson();
    fn SetServerPerson();
    fn ServerPersonBaseline();
    fn SetServerPersonBaseline();
    fn SyncIdentityHash();
    fn SetSyncIdentityHash();
}
pub trait IContactAggregationLinkCollectionImpl: Sized {
    fn FindFirst();
    fn FindFirstByRemoteId();
    fn FindNext();
    fn Count();
}
pub trait IContactAggregationManagerImpl: Sized {
    fn GetVersionInfo();
    fn CreateOrOpenGroup();
    fn CreateExternalContact();
    fn CreateServerPerson();
    fn CreateServerContactLink();
    fn Flush();
    fn OpenAggregateContact();
    fn OpenContact();
    fn OpenServerContactLink();
    fn OpenServerPerson();
    fn Contacts();
    fn AggregateContacts();
    fn Groups();
    fn ServerPersons();
    fn ServerContactLinks();
}
pub trait IContactAggregationServerPersonImpl: Sized {
    fn Delete();
    fn Save();
    fn AggregateId();
    fn SetAggregateId();
    fn AntiLink();
    fn SetAntiLink();
    fn AntiLinkBaseline();
    fn SetAntiLinkBaseline();
    fn FavoriteOrder();
    fn SetFavoriteOrder();
    fn FavoriteOrderBaseline();
    fn SetFavoriteOrderBaseline();
    fn Groups();
    fn SetGroups();
    fn GroupsBaseline();
    fn SetGroupsBaseline();
    fn Id();
    fn IsTombstone();
    fn SetIsTombstone();
    fn LinkedAggregateId();
    fn SetLinkedAggregateId();
    fn ObjectId();
    fn SetObjectId();
}
pub trait IContactAggregationServerPersonCollectionImpl: Sized {
    fn FindFirst();
    fn FindFirstByServerId();
    fn FindFirstByAggregateId();
    fn FindFirstByLinkedAggregateId();
    fn FindNext();
    fn Count();
}
pub trait IContactCollectionImpl: Sized {
    fn Reset();
    fn Next();
    fn GetCurrent();
}
pub trait IContactManagerImpl: Sized {
    fn Initialize();
    fn Load();
    fn MergeContactIDs();
    fn GetMeContact();
    fn SetMeContact();
    fn GetContactCollection();
}
pub trait IContactPropertiesImpl: Sized {
    fn GetString();
    fn GetDate();
    fn GetBinary();
    fn GetLabels();
    fn SetString();
    fn SetDate();
    fn SetBinary();
    fn SetLabels();
    fn CreateArrayNode();
    fn DeleteProperty();
    fn DeleteArrayNode();
    fn DeleteLabels();
    fn GetPropertyCollection();
}
pub trait IContactPropertyCollectionImpl: Sized {
    fn Reset();
    fn Next();
    fn GetPropertyName();
    fn GetPropertyType();
    fn GetPropertyVersion();
    fn GetPropertyModificationDate();
    fn GetPropertyArrayElementID();
}
