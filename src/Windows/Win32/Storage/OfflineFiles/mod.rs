#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumOfflineFilesItems(pub ::windows::runtime::IUnknown);
impl IEnumOfflineFilesItems {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::std::option::Option<IOfflineFilesItem>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumOfflineFilesItems> {
        let mut result__: <IEnumOfflineFilesItems as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOfflineFilesItems>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumOfflineFilesItems {
    type Vtable = IEnumOfflineFilesItems_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3664832533, 50017, 17415, [188, 11, 13, 112, 70, 229, 242, 205]);
}
impl ::std::convert::From<IEnumOfflineFilesItems> for ::windows::runtime::IUnknown {
    fn from(value: IEnumOfflineFilesItems) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumOfflineFilesItems> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumOfflineFilesItems) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumOfflineFilesItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumOfflineFilesItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOfflineFilesItems_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumOfflineFilesSettings(pub ::windows::runtime::IUnknown);
impl IEnumOfflineFilesSettings {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::std::option::Option<IOfflineFilesSetting>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumOfflineFilesSettings> {
        let mut result__: <IEnumOfflineFilesSettings as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOfflineFilesSettings>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumOfflineFilesSettings {
    type Vtable = IEnumOfflineFilesSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1922465988, 6712, 18364, [158, 92, 2, 197, 21, 98, 172, 48]);
}
impl ::std::convert::From<IEnumOfflineFilesSettings> for ::windows::runtime::IUnknown {
    fn from(value: IEnumOfflineFilesSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumOfflineFilesSettings> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumOfflineFilesSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumOfflineFilesSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumOfflineFilesSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOfflineFilesSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesCache(pub ::windows::runtime::IUnknown);
impl IOfflineFilesCache {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Synchronize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncConflictHandler>, Param6: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(
        &self,
        hwndparent: Param0,
        rgpszpaths: *const super::super::Foundation::PWSTR,
        cpaths: u32,
        basync: Param3,
        dwsynccontrol: u32,
        pisyncconflicthandler: Param5,
        piprogress: Param6,
        psyncid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), basync.into_param().abi(), ::std::mem::transmute(dwsynccontrol), pisyncconflicthandler.into_param().abi(), piprogress.into_param().abi(), ::std::mem::transmute(psyncid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn DeleteItems<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, IOfflineFilesSimpleProgress>>(&self, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: Param3, piprogress: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), ::std::mem::transmute(dwflags), basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn DeleteItemsForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::runtime::IntoParam<'a, IOfflineFilesSimpleProgress>>(&self, pszuser: Param0, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: Param4, piprogress: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszuser.into_param().abi(), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), ::std::mem::transmute(dwflags), basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Pin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(
        &self,
        hwndparent: Param0,
        rgpszpaths: *const super::super::Foundation::PWSTR,
        cpaths: u32,
        bdeep: Param3,
        basync: Param4,
        dwpincontrolflags: u32,
        piprogress: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), bdeep.into_param().abi(), basync.into_param().abi(), ::std::mem::transmute(dwpincontrolflags), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Unpin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(
        &self,
        hwndparent: Param0,
        rgpszpaths: *const super::super::Foundation::PWSTR,
        cpaths: u32,
        bdeep: Param3,
        basync: Param4,
        dwpincontrolflags: u32,
        piprogress: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), bdeep.into_param().abi(), basync.into_param().abi(), ::std::mem::transmute(dwpincontrolflags), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetEncryptionStatus(&self, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbencrypted), ::std::mem::transmute(pbpartial)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Encrypt<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(&self, hwndparent: Param0, bencrypt: Param1, dwencryptioncontrolflags: u32, basync: Param3, piprogress: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), bencrypt.into_param().abi(), ::std::mem::transmute(dwencryptioncontrolflags), basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn FindItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, dwqueryflags: u32) -> ::windows::runtime::Result<IOfflineFilesItem> {
        let mut result__: <IOfflineFilesItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(dwqueryflags), &mut result__).from_abi::<IOfflineFilesItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn FindItemEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>, Param2: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>, Param3: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>, Param4: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>>(
        &self,
        pszpath: Param0,
        pincludefilefilter: Param1,
        pincludedirfilter: Param2,
        pexcludefilefilter: Param3,
        pexcludedirfilter: Param4,
        dwqueryflags: u32,
    ) -> ::windows::runtime::Result<IOfflineFilesItem> {
        let mut result__: <IOfflineFilesItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), pincludefilefilter.into_param().abi(), pincludedirfilter.into_param().abi(), pexcludefilefilter.into_param().abi(), pexcludedirfilter.into_param().abi(), ::std::mem::transmute(dwqueryflags), &mut result__).from_abi::<IOfflineFilesItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn RenameItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszpathoriginal: Param0, pszpathnew: Param1, breplaceifexists: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszpathoriginal.into_param().abi(), pszpathnew.into_param().abi(), breplaceifexists.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetLocation(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetDiskSpaceInformation(&self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcbvolumetotal), ::std::mem::transmute(pcblimit), ::std::mem::transmute(pcbused), ::std::mem::transmute(pcbunpinnedlimit), ::std::mem::transmute(pcbunpinnedused)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SetDiskSpaceLimits(&self, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(cblimit), ::std::mem::transmute(cbunpinnedlimit)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn ProcessAdminPinPolicy<'a, Param0: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>, Param1: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(&self, ppinprogress: Param0, punpinprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ppinprogress.into_param().abi(), punpinprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetSettingObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsettingname: Param0) -> ::windows::runtime::Result<IOfflineFilesSetting> {
        let mut result__: <IOfflineFilesSetting as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pszsettingname.into_param().abi(), &mut result__).from_abi::<IOfflineFilesSetting>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn EnumSettingObjects(&self) -> ::windows::runtime::Result<IEnumOfflineFilesSettings> {
        let mut result__: <IEnumOfflineFilesSettings as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOfflineFilesSettings>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPathCacheable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(pbcacheable), ::std::mem::transmute(psharecachingmode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesCache {
    type Vtable = IOfflineFilesCache_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2237489667, 30996, 18617, [141, 64, 76, 86, 245, 172, 255, 197]);
}
impl ::std::convert::From<IOfflineFilesCache> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesCache) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesCache> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesCache) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesCache_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: ::windows::runtime::RawPtr, piprogress: ::windows::runtime::RawPtr, psyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszuser: super::super::Foundation::PWSTR, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, dwqueryflags: u32, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, pincludefilefilter: ::windows::runtime::RawPtr, pincludedirfilter: ::windows::runtime::RawPtr, pexcludefilefilter: ::windows::runtime::RawPtr, pexcludedirfilter: ::windows::runtime::RawPtr, dwqueryflags: u32, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppinprogress: ::windows::runtime::RawPtr, punpinprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsettingname: super::super::Foundation::PWSTR, ppsetting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesCache2(pub ::windows::runtime::IUnknown);
impl IOfflineFilesCache2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Synchronize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncConflictHandler>, Param6: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(
        &self,
        hwndparent: Param0,
        rgpszpaths: *const super::super::Foundation::PWSTR,
        cpaths: u32,
        basync: Param3,
        dwsynccontrol: u32,
        pisyncconflicthandler: Param5,
        piprogress: Param6,
        psyncid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), basync.into_param().abi(), ::std::mem::transmute(dwsynccontrol), pisyncconflicthandler.into_param().abi(), piprogress.into_param().abi(), ::std::mem::transmute(psyncid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn DeleteItems<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, IOfflineFilesSimpleProgress>>(&self, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: Param3, piprogress: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), ::std::mem::transmute(dwflags), basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn DeleteItemsForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::runtime::IntoParam<'a, IOfflineFilesSimpleProgress>>(&self, pszuser: Param0, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: Param4, piprogress: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszuser.into_param().abi(), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), ::std::mem::transmute(dwflags), basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Pin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(
        &self,
        hwndparent: Param0,
        rgpszpaths: *const super::super::Foundation::PWSTR,
        cpaths: u32,
        bdeep: Param3,
        basync: Param4,
        dwpincontrolflags: u32,
        piprogress: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), bdeep.into_param().abi(), basync.into_param().abi(), ::std::mem::transmute(dwpincontrolflags), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Unpin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(
        &self,
        hwndparent: Param0,
        rgpszpaths: *const super::super::Foundation::PWSTR,
        cpaths: u32,
        bdeep: Param3,
        basync: Param4,
        dwpincontrolflags: u32,
        piprogress: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(rgpszpaths), ::std::mem::transmute(cpaths), bdeep.into_param().abi(), basync.into_param().abi(), ::std::mem::transmute(dwpincontrolflags), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetEncryptionStatus(&self, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbencrypted), ::std::mem::transmute(pbpartial)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Encrypt<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(&self, hwndparent: Param0, bencrypt: Param1, dwencryptioncontrolflags: u32, basync: Param3, piprogress: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), bencrypt.into_param().abi(), ::std::mem::transmute(dwencryptioncontrolflags), basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn FindItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, dwqueryflags: u32) -> ::windows::runtime::Result<IOfflineFilesItem> {
        let mut result__: <IOfflineFilesItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(dwqueryflags), &mut result__).from_abi::<IOfflineFilesItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn FindItemEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>, Param2: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>, Param3: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>, Param4: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>>(
        &self,
        pszpath: Param0,
        pincludefilefilter: Param1,
        pincludedirfilter: Param2,
        pexcludefilefilter: Param3,
        pexcludedirfilter: Param4,
        dwqueryflags: u32,
    ) -> ::windows::runtime::Result<IOfflineFilesItem> {
        let mut result__: <IOfflineFilesItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), pincludefilefilter.into_param().abi(), pincludedirfilter.into_param().abi(), pexcludefilefilter.into_param().abi(), pexcludedirfilter.into_param().abi(), ::std::mem::transmute(dwqueryflags), &mut result__).from_abi::<IOfflineFilesItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn RenameItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszpathoriginal: Param0, pszpathnew: Param1, breplaceifexists: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszpathoriginal.into_param().abi(), pszpathnew.into_param().abi(), breplaceifexists.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetLocation(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetDiskSpaceInformation(&self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcbvolumetotal), ::std::mem::transmute(pcblimit), ::std::mem::transmute(pcbused), ::std::mem::transmute(pcbunpinnedlimit), ::std::mem::transmute(pcbunpinnedused)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SetDiskSpaceLimits(&self, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(cblimit), ::std::mem::transmute(cbunpinnedlimit)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn ProcessAdminPinPolicy<'a, Param0: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>, Param1: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncProgress>>(&self, ppinprogress: Param0, punpinprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ppinprogress.into_param().abi(), punpinprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetSettingObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsettingname: Param0) -> ::windows::runtime::Result<IOfflineFilesSetting> {
        let mut result__: <IOfflineFilesSetting as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pszsettingname.into_param().abi(), &mut result__).from_abi::<IOfflineFilesSetting>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn EnumSettingObjects(&self) -> ::windows::runtime::Result<IEnumOfflineFilesSettings> {
        let mut result__: <IEnumOfflineFilesSettings as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOfflineFilesSettings>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPathCacheable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(pbcacheable), ::std::mem::transmute(psharecachingmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn RenameItemEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszpathoriginal: Param0, pszpathnew: Param1, breplaceifexists: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), pszpathoriginal.into_param().abi(), pszpathnew.into_param().abi(), breplaceifexists.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesCache2 {
    type Vtable = IOfflineFilesCache2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2349289529, 5457, 20185, [135, 129, 86, 112, 92, 4, 211, 192]);
}
impl ::std::convert::From<IOfflineFilesCache2> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesCache2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesCache2> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesCache2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesCache2> for IOfflineFilesCache {
    fn from(value: IOfflineFilesCache2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesCache2> for IOfflineFilesCache {
    fn from(value: &IOfflineFilesCache2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesCache> for IOfflineFilesCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesCache> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesCache> for &IOfflineFilesCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesCache> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesCache2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: ::windows::runtime::RawPtr, piprogress: ::windows::runtime::RawPtr, psyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszuser: super::super::Foundation::PWSTR, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, dwqueryflags: u32, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, pincludefilefilter: ::windows::runtime::RawPtr, pincludedirfilter: ::windows::runtime::RawPtr, pexcludefilefilter: ::windows::runtime::RawPtr, pexcludedirfilter: ::windows::runtime::RawPtr, dwqueryflags: u32, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppinprogress: ::windows::runtime::RawPtr, punpinprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsettingname: super::super::Foundation::PWSTR, ppsetting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesChangeInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesChangeInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsDeletedOffline(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsCreatedOffline(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsLocallyModifiedData(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsLocallyModifiedAttributes(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsLocallyModifiedTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesChangeInfo {
    type Vtable = IOfflineFilesChangeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2842587044, 57553, 19497, [150, 11, 238, 80, 143, 230, 140, 114]);
}
impl ::std::convert::From<IOfflineFilesChangeInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesChangeInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesChangeInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesChangeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesChangeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesChangeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesChangeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdeletedoffline: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbcreatedoffline: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblocallymodifieddata: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblocallymodifiedattributes: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblocallymodifiedtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesConnectionInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesConnectionInfo {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetConnectState(&self, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pconnectstate), ::std::mem::transmute(pofflinereason)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SetConnectState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(connectstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn TransitionOnline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn TransitionOffline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hwndparent: Param0, dwflags: u32, bforceopenfilesclosed: Param2) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(dwflags), bforceopenfilesclosed.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesConnectionInfo {
    type Vtable = IOfflineFilesConnectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4021434889, 43111, 19432, [131, 166, 134, 150, 154, 125, 8, 86]);
}
impl ::std::convert::From<IOfflineFilesConnectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesConnectionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesConnectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesConnectionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesConnectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesConnectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesConnectionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL, pbopenfilespreventedtransition: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesDirectoryItem(pub ::windows::runtime::IUnknown);
impl IOfflineFilesDirectoryItem {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetItemType(&self) -> ::windows::runtime::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__: <OFFLINEFILES_ITEM_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OFFLINEFILES_ITEM_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetParentItem(&self) -> ::windows::runtime::Result<IOfflineFilesItem> {
        let mut result__: <IOfflineFilesItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOfflineFilesItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwqueryflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesDirectoryItem {
    type Vtable = IOfflineFilesDirectoryItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(577984890, 41100, 18944, [163, 122, 193, 174, 78, 154, 28, 253]);
}
impl ::std::convert::From<IOfflineFilesDirectoryItem> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesDirectoryItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesDirectoryItem> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesDirectoryItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesDirectoryItem> for IOfflineFilesItem {
    fn from(value: IOfflineFilesDirectoryItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesDirectoryItem> for IOfflineFilesItem {
    fn from(value: &IOfflineFilesDirectoryItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesItem> for IOfflineFilesDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesItem> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesItem> for &IOfflineFilesDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesDirectoryItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwqueryflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesDirtyInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesDirtyInfo {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn LocalDirtyByteCount(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn RemoteDirtyByteCount(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesDirtyInfo {
    type Vtable = IOfflineFilesDirtyInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(256953907, 47817, 20138, [161, 29, 218, 14, 82, 125, 4, 125]);
}
impl ::std::convert::From<IOfflineFilesDirtyInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesDirtyInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesDirtyInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesDirtyInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesDirtyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesDirtyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesDirtyInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirtybytecount: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirtybytecount: *mut i64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesErrorInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesErrorInfo {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_System_Com`*"]
    pub unsafe fn GetRawData(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::BYTE_BLOB> {
        let mut result__: <*mut super::super::System::Com::BYTE_BLOB as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::BYTE_BLOB>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesErrorInfo {
    type Vtable = IOfflineFilesErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1897069151, 30065, 17242, [142, 183, 25, 92, 124, 20, 41, 188]);
}
impl ::std::convert::From<IOfflineFilesErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesErrorInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesErrorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesErrorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesEvents(pub ::windows::runtime::IUnknown);
impl IOfflineFilesEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn CacheMoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszoldpath: Param0, psznewpath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheIsFull(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Enabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), benabled.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn EncryptionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bwasencrypted: Param0, bwaspartial: Param1, bisencrypted: Param2, bispartial: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bwasencrypted.into_param().abi(), bwaspartial.into_param().abi(), bisencrypted.into_param().abi(), bispartial.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncFileResult<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rsyncid: *const ::windows::runtime::GUID, pszfile: Param1, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid), pszfile.into_param().abi(), ::std::mem::transmute(hrresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows::runtime::GUID, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid), ::std::mem::transmute(hrresult)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn NetTransportArrived(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn NoNetTransports(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemReconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemAvailableOffline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemNotAvailableOffline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemPinned<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemNotPinned<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemModified<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: Param2, bmodifiedattributes: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype), bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemAddedToCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemDeletedFromCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemRenamed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszoldpath: Param0, psznewpath: Param1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn DataLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Ping(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesEvents {
    type Vtable = IOfflineFilesEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3797255617, 3242, 20145, [135, 59, 28, 174, 91, 119, 195, 20]);
}
impl ::std::convert::From<IOfflineFilesEvents> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benabled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesEvents2(pub ::windows::runtime::IUnknown);
impl IOfflineFilesEvents2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn CacheMoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszoldpath: Param0, psznewpath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheIsFull(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Enabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), benabled.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn EncryptionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bwasencrypted: Param0, bwaspartial: Param1, bisencrypted: Param2, bispartial: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bwasencrypted.into_param().abi(), bwaspartial.into_param().abi(), bisencrypted.into_param().abi(), bispartial.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncFileResult<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rsyncid: *const ::windows::runtime::GUID, pszfile: Param1, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid), pszfile.into_param().abi(), ::std::mem::transmute(hrresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows::runtime::GUID, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid), ::std::mem::transmute(hrresult)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn NetTransportArrived(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn NoNetTransports(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemReconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemAvailableOffline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemNotAvailableOffline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemPinned<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemNotPinned<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemModified<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: Param2, bmodifiedattributes: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype), bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemAddedToCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemDeletedFromCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemRenamed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszoldpath: Param0, psznewpath: Param1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn DataLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Ping(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn ItemReconnectBegin(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn ItemReconnectEnd(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheEvictBegin(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheEvictEnd(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsynccontrolflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsynccontrolflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn PolicyChangeDetected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn PreferenceChangeDetected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SettingsChangesApplied(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesEvents2 {
    type Vtable = IOfflineFilesEvents2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(514690902, 65398, 20394, [167, 149, 111, 110, 247, 146, 73, 139]);
}
impl ::std::convert::From<IOfflineFilesEvents2> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesEvents2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesEvents2> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesEvents2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesEvents2> for IOfflineFilesEvents {
    fn from(value: IOfflineFilesEvents2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesEvents2> for IOfflineFilesEvents {
    fn from(value: &IOfflineFilesEvents2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents> for IOfflineFilesEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents> for &IOfflineFilesEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benabled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsynccontrolflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsynccontrolflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesEvents3(pub ::windows::runtime::IUnknown);
impl IOfflineFilesEvents3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn CacheMoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszoldpath: Param0, psznewpath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheIsFull(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Enabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), benabled.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn EncryptionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bwasencrypted: Param0, bwaspartial: Param1, bisencrypted: Param2, bispartial: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bwasencrypted.into_param().abi(), bwaspartial.into_param().abi(), bisencrypted.into_param().abi(), bispartial.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncFileResult<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rsyncid: *const ::windows::runtime::GUID, pszfile: Param1, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid), pszfile.into_param().abi(), ::std::mem::transmute(hrresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows::runtime::GUID, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid), ::std::mem::transmute(hrresult)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn NetTransportArrived(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn NoNetTransports(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemReconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemAvailableOffline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemNotAvailableOffline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemPinned<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemNotPinned<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemModified<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: Param2, bmodifiedattributes: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype), bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemAddedToCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemDeletedFromCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemRenamed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszoldpath: Param0, psznewpath: Param1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn DataLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Ping(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn ItemReconnectBegin(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn ItemReconnectEnd(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheEvictBegin(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheEvictEnd(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsynccontrolflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsynccontrolflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn PolicyChangeDetected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn PreferenceChangeDetected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SettingsChangesApplied(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn TransparentCacheItemNotify<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        pszpath: Param0,
        eventtype: OFFLINEFILES_EVENTS,
        itemtype: OFFLINEFILES_ITEM_TYPE,
        bmodifieddata: Param3,
        bmodifiedattributes: Param4,
        pzsoldpath: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(eventtype), ::std::mem::transmute(itemtype), bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi(), pzsoldpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn PrefetchFileBegin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn PrefetchFileEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(hrresult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesEvents3 {
    type Vtable = IOfflineFilesEvents3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2610973253, 61033, 17136, [154, 177, 125, 181, 200, 128, 88, 8]);
}
impl ::std::convert::From<IOfflineFilesEvents3> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesEvents3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesEvents3> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesEvents3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesEvents3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesEvents3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesEvents3> for IOfflineFilesEvents2 {
    fn from(value: IOfflineFilesEvents3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesEvents3> for IOfflineFilesEvents2 {
    fn from(value: &IOfflineFilesEvents3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents2> for IOfflineFilesEvents3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents2> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents2> for &IOfflineFilesEvents3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IOfflineFilesEvents3> for IOfflineFilesEvents {
    fn from(value: IOfflineFilesEvents3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesEvents3> for IOfflineFilesEvents {
    fn from(value: &IOfflineFilesEvents3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents> for IOfflineFilesEvents3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents> for &IOfflineFilesEvents3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benabled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsynccontrolflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsynccontrolflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesEvents4(pub ::windows::runtime::IUnknown);
impl IOfflineFilesEvents4 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn CacheMoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszoldpath: Param0, psznewpath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheIsFull(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Enabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), benabled.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn EncryptionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bwasencrypted: Param0, bwaspartial: Param1, bisencrypted: Param2, bispartial: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bwasencrypted.into_param().abi(), bwaspartial.into_param().abi(), bisencrypted.into_param().abi(), bispartial.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncFileResult<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rsyncid: *const ::windows::runtime::GUID, pszfile: Param1, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid), pszfile.into_param().abi(), ::std::mem::transmute(hrresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncConflictRecRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconflictpath: Param0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszconflictpath.into_param().abi(), ::std::mem::transmute(pftconflictdatetime), ::std::mem::transmute(conflictsyncstate)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows::runtime::GUID, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(rsyncid), ::std::mem::transmute(hrresult)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn NetTransportArrived(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn NoNetTransports(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemReconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemAvailableOffline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemNotAvailableOffline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemPinned<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemNotPinned<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemModified<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: Param2, bmodifiedattributes: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype), bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemAddedToCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemDeletedFromCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemRenamed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszoldpath: Param0, psznewpath: Param1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi(), ::std::mem::transmute(itemtype)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn DataLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Ping(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn ItemReconnectBegin(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn ItemReconnectEnd(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheEvictBegin(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn CacheEvictEnd(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsynccontrolflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsynccontrolflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn PolicyChangeDetected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn PreferenceChangeDetected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn SettingsChangesApplied(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn TransparentCacheItemNotify<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        pszpath: Param0,
        eventtype: OFFLINEFILES_EVENTS,
        itemtype: OFFLINEFILES_ITEM_TYPE,
        bmodifieddata: Param3,
        bmodifiedattributes: Param4,
        pzsoldpath: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(eventtype), ::std::mem::transmute(itemtype), bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi(), pzsoldpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn PrefetchFileBegin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn PrefetchFileEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(hrresult)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn PrefetchCloseHandleBegin(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn PrefetchCloseHandleEnd(&self, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwclosedhandlecount), ::std::mem::transmute(dwopenhandlecount), ::std::mem::transmute(hrresult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesEvents4 {
    type Vtable = IOfflineFilesEvents4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3688274718, 51154, 18238, [179, 95, 157, 140, 36, 192, 196, 132]);
}
impl ::std::convert::From<IOfflineFilesEvents4> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesEvents4) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesEvents4> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesEvents4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesEvents4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesEvents4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesEvents4> for IOfflineFilesEvents3 {
    fn from(value: IOfflineFilesEvents4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesEvents4> for IOfflineFilesEvents3 {
    fn from(value: &IOfflineFilesEvents4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents3> for IOfflineFilesEvents4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents3> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents3> for &IOfflineFilesEvents4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents3> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IOfflineFilesEvents4> for IOfflineFilesEvents2 {
    fn from(value: IOfflineFilesEvents4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesEvents4> for IOfflineFilesEvents2 {
    fn from(value: &IOfflineFilesEvents4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents2> for IOfflineFilesEvents4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents2> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents2> for &IOfflineFilesEvents4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IOfflineFilesEvents4> for IOfflineFilesEvents {
    fn from(value: IOfflineFilesEvents4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesEvents4> for IOfflineFilesEvents {
    fn from(value: &IOfflineFilesEvents4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents> for IOfflineFilesEvents4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesEvents> for &IOfflineFilesEvents4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesEvents> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benabled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rsyncid: *const ::windows::runtime::GUID, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsynccontrolflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsynccontrolflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesEventsFilter(pub ::windows::runtime::IUnknown);
impl IOfflineFilesEventsFilter {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetPathFilter(&self, ppszfilter: *mut super::super::Foundation::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppszfilter), ::std::mem::transmute(pmatch)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetIncludedEvents(&self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celements), ::std::mem::transmute(prgevents), ::std::mem::transmute(pcevents)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetExcludedEvents(&self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(celements), ::std::mem::transmute(prgevents), ::std::mem::transmute(pcevents)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesEventsFilter {
    type Vtable = IOfflineFilesEventsFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(872173083, 1814, 16634, [186, 101, 110, 98, 168, 74, 132, 111]);
}
impl ::std::convert::From<IOfflineFilesEventsFilter> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesEventsFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesEventsFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesEventsFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesEventsFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesEventsFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEventsFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszfilter: *mut super::super::Foundation::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesFileItem(pub ::windows::runtime::IUnknown);
impl IOfflineFilesFileItem {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetItemType(&self) -> ::windows::runtime::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__: <OFFLINEFILES_ITEM_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OFFLINEFILES_ITEM_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetParentItem(&self) -> ::windows::runtime::Result<IOfflineFilesItem> {
        let mut result__: <IOfflineFilesItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOfflineFilesItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwqueryflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsSparse(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsEncrypted(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesFileItem {
    type Vtable = IOfflineFilesFileItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2382028461, 9922, 20223, [138, 114, 107, 80, 114, 61, 154, 0]);
}
impl ::std::convert::From<IOfflineFilesFileItem> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesFileItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesFileItem> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesFileItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesFileItem> for IOfflineFilesItem {
    fn from(value: IOfflineFilesFileItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesFileItem> for IOfflineFilesItem {
    fn from(value: &IOfflineFilesFileItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesItem> for IOfflineFilesFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesItem> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesItem> for &IOfflineFilesFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesFileItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwqueryflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbissparse: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisencrypted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesFileSysInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesFileSysInfo {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetAttributes(&self, copy: OFFLINEFILES_ITEM_COPY) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(copy), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetTimes(&self, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(copy), ::std::mem::transmute(pftcreationtime), ::std::mem::transmute(pftlastwritetime), ::std::mem::transmute(pftchangetime), ::std::mem::transmute(pftlastaccesstime)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetFileSize(&self, copy: OFFLINEFILES_ITEM_COPY) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(copy), &mut result__).from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesFileSysInfo {
    type Vtable = IOfflineFilesFileSysInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3155826239, 31741, 19848, [156, 102, 150, 234, 154, 106, 61, 107]);
}
impl ::std::convert::From<IOfflineFilesFileSysInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesFileSysInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesFileSysInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesFileSysInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesFileSysInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesFileSysInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesFileSysInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesGhostInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesGhostInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsGhosted(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesGhostInfo {
    type Vtable = IOfflineFilesGhostInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(722064524, 35509, 17999, [167, 85, 165, 157, 146, 249, 148, 41]);
}
impl ::std::convert::From<IOfflineFilesGhostInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesGhostInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesGhostInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesGhostInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesGhostInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesGhostInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesGhostInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbghosted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesItem(pub ::windows::runtime::IUnknown);
impl IOfflineFilesItem {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetItemType(&self) -> ::windows::runtime::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__: <OFFLINEFILES_ITEM_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OFFLINEFILES_ITEM_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetParentItem(&self) -> ::windows::runtime::Result<IOfflineFilesItem> {
        let mut result__: <IOfflineFilesItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOfflineFilesItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwqueryflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesItem {
    type Vtable = IOfflineFilesItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1249197478, 57412, 20242, [167, 24, 93, 20, 208, 121, 169, 6]);
}
impl ::std::convert::From<IOfflineFilesItem> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesItem> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwqueryflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesItemContainer(pub ::windows::runtime::IUnknown);
impl IOfflineFilesItemContainer {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn EnumItems(&self, dwqueryflags: u32) -> ::windows::runtime::Result<IEnumOfflineFilesItems> {
        let mut result__: <IEnumOfflineFilesItems as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwqueryflags), &mut result__).from_abi::<IEnumOfflineFilesItems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn EnumItemsEx<'a, Param0: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>, Param1: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>, Param2: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>, Param3: ::windows::runtime::IntoParam<'a, IOfflineFilesItemFilter>>(
        &self,
        pincludefilefilter: Param0,
        pincludedirfilter: Param1,
        pexcludefilefilter: Param2,
        pexcludedirfilter: Param3,
        dwenumflags: u32,
        dwqueryflags: u32,
    ) -> ::windows::runtime::Result<IEnumOfflineFilesItems> {
        let mut result__: <IEnumOfflineFilesItems as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pincludefilefilter.into_param().abi(), pincludedirfilter.into_param().abi(), pexcludefilefilter.into_param().abi(), pexcludedirfilter.into_param().abi(), ::std::mem::transmute(dwenumflags), ::std::mem::transmute(dwqueryflags), &mut result__).from_abi::<IEnumOfflineFilesItems>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesItemContainer {
    type Vtable = IOfflineFilesItemContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(943124553, 37907, 17885, [191, 70, 181, 170, 168, 45, 195, 16]);
}
impl ::std::convert::From<IOfflineFilesItemContainer> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesItemContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesItemContainer> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesItemContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesItemContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesItemContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesItemContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwqueryflags: u32, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pincludefilefilter: ::windows::runtime::RawPtr, pincludedirfilter: ::windows::runtime::RawPtr, pexcludefilefilter: ::windows::runtime::RawPtr, pexcludedirfilter: ::windows::runtime::RawPtr, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesItemFilter(pub ::windows::runtime::IUnknown);
impl IOfflineFilesItemFilter {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetFilterFlags(&self, pullflags: *mut u64, pullmask: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pullflags), ::std::mem::transmute(pullmask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetTimeFilter(&self, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfttime), ::std::mem::transmute(pbevaltimeofday), ::std::mem::transmute(ptimetype), ::std::mem::transmute(pcompare)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetPatternFilter(&self, pszpattern: super::super::Foundation::PWSTR, cchpattern: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pszpattern), ::std::mem::transmute(cchpattern)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesItemFilter {
    type Vtable = IOfflineFilesItemFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4105544300, 56325, 20256, [173, 164, 85, 31, 16, 119, 190, 92]);
}
impl ::std::convert::From<IOfflineFilesItemFilter> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesItemFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesItemFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesItemFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesItemFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesItemFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesItemFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pullflags: *mut u64, pullmask: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpattern: super::super::Foundation::PWSTR, cchpattern: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesPinInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesPinInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinned(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinnedForUser(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbpinnedforuser), ::std::mem::transmute(pbinherit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinnedForUserByPolicy(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbpinnedforuser), ::std::mem::transmute(pbinherit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinnedForComputer(&self, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbpinnedforcomputer), ::std::mem::transmute(pbinherit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinnedForFolderRedirection(&self, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbpinnedforfolderredirection), ::std::mem::transmute(pbinherit)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesPinInfo {
    type Vtable = IOfflineFilesPinInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1529546325, 46077, 18813, [173, 235, 189, 21, 107, 200, 53, 91]);
}
impl ::std::convert::From<IOfflineFilesPinInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesPinInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesPinInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesPinInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesPinInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesPinInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesPinInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinned: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesPinInfo2(pub ::windows::runtime::IUnknown);
impl IOfflineFilesPinInfo2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinned(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinnedForUser(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbpinnedforuser), ::std::mem::transmute(pbinherit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinnedForUserByPolicy(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbpinnedforuser), ::std::mem::transmute(pbinherit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinnedForComputer(&self, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbpinnedforcomputer), ::std::mem::transmute(pbinherit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPinnedForFolderRedirection(&self, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbpinnedforfolderredirection), ::std::mem::transmute(pbinherit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsPartlyPinned(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesPinInfo2 {
    type Vtable = IOfflineFilesPinInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1648122018, 17133, 19159, [182, 154, 15, 27, 48, 167, 45, 13]);
}
impl ::std::convert::From<IOfflineFilesPinInfo2> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesPinInfo2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesPinInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesPinInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesPinInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesPinInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesPinInfo2> for IOfflineFilesPinInfo {
    fn from(value: IOfflineFilesPinInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesPinInfo2> for IOfflineFilesPinInfo {
    fn from(value: &IOfflineFilesPinInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesPinInfo> for IOfflineFilesPinInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesPinInfo> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesPinInfo> for &IOfflineFilesPinInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesPinInfo> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesPinInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinned: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpartlypinned: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesProgress(pub ::windows::runtime::IUnknown);
impl IOfflineFilesProgress {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Begin(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn QueryAbort(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn End(&self, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrresult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesProgress {
    type Vtable = IOfflineFilesProgress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4208341559, 50523, 18705, [152, 80, 188, 249, 109, 76, 151, 158]);
}
impl ::std::convert::From<IOfflineFilesProgress> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesProgress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesProgress> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesProgress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesProgress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesServerItem(pub ::windows::runtime::IUnknown);
impl IOfflineFilesServerItem {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetItemType(&self) -> ::windows::runtime::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__: <OFFLINEFILES_ITEM_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OFFLINEFILES_ITEM_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetParentItem(&self) -> ::windows::runtime::Result<IOfflineFilesItem> {
        let mut result__: <IOfflineFilesItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOfflineFilesItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwqueryflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesServerItem {
    type Vtable = IOfflineFilesServerItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2602341750, 43307, 16721, [142, 158, 124, 123, 62, 194, 224, 22]);
}
impl ::std::convert::From<IOfflineFilesServerItem> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesServerItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesServerItem> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesServerItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesServerItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesServerItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesServerItem> for IOfflineFilesItem {
    fn from(value: IOfflineFilesServerItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesServerItem> for IOfflineFilesItem {
    fn from(value: &IOfflineFilesServerItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesItem> for IOfflineFilesServerItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesItem> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesItem> for &IOfflineFilesServerItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesServerItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwqueryflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesSetting(pub ::windows::runtime::IUnknown);
impl IOfflineFilesSetting {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetValueType(&self) -> ::windows::runtime::Result<OFFLINEFILES_SETTING_VALUE_TYPE> {
        let mut result__: <OFFLINEFILES_SETTING_VALUE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OFFLINEFILES_SETTING_VALUE_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetPreference(&self, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvarvalue), ::std::mem::transmute(dwscope)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetPreferenceScope(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetPreference(&self, pvarvalue: *const super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvarvalue), ::std::mem::transmute(dwscope)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn DeletePreference(&self, dwscope: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwscope)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetPolicy(&self, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvarvalue), ::std::mem::transmute(dwscope)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetPolicyScope(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetValue(&self, pvarvalue: *mut super::super::System::Com::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvarvalue), ::std::mem::transmute(pbsetbypolicy)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesSetting {
    type Vtable = IOfflineFilesSetting_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3631338487, 62995, 18593, [130, 126, 122, 52, 229, 96, 255, 246]);
}
impl ::std::convert::From<IOfflineFilesSetting> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesSetting) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesSetting> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesSetting) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesSetting {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesSetting {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSetting_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, dwscope: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwscope: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarvalue: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, dwscope: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwscope: u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, dwscope: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwscope: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesShareInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesShareInfo {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetShareItem(&self) -> ::windows::runtime::Result<IOfflineFilesShareItem> {
        let mut result__: <IOfflineFilesShareItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOfflineFilesShareItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetShareCachingMode(&self) -> ::windows::runtime::Result<OFFLINEFILES_CACHING_MODE> {
        let mut result__: <OFFLINEFILES_CACHING_MODE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OFFLINEFILES_CACHING_MODE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsShareDfsJunction(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesShareInfo {
    type Vtable = IOfflineFilesShareInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2076984295, 12750, 19620, [140, 205, 28, 255, 45, 196, 148, 218]);
}
impl ::std::convert::From<IOfflineFilesShareInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesShareInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesShareInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesShareInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesShareInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesShareInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesShareInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppshareitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisdfsjunction: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesShareItem(pub ::windows::runtime::IUnknown);
impl IOfflineFilesShareItem {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetItemType(&self) -> ::windows::runtime::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__: <OFFLINEFILES_ITEM_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OFFLINEFILES_ITEM_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetParentItem(&self) -> ::windows::runtime::Result<IOfflineFilesItem> {
        let mut result__: <IOfflineFilesItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOfflineFilesItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwqueryflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesShareItem {
    type Vtable = IOfflineFilesShareItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3132613773, 18436, 16821, [164, 77, 15, 25, 155, 6, 177, 69]);
}
impl ::std::convert::From<IOfflineFilesShareItem> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesShareItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesShareItem> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesShareItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesShareItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesShareItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesShareItem> for IOfflineFilesItem {
    fn from(value: IOfflineFilesShareItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesShareItem> for IOfflineFilesItem {
    fn from(value: &IOfflineFilesShareItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesItem> for IOfflineFilesShareItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesItem> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesItem> for &IOfflineFilesShareItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesShareItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwqueryflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesSimpleProgress(pub ::windows::runtime::IUnknown);
impl IOfflineFilesSimpleProgress {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Begin(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn QueryAbort(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn End(&self, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemBegin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfile: Param0) -> ::windows::runtime::Result<OFFLINEFILES_OP_RESPONSE> {
        let mut result__: <OFFLINEFILES_OP_RESPONSE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszfile.into_param().abi(), &mut result__).from_abi::<OFFLINEFILES_OP_RESPONSE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ItemResult<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfile: Param0, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<OFFLINEFILES_OP_RESPONSE> {
        let mut result__: <OFFLINEFILES_OP_RESPONSE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pszfile.into_param().abi(), ::std::mem::transmute(hrresult), &mut result__).from_abi::<OFFLINEFILES_OP_RESPONSE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesSimpleProgress {
    type Vtable = IOfflineFilesSimpleProgress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3276767131, 50237, 20381, [167, 118, 192, 235, 109, 229, 212, 1]);
}
impl ::std::convert::From<IOfflineFilesSimpleProgress> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesSimpleProgress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesSimpleProgress> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesSimpleProgress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesSimpleProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesSimpleProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesSimpleProgress> for IOfflineFilesProgress {
    fn from(value: IOfflineFilesSimpleProgress) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesSimpleProgress> for IOfflineFilesProgress {
    fn from(value: &IOfflineFilesSimpleProgress) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesProgress> for IOfflineFilesSimpleProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesProgress> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesProgress> for &IOfflineFilesSimpleProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesProgress> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSimpleProgress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::runtime::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesSuspend(pub ::windows::runtime::IUnknown);
impl IOfflineFilesSuspend {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SuspendRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bsuspend: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bsuspend.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesSuspend {
    type Vtable = IOfflineFilesSuspend_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1657034255, 48139, 18634, [173, 157, 52, 203, 82, 141, 153, 169]);
}
impl ::std::convert::From<IOfflineFilesSuspend> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesSuspend) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesSuspend> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesSuspend) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesSuspend {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesSuspend {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSuspend_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bsuspend: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesSuspendInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesSuspendInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsSuspended(&self, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbsuspended), ::std::mem::transmute(pbsuspendedroot)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesSuspendInfo {
    type Vtable = IOfflineFilesSuspendInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2757214811, 20124, 19204, [133, 175, 137, 50, 204, 217, 120, 137]);
}
impl ::std::convert::From<IOfflineFilesSuspendInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesSuspendInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesSuspendInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesSuspendInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesSuspendInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesSuspendInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSuspendInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesSyncConflictHandler(pub ::windows::runtime::IUnknown);
impl IOfflineFilesSyncConflictHandler {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn ResolveConflict<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszpath.into_param().abi(), ::std::mem::transmute(fstateknown), ::std::mem::transmute(state), ::std::mem::transmute(fchangedetails), ::std::mem::transmute(pconflictresolution), ::std::mem::transmute(ppsznewname)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesSyncConflictHandler {
    type Vtable = IOfflineFilesSyncConflictHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3067957394, 50780, 18102, [151, 184, 250, 221, 8, 231, 225, 190]);
}
impl ::std::convert::From<IOfflineFilesSyncConflictHandler> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesSyncConflictHandler) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesSyncConflictHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesSyncConflictHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesSyncConflictHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesSyncConflictHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncConflictHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesSyncErrorInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesSyncErrorInfo {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_System_Com`*"]
    pub unsafe fn GetRawData(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::BYTE_BLOB> {
        let mut result__: <*mut super::super::System::Com::BYTE_BLOB as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::BYTE_BLOB>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetSyncOperation(&self) -> ::windows::runtime::Result<OFFLINEFILES_SYNC_OPERATION> {
        let mut result__: <OFFLINEFILES_SYNC_OPERATION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OFFLINEFILES_SYNC_OPERATION>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetItemChangeFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn InfoEnumerated(&self, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblocalenumerated), ::std::mem::transmute(pbremoteenumerated), ::std::mem::transmute(pboriginalenumerated)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn InfoAvailable(&self, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblocalinfo), ::std::mem::transmute(pbremoteinfo), ::std::mem::transmute(pboriginalinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetLocalInfo(&self) -> ::windows::runtime::Result<IOfflineFilesSyncErrorItemInfo> {
        let mut result__: <IOfflineFilesSyncErrorItemInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOfflineFilesSyncErrorItemInfo>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetRemoteInfo(&self) -> ::windows::runtime::Result<IOfflineFilesSyncErrorItemInfo> {
        let mut result__: <IOfflineFilesSyncErrorItemInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOfflineFilesSyncErrorItemInfo>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetOriginalInfo(&self) -> ::windows::runtime::Result<IOfflineFilesSyncErrorItemInfo> {
        let mut result__: <IOfflineFilesSyncErrorItemInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOfflineFilesSyncErrorItemInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesSyncErrorInfo {
    type Vtable = IOfflineFilesSyncErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1509514822, 60244, 18897, [190, 118, 222, 149, 69, 141, 1, 176]);
}
impl ::std::convert::From<IOfflineFilesSyncErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesSyncErrorInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesSyncErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesSyncErrorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesSyncErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesSyncErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesSyncErrorInfo> for IOfflineFilesErrorInfo {
    fn from(value: IOfflineFilesSyncErrorInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesSyncErrorInfo> for IOfflineFilesErrorInfo {
    fn from(value: &IOfflineFilesSyncErrorInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesErrorInfo> for IOfflineFilesSyncErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesErrorInfo> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesErrorInfo> for &IOfflineFilesSyncErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesErrorInfo> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncErrorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwitemchangeflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesSyncErrorItemInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesSyncErrorItemInfo {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetFileAttributes(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn GetFileTimes(&self, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pftlastwrite), ::std::mem::transmute(pftchange)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn GetFileSize(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesSyncErrorItemInfo {
    type Vtable = IOfflineFilesSyncErrorItemInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3973820173, 27160, 19797, [128, 23, 16, 143, 118, 96, 186, 68]);
}
impl ::std::convert::From<IOfflineFilesSyncErrorItemInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesSyncErrorItemInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesSyncErrorItemInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesSyncErrorItemInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesSyncErrorItemInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesSyncErrorItemInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncErrorItemInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psize: *mut i64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesSyncProgress(pub ::windows::runtime::IUnknown);
impl IOfflineFilesSyncProgress {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn Begin(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn QueryAbort(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub unsafe fn End(&self, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncItemBegin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfile: Param0) -> ::windows::runtime::Result<OFFLINEFILES_OP_RESPONSE> {
        let mut result__: <OFFLINEFILES_OP_RESPONSE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszfile.into_param().abi(), &mut result__).from_abi::<OFFLINEFILES_OP_RESPONSE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn SyncItemResult<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IOfflineFilesSyncErrorInfo>>(&self, pszfile: Param0, hrresult: ::windows::runtime::HRESULT, perrorinfo: Param2) -> ::windows::runtime::Result<OFFLINEFILES_OP_RESPONSE> {
        let mut result__: <OFFLINEFILES_OP_RESPONSE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pszfile.into_param().abi(), ::std::mem::transmute(hrresult), perrorinfo.into_param().abi(), &mut result__).from_abi::<OFFLINEFILES_OP_RESPONSE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesSyncProgress {
    type Vtable = IOfflineFilesSyncProgress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1764881562, 28615, 19483, [178, 101, 86, 121, 63, 196, 81, 183]);
}
impl ::std::convert::From<IOfflineFilesSyncProgress> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesSyncProgress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesSyncProgress> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesSyncProgress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesSyncProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesSyncProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOfflineFilesSyncProgress> for IOfflineFilesProgress {
    fn from(value: IOfflineFilesSyncProgress) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOfflineFilesSyncProgress> for IOfflineFilesProgress {
    fn from(value: &IOfflineFilesSyncProgress) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesProgress> for IOfflineFilesSyncProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesProgress> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOfflineFilesProgress> for &IOfflineFilesSyncProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOfflineFilesProgress> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncProgress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::runtime::HRESULT, perrorinfo: ::windows::runtime::RawPtr, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOfflineFilesTransparentCacheInfo(pub ::windows::runtime::IUnknown);
impl IOfflineFilesTransparentCacheInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    pub unsafe fn IsTransparentlyCached(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOfflineFilesTransparentCacheInfo {
    type Vtable = IOfflineFilesTransparentCacheInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3165604353, 23400, 19286, [166, 161, 141, 39, 134, 237, 232, 227]);
}
impl ::std::convert::From<IOfflineFilesTransparentCacheInfo> for ::windows::runtime::IUnknown {
    fn from(value: IOfflineFilesTransparentCacheInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOfflineFilesTransparentCacheInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IOfflineFilesTransparentCacheInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOfflineFilesTransparentCacheInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOfflineFilesTransparentCacheInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesTransparentCacheInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtransparentlycached: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_CACHING_MODE(pub i32);
pub const OFFLINEFILES_CACHING_MODE_NONE: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(0i32);
pub const OFFLINEFILES_CACHING_MODE_NOCACHING: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(1i32);
pub const OFFLINEFILES_CACHING_MODE_MANUAL: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(2i32);
pub const OFFLINEFILES_CACHING_MODE_AUTO_DOC: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(3i32);
pub const OFFLINEFILES_CACHING_MODE_AUTO_PROGANDDOC: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(4i32);
impl ::std::convert::From<i32> for OFFLINEFILES_CACHING_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_CACHING_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_CHANGES_LOCAL_ATTRIBUTES: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_CHANGES_LOCAL_SIZE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_CHANGES_LOCAL_TIME: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_CHANGES_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_CHANGES_REMOTE_ATTRIBUTES: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_CHANGES_REMOTE_SIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_CHANGES_REMOTE_TIME: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_COMPARE(pub i32);
pub const OFFLINEFILES_COMPARE_EQ: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(0i32);
pub const OFFLINEFILES_COMPARE_NEQ: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(1i32);
pub const OFFLINEFILES_COMPARE_LT: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(2i32);
pub const OFFLINEFILES_COMPARE_GT: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(3i32);
pub const OFFLINEFILES_COMPARE_LTE: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(4i32);
pub const OFFLINEFILES_COMPARE_GTE: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(5i32);
impl ::std::convert::From<i32> for OFFLINEFILES_COMPARE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_COMPARE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_CONNECT_STATE(pub i32);
pub const OFFLINEFILES_CONNECT_STATE_UNKNOWN: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(0i32);
pub const OFFLINEFILES_CONNECT_STATE_OFFLINE: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(1i32);
pub const OFFLINEFILES_CONNECT_STATE_ONLINE: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(2i32);
pub const OFFLINEFILES_CONNECT_STATE_TRANSPARENTLY_CACHED: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(3i32);
pub const OFFLINEFILES_CONNECT_STATE_PARTLY_TRANSPARENTLY_CACHED: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(4i32);
impl ::std::convert::From<i32> for OFFLINEFILES_CONNECT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_CONNECT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_DELETE_FLAG_ADMIN: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_DELETE_FLAG_DELMODIFIED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_DELETE_FLAG_NOAUTOCACHED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_DELETE_FLAG_NOPINNED: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ENUM_FLAT: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ENUM_FLAT_FILESONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_EVENTS(pub i32);
pub const OFFLINEFILES_EVENT_CACHEMOVED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(0i32);
pub const OFFLINEFILES_EVENT_CACHEISFULL: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(1i32);
pub const OFFLINEFILES_EVENT_CACHEISCORRUPTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(2i32);
pub const OFFLINEFILES_EVENT_ENABLED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(3i32);
pub const OFFLINEFILES_EVENT_ENCRYPTIONCHANGED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(4i32);
pub const OFFLINEFILES_EVENT_SYNCBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(5i32);
pub const OFFLINEFILES_EVENT_SYNCFILERESULT: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(6i32);
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECADDED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(7i32);
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECUPDATED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(8i32);
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECREMOVED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(9i32);
pub const OFFLINEFILES_EVENT_SYNCEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(10i32);
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(11i32);
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(12i32);
pub const OFFLINEFILES_EVENT_NETTRANSPORTARRIVED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(13i32);
pub const OFFLINEFILES_EVENT_NONETTRANSPORTS: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(14i32);
pub const OFFLINEFILES_EVENT_ITEMDISCONNECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(15i32);
pub const OFFLINEFILES_EVENT_ITEMRECONNECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(16i32);
pub const OFFLINEFILES_EVENT_ITEMAVAILABLEOFFLINE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(17i32);
pub const OFFLINEFILES_EVENT_ITEMNOTAVAILABLEOFFLINE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(18i32);
pub const OFFLINEFILES_EVENT_ITEMPINNED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(19i32);
pub const OFFLINEFILES_EVENT_ITEMNOTPINNED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(20i32);
pub const OFFLINEFILES_EVENT_ITEMMODIFIED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(21i32);
pub const OFFLINEFILES_EVENT_ITEMADDEDTOCACHE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(22i32);
pub const OFFLINEFILES_EVENT_ITEMDELETEDFROMCACHE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(23i32);
pub const OFFLINEFILES_EVENT_ITEMRENAMED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(24i32);
pub const OFFLINEFILES_EVENT_DATALOST: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(25i32);
pub const OFFLINEFILES_EVENT_PING: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(26i32);
pub const OFFLINEFILES_EVENT_ITEMRECONNECTBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(27i32);
pub const OFFLINEFILES_EVENT_ITEMRECONNECTEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(28i32);
pub const OFFLINEFILES_EVENT_CACHEEVICTBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(29i32);
pub const OFFLINEFILES_EVENT_CACHEEVICTEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(30i32);
pub const OFFLINEFILES_EVENT_POLICYCHANGEDETECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(31i32);
pub const OFFLINEFILES_EVENT_PREFERENCECHANGEDETECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(32i32);
pub const OFFLINEFILES_EVENT_SETTINGSCHANGESAPPLIED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(33i32);
pub const OFFLINEFILES_EVENT_TRANSPARENTCACHEITEMNOTIFY: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(34i32);
pub const OFFLINEFILES_EVENT_PREFETCHFILEBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(35i32);
pub const OFFLINEFILES_EVENT_PREFETCHFILEEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(36i32);
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(37i32);
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(38i32);
pub const OFFLINEFILES_NUM_EVENTS: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(39i32);
impl ::std::convert::From<i32> for OFFLINEFILES_EVENTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_EVENTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_ITEM_COPY(pub i32);
pub const OFFLINEFILES_ITEM_COPY_LOCAL: OFFLINEFILES_ITEM_COPY = OFFLINEFILES_ITEM_COPY(0i32);
pub const OFFLINEFILES_ITEM_COPY_REMOTE: OFFLINEFILES_ITEM_COPY = OFFLINEFILES_ITEM_COPY(1i32);
pub const OFFLINEFILES_ITEM_COPY_ORIGINAL: OFFLINEFILES_ITEM_COPY = OFFLINEFILES_ITEM_COPY(2i32);
impl ::std::convert::From<i32> for OFFLINEFILES_ITEM_COPY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_ITEM_COPY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_CREATED: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DELETED: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRECTORY: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRTY: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_FILE: u32 = 128u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GHOST: u32 = 8192u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_ANYACCESS: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_READ: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_WRITE: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_ATTRIBUTES: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OFFLINE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_ONLINE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_ANYACCESS: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_READ: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_WRITE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_COMPUTER: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_OTHERS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_USER: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SPARSE: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SUSPENDED: u32 = 16384u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_ANYACCESS: u32 = 524288u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_READ: u32 = 262144u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_WRITE: u32 = 131072u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_QUERY_ADMIN: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_QUERY_ATTEMPT_TRANSITIONONLINE: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_QUERY_CONNECTIONSTATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_QUERY_INCLUDETRANSPARENTCACHE: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_QUERY_LOCALDIRTYBYTECOUNT: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_QUERY_REMOTEDIRTYBYTECOUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_ITEM_QUERY_REMOTEINFO: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_ITEM_TIME(pub i32);
pub const OFFLINEFILES_ITEM_TIME_CREATION: OFFLINEFILES_ITEM_TIME = OFFLINEFILES_ITEM_TIME(0i32);
pub const OFFLINEFILES_ITEM_TIME_LASTACCESS: OFFLINEFILES_ITEM_TIME = OFFLINEFILES_ITEM_TIME(1i32);
pub const OFFLINEFILES_ITEM_TIME_LASTWRITE: OFFLINEFILES_ITEM_TIME = OFFLINEFILES_ITEM_TIME(2i32);
impl ::std::convert::From<i32> for OFFLINEFILES_ITEM_TIME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_ITEM_TIME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_ITEM_TYPE(pub i32);
pub const OFFLINEFILES_ITEM_TYPE_FILE: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(0i32);
pub const OFFLINEFILES_ITEM_TYPE_DIRECTORY: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(1i32);
pub const OFFLINEFILES_ITEM_TYPE_SHARE: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(2i32);
pub const OFFLINEFILES_ITEM_TYPE_SERVER: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(3i32);
impl ::std::convert::From<i32> for OFFLINEFILES_ITEM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_ITEM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_OFFLINE_REASON(pub i32);
pub const OFFLINEFILES_OFFLINE_REASON_UNKNOWN: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(0i32);
pub const OFFLINEFILES_OFFLINE_REASON_NOT_APPLICABLE: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(1i32);
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_FORCED: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(2i32);
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_SLOW: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(3i32);
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_ERROR: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(4i32);
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_VERSION_CONFLICT: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(5i32);
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_SUSPENDED: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(6i32);
impl ::std::convert::From<i32> for OFFLINEFILES_OFFLINE_REASON {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_OFFLINE_REASON {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_OP_RESPONSE(pub i32);
pub const OFFLINEFILES_OP_CONTINUE: OFFLINEFILES_OP_RESPONSE = OFFLINEFILES_OP_RESPONSE(0i32);
pub const OFFLINEFILES_OP_RETRY: OFFLINEFILES_OP_RESPONSE = OFFLINEFILES_OP_RESPONSE(1i32);
pub const OFFLINEFILES_OP_ABORT: OFFLINEFILES_OP_RESPONSE = OFFLINEFILES_OP_RESPONSE(2i32);
impl ::std::convert::From<i32> for OFFLINEFILES_OP_RESPONSE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_OP_RESPONSE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_PATHFILTER_MATCH(pub i32);
pub const OFFLINEFILES_PATHFILTER_SELF: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(0i32);
pub const OFFLINEFILES_PATHFILTER_CHILD: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(1i32);
pub const OFFLINEFILES_PATHFILTER_DESCENDENT: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(2i32);
pub const OFFLINEFILES_PATHFILTER_SELFORCHILD: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(3i32);
pub const OFFLINEFILES_PATHFILTER_SELFORDESCENDENT: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(4i32);
impl ::std::convert::From<i32> for OFFLINEFILES_PATHFILTER_MATCH {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_PATHFILTER_MATCH {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PINLINKTARGETS_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PINLINKTARGETS_EXPLICIT: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PINLINKTARGETS_NEVER: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FILL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORALL: u32 = 128u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORREDIR: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER_POLICY: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SETTING_SCOPE_COMPUTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SETTING_SCOPE_USER: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_SETTING_VALUE_TYPE(pub i32);
pub const OFFLINEFILES_SETTING_VALUE_UI4: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(0i32);
pub const OFFLINEFILES_SETTING_VALUE_BSTR: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(1i32);
pub const OFFLINEFILES_SETTING_VALUE_BSTR_DBLNULTERM: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(2i32);
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_UI4: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(3i32);
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_BSTR: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(4i32);
impl ::std::convert::From<i32> for OFFLINEFILES_SETTING_VALUE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_SETTING_VALUE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_SYNC_CONFLICT_RESOLVE(pub i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NONE: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(0i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLOCAL: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(1i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPREMOTE: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(2i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPALLCHANGES: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(3i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLATEST: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(4i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_LOG: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(5i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_SKIP: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(6i32);
pub const OFFLINEFILES_SYNC_CONFLICT_ABORT: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(7i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NUMCODES: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(8i32);
impl ::std::convert::From<i32> for OFFLINEFILES_SYNC_CONFLICT_RESOLVE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_SYNC_CONFLICT_RESOLVE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLATEST: u32 = 805306368u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLOCAL: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPREMOTE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_MASK: u32 = 4026531840u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_FILLSPARSE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_NONEWFILESOUT: u32 = 131072u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORALL: u32 = 128u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORREDIR: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER_POLICY: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINNEWFILES: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SKIPSUSPENDEDDIRS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCIN: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCOUT: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_ATTRIBUTES: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_CHANGETIME: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_FILESIZE: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_WRITETIME: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_SYNC_OPERATION(pub i32);
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_SERVER: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(0i32);
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_CLIENT: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(1i32);
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_SERVER: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(2i32);
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_CLIENT: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(3i32);
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_SERVER_COPY: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(4i32);
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_CLIENT_COPY: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(5i32);
pub const OFFLINEFILES_SYNC_OPERATION_PIN: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(6i32);
pub const OFFLINEFILES_SYNC_OPERATION_PREPARE: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(7i32);
impl ::std::convert::From<i32> for OFFLINEFILES_SYNC_OPERATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_SYNC_OPERATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLINEFILES_SYNC_STATE(pub i32);
pub const OFFLINEFILES_SYNC_STATE_Stable: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(0i32);
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(1i32);
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(2i32);
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(3i32);
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(4i32);
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(5i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(6i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(7i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(8i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(9i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(10i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(11i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(12i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(13i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(14i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(15i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(16i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(17i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(18i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(19i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(20i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(21i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(22i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(23i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(24i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(25i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(26i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(27i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(28i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(29i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(30i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(31i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(32i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(33i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(34i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(35i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(36i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(37i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(38i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(39i32);
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(40i32);
pub const OFFLINEFILES_SYNC_STATE_DirSparseOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(41i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(42i32);
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(43i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(44i32);
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(45i32);
pub const OFFLINEFILES_SYNC_STATE_FileDeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(46i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(47i32);
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(48i32);
pub const OFFLINEFILES_SYNC_STATE_DirDeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(49i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(50i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(51i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(52i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(53i32);
pub const OFFLINEFILES_SYNC_STATE_NUMSTATES: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(54i32);
impl ::std::convert::From<i32> for OFFLINEFILES_SYNC_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLINEFILES_SYNC_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_STATE_LOCAL_KNOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_SYNC_STATE_REMOTE_KNOWN: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_TRANSITION_FLAG_CONSOLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
pub const OFFLINEFILES_TRANSITION_FLAG_INTERACTIVE: u32 = 1u32;
pub const OfflineFilesCache: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1220984444, 14449, 17356, [180, 111, 20, 73, 161, 187, 47, 243]);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OfflineFilesEnable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(benable: Param0, pbrebootrequired: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OfflineFilesEnable(benable: super::super::Foundation::BOOL, pbrebootrequired: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(OfflineFilesEnable(benable.into_param().abi(), ::std::mem::transmute(pbrebootrequired)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OfflineFilesQueryStatus(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OfflineFilesQueryStatus(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(OfflineFilesQueryStatus(::std::mem::transmute(pbactive), ::std::mem::transmute(pbenabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OfflineFilesQueryStatusEx(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL, pbavailable: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OfflineFilesQueryStatusEx(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL, pbavailable: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(OfflineFilesQueryStatusEx(::std::mem::transmute(pbactive), ::std::mem::transmute(pbenabled), ::std::mem::transmute(pbavailable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const OfflineFilesSetting: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4248197609, 43296, 16675, [173, 100, 127, 199, 108, 122, 172, 223]);
#[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
#[inline]
pub unsafe fn OfflineFilesStart() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OfflineFilesStart() -> u32;
        }
        ::std::mem::transmute(OfflineFilesStart())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
