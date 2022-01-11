#[cfg(feature = "Win32_Foundation")]
pub trait IContactImpl: Sized {
    fn GetContactID();
    fn GetPath();
    fn CommitChanges();
}
#[cfg(feature = "Win32_Foundation")]
impl IContactVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactVtbl {
        unsafe extern "system" fn GetContactID<Impl: IContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcontactid: super::super::Foundation::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPath<Impl: IContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitChanges<Impl: IContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetContactID: GetContactID::<Impl, IMPL_OFFSET>,
            GetPath: GetPath::<Impl, IMPL_OFFSET>,
            CommitChanges: CommitChanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContact as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationAggregateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationAggregateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationAggregateVtbl {
        unsafe extern "system" fn Save<Impl: IContactAggregationAggregateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetComponentItems<Impl: IContactAggregationAggregateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomponentitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Link<Impl: IContactAggregationAggregateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Groups<Impl: IContactAggregationAggregateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AntiLink<Impl: IContactAggregationAggregateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppantilink: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAntiLink<Impl: IContactAggregationAggregateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilink: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FavoriteOrder<Impl: IContactAggregationAggregateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFavoriteOrder<Impl: IContactAggregationAggregateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationAggregateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Save: Save::<Impl, IMPL_OFFSET>,
            GetComponentItems: GetComponentItems::<Impl, IMPL_OFFSET>,
            Link: Link::<Impl, IMPL_OFFSET>,
            Groups: Groups::<Impl, IMPL_OFFSET>,
            AntiLink: AntiLink::<Impl, IMPL_OFFSET>,
            SetAntiLink: SetAntiLink::<Impl, IMPL_OFFSET>,
            FavoriteOrder: FavoriteOrder::<Impl, IMPL_OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationAggregate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationAggregateCollectionImpl: Sized {
    fn FindFirst();
    fn FindFirstByAntiLinkId();
    fn FindNext();
    fn Count();
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationAggregateCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationAggregateCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationAggregateCollectionVtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationAggregateCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstByAntiLinkId<Impl: IContactAggregationAggregateCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilinkid: super::super::Foundation::PWSTR, ppaggregate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationAggregateCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationAggregateCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FindFirst: FindFirst::<Impl, IMPL_OFFSET>,
            FindFirstByAntiLinkId: FindFirstByAntiLinkId::<Impl, IMPL_OFFSET>,
            FindNext: FindNext::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationAggregateCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationContactVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationContactImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationContactVtbl {
        unsafe extern "system" fn Delete<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveToAggregate<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlink<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AccountId<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccountid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccountId<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccountid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AggregateId<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregateid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsMe<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisme: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsExternal<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisexternal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkSourceId<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksourceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkSourceId<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networksourceid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkSourceIdString<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksourceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteObjectId<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteObjectId<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncIdentityHash<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSyncIdentityHash<Impl: IContactAggregationContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            MoveToAggregate: MoveToAggregate::<Impl, IMPL_OFFSET>,
            Unlink: Unlink::<Impl, IMPL_OFFSET>,
            AccountId: AccountId::<Impl, IMPL_OFFSET>,
            SetAccountId: SetAccountId::<Impl, IMPL_OFFSET>,
            AggregateId: AggregateId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            IsMe: IsMe::<Impl, IMPL_OFFSET>,
            IsExternal: IsExternal::<Impl, IMPL_OFFSET>,
            NetworkSourceId: NetworkSourceId::<Impl, IMPL_OFFSET>,
            SetNetworkSourceId: SetNetworkSourceId::<Impl, IMPL_OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Impl, IMPL_OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Impl, IMPL_OFFSET>,
            RemoteObjectId: RemoteObjectId::<Impl, IMPL_OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Impl, IMPL_OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Impl, IMPL_OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationContact as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationContactCollectionImpl: Sized {
    fn FindFirst();
    fn FindNext();
    fn FindFirstByIdentityHash();
    fn Count();
    fn FindFirstByRemoteId();
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationContactCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationContactCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationContactCollectionVtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationContactCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationContactCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstByIdentityHash<Impl: IContactAggregationContactCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationContactCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstByRemoteId<Impl: IContactAggregationContactCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FindFirst: FindFirst::<Impl, IMPL_OFFSET>,
            FindNext: FindNext::<Impl, IMPL_OFFSET>,
            FindFirstByIdentityHash: FindFirstByIdentityHash::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationContactCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationGroupVtbl {
        unsafe extern "system" fn Delete<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Members<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregatecontactcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GlobalObjectId<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGlobalObjectId<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IContactAggregationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Members: Members::<Impl, IMPL_OFFSET>,
            GlobalObjectId: GlobalObjectId::<Impl, IMPL_OFFSET>,
            SetGlobalObjectId: SetGlobalObjectId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationGroup as ::windows::core::Interface>::IID
    }
}
pub trait IContactAggregationGroupCollectionImpl: Sized {
    fn FindFirst();
    fn FindFirstByGlobalObjectId();
    fn FindNext();
    fn Count();
}
impl IContactAggregationGroupCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationGroupCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationGroupCollectionVtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationGroupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstByGlobalObjectId<Impl: IContactAggregationGroupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationGroupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationGroupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FindFirst: FindFirst::<Impl, IMPL_OFFSET>,
            FindFirstByGlobalObjectId: FindFirstByGlobalObjectId::<Impl, IMPL_OFFSET>,
            FindNext: FindNext::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationGroupCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationLinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationLinkVtbl {
        unsafe extern "system" fn Delete<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AccountId<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccountid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccountId<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccountid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLinkResolved<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislinkresolved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsLinkResolved<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islinkresolved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkSourceIdString<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksourceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteObjectId<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteObjectId<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerPerson<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerPerson<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserverpersonid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerPersonBaseline<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerPersonBaseline<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserverpersonid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncIdentityHash<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSyncIdentityHash<Impl: IContactAggregationLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            AccountId: AccountId::<Impl, IMPL_OFFSET>,
            SetAccountId: SetAccountId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            IsLinkResolved: IsLinkResolved::<Impl, IMPL_OFFSET>,
            SetIsLinkResolved: SetIsLinkResolved::<Impl, IMPL_OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Impl, IMPL_OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Impl, IMPL_OFFSET>,
            RemoteObjectId: RemoteObjectId::<Impl, IMPL_OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Impl, IMPL_OFFSET>,
            ServerPerson: ServerPerson::<Impl, IMPL_OFFSET>,
            SetServerPerson: SetServerPerson::<Impl, IMPL_OFFSET>,
            ServerPersonBaseline: ServerPersonBaseline::<Impl, IMPL_OFFSET>,
            SetServerPersonBaseline: SetServerPersonBaseline::<Impl, IMPL_OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Impl, IMPL_OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationLink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationLinkCollectionImpl: Sized {
    fn FindFirst();
    fn FindFirstByRemoteId();
    fn FindNext();
    fn Count();
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationLinkCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationLinkCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationLinkCollectionVtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationLinkCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstByRemoteId<Impl: IContactAggregationLinkCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB, ppservercontactlink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationLinkCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationLinkCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FindFirst: FindFirst::<Impl, IMPL_OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Impl, IMPL_OFFSET>,
            FindNext: FindNext::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationLinkCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationManagerVtbl {
        unsafe extern "system" fn GetVersionInfo<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateOrOpenGroup<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroupname: super::super::Foundation::PWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateExternalContact<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateServerPerson<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateServerContactLink<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenAggregateContact<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenContact<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenServerContactLink<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenServerPerson<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Contacts<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AggregateContacts<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppaggregates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Groups<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerPersons<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverpersoncollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerContactLinks<Impl: IContactAggregationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppersonitemid: super::super::Foundation::PWSTR, ppservercontactlinkcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetVersionInfo: GetVersionInfo::<Impl, IMPL_OFFSET>,
            CreateOrOpenGroup: CreateOrOpenGroup::<Impl, IMPL_OFFSET>,
            CreateExternalContact: CreateExternalContact::<Impl, IMPL_OFFSET>,
            CreateServerPerson: CreateServerPerson::<Impl, IMPL_OFFSET>,
            CreateServerContactLink: CreateServerContactLink::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            OpenAggregateContact: OpenAggregateContact::<Impl, IMPL_OFFSET>,
            OpenContact: OpenContact::<Impl, IMPL_OFFSET>,
            OpenServerContactLink: OpenServerContactLink::<Impl, IMPL_OFFSET>,
            OpenServerPerson: OpenServerPerson::<Impl, IMPL_OFFSET>,
            Contacts: Contacts::<Impl, IMPL_OFFSET>,
            AggregateContacts: AggregateContacts::<Impl, IMPL_OFFSET>,
            Groups: Groups::<Impl, IMPL_OFFSET>,
            ServerPersons: ServerPersons::<Impl, IMPL_OFFSET>,
            ServerContactLinks: ServerContactLinks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationServerPersonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationServerPersonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationServerPersonVtbl {
        unsafe extern "system" fn Delete<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AggregateId<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregateid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAggregateId<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AntiLink<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppantilink: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAntiLink<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilink: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AntiLinkBaseline<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppantilink: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAntiLinkBaseline<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilink: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FavoriteOrder<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFavoriteOrder<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FavoriteOrderBaseline<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFavoriteOrderBaseline<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Groups<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGroups<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GroupsBaseline<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGroupsBaseline<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTombstone<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistombstone: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsTombstone<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istombstone: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LinkedAggregateId<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkedaggregateid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLinkedAggregateId<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plinkedaggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectId<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObjectId<Impl: IContactAggregationServerPersonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            AggregateId: AggregateId::<Impl, IMPL_OFFSET>,
            SetAggregateId: SetAggregateId::<Impl, IMPL_OFFSET>,
            AntiLink: AntiLink::<Impl, IMPL_OFFSET>,
            SetAntiLink: SetAntiLink::<Impl, IMPL_OFFSET>,
            AntiLinkBaseline: AntiLinkBaseline::<Impl, IMPL_OFFSET>,
            SetAntiLinkBaseline: SetAntiLinkBaseline::<Impl, IMPL_OFFSET>,
            FavoriteOrder: FavoriteOrder::<Impl, IMPL_OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Impl, IMPL_OFFSET>,
            FavoriteOrderBaseline: FavoriteOrderBaseline::<Impl, IMPL_OFFSET>,
            SetFavoriteOrderBaseline: SetFavoriteOrderBaseline::<Impl, IMPL_OFFSET>,
            Groups: Groups::<Impl, IMPL_OFFSET>,
            SetGroups: SetGroups::<Impl, IMPL_OFFSET>,
            GroupsBaseline: GroupsBaseline::<Impl, IMPL_OFFSET>,
            SetGroupsBaseline: SetGroupsBaseline::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            IsTombstone: IsTombstone::<Impl, IMPL_OFFSET>,
            SetIsTombstone: SetIsTombstone::<Impl, IMPL_OFFSET>,
            LinkedAggregateId: LinkedAggregateId::<Impl, IMPL_OFFSET>,
            SetLinkedAggregateId: SetLinkedAggregateId::<Impl, IMPL_OFFSET>,
            ObjectId: ObjectId::<Impl, IMPL_OFFSET>,
            SetObjectId: SetObjectId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationServerPerson as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationServerPersonCollectionImpl: Sized {
    fn FindFirst();
    fn FindFirstByServerId();
    fn FindFirstByAggregateId();
    fn FindFirstByLinkedAggregateId();
    fn FindNext();
    fn Count();
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationServerPersonCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationServerPersonCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationServerPersonCollectionVtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationServerPersonCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstByServerId<Impl: IContactAggregationServerPersonCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserverid: super::super::Foundation::PWSTR, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstByAggregateId<Impl: IContactAggregationServerPersonCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstByLinkedAggregateId<Impl: IContactAggregationServerPersonCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationServerPersonCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationServerPersonCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FindFirst: FindFirst::<Impl, IMPL_OFFSET>,
            FindFirstByServerId: FindFirstByServerId::<Impl, IMPL_OFFSET>,
            FindFirstByAggregateId: FindFirstByAggregateId::<Impl, IMPL_OFFSET>,
            FindFirstByLinkedAggregateId: FindFirstByLinkedAggregateId::<Impl, IMPL_OFFSET>,
            FindNext: FindNext::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationServerPersonCollection as ::windows::core::Interface>::IID
    }
}
pub trait IContactCollectionImpl: Sized {
    fn Reset();
    fn Next();
    fn GetCurrent();
}
impl IContactCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactCollectionVtbl {
        unsafe extern "system" fn Reset<Impl: IContactCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IContactCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IContactCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactManagerImpl: Sized {
    fn Initialize();
    fn Load();
    fn MergeContactIDs();
    fn GetMeContact();
    fn SetMeContact();
    fn GetContactCollection();
}
#[cfg(feature = "Win32_Foundation")]
impl IContactManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactManagerVtbl {
        unsafe extern "system" fn Initialize<Impl: IContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszappname: super::super::Foundation::PWSTR, pszappversion: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcontactid: super::super::Foundation::PWSTR, ppcontact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MergeContactIDs<Impl: IContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psznewcontactid: super::super::Foundation::PWSTR, pszoldcontactid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMeContact<Impl: IContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmecontact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMeContact<Impl: IContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmecontact: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContactCollection<Impl: IContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontactcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            MergeContactIDs: MergeContactIDs::<Impl, IMPL_OFFSET>,
            GetMeContact: GetMeContact::<Impl, IMPL_OFFSET>,
            SetMeContact: SetMeContact::<Impl, IMPL_OFFSET>,
            GetContactCollection: GetContactCollection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IContactPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPropertiesVtbl {
        unsafe extern "system" fn GetString<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszvalue: super::super::Foundation::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDate<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBinary<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszcontenttype: super::super::Foundation::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLabels<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32, pszlabels: super::super::Foundation::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetString<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDate<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBinary<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszcontenttype: super::super::Foundation::PWSTR, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabels<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateArrayNode<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayname: super::super::Foundation::PWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: super::super::Foundation::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteProperty<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteArrayNode<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteLabels<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyCollection<Impl: IContactPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertycollection: *mut ::windows::core::RawPtr, dwflags: u32, pszmultivaluename: super::super::Foundation::PWSTR, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetString: GetString::<Impl, IMPL_OFFSET>,
            GetDate: GetDate::<Impl, IMPL_OFFSET>,
            GetBinary: GetBinary::<Impl, IMPL_OFFSET>,
            GetLabels: GetLabels::<Impl, IMPL_OFFSET>,
            SetString: SetString::<Impl, IMPL_OFFSET>,
            SetDate: SetDate::<Impl, IMPL_OFFSET>,
            SetBinary: SetBinary::<Impl, IMPL_OFFSET>,
            SetLabels: SetLabels::<Impl, IMPL_OFFSET>,
            CreateArrayNode: CreateArrayNode::<Impl, IMPL_OFFSET>,
            DeleteProperty: DeleteProperty::<Impl, IMPL_OFFSET>,
            DeleteArrayNode: DeleteArrayNode::<Impl, IMPL_OFFSET>,
            DeleteLabels: DeleteLabels::<Impl, IMPL_OFFSET>,
            GetPropertyCollection: GetPropertyCollection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactPropertyCollectionImpl: Sized {
    fn Reset();
    fn Next();
    fn GetPropertyName();
    fn GetPropertyType();
    fn GetPropertyVersion();
    fn GetPropertyModificationDate();
    fn GetPropertyArrayElementID();
}
#[cfg(feature = "Win32_Foundation")]
impl IContactPropertyCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPropertyCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPropertyCollectionVtbl {
        unsafe extern "system" fn Reset<Impl: IContactPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IContactPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyName<Impl: IContactPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyType<Impl: IContactPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyVersion<Impl: IContactPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyModificationDate<Impl: IContactPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyArrayElementID<Impl: IContactPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementid: super::super::Foundation::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            GetPropertyName: GetPropertyName::<Impl, IMPL_OFFSET>,
            GetPropertyType: GetPropertyType::<Impl, IMPL_OFFSET>,
            GetPropertyVersion: GetPropertyVersion::<Impl, IMPL_OFFSET>,
            GetPropertyModificationDate: GetPropertyModificationDate::<Impl, IMPL_OFFSET>,
            GetPropertyArrayElementID: GetPropertyArrayElementID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPropertyCollection as ::windows::core::Interface>::IID
    }
}
