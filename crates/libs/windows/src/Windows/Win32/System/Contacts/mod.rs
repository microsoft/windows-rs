#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContact(::windows::core::IUnknown);
impl IContact {
    pub unsafe fn GetContactID(&self, pszcontactid: &mut [u16], pdwcchcontactidrequired: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetContactID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszcontactid.as_ptr()), pszcontactid.len() as _, pdwcchcontactidrequired).ok()
    }
    pub unsafe fn GetPath(&self, pszpath: &mut [u16], pdwcchpathrequired: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszpath.as_ptr()), pszpath.len() as _, pdwcchpathrequired).ok()
    }
    pub unsafe fn CommitChanges(&self, dwcommitflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CommitChanges)(::windows::core::Vtable::as_raw(self), dwcommitflags).ok()
    }
}
::windows::core::interface_hierarchy!(IContact, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContact {}
impl ::core::fmt::Debug for IContact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContact").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContact {
    type Vtable = IContact_Vtbl;
}
unsafe impl ::windows::core::Interface for IContact {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf941b671_bda7_4f77_884a_f46462f226a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetContactID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcontactid: ::windows::core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows::core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows::core::HRESULT,
    pub CommitChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcommitflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationAggregate(::windows::core::IUnknown);
impl IContactAggregationAggregate {
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetComponentItems(&self) -> ::windows::core::Result<IContactAggregationContactCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetComponentItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Link<P0>(&self, paggregateid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Link)(::windows::core::Vtable::as_raw(self), paggregateid.into().abi()).ok()
    }
    pub unsafe fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationGroupCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Groups)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AntiLink(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AntiLink)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAntiLink<P0>(&self, pantilink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAntiLink)(::windows::core::Vtable::as_raw(self), pantilink.into().abi()).ok()
    }
    pub unsafe fn FavoriteOrder(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FavoriteOrder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFavoriteOrder(&self, favoriteorder: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFavoriteOrder)(::windows::core::Vtable::as_raw(self), favoriteorder).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IContactAggregationAggregate, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationAggregate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationAggregate {}
impl ::core::fmt::Debug for IContactAggregationAggregate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationAggregate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationAggregate {
    type Vtable = IContactAggregationAggregate_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationAggregate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed1c814_cd30_43c8_9b8d_2e489e53d54b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationAggregate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetComponentItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomponentitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub get_Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AntiLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAntiLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pantilink: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FavoriteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT,
    pub SetFavoriteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationAggregateCollection(::windows::core::IUnknown);
impl IContactAggregationAggregateCollection {
    pub unsafe fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationAggregate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirst)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstByAntiLinkId<P0>(&self, pantilinkid: P0) -> ::windows::core::Result<IContactAggregationAggregate>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstByAntiLinkId)(::windows::core::Vtable::as_raw(self), pantilinkid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows::core::Result<IContactAggregationAggregate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IContactAggregationAggregateCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationAggregateCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationAggregateCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationAggregateCollection {}
impl ::core::fmt::Debug for IContactAggregationAggregateCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationAggregateCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationAggregateCollection {
    type Vtable = IContactAggregationAggregateCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationAggregateCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2359f3a6_3a68_40af_98db_0f9eb143c3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationAggregateCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstByAntiLinkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pantilinkid: ::windows::core::PCWSTR, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationContact(::windows::core::IUnknown);
impl IContactAggregationContact {
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MoveToAggregate<P0>(&self, paggregateid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).MoveToAggregate)(::windows::core::Vtable::as_raw(self), paggregateid.into().abi()).ok()
    }
    pub unsafe fn Unlink(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unlink)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AccountId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AccountId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccountId<P0>(&self, paccountid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAccountId)(::windows::core::Vtable::as_raw(self), paccountid.into().abi()).ok()
    }
    pub unsafe fn AggregateId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AggregateId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsMe)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsExternal(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsExternal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NetworkSourceId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NetworkSourceId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNetworkSourceId(&self, networksourceid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNetworkSourceId)(::windows::core::Vtable::as_raw(self), networksourceid).ok()
    }
    pub unsafe fn NetworkSourceIdString(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NetworkSourceIdString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNetworkSourceIdString<P0>(&self, pnetworksourceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetNetworkSourceIdString)(::windows::core::Vtable::as_raw(self), pnetworksourceid.into().abi()).ok()
    }
    pub unsafe fn RemoteObjectId(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RemoteObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRemoteObjectId)(::windows::core::Vtable::as_raw(self), premoteobjectid).ok()
    }
    pub unsafe fn SyncIdentityHash(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SyncIdentityHash)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSyncIdentityHash)(::windows::core::Vtable::as_raw(self), psyncidentityhash).ok()
    }
}
::windows::core::interface_hierarchy!(IContactAggregationContact, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationContact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationContact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationContact {}
impl ::core::fmt::Debug for IContactAggregationContact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationContact").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationContact {
    type Vtable = IContactAggregationContact_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationContact {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eb22e86_4c86_41f0_9f9f_c251e9fda6c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationContact_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveToAggregate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Unlink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaccountid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paccountid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregateid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisme: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMe: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsExternal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisexternal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsExternal: usize,
    pub NetworkSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworksourceid: *mut u32) -> ::windows::core::HRESULT,
    pub SetNetworkSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networksourceid: u32) -> ::windows::core::HRESULT,
    pub NetworkSourceIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetNetworkSourceIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworksourceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RemoteObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub SetRemoteObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub SyncIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub SetSyncIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationContactCollection(::windows::core::IUnknown);
impl IContactAggregationContactCollection {
    pub unsafe fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationContact> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirst)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows::core::Result<IContactAggregationContact> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstByIdentityHash<P0, P1>(&self, psourcetype: P0, paccountid: P1, pidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<IContactAggregationContact>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstByIdentityHash)(::windows::core::Vtable::as_raw(self), psourcetype.into().abi(), paccountid.into().abi(), pidentityhash, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstByRemoteId<P0, P1>(&self, psourcetype: P0, paccountid: P1, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<IContactAggregationContact>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstByRemoteId)(::windows::core::Vtable::as_raw(self), psourcetype.into().abi(), paccountid.into().abi(), premoteobjectid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IContactAggregationContactCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationContactCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationContactCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationContactCollection {}
impl ::core::fmt::Debug for IContactAggregationContactCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationContactCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationContactCollection {
    type Vtable = IContactAggregationContactCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationContactCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x826e66fa_81de_43ca_a6fb_8c785cd996c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationContactCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstByIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcetype: ::windows::core::PCWSTR, paccountid: ::windows::core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub FindFirstByRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcetype: ::windows::core::PCWSTR, paccountid: ::windows::core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationGroup(::windows::core::IUnknown);
impl IContactAggregationGroup {
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Add<P0>(&self, paggregateid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), paggregateid.into().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, paggregateid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), paggregateid.into().abi()).ok()
    }
    pub unsafe fn Members(&self) -> ::windows::core::Result<IContactAggregationAggregateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Members)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GlobalObjectId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GlobalObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGlobalObjectId(&self, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGlobalObjectId)(::windows::core::Vtable::as_raw(self), pglobalobjectid).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, pname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), pname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IContactAggregationGroup, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationGroup {}
impl ::core::fmt::Debug for IContactAggregationGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationGroup {
    type Vtable = IContactAggregationGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc93c545f_1284_499b_96af_07372af473e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationGroup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregatecontactcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GlobalObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pglobalobjectid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetGlobalObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationGroupCollection(::windows::core::IUnknown);
impl IContactAggregationGroupCollection {
    pub unsafe fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirst)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstByGlobalObjectId(&self, pglobalobjectid: *const ::windows::core::GUID) -> ::windows::core::Result<IContactAggregationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstByGlobalObjectId)(::windows::core::Vtable::as_raw(self), pglobalobjectid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows::core::Result<IContactAggregationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IContactAggregationGroupCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationGroupCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationGroupCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationGroupCollection {}
impl ::core::fmt::Debug for IContactAggregationGroupCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationGroupCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationGroupCollection {
    type Vtable = IContactAggregationGroupCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationGroupCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20a19a9c_d2f3_4b83_9143_beffd2cc226d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationGroupCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstByGlobalObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows::core::GUID, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationLink(::windows::core::IUnknown);
impl IContactAggregationLink {
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AccountId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AccountId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccountId<P0>(&self, paccountid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAccountId)(::windows::core::Vtable::as_raw(self), paccountid.into().abi()).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLinkResolved(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsLinkResolved)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsLinkResolved<P0>(&self, islinkresolved: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsLinkResolved)(::windows::core::Vtable::as_raw(self), islinkresolved.into()).ok()
    }
    pub unsafe fn NetworkSourceIdString(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NetworkSourceIdString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNetworkSourceIdString<P0>(&self, pnetworksourceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetNetworkSourceIdString)(::windows::core::Vtable::as_raw(self), pnetworksourceid.into().abi()).ok()
    }
    pub unsafe fn RemoteObjectId(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RemoteObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRemoteObjectId)(::windows::core::Vtable::as_raw(self), premoteobjectid).ok()
    }
    pub unsafe fn ServerPerson(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServerPerson)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServerPerson<P0>(&self, pserverpersonid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetServerPerson)(::windows::core::Vtable::as_raw(self), pserverpersonid.into().abi()).ok()
    }
    pub unsafe fn ServerPersonBaseline(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServerPersonBaseline)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServerPersonBaseline<P0>(&self, pserverpersonid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetServerPersonBaseline)(::windows::core::Vtable::as_raw(self), pserverpersonid.into().abi()).ok()
    }
    pub unsafe fn SyncIdentityHash(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SyncIdentityHash)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSyncIdentityHash)(::windows::core::Vtable::as_raw(self), psyncidentityhash).ok()
    }
}
::windows::core::interface_hierarchy!(IContactAggregationLink, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationLink {}
impl ::core::fmt::Debug for IContactAggregationLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationLink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationLink {
    type Vtable = IContactAggregationLink_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationLink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6813323_a183_4654_8627_79b30de3a0ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationLink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaccountid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paccountid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLinkResolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislinkresolved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLinkResolved: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsLinkResolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, islinkresolved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsLinkResolved: usize,
    pub NetworkSourceIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetNetworkSourceIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworksourceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RemoteObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub SetRemoteObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub ServerPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetServerPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserverpersonid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ServerPersonBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetServerPersonBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserverpersonid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SyncIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub SetSyncIdentityHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationLinkCollection(::windows::core::IUnknown);
impl IContactAggregationLinkCollection {
    pub unsafe fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationLink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirst)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstByRemoteId<P0, P1>(&self, psourcetype: P0, paccountid: P1, premoteid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<IContactAggregationLink>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstByRemoteId)(::windows::core::Vtable::as_raw(self), psourcetype.into().abi(), paccountid.into().abi(), premoteid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows::core::Result<IContactAggregationLink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IContactAggregationLinkCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationLinkCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationLinkCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationLinkCollection {}
impl ::core::fmt::Debug for IContactAggregationLinkCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationLinkCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationLinkCollection {
    type Vtable = IContactAggregationLinkCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationLinkCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8bc0e93_fb55_4f28_b9fa_b1c274153292);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationLinkCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstByRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcetype: ::windows::core::PCWSTR, paccountid: ::windows::core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationManager(::windows::core::IUnknown);
impl IContactAggregationManager {
    pub unsafe fn GetVersionInfo(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetVersionInfo)(::windows::core::Vtable::as_raw(self), plmajorversion, plminorversion).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOrOpenGroup<P0>(&self, pgroupname: P0, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut ::core::option::Option<IContactAggregationGroup>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).CreateOrOpenGroup)(::windows::core::Vtable::as_raw(self), pgroupname.into().abi(), options, pcreatedgroup, ::core::mem::transmute(ppgroup)).ok()
    }
    pub unsafe fn CreateExternalContact(&self) -> ::windows::core::Result<IContactAggregationContact> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateExternalContact)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateServerPerson(&self) -> ::windows::core::Result<IContactAggregationServerPerson> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateServerPerson)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateServerContactLink(&self) -> ::windows::core::Result<IContactAggregationLink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateServerContactLink)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Flush)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OpenAggregateContact<P0>(&self, pitemid: P0) -> ::windows::core::Result<IContactAggregationAggregate>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenAggregateContact)(::windows::core::Vtable::as_raw(self), pitemid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenContact<P0>(&self, pitemid: P0) -> ::windows::core::Result<IContactAggregationContact>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenContact)(::windows::core::Vtable::as_raw(self), pitemid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenServerContactLink<P0>(&self, pitemid: P0) -> ::windows::core::Result<IContactAggregationLink>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenServerContactLink)(::windows::core::Vtable::as_raw(self), pitemid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenServerPerson<P0>(&self, pitemid: P0) -> ::windows::core::Result<IContactAggregationServerPerson>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenServerPerson)(::windows::core::Vtable::as_raw(self), pitemid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Contacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationContactCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Contacts)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_AggregateContacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationAggregateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_AggregateContacts)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::core::Result<IContactAggregationGroupCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Groups)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ServerPersons(&self) -> ::windows::core::Result<IContactAggregationServerPersonCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServerPersons)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_ServerContactLinks<P0>(&self, ppersonitemid: P0) -> ::windows::core::Result<IContactAggregationLinkCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ServerContactLinks)(::windows::core::Vtable::as_raw(self), ppersonitemid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IContactAggregationManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationManager {}
impl ::core::fmt::Debug for IContactAggregationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationManager {
    type Vtable = IContactAggregationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d865989_4b1f_4b60_8f34_c2ad468b2b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateOrOpenGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroupname: ::windows::core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateOrOpenGroup: usize,
    pub CreateExternalContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateServerPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateServerContactLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenAggregateContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: ::windows::core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: ::windows::core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenServerContactLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: ::windows::core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenServerPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: ::windows::core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_Contacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_AggregateContacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppaggregates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ServerPersons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverpersoncollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_ServerContactLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppersonitemid: ::windows::core::PCWSTR, ppservercontactlinkcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationServerPerson(::windows::core::IUnknown);
impl IContactAggregationServerPerson {
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AggregateId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AggregateId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAggregateId<P0>(&self, paggregateid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAggregateId)(::windows::core::Vtable::as_raw(self), paggregateid.into().abi()).ok()
    }
    pub unsafe fn AntiLink(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AntiLink)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAntiLink<P0>(&self, pantilink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAntiLink)(::windows::core::Vtable::as_raw(self), pantilink.into().abi()).ok()
    }
    pub unsafe fn AntiLinkBaseline(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AntiLinkBaseline)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAntiLinkBaseline<P0>(&self, pantilink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAntiLinkBaseline)(::windows::core::Vtable::as_raw(self), pantilink.into().abi()).ok()
    }
    pub unsafe fn FavoriteOrder(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FavoriteOrder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFavoriteOrder(&self, favoriteorder: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFavoriteOrder)(::windows::core::Vtable::as_raw(self), favoriteorder).ok()
    }
    pub unsafe fn FavoriteOrderBaseline(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FavoriteOrderBaseline)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFavoriteOrderBaseline(&self, favoriteorder: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFavoriteOrderBaseline)(::windows::core::Vtable::as_raw(self), favoriteorder).ok()
    }
    pub unsafe fn Groups(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Groups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGroups(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGroups)(::windows::core::Vtable::as_raw(self), pgroups).ok()
    }
    pub unsafe fn GroupsBaseline(&self) -> ::windows::core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GroupsBaseline)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGroupsBaseline(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGroupsBaseline)(::windows::core::Vtable::as_raw(self), pgroups).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTombstone(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsTombstone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsTombstone<P0>(&self, istombstone: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsTombstone)(::windows::core::Vtable::as_raw(self), istombstone.into()).ok()
    }
    pub unsafe fn LinkedAggregateId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LinkedAggregateId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLinkedAggregateId<P0>(&self, plinkedaggregateid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetLinkedAggregateId)(::windows::core::Vtable::as_raw(self), plinkedaggregateid.into().abi()).ok()
    }
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetObjectId<P0>(&self, pobjectid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetObjectId)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IContactAggregationServerPerson, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationServerPerson {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationServerPerson {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationServerPerson {}
impl ::core::fmt::Debug for IContactAggregationServerPerson {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationServerPerson").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationServerPerson {
    type Vtable = IContactAggregationServerPerson_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationServerPerson {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fdc3d4b_1b82_4334_85c5_25184ee5a5f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationServerPerson_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaggregateid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AntiLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAntiLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pantilink: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AntiLinkBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAntiLinkBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pantilink: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FavoriteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT,
    pub SetFavoriteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT,
    pub FavoriteOrderBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows::core::HRESULT,
    pub SetFavoriteOrderBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows::core::HRESULT,
    pub Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub SetGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub GroupsBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub SetGroupsBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTombstone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistombstone: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTombstone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsTombstone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istombstone: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsTombstone: usize,
    pub LinkedAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplinkedaggregateid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLinkedAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plinkedaggregateid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactAggregationServerPersonCollection(::windows::core::IUnknown);
impl IContactAggregationServerPersonCollection {
    pub unsafe fn FindFirst(&self) -> ::windows::core::Result<IContactAggregationServerPerson> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirst)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstByServerId<P0>(&self, pserverid: P0) -> ::windows::core::Result<IContactAggregationServerPerson>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstByServerId)(::windows::core::Vtable::as_raw(self), pserverid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstByAggregateId<P0>(&self, paggregateid: P0) -> ::windows::core::Result<IContactAggregationServerPerson>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstByAggregateId)(::windows::core::Vtable::as_raw(self), paggregateid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstByLinkedAggregateId<P0>(&self, paggregateid: P0) -> ::windows::core::Result<IContactAggregationServerPerson>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstByLinkedAggregateId)(::windows::core::Vtable::as_raw(self), paggregateid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindNext(&self) -> ::windows::core::Result<IContactAggregationServerPerson> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IContactAggregationServerPersonCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactAggregationServerPersonCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactAggregationServerPersonCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationServerPersonCollection {}
impl ::core::fmt::Debug for IContactAggregationServerPersonCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationServerPersonCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactAggregationServerPersonCollection {
    type Vtable = IContactAggregationServerPersonCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAggregationServerPersonCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f730a4a_6604_47b6_a987_669ecf1e5751);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationServerPersonCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstByServerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserverid: ::windows::core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstByAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstByLinkedAggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregateid: ::windows::core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactCollection(::windows::core::IUnknown);
impl IContactCollection {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IContact> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IContactCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactCollection {}
impl ::core::fmt::Debug for IContactCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactCollection {
    type Vtable = IContactCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6afa338_d779_11d9_8bde_f66bad1e3f3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontact: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactManager(::windows::core::IUnknown);
impl IContactManager {
    pub unsafe fn Initialize<P0, P1>(&self, pszappname: P0, pszappversion: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), pszappname.into().abi(), pszappversion.into().abi()).ok()
    }
    pub unsafe fn Load<P0>(&self, pszcontactid: P0) -> ::windows::core::Result<IContact>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), pszcontactid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MergeContactIDs<P0, P1>(&self, psznewcontactid: P0, pszoldcontactid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).MergeContactIDs)(::windows::core::Vtable::as_raw(self), psznewcontactid.into().abi(), pszoldcontactid.into().abi()).ok()
    }
    pub unsafe fn GetMeContact(&self) -> ::windows::core::Result<IContact> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMeContact)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMeContact<P0>(&self, pmecontact: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IContact>>,
    {
        (::windows::core::Vtable::vtable(self).SetMeContact)(::windows::core::Vtable::as_raw(self), pmecontact.into().abi()).ok()
    }
    pub unsafe fn GetContactCollection(&self) -> ::windows::core::Result<IContactCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContactCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IContactManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactManager {}
impl ::core::fmt::Debug for IContactManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactManager {
    type Vtable = IContactManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad553d98_deb1_474a_8e17_fc0c2075b738);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszappname: ::windows::core::PCWSTR, pszappversion: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcontactid: ::windows::core::PCWSTR, ppcontact: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MergeContactIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psznewcontactid: ::windows::core::PCWSTR, pszoldcontactid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetMeContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmecontact: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMeContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmecontact: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContactCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontactcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactProperties(::windows::core::IUnknown);
impl IContactProperties {
    pub unsafe fn GetString<P0>(&self, pszpropertyname: P0, dwflags: u32, pszvalue: &mut [u16], pdwcchpropertyvaluerequired: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetString)(::windows::core::Vtable::as_raw(self), pszpropertyname.into().abi(), dwflags, ::core::mem::transmute(pszvalue.as_ptr()), pszvalue.len() as _, pdwcchpropertyvaluerequired).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDate<P0>(&self, pszpropertyname: P0, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetDate)(::windows::core::Vtable::as_raw(self), pszpropertyname.into().abi(), dwflags, pftdatetime).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBinary<P0>(&self, pszpropertyname: P0, dwflags: u32, pszcontenttype: &mut [u16], pdwcchcontenttyperequired: *mut u32, ppstream: *mut ::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetBinary)(::windows::core::Vtable::as_raw(self), pszpropertyname.into().abi(), dwflags, ::core::mem::transmute(pszcontenttype.as_ptr()), pszcontenttype.len() as _, pdwcchcontenttyperequired, ::core::mem::transmute(ppstream)).ok()
    }
    pub unsafe fn GetLabels<P0>(&self, pszarrayelementname: P0, dwflags: u32, pszlabels: &mut [u16], pdwcchlabelsrequired: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetLabels)(::windows::core::Vtable::as_raw(self), pszarrayelementname.into().abi(), dwflags, ::core::mem::transmute(pszlabels.as_ptr()), pszlabels.len() as _, pdwcchlabelsrequired).ok()
    }
    pub unsafe fn SetString<P0, P1>(&self, pszpropertyname: P0, dwflags: u32, pszvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetString)(::windows::core::Vtable::as_raw(self), pszpropertyname.into().abi(), dwflags, pszvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDate<P0>(&self, pszpropertyname: P0, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetDate)(::windows::core::Vtable::as_raw(self), pszpropertyname.into().abi(), dwflags, ::core::mem::transmute(ftdatetime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBinary<P0, P1, P2>(&self, pszpropertyname: P0, dwflags: u32, pszcontenttype: P1, pstream: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).SetBinary)(::windows::core::Vtable::as_raw(self), pszpropertyname.into().abi(), dwflags, pszcontenttype.into().abi(), pstream.into().abi()).ok()
    }
    pub unsafe fn SetLabels<P0>(&self, pszarrayelementname: P0, dwflags: u32, ppszlabels: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetLabels)(::windows::core::Vtable::as_raw(self), pszarrayelementname.into().abi(), dwflags, ppszlabels.len() as _, ::core::mem::transmute(ppszlabels.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateArrayNode<P0, P1>(&self, pszarrayname: P0, dwflags: u32, fappend: P1, psznewarrayelementname: &mut [u16], pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).CreateArrayNode)(::windows::core::Vtable::as_raw(self), pszarrayname.into().abi(), dwflags, fappend.into(), ::core::mem::transmute(psznewarrayelementname.as_ptr()), psznewarrayelementname.len() as _, pdwcchnewarrayelementnamerequired).ok()
    }
    pub unsafe fn DeleteProperty<P0>(&self, pszpropertyname: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteProperty)(::windows::core::Vtable::as_raw(self), pszpropertyname.into().abi(), dwflags).ok()
    }
    pub unsafe fn DeleteArrayNode<P0>(&self, pszarrayelementname: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteArrayNode)(::windows::core::Vtable::as_raw(self), pszarrayelementname.into().abi(), dwflags).ok()
    }
    pub unsafe fn DeleteLabels<P0>(&self, pszarrayelementname: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteLabels)(::windows::core::Vtable::as_raw(self), pszarrayelementname.into().abi(), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyCollection<P0, P1>(&self, pppropertycollection: *mut ::core::option::Option<IContactPropertyCollection>, dwflags: u32, pszmultivaluename: P0, ppszlabels: &[::windows::core::PCWSTR], fanylabelmatches: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).GetPropertyCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppropertycollection), dwflags, pszmultivaluename.into().abi(), ppszlabels.len() as _, ::core::mem::transmute(ppszlabels.as_ptr()), fanylabelmatches.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IContactProperties, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactProperties {}
impl ::core::fmt::Debug for IContactProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactProperties {
    type Vtable = IContactProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70dd27dd_5cbd_46e8_bef0_23b6b346288f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pszvalue: ::windows::core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pszcontenttype: ::windows::core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBinary: usize,
    pub GetLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows::core::PCWSTR, dwflags: u32, pszlabels: ::windows::core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows::core::HRESULT,
    pub SetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32, pszcontenttype: ::windows::core::PCWSTR, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBinary: usize,
    pub SetLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows::core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateArrayNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayname: ::windows::core::PCWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: ::windows::core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateArrayNode: usize,
    pub DeleteProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub DeleteArrayNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub DeleteLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertycollection: *mut *mut ::core::ffi::c_void, dwflags: u32, pszmultivaluename: ::windows::core::PCWSTR, dwlabelcount: u32, ppszlabels: *const ::windows::core::PCWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyCollection: usize,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactPropertyCollection(::windows::core::IUnknown);
impl IContactPropertyCollection {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetPropertyName(&self, pszpropertyname: &mut [u16], pdwcchpropertynamerequired: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszpropertyname.as_ptr()), pszpropertyname.len() as _, pdwcchpropertynamerequired).ok()
    }
    pub unsafe fn GetPropertyType(&self, pdwtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyType)(::windows::core::Vtable::as_raw(self), pdwtype).ok()
    }
    pub unsafe fn GetPropertyVersion(&self, pdwversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyVersion)(::windows::core::Vtable::as_raw(self), pdwversion).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyModificationDate(&self, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyModificationDate)(::windows::core::Vtable::as_raw(self), pftmodificationdate).ok()
    }
    pub unsafe fn GetPropertyArrayElementID(&self, pszarrayelementid: &mut [u16], pdwccharrayelementidrequired: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyArrayElementID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszarrayelementid.as_ptr()), pszarrayelementid.len() as _, pdwccharrayelementidrequired).ok()
    }
}
::windows::core::interface_hierarchy!(IContactPropertyCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContactPropertyCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPropertyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPropertyCollection {}
impl ::core::fmt::Debug for IContactPropertyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPropertyCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContactPropertyCollection {
    type Vtable = IContactPropertyCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPropertyCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffd3adf8_fa64_4328_b1b6_2e0db509cb3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPropertyCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows::core::HRESULT,
    pub GetPropertyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    pub GetPropertyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyModificationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyModificationDate: usize,
    pub GetPropertyArrayElementID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszarrayelementid: ::windows::core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CGD_ARRAY_NODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CGD_BINARY_PROPERTY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CGD_DATE_PROPERTY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CGD_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CGD_STRING_PROPERTY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CGD_UNKNOWN_PROPERTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CLSID_ContactAggregationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96c8ad95_c199_44de_b34e_ac33c442df39);
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_AGENT: ::windows::core::PCWSTR = ::windows::w!("Agent");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_BBS: ::windows::core::PCWSTR = ::windows::w!("BBS");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_BUSINESS: ::windows::core::PCWSTR = ::windows::w!("Business");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_CAR: ::windows::core::PCWSTR = ::windows::w!("Car");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_CELLULAR: ::windows::core::PCWSTR = ::windows::w!("Cellular");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_DOMESTIC: ::windows::core::PCWSTR = ::windows::w!("Domestic");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_FAX: ::windows::core::PCWSTR = ::windows::w!("Fax");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_INTERNATIONAL: ::windows::core::PCWSTR = ::windows::w!("International");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_ISDN: ::windows::core::PCWSTR = ::windows::w!("ISDN");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_LOGO: ::windows::core::PCWSTR = ::windows::w!("Logo");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_MOBILE: ::windows::core::PCWSTR = ::windows::w!("Mobile");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_MODEM: ::windows::core::PCWSTR = ::windows::w!("Modem");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_OTHER: ::windows::core::PCWSTR = ::windows::w!("Other");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_PAGER: ::windows::core::PCWSTR = ::windows::w!("Pager");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_PARCEL: ::windows::core::PCWSTR = ::windows::w!("Parcel");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_PCS: ::windows::core::PCWSTR = ::windows::w!("PCS");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_PERSONAL: ::windows::core::PCWSTR = ::windows::w!("Personal");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_POSTAL: ::windows::core::PCWSTR = ::windows::w!("Postal");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_PREFERRED: ::windows::core::PCWSTR = ::windows::w!("Preferred");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_TTY: ::windows::core::PCWSTR = ::windows::w!("TTY");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_USERTILE: ::windows::core::PCWSTR = ::windows::w!("UserTile");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_VIDEO: ::windows::core::PCWSTR = ::windows::w!("Video");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_PUB_VOICE: ::windows::core::PCWSTR = ::windows::w!("Voice");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_WAB_ANNIVERSARY: ::windows::core::PCWSTR = ::windows::w!("wab:Anniversary");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_WAB_ASSISTANT: ::windows::core::PCWSTR = ::windows::w!("wab:Assistant");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_WAB_BIRTHDAY: ::windows::core::PCWSTR = ::windows::w!("wab:Birthday");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_WAB_CHILD: ::windows::core::PCWSTR = ::windows::w!("wab:Child");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_WAB_MANAGER: ::windows::core::PCWSTR = ::windows::w!("wab:Manager");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_WAB_SCHOOL: ::windows::core::PCWSTR = ::windows::w!("wab:School");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_WAB_SOCIALNETWORK: ::windows::core::PCWSTR = ::windows::w!("wab:SocialNetwork");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_WAB_SPOUSE: ::windows::core::PCWSTR = ::windows::w!("wab:Spouse");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTLABEL_WAB_WISHLIST: ::windows::core::PCWSTR = ::windows::w!("wab:WishList");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_CREATIONDATE: ::windows::core::PCWSTR = ::windows::w!("CreationDate");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_GENDER: ::windows::core::PCWSTR = ::windows::w!("Gender");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_GENDER_FEMALE: ::windows::core::PCWSTR = ::windows::w!("Female");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_GENDER_MALE: ::windows::core::PCWSTR = ::windows::w!("Male");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_GENDER_UNSPECIFIED: ::windows::core::PCWSTR = ::windows::w!("Unspecified");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_CERTIFICATECOLLECTION: ::windows::core::PCWSTR = ::windows::w!("CertificateCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_CONTACTIDCOLLECTION: ::windows::core::PCWSTR = ::windows::w!("ContactIDCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_DATECOLLECTION: ::windows::core::PCWSTR = ::windows::w!("DateCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_EMAILADDRESSCOLLECTION: ::windows::core::PCWSTR = ::windows::w!("EmailAddressCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_IMADDRESSCOLLECTION: ::windows::core::PCWSTR = ::windows::w!("IMAddressCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_NAMECOLLECTION: ::windows::core::PCWSTR = ::windows::w!("NameCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_PERSONCOLLECTION: ::windows::core::PCWSTR = ::windows::w!("PersonCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_PHONENUMBERCOLLECTION: ::windows::core::PCWSTR = ::windows::w!("PhoneNumberCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_PHOTOCOLLECTION: ::windows::core::PCWSTR = ::windows::w!("PhotoCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_PHYSICALADDRESSCOLLECTION: ::windows::core::PCWSTR = ::windows::w!("PhysicalAddressCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_POSITIONCOLLECTION: ::windows::core::PCWSTR = ::windows::w!("PositionCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L1_URLCOLLECTION: ::windows::core::PCWSTR = ::windows::w!("UrlCollection");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_CERTIFICATE: ::windows::core::PCWSTR = ::windows::w!("/Certificate");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_CONTACTID: ::windows::core::PCWSTR = ::windows::w!("/ContactID");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_DATE: ::windows::core::PCWSTR = ::windows::w!("/Date");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_EMAILADDRESS: ::windows::core::PCWSTR = ::windows::w!("/EmailAddress");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_IMADDRESSENTRY: ::windows::core::PCWSTR = ::windows::w!("/IMAddress");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_NAME: ::windows::core::PCWSTR = ::windows::w!("/Name");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_PERSON: ::windows::core::PCWSTR = ::windows::w!("/Person");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_PHONENUMBER: ::windows::core::PCWSTR = ::windows::w!("/PhoneNumber");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_PHOTO: ::windows::core::PCWSTR = ::windows::w!("/Photo");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_PHYSICALADDRESS: ::windows::core::PCWSTR = ::windows::w!("/PhysicalAddress");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_POSITION: ::windows::core::PCWSTR = ::windows::w!("/Position");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L2_URL: ::windows::core::PCWSTR = ::windows::w!("/Url");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("/Address");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_ADDRESSLABEL: ::windows::core::PCWSTR = ::windows::w!("/AddressLabel");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_ALTERNATE: ::windows::core::PCWSTR = ::windows::w!("/Alternate");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_COMPANY: ::windows::core::PCWSTR = ::windows::w!("/Company");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_COUNTRY: ::windows::core::PCWSTR = ::windows::w!("/Country");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_DEPARTMENT: ::windows::core::PCWSTR = ::windows::w!("/Department");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_EXTENDEDADDRESS: ::windows::core::PCWSTR = ::windows::w!("/ExtendedAddress");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_FAMILYNAME: ::windows::core::PCWSTR = ::windows::w!("/FamilyName");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_FORMATTEDNAME: ::windows::core::PCWSTR = ::windows::w!("/FormattedName");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_GENERATION: ::windows::core::PCWSTR = ::windows::w!("/Generation");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_GIVENNAME: ::windows::core::PCWSTR = ::windows::w!("/GivenName");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_JOB_TITLE: ::windows::core::PCWSTR = ::windows::w!("/JobTitle");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_LOCALITY: ::windows::core::PCWSTR = ::windows::w!("/Locality");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_MIDDLENAME: ::windows::core::PCWSTR = ::windows::w!("/MiddleName");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_NICKNAME: ::windows::core::PCWSTR = ::windows::w!("/NickName");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_NUMBER: ::windows::core::PCWSTR = ::windows::w!("/Number");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_OFFICE: ::windows::core::PCWSTR = ::windows::w!("/Office");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_ORGANIZATION: ::windows::core::PCWSTR = ::windows::w!("/Organization");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_PERSONID: ::windows::core::PCWSTR = ::windows::w!("/PersonID");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_PHONETIC: ::windows::core::PCWSTR = ::windows::w!("/Phonetic");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_POBOX: ::windows::core::PCWSTR = ::windows::w!("/POBox");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_POSTALCODE: ::windows::core::PCWSTR = ::windows::w!("/PostalCode");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_PREFIX: ::windows::core::PCWSTR = ::windows::w!("/Prefix");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_PROFESSION: ::windows::core::PCWSTR = ::windows::w!("/Profession");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_PROTOCOL: ::windows::core::PCWSTR = ::windows::w!("/Protocol");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_REGION: ::windows::core::PCWSTR = ::windows::w!("/Region");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_ROLE: ::windows::core::PCWSTR = ::windows::w!("/Role");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_STREET: ::windows::core::PCWSTR = ::windows::w!("/Street");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_SUFFIX: ::windows::core::PCWSTR = ::windows::w!("/Suffix");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_THUMBPRINT: ::windows::core::PCWSTR = ::windows::w!("/ThumbPrint");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_TITLE: ::windows::core::PCWSTR = ::windows::w!("/Title");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_TYPE: ::windows::core::PCWSTR = ::windows::w!("/Type");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_URL: ::windows::core::PCWSTR = ::windows::w!("/Url");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_L3_VALUE: ::windows::core::PCWSTR = ::windows::w!("/Value");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_MAILER: ::windows::core::PCWSTR = ::windows::w!("Mailer");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_NOTES: ::windows::core::PCWSTR = ::windows::w!("Notes");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CONTACTPROP_PUB_PROGID: ::windows::core::PCWSTR = ::windows::w!("ProgID");
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const Contact: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61b68808_8eee_4fd1_acb8_3d804c8db056);
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const ContactManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7165c8ab_af88_42bd_86fd_5310b4285a02);
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTACT_AGGREGATION_COLLECTION_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CACO_DEFAULT: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CACO_INCLUDE_EXTERNAL: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CACO_EXTERNAL_ONLY: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(2i32);
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
unsafe impl ::windows::core::Abi for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTACT_AGGREGATION_COLLECTION_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CA_CREATE_LOCAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
pub const CA_CREATE_EXTERNAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(1i32);
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
unsafe impl ::windows::core::Abi for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Contacts\"`*"]
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
unsafe impl ::windows::core::Abi for CONTACT_AGGREGATION_BLOB {
    type Abi = Self;
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
