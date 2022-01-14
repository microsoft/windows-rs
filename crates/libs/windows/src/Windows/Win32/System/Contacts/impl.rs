#[cfg(feature = "Win32_Foundation")]
pub trait IContact_Impl: Sized {
    fn GetContactID(&mut self, pszcontactid: super::super::Foundation::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetPath(&mut self, pszpath: super::super::Foundation::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows::core::Result<()>;
    fn CommitChanges(&mut self, dwcommitflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContact_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContact_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContact_Vtbl {
        unsafe extern "system" fn GetContactID<Impl: IContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcontactid: super::super::Foundation::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContactID(::core::mem::transmute_copy(&pszcontactid), ::core::mem::transmute_copy(&cchcontactid), ::core::mem::transmute_copy(&pdwcchcontactidrequired)).into()
        }
        unsafe extern "system" fn GetPath<Impl: IContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPath(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchpath), ::core::mem::transmute_copy(&pdwcchpathrequired)).into()
        }
        unsafe extern "system" fn CommitChanges<Impl: IContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitChanges(::core::mem::transmute_copy(&dwcommitflags)).into()
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
pub trait IContactAggregationAggregate_Impl: Sized {
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn GetComponentItems(&mut self) -> ::windows::core::Result<IContactAggregationContactCollection>;
    fn Link(&mut self, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Groups(&mut self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationGroupCollection>;
    fn AntiLink(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAntiLink(&mut self, pantilink: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FavoriteOrder(&mut self) -> ::windows::core::Result<u32>;
    fn SetFavoriteOrder(&mut self, favoriteorder: u32) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationAggregate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationAggregate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationAggregate_Vtbl {
        unsafe extern "system" fn Save<Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn GetComponentItems<Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomponentitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentItems() {
                ::core::result::Result::Ok(ok__) => {
                    *pcomponentitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Link(::core::mem::transmute_copy(&paggregateid)).into()
        }
        unsafe extern "system" fn Groups<Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntiLink<Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppantilink: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntiLink() {
                ::core::result::Result::Ok(ok__) => {
                    *ppantilink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiLink<Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilink: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAntiLink(::core::mem::transmute_copy(&pantilink)).into()
        }
        unsafe extern "system" fn FavoriteOrder<Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FavoriteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *pfavoriteorder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFavoriteOrder<Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFavoriteOrder(::core::mem::transmute_copy(&favoriteorder)).into()
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IContactAggregationAggregateCollection_Impl: Sized {
    fn FindFirst(&mut self) -> ::windows::core::Result<IContactAggregationAggregate>;
    fn FindFirstByAntiLinkId(&mut self, pantilinkid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContactAggregationAggregate>;
    fn FindNext(&mut self) -> ::windows::core::Result<IContactAggregationAggregate>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationAggregateCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationAggregateCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationAggregateCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaggregate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByAntiLinkId<Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilinkid: super::super::Foundation::PWSTR, ppaggregate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstByAntiLinkId(::core::mem::transmute_copy(&pantilinkid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppaggregate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaggregate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IContactAggregationContact_Impl: Sized {
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn MoveToAggregate(&mut self, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Unlink(&mut self) -> ::windows::core::Result<()>;
    fn AccountId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAccountId(&mut self, paccountid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AggregateId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsMe(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsExternal(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn NetworkSourceId(&mut self) -> ::windows::core::Result<u32>;
    fn SetNetworkSourceId(&mut self, networksourceid: u32) -> ::windows::core::Result<()>;
    fn NetworkSourceIdString(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetNetworkSourceIdString(&mut self, pnetworksourceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RemoteObjectId(&mut self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(&mut self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
    fn SyncIdentityHash(&mut self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(&mut self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationContact_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationContact_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationContact_Vtbl {
        unsafe extern "system" fn Delete<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Save<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn MoveToAggregate<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveToAggregate(::core::mem::transmute_copy(&paggregateid)).into()
        }
        unsafe extern "system" fn Unlink<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unlink().into()
        }
        unsafe extern "system" fn AccountId<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccountid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaccountid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountId<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccountid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountId(::core::mem::transmute_copy(&paccountid)).into()
        }
        unsafe extern "system" fn AggregateId<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregateid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AggregateId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaggregateid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMe<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisme: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMe() {
                ::core::result::Result::Ok(ok__) => {
                    *pisme = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsExternal<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisexternal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsExternal() {
                ::core::result::Result::Ok(ok__) => {
                    *pisexternal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkSourceId<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksourceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkSourceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pnetworksourceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSourceId<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networksourceid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkSourceId(::core::mem::transmute_copy(&networksourceid)).into()
        }
        unsafe extern "system" fn NetworkSourceIdString<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkSourceIdString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetworksourceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksourceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkSourceIdString(::core::mem::transmute_copy(&pnetworksourceid)).into()
        }
        unsafe extern "system" fn RemoteObjectId<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppremoteobjectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteObjectId<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteObjectId(::core::mem::transmute_copy(&premoteobjectid)).into()
        }
        unsafe extern "system" fn SyncIdentityHash<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncIdentityHash() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncidentityhash = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncIdentityHash<Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncIdentityHash(::core::mem::transmute_copy(&psyncidentityhash)).into()
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
pub trait IContactAggregationContactCollection_Impl: Sized {
    fn FindFirst(&mut self) -> ::windows::core::Result<IContactAggregationContact>;
    fn FindNext(&mut self) -> ::windows::core::Result<IContactAggregationContact>;
    fn FindFirstByIdentityHash(&mut self, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<IContactAggregationContact>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn FindFirstByRemoteId(&mut self, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<IContactAggregationContact>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationContactCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationContactCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationContactCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByIdentityHash<Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstByIdentityHash(::core::mem::transmute_copy(&psourcetype), ::core::mem::transmute_copy(&paccountid), ::core::mem::transmute_copy(&pidentityhash)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByRemoteId<Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstByRemoteId(::core::mem::transmute_copy(&psourcetype), ::core::mem::transmute_copy(&paccountid), ::core::mem::transmute_copy(&premoteobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IContactAggregationGroup_Impl: Sized {
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn Add(&mut self, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Remove(&mut self, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Members(&mut self) -> ::windows::core::Result<IContactAggregationAggregateCollection>;
    fn GlobalObjectId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetGlobalObjectId(&mut self, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetName(&mut self, pname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationGroup_Vtbl {
        unsafe extern "system" fn Delete<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Save<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn Add<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&paggregateid)).into()
        }
        unsafe extern "system" fn Remove<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&paggregateid)).into()
        }
        unsafe extern "system" fn Members<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregatecontactcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Members() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaggregatecontactcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalObjectId<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *pglobalobjectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalObjectId<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlobalObjectId(::core::mem::transmute_copy(&pglobalobjectid)).into()
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&pname)).into()
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
pub trait IContactAggregationGroupCollection_Impl: Sized {
    fn FindFirst(&mut self) -> ::windows::core::Result<IContactAggregationGroup>;
    fn FindFirstByGlobalObjectId(&mut self, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::Result<IContactAggregationGroup>;
    fn FindNext(&mut self) -> ::windows::core::Result<IContactAggregationGroup>;
    fn Count(&mut self) -> ::windows::core::Result<u32>;
}
impl IContactAggregationGroupCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationGroupCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationGroupCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByGlobalObjectId<Impl: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstByGlobalObjectId(::core::mem::transmute_copy(&pglobalobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IContactAggregationLink_Impl: Sized {
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn AccountId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAccountId(&mut self, paccountid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsLinkResolved(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsLinkResolved(&mut self, islinkresolved: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn NetworkSourceIdString(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetNetworkSourceIdString(&mut self, pnetworksourceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RemoteObjectId(&mut self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(&mut self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
    fn ServerPerson(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetServerPerson(&mut self, pserverpersonid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ServerPersonBaseline(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetServerPersonBaseline(&mut self, pserverpersonid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SyncIdentityHash(&mut self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(&mut self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationLink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationLink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationLink_Vtbl {
        unsafe extern "system" fn Delete<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Save<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn AccountId<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccountid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaccountid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountId<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccountid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountId(::core::mem::transmute_copy(&paccountid)).into()
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLinkResolved<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislinkresolved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLinkResolved() {
                ::core::result::Result::Ok(ok__) => {
                    *pislinkresolved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLinkResolved<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islinkresolved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLinkResolved(::core::mem::transmute_copy(&islinkresolved)).into()
        }
        unsafe extern "system" fn NetworkSourceIdString<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkSourceIdString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetworksourceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksourceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkSourceIdString(::core::mem::transmute_copy(&pnetworksourceid)).into()
        }
        unsafe extern "system" fn RemoteObjectId<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppremoteobjectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteObjectId<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteObjectId(::core::mem::transmute_copy(&premoteobjectid)).into()
        }
        unsafe extern "system" fn ServerPerson<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerPerson() {
                ::core::result::Result::Ok(ok__) => {
                    *ppserverpersonid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerPerson<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserverpersonid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerPerson(::core::mem::transmute_copy(&pserverpersonid)).into()
        }
        unsafe extern "system" fn ServerPersonBaseline<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerPersonBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    *ppserverpersonid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerPersonBaseline<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserverpersonid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerPersonBaseline(::core::mem::transmute_copy(&pserverpersonid)).into()
        }
        unsafe extern "system" fn SyncIdentityHash<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncIdentityHash() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncidentityhash = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncIdentityHash<Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncIdentityHash(::core::mem::transmute_copy(&psyncidentityhash)).into()
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
pub trait IContactAggregationLinkCollection_Impl: Sized {
    fn FindFirst(&mut self) -> ::windows::core::Result<IContactAggregationLink>;
    fn FindFirstByRemoteId(&mut self, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<IContactAggregationLink>;
    fn FindNext(&mut self) -> ::windows::core::Result<IContactAggregationLink>;
    fn Count(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationLinkCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationLinkCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationLinkCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *ppservercontactlink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByRemoteId<Impl: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB, ppservercontactlink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstByRemoteId(::core::mem::transmute_copy(&psourcetype), ::core::mem::transmute_copy(&paccountid), ::core::mem::transmute_copy(&premoteid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservercontactlink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppservercontactlink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IContactAggregationManager_Impl: Sized {
    fn GetVersionInfo(&mut self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::Result<()>;
    fn CreateOrOpenGroup(&mut self, pgroupname: super::super::Foundation::PWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut ::core::option::Option<IContactAggregationGroup>) -> ::windows::core::Result<()>;
    fn CreateExternalContact(&mut self) -> ::windows::core::Result<IContactAggregationContact>;
    fn CreateServerPerson(&mut self) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn CreateServerContactLink(&mut self) -> ::windows::core::Result<IContactAggregationLink>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
    fn OpenAggregateContact(&mut self, pitemid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContactAggregationAggregate>;
    fn OpenContact(&mut self, pitemid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContactAggregationContact>;
    fn OpenServerContactLink(&mut self, pitemid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContactAggregationLink>;
    fn OpenServerPerson(&mut self, pitemid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn Contacts(&mut self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationContactCollection>;
    fn AggregateContacts(&mut self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationAggregateCollection>;
    fn Groups(&mut self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationGroupCollection>;
    fn ServerPersons(&mut self) -> ::windows::core::Result<IContactAggregationServerPersonCollection>;
    fn ServerContactLinks(&mut self, ppersonitemid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContactAggregationLinkCollection>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationManager_Vtbl {
        unsafe extern "system" fn GetVersionInfo<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVersionInfo(::core::mem::transmute_copy(&plmajorversion), ::core::mem::transmute_copy(&plminorversion)).into()
        }
        unsafe extern "system" fn CreateOrOpenGroup<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroupname: super::super::Foundation::PWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateOrOpenGroup(::core::mem::transmute_copy(&pgroupname), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&pcreatedgroup), ::core::mem::transmute_copy(&ppgroup)).into()
        }
        unsafe extern "system" fn CreateExternalContact<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateExternalContact() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServerPerson<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateServerPerson() {
                ::core::result::Result::Ok(ok__) => {
                    *ppserverperson = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServerContactLink<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateServerContactLink() {
                ::core::result::Result::Ok(ok__) => {
                    *ppservercontactlink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn OpenAggregateContact<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAggregateContact(::core::mem::transmute_copy(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenContact<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenContact(::core::mem::transmute_copy(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenServerContactLink<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenServerContactLink(::core::mem::transmute_copy(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenServerPerson<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenServerPerson(::core::mem::transmute_copy(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contacts<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contacts(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AggregateContacts<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppaggregates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AggregateContacts(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppaggregates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerPersons<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverpersoncollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerPersons() {
                ::core::result::Result::Ok(ok__) => {
                    *ppserverpersoncollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerContactLinks<Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppersonitemid: super::super::Foundation::PWSTR, ppservercontactlinkcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerContactLinks(::core::mem::transmute_copy(&ppersonitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservercontactlinkcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IContactAggregationServerPerson_Impl: Sized {
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn AggregateId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAggregateId(&mut self, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AntiLink(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAntiLink(&mut self, pantilink: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AntiLinkBaseline(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAntiLinkBaseline(&mut self, pantilink: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FavoriteOrder(&mut self) -> ::windows::core::Result<u32>;
    fn SetFavoriteOrder(&mut self, favoriteorder: u32) -> ::windows::core::Result<()>;
    fn FavoriteOrderBaseline(&mut self) -> ::windows::core::Result<u32>;
    fn SetFavoriteOrderBaseline(&mut self, favoriteorder: u32) -> ::windows::core::Result<()>;
    fn Groups(&mut self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroups(&mut self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
    fn GroupsBaseline(&mut self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroupsBaseline(&mut self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsTombstone(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsTombstone(&mut self, istombstone: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn LinkedAggregateId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetLinkedAggregateId(&mut self, plinkedaggregateid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ObjectId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetObjectId(&mut self, pobjectid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationServerPerson_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationServerPerson_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationServerPerson_Vtbl {
        unsafe extern "system" fn Delete<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Save<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn AggregateId<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregateid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AggregateId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaggregateid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAggregateId<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAggregateId(::core::mem::transmute_copy(&paggregateid)).into()
        }
        unsafe extern "system" fn AntiLink<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppantilink: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntiLink() {
                ::core::result::Result::Ok(ok__) => {
                    *ppantilink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiLink<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilink: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAntiLink(::core::mem::transmute_copy(&pantilink)).into()
        }
        unsafe extern "system" fn AntiLinkBaseline<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppantilink: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntiLinkBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    *ppantilink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiLinkBaseline<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilink: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAntiLinkBaseline(::core::mem::transmute_copy(&pantilink)).into()
        }
        unsafe extern "system" fn FavoriteOrder<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FavoriteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *pfavoriteorder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFavoriteOrder<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFavoriteOrder(::core::mem::transmute_copy(&favoriteorder)).into()
        }
        unsafe extern "system" fn FavoriteOrderBaseline<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FavoriteOrderBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    *pfavoriteorder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFavoriteOrderBaseline<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFavoriteOrderBaseline(::core::mem::transmute_copy(&favoriteorder)).into()
        }
        unsafe extern "system" fn Groups<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups() {
                ::core::result::Result::Ok(ok__) => {
                    *pgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroups<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroups(::core::mem::transmute_copy(&pgroups)).into()
        }
        unsafe extern "system" fn GroupsBaseline<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupsBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupsBaseline<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupsBaseline(::core::mem::transmute_copy(&pgroups)).into()
        }
        unsafe extern "system" fn Id<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *ppid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTombstone<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistombstone: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTombstone() {
                ::core::result::Result::Ok(ok__) => {
                    *pistombstone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTombstone<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istombstone: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTombstone(::core::mem::transmute_copy(&istombstone)).into()
        }
        unsafe extern "system" fn LinkedAggregateId<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkedaggregateid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinkedAggregateId() {
                ::core::result::Result::Ok(ok__) => {
                    *pplinkedaggregateid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkedAggregateId<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plinkedaggregateid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinkedAggregateId(::core::mem::transmute_copy(&plinkedaggregateid)).into()
        }
        unsafe extern "system" fn ObjectId<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectId<Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectId(::core::mem::transmute_copy(&pobjectid)).into()
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
pub trait IContactAggregationServerPersonCollection_Impl: Sized {
    fn FindFirst(&mut self) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn FindFirstByServerId(&mut self, pserverid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn FindFirstByAggregateId(&mut self, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn FindFirstByLinkedAggregateId(&mut self, paggregateid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn FindNext(&mut self) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn Count(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationServerPersonCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAggregationServerPersonCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAggregationServerPersonCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *ppserverperson = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByServerId<Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserverid: super::super::Foundation::PWSTR, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstByServerId(::core::mem::transmute_copy(&pserverid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppserverperson = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByAggregateId<Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstByAggregateId(::core::mem::transmute_copy(&paggregateid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppserverperson = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByLinkedAggregateId<Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: super::super::Foundation::PWSTR, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstByLinkedAggregateId(::core::mem::transmute_copy(&paggregateid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppserverperson = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppserverperson = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IContactCollection_Impl: Sized {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<()>;
    fn GetCurrent(&mut self) -> ::windows::core::Result<IContact>;
}
impl IContactCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactCollection_Vtbl {
        unsafe extern "system" fn Reset<Impl: IContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next().into()
        }
        unsafe extern "system" fn GetCurrent<Impl: IContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IContactManager_Impl: Sized {
    fn Initialize(&mut self, pszappname: super::super::Foundation::PWSTR, pszappversion: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Load(&mut self, pszcontactid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IContact>;
    fn MergeContactIDs(&mut self, psznewcontactid: super::super::Foundation::PWSTR, pszoldcontactid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetMeContact(&mut self) -> ::windows::core::Result<IContact>;
    fn SetMeContact(&mut self, pmecontact: &::core::option::Option<IContact>) -> ::windows::core::Result<()>;
    fn GetContactCollection(&mut self) -> ::windows::core::Result<IContactCollection>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactManager_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszappname: super::super::Foundation::PWSTR, pszappversion: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pszappname), ::core::mem::transmute_copy(&pszappversion)).into()
        }
        unsafe extern "system" fn Load<Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcontactid: super::super::Foundation::PWSTR, ppcontact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(::core::mem::transmute_copy(&pszcontactid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MergeContactIDs<Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psznewcontactid: super::super::Foundation::PWSTR, pszoldcontactid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MergeContactIDs(::core::mem::transmute_copy(&psznewcontactid), ::core::mem::transmute_copy(&pszoldcontactid)).into()
        }
        unsafe extern "system" fn GetMeContact<Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmecontact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMeContact() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmecontact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeContact<Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmecontact: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMeContact(::core::mem::transmute(&pmecontact)).into()
        }
        unsafe extern "system" fn GetContactCollection<Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontactcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontactcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IContactProperties_Impl: Sized {
    fn GetString(&mut self, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszvalue: super::super::Foundation::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetDate(&mut self, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetBinary(&mut self, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszcontenttype: super::super::Foundation::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut ::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()>;
    fn GetLabels(&mut self, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32, pszlabels: super::super::Foundation::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows::core::Result<()>;
    fn SetString(&mut self, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetDate(&mut self, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, ftdatetime: &super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SetBinary(&mut self, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszcontenttype: super::super::Foundation::PWSTR, pstream: &::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()>;
    fn SetLabels(&mut self, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CreateArrayNode(&mut self, pszarrayname: super::super::Foundation::PWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: super::super::Foundation::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows::core::Result<()>;
    fn DeleteProperty(&mut self, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn DeleteArrayNode(&mut self, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn DeleteLabels(&mut self, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetPropertyCollection(&mut self, pppropertycollection: *mut ::core::option::Option<IContactPropertyCollection>, dwflags: u32, pszmultivaluename: super::super::Foundation::PWSTR, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IContactProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactProperties_Vtbl {
        unsafe extern "system" fn GetString<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszvalue: super::super::Foundation::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetString(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszvalue), ::core::mem::transmute_copy(&cchvalue), ::core::mem::transmute_copy(&pdwcchpropertyvaluerequired)).into()
        }
        unsafe extern "system" fn GetDate<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDate(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pftdatetime)).into()
        }
        unsafe extern "system" fn GetBinary<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszcontenttype: super::super::Foundation::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBinary(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszcontenttype), ::core::mem::transmute_copy(&cchcontenttype), ::core::mem::transmute_copy(&pdwcchcontenttyperequired), ::core::mem::transmute_copy(&ppstream)).into()
        }
        unsafe extern "system" fn GetLabels<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32, pszlabels: super::super::Foundation::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLabels(::core::mem::transmute_copy(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszlabels), ::core::mem::transmute_copy(&cchlabels), ::core::mem::transmute_copy(&pdwcchlabelsrequired)).into()
        }
        unsafe extern "system" fn SetString<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetString(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszvalue)).into()
        }
        unsafe extern "system" fn SetDate<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDate(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ftdatetime)).into()
        }
        unsafe extern "system" fn SetBinary<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszcontenttype: super::super::Foundation::PWSTR, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBinary(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszcontenttype), ::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn SetLabels<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabels(::core::mem::transmute_copy(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwlabelcount), ::core::mem::transmute_copy(&ppszlabels)).into()
        }
        unsafe extern "system" fn CreateArrayNode<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayname: super::super::Foundation::PWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: super::super::Foundation::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateArrayNode(::core::mem::transmute_copy(&pszarrayname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&fappend), ::core::mem::transmute_copy(&psznewarrayelementname), ::core::mem::transmute_copy(&cchnewarrayelementname), ::core::mem::transmute_copy(&pdwcchnewarrayelementnamerequired)).into()
        }
        unsafe extern "system" fn DeleteProperty<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteProperty(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteArrayNode<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteArrayNode(::core::mem::transmute_copy(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteLabels<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteLabels(::core::mem::transmute_copy(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetPropertyCollection<Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertycollection: *mut ::windows::core::RawPtr, dwflags: u32, pszmultivaluename: super::super::Foundation::PWSTR, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyCollection(::core::mem::transmute_copy(&pppropertycollection), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszmultivaluename), ::core::mem::transmute_copy(&dwlabelcount), ::core::mem::transmute_copy(&ppszlabels), ::core::mem::transmute_copy(&fanylabelmatches)).into()
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
pub trait IContactPropertyCollection_Impl: Sized {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<()>;
    fn GetPropertyName(&mut self, pszpropertyname: super::super::Foundation::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetPropertyType(&mut self, pdwtype: *mut u32) -> ::windows::core::Result<()>;
    fn GetPropertyVersion(&mut self, pdwversion: *mut u32) -> ::windows::core::Result<()>;
    fn GetPropertyModificationDate(&mut self, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetPropertyArrayElementID(&mut self, pszarrayelementid: super::super::Foundation::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContactPropertyCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPropertyCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPropertyCollection_Vtbl {
        unsafe extern "system" fn Reset<Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next().into()
        }
        unsafe extern "system" fn GetPropertyName<Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyName(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&cchpropertyname), ::core::mem::transmute_copy(&pdwcchpropertynamerequired)).into()
        }
        unsafe extern "system" fn GetPropertyType<Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyType(::core::mem::transmute_copy(&pdwtype)).into()
        }
        unsafe extern "system" fn GetPropertyVersion<Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyVersion(::core::mem::transmute_copy(&pdwversion)).into()
        }
        unsafe extern "system" fn GetPropertyModificationDate<Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyModificationDate(::core::mem::transmute_copy(&pftmodificationdate)).into()
        }
        unsafe extern "system" fn GetPropertyArrayElementID<Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementid: super::super::Foundation::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyArrayElementID(::core::mem::transmute_copy(&pszarrayelementid), ::core::mem::transmute_copy(&ccharrayelementid), ::core::mem::transmute_copy(&pdwccharrayelementidrequired)).into()
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
