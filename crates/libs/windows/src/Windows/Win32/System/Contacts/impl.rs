pub trait IContact_Impl: Sized {
    fn GetContactID(&self, pszcontactid: &::windows::core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetPath(&self, pszpath: &::windows::core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows::core::Result<()>;
    fn CommitChanges(&self, dwcommitflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IContact {}
impl IContact_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContact_Impl, const OFFSET: isize>() -> IContact_Vtbl {
        unsafe extern "system" fn GetContactID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcontactid: ::windows::core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContactID(::core::mem::transmute(&pszcontactid), ::core::mem::transmute_copy(&cchcontactid), ::core::mem::transmute_copy(&pdwcchcontactidrequired)).into()
        }
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPath(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&cchpath), ::core::mem::transmute_copy(&pdwcchpathrequired)).into()
        }
        unsafe extern "system" fn CommitChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitChanges(::core::mem::transmute_copy(&dwcommitflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetContactID: GetContactID::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            CommitChanges: CommitChanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContact as ::windows::core::Interface>::IID
    }
}
pub trait IContactAggregationAggregate_Impl: Sized {
    fn Save(&self) -> ::windows::core::Result<()>;
    fn GetComponentItems(&self) -> ::windows::core::Result<IContactAggregationContactCollection>;
    fn Link(&self, paggregateid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationGroupCollection>;
    fn AntiLink(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetAntiLink(&self, pantilink: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FavoriteOrder(&self) -> ::windows::core::Result<u32>;
    fn SetFavoriteOrder(&self, favoriteorder: u32) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IContactAggregationAggregate {}
impl IContactAggregationAggregate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>() -> IContactAggregationAggregate_Vtbl {
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn GetComponentItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomponentitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetComponentItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcomponentitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Link(::core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn get_Groups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Groups(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntiLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntiLink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppantilink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilink: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAntiLink(::core::mem::transmute(&pantilink)).into()
        }
        unsafe extern "system" fn FavoriteOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FavoriteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfavoriteorder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFavoriteOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFavoriteOrder(::core::mem::transmute_copy(&favoriteorder)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitemid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Save: Save::<Identity, Impl, OFFSET>,
            GetComponentItems: GetComponentItems::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            get_Groups: get_Groups::<Identity, Impl, OFFSET>,
            AntiLink: AntiLink::<Identity, Impl, OFFSET>,
            SetAntiLink: SetAntiLink::<Identity, Impl, OFFSET>,
            FavoriteOrder: FavoriteOrder::<Identity, Impl, OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationAggregate as ::windows::core::Interface>::IID
    }
}
pub trait IContactAggregationAggregateCollection_Impl: Sized {
    fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationAggregate>;
    fn FindFirstByAntiLinkId(&self, pantilinkid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContactAggregationAggregate>;
    fn FindNext(&self) -> ::windows::core::Result<IContactAggregationAggregate>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IContactAggregationAggregateCollection {}
impl IContactAggregationAggregateCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>() -> IContactAggregationAggregateCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByAntiLinkId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilinkid: ::windows::core::PCWSTR, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstByAntiLinkId(::core::mem::transmute(&pantilinkid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindFirstByAntiLinkId: FindFirstByAntiLinkId::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationAggregateCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationContact_Impl: Sized {
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn MoveToAggregate(&self, paggregateid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Unlink(&self) -> ::windows::core::Result<()>;
    fn AccountId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetAccountId(&self, paccountid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn AggregateId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn IsMe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsExternal(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn NetworkSourceId(&self) -> ::windows::core::Result<u32>;
    fn SetNetworkSourceId(&self, networksourceid: u32) -> ::windows::core::Result<()>;
    fn NetworkSourceIdString(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetNetworkSourceIdString(&self, pnetworksourceid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RemoteObjectId(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
    fn SyncIdentityHash(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IContactAggregationContact {}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationContact_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>() -> IContactAggregationContact_Vtbl {
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn MoveToAggregate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveToAggregate(::core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn Unlink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlink().into()
        }
        unsafe extern "system" fn AccountId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccountid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccountId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccountid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccountid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccountId(::core::mem::transmute(&paccountid)).into()
        }
        unsafe extern "system" fn AggregateId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregateid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AggregateId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregateid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitemid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMe<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisme: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMe() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisme, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsExternal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisexternal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsExternal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisexternal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkSourceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksourceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkSourceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnetworksourceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSourceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networksourceid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkSourceId(::core::mem::transmute_copy(&networksourceid)).into()
        }
        unsafe extern "system" fn NetworkSourceIdString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkSourceIdString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworksourceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksourceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkSourceIdString(::core::mem::transmute(&pnetworksourceid)).into()
        }
        unsafe extern "system" fn RemoteObjectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoteObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppremoteobjectid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteObjectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRemoteObjectId(::core::mem::transmute_copy(&premoteobjectid)).into()
        }
        unsafe extern "system" fn SyncIdentityHash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SyncIdentityHash() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsyncidentityhash, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncIdentityHash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSyncIdentityHash(::core::mem::transmute_copy(&psyncidentityhash)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            MoveToAggregate: MoveToAggregate::<Identity, Impl, OFFSET>,
            Unlink: Unlink::<Identity, Impl, OFFSET>,
            AccountId: AccountId::<Identity, Impl, OFFSET>,
            SetAccountId: SetAccountId::<Identity, Impl, OFFSET>,
            AggregateId: AggregateId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            IsMe: IsMe::<Identity, Impl, OFFSET>,
            IsExternal: IsExternal::<Identity, Impl, OFFSET>,
            NetworkSourceId: NetworkSourceId::<Identity, Impl, OFFSET>,
            SetNetworkSourceId: SetNetworkSourceId::<Identity, Impl, OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Identity, Impl, OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Identity, Impl, OFFSET>,
            RemoteObjectId: RemoteObjectId::<Identity, Impl, OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Identity, Impl, OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Identity, Impl, OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationContact as ::windows::core::Interface>::IID
    }
}
pub trait IContactAggregationContactCollection_Impl: Sized {
    fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationContact>;
    fn FindNext(&self) -> ::windows::core::Result<IContactAggregationContact>;
    fn FindFirstByIdentityHash(&self, psourcetype: &::windows::core::PCWSTR, paccountid: &::windows::core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<IContactAggregationContact>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn FindFirstByRemoteId(&self, psourcetype: &::windows::core::PCWSTR, paccountid: &::windows::core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<IContactAggregationContact>;
}
impl ::windows::core::RuntimeName for IContactAggregationContactCollection {}
impl IContactAggregationContactCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>() -> IContactAggregationContactCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByIdentityHash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetype: ::windows::core::PCWSTR, paccountid: ::windows::core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstByIdentityHash(::core::mem::transmute(&psourcetype), ::core::mem::transmute(&paccountid), ::core::mem::transmute_copy(&pidentityhash)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByRemoteId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetype: ::windows::core::PCWSTR, paccountid: ::windows::core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstByRemoteId(::core::mem::transmute(&psourcetype), ::core::mem::transmute(&paccountid), ::core::mem::transmute_copy(&premoteobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            FindFirstByIdentityHash: FindFirstByIdentityHash::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationContactCollection as ::windows::core::Interface>::IID
    }
}
pub trait IContactAggregationGroup_Impl: Sized {
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn Add(&self, paggregateid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Remove(&self, paggregateid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Members(&self) -> ::windows::core::Result<IContactAggregationAggregateCollection>;
    fn GlobalObjectId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetGlobalObjectId(&self, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetName(&self, pname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IContactAggregationGroup {}
impl IContactAggregationGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>() -> IContactAggregationGroup_Vtbl {
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn Members<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregatecontactcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Members() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregatecontactcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalObjectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GlobalObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pglobalobjectid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalObjectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGlobalObjectId(::core::mem::transmute_copy(&pglobalobjectid)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitemid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&pname)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            GlobalObjectId: GlobalObjectId::<Identity, Impl, OFFSET>,
            SetGlobalObjectId: SetGlobalObjectId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationGroup as ::windows::core::Interface>::IID
    }
}
pub trait IContactAggregationGroupCollection_Impl: Sized {
    fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationGroup>;
    fn FindFirstByGlobalObjectId(&self, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::Result<IContactAggregationGroup>;
    fn FindNext(&self) -> ::windows::core::Result<IContactAggregationGroup>;
    fn Count(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IContactAggregationGroupCollection {}
impl IContactAggregationGroupCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: isize>() -> IContactAggregationGroupCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByGlobalObjectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows::core::GUID, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstByGlobalObjectId(::core::mem::transmute_copy(&pglobalobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindFirstByGlobalObjectId: FindFirstByGlobalObjectId::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationGroupCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationLink_Impl: Sized {
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn AccountId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetAccountId(&self, paccountid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn IsLinkResolved(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsLinkResolved(&self, islinkresolved: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn NetworkSourceIdString(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetNetworkSourceIdString(&self, pnetworksourceid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RemoteObjectId(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
    fn ServerPerson(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetServerPerson(&self, pserverpersonid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ServerPersonBaseline(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetServerPersonBaseline(&self, pserverpersonid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SyncIdentityHash(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IContactAggregationLink {}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationLink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>() -> IContactAggregationLink_Vtbl {
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn AccountId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccountid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccountId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccountid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccountid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccountId(::core::mem::transmute(&paccountid)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitemid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLinkResolved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislinkresolved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLinkResolved() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislinkresolved, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLinkResolved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islinkresolved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsLinkResolved(::core::mem::transmute_copy(&islinkresolved)).into()
        }
        unsafe extern "system" fn NetworkSourceIdString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkSourceIdString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworksourceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksourceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkSourceIdString(::core::mem::transmute(&pnetworksourceid)).into()
        }
        unsafe extern "system" fn RemoteObjectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoteObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppremoteobjectid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteObjectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRemoteObjectId(::core::mem::transmute_copy(&premoteobjectid)).into()
        }
        unsafe extern "system" fn ServerPerson<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServerPerson() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverpersonid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerPerson<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserverpersonid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServerPerson(::core::mem::transmute(&pserverpersonid)).into()
        }
        unsafe extern "system" fn ServerPersonBaseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServerPersonBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverpersonid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerPersonBaseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserverpersonid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServerPersonBaseline(::core::mem::transmute(&pserverpersonid)).into()
        }
        unsafe extern "system" fn SyncIdentityHash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SyncIdentityHash() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsyncidentityhash, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncIdentityHash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSyncIdentityHash(::core::mem::transmute_copy(&psyncidentityhash)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            AccountId: AccountId::<Identity, Impl, OFFSET>,
            SetAccountId: SetAccountId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            IsLinkResolved: IsLinkResolved::<Identity, Impl, OFFSET>,
            SetIsLinkResolved: SetIsLinkResolved::<Identity, Impl, OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Identity, Impl, OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Identity, Impl, OFFSET>,
            RemoteObjectId: RemoteObjectId::<Identity, Impl, OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Identity, Impl, OFFSET>,
            ServerPerson: ServerPerson::<Identity, Impl, OFFSET>,
            SetServerPerson: SetServerPerson::<Identity, Impl, OFFSET>,
            ServerPersonBaseline: ServerPersonBaseline::<Identity, Impl, OFFSET>,
            SetServerPersonBaseline: SetServerPersonBaseline::<Identity, Impl, OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Identity, Impl, OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationLink as ::windows::core::Interface>::IID
    }
}
pub trait IContactAggregationLinkCollection_Impl: Sized {
    fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationLink>;
    fn FindFirstByRemoteId(&self, psourcetype: &::windows::core::PCWSTR, paccountid: &::windows::core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<IContactAggregationLink>;
    fn FindNext(&self) -> ::windows::core::Result<IContactAggregationLink>;
    fn Count(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IContactAggregationLinkCollection {}
impl IContactAggregationLinkCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: isize>() -> IContactAggregationLinkCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByRemoteId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetype: ::windows::core::PCWSTR, paccountid: ::windows::core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstByRemoteId(::core::mem::transmute(&psourcetype), ::core::mem::transmute(&paccountid), ::core::mem::transmute_copy(&premoteid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationLinkCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationManager_Impl: Sized {
    fn GetVersionInfo(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::Result<()>;
    fn CreateOrOpenGroup(&self, pgroupname: &::windows::core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut ::core::option::Option<IContactAggregationGroup>) -> ::windows::core::Result<()>;
    fn CreateExternalContact(&self) -> ::windows::core::Result<IContactAggregationContact>;
    fn CreateServerPerson(&self) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn CreateServerContactLink(&self) -> ::windows::core::Result<IContactAggregationLink>;
    fn Flush(&self) -> ::windows::core::Result<()>;
    fn OpenAggregateContact(&self, pitemid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContactAggregationAggregate>;
    fn OpenContact(&self, pitemid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContactAggregationContact>;
    fn OpenServerContactLink(&self, pitemid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContactAggregationLink>;
    fn OpenServerPerson(&self, pitemid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn get_Contacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationContactCollection>;
    fn get_AggregateContacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationAggregateCollection>;
    fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationGroupCollection>;
    fn ServerPersons(&self) -> ::windows::core::Result<IContactAggregationServerPersonCollection>;
    fn get_ServerContactLinks(&self, ppersonitemid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContactAggregationLinkCollection>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IContactAggregationManager {}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>() -> IContactAggregationManager_Vtbl {
        unsafe extern "system" fn GetVersionInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersionInfo(::core::mem::transmute_copy(&plmajorversion), ::core::mem::transmute_copy(&plminorversion)).into()
        }
        unsafe extern "system" fn CreateOrOpenGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroupname: ::windows::core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateOrOpenGroup(::core::mem::transmute(&pgroupname), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&pcreatedgroup), ::core::mem::transmute_copy(&ppgroup)).into()
        }
        unsafe extern "system" fn CreateExternalContact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateExternalContact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServerPerson<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateServerPerson() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServerContactLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateServerContactLink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flush().into()
        }
        unsafe extern "system" fn OpenAggregateContact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: ::windows::core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenAggregateContact(::core::mem::transmute(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenContact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: ::windows::core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenContact(::core::mem::transmute(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenServerContactLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: ::windows::core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenServerContactLink(::core::mem::transmute(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenServerPerson<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: ::windows::core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenServerPerson(::core::mem::transmute(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Contacts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Contacts(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_AggregateContacts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppaggregates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_AggregateContacts(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Groups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Groups(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerPersons<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverpersoncollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServerPersons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverpersoncollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ServerContactLinks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppersonitemid: ::windows::core::PCWSTR, ppservercontactlinkcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_ServerContactLinks(::core::mem::transmute(&ppersonitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlinkcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetVersionInfo: GetVersionInfo::<Identity, Impl, OFFSET>,
            CreateOrOpenGroup: CreateOrOpenGroup::<Identity, Impl, OFFSET>,
            CreateExternalContact: CreateExternalContact::<Identity, Impl, OFFSET>,
            CreateServerPerson: CreateServerPerson::<Identity, Impl, OFFSET>,
            CreateServerContactLink: CreateServerContactLink::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            OpenAggregateContact: OpenAggregateContact::<Identity, Impl, OFFSET>,
            OpenContact: OpenContact::<Identity, Impl, OFFSET>,
            OpenServerContactLink: OpenServerContactLink::<Identity, Impl, OFFSET>,
            OpenServerPerson: OpenServerPerson::<Identity, Impl, OFFSET>,
            get_Contacts: get_Contacts::<Identity, Impl, OFFSET>,
            get_AggregateContacts: get_AggregateContacts::<Identity, Impl, OFFSET>,
            get_Groups: get_Groups::<Identity, Impl, OFFSET>,
            ServerPersons: ServerPersons::<Identity, Impl, OFFSET>,
            get_ServerContactLinks: get_ServerContactLinks::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationServerPerson_Impl: Sized {
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn AggregateId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetAggregateId(&self, paggregateid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn AntiLink(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetAntiLink(&self, pantilink: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn AntiLinkBaseline(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetAntiLinkBaseline(&self, pantilink: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FavoriteOrder(&self) -> ::windows::core::Result<u32>;
    fn SetFavoriteOrder(&self, favoriteorder: u32) -> ::windows::core::Result<()>;
    fn FavoriteOrderBaseline(&self) -> ::windows::core::Result<u32>;
    fn SetFavoriteOrderBaseline(&self, favoriteorder: u32) -> ::windows::core::Result<()>;
    fn Groups(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroups(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
    fn GroupsBaseline(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroupsBaseline(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn IsTombstone(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsTombstone(&self, istombstone: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn LinkedAggregateId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetLinkedAggregateId(&self, plinkedaggregateid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ObjectId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetObjectId(&self, pobjectid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IContactAggregationServerPerson {}
#[cfg(feature = "Win32_Foundation")]
impl IContactAggregationServerPerson_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>() -> IContactAggregationServerPerson_Vtbl {
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn AggregateId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaggregateid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AggregateId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregateid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAggregateId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAggregateId(::core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn AntiLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntiLink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppantilink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilink: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAntiLink(::core::mem::transmute(&pantilink)).into()
        }
        unsafe extern "system" fn AntiLinkBaseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntiLinkBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppantilink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiLinkBaseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pantilink: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAntiLinkBaseline(::core::mem::transmute(&pantilink)).into()
        }
        unsafe extern "system" fn FavoriteOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FavoriteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfavoriteorder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFavoriteOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFavoriteOrder(::core::mem::transmute_copy(&favoriteorder)).into()
        }
        unsafe extern "system" fn FavoriteOrderBaseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FavoriteOrderBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfavoriteorder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFavoriteOrderBaseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFavoriteOrderBaseline(::core::mem::transmute_copy(&favoriteorder)).into()
        }
        unsafe extern "system" fn Groups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Groups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroups(::core::mem::transmute_copy(&pgroups)).into()
        }
        unsafe extern "system" fn GroupsBaseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GroupsBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupsBaseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroupsBaseline(::core::mem::transmute_copy(&pgroups)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTombstone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistombstone: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTombstone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistombstone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTombstone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istombstone: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsTombstone(::core::mem::transmute_copy(&istombstone)).into()
        }
        unsafe extern "system" fn LinkedAggregateId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkedaggregateid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LinkedAggregateId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplinkedaggregateid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkedAggregateId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plinkedaggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLinkedAggregateId(::core::mem::transmute(&plinkedaggregateid)).into()
        }
        unsafe extern "system" fn ObjectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectId(::core::mem::transmute(&pobjectid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            AggregateId: AggregateId::<Identity, Impl, OFFSET>,
            SetAggregateId: SetAggregateId::<Identity, Impl, OFFSET>,
            AntiLink: AntiLink::<Identity, Impl, OFFSET>,
            SetAntiLink: SetAntiLink::<Identity, Impl, OFFSET>,
            AntiLinkBaseline: AntiLinkBaseline::<Identity, Impl, OFFSET>,
            SetAntiLinkBaseline: SetAntiLinkBaseline::<Identity, Impl, OFFSET>,
            FavoriteOrder: FavoriteOrder::<Identity, Impl, OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Identity, Impl, OFFSET>,
            FavoriteOrderBaseline: FavoriteOrderBaseline::<Identity, Impl, OFFSET>,
            SetFavoriteOrderBaseline: SetFavoriteOrderBaseline::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            SetGroups: SetGroups::<Identity, Impl, OFFSET>,
            GroupsBaseline: GroupsBaseline::<Identity, Impl, OFFSET>,
            SetGroupsBaseline: SetGroupsBaseline::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            IsTombstone: IsTombstone::<Identity, Impl, OFFSET>,
            SetIsTombstone: SetIsTombstone::<Identity, Impl, OFFSET>,
            LinkedAggregateId: LinkedAggregateId::<Identity, Impl, OFFSET>,
            SetLinkedAggregateId: SetLinkedAggregateId::<Identity, Impl, OFFSET>,
            ObjectId: ObjectId::<Identity, Impl, OFFSET>,
            SetObjectId: SetObjectId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationServerPerson as ::windows::core::Interface>::IID
    }
}
pub trait IContactAggregationServerPersonCollection_Impl: Sized {
    fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn FindFirstByServerId(&self, pserverid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn FindFirstByAggregateId(&self, paggregateid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn FindFirstByLinkedAggregateId(&self, paggregateid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn FindNext(&self) -> ::windows::core::Result<IContactAggregationServerPerson>;
    fn Count(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IContactAggregationServerPersonCollection {}
impl IContactAggregationServerPersonCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>() -> IContactAggregationServerPersonCollection_Vtbl {
        unsafe extern "system" fn FindFirst<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByServerId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserverid: ::windows::core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstByServerId(::core::mem::transmute(&pserverid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByAggregateId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstByAggregateId(::core::mem::transmute(&paggregateid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByLinkedAggregateId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstByLinkedAggregateId(::core::mem::transmute(&paggregateid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindFirstByServerId: FindFirstByServerId::<Identity, Impl, OFFSET>,
            FindFirstByAggregateId: FindFirstByAggregateId::<Identity, Impl, OFFSET>,
            FindFirstByLinkedAggregateId: FindFirstByLinkedAggregateId::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAggregationServerPersonCollection as ::windows::core::Interface>::IID
    }
}
pub trait IContactCollection_Impl: Sized {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<()>;
    fn GetCurrent(&self) -> ::windows::core::Result<IContact>;
}
impl ::windows::core::RuntimeName for IContactCollection {}
impl IContactCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactCollection_Impl, const OFFSET: isize>() -> IContactCollection_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next().into()
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontact: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactCollection as ::windows::core::Interface>::IID
    }
}
pub trait IContactManager_Impl: Sized {
    fn Initialize(&self, pszappname: &::windows::core::PCWSTR, pszappversion: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Load(&self, pszcontactid: &::windows::core::PCWSTR) -> ::windows::core::Result<IContact>;
    fn MergeContactIDs(&self, psznewcontactid: &::windows::core::PCWSTR, pszoldcontactid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetMeContact(&self) -> ::windows::core::Result<IContact>;
    fn SetMeContact(&self, pmecontact: &::core::option::Option<IContact>) -> ::windows::core::Result<()>;
    fn GetContactCollection(&self) -> ::windows::core::Result<IContactCollection>;
}
impl ::windows::core::RuntimeName for IContactManager {}
impl IContactManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: isize>() -> IContactManager_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszappname: ::windows::core::PCWSTR, pszappversion: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&pszappname), ::core::mem::transmute(&pszappversion)).into()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcontactid: ::windows::core::PCWSTR, ppcontact: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Load(::core::mem::transmute(&pszcontactid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MergeContactIDs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psznewcontactid: ::windows::core::PCWSTR, pszoldcontactid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MergeContactIDs(::core::mem::transmute(&psznewcontactid), ::core::mem::transmute(&pszoldcontactid)).into()
        }
        unsafe extern "system" fn GetMeContact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmecontact: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMeContact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmecontact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeContact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmecontact: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMeContact(::core::mem::transmute(&pmecontact)).into()
        }
        unsafe extern "system" fn GetContactCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontactcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContactCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontactcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            MergeContactIDs: MergeContactIDs::<Identity, Impl, OFFSET>,
            GetMeContact: GetMeContact::<Identity, Impl, OFFSET>,
            SetMeContact: SetMeContact::<Identity, Impl, OFFSET>,
            GetContactCollection: GetContactCollection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IContactProperties_Impl: Sized {
    fn GetString(&self, pszpropertyname: &::windows::core::PCWSTR, dwflags: u32, pszvalue: &::windows::core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetDate(&self, pszpropertyname: &::windows::core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetBinary(&self, pszpropertyname: &::windows::core::PCWSTR, dwflags: u32, pszcontenttype: &::windows::core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut ::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()>;
    fn GetLabels(&self, pszarrayelementname: &::windows::core::PCWSTR, dwflags: u32, pszlabels: &::windows::core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows::core::Result<()>;
    fn SetString(&self, pszpropertyname: &::windows::core::PCWSTR, dwflags: u32, pszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetDate(&self, pszpropertyname: &::windows::core::PCWSTR, dwflags: u32, ftdatetime: &super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SetBinary(&self, pszpropertyname: &::windows::core::PCWSTR, dwflags: u32, pszcontenttype: &::windows::core::PCWSTR, pstream: &::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()>;
    fn SetLabels(&self, pszarrayelementname: &::windows::core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn CreateArrayNode(&self, pszarrayname: &::windows::core::PCWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: &::windows::core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows::core::Result<()>;
    fn DeleteProperty(&self, pszpropertyname: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn DeleteArrayNode(&self, pszarrayelementname: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn DeleteLabels(&self, pszarrayelementname: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetPropertyCollection(&self, pppropertycollection: *mut ::core::option::Option<IContactPropertyCollection>, dwflags: u32, pszmultivaluename: &::windows::core::PCWSTR, dwlabelcount: u32, ppszlabels: *const ::windows::core::PWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IContactProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IContactProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>() -> IContactProperties_Vtbl {
        unsafe extern "system" fn GetString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pszvalue: ::windows::core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetString(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszvalue), ::core::mem::transmute_copy(&cchvalue), ::core::mem::transmute_copy(&pdwcchpropertyvaluerequired)).into()
        }
        unsafe extern "system" fn GetDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDate(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pftdatetime)).into()
        }
        unsafe extern "system" fn GetBinary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pszcontenttype: ::windows::core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBinary(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszcontenttype), ::core::mem::transmute_copy(&cchcontenttype), ::core::mem::transmute_copy(&pdwcchcontenttyperequired), ::core::mem::transmute_copy(&ppstream)).into()
        }
        unsafe extern "system" fn GetLabels<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows::core::PCWSTR, dwflags: u32, pszlabels: ::windows::core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLabels(::core::mem::transmute(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszlabels), ::core::mem::transmute_copy(&cchlabels), ::core::mem::transmute_copy(&pdwcchlabelsrequired)).into()
        }
        unsafe extern "system" fn SetString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetString(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszvalue)).into()
        }
        unsafe extern "system" fn SetDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDate(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&ftdatetime)).into()
        }
        unsafe extern "system" fn SetBinary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pszcontenttype: ::windows::core::PCWSTR, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBinary(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszcontenttype), ::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn SetLabels<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows::core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabels(::core::mem::transmute(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwlabelcount), ::core::mem::transmute_copy(&ppszlabels)).into()
        }
        unsafe extern "system" fn CreateArrayNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayname: ::windows::core::PCWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: ::windows::core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateArrayNode(::core::mem::transmute(&pszarrayname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&fappend), ::core::mem::transmute(&psznewarrayelementname), ::core::mem::transmute_copy(&cchnewarrayelementname), ::core::mem::transmute_copy(&pdwcchnewarrayelementnamerequired)).into()
        }
        unsafe extern "system" fn DeleteProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteProperty(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteArrayNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteArrayNode(::core::mem::transmute(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteLabels<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteLabels(::core::mem::transmute(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetPropertyCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertycollection: *mut *mut ::core::ffi::c_void, dwflags: u32, pszmultivaluename: ::windows::core::PCWSTR, dwlabelcount: u32, ppszlabels: *const ::windows::core::PWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyCollection(::core::mem::transmute_copy(&pppropertycollection), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszmultivaluename), ::core::mem::transmute_copy(&dwlabelcount), ::core::mem::transmute_copy(&ppszlabels), ::core::mem::transmute_copy(&fanylabelmatches)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetDate: GetDate::<Identity, Impl, OFFSET>,
            GetBinary: GetBinary::<Identity, Impl, OFFSET>,
            GetLabels: GetLabels::<Identity, Impl, OFFSET>,
            SetString: SetString::<Identity, Impl, OFFSET>,
            SetDate: SetDate::<Identity, Impl, OFFSET>,
            SetBinary: SetBinary::<Identity, Impl, OFFSET>,
            SetLabels: SetLabels::<Identity, Impl, OFFSET>,
            CreateArrayNode: CreateArrayNode::<Identity, Impl, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, Impl, OFFSET>,
            DeleteArrayNode: DeleteArrayNode::<Identity, Impl, OFFSET>,
            DeleteLabels: DeleteLabels::<Identity, Impl, OFFSET>,
            GetPropertyCollection: GetPropertyCollection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContactPropertyCollection_Impl: Sized {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<()>;
    fn GetPropertyName(&self, pszpropertyname: &::windows::core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetPropertyType(&self, pdwtype: *mut u32) -> ::windows::core::Result<()>;
    fn GetPropertyVersion(&self, pdwversion: *mut u32) -> ::windows::core::Result<()>;
    fn GetPropertyModificationDate(&self, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetPropertyArrayElementID(&self, pszarrayelementid: &::windows::core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IContactPropertyCollection {}
#[cfg(feature = "Win32_Foundation")]
impl IContactPropertyCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: isize>() -> IContactPropertyCollection_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next().into()
        }
        unsafe extern "system" fn GetPropertyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyName(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&cchpropertyname), ::core::mem::transmute_copy(&pdwcchpropertynamerequired)).into()
        }
        unsafe extern "system" fn GetPropertyType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyType(::core::mem::transmute_copy(&pdwtype)).into()
        }
        unsafe extern "system" fn GetPropertyVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyVersion(::core::mem::transmute_copy(&pdwversion)).into()
        }
        unsafe extern "system" fn GetPropertyModificationDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyModificationDate(::core::mem::transmute_copy(&pftmodificationdate)).into()
        }
        unsafe extern "system" fn GetPropertyArrayElementID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszarrayelementid: ::windows::core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyArrayElementID(::core::mem::transmute(&pszarrayelementid), ::core::mem::transmute_copy(&ccharrayelementid), ::core::mem::transmute_copy(&pdwccharrayelementidrequired)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            GetPropertyName: GetPropertyName::<Identity, Impl, OFFSET>,
            GetPropertyType: GetPropertyType::<Identity, Impl, OFFSET>,
            GetPropertyVersion: GetPropertyVersion::<Identity, Impl, OFFSET>,
            GetPropertyModificationDate: GetPropertyModificationDate::<Identity, Impl, OFFSET>,
            GetPropertyArrayElementID: GetPropertyArrayElementID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPropertyCollection as ::windows::core::Interface>::IID
    }
}
