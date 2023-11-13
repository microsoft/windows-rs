#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContact(::windows_core::IUnknown);
impl IContact {
    pub unsafe fn GetContactID(&self, pszcontactid: &mut [u16], pdwcchcontactidrequired: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetContactID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszcontactid.as_ptr()), pszcontactid.len().try_into().unwrap(), pdwcchcontactidrequired).ok()
    }
    pub unsafe fn GetPath(&self, pszpath: &mut [u16], pdwcchpathrequired: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap(), pdwcchpathrequired).ok()
    }
    pub unsafe fn CommitChanges(&self, dwcommitflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitChanges)(::windows_core::Interface::as_raw(self), dwcommitflags).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IContact, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContact {
    type Vtable = IContact_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContact {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf941b671_bda7_4f77_884a_f46462f226a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetContactID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcontactid: ::windows_core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows_core::HRESULT,
    pub CommitChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcommitflags: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationAggregate(::windows_core::IUnknown);
impl IContactAggregationAggregate {
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetComponentItems(&self) -> ::windows_core::Result<IContactAggregationContactCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetComponentItems)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Link<P0>(&self, paggregateid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Link)(::windows_core::Interface::as_raw(self), paggregateid.into_param().abi()).ok()
    }
    pub unsafe fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows_core::Result<IContactAggregationGroupCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Groups)(::windows_core::Interface::as_raw(self), options, &mut result__).from_abi(result__)
    }
    pub unsafe fn AntiLink(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AntiLink)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAntiLink<P0>(&self, pantilink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAntiLink)(::windows_core::Interface::as_raw(self), pantilink.into_param().abi()).ok()
    }
    pub unsafe fn FavoriteOrder(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FavoriteOrder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFavoriteOrder(&self, favoriteorder: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFavoriteOrder)(::windows_core::Interface::as_raw(self), favoriteorder).ok()
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationAggregate, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationAggregate {
    type Vtable = IContactAggregationAggregate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationAggregate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ed1c814_cd30_43c8_9b8d_2e489e53d54b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationAggregate_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetComponentItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomponentitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub get_Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AntiLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAntiLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pantilink: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FavoriteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows_core::HRESULT,
    pub SetFavoriteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationAggregateCollection(::windows_core::IUnknown);
impl IContactAggregationAggregateCollection {
    pub unsafe fn FindFirst(&self) -> ::windows_core::Result<IContactAggregationAggregate> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirst)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFirstByAntiLinkId<P0>(&self, pantilinkid: P0) -> ::windows_core::Result<IContactAggregationAggregate>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstByAntiLinkId)(::windows_core::Interface::as_raw(self), pantilinkid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows_core::Result<IContactAggregationAggregate> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationAggregateCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationAggregateCollection {
    type Vtable = IContactAggregationAggregateCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationAggregateCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2359f3a6_3a68_40af_98db_0f9eb143c3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationAggregateCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindFirstByAntiLinkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pantilinkid: ::windows_core::PCWSTR, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationContact(::windows_core::IUnknown);
impl IContactAggregationContact {
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MoveToAggregate<P0>(&self, paggregateid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).MoveToAggregate)(::windows_core::Interface::as_raw(self), paggregateid.into_param().abi()).ok()
    }
    pub unsafe fn Unlink(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unlink)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AccountId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AccountId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAccountId<P0>(&self, paccountid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAccountId)(::windows_core::Interface::as_raw(self), paccountid.into_param().abi()).ok()
    }
    pub unsafe fn AggregateId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AggregateId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMe(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsMe)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsExternal(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsExternal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NetworkSourceId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NetworkSourceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNetworkSourceId(&self, networksourceid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkSourceId)(::windows_core::Interface::as_raw(self), networksourceid).ok()
    }
    pub unsafe fn NetworkSourceIdString(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NetworkSourceIdString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNetworkSourceIdString<P0>(&self, pnetworksourceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetNetworkSourceIdString)(::windows_core::Interface::as_raw(self), pnetworksourceid.into_param().abi()).ok()
    }
    pub unsafe fn RemoteObjectId(&self) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteObjectId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteObjectId)(::windows_core::Interface::as_raw(self), premoteobjectid).ok()
    }
    pub unsafe fn SyncIdentityHash(&self) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SyncIdentityHash)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSyncIdentityHash)(::windows_core::Interface::as_raw(self), psyncidentityhash).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationContact, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationContact {
    type Vtable = IContactAggregationContact_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationContact {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1eb22e86_4c86_41f0_9f9f_c251e9fda6c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationContact_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveToAggregate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Unlink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaccountid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paccountid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub AggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregateid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisme: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMe: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsExternal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisexternal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsExternal: usize,
    pub NetworkSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworksourceid: *mut u32) -> ::windows_core::HRESULT,
    pub SetNetworkSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networksourceid: u32) -> ::windows_core::HRESULT,
    pub NetworkSourceIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetNetworkSourceIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworksourceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub RemoteObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub SetRemoteObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub SyncIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub SetSyncIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationContactCollection(::windows_core::IUnknown);
impl IContactAggregationContactCollection {
    pub unsafe fn FindFirst(&self) -> ::windows_core::Result<IContactAggregationContact> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirst)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows_core::Result<IContactAggregationContact> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFirstByIdentityHash<P0, P1>(&self, psourcetype: P0, paccountid: P1, pidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<IContactAggregationContact>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstByIdentityHash)(::windows_core::Interface::as_raw(self), psourcetype.into_param().abi(), paccountid.into_param().abi(), pidentityhash, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFirstByRemoteId<P0, P1>(&self, psourcetype: P0, paccountid: P1, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<IContactAggregationContact>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstByRemoteId)(::windows_core::Interface::as_raw(self), psourcetype.into_param().abi(), paccountid.into_param().abi(), premoteobjectid, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationContactCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationContactCollection {
    type Vtable = IContactAggregationContactCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationContactCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x826e66fa_81de_43ca_a6fb_8c785cd996c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationContactCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindFirstByIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcetype: ::windows_core::PCWSTR, paccountid: ::windows_core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub FindFirstByRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcetype: ::windows_core::PCWSTR, paccountid: ::windows_core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationGroup(::windows_core::IUnknown);
impl IContactAggregationGroup {
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Add<P0>(&self, paggregateid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), paggregateid.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, paggregateid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), paggregateid.into_param().abi()).ok()
    }
    pub unsafe fn Members(&self) -> ::windows_core::Result<IContactAggregationAggregateCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Members)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GlobalObjectId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GlobalObjectId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGlobalObjectId(&self, pglobalobjectid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGlobalObjectId)(::windows_core::Interface::as_raw(self), pglobalobjectid).ok()
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, pname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), pname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationGroup, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationGroup {
    type Vtable = IContactAggregationGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc93c545f_1284_499b_96af_07372af473e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationGroup_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregatecontactcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GlobalObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pglobalobjectid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetGlobalObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationGroupCollection(::windows_core::IUnknown);
impl IContactAggregationGroupCollection {
    pub unsafe fn FindFirst(&self) -> ::windows_core::Result<IContactAggregationGroup> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirst)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFirstByGlobalObjectId(&self, pglobalobjectid: *const ::windows_core::GUID) -> ::windows_core::Result<IContactAggregationGroup> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstByGlobalObjectId)(::windows_core::Interface::as_raw(self), pglobalobjectid, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows_core::Result<IContactAggregationGroup> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationGroupCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationGroupCollection {
    type Vtable = IContactAggregationGroupCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationGroupCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20a19a9c_d2f3_4b83_9143_beffd2cc226d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationGroupCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindFirstByGlobalObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows_core::GUID, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationLink(::windows_core::IUnknown);
impl IContactAggregationLink {
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AccountId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AccountId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAccountId<P0>(&self, paccountid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAccountId)(::windows_core::Interface::as_raw(self), paccountid.into_param().abi()).ok()
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLinkResolved(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsLinkResolved)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsLinkResolved<P0>(&self, islinkresolved: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsLinkResolved)(::windows_core::Interface::as_raw(self), islinkresolved.into_param().abi()).ok()
    }
    pub unsafe fn NetworkSourceIdString(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NetworkSourceIdString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNetworkSourceIdString<P0>(&self, pnetworksourceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetNetworkSourceIdString)(::windows_core::Interface::as_raw(self), pnetworksourceid.into_param().abi()).ok()
    }
    pub unsafe fn RemoteObjectId(&self) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteObjectId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteObjectId)(::windows_core::Interface::as_raw(self), premoteobjectid).ok()
    }
    pub unsafe fn ServerPerson(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServerPerson)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServerPerson<P0>(&self, pserverpersonid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServerPerson)(::windows_core::Interface::as_raw(self), pserverpersonid.into_param().abi()).ok()
    }
    pub unsafe fn ServerPersonBaseline(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServerPersonBaseline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServerPersonBaseline<P0>(&self, pserverpersonid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServerPersonBaseline)(::windows_core::Interface::as_raw(self), pserverpersonid.into_param().abi()).ok()
    }
    pub unsafe fn SyncIdentityHash(&self) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SyncIdentityHash)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSyncIdentityHash)(::windows_core::Interface::as_raw(self), psyncidentityhash).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationLink, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationLink {
    type Vtable = IContactAggregationLink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationLink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6813323_a183_4654_8627_79b30de3a0ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationLink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaccountid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paccountid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLinkResolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislinkresolved: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLinkResolved: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsLinkResolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, islinkresolved: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsLinkResolved: usize,
    pub NetworkSourceIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetNetworkSourceIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworksourceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub RemoteObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub SetRemoteObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub ServerPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetServerPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserverpersonid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub ServerPersonBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetServerPersonBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserverpersonid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SyncIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub SetSyncIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationLinkCollection(::windows_core::IUnknown);
impl IContactAggregationLinkCollection {
    pub unsafe fn FindFirst(&self) -> ::windows_core::Result<IContactAggregationLink> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirst)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFirstByRemoteId<P0, P1>(&self, psourcetype: P0, paccountid: P1, premoteid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<IContactAggregationLink>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstByRemoteId)(::windows_core::Interface::as_raw(self), psourcetype.into_param().abi(), paccountid.into_param().abi(), premoteid, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows_core::Result<IContactAggregationLink> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationLinkCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationLinkCollection {
    type Vtable = IContactAggregationLinkCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationLinkCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8bc0e93_fb55_4f28_b9fa_b1c274153292);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationLinkCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindFirstByRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcetype: ::windows_core::PCWSTR, paccountid: ::windows_core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationManager(::windows_core::IUnknown);
impl IContactAggregationManager {
    pub unsafe fn GetVersionInfo(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVersionInfo)(::windows_core::Interface::as_raw(self), plmajorversion, plminorversion).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOrOpenGroup<P0>(&self, pgroupname: P0, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut ::core::option::Option<IContactAggregationGroup>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).CreateOrOpenGroup)(::windows_core::Interface::as_raw(self), pgroupname.into_param().abi(), options, pcreatedgroup, ::core::mem::transmute(ppgroup)).ok()
    }
    pub unsafe fn CreateExternalContact(&self) -> ::windows_core::Result<IContactAggregationContact> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateExternalContact)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateServerPerson(&self) -> ::windows_core::Result<IContactAggregationServerPerson> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateServerPerson)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateServerContactLink(&self) -> ::windows_core::Result<IContactAggregationLink> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateServerContactLink)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Flush(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Flush)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OpenAggregateContact<P0>(&self, pitemid: P0) -> ::windows_core::Result<IContactAggregationAggregate>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OpenAggregateContact)(::windows_core::Interface::as_raw(self), pitemid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenContact<P0>(&self, pitemid: P0) -> ::windows_core::Result<IContactAggregationContact>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OpenContact)(::windows_core::Interface::as_raw(self), pitemid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenServerContactLink<P0>(&self, pitemid: P0) -> ::windows_core::Result<IContactAggregationLink>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OpenServerContactLink)(::windows_core::Interface::as_raw(self), pitemid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenServerPerson<P0>(&self, pitemid: P0) -> ::windows_core::Result<IContactAggregationServerPerson>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OpenServerPerson)(::windows_core::Interface::as_raw(self), pitemid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Contacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows_core::Result<IContactAggregationContactCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Contacts)(::windows_core::Interface::as_raw(self), options, &mut result__).from_abi(result__)
    }
    pub unsafe fn get_AggregateContacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows_core::Result<IContactAggregationAggregateCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_AggregateContacts)(::windows_core::Interface::as_raw(self), options, &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows_core::Result<IContactAggregationGroupCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Groups)(::windows_core::Interface::as_raw(self), options, &mut result__).from_abi(result__)
    }
    pub unsafe fn ServerPersons(&self) -> ::windows_core::Result<IContactAggregationServerPersonCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServerPersons)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_ServerContactLinks<P0>(&self, ppersonitemid: P0) -> ::windows_core::Result<IContactAggregationLinkCollection>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_ServerContactLinks)(::windows_core::Interface::as_raw(self), ppersonitemid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationManager {
    type Vtable = IContactAggregationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d865989_4b1f_4b60_8f34_c2ad468b2b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateOrOpenGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroupname: ::windows_core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateOrOpenGroup: usize,
    pub CreateExternalContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateServerPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateServerContactLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenAggregateContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: ::windows_core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: ::windows_core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenServerContactLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: ::windows_core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenServerPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: ::windows_core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_Contacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_AggregateContacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppaggregates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ServerPersons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverpersoncollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_ServerContactLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppersonitemid: ::windows_core::PCWSTR, ppservercontactlinkcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationServerPerson(::windows_core::IUnknown);
impl IContactAggregationServerPerson {
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AggregateId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AggregateId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAggregateId<P0>(&self, paggregateid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAggregateId)(::windows_core::Interface::as_raw(self), paggregateid.into_param().abi()).ok()
    }
    pub unsafe fn AntiLink(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AntiLink)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAntiLink<P0>(&self, pantilink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAntiLink)(::windows_core::Interface::as_raw(self), pantilink.into_param().abi()).ok()
    }
    pub unsafe fn AntiLinkBaseline(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AntiLinkBaseline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAntiLinkBaseline<P0>(&self, pantilink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAntiLinkBaseline)(::windows_core::Interface::as_raw(self), pantilink.into_param().abi()).ok()
    }
    pub unsafe fn FavoriteOrder(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FavoriteOrder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFavoriteOrder(&self, favoriteorder: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFavoriteOrder)(::windows_core::Interface::as_raw(self), favoriteorder).ok()
    }
    pub unsafe fn FavoriteOrderBaseline(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FavoriteOrderBaseline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFavoriteOrderBaseline(&self, favoriteorder: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFavoriteOrderBaseline)(::windows_core::Interface::as_raw(self), favoriteorder).ok()
    }
    pub unsafe fn Groups(&self) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Groups)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGroups(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGroups)(::windows_core::Interface::as_raw(self), pgroups).ok()
    }
    pub unsafe fn GroupsBaseline(&self) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GroupsBaseline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGroupsBaseline(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGroupsBaseline)(::windows_core::Interface::as_raw(self), pgroups).ok()
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTombstone(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsTombstone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsTombstone<P0>(&self, istombstone: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsTombstone)(::windows_core::Interface::as_raw(self), istombstone.into_param().abi()).ok()
    }
    pub unsafe fn LinkedAggregateId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LinkedAggregateId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLinkedAggregateId<P0>(&self, plinkedaggregateid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLinkedAggregateId)(::windows_core::Interface::as_raw(self), plinkedaggregateid.into_param().abi()).ok()
    }
    pub unsafe fn ObjectId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ObjectId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetObjectId<P0>(&self, pobjectid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetObjectId)(::windows_core::Interface::as_raw(self), pobjectid.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationServerPerson, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationServerPerson {
    type Vtable = IContactAggregationServerPerson_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationServerPerson {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fdc3d4b_1b82_4334_85c5_25184ee5a5f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationServerPerson_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregateid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub AntiLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAntiLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pantilink: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub AntiLinkBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAntiLinkBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pantilink: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FavoriteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows_core::HRESULT,
    pub SetFavoriteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows_core::HRESULT,
    pub FavoriteOrderBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows_core::HRESULT,
    pub SetFavoriteOrderBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows_core::HRESULT,
    pub Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub SetGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub GroupsBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub SetGroupsBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTombstone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistombstone: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTombstone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsTombstone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istombstone: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsTombstone: usize,
    pub LinkedAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplinkedaggregateid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetLinkedAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plinkedaggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub ObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactAggregationServerPersonCollection(::windows_core::IUnknown);
impl IContactAggregationServerPersonCollection {
    pub unsafe fn FindFirst(&self) -> ::windows_core::Result<IContactAggregationServerPerson> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirst)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFirstByServerId<P0>(&self, pserverid: P0) -> ::windows_core::Result<IContactAggregationServerPerson>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstByServerId)(::windows_core::Interface::as_raw(self), pserverid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFirstByAggregateId<P0>(&self, paggregateid: P0) -> ::windows_core::Result<IContactAggregationServerPerson>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstByAggregateId)(::windows_core::Interface::as_raw(self), paggregateid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFirstByLinkedAggregateId<P0>(&self, paggregateid: P0) -> ::windows_core::Result<IContactAggregationServerPerson>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstByLinkedAggregateId)(::windows_core::Interface::as_raw(self), paggregateid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows_core::Result<IContactAggregationServerPerson> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IContactAggregationServerPersonCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAggregationServerPersonCollection {
    type Vtable = IContactAggregationServerPersonCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactAggregationServerPersonCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f730a4a_6604_47b6_a987_669ecf1e5751);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationServerPersonCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindFirstByServerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserverid: ::windows_core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindFirstByAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindFirstByLinkedAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactCollection(::windows_core::IUnknown);
impl IContactCollection {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<IContact> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IContactCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactCollection {
    type Vtable = IContactCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6afa338_d779_11d9_8bde_f66bad1e3f3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontact: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactManager(::windows_core::IUnknown);
impl IContactManager {
    pub unsafe fn Initialize<P0, P1>(&self, pszappname: P0, pszappversion: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pszappname.into_param().abi(), pszappversion.into_param().abi()).ok()
    }
    pub unsafe fn Load<P0>(&self, pszcontactid: P0) -> ::windows_core::Result<IContact>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), pszcontactid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn MergeContactIDs<P0, P1>(&self, psznewcontactid: P0, pszoldcontactid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).MergeContactIDs)(::windows_core::Interface::as_raw(self), psznewcontactid.into_param().abi(), pszoldcontactid.into_param().abi()).ok()
    }
    pub unsafe fn GetMeContact(&self) -> ::windows_core::Result<IContact> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMeContact)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMeContact<P0>(&self, pmecontact: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IContact>,
    {
        (::windows_core::Interface::vtable(self).SetMeContact)(::windows_core::Interface::as_raw(self), pmecontact.into_param().abi()).ok()
    }
    pub unsafe fn GetContactCollection(&self) -> ::windows_core::Result<IContactCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContactCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IContactManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactManager {
    type Vtable = IContactManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad553d98_deb1_474a_8e17_fc0c2075b738);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszappname: ::windows_core::PCWSTR, pszappversion: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcontactid: ::windows_core::PCWSTR, ppcontact: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MergeContactIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psznewcontactid: ::windows_core::PCWSTR, pszoldcontactid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetMeContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmecontact: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMeContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmecontact: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetContactCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontactcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactProperties(::windows_core::IUnknown);
impl IContactProperties {
    pub unsafe fn GetString<P0>(&self, pszpropertyname: P0, dwflags: u32, pszvalue: &mut [u16], pdwcchpropertyvaluerequired: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetString)(::windows_core::Interface::as_raw(self), pszpropertyname.into_param().abi(), dwflags, ::core::mem::transmute(pszvalue.as_ptr()), pszvalue.len().try_into().unwrap(), pdwcchpropertyvaluerequired).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDate<P0>(&self, pszpropertyname: P0, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetDate)(::windows_core::Interface::as_raw(self), pszpropertyname.into_param().abi(), dwflags, pftdatetime).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBinary<P0>(&self, pszpropertyname: P0, dwflags: u32, pszcontenttype: &mut [u16], pdwcchcontenttyperequired: *mut u32, ppstream: *mut ::core::option::Option<super::Com::IStream>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetBinary)(::windows_core::Interface::as_raw(self), pszpropertyname.into_param().abi(), dwflags, ::core::mem::transmute(pszcontenttype.as_ptr()), pszcontenttype.len().try_into().unwrap(), pdwcchcontenttyperequired, ::core::mem::transmute(ppstream)).ok()
    }
    pub unsafe fn GetLabels<P0>(&self, pszarrayelementname: P0, dwflags: u32, pszlabels: &mut [u16], pdwcchlabelsrequired: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetLabels)(::windows_core::Interface::as_raw(self), pszarrayelementname.into_param().abi(), dwflags, ::core::mem::transmute(pszlabels.as_ptr()), pszlabels.len().try_into().unwrap(), pdwcchlabelsrequired).ok()
    }
    pub unsafe fn SetString<P0, P1>(&self, pszpropertyname: P0, dwflags: u32, pszvalue: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetString)(::windows_core::Interface::as_raw(self), pszpropertyname.into_param().abi(), dwflags, pszvalue.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDate<P0>(&self, pszpropertyname: P0, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDate)(::windows_core::Interface::as_raw(self), pszpropertyname.into_param().abi(), dwflags, ::core::mem::transmute(ftdatetime)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBinary<P0, P1, P2>(&self, pszpropertyname: P0, dwflags: u32, pszcontenttype: P1, pstream: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).SetBinary)(::windows_core::Interface::as_raw(self), pszpropertyname.into_param().abi(), dwflags, pszcontenttype.into_param().abi(), pstream.into_param().abi()).ok()
    }
    pub unsafe fn SetLabels<P0>(&self, pszarrayelementname: P0, dwflags: u32, ppszlabels: &[::windows_core::PCWSTR]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLabels)(::windows_core::Interface::as_raw(self), pszarrayelementname.into_param().abi(), dwflags, ppszlabels.len().try_into().unwrap(), ::core::mem::transmute(ppszlabels.as_ptr())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateArrayNode<P0, P1>(&self, pszarrayname: P0, dwflags: u32, fappend: P1, psznewarrayelementname: &mut [u16], pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).CreateArrayNode)(::windows_core::Interface::as_raw(self), pszarrayname.into_param().abi(), dwflags, fappend.into_param().abi(), ::core::mem::transmute(psznewarrayelementname.as_ptr()), psznewarrayelementname.len().try_into().unwrap(), pdwcchnewarrayelementnamerequired).ok()
    }
    pub unsafe fn DeleteProperty<P0>(&self, pszpropertyname: P0, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteProperty)(::windows_core::Interface::as_raw(self), pszpropertyname.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn DeleteArrayNode<P0>(&self, pszarrayelementname: P0, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteArrayNode)(::windows_core::Interface::as_raw(self), pszarrayelementname.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn DeleteLabels<P0>(&self, pszarrayelementname: P0, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteLabels)(::windows_core::Interface::as_raw(self), pszarrayelementname.into_param().abi(), dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyCollection<P0, P1>(&self, pppropertycollection: *mut ::core::option::Option<IContactPropertyCollection>, dwflags: u32, pszmultivaluename: P0, ppszlabels: &[::windows_core::PCWSTR], fanylabelmatches: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).GetPropertyCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pppropertycollection), dwflags, pszmultivaluename.into_param().abi(), ppszlabels.len().try_into().unwrap(), ::core::mem::transmute(ppszlabels.as_ptr()), fanylabelmatches.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IContactProperties, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactProperties {
    type Vtable = IContactProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70dd27dd_5cbd_46e8_bef0_23b6b346288f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactProperties_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pszvalue: ::windows_core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pszcontenttype: ::windows_core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBinary: usize,
    pub GetLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows_core::PCWSTR, dwflags: u32, pszlabels: ::windows_core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows_core::HRESULT,
    pub SetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pszcontenttype: ::windows_core::PCWSTR, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBinary: usize,
    pub SetLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows_core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateArrayNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayname: ::windows_core::PCWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: ::windows_core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateArrayNode: usize,
    pub DeleteProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    pub DeleteArrayNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    pub DeleteLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertycollection: *mut *mut ::core::ffi::c_void, dwflags: u32, pszmultivaluename: ::windows_core::PCWSTR, dwlabelcount: u32, ppszlabels: *const ::windows_core::PCWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyCollection: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactPropertyCollection(::windows_core::IUnknown);
impl IContactPropertyCollection {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPropertyName(&self, pszpropertyname: &mut [u16], pdwcchpropertynamerequired: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszpropertyname.as_ptr()), pszpropertyname.len().try_into().unwrap(), pdwcchpropertynamerequired).ok()
    }
    pub unsafe fn GetPropertyType(&self, pdwtype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyType)(::windows_core::Interface::as_raw(self), pdwtype).ok()
    }
    pub unsafe fn GetPropertyVersion(&self, pdwversion: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyVersion)(::windows_core::Interface::as_raw(self), pdwversion).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyModificationDate(&self, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyModificationDate)(::windows_core::Interface::as_raw(self), pftmodificationdate).ok()
    }
    pub unsafe fn GetPropertyArrayElementID(&self, pszarrayelementid: &mut [u16], pdwccharrayelementidrequired: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyArrayElementID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszarrayelementid.as_ptr()), pszarrayelementid.len().try_into().unwrap(), pdwccharrayelementidrequired).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IContactPropertyCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPropertyCollection {
    type Vtable = IContactPropertyCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactPropertyCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xffd3adf8_fa64_4328_b1b6_2e0db509cb3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPropertyCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows_core::HRESULT,
    pub GetPropertyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows_core::HRESULT,
    pub GetPropertyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyModificationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyModificationDate: usize,
    pub GetPropertyArrayElementID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementid: ::windows_core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows_core::HRESULT,
}
pub const CACO_DEFAULT: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(0i32);
pub const CACO_EXTERNAL_ONLY: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(2i32);
pub const CACO_INCLUDE_EXTERNAL: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(1i32);
pub const CA_CREATE_EXTERNAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(1i32);
pub const CA_CREATE_LOCAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(0i32);
pub const CGD_ARRAY_NODE: u32 = 8u32;
pub const CGD_BINARY_PROPERTY: u32 = 4u32;
pub const CGD_DATE_PROPERTY: u32 = 2u32;
pub const CGD_DEFAULT: u32 = 0u32;
pub const CGD_STRING_PROPERTY: u32 = 1u32;
pub const CGD_UNKNOWN_PROPERTY: u32 = 0u32;
pub const CLSID_ContactAggregationManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96c8ad95_c199_44de_b34e_ac33c442df39);
pub const CONTACTLABEL_PUB_AGENT: ::windows_core::PCWSTR = ::windows_core::w!("Agent");
pub const CONTACTLABEL_PUB_BBS: ::windows_core::PCWSTR = ::windows_core::w!("BBS");
pub const CONTACTLABEL_PUB_BUSINESS: ::windows_core::PCWSTR = ::windows_core::w!("Business");
pub const CONTACTLABEL_PUB_CAR: ::windows_core::PCWSTR = ::windows_core::w!("Car");
pub const CONTACTLABEL_PUB_CELLULAR: ::windows_core::PCWSTR = ::windows_core::w!("Cellular");
pub const CONTACTLABEL_PUB_DOMESTIC: ::windows_core::PCWSTR = ::windows_core::w!("Domestic");
pub const CONTACTLABEL_PUB_FAX: ::windows_core::PCWSTR = ::windows_core::w!("Fax");
pub const CONTACTLABEL_PUB_INTERNATIONAL: ::windows_core::PCWSTR = ::windows_core::w!("International");
pub const CONTACTLABEL_PUB_ISDN: ::windows_core::PCWSTR = ::windows_core::w!("ISDN");
pub const CONTACTLABEL_PUB_LOGO: ::windows_core::PCWSTR = ::windows_core::w!("Logo");
pub const CONTACTLABEL_PUB_MOBILE: ::windows_core::PCWSTR = ::windows_core::w!("Mobile");
pub const CONTACTLABEL_PUB_MODEM: ::windows_core::PCWSTR = ::windows_core::w!("Modem");
pub const CONTACTLABEL_PUB_OTHER: ::windows_core::PCWSTR = ::windows_core::w!("Other");
pub const CONTACTLABEL_PUB_PAGER: ::windows_core::PCWSTR = ::windows_core::w!("Pager");
pub const CONTACTLABEL_PUB_PARCEL: ::windows_core::PCWSTR = ::windows_core::w!("Parcel");
pub const CONTACTLABEL_PUB_PCS: ::windows_core::PCWSTR = ::windows_core::w!("PCS");
pub const CONTACTLABEL_PUB_PERSONAL: ::windows_core::PCWSTR = ::windows_core::w!("Personal");
pub const CONTACTLABEL_PUB_POSTAL: ::windows_core::PCWSTR = ::windows_core::w!("Postal");
pub const CONTACTLABEL_PUB_PREFERRED: ::windows_core::PCWSTR = ::windows_core::w!("Preferred");
pub const CONTACTLABEL_PUB_TTY: ::windows_core::PCWSTR = ::windows_core::w!("TTY");
pub const CONTACTLABEL_PUB_USERTILE: ::windows_core::PCWSTR = ::windows_core::w!("UserTile");
pub const CONTACTLABEL_PUB_VIDEO: ::windows_core::PCWSTR = ::windows_core::w!("Video");
pub const CONTACTLABEL_PUB_VOICE: ::windows_core::PCWSTR = ::windows_core::w!("Voice");
pub const CONTACTLABEL_WAB_ANNIVERSARY: ::windows_core::PCWSTR = ::windows_core::w!("wab:Anniversary");
pub const CONTACTLABEL_WAB_ASSISTANT: ::windows_core::PCWSTR = ::windows_core::w!("wab:Assistant");
pub const CONTACTLABEL_WAB_BIRTHDAY: ::windows_core::PCWSTR = ::windows_core::w!("wab:Birthday");
pub const CONTACTLABEL_WAB_CHILD: ::windows_core::PCWSTR = ::windows_core::w!("wab:Child");
pub const CONTACTLABEL_WAB_MANAGER: ::windows_core::PCWSTR = ::windows_core::w!("wab:Manager");
pub const CONTACTLABEL_WAB_SCHOOL: ::windows_core::PCWSTR = ::windows_core::w!("wab:School");
pub const CONTACTLABEL_WAB_SOCIALNETWORK: ::windows_core::PCWSTR = ::windows_core::w!("wab:SocialNetwork");
pub const CONTACTLABEL_WAB_SPOUSE: ::windows_core::PCWSTR = ::windows_core::w!("wab:Spouse");
pub const CONTACTLABEL_WAB_WISHLIST: ::windows_core::PCWSTR = ::windows_core::w!("wab:WishList");
pub const CONTACTPROP_PUB_CREATIONDATE: ::windows_core::PCWSTR = ::windows_core::w!("CreationDate");
pub const CONTACTPROP_PUB_GENDER: ::windows_core::PCWSTR = ::windows_core::w!("Gender");
pub const CONTACTPROP_PUB_GENDER_FEMALE: ::windows_core::PCWSTR = ::windows_core::w!("Female");
pub const CONTACTPROP_PUB_GENDER_MALE: ::windows_core::PCWSTR = ::windows_core::w!("Male");
pub const CONTACTPROP_PUB_GENDER_UNSPECIFIED: ::windows_core::PCWSTR = ::windows_core::w!("Unspecified");
pub const CONTACTPROP_PUB_L1_CERTIFICATECOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("CertificateCollection");
pub const CONTACTPROP_PUB_L1_CONTACTIDCOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("ContactIDCollection");
pub const CONTACTPROP_PUB_L1_DATECOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("DateCollection");
pub const CONTACTPROP_PUB_L1_EMAILADDRESSCOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("EmailAddressCollection");
pub const CONTACTPROP_PUB_L1_IMADDRESSCOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("IMAddressCollection");
pub const CONTACTPROP_PUB_L1_NAMECOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("NameCollection");
pub const CONTACTPROP_PUB_L1_PERSONCOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("PersonCollection");
pub const CONTACTPROP_PUB_L1_PHONENUMBERCOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("PhoneNumberCollection");
pub const CONTACTPROP_PUB_L1_PHOTOCOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("PhotoCollection");
pub const CONTACTPROP_PUB_L1_PHYSICALADDRESSCOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("PhysicalAddressCollection");
pub const CONTACTPROP_PUB_L1_POSITIONCOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("PositionCollection");
pub const CONTACTPROP_PUB_L1_URLCOLLECTION: ::windows_core::PCWSTR = ::windows_core::w!("UrlCollection");
pub const CONTACTPROP_PUB_L2_CERTIFICATE: ::windows_core::PCWSTR = ::windows_core::w!("/Certificate");
pub const CONTACTPROP_PUB_L2_CONTACTID: ::windows_core::PCWSTR = ::windows_core::w!("/ContactID");
pub const CONTACTPROP_PUB_L2_DATE: ::windows_core::PCWSTR = ::windows_core::w!("/Date");
pub const CONTACTPROP_PUB_L2_EMAILADDRESS: ::windows_core::PCWSTR = ::windows_core::w!("/EmailAddress");
pub const CONTACTPROP_PUB_L2_IMADDRESSENTRY: ::windows_core::PCWSTR = ::windows_core::w!("/IMAddress");
pub const CONTACTPROP_PUB_L2_NAME: ::windows_core::PCWSTR = ::windows_core::w!("/Name");
pub const CONTACTPROP_PUB_L2_PERSON: ::windows_core::PCWSTR = ::windows_core::w!("/Person");
pub const CONTACTPROP_PUB_L2_PHONENUMBER: ::windows_core::PCWSTR = ::windows_core::w!("/PhoneNumber");
pub const CONTACTPROP_PUB_L2_PHOTO: ::windows_core::PCWSTR = ::windows_core::w!("/Photo");
pub const CONTACTPROP_PUB_L2_PHYSICALADDRESS: ::windows_core::PCWSTR = ::windows_core::w!("/PhysicalAddress");
pub const CONTACTPROP_PUB_L2_POSITION: ::windows_core::PCWSTR = ::windows_core::w!("/Position");
pub const CONTACTPROP_PUB_L2_URL: ::windows_core::PCWSTR = ::windows_core::w!("/Url");
pub const CONTACTPROP_PUB_L3_ADDRESS: ::windows_core::PCWSTR = ::windows_core::w!("/Address");
pub const CONTACTPROP_PUB_L3_ADDRESSLABEL: ::windows_core::PCWSTR = ::windows_core::w!("/AddressLabel");
pub const CONTACTPROP_PUB_L3_ALTERNATE: ::windows_core::PCWSTR = ::windows_core::w!("/Alternate");
pub const CONTACTPROP_PUB_L3_COMPANY: ::windows_core::PCWSTR = ::windows_core::w!("/Company");
pub const CONTACTPROP_PUB_L3_COUNTRY: ::windows_core::PCWSTR = ::windows_core::w!("/Country");
pub const CONTACTPROP_PUB_L3_DEPARTMENT: ::windows_core::PCWSTR = ::windows_core::w!("/Department");
pub const CONTACTPROP_PUB_L3_EXTENDEDADDRESS: ::windows_core::PCWSTR = ::windows_core::w!("/ExtendedAddress");
pub const CONTACTPROP_PUB_L3_FAMILYNAME: ::windows_core::PCWSTR = ::windows_core::w!("/FamilyName");
pub const CONTACTPROP_PUB_L3_FORMATTEDNAME: ::windows_core::PCWSTR = ::windows_core::w!("/FormattedName");
pub const CONTACTPROP_PUB_L3_GENERATION: ::windows_core::PCWSTR = ::windows_core::w!("/Generation");
pub const CONTACTPROP_PUB_L3_GIVENNAME: ::windows_core::PCWSTR = ::windows_core::w!("/GivenName");
pub const CONTACTPROP_PUB_L3_JOB_TITLE: ::windows_core::PCWSTR = ::windows_core::w!("/JobTitle");
pub const CONTACTPROP_PUB_L3_LOCALITY: ::windows_core::PCWSTR = ::windows_core::w!("/Locality");
pub const CONTACTPROP_PUB_L3_MIDDLENAME: ::windows_core::PCWSTR = ::windows_core::w!("/MiddleName");
pub const CONTACTPROP_PUB_L3_NICKNAME: ::windows_core::PCWSTR = ::windows_core::w!("/NickName");
pub const CONTACTPROP_PUB_L3_NUMBER: ::windows_core::PCWSTR = ::windows_core::w!("/Number");
pub const CONTACTPROP_PUB_L3_OFFICE: ::windows_core::PCWSTR = ::windows_core::w!("/Office");
pub const CONTACTPROP_PUB_L3_ORGANIZATION: ::windows_core::PCWSTR = ::windows_core::w!("/Organization");
pub const CONTACTPROP_PUB_L3_PERSONID: ::windows_core::PCWSTR = ::windows_core::w!("/PersonID");
pub const CONTACTPROP_PUB_L3_PHONETIC: ::windows_core::PCWSTR = ::windows_core::w!("/Phonetic");
pub const CONTACTPROP_PUB_L3_POBOX: ::windows_core::PCWSTR = ::windows_core::w!("/POBox");
pub const CONTACTPROP_PUB_L3_POSTALCODE: ::windows_core::PCWSTR = ::windows_core::w!("/PostalCode");
pub const CONTACTPROP_PUB_L3_PREFIX: ::windows_core::PCWSTR = ::windows_core::w!("/Prefix");
pub const CONTACTPROP_PUB_L3_PROFESSION: ::windows_core::PCWSTR = ::windows_core::w!("/Profession");
pub const CONTACTPROP_PUB_L3_PROTOCOL: ::windows_core::PCWSTR = ::windows_core::w!("/Protocol");
pub const CONTACTPROP_PUB_L3_REGION: ::windows_core::PCWSTR = ::windows_core::w!("/Region");
pub const CONTACTPROP_PUB_L3_ROLE: ::windows_core::PCWSTR = ::windows_core::w!("/Role");
pub const CONTACTPROP_PUB_L3_STREET: ::windows_core::PCWSTR = ::windows_core::w!("/Street");
pub const CONTACTPROP_PUB_L3_SUFFIX: ::windows_core::PCWSTR = ::windows_core::w!("/Suffix");
pub const CONTACTPROP_PUB_L3_THUMBPRINT: ::windows_core::PCWSTR = ::windows_core::w!("/ThumbPrint");
pub const CONTACTPROP_PUB_L3_TITLE: ::windows_core::PCWSTR = ::windows_core::w!("/Title");
pub const CONTACTPROP_PUB_L3_TYPE: ::windows_core::PCWSTR = ::windows_core::w!("/Type");
pub const CONTACTPROP_PUB_L3_URL: ::windows_core::PCWSTR = ::windows_core::w!("/Url");
pub const CONTACTPROP_PUB_L3_VALUE: ::windows_core::PCWSTR = ::windows_core::w!("/Value");
pub const CONTACTPROP_PUB_MAILER: ::windows_core::PCWSTR = ::windows_core::w!("Mailer");
pub const CONTACTPROP_PUB_NOTES: ::windows_core::PCWSTR = ::windows_core::w!("Notes");
pub const CONTACTPROP_PUB_PROGID: ::windows_core::PCWSTR = ::windows_core::w!("ProgID");
pub const Contact: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61b68808_8eee_4fd1_acb8_3d804c8db056);
pub const ContactManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7165c8ab_af88_42bd_86fd_5310b4285a02);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTACT_AGGREGATION_COLLECTION_OPTIONS(pub i32);
impl ::core::marker::Copy for CONTACT_AGGREGATION_COLLECTION_OPTIONS {}
impl ::core::clone::Clone for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTACT_AGGREGATION_COLLECTION_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(pub i32);
impl ::core::marker::Copy for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {}
impl ::core::clone::Clone for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct CONTACT_AGGREGATION_BLOB {
    pub dwCount: u32,
    pub lpb: *mut u8,
}
impl ::core::marker::Copy for CONTACT_AGGREGATION_BLOB {}
impl ::core::clone::Clone for CONTACT_AGGREGATION_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONTACT_AGGREGATION_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTACT_AGGREGATION_BLOB").field("dwCount", &self.dwCount).field("lpb", &self.lpb).finish()
    }
}
impl ::windows_core::TypeKind for CONTACT_AGGREGATION_BLOB {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CONTACT_AGGREGATION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwCount == other.dwCount && self.lpb == other.lpb
    }
}
impl ::core::cmp::Eq for CONTACT_AGGREGATION_BLOB {}
impl ::core::default::Default for CONTACT_AGGREGATION_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
