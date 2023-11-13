#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeed(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeed {
    pub unsafe fn Xml(&self, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Xml)(::windows_core::Interface::as_raw(self), count, sortproperty, sortorder, filterflags, includeflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Rename<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Rename)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Url(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Url)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUrl<P0>(&self, feedurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetUrl)(::windows_core::Interface::as_raw(self), feedurl.into_param().abi()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Move<P0>(&self, newparentpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), newparentpath.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWriteTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastWriteTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Download)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows_core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SyncSetting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSyncSetting(&self, syncsetting: FEEDS_SYNC_SETTING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSyncSetting)(::windows_core::Interface::as_raw(self), syncsetting).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Interval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInterval(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterval)(::windows_core::Interface::as_raw(self), minutes).ok()
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalEnclosurePath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Items(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Items)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetItem(&self, itemid: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetItem)(::windows_core::Interface::as_raw(self), itemid, &mut result__).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Link)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Image(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Image)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastBuildDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastBuildDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PubDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Ttl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Language(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Language)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Copyright)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, count: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxItemCount)(::windows_core::Interface::as_raw(self), count).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadEnclosuresAutomatically)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDownloadEnclosuresAutomatically<P0>(&self, downloadenclosuresautomatically: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetDownloadEnclosuresAutomatically)(::windows_core::Interface::as_raw(self), downloadenclosuresautomatically.into_param().abi()).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastDownloadError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Merge<P0, P1>(&self, feedxml: P0, feedurl: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Merge)(::windows_core::Interface::as_raw(self), feedxml.into_param().abi(), feedurl.into_param().abi()).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsList(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MarkAllItemsRead)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWatcher)(::windows_core::Interface::as_raw(self), scope, mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UnreadItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeed, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeed {
    type Vtable = IFeed_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeed {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7f915d8_2ede_42bc_98e7_a5d05063a757);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeed_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newparentpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    pub LastWriteTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastwrite: *mut f64) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CancelAsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SyncSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncsetting: *mut FEEDS_SYNC_SETTING) -> ::windows_core::HRESULT,
    pub SetSyncSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncsetting: FEEDS_SYNC_SETTING) -> ::windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows_core::HRESULT,
    pub LastDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows_core::HRESULT,
    pub LocalEnclosurePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Items: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetItem: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, homepage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LastBuildDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastbuilddate: *mut f64) -> ::windows_core::HRESULT,
    pub PubDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastpopulatedate: *mut f64) -> ::windows_core::HRESULT,
    pub Ttl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ttl: *mut i32) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Copyright: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copyright: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DownloadEnclosuresAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DownloadEnclosuresAutomatically: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDownloadEnclosuresAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDownloadEnclosuresAutomatically: usize,
    pub DownloadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows_core::HRESULT,
    pub LastDownloadError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT,
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedxml: ::std::mem::MaybeUninit<::windows_core::BSTR>, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, islist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsList: usize,
    pub MarkAllItemsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWatcher: usize,
    pub UnreadItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeed2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeed2 {
    pub unsafe fn Xml(&self, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Xml)(::windows_core::Interface::as_raw(self), count, sortproperty, sortorder, filterflags, includeflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Rename<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Rename)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Url(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Url)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUrl<P0>(&self, feedurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetUrl)(::windows_core::Interface::as_raw(self), feedurl.into_param().abi()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Move<P0>(&self, newparentpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Move)(::windows_core::Interface::as_raw(self), newparentpath.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWriteTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastWriteTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Download)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CancelAsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows_core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SyncSetting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSyncSetting(&self, syncsetting: FEEDS_SYNC_SETTING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSyncSetting)(::windows_core::Interface::as_raw(self), syncsetting).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Interval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInterval(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInterval)(::windows_core::Interface::as_raw(self), minutes).ok()
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalEnclosurePath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Items(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Items)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetItem(&self, itemid: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetItem)(::windows_core::Interface::as_raw(self), itemid, &mut result__).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Link)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Image(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Image)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastBuildDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastBuildDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PubDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Ttl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Language(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Language)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Copyright)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MaxItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, count: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMaxItemCount)(::windows_core::Interface::as_raw(self), count).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadEnclosuresAutomatically)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDownloadEnclosuresAutomatically<P0>(&self, downloadenclosuresautomatically: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDownloadEnclosuresAutomatically)(::windows_core::Interface::as_raw(self), downloadenclosuresautomatically.into_param().abi()).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastDownloadError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Merge<P0, P1>(&self, feedxml: P0, feedurl: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Merge)(::windows_core::Interface::as_raw(self), feedxml.into_param().abi(), feedurl.into_param().abi()).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsList(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MarkAllItemsRead)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWatcher)(::windows_core::Interface::as_raw(self), scope, mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UnreadItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetItemByEffectiveId(&self, itemeffectiveid: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetItemByEffectiveId)(::windows_core::Interface::as_raw(self), itemeffectiveid, &mut result__).from_abi(result__)
    }
    pub unsafe fn LastItemDownloadTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastItemDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Username(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Username)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Password(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Password)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCredentials<P0, P1>(&self, username: P0, password: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCredentials)(::windows_core::Interface::as_raw(self), username.into_param().abi(), password.into_param().abi()).ok()
    }
    pub unsafe fn ClearCredentials(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearCredentials)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeed2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFeed);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeed2 {
    type Vtable = IFeed2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeed2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33f2ea09_1398_4ab9_b6a4_f94b49d0a42e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeed2_Vtbl {
    pub base__: IFeed_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetItemByEffectiveId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemeffectiveid: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetItemByEffectiveId: usize,
    pub LastItemDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastitemdownloadtime: *mut f64) -> ::windows_core::HRESULT,
    pub Username: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, password: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ClearCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeedEnclosure(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedEnclosure {
    pub unsafe fn Url(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Url)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Length(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Length)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastDownloadError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LocalPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DownloadMimeType(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadMimeType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveFile(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveFile)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetFile<P0, P1, P2, P3>(&self, downloadurl: P0, downloadfilepath: P1, downloadmimetype: P2, enclosurefilename: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFile)(::windows_core::Interface::as_raw(self), downloadurl.into_param().abi(), downloadfilepath.into_param().abi(), downloadmimetype.into_param().abi(), enclosurefilename.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeedEnclosure, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeedEnclosure {
    type Vtable = IFeedEnclosure_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeedEnclosure {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x361c26f7_90a4_4e67_ae09_3a36a546436a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedEnclosure_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enclosureurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT,
    pub AsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CancelAsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DownloadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows_core::HRESULT,
    pub LastDownloadError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT,
    pub LocalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enclosureurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DownloadMimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RemoveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, downloadfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, downloadmimetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, enclosurefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeedEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedEvents {
    pub unsafe fn Error(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FeedDeleted<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDeleted)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn FeedRenamed<P0, P1>(&self, path: P0, oldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedRenamed)(::windows_core::Interface::as_raw(self), path.into_param().abi(), oldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedUrlChanged<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedUrlChanged)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn FeedMoved<P0, P1>(&self, path: P0, oldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedMoved)(::windows_core::Interface::as_raw(self), path.into_param().abi(), oldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedDownloading<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDownloading)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn FeedDownloadCompleted<P0>(&self, path: P0, error: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDownloadCompleted)(::windows_core::Interface::as_raw(self), path.into_param().abi(), error).ok()
    }
    pub unsafe fn FeedItemCountChanged<P0>(&self, path: P0, itemcounttype: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedItemCountChanged)(::windows_core::Interface::as_raw(self), path.into_param().abi(), itemcounttype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeedEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeedEvents {
    type Vtable = IFeedEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeedEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xabf35c99_0681_47ea_9a8c_1436a375a99e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FeedDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedUrlChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedDownloading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT,
    pub FeedItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, itemcounttype: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeedFolder(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedFolder {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Feeds(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Feeds)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subfolders(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Subfolders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFeed<P0, P1>(&self, feedname: P0, feedurl: P1) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateFeed)(::windows_core::Interface::as_raw(self), feedname.into_param().abi(), feedurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSubfolder<P0>(&self, foldername: P0) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSubfolder)(::windows_core::Interface::as_raw(self), foldername.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsFeed<P0>(&self, feedname: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExistsFeed)(::windows_core::Interface::as_raw(self), feedname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeed<P0>(&self, feedname: P0) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFeed)(::windows_core::Interface::as_raw(self), feedname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsSubfolder<P0>(&self, foldername: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExistsSubfolder)(::windows_core::Interface::as_raw(self), foldername.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSubfolder<P0>(&self, foldername: P0) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSubfolder)(::windows_core::Interface::as_raw(self), foldername.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Rename<P0>(&self, foldername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Rename)(::windows_core::Interface::as_raw(self), foldername.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Move<P0>(&self, newparentpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), newparentpath.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRoot(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRoot)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalUnreadItemCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalUnreadItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalItemCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWatcher)(::windows_core::Interface::as_raw(self), scope, mask, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeedFolder, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeedFolder {
    type Vtable = IFeedFolder_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeedFolder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81f04ad1_4194_4d7d_86d6_11813cec163c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedFolder_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Feeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Feeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Subfolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Subfolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedname: ::std::mem::MaybeUninit<::windows_core::BSTR>, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFeed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSubfolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedname: ::std::mem::MaybeUninit<::windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsFeed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedname: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsSubfolder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSubfolder: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newparentpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isroot: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRoot: usize,
    pub TotalUnreadItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub TotalItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWatcher: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeedFolderEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedFolderEvents {
    pub unsafe fn Error(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FolderAdded<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderAdded)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn FolderDeleted<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderDeleted)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn FolderRenamed<P0, P1>(&self, path: P0, oldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderRenamed)(::windows_core::Interface::as_raw(self), path.into_param().abi(), oldpath.into_param().abi()).ok()
    }
    pub unsafe fn FolderMovedFrom<P0, P1>(&self, path: P0, oldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderMovedFrom)(::windows_core::Interface::as_raw(self), path.into_param().abi(), oldpath.into_param().abi()).ok()
    }
    pub unsafe fn FolderMovedTo<P0, P1>(&self, path: P0, oldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderMovedTo)(::windows_core::Interface::as_raw(self), path.into_param().abi(), oldpath.into_param().abi()).ok()
    }
    pub unsafe fn FolderItemCountChanged<P0>(&self, path: P0, itemcounttype: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderItemCountChanged)(::windows_core::Interface::as_raw(self), path.into_param().abi(), itemcounttype).ok()
    }
    pub unsafe fn FeedAdded<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedAdded)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn FeedDeleted<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDeleted)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn FeedRenamed<P0, P1>(&self, path: P0, oldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedRenamed)(::windows_core::Interface::as_raw(self), path.into_param().abi(), oldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedUrlChanged<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedUrlChanged)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn FeedMovedFrom<P0, P1>(&self, path: P0, oldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedMovedFrom)(::windows_core::Interface::as_raw(self), path.into_param().abi(), oldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedMovedTo<P0, P1>(&self, path: P0, oldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedMovedTo)(::windows_core::Interface::as_raw(self), path.into_param().abi(), oldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedDownloading<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDownloading)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn FeedDownloadCompleted<P0>(&self, path: P0, error: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDownloadCompleted)(::windows_core::Interface::as_raw(self), path.into_param().abi(), error).ok()
    }
    pub unsafe fn FeedItemCountChanged<P0>(&self, path: P0, itemcounttype: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedItemCountChanged)(::windows_core::Interface::as_raw(self), path.into_param().abi(), itemcounttype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeedFolderEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeedFolderEvents {
    type Vtable = IFeedFolderEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeedFolderEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20a59fa6_a844_4630_9e98_175f70b4d55b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedFolderEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FolderAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FolderDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FolderRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FolderMovedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FolderMovedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FolderItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, itemcounttype: i32) -> ::windows_core::HRESULT,
    pub FeedAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedUrlChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedMovedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedMovedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedDownloading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FeedDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT,
    pub FeedItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, itemcounttype: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeedItem(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedItem {
    pub unsafe fn Xml(&self, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Xml)(::windows_core::Interface::as_raw(self), includeflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Link)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Guid)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PubDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Comments)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Author(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Author)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enclosure(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enclosure)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRead(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRead)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRead<P0>(&self, isread: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsRead)(::windows_core::Interface::as_raw(self), isread.into_param().abi()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Modified(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Modified)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeedItem, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeedItem {
    type Vtable = IFeedItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeedItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a1e6cad_0a47_4da2_a13d_5baaa5c8bd4f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linkurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PubDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pubdate: *mut f64) -> ::windows_core::HRESULT,
    pub Comments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comments: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, author: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Enclosure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Enclosure: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isread: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRead: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isread: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsRead: usize,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LastDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows_core::HRESULT,
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *mut f64) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeedItem2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedItem2 {
    pub unsafe fn Xml(&self, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Xml)(::windows_core::Interface::as_raw(self), includeflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Link)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Guid)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PubDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Comments)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Author(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Author)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enclosure(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Enclosure)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRead(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRead)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRead<P0>(&self, isread: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetIsRead)(::windows_core::Interface::as_raw(self), isread.into_param().abi()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Modified(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Modified)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EffectiveId(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EffectiveId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeedItem2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFeedItem);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeedItem2 {
    type Vtable = IFeedItem2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeedItem2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79ac9ef4_f9c1_4d2b_a50b_a7ffba4dcf37);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedItem2_Vtbl {
    pub base__: IFeedItem_Vtbl,
    pub EffectiveId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectiveid: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeedsEnum(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedsEnum {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeedsEnum, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeedsEnum {
    type Vtable = IFeedsEnum_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeedsEnum {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3cd0028_2eed_4c60_8fae_a3225309a836);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedsEnum_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFeedsManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedsManager {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootFolder(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RootFolder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<P0>(&self, feedurl: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsSubscribed)(::windows_core::Interface::as_raw(self), feedurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsFeed<P0>(&self, feedpath: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExistsFeed)(::windows_core::Interface::as_raw(self), feedpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeed<P0>(&self, feedpath: P0) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFeed)(::windows_core::Interface::as_raw(self), feedpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeedByUrl<P0>(&self, feedurl: P0) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFeedByUrl)(::windows_core::Interface::as_raw(self), feedurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsFolder<P0>(&self, folderpath: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExistsFolder)(::windows_core::Interface::as_raw(self), folderpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFolder<P0>(&self, folderpath: P0) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFolder)(::windows_core::Interface::as_raw(self), folderpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteFeed<P0>(&self, feedpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteFeed)(::windows_core::Interface::as_raw(self), feedpath.into_param().abi()).ok()
    }
    pub unsafe fn DeleteFolder<P0>(&self, folderpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteFolder)(::windows_core::Interface::as_raw(self), folderpath.into_param().abi()).ok()
    }
    pub unsafe fn BackgroundSync(&self, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackgroundSync)(::windows_core::Interface::as_raw(self), action).ok()
    }
    pub unsafe fn BackgroundSyncStatus(&self) -> ::windows_core::Result<FEEDS_BACKGROUNDSYNC_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BackgroundSyncStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DefaultInterval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DefaultInterval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultInterval(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultInterval)(::windows_core::Interface::as_raw(self), minutes).ok()
    }
    pub unsafe fn AsyncSyncAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncSyncAll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Normalize<P0>(&self, feedxmlin: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Normalize)(::windows_core::Interface::as_raw(self), feedxmlin.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ItemCountLimit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ItemCountLimit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFeedsManager, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFeedsManager {
    type Vtable = IFeedsManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFeedsManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa74029cc_1f1a_4906_88f0_810638d86591);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedsManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RootFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootFolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSubscribed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, subscribed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSubscribed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsFeed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeedByUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeedByUrl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsFolder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFolder: usize,
    pub DeleteFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BackgroundSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows_core::HRESULT,
    pub BackgroundSyncStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows_core::HRESULT,
    pub DefaultInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows_core::HRESULT,
    pub SetDefaultInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows_core::HRESULT,
    pub AsyncSyncAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Normalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedxmlin: ::std::mem::MaybeUninit<::windows_core::BSTR>, feedxmlout: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ItemCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemcountlimit: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPAudioRenderConfig(::windows_core::IUnknown);
impl IWMPAudioRenderConfig {
    pub unsafe fn audioOutputDevice(&self, pbstroutputdevice: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).audioOutputDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstroutputdevice)).ok()
    }
    pub unsafe fn SetaudioOutputDevice<P0>(&self, bstroutputdevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetaudioOutputDevice)(::windows_core::Interface::as_raw(self), bstroutputdevice.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPAudioRenderConfig, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPAudioRenderConfig {
    type Vtable = IWMPAudioRenderConfig_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPAudioRenderConfig {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79c6349_5997_4ce4_917c_22a3391ec564);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPAudioRenderConfig_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub audioOutputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstroutputdevice: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetaudioOutputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroutputdevice: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPCdrom(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCdrom {
    pub unsafe fn driveSpecifier(&self, pbstrdrive: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).driveSpecifier)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdrive)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlist(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).playlist)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn eject(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).eject)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPCdrom, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPCdrom {
    type Vtable = IWMPCdrom_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPCdrom {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfab6e98_8730_11d3_b388_00c04f68574b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdrom_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub driveSpecifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdrive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub playlist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    playlist: usize,
    pub eject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPCdromBurn(::windows_core::IUnknown);
impl IWMPCdromBurn {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isAvailable<P0>(&self, bstritem: P0, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).isAvailable)(::windows_core::Interface::as_raw(self), bstritem.into_param().abi(), pisavailable).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, bstritem: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getItemInfo)(::windows_core::Interface::as_raw(self), bstritem.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn label(&self, pbstrlabel: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).label)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrlabel)).ok()
    }
    pub unsafe fn Setlabel<P0>(&self, bstrlabel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Setlabel)(::windows_core::Interface::as_raw(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn burnFormat(&self, pwmpbf: *mut WMPBurnFormat) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).burnFormat)(::windows_core::Interface::as_raw(self), pwmpbf).ok()
    }
    pub unsafe fn SetburnFormat(&self, wmpbf: WMPBurnFormat) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetburnFormat)(::windows_core::Interface::as_raw(self), wmpbf).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn burnPlaylist(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).burnPlaylist)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetburnPlaylist<P0>(&self, pplaylist: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).SetburnPlaylist)(::windows_core::Interface::as_raw(self), pplaylist.into_param().abi()).ok()
    }
    pub unsafe fn refreshStatus(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).refreshStatus)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn burnState(&self, pwmpbs: *mut WMPBurnState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).burnState)(::windows_core::Interface::as_raw(self), pwmpbs).ok()
    }
    pub unsafe fn burnProgress(&self, plprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).burnProgress)(::windows_core::Interface::as_raw(self), plprogress).ok()
    }
    pub unsafe fn startBurn(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).startBurn)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn stopBurn(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).stopBurn)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn erase(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).erase)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPCdromBurn, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPCdromBurn {
    type Vtable = IWMPCdromBurn_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPCdromBurn {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd94dbeb_417f_4928_aa06_087d56ed9b59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdromBurn_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub isAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isAvailable: usize,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Setlabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub burnFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> ::windows_core::HRESULT,
    pub SetburnFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmpbf: WMPBurnFormat) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub burnPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    burnPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetburnPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplaylist: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetburnPlaylist: usize,
    pub refreshStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub burnState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> ::windows_core::HRESULT,
    pub burnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT,
    pub startBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub stopBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub erase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPCdromCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCdromCollection {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).count)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn item(&self, lindex: i32) -> ::windows_core::Result<IWMPCdrom> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByDriveSpecifier<P0>(&self, bstrdrivespecifier: P0) -> ::windows_core::Result<IWMPCdrom>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getByDriveSpecifier)(::windows_core::Interface::as_raw(self), bstrdrivespecifier.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPCdromCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPCdromCollection {
    type Vtable = IWMPCdromCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPCdromCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee4c8fe2_34b2_11d3_a3bf_006097c9b344);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdromCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByDriveSpecifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdrivespecifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcdrom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByDriveSpecifier: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPCdromRip(::windows_core::IUnknown);
impl IWMPCdromRip {
    pub unsafe fn ripState(&self, pwmprs: *mut WMPRipState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ripState)(::windows_core::Interface::as_raw(self), pwmprs).ok()
    }
    pub unsafe fn ripProgress(&self, plprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ripProgress)(::windows_core::Interface::as_raw(self), plprogress).ok()
    }
    pub unsafe fn startRip(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).startRip)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn stopRip(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).stopRip)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPCdromRip, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPCdromRip {
    type Vtable = IWMPCdromRip_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPCdromRip {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56e2294f_69ed_4629_a869_aea72c0dcc2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdromRip_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ripState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmprs: *mut WMPRipState) -> ::windows_core::HRESULT,
    pub ripProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT,
    pub startRip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub stopRip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPClosedCaption(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPClosedCaption {
    pub unsafe fn SAMIStyle(&self, pbstrsamistyle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SAMIStyle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsamistyle)).ok()
    }
    pub unsafe fn SetSAMIStyle<P0>(&self, bstrsamistyle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSAMIStyle)(::windows_core::Interface::as_raw(self), bstrsamistyle.into_param().abi()).ok()
    }
    pub unsafe fn SAMILang(&self, pbstrsamilang: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SAMILang)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsamilang)).ok()
    }
    pub unsafe fn SetSAMILang<P0>(&self, bstrsamilang: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSAMILang)(::windows_core::Interface::as_raw(self), bstrsamilang.into_param().abi()).ok()
    }
    pub unsafe fn SAMIFileName(&self, pbstrsamifilename: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SAMIFileName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsamifilename)).ok()
    }
    pub unsafe fn SetSAMIFileName<P0>(&self, bstrsamifilename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSAMIFileName)(::windows_core::Interface::as_raw(self), bstrsamifilename.into_param().abi()).ok()
    }
    pub unsafe fn captioningId(&self, pbstrcaptioningid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).captioningId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcaptioningid)).ok()
    }
    pub unsafe fn SetcaptioningId<P0>(&self, bstrcaptioningid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetcaptioningId)(::windows_core::Interface::as_raw(self), bstrcaptioningid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPClosedCaption, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPClosedCaption {
    type Vtable = IWMPClosedCaption_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPClosedCaption {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f2df574_c588_11d3_9ed0_00c04fb6e937);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPClosedCaption_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SAMIStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsamistyle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSAMIStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsamistyle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SAMILang: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsamilang: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSAMILang: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsamilang: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SAMIFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsamifilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSAMIFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsamifilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub captioningId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcaptioningid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetcaptioningId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcaptioningid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPClosedCaption2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPClosedCaption2 {
    pub unsafe fn SAMIStyle(&self, pbstrsamistyle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SAMIStyle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsamistyle)).ok()
    }
    pub unsafe fn SetSAMIStyle<P0>(&self, bstrsamistyle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetSAMIStyle)(::windows_core::Interface::as_raw(self), bstrsamistyle.into_param().abi()).ok()
    }
    pub unsafe fn SAMILang(&self, pbstrsamilang: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SAMILang)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsamilang)).ok()
    }
    pub unsafe fn SetSAMILang<P0>(&self, bstrsamilang: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetSAMILang)(::windows_core::Interface::as_raw(self), bstrsamilang.into_param().abi()).ok()
    }
    pub unsafe fn SAMIFileName(&self, pbstrsamifilename: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SAMIFileName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsamifilename)).ok()
    }
    pub unsafe fn SetSAMIFileName<P0>(&self, bstrsamifilename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetSAMIFileName)(::windows_core::Interface::as_raw(self), bstrsamifilename.into_param().abi()).ok()
    }
    pub unsafe fn captioningId(&self, pbstrcaptioningid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.captioningId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcaptioningid)).ok()
    }
    pub unsafe fn SetcaptioningId<P0>(&self, bstrcaptioningid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetcaptioningId)(::windows_core::Interface::as_raw(self), bstrcaptioningid.into_param().abi()).ok()
    }
    pub unsafe fn SAMILangCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SAMILangCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn getSAMILangName(&self, nindex: i32, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getSAMILangName)(::windows_core::Interface::as_raw(self), nindex, ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn getSAMILangID(&self, nindex: i32, pllangid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getSAMILangID)(::windows_core::Interface::as_raw(self), nindex, pllangid).ok()
    }
    pub unsafe fn SAMIStyleCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SAMIStyleCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn getSAMIStyleName(&self, nindex: i32, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getSAMIStyleName)(::windows_core::Interface::as_raw(self), nindex, ::core::mem::transmute(pbstrname)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPClosedCaption2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPClosedCaption);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPClosedCaption2 {
    type Vtable = IWMPClosedCaption2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPClosedCaption2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x350ba78b_6bc8_4113_a5f5_312056934eb6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPClosedCaption2_Vtbl {
    pub base__: IWMPClosedCaption_Vtbl,
    pub SAMILangCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub getSAMILangName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub getSAMILangID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> ::windows_core::HRESULT,
    pub SAMIStyleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub getSAMIStyleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPContentContainer(::windows_core::IUnknown);
impl IWMPContentContainer {
    pub unsafe fn GetID(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrice(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPrice)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContentCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContentPrice(&self, idxcontent: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentPrice)(::windows_core::Interface::as_raw(self), idxcontent, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContentID(&self, idxcontent: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentID)(::windows_core::Interface::as_raw(self), idxcontent, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMPContentContainer, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPContentContainer {
    type Vtable = IWMPContentContainer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPContentContainer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad7f4d9c_1a9f_4ed2_9815_ecc0b58cb616);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPContentContainer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontentid: *mut u32) -> ::windows_core::HRESULT,
    pub GetPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprice: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetContentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccontent: *mut u32) -> ::windows_core::HRESULT,
    pub GetContentPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idxcontent: u32, pbstrprice: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetContentID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idxcontent: u32, pcontentid: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPContentContainerList(::windows_core::IUnknown);
impl IWMPContentContainerList {
    pub unsafe fn GetTransactionType(&self) -> ::windows_core::Result<WMPTransactionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransactionType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContainerCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContainerCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContainer(&self, idxcontainer: u32) -> ::windows_core::Result<IWMPContentContainer> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContainer)(::windows_core::Interface::as_raw(self), idxcontainer, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMPContentContainerList, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPContentContainerList {
    type Vtable = IWMPContentContainerList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPContentContainerList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9937f78_0802_4af8_8b8d_e3f045bc8ab5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPContentContainerList_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetTransactionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmptt: *mut WMPTransactionType) -> ::windows_core::HRESULT,
    pub GetContainerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccontainer: *mut u32) -> ::windows_core::HRESULT,
    pub GetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idxcontainer: u32, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPContentPartner(::windows_core::IUnknown);
impl IWMPContentPartner {
    pub unsafe fn SetCallback<P0>(&self, pcallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPContentPartnerCallback>,
    {
        (::windows_core::Interface::vtable(self).SetCallback)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Notify(&self, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), r#type, pcontext).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetItemInfo<P0>(&self, bstrinfoname: P0, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetItemInfo)(::windows_core::Interface::as_raw(self), bstrinfoname.into_param().abi(), pcontext, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetContentPartnerInfo<P0>(&self, bstrinfoname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentPartnerInfo)(::windows_core::Interface::as_raw(self), bstrinfoname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetCommands<P0, P1>(&self, location: P0, plocationcontext: *const super::super::System::Variant::VARIANT, itemlocation: P1, prgitemids: &[u32], pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).GetCommands)(::windows_core::Interface::as_raw(self), location.into_param().abi(), plocationcontext, itemlocation.into_param().abi(), prgitemids.len().try_into().unwrap(), ::core::mem::transmute(prgitemids.as_ptr()), pcitemids, pprgitems).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn InvokeCommand<P0, P1>(&self, dwcommandid: u32, location: P0, plocationcontext: *const super::super::System::Variant::VARIANT, itemlocation: P1, rgitemids: &[u32]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).InvokeCommand)(::windows_core::Interface::as_raw(self), dwcommandid, location.into_param().abi(), plocationcontext, itemlocation.into_param().abi(), rgitemids.len().try_into().unwrap(), ::core::mem::transmute(rgitemids.as_ptr())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanBuySilent<P0>(&self, pinfo: P0, pbstrtotalprice: *mut ::windows_core::BSTR, psilentok: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPContentContainerList>,
    {
        (::windows_core::Interface::vtable(self).CanBuySilent)(::windows_core::Interface::as_raw(self), pinfo.into_param().abi(), ::core::mem::transmute(pbstrtotalprice), psilentok).ok()
    }
    pub unsafe fn Buy<P0>(&self, pinfo: P0, cookie: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPContentContainerList>,
    {
        (::windows_core::Interface::vtable(self).Buy)(::windows_core::Interface::as_raw(self), pinfo.into_param().abi(), cookie).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetStreamingURL(&self, st: WMPStreamingType, pstreamcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStreamingURL)(::windows_core::Interface::as_raw(self), st, pstreamcontext, &mut result__).from_abi(result__)
    }
    pub unsafe fn Download<P0>(&self, pinfo: P0, cookie: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPContentContainerList>,
    {
        (::windows_core::Interface::vtable(self).Download)(::windows_core::Interface::as_raw(self), pinfo.into_param().abi(), cookie).ok()
    }
    pub unsafe fn DownloadTrackComplete<P0>(&self, hrresult: ::windows_core::HRESULT, contentid: u32, downloadtrackparam: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DownloadTrackComplete)(::windows_core::Interface::as_raw(self), hrresult, contentid, downloadtrackparam.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RefreshLicense<P0, P1, P2>(&self, dwcookie: u32, flocal: P0, bstrurl: P1, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: P2, preasoncontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RefreshLicense)(::windows_core::Interface::as_raw(self), dwcookie, flocal.into_param().abi(), bstrurl.into_param().abi(), r#type, contentid, bstrrefreshreason.into_param().abi(), preasoncontext).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetCatalogURL(&self, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut ::windows_core::BSTR, pexpirationdate: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCatalogURL)(::windows_core::Interface::as_raw(self), dwcatalogversion, dwcatalogschemaversion, cataloglcid, pdwnewcatalogversion, ::core::mem::transmute(pbstrcatalogurl), pexpirationdate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetTemplate<P0, P1, P2, P3>(&self, task: WMPTaskType, location: P0, pcontext: *const super::super::System::Variant::VARIANT, clicklocation: P1, pclickcontext: *const super::super::System::Variant::VARIANT, bstrfilter: P2, bstrviewparams: P3, pbstrtemplateurl: *mut ::windows_core::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).GetTemplate)(::windows_core::Interface::as_raw(self), task, location.into_param().abi(), pcontext, clicklocation.into_param().abi(), pclickcontext, bstrfilter.into_param().abi(), bstrviewparams.into_param().abi(), ::core::mem::transmute(pbstrtemplateurl), ptemplatesize).ok()
    }
    pub unsafe fn UpdateDevice<P0>(&self, bstrdevicename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).UpdateDevice)(::windows_core::Interface::as_raw(self), bstrdevicename.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetListContents<P0, P1, P2>(&self, location: P0, pcontext: *const super::super::System::Variant::VARIANT, bstrlisttype: P1, bstrparams: P2, dwlistcookie: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).GetListContents)(::windows_core::Interface::as_raw(self), location.into_param().abi(), pcontext, bstrlisttype.into_param().abi(), bstrparams.into_param().abi(), dwlistcookie).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Login<P0, P1>(&self, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: P0, foktocache: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Login)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(userinfo), ::core::mem::transmute(pwdinfo), fusedcachedcreds.into_param().abi(), foktocache.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Authenticate(&self, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Authenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(userinfo), ::core::mem::transmute(pwdinfo)).ok()
    }
    pub unsafe fn Logout(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Logout)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendMessage<P0, P1>(&self, bstrmsg: P0, bstrparam: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SendMessage)(::windows_core::Interface::as_raw(self), bstrmsg.into_param().abi(), bstrparam.into_param().abi()).ok()
    }
    pub unsafe fn StationEvent<P0, P1>(&self, bstrstationeventtype: P0, stationid: u32, playlistindex: u32, trackid: u32, trackdata: P1, dwsecondsplayed: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).StationEvent)(::windows_core::Interface::as_raw(self), bstrstationeventtype.into_param().abi(), stationid, playlistindex, trackid, trackdata.into_param().abi(), dwsecondsplayed).ok()
    }
    pub unsafe fn CompareContainerListPrices<P0, P1>(&self, plistbase: P0, plistcompare: P1) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<IWMPContentContainerList>,
        P1: ::windows_core::IntoParam<IWMPContentContainerList>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CompareContainerListPrices)(::windows_core::Interface::as_raw(self), plistbase.into_param().abi(), plistcompare.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn VerifyPermission<P0>(&self, bstrpermission: P0, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).VerifyPermission)(::windows_core::Interface::as_raw(self), bstrpermission.into_param().abi(), pcontext).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPContentPartner, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPContentPartner {
    type Vtable = IWMPContentPartner_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPContentPartner {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55455073_41b5_4e75_87b8_f13bdb291d08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPContentPartner_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Notify: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinfoname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT, pdata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetItemInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetContentPartnerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinfoname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetContentPartnerInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::std::mem::MaybeUninit<::windows_core::BSTR>, plocationcontext: *const super::super::System::Variant::VARIANT, itemlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetCommands: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub InvokeCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcommandid: u32, location: ::std::mem::MaybeUninit<::windows_core::BSTR>, plocationcontext: *const super::super::System::Variant::VARIANT, itemlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, citemids: u32, rgitemids: *const u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    InvokeCommand: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanBuySilent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, pbstrtotalprice: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, psilentok: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanBuySilent: usize,
    pub Buy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetStreamingURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, st: WMPStreamingType, pstreamcontext: *const super::super::System::Variant::VARIANT, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetStreamingURL: usize,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT,
    pub DownloadTrackComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT, contentid: u32, downloadtrackparam: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RefreshLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32, flocal: super::super::Foundation::VARIANT_BOOL, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: ::std::mem::MaybeUninit<::windows_core::BSTR>, preasoncontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RefreshLicense: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetCatalogURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pexpirationdate: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetCatalogURL: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: WMPTaskType, location: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT, clicklocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, pclickcontext: *const super::super::System::Variant::VARIANT, bstrfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrviewparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrtemplateurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ptemplatesize: *mut WMPTemplateSize) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetTemplate: usize,
    pub UpdateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetListContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT, bstrlisttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwlistcookie: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetListContents: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Login: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: super::super::Foundation::VARIANT_BOOL, foktocache: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Login: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Authenticate: usize,
    pub Logout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmsg: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrparam: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub StationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstationeventtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, stationid: u32, playlistindex: u32, trackid: u32, trackdata: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsecondsplayed: u32) -> ::windows_core::HRESULT,
    pub CompareContainerListPrices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plistbase: *mut ::core::ffi::c_void, plistcompare: *mut ::core::ffi::c_void, presult: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub VerifyPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpermission: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    VerifyPermission: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPContentPartnerCallback(::windows_core::IUnknown);
impl IWMPContentPartnerCallback {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Notify(&self, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), r#type, pcontext).ok()
    }
    pub unsafe fn BuyComplete(&self, hrresult: ::windows_core::HRESULT, dwbuycookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BuyComplete)(::windows_core::Interface::as_raw(self), hrresult, dwbuycookie).ok()
    }
    pub unsafe fn DownloadTrack<P0, P1>(&self, cookie: u32, bstrtrackurl: P0, dwservicetrackid: u32, bstrdownloadparams: P1, hrdownload: ::windows_core::HRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DownloadTrack)(::windows_core::Interface::as_raw(self), cookie, bstrtrackurl.into_param().abi(), dwservicetrackid, bstrdownloadparams.into_param().abi(), hrdownload).ok()
    }
    pub unsafe fn GetCatalogVersion(&self, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCatalogVersion)(::windows_core::Interface::as_raw(self), pdwversion, pdwschemaversion, plcid).ok()
    }
    pub unsafe fn UpdateDeviceComplete<P0>(&self, bstrdevicename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).UpdateDeviceComplete)(::windows_core::Interface::as_raw(self), bstrdevicename.into_param().abi()).ok()
    }
    pub unsafe fn ChangeView<P0, P1, P2>(&self, bstrtype: P0, bstrid: P1, bstrfilter: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ChangeView)(::windows_core::Interface::as_raw(self), bstrtype.into_param().abi(), bstrid.into_param().abi(), bstrfilter.into_param().abi()).ok()
    }
    pub unsafe fn AddListContents(&self, dwlistcookie: u32, prgitems: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddListContents)(::windows_core::Interface::as_raw(self), dwlistcookie, prgitems.len().try_into().unwrap(), ::core::mem::transmute(prgitems.as_ptr())).ok()
    }
    pub unsafe fn ListContentsComplete(&self, dwlistcookie: u32, hrsuccess: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ListContentsComplete)(::windows_core::Interface::as_raw(self), dwlistcookie, hrsuccess).ok()
    }
    pub unsafe fn SendMessageComplete<P0, P1, P2>(&self, bstrmsg: P0, bstrparam: P1, bstrresult: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SendMessageComplete)(::windows_core::Interface::as_raw(self), bstrmsg.into_param().abi(), bstrparam.into_param().abi(), bstrresult.into_param().abi()).ok()
    }
    pub unsafe fn GetContentIDsInLibrary(&self, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetContentIDsInLibrary)(::windows_core::Interface::as_raw(self), pccontentids, pprgids).ok()
    }
    pub unsafe fn RefreshLicenseComplete(&self, dwcookie: u32, contentid: u32, hrrefresh: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RefreshLicenseComplete)(::windows_core::Interface::as_raw(self), dwcookie, contentid, hrrefresh).ok()
    }
    pub unsafe fn ShowPopup<P0>(&self, lindex: i32, bstrparameters: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ShowPopup)(::windows_core::Interface::as_raw(self), lindex, bstrparameters.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn VerifyPermissionComplete<P0>(&self, bstrpermission: P0, pcontext: *const super::super::System::Variant::VARIANT, hrpermission: ::windows_core::HRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).VerifyPermissionComplete)(::windows_core::Interface::as_raw(self), bstrpermission.into_param().abi(), pcontext, hrpermission).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPContentPartnerCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPContentPartnerCallback {
    type Vtable = IWMPContentPartnerCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPContentPartnerCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e8f7da2_0695_403c_b697_da10fafaa676);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPContentPartnerCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Notify: usize,
    pub BuyComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT, dwbuycookie: u32) -> ::windows_core::HRESULT,
    pub DownloadTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32, bstrtrackurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwservicetrackid: u32, bstrdownloadparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, hrdownload: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetCatalogVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows_core::HRESULT,
    pub UpdateDeviceComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ChangeView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AddListContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows_core::HRESULT,
    pub ListContentsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlistcookie: u32, hrsuccess: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub SendMessageComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmsg: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrparam: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresult: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetContentIDsInLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows_core::HRESULT,
    pub RefreshLicenseComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32, contentid: u32, hrrefresh: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub ShowPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, bstrparameters: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub VerifyPermissionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpermission: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT, hrpermission: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    VerifyPermissionComplete: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPControls(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_isAvailable<P0>(&self, bstritem: P0, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).get_isAvailable)(::windows_core::Interface::as_raw(self), bstritem.into_param().abi(), pisavailable).ok()
    }
    pub unsafe fn play(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).play)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn fastForward(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).fastForward)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn fastReverse(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).fastReverse)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn currentPosition(&self, pdcurrentposition: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).currentPosition)(::windows_core::Interface::as_raw(self), pdcurrentposition).ok()
    }
    pub unsafe fn SetcurrentPosition(&self, dcurrentposition: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetcurrentPosition)(::windows_core::Interface::as_raw(self), dcurrentposition).ok()
    }
    pub unsafe fn currentPositionString(&self, pbstrcurrentposition: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).currentPositionString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcurrentposition)).ok()
    }
    pub unsafe fn next(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).next)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn previous(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).previous)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentItem(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).currentItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentItem<P0>(&self, piwmpmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).SetcurrentItem)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi()).ok()
    }
    pub unsafe fn currentMarker(&self, plmarker: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).currentMarker)(::windows_core::Interface::as_raw(self), plmarker).ok()
    }
    pub unsafe fn SetcurrentMarker(&self, lmarker: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetcurrentMarker)(::windows_core::Interface::as_raw(self), lmarker).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playItem<P0>(&self, piwmpmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).playItem)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPControls, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPControls {
    type Vtable = IWMPControls_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPControls {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74c09e02_f828_11d2_a74b_00a0c905f36e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPControls_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_isAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_isAvailable: usize,
    pub play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub fastForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub fastReverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub currentPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcurrentposition: *mut f64) -> ::windows_core::HRESULT,
    pub SetcurrentPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dcurrentposition: f64) -> ::windows_core::HRESULT,
    pub currentPositionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcurrentposition: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub previous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub currentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwmpmedia: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    currentItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetcurrentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetcurrentItem: usize,
    pub currentMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmarker: *mut i32) -> ::windows_core::HRESULT,
    pub SetcurrentMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmarker: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub playItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    playItem: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPControls2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls2 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_isAvailable<P0>(&self, bstritem: P0, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.get_isAvailable)(::windows_core::Interface::as_raw(self), bstritem.into_param().abi(), pisavailable).ok()
    }
    pub unsafe fn play(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.play)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn fastForward(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.fastForward)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn fastReverse(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.fastReverse)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn currentPosition(&self, pdcurrentposition: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.currentPosition)(::windows_core::Interface::as_raw(self), pdcurrentposition).ok()
    }
    pub unsafe fn SetcurrentPosition(&self, dcurrentposition: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetcurrentPosition)(::windows_core::Interface::as_raw(self), dcurrentposition).ok()
    }
    pub unsafe fn currentPositionString(&self, pbstrcurrentposition: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.currentPositionString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcurrentposition)).ok()
    }
    pub unsafe fn next(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.next)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn previous(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.previous)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentItem(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.currentItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentItem<P0>(&self, piwmpmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.SetcurrentItem)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi()).ok()
    }
    pub unsafe fn currentMarker(&self, plmarker: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.currentMarker)(::windows_core::Interface::as_raw(self), plmarker).ok()
    }
    pub unsafe fn SetcurrentMarker(&self, lmarker: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetcurrentMarker)(::windows_core::Interface::as_raw(self), lmarker).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playItem<P0>(&self, piwmpmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.playItem)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi()).ok()
    }
    pub unsafe fn step(&self, lstep: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).step)(::windows_core::Interface::as_raw(self), lstep).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPControls2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPControls);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPControls2 {
    type Vtable = IWMPControls2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPControls2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f030d25_0890_480f_9775_1f7e40ab5b8e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPControls2_Vtbl {
    pub base__: IWMPControls_Vtbl,
    pub step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstep: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPControls3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls3 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_isAvailable<P0>(&self, bstritem: P0, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.get_isAvailable)(::windows_core::Interface::as_raw(self), bstritem.into_param().abi(), pisavailable).ok()
    }
    pub unsafe fn play(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.play)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn fastForward(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.fastForward)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn fastReverse(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.fastReverse)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn currentPosition(&self, pdcurrentposition: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.currentPosition)(::windows_core::Interface::as_raw(self), pdcurrentposition).ok()
    }
    pub unsafe fn SetcurrentPosition(&self, dcurrentposition: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetcurrentPosition)(::windows_core::Interface::as_raw(self), dcurrentposition).ok()
    }
    pub unsafe fn currentPositionString(&self, pbstrcurrentposition: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.currentPositionString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcurrentposition)).ok()
    }
    pub unsafe fn next(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.next)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn previous(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.previous)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentItem(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.currentItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentItem<P0>(&self, piwmpmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetcurrentItem)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi()).ok()
    }
    pub unsafe fn currentMarker(&self, plmarker: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.currentMarker)(::windows_core::Interface::as_raw(self), plmarker).ok()
    }
    pub unsafe fn SetcurrentMarker(&self, lmarker: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetcurrentMarker)(::windows_core::Interface::as_raw(self), lmarker).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playItem<P0>(&self, piwmpmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.playItem)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi()).ok()
    }
    pub unsafe fn step(&self, lstep: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.step)(::windows_core::Interface::as_raw(self), lstep).ok()
    }
    pub unsafe fn audioLanguageCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).audioLanguageCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn getAudioLanguageID(&self, lindex: i32, pllangid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getAudioLanguageID)(::windows_core::Interface::as_raw(self), lindex, pllangid).ok()
    }
    pub unsafe fn getAudioLanguageDescription(&self, lindex: i32, pbstrlangdesc: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getAudioLanguageDescription)(::windows_core::Interface::as_raw(self), lindex, ::core::mem::transmute(pbstrlangdesc)).ok()
    }
    pub unsafe fn currentAudioLanguage(&self, pllangid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).currentAudioLanguage)(::windows_core::Interface::as_raw(self), pllangid).ok()
    }
    pub unsafe fn SetcurrentAudioLanguage(&self, llangid: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetcurrentAudioLanguage)(::windows_core::Interface::as_raw(self), llangid).ok()
    }
    pub unsafe fn currentAudioLanguageIndex(&self, plindex: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).currentAudioLanguageIndex)(::windows_core::Interface::as_raw(self), plindex).ok()
    }
    pub unsafe fn SetcurrentAudioLanguageIndex(&self, lindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetcurrentAudioLanguageIndex)(::windows_core::Interface::as_raw(self), lindex).ok()
    }
    pub unsafe fn getLanguageName(&self, llangid: i32, pbstrlangname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getLanguageName)(::windows_core::Interface::as_raw(self), llangid, ::core::mem::transmute(pbstrlangname)).ok()
    }
    pub unsafe fn currentPositionTimecode(&self, bstrtimecode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).currentPositionTimecode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bstrtimecode)).ok()
    }
    pub unsafe fn SetcurrentPositionTimecode<P0>(&self, bstrtimecode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetcurrentPositionTimecode)(::windows_core::Interface::as_raw(self), bstrtimecode.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPControls3, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPControls, IWMPControls2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPControls3 {
    type Vtable = IWMPControls3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPControls3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1d1110e_d545_476a_9a78_ac3e4cb1e6bd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPControls3_Vtbl {
    pub base__: IWMPControls2_Vtbl,
    pub audioLanguageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub getAudioLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> ::windows_core::HRESULT,
    pub getAudioLanguageDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub currentAudioLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows_core::HRESULT,
    pub SetcurrentAudioLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llangid: i32) -> ::windows_core::HRESULT,
    pub currentAudioLanguageIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows_core::HRESULT,
    pub SetcurrentAudioLanguageIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT,
    pub getLanguageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llangid: i32, pbstrlangname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub currentPositionTimecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtimecode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetcurrentPositionTimecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtimecode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPConvert(::windows_core::IUnknown);
impl IWMPConvert {
    pub unsafe fn ConvertFile<P0, P1>(&self, bstrinputfile: P0, bstrdestinationfolder: P1, pbstroutputfile: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ConvertFile)(::windows_core::Interface::as_raw(self), bstrinputfile.into_param().abi(), bstrdestinationfolder.into_param().abi(), ::core::mem::transmute(pbstroutputfile)).ok()
    }
    pub unsafe fn GetErrorURL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetErrorURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPConvert, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPConvert {
    type Vtable = IWMPConvert_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPConvert {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd683162f_57d4_4108_8373_4a9676d1c2e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPConvert_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ConvertFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinputfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdestinationfolder: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstroutputfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetErrorURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPCore(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore {
    pub unsafe fn close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).URL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).openState)(::windows_core::Interface::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).playState)(::windows_core::Interface::as_raw(self), pwmpps).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows_core::Result<IWMPControls> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).controls)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows_core::Result<IWMPSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).settings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).currentMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).SetcurrentMedia)(::windows_core::Interface::as_raw(self), pmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows_core::Result<IWMPMediaCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).mediaCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows_core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).playlistCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).versionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).launchURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows_core::Result<IWMPNetwork> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).network)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).currentPlaylist)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).SetcurrentPlaylist)(::windows_core::Interface::as_raw(self), ppl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows_core::Result<IWMPCdromCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).cdromCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows_core::Result<IWMPClosedCaption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).closedCaption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).isOnline)(::windows_core::Interface::as_raw(self), pfonline).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows_core::Result<IWMPError> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPCore, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPCore {
    type Vtable = IWMPCore_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPCore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd84cca99_cce2_11d2_9ecc_0000f8085981);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCore_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub URL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub openState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpos: *mut WMPOpenState) -> ::windows_core::HRESULT,
    pub playState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpps: *mut WMPPlayState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub controls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    controls: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    settings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub currentMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmedia: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    currentMedia: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetcurrentMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetcurrentMedia: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub mediaCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmediacollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    mediaCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub playlistCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplaylistcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    playlistCollection: usize,
    pub versionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrversioninfo: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub launchURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub network: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqni: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    network: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub currentPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    currentPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetcurrentPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetcurrentPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub cdromCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcdromcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    cdromCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub closedCaption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclosedcaption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    closedCaption: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub isOnline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isOnline: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperror: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    error: usize,
    pub status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPCore2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore2 {
    pub unsafe fn close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.URL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.openState)(::windows_core::Interface::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.playState)(::windows_core::Interface::as_raw(self), pwmpps).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows_core::Result<IWMPControls> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.controls)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows_core::Result<IWMPSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.settings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.currentMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.SetcurrentMedia)(::windows_core::Interface::as_raw(self), pmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows_core::Result<IWMPMediaCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.mediaCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows_core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.playlistCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.versionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.launchURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows_core::Result<IWMPNetwork> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.network)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.currentPlaylist)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.SetcurrentPlaylist)(::windows_core::Interface::as_raw(self), ppl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows_core::Result<IWMPCdromCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.cdromCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows_core::Result<IWMPClosedCaption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.closedCaption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.isOnline)(::windows_core::Interface::as_raw(self), pfonline).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows_core::Result<IWMPError> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows_core::Result<IWMPDVD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).dvd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPCore2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPCore);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPCore2 {
    type Vtable = IWMPCore2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPCore2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc17e5b7_7561_4c18_bb90_17d485775659);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCore2_Vtbl {
    pub base__: IWMPCore_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub dvd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdvd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    dvd: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPCore3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore3 {
    pub unsafe fn close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.URL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.openState)(::windows_core::Interface::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.playState)(::windows_core::Interface::as_raw(self), pwmpps).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows_core::Result<IWMPControls> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.controls)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows_core::Result<IWMPSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.settings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.currentMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetcurrentMedia)(::windows_core::Interface::as_raw(self), pmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows_core::Result<IWMPMediaCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.mediaCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows_core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.playlistCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.versionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.launchURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows_core::Result<IWMPNetwork> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.network)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.currentPlaylist)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetcurrentPlaylist)(::windows_core::Interface::as_raw(self), ppl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows_core::Result<IWMPCdromCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.cdromCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows_core::Result<IWMPClosedCaption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.closedCaption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.isOnline)(::windows_core::Interface::as_raw(self), pfonline).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows_core::Result<IWMPError> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows_core::Result<IWMPDVD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.dvd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newPlaylist<P0, P1>(&self, bstrname: P0, bstrurl: P1) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).newPlaylist)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), bstrurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newMedia<P0>(&self, bstrurl: P0) -> ::windows_core::Result<IWMPMedia>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).newMedia)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPCore3, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPCore, IWMPCore2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPCore3 {
    type Vtable = IWMPCore3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPCore3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7587c667_628f_499f_88e7_6a6f4e888464);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCore3_Vtbl {
    pub base__: IWMPCore2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub newPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    newPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub newMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmedia: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    newMedia: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPDVD(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDVD {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_isAvailable<P0>(&self, bstritem: P0, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).get_isAvailable)(::windows_core::Interface::as_raw(self), bstritem.into_param().abi(), pisavailable).ok()
    }
    pub unsafe fn domain(&self, strdomain: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).domain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(strdomain)).ok()
    }
    pub unsafe fn topMenu(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).topMenu)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn titleMenu(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).titleMenu)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn back(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).back)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).resume)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPDVD, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPDVD {
    type Vtable = IWMPDVD_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPDVD {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8da61686_4668_4a5c_ae5d_803193293dbe);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDVD_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_isAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_isAvailable: usize,
    pub domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdomain: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub topMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub titleMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub back: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPDownloadCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadCollection {
    pub unsafe fn id(&self, plid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).id)(::windows_core::Interface::as_raw(self), plid).ok()
    }
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).count)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn item(&self, litem: i32) -> ::windows_core::Result<IWMPDownloadItem2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).item)(::windows_core::Interface::as_raw(self), litem, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn startDownload<P0, P1>(&self, bstrsourceurl: P0, bstrtype: P1) -> ::windows_core::Result<IWMPDownloadItem2>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).startDownload)(::windows_core::Interface::as_raw(self), bstrsourceurl.into_param().abi(), bstrtype.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn removeItem(&self, litem: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).removeItem)(::windows_core::Interface::as_raw(self), litem).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPDownloadCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPDownloadCollection {
    type Vtable = IWMPDownloadCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPDownloadCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a319c7f_85f9_436c_b88e_82fd88000e1c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDownloadCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_core::HRESULT,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, litem: i32, ppdownload: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub startDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsourceurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdownload: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    startDownload: usize,
    pub removeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, litem: i32) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPDownloadItem(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadItem {
    pub unsafe fn sourceURL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).sourceURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn size(&self, plsize: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).size)(::windows_core::Interface::as_raw(self), plsize).ok()
    }
    pub unsafe fn r#type(&self, pbstrtype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).r#type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrtype)).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).progress)(::windows_core::Interface::as_raw(self), plprogress).ok()
    }
    pub unsafe fn downloadState(&self, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).downloadState)(::windows_core::Interface::as_raw(self), pwmpsdls).ok()
    }
    pub unsafe fn pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPDownloadItem, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPDownloadItem {
    type Vtable = IWMPDownloadItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPDownloadItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9470e8e_3f6b_46a9_a0a9_452815c34297);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDownloadItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub sourceURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT,
    pub r#type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT,
    pub downloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows_core::HRESULT,
    pub pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPDownloadItem2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadItem2 {
    pub unsafe fn sourceURL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.sourceURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn size(&self, plsize: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.size)(::windows_core::Interface::as_raw(self), plsize).ok()
    }
    pub unsafe fn r#type(&self, pbstrtype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.r#type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrtype)).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.progress)(::windows_core::Interface::as_raw(self), plprogress).ok()
    }
    pub unsafe fn downloadState(&self, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.downloadState)(::windows_core::Interface::as_raw(self), pwmpsdls).ok()
    }
    pub unsafe fn pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, bstritemname: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPDownloadItem2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPDownloadItem);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPDownloadItem2 {
    type Vtable = IWMPDownloadItem2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPDownloadItem2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fbb3336_6da3_479d_b8ff_67d46e20a987);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDownloadItem2_Vtbl {
    pub base__: IWMPDownloadItem_Vtbl,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPDownloadManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadManager {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getDownloadCollection(&self, lcollectionid: i32) -> ::windows_core::Result<IWMPDownloadCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getDownloadCollection)(::windows_core::Interface::as_raw(self), lcollectionid, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createDownloadCollection(&self) -> ::windows_core::Result<IWMPDownloadCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).createDownloadCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPDownloadManager, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPDownloadManager {
    type Vtable = IWMPDownloadManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPDownloadManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe15e9ad1_8f20_4cc4_9ec7_1a328ca86a0d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDownloadManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub getDownloadCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcollectionid: i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getDownloadCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createDownloadCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createDownloadCollection: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPEffects(::windows_core::IUnknown);
impl IWMPEffects {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Render<P0>(&self, plevels: *mut TimedLevel, hdc: P0, prc: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).Render)(::windows_core::Interface::as_raw(self), plevels, hdc.into_param().abi(), prc).ok()
    }
    pub unsafe fn MediaInfo<P0>(&self, lchannelcount: i32, lsamplerate: i32, bstrtitle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).MediaInfo)(::windows_core::Interface::as_raw(self), lchannelcount, lsamplerate, bstrtitle.into_param().abi()).ok()
    }
    pub unsafe fn GetCapabilities(&self, pdwcapabilities: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), pdwcapabilities).ok()
    }
    pub unsafe fn GetTitle(&self, bstrtitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTitle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bstrtitle)).ok()
    }
    pub unsafe fn GetPresetTitle(&self, npreset: i32, bstrpresettitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPresetTitle)(::windows_core::Interface::as_raw(self), npreset, ::core::mem::transmute(bstrpresettitle)).ok()
    }
    pub unsafe fn GetPresetCount(&self, pnpresetcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPresetCount)(::windows_core::Interface::as_raw(self), pnpresetcount).ok()
    }
    pub unsafe fn SetCurrentPreset(&self, npreset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCurrentPreset)(::windows_core::Interface::as_raw(self), npreset).ok()
    }
    pub unsafe fn GetCurrentPreset(&self, pnpreset: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCurrentPreset)(::windows_core::Interface::as_raw(self), pnpreset).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayPropertyPage<P0>(&self, hwndowner: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).DisplayPropertyPage)(::windows_core::Interface::as_raw(self), hwndowner.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GoFullscreen<P0>(&self, ffullscreen: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).GoFullscreen)(::windows_core::Interface::as_raw(self), ffullscreen.into_param().abi()).ok()
    }
    pub unsafe fn RenderFullScreen(&self, plevels: *mut TimedLevel) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RenderFullScreen)(::windows_core::Interface::as_raw(self), plevels).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPEffects, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPEffects {
    type Vtable = IWMPEffects_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPEffects {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3984c13_c3cb_48e2_8be5_5168340b4f35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEffects_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub Render: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    Render: usize,
    pub MediaInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lchannelcount: i32, lsamplerate: i32, bstrtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows_core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetPresetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npreset: i32, bstrpresettitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetPresetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpresetcount: *mut i32) -> ::windows_core::HRESULT,
    pub SetCurrentPreset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npreset: i32) -> ::windows_core::HRESULT,
    pub GetCurrentPreset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpreset: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayPropertyPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayPropertyPage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GoFullscreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GoFullscreen: usize,
    pub RenderFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPEffects2(::windows_core::IUnknown);
impl IWMPEffects2 {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Render<P0>(&self, plevels: *mut TimedLevel, hdc: P0, prc: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).base__.Render)(::windows_core::Interface::as_raw(self), plevels, hdc.into_param().abi(), prc).ok()
    }
    pub unsafe fn MediaInfo<P0>(&self, lchannelcount: i32, lsamplerate: i32, bstrtitle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.MediaInfo)(::windows_core::Interface::as_raw(self), lchannelcount, lsamplerate, bstrtitle.into_param().abi()).ok()
    }
    pub unsafe fn GetCapabilities(&self, pdwcapabilities: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCapabilities)(::windows_core::Interface::as_raw(self), pdwcapabilities).ok()
    }
    pub unsafe fn GetTitle(&self, bstrtitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetTitle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bstrtitle)).ok()
    }
    pub unsafe fn GetPresetTitle(&self, npreset: i32, bstrpresettitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPresetTitle)(::windows_core::Interface::as_raw(self), npreset, ::core::mem::transmute(bstrpresettitle)).ok()
    }
    pub unsafe fn GetPresetCount(&self, pnpresetcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPresetCount)(::windows_core::Interface::as_raw(self), pnpresetcount).ok()
    }
    pub unsafe fn SetCurrentPreset(&self, npreset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCurrentPreset)(::windows_core::Interface::as_raw(self), npreset).ok()
    }
    pub unsafe fn GetCurrentPreset(&self, pnpreset: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCurrentPreset)(::windows_core::Interface::as_raw(self), pnpreset).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayPropertyPage<P0>(&self, hwndowner: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.DisplayPropertyPage)(::windows_core::Interface::as_raw(self), hwndowner.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GoFullscreen<P0>(&self, ffullscreen: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.GoFullscreen)(::windows_core::Interface::as_raw(self), ffullscreen.into_param().abi()).ok()
    }
    pub unsafe fn RenderFullScreen(&self, plevels: *mut TimedLevel) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RenderFullScreen)(::windows_core::Interface::as_raw(self), plevels).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCore<P0>(&self, pplayer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPCore>,
    {
        (::windows_core::Interface::vtable(self).SetCore)(::windows_core::Interface::as_raw(self), pplayer.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<P0>(&self, hwndparent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
    pub unsafe fn Destroy(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Destroy)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NotifyNewMedia<P0>(&self, pmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).NotifyNewMedia)(::windows_core::Interface::as_raw(self), pmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).OnWindowMessage)(::windows_core::Interface::as_raw(self), msg, wparam.into_param().abi(), lparam.into_param().abi(), plresultparam).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenderWindowed<P0>(&self, pdata: *mut TimedLevel, frequiredrender: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).RenderWindowed)(::windows_core::Interface::as_raw(self), pdata, frequiredrender.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPEffects2, ::windows_core::IUnknown, IWMPEffects);
unsafe impl ::windows_core::Interface for IWMPEffects2 {
    type Vtable = IWMPEffects2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPEffects2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x695386ec_aa3c_4618_a5e1_dd9a8b987632);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEffects2_Vtbl {
    pub base__: IWMPEffects_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplayer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NotifyNewMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NotifyNewMedia: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RenderWindowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenderWindowed: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPError(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPError {
    pub unsafe fn clearErrorQueue(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).clearErrorQueue)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn errorCount(&self, plnumerrors: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).errorCount)(::windows_core::Interface::as_raw(self), plnumerrors).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_item(&self, dwindex: i32) -> ::windows_core::Result<IWMPErrorItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_item)(::windows_core::Interface::as_raw(self), dwindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn webHelp(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).webHelp)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPError, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPError {
    type Vtable = IWMPError_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPError {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa12dcf7d_14ab_4c1b_a8cd_63909f06025b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPError_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub clearErrorQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub errorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plnumerrors: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: i32, pperroritem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_item: usize,
    pub webHelp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPErrorItem(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorItem {
    pub unsafe fn errorCode(&self, phr: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).errorCode)(::windows_core::Interface::as_raw(self), phr).ok()
    }
    pub unsafe fn errorDescription(&self, pbstrdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).errorDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdescription)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn errorContext(&self, pvarcontext: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).errorContext)(::windows_core::Interface::as_raw(self), pvarcontext).ok()
    }
    pub unsafe fn remedy(&self, plremedy: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).remedy)(::windows_core::Interface::as_raw(self), plremedy).ok()
    }
    pub unsafe fn customUrl(&self, pbstrcustomurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).customUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcustomurl)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPErrorItem, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPErrorItem {
    type Vtable = IWMPErrorItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPErrorItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3614c646_3b3b_4de7_a81e_930e3f2127b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPErrorItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub errorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phr: *mut i32) -> ::windows_core::HRESULT,
    pub errorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub errorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcontext: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    errorContext: usize,
    pub remedy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plremedy: *mut i32) -> ::windows_core::HRESULT,
    pub customUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcustomurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPErrorItem2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorItem2 {
    pub unsafe fn errorCode(&self, phr: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.errorCode)(::windows_core::Interface::as_raw(self), phr).ok()
    }
    pub unsafe fn errorDescription(&self, pbstrdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.errorDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdescription)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn errorContext(&self, pvarcontext: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.errorContext)(::windows_core::Interface::as_raw(self), pvarcontext).ok()
    }
    pub unsafe fn remedy(&self, plremedy: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.remedy)(::windows_core::Interface::as_raw(self), plremedy).ok()
    }
    pub unsafe fn customUrl(&self, pbstrcustomurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.customUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcustomurl)).ok()
    }
    pub unsafe fn condition(&self, plcondition: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).condition)(::windows_core::Interface::as_raw(self), plcondition).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPErrorItem2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPErrorItem);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPErrorItem2 {
    type Vtable = IWMPErrorItem2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPErrorItem2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf75ccec0_c67c_475c_931e_8719870bee7d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPErrorItem2_Vtbl {
    pub base__: IWMPErrorItem_Vtbl,
    pub condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcondition: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPEvents(::windows_core::IUnknown);
impl IWMPEvents {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows_core::Interface::vtable(self).OpenStateChange)(::windows_core::Interface::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows_core::Interface::vtable(self).PlayStateChange)(::windows_core::Interface::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows_core::Interface::vtable(self).AudioLanguageChange)(::windows_core::Interface::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows_core::Interface::vtable(self).StatusChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ScriptCommand<P0, P1>(&self, sctype: P0, param: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ScriptCommand)(::windows_core::Interface::as_raw(self), sctype.into_param().abi(), param.into_param().abi())
    }
    pub unsafe fn NewStream(&self) {
        (::windows_core::Interface::vtable(self).NewStream)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self), result)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Buffering<P0>(&self, start: P0)
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Buffering)(::windows_core::Interface::as_raw(self), start.into_param().abi())
    }
    pub unsafe fn Error(&self) {
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Warning<P0>(&self, warningtype: i32, param: i32, description: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Warning)(::windows_core::Interface::as_raw(self), warningtype, param, description.into_param().abi())
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows_core::Interface::vtable(self).EndOfStream)(::windows_core::Interface::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows_core::Interface::vtable(self).PositionChange)(::windows_core::Interface::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows_core::Interface::vtable(self).MarkerHit)(::windows_core::Interface::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows_core::Interface::vtable(self).DurationUnitChange)(::windows_core::Interface::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows_core::Interface::vtable(self).CdromMediaChange)(::windows_core::Interface::as_raw(self), cdromnum)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).PlaylistChange)(::windows_core::Interface::as_raw(self), playlist.into_param().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows_core::Interface::vtable(self).CurrentPlaylistChange)(::windows_core::Interface::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable<P0>(&self, bstritemname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).CurrentPlaylistItemAvailable)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<P0>(&self, item: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).MediaChange)(::windows_core::Interface::as_raw(self), item.into_param().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable<P0>(&self, bstritemname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).CurrentMediaItemAvailable)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<P0>(&self, pdispmedia: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).CurrentItemChange)(::windows_core::Interface::as_raw(self), pdispmedia.into_param().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows_core::Interface::vtable(self).MediaCollectionChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded<P0, P1>(&self, bstrattribname: P0, bstrattribval: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).MediaCollectionAttributeStringAdded)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstrattribval.into_param().abi())
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved<P0, P1>(&self, bstrattribname: P0, bstrattribval: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).MediaCollectionAttributeStringRemoved)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstrattribval.into_param().abi())
    }
    pub unsafe fn MediaCollectionAttributeStringChanged<P0, P1, P2>(&self, bstrattribname: P0, bstroldattribval: P1, bstrnewattribval: P2)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).MediaCollectionAttributeStringChanged)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstroldattribval.into_param().abi(), bstrnewattribval.into_param().abi())
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows_core::Interface::vtable(self).PlaylistCollectionChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded<P0>(&self, bstrplaylistname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PlaylistCollectionPlaylistAdded)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi())
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved<P0>(&self, bstrplaylistname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PlaylistCollectionPlaylistRemoved)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted<P0, P1>(&self, bstrplaylistname: P0, varfisdeleted: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).PlaylistCollectionPlaylistSetAsDeleted)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi(), varfisdeleted.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModeChange<P0, P1>(&self, modename: P0, newvalue: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).ModeChange)(::windows_core::Interface::as_raw(self), modename.into_param().abi(), newvalue.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<P0>(&self, pmediaobject: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).MediaError)(::windows_core::Interface::as_raw(self), pmediaobject.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<P0>(&self, pitem: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).OpenPlaylistSwitch)(::windows_core::Interface::as_raw(self), pitem.into_param().abi())
    }
    pub unsafe fn DomainChange<P0>(&self, strdomain: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DomainChange)(::windows_core::Interface::as_raw(self), strdomain.into_param().abi())
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows_core::Interface::vtable(self).SwitchedToPlayerApplication)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows_core::Interface::vtable(self).SwitchedToControl)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows_core::Interface::vtable(self).PlayerDockedStateChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows_core::Interface::vtable(self).PlayerReconnect)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).Click)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).DoubleClick)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows_core::Interface::vtable(self).KeyDown)(::windows_core::Interface::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows_core::Interface::vtable(self).KeyPress)(::windows_core::Interface::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows_core::Interface::vtable(self).KeyUp)(::windows_core::Interface::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).MouseDown)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).MouseMove)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).MouseUp)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
}
::windows_core::imp::interface_hierarchy!(IWMPEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPEvents {
    type Vtable = IWMPEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19a6627b_da9e_47c1_bb23_00b5e668236a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OpenStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: i32),
    pub PlayStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: i32),
    pub AudioLanguageChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: i32),
    pub StatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub ScriptCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sctype: ::std::mem::MaybeUninit<::windows_core::BSTR>, param: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    pub NewStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: i32),
    #[cfg(feature = "Win32_Foundation")]
    pub Buffering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: super::super::Foundation::VARIANT_BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    Buffering: usize,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Warning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, warningtype: i32, param: i32, description: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    pub EndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: i32),
    pub PositionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldposition: f64, newposition: f64),
    pub MarkerHit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, markernum: i32),
    pub DurationUnitChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newdurationunit: i32),
    pub CdromMediaChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cdromnum: i32),
    #[cfg(feature = "Win32_System_Com")]
    pub PlaylistChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, playlist: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType),
    #[cfg(not(feature = "Win32_System_Com"))]
    PlaylistChange: usize,
    pub CurrentPlaylistChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType),
    pub CurrentPlaylistItemAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    #[cfg(feature = "Win32_System_Com")]
    pub MediaChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    MediaChange: usize,
    pub CurrentMediaItemAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentItemChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentItemChange: usize,
    pub MediaCollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub MediaCollectionAttributeStringAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrattribval: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    pub MediaCollectionAttributeStringRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrattribval: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    pub MediaCollectionAttributeStringChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstroldattribval: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnewattribval: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    pub PlaylistCollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PlaylistCollectionPlaylistAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrplaylistname: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    pub PlaylistCollectionPlaylistRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrplaylistname: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    #[cfg(feature = "Win32_Foundation")]
    pub PlaylistCollectionPlaylistSetAsDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrplaylistname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varfisdeleted: super::super::Foundation::VARIANT_BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    PlaylistCollectionPlaylistSetAsDeleted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modename: ::std::mem::MaybeUninit<::windows_core::BSTR>, newvalue: super::super::Foundation::VARIANT_BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    ModeChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MediaError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediaobject: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    MediaError: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenPlaylistSwitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenPlaylistSwitch: usize,
    pub DomainChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>),
    pub SwitchedToPlayerApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub SwitchedToControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PlayerDockedStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PlayerReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Click: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
    pub DoubleClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
    pub KeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16),
    pub KeyPress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nkeyascii: i16),
    pub KeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16),
    pub MouseDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
    pub MouseMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
    pub MouseUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPEvents2(::windows_core::IUnknown);
impl IWMPEvents2 {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows_core::Interface::vtable(self).base__.OpenStateChange)(::windows_core::Interface::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows_core::Interface::vtable(self).base__.PlayStateChange)(::windows_core::Interface::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows_core::Interface::vtable(self).base__.AudioLanguageChange)(::windows_core::Interface::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows_core::Interface::vtable(self).base__.StatusChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ScriptCommand<P0, P1>(&self, sctype: P0, param: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ScriptCommand)(::windows_core::Interface::as_raw(self), sctype.into_param().abi(), param.into_param().abi())
    }
    pub unsafe fn NewStream(&self) {
        (::windows_core::Interface::vtable(self).base__.NewStream)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows_core::Interface::vtable(self).base__.Disconnect)(::windows_core::Interface::as_raw(self), result)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Buffering<P0>(&self, start: P0)
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Buffering)(::windows_core::Interface::as_raw(self), start.into_param().abi())
    }
    pub unsafe fn Error(&self) {
        (::windows_core::Interface::vtable(self).base__.Error)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Warning<P0>(&self, warningtype: i32, param: i32, description: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Warning)(::windows_core::Interface::as_raw(self), warningtype, param, description.into_param().abi())
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows_core::Interface::vtable(self).base__.EndOfStream)(::windows_core::Interface::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows_core::Interface::vtable(self).base__.PositionChange)(::windows_core::Interface::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows_core::Interface::vtable(self).base__.MarkerHit)(::windows_core::Interface::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows_core::Interface::vtable(self).base__.DurationUnitChange)(::windows_core::Interface::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows_core::Interface::vtable(self).base__.CdromMediaChange)(::windows_core::Interface::as_raw(self), cdromnum)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.PlaylistChange)(::windows_core::Interface::as_raw(self), playlist.into_param().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows_core::Interface::vtable(self).base__.CurrentPlaylistChange)(::windows_core::Interface::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable<P0>(&self, bstritemname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.CurrentPlaylistItemAvailable)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<P0>(&self, item: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.MediaChange)(::windows_core::Interface::as_raw(self), item.into_param().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable<P0>(&self, bstritemname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.CurrentMediaItemAvailable)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<P0>(&self, pdispmedia: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.CurrentItemChange)(::windows_core::Interface::as_raw(self), pdispmedia.into_param().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows_core::Interface::vtable(self).base__.MediaCollectionChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded<P0, P1>(&self, bstrattribname: P0, bstrattribval: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.MediaCollectionAttributeStringAdded)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstrattribval.into_param().abi())
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved<P0, P1>(&self, bstrattribname: P0, bstrattribval: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.MediaCollectionAttributeStringRemoved)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstrattribval.into_param().abi())
    }
    pub unsafe fn MediaCollectionAttributeStringChanged<P0, P1, P2>(&self, bstrattribname: P0, bstroldattribval: P1, bstrnewattribval: P2)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.MediaCollectionAttributeStringChanged)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstroldattribval.into_param().abi(), bstrnewattribval.into_param().abi())
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows_core::Interface::vtable(self).base__.PlaylistCollectionChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded<P0>(&self, bstrplaylistname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PlaylistCollectionPlaylistAdded)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi())
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved<P0>(&self, bstrplaylistname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PlaylistCollectionPlaylistRemoved)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted<P0, P1>(&self, bstrplaylistname: P0, varfisdeleted: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.PlaylistCollectionPlaylistSetAsDeleted)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi(), varfisdeleted.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModeChange<P0, P1>(&self, modename: P0, newvalue: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.ModeChange)(::windows_core::Interface::as_raw(self), modename.into_param().abi(), newvalue.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<P0>(&self, pmediaobject: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.MediaError)(::windows_core::Interface::as_raw(self), pmediaobject.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<P0>(&self, pitem: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.OpenPlaylistSwitch)(::windows_core::Interface::as_raw(self), pitem.into_param().abi())
    }
    pub unsafe fn DomainChange<P0>(&self, strdomain: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.DomainChange)(::windows_core::Interface::as_raw(self), strdomain.into_param().abi())
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows_core::Interface::vtable(self).base__.SwitchedToPlayerApplication)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows_core::Interface::vtable(self).base__.SwitchedToControl)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows_core::Interface::vtable(self).base__.PlayerDockedStateChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows_core::Interface::vtable(self).base__.PlayerReconnect)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.Click)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.DoubleClick)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows_core::Interface::vtable(self).base__.KeyDown)(::windows_core::Interface::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows_core::Interface::vtable(self).base__.KeyPress)(::windows_core::Interface::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows_core::Interface::vtable(self).base__.KeyUp)(::windows_core::Interface::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.MouseDown)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.MouseMove)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.MouseUp)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DeviceConnect<P0>(&self, pdevice: P0)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).DeviceConnect)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi())
    }
    pub unsafe fn DeviceDisconnect<P0>(&self, pdevice: P0)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).DeviceDisconnect)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi())
    }
    pub unsafe fn DeviceStatusChange<P0>(&self, pdevice: P0, newstatus: WMPDeviceStatus)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).DeviceStatusChange)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), newstatus)
    }
    pub unsafe fn DeviceSyncStateChange<P0>(&self, pdevice: P0, newstate: WMPSyncState)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).DeviceSyncStateChange)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), newstate)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSyncError<P0, P1>(&self, pdevice: P0, pmedia: P1)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).DeviceSyncError)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pmedia.into_param().abi())
    }
    pub unsafe fn CreatePartnershipComplete<P0>(&self, pdevice: P0, hrresult: ::windows_core::HRESULT)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).CreatePartnershipComplete)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hrresult)
    }
}
::windows_core::imp::interface_hierarchy!(IWMPEvents2, ::windows_core::IUnknown, IWMPEvents);
unsafe impl ::windows_core::Interface for IWMPEvents2 {
    type Vtable = IWMPEvents2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPEvents2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e7601fa_47ea_4107_9ea9_9004ed9684ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents2_Vtbl {
    pub base__: IWMPEvents_Vtbl,
    pub DeviceConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void),
    pub DeviceDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void),
    pub DeviceStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, newstatus: WMPDeviceStatus),
    pub DeviceSyncStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, newstate: WMPSyncState),
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceSyncError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceSyncError: usize,
    pub CreatePartnershipComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPEvents3(::windows_core::IUnknown);
impl IWMPEvents3 {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.OpenStateChange)(::windows_core::Interface::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.PlayStateChange)(::windows_core::Interface::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.AudioLanguageChange)(::windows_core::Interface::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.StatusChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ScriptCommand<P0, P1>(&self, sctype: P0, param: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ScriptCommand)(::windows_core::Interface::as_raw(self), sctype.into_param().abi(), param.into_param().abi())
    }
    pub unsafe fn NewStream(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.NewStream)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.Disconnect)(::windows_core::Interface::as_raw(self), result)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Buffering<P0>(&self, start: P0)
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Buffering)(::windows_core::Interface::as_raw(self), start.into_param().abi())
    }
    pub unsafe fn Error(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.Error)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Warning<P0>(&self, warningtype: i32, param: i32, description: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Warning)(::windows_core::Interface::as_raw(self), warningtype, param, description.into_param().abi())
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.EndOfStream)(::windows_core::Interface::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows_core::Interface::vtable(self).base__.base__.PositionChange)(::windows_core::Interface::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.MarkerHit)(::windows_core::Interface::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.DurationUnitChange)(::windows_core::Interface::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.CdromMediaChange)(::windows_core::Interface::as_raw(self), cdromnum)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.PlaylistChange)(::windows_core::Interface::as_raw(self), playlist.into_param().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows_core::Interface::vtable(self).base__.base__.CurrentPlaylistChange)(::windows_core::Interface::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable<P0>(&self, bstritemname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CurrentPlaylistItemAvailable)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<P0>(&self, item: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.MediaChange)(::windows_core::Interface::as_raw(self), item.into_param().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable<P0>(&self, bstritemname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CurrentMediaItemAvailable)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<P0>(&self, pdispmedia: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CurrentItemChange)(::windows_core::Interface::as_raw(self), pdispmedia.into_param().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.MediaCollectionChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded<P0, P1>(&self, bstrattribname: P0, bstrattribval: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.MediaCollectionAttributeStringAdded)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstrattribval.into_param().abi())
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved<P0, P1>(&self, bstrattribname: P0, bstrattribval: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.MediaCollectionAttributeStringRemoved)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstrattribval.into_param().abi())
    }
    pub unsafe fn MediaCollectionAttributeStringChanged<P0, P1, P2>(&self, bstrattribname: P0, bstroldattribval: P1, bstrnewattribval: P2)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.MediaCollectionAttributeStringChanged)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstroldattribval.into_param().abi(), bstrnewattribval.into_param().abi())
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.PlaylistCollectionChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded<P0>(&self, bstrplaylistname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.PlaylistCollectionPlaylistAdded)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi())
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved<P0>(&self, bstrplaylistname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.PlaylistCollectionPlaylistRemoved)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted<P0, P1>(&self, bstrplaylistname: P0, varfisdeleted: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.PlaylistCollectionPlaylistSetAsDeleted)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi(), varfisdeleted.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModeChange<P0, P1>(&self, modename: P0, newvalue: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ModeChange)(::windows_core::Interface::as_raw(self), modename.into_param().abi(), newvalue.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<P0>(&self, pmediaobject: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.MediaError)(::windows_core::Interface::as_raw(self), pmediaobject.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<P0>(&self, pitem: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.OpenPlaylistSwitch)(::windows_core::Interface::as_raw(self), pitem.into_param().abi())
    }
    pub unsafe fn DomainChange<P0>(&self, strdomain: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.DomainChange)(::windows_core::Interface::as_raw(self), strdomain.into_param().abi())
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.SwitchedToPlayerApplication)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.SwitchedToControl)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.PlayerDockedStateChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.PlayerReconnect)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.Click)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.DoubleClick)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows_core::Interface::vtable(self).base__.base__.KeyDown)(::windows_core::Interface::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows_core::Interface::vtable(self).base__.base__.KeyPress)(::windows_core::Interface::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows_core::Interface::vtable(self).base__.base__.KeyUp)(::windows_core::Interface::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.MouseDown)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.MouseMove)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.MouseUp)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DeviceConnect<P0>(&self, pdevice: P0)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.DeviceConnect)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi())
    }
    pub unsafe fn DeviceDisconnect<P0>(&self, pdevice: P0)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.DeviceDisconnect)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi())
    }
    pub unsafe fn DeviceStatusChange<P0>(&self, pdevice: P0, newstatus: WMPDeviceStatus)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.DeviceStatusChange)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), newstatus)
    }
    pub unsafe fn DeviceSyncStateChange<P0>(&self, pdevice: P0, newstate: WMPSyncState)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.DeviceSyncStateChange)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), newstate)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSyncError<P0, P1>(&self, pdevice: P0, pmedia: P1)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.DeviceSyncError)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pmedia.into_param().abi())
    }
    pub unsafe fn CreatePartnershipComplete<P0>(&self, pdevice: P0, hrresult: ::windows_core::HRESULT)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.CreatePartnershipComplete)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hrresult)
    }
    pub unsafe fn CdromRipStateChange<P0>(&self, pcdromrip: P0, wmprs: WMPRipState)
    where
        P0: ::windows_core::IntoParam<IWMPCdromRip>,
    {
        (::windows_core::Interface::vtable(self).CdromRipStateChange)(::windows_core::Interface::as_raw(self), pcdromrip.into_param().abi(), wmprs)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromRipMediaError<P0, P1>(&self, pcdromrip: P0, pmedia: P1)
    where
        P0: ::windows_core::IntoParam<IWMPCdromRip>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).CdromRipMediaError)(::windows_core::Interface::as_raw(self), pcdromrip.into_param().abi(), pmedia.into_param().abi())
    }
    pub unsafe fn CdromBurnStateChange<P0>(&self, pcdromburn: P0, wmpbs: WMPBurnState)
    where
        P0: ::windows_core::IntoParam<IWMPCdromBurn>,
    {
        (::windows_core::Interface::vtable(self).CdromBurnStateChange)(::windows_core::Interface::as_raw(self), pcdromburn.into_param().abi(), wmpbs)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromBurnMediaError<P0, P1>(&self, pcdromburn: P0, pmedia: P1)
    where
        P0: ::windows_core::IntoParam<IWMPCdromBurn>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).CdromBurnMediaError)(::windows_core::Interface::as_raw(self), pcdromburn.into_param().abi(), pmedia.into_param().abi())
    }
    pub unsafe fn CdromBurnError<P0>(&self, pcdromburn: P0, hrerror: ::windows_core::HRESULT)
    where
        P0: ::windows_core::IntoParam<IWMPCdromBurn>,
    {
        (::windows_core::Interface::vtable(self).CdromBurnError)(::windows_core::Interface::as_raw(self), pcdromburn.into_param().abi(), hrerror)
    }
    pub unsafe fn LibraryConnect<P0>(&self, plibrary: P0)
    where
        P0: ::windows_core::IntoParam<IWMPLibrary>,
    {
        (::windows_core::Interface::vtable(self).LibraryConnect)(::windows_core::Interface::as_raw(self), plibrary.into_param().abi())
    }
    pub unsafe fn LibraryDisconnect<P0>(&self, plibrary: P0)
    where
        P0: ::windows_core::IntoParam<IWMPLibrary>,
    {
        (::windows_core::Interface::vtable(self).LibraryDisconnect)(::windows_core::Interface::as_raw(self), plibrary.into_param().abi())
    }
    pub unsafe fn FolderScanStateChange(&self, wmpfss: WMPFolderScanState) {
        (::windows_core::Interface::vtable(self).FolderScanStateChange)(::windows_core::Interface::as_raw(self), wmpfss)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StringCollectionChange<P0>(&self, pdispstringcollection: P0, change: WMPStringCollectionChangeEventType, lcollectionindex: i32)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).StringCollectionChange)(::windows_core::Interface::as_raw(self), pdispstringcollection.into_param().abi(), change, lcollectionindex)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaAdded<P0>(&self, pdispmedia: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).MediaCollectionMediaAdded)(::windows_core::Interface::as_raw(self), pdispmedia.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaRemoved<P0>(&self, pdispmedia: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).MediaCollectionMediaRemoved)(::windows_core::Interface::as_raw(self), pdispmedia.into_param().abi())
    }
}
::windows_core::imp::interface_hierarchy!(IWMPEvents3, ::windows_core::IUnknown, IWMPEvents, IWMPEvents2);
unsafe impl ::windows_core::Interface for IWMPEvents3 {
    type Vtable = IWMPEvents3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPEvents3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f504270_a66b_4223_8e96_26a06c63d69f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents3_Vtbl {
    pub base__: IWMPEvents2_Vtbl,
    pub CdromRipStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromrip: *mut ::core::ffi::c_void, wmprs: WMPRipState),
    #[cfg(feature = "Win32_System_Com")]
    pub CdromRipMediaError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromrip: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    CdromRipMediaError: usize,
    pub CdromBurnStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, wmpbs: WMPBurnState),
    #[cfg(feature = "Win32_System_Com")]
    pub CdromBurnMediaError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    CdromBurnMediaError: usize,
    pub CdromBurnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT),
    pub LibraryConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plibrary: *mut ::core::ffi::c_void),
    pub LibraryDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plibrary: *mut ::core::ffi::c_void),
    pub FolderScanStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmpfss: WMPFolderScanState),
    #[cfg(feature = "Win32_System_Com")]
    pub StringCollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdispstringcollection: *mut ::core::ffi::c_void, change: WMPStringCollectionChangeEventType, lcollectionindex: i32),
    #[cfg(not(feature = "Win32_System_Com"))]
    StringCollectionChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MediaCollectionMediaAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    MediaCollectionMediaAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MediaCollectionMediaRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    MediaCollectionMediaRemoved: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPEvents4(::windows_core::IUnknown);
impl IWMPEvents4 {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.OpenStateChange)(::windows_core::Interface::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.PlayStateChange)(::windows_core::Interface::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.AudioLanguageChange)(::windows_core::Interface::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.StatusChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ScriptCommand<P0, P1>(&self, sctype: P0, param: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ScriptCommand)(::windows_core::Interface::as_raw(self), sctype.into_param().abi(), param.into_param().abi())
    }
    pub unsafe fn NewStream(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.NewStream)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Disconnect)(::windows_core::Interface::as_raw(self), result)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Buffering<P0>(&self, start: P0)
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Buffering)(::windows_core::Interface::as_raw(self), start.into_param().abi())
    }
    pub unsafe fn Error(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Error)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Warning<P0>(&self, warningtype: i32, param: i32, description: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Warning)(::windows_core::Interface::as_raw(self), warningtype, param, description.into_param().abi())
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.EndOfStream)(::windows_core::Interface::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.PositionChange)(::windows_core::Interface::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MarkerHit)(::windows_core::Interface::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.DurationUnitChange)(::windows_core::Interface::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CdromMediaChange)(::windows_core::Interface::as_raw(self), cdromnum)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.PlaylistChange)(::windows_core::Interface::as_raw(self), playlist.into_param().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CurrentPlaylistChange)(::windows_core::Interface::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable<P0>(&self, bstritemname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CurrentPlaylistItemAvailable)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<P0>(&self, item: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MediaChange)(::windows_core::Interface::as_raw(self), item.into_param().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable<P0>(&self, bstritemname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CurrentMediaItemAvailable)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<P0>(&self, pdispmedia: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CurrentItemChange)(::windows_core::Interface::as_raw(self), pdispmedia.into_param().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MediaCollectionChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded<P0, P1>(&self, bstrattribname: P0, bstrattribval: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MediaCollectionAttributeStringAdded)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstrattribval.into_param().abi())
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved<P0, P1>(&self, bstrattribname: P0, bstrattribval: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MediaCollectionAttributeStringRemoved)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstrattribval.into_param().abi())
    }
    pub unsafe fn MediaCollectionAttributeStringChanged<P0, P1, P2>(&self, bstrattribname: P0, bstroldattribval: P1, bstrnewattribval: P2)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MediaCollectionAttributeStringChanged)(::windows_core::Interface::as_raw(self), bstrattribname.into_param().abi(), bstroldattribval.into_param().abi(), bstrnewattribval.into_param().abi())
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.PlaylistCollectionChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded<P0>(&self, bstrplaylistname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.PlaylistCollectionPlaylistAdded)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi())
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved<P0>(&self, bstrplaylistname: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.PlaylistCollectionPlaylistRemoved)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted<P0, P1>(&self, bstrplaylistname: P0, varfisdeleted: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.PlaylistCollectionPlaylistSetAsDeleted)(::windows_core::Interface::as_raw(self), bstrplaylistname.into_param().abi(), varfisdeleted.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModeChange<P0, P1>(&self, modename: P0, newvalue: P1)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ModeChange)(::windows_core::Interface::as_raw(self), modename.into_param().abi(), newvalue.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<P0>(&self, pmediaobject: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MediaError)(::windows_core::Interface::as_raw(self), pmediaobject.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<P0>(&self, pitem: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.OpenPlaylistSwitch)(::windows_core::Interface::as_raw(self), pitem.into_param().abi())
    }
    pub unsafe fn DomainChange<P0>(&self, strdomain: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.DomainChange)(::windows_core::Interface::as_raw(self), strdomain.into_param().abi())
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SwitchedToPlayerApplication)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SwitchedToControl)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.PlayerDockedStateChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.PlayerReconnect)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Click)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.DoubleClick)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.KeyDown)(::windows_core::Interface::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.KeyPress)(::windows_core::Interface::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.KeyUp)(::windows_core::Interface::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MouseDown)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MouseMove)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MouseUp)(::windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DeviceConnect<P0>(&self, pdevice: P0)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.DeviceConnect)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi())
    }
    pub unsafe fn DeviceDisconnect<P0>(&self, pdevice: P0)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.DeviceDisconnect)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi())
    }
    pub unsafe fn DeviceStatusChange<P0>(&self, pdevice: P0, newstatus: WMPDeviceStatus)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.DeviceStatusChange)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), newstatus)
    }
    pub unsafe fn DeviceSyncStateChange<P0>(&self, pdevice: P0, newstate: WMPSyncState)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.DeviceSyncStateChange)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), newstate)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSyncError<P0, P1>(&self, pdevice: P0, pmedia: P1)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.DeviceSyncError)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pmedia.into_param().abi())
    }
    pub unsafe fn CreatePartnershipComplete<P0>(&self, pdevice: P0, hrresult: ::windows_core::HRESULT)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CreatePartnershipComplete)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hrresult)
    }
    pub unsafe fn CdromRipStateChange<P0>(&self, pcdromrip: P0, wmprs: WMPRipState)
    where
        P0: ::windows_core::IntoParam<IWMPCdromRip>,
    {
        (::windows_core::Interface::vtable(self).base__.CdromRipStateChange)(::windows_core::Interface::as_raw(self), pcdromrip.into_param().abi(), wmprs)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromRipMediaError<P0, P1>(&self, pcdromrip: P0, pmedia: P1)
    where
        P0: ::windows_core::IntoParam<IWMPCdromRip>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.CdromRipMediaError)(::windows_core::Interface::as_raw(self), pcdromrip.into_param().abi(), pmedia.into_param().abi())
    }
    pub unsafe fn CdromBurnStateChange<P0>(&self, pcdromburn: P0, wmpbs: WMPBurnState)
    where
        P0: ::windows_core::IntoParam<IWMPCdromBurn>,
    {
        (::windows_core::Interface::vtable(self).base__.CdromBurnStateChange)(::windows_core::Interface::as_raw(self), pcdromburn.into_param().abi(), wmpbs)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromBurnMediaError<P0, P1>(&self, pcdromburn: P0, pmedia: P1)
    where
        P0: ::windows_core::IntoParam<IWMPCdromBurn>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.CdromBurnMediaError)(::windows_core::Interface::as_raw(self), pcdromburn.into_param().abi(), pmedia.into_param().abi())
    }
    pub unsafe fn CdromBurnError<P0>(&self, pcdromburn: P0, hrerror: ::windows_core::HRESULT)
    where
        P0: ::windows_core::IntoParam<IWMPCdromBurn>,
    {
        (::windows_core::Interface::vtable(self).base__.CdromBurnError)(::windows_core::Interface::as_raw(self), pcdromburn.into_param().abi(), hrerror)
    }
    pub unsafe fn LibraryConnect<P0>(&self, plibrary: P0)
    where
        P0: ::windows_core::IntoParam<IWMPLibrary>,
    {
        (::windows_core::Interface::vtable(self).base__.LibraryConnect)(::windows_core::Interface::as_raw(self), plibrary.into_param().abi())
    }
    pub unsafe fn LibraryDisconnect<P0>(&self, plibrary: P0)
    where
        P0: ::windows_core::IntoParam<IWMPLibrary>,
    {
        (::windows_core::Interface::vtable(self).base__.LibraryDisconnect)(::windows_core::Interface::as_raw(self), plibrary.into_param().abi())
    }
    pub unsafe fn FolderScanStateChange(&self, wmpfss: WMPFolderScanState) {
        (::windows_core::Interface::vtable(self).base__.FolderScanStateChange)(::windows_core::Interface::as_raw(self), wmpfss)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StringCollectionChange<P0>(&self, pdispstringcollection: P0, change: WMPStringCollectionChangeEventType, lcollectionindex: i32)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.StringCollectionChange)(::windows_core::Interface::as_raw(self), pdispstringcollection.into_param().abi(), change, lcollectionindex)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaAdded<P0>(&self, pdispmedia: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.MediaCollectionMediaAdded)(::windows_core::Interface::as_raw(self), pdispmedia.into_param().abi())
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaRemoved<P0>(&self, pdispmedia: P0)
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.MediaCollectionMediaRemoved)(::windows_core::Interface::as_raw(self), pdispmedia.into_param().abi())
    }
    pub unsafe fn DeviceEstimation<P0>(&self, pdevice: P0, hrresult: ::windows_core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64)
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).DeviceEstimation)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hrresult, qwestimatedusedspace, qwestimatedspace)
    }
}
::windows_core::imp::interface_hierarchy!(IWMPEvents4, ::windows_core::IUnknown, IWMPEvents, IWMPEvents2, IWMPEvents3);
unsafe impl ::windows_core::Interface for IWMPEvents4 {
    type Vtable = IWMPEvents4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPEvents4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26dabcfa_306b_404d_9a6f_630a8405048d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents4_Vtbl {
    pub base__: IWMPEvents3_Vtbl,
    pub DeviceEstimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPFolderMonitorServices(::windows_core::IUnknown);
impl IWMPFolderMonitorServices {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).count)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn item(&self, lindex: i32, pbstrfolder: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).item)(::windows_core::Interface::as_raw(self), lindex, ::core::mem::transmute(pbstrfolder)).ok()
    }
    pub unsafe fn add<P0>(&self, bstrfolder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).add)(::windows_core::Interface::as_raw(self), bstrfolder.into_param().abi()).ok()
    }
    pub unsafe fn remove(&self, lindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).remove)(::windows_core::Interface::as_raw(self), lindex).ok()
    }
    pub unsafe fn scanState(&self, pwmpfss: *mut WMPFolderScanState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).scanState)(::windows_core::Interface::as_raw(self), pwmpfss).ok()
    }
    pub unsafe fn currentFolder(&self, pbstrfolder: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).currentFolder)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrfolder)).ok()
    }
    pub unsafe fn scannedFilesCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).scannedFilesCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn addedFilesCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).addedFilesCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn updateProgress(&self, plprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).updateProgress)(::windows_core::Interface::as_raw(self), plprogress).ok()
    }
    pub unsafe fn startScan(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).startScan)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn stopScan(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).stopScan)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPFolderMonitorServices, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPFolderMonitorServices {
    type Vtable = IWMPFolderMonitorServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPFolderMonitorServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x788c8743_e57f_439d_a468_5bc77f2e59c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPFolderMonitorServices_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstrfolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfolder: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT,
    pub scanState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> ::windows_core::HRESULT,
    pub currentFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub scannedFilesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub addedFilesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub updateProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT,
    pub startScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub stopScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPGraphCreation(::windows_core::IUnknown);
impl IWMPGraphCreation {
    pub unsafe fn GraphCreationPreRender<P0, P1>(&self, pfiltergraph: P0, preserved: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).GraphCreationPreRender)(::windows_core::Interface::as_raw(self), pfiltergraph.into_param().abi(), preserved.into_param().abi()).ok()
    }
    pub unsafe fn GraphCreationPostRender<P0>(&self, pfiltergraph: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).GraphCreationPostRender)(::windows_core::Interface::as_raw(self), pfiltergraph.into_param().abi()).ok()
    }
    pub unsafe fn GetGraphCreationFlags(&self, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGraphCreationFlags)(::windows_core::Interface::as_raw(self), pdwflags).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPGraphCreation, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPGraphCreation {
    type Vtable = IWMPGraphCreation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPGraphCreation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfb377e5_c594_4369_a970_de896d5ece74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPGraphCreation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GraphCreationPreRender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GraphCreationPostRender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetGraphCreationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPLibrary(::windows_core::IUnknown);
impl IWMPLibrary {
    pub unsafe fn name(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn r#type(&self, pwmplt: *mut WMPLibraryType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).r#type)(::windows_core::Interface::as_raw(self), pwmplt).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows_core::Result<IWMPMediaCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).mediaCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isIdentical<P0>(&self, piwmplibrary: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPLibrary>,
    {
        (::windows_core::Interface::vtable(self).isIdentical)(::windows_core::Interface::as_raw(self), piwmplibrary.into_param().abi(), pvbool).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPLibrary, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPLibrary {
    type Vtable = IWMPLibrary_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPLibrary {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3df47861_7df1_4c1f_a81b_4c26f0f7a7c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibrary_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub r#type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub mediaCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwmpmediacollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    mediaCollection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmplibrary: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isIdentical: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPLibrary2(::windows_core::IUnknown);
impl IWMPLibrary2 {
    pub unsafe fn name(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn r#type(&self, pwmplt: *mut WMPLibraryType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.r#type)(::windows_core::Interface::as_raw(self), pwmplt).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows_core::Result<IWMPMediaCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.mediaCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isIdentical<P0>(&self, piwmplibrary: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPLibrary>,
    {
        (::windows_core::Interface::vtable(self).base__.isIdentical)(::windows_core::Interface::as_raw(self), piwmplibrary.into_param().abi(), pvbool).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, bstritemname: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPLibrary2, ::windows_core::IUnknown, IWMPLibrary);
unsafe impl ::windows_core::Interface for IWMPLibrary2 {
    type Vtable = IWMPLibrary2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPLibrary2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd578a4e_79b1_426c_bf8f_3add9072500b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibrary2_Vtbl {
    pub base__: IWMPLibrary_Vtbl,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPLibraryServices(::windows_core::IUnknown);
impl IWMPLibraryServices {
    pub unsafe fn getCountByType(&self, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getCountByType)(::windows_core::Interface::as_raw(self), wmplt, plcount).ok()
    }
    pub unsafe fn getLibraryByType(&self, wmplt: WMPLibraryType, lindex: i32) -> ::windows_core::Result<IWMPLibrary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getLibraryByType)(::windows_core::Interface::as_raw(self), wmplt, lindex, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMPLibraryServices, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPLibraryServices {
    type Vtable = IWMPLibraryServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPLibraryServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39c2f8d5_1cf2_4d5e_ae09_d73492cf9eaa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibraryServices_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub getCountByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub getLibraryByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPLibrarySharingServices(::windows_core::IUnknown);
impl IWMPLibrarySharingServices {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isLibraryShared(&self, pvbshared: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).isLibraryShared)(::windows_core::Interface::as_raw(self), pvbshared).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isLibrarySharingEnabled(&self, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).isLibrarySharingEnabled)(::windows_core::Interface::as_raw(self), pvbenabled).ok()
    }
    pub unsafe fn showLibrarySharing(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).showLibrarySharing)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPLibrarySharingServices, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPLibrarySharingServices {
    type Vtable = IWMPLibrarySharingServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPLibrarySharingServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82cba86b_9f04_474b_a365_d6dd1466e541);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibrarySharingServices_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub isLibraryShared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbshared: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isLibraryShared: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub isLibrarySharingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isLibrarySharingEnabled: usize,
    pub showLibrarySharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPMedia(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_isIdentical<P0>(&self, piwmpmedia: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).get_isIdentical)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi(), pvbool).ok()
    }
    pub unsafe fn sourceURL(&self, pbstrsourceurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).sourceURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsourceurl)).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Setname)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn imageSourceWidth(&self, pwidth: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).imageSourceWidth)(::windows_core::Interface::as_raw(self), pwidth).ok()
    }
    pub unsafe fn imageSourceHeight(&self, pheight: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).imageSourceHeight)(::windows_core::Interface::as_raw(self), pheight).ok()
    }
    pub unsafe fn markerCount(&self, pmarkercount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).markerCount)(::windows_core::Interface::as_raw(self), pmarkercount).ok()
    }
    pub unsafe fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getMarkerTime)(::windows_core::Interface::as_raw(self), markernum, pmarkertime).ok()
    }
    pub unsafe fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getMarkerName)(::windows_core::Interface::as_raw(self), markernum, ::core::mem::transmute(pbstrmarkername)).ok()
    }
    pub unsafe fn duration(&self, pduration: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).duration)(::windows_core::Interface::as_raw(self), pduration).ok()
    }
    pub unsafe fn durationString(&self, pbstrduration: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).durationString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrduration)).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).attributeCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn getAttributeName(&self, lindex: i32, pbstritemname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getAttributeName)(::windows_core::Interface::as_raw(self), lindex, ::core::mem::transmute(pbstritemname)).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, bstritemname: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo<P0, P1>(&self, bstritemname: P0, bstrval: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).getItemInfoByAtom)(::windows_core::Interface::as_raw(self), latom, ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isMemberOf<P0>(&self, pplaylist: P0, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).isMemberOf)(::windows_core::Interface::as_raw(self), pplaylist.into_param().abi(), pvarfismemberof).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isReadOnlyItem<P0>(&self, bstritemname: P0, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).isReadOnlyItem)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), pvarfisreadonly).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPMedia, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPMedia {
    type Vtable = IWMPMedia_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPMedia {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94d55e95_3fac_11d3_b155_00c04f79faa6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMedia_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_isIdentical: usize,
    pub sourceURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsourceurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Setname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub imageSourceWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows_core::HRESULT,
    pub imageSourceHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows_core::HRESULT,
    pub markerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmarkercount: *mut i32) -> ::windows_core::HRESULT,
    pub getMarkerTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> ::windows_core::HRESULT,
    pub getMarkerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, markernum: i32, pbstrmarkername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduration: *mut f64) -> ::windows_core::HRESULT,
    pub durationString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrduration: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub attributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub getAttributeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstritemname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub setItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub getItemInfoByAtom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latom: i32, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub isMemberOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplaylist: *mut ::core::ffi::c_void, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    isMemberOf: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub isReadOnlyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isReadOnlyItem: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPMedia2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia2 {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_isIdentical<P0>(&self, piwmpmedia: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.get_isIdentical)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi(), pvbool).ok()
    }
    pub unsafe fn sourceURL(&self, pbstrsourceurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.sourceURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsourceurl)).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Setname)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn imageSourceWidth(&self, pwidth: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.imageSourceWidth)(::windows_core::Interface::as_raw(self), pwidth).ok()
    }
    pub unsafe fn imageSourceHeight(&self, pheight: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.imageSourceHeight)(::windows_core::Interface::as_raw(self), pheight).ok()
    }
    pub unsafe fn markerCount(&self, pmarkercount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.markerCount)(::windows_core::Interface::as_raw(self), pmarkercount).ok()
    }
    pub unsafe fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.getMarkerTime)(::windows_core::Interface::as_raw(self), markernum, pmarkertime).ok()
    }
    pub unsafe fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.getMarkerName)(::windows_core::Interface::as_raw(self), markernum, ::core::mem::transmute(pbstrmarkername)).ok()
    }
    pub unsafe fn duration(&self, pduration: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.duration)(::windows_core::Interface::as_raw(self), pduration).ok()
    }
    pub unsafe fn durationString(&self, pbstrduration: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.durationString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrduration)).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.attributeCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn getAttributeName(&self, lindex: i32, pbstritemname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.getAttributeName)(::windows_core::Interface::as_raw(self), lindex, ::core::mem::transmute(pbstritemname)).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, bstritemname: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.getItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo<P0, P1>(&self, bstritemname: P0, bstrval: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.setItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.getItemInfoByAtom)(::windows_core::Interface::as_raw(self), latom, ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isMemberOf<P0>(&self, pplaylist: P0, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.isMemberOf)(::windows_core::Interface::as_raw(self), pplaylist.into_param().abi(), pvarfismemberof).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isReadOnlyItem<P0>(&self, bstritemname: P0, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.isReadOnlyItem)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), pvarfisreadonly).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows_core::Result<IWMPErrorItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPMedia2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPMedia);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPMedia2 {
    type Vtable = IWMPMedia2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPMedia2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab7c88bb_143e_4ea4_acc3_e4350b2106c3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMedia2_Vtbl {
    pub base__: IWMPMedia_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwmperroritem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    error: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPMedia3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia3 {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_isIdentical<P0>(&self, piwmpmedia: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.get_isIdentical)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi(), pvbool).ok()
    }
    pub unsafe fn sourceURL(&self, pbstrsourceurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.sourceURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsourceurl)).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Setname)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn imageSourceWidth(&self, pwidth: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.imageSourceWidth)(::windows_core::Interface::as_raw(self), pwidth).ok()
    }
    pub unsafe fn imageSourceHeight(&self, pheight: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.imageSourceHeight)(::windows_core::Interface::as_raw(self), pheight).ok()
    }
    pub unsafe fn markerCount(&self, pmarkercount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.markerCount)(::windows_core::Interface::as_raw(self), pmarkercount).ok()
    }
    pub unsafe fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.getMarkerTime)(::windows_core::Interface::as_raw(self), markernum, pmarkertime).ok()
    }
    pub unsafe fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.getMarkerName)(::windows_core::Interface::as_raw(self), markernum, ::core::mem::transmute(pbstrmarkername)).ok()
    }
    pub unsafe fn duration(&self, pduration: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.duration)(::windows_core::Interface::as_raw(self), pduration).ok()
    }
    pub unsafe fn durationString(&self, pbstrduration: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.durationString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrduration)).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.attributeCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn getAttributeName(&self, lindex: i32, pbstritemname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.getAttributeName)(::windows_core::Interface::as_raw(self), lindex, ::core::mem::transmute(pbstritemname)).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, bstritemname: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.getItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo<P0, P1>(&self, bstritemname: P0, bstrval: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.setItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.getItemInfoByAtom)(::windows_core::Interface::as_raw(self), latom, ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isMemberOf<P0>(&self, pplaylist: P0, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.isMemberOf)(::windows_core::Interface::as_raw(self), pplaylist.into_param().abi(), pvarfismemberof).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isReadOnlyItem<P0>(&self, bstritemname: P0, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.isReadOnlyItem)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), pvarfisreadonly).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows_core::Result<IWMPErrorItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn getAttributeCountByType<P0, P1>(&self, bstrtype: P0, bstrlanguage: P1, plcount: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getAttributeCountByType)(::windows_core::Interface::as_raw(self), bstrtype.into_param().abi(), bstrlanguage.into_param().abi(), plcount).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn getItemInfoByType<P0, P1>(&self, bstrtype: P0, bstrlanguage: P1, lindex: i32, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getItemInfoByType)(::windows_core::Interface::as_raw(self), bstrtype.into_param().abi(), bstrlanguage.into_param().abi(), lindex, pvarvalue).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPMedia3, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPMedia, IWMPMedia2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPMedia3 {
    type Vtable = IWMPMedia3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPMedia3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf118efc7_f03a_4fb4_99c9_1c02a5c1065b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMedia3_Vtbl {
    pub base__: IWMPMedia2_Vtbl,
    pub getAttributeCountByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub getItemInfoByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, lindex: i32, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    getItemInfoByType: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPMediaCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaCollection {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn add<P0>(&self, bstrurl: P0) -> ::windows_core::Result<IWMPMedia>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).add)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAll(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getAll)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByName<P0>(&self, bstrname: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getByName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByGenre<P0>(&self, bstrgenre: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getByGenre)(::windows_core::Interface::as_raw(self), bstrgenre.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAuthor<P0>(&self, bstrauthor: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getByAuthor)(::windows_core::Interface::as_raw(self), bstrauthor.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAlbum<P0>(&self, bstralbum: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getByAlbum)(::windows_core::Interface::as_raw(self), bstralbum.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAttribute<P0, P1>(&self, bstrattribute: P0, bstrvalue: P1) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getByAttribute)(::windows_core::Interface::as_raw(self), bstrattribute.into_param().abi(), bstrvalue.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn remove<P0, P1>(&self, pitem: P0, varfdeletefile: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).remove)(::windows_core::Interface::as_raw(self), pitem.into_param().abi(), varfdeletefile.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAttributeStringCollection<P0, P1>(&self, bstrattribute: P0, bstrmediatype: P1) -> ::windows_core::Result<IWMPStringCollection>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getAttributeStringCollection)(::windows_core::Interface::as_raw(self), bstrattribute.into_param().abi(), bstrmediatype.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getMediaAtom<P0>(&self, bstritemname: P0, platom: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getMediaAtom)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), platom).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn setDeleted<P0, P1>(&self, pitem: P0, varfisdeleted: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).setDeleted)(::windows_core::Interface::as_raw(self), pitem.into_param().abi(), varfisdeleted.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isDeleted<P0>(&self, pitem: P0, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).isDeleted)(::windows_core::Interface::as_raw(self), pitem.into_param().abi(), pvarfisdeleted).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPMediaCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPMediaCollection {
    type Vtable = IWMPMediaCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPMediaCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8363bc22_b4b4_4b19_989d_1cd765749dd1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMediaCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAll: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByGenre: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgenre: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByGenre: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByAuthor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrauthor: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByAuthor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByAlbum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstralbum: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByAlbum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByAttribute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfdeletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    remove: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getAttributeStringCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrmediatype: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppstringcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAttributeStringCollection: usize,
    pub getMediaAtom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, platom: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub setDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    setDeleted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub isDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    isDeleted: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPMediaCollection2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaCollection2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn add<P0>(&self, bstrurl: P0) -> ::windows_core::Result<IWMPMedia>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.add)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAll(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.getAll)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByName<P0>(&self, bstrname: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.getByName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByGenre<P0>(&self, bstrgenre: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.getByGenre)(::windows_core::Interface::as_raw(self), bstrgenre.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAuthor<P0>(&self, bstrauthor: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.getByAuthor)(::windows_core::Interface::as_raw(self), bstrauthor.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAlbum<P0>(&self, bstralbum: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.getByAlbum)(::windows_core::Interface::as_raw(self), bstralbum.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAttribute<P0, P1>(&self, bstrattribute: P0, bstrvalue: P1) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.getByAttribute)(::windows_core::Interface::as_raw(self), bstrattribute.into_param().abi(), bstrvalue.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn remove<P0, P1>(&self, pitem: P0, varfdeletefile: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.remove)(::windows_core::Interface::as_raw(self), pitem.into_param().abi(), varfdeletefile.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAttributeStringCollection<P0, P1>(&self, bstrattribute: P0, bstrmediatype: P1) -> ::windows_core::Result<IWMPStringCollection>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.getAttributeStringCollection)(::windows_core::Interface::as_raw(self), bstrattribute.into_param().abi(), bstrmediatype.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getMediaAtom<P0>(&self, bstritemname: P0, platom: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.getMediaAtom)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), platom).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn setDeleted<P0, P1>(&self, pitem: P0, varfisdeleted: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.setDeleted)(::windows_core::Interface::as_raw(self), pitem.into_param().abi(), varfisdeleted.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isDeleted<P0>(&self, pitem: P0, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.isDeleted)(::windows_core::Interface::as_raw(self), pitem.into_param().abi(), pvarfisdeleted).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createQuery(&self) -> ::windows_core::Result<IWMPQuery> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).createQuery)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn getPlaylistByQuery<P0, P1, P2, P3>(&self, pquery: P0, bstrmediatype: P1, bstrsortattribute: P2, fsortascending: P3) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<IWMPQuery>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getPlaylistByQuery)(::windows_core::Interface::as_raw(self), pquery.into_param().abi(), bstrmediatype.into_param().abi(), bstrsortattribute.into_param().abi(), fsortascending.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn getStringCollectionByQuery<P0, P1, P2, P3, P4>(&self, bstrattribute: P0, pquery: P1, bstrmediatype: P2, bstrsortattribute: P3, fsortascending: P4) -> ::windows_core::Result<IWMPStringCollection>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<IWMPQuery>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getStringCollectionByQuery)(::windows_core::Interface::as_raw(self), bstrattribute.into_param().abi(), pquery.into_param().abi(), bstrmediatype.into_param().abi(), bstrsortattribute.into_param().abi(), fsortascending.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAttributeAndMediaType<P0, P1, P2>(&self, bstrattribute: P0, bstrvalue: P1, bstrmediatype: P2) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getByAttributeAndMediaType)(::windows_core::Interface::as_raw(self), bstrattribute.into_param().abi(), bstrvalue.into_param().abi(), bstrmediatype.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPMediaCollection2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPMediaCollection);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPMediaCollection2 {
    type Vtable = IWMPMediaCollection2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPMediaCollection2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ba957f5_fd8c_4791_b82d_f840401ee474);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMediaCollection2_Vtbl {
    pub base__: IWMPMediaCollection_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub createQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createQuery: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub getPlaylistByQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pquery: *mut ::core::ffi::c_void, bstrmediatype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsortattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, fsortascending: super::super::Foundation::VARIANT_BOOL, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    getPlaylistByQuery: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub getStringCollectionByQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, pquery: *mut ::core::ffi::c_void, bstrmediatype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsortattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, fsortascending: super::super::Foundation::VARIANT_BOOL, ppstringcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    getStringCollectionByQuery: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByAttributeAndMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrmediatype: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByAttributeAndMediaType: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPMediaPluginRegistrar(::windows_core::IUnknown);
impl IWMPMediaPluginRegistrar {
    pub unsafe fn WMPRegisterPlayerPlugin<P0, P1, P2>(&self, pwszfriendlyname: P0, pwszdescription: P1, pwszuninstallstring: P2, dwpriority: u32, guidplugintype: ::windows_core::GUID, clsid: ::windows_core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WMPRegisterPlayerPlugin)(::windows_core::Interface::as_raw(self), pwszfriendlyname.into_param().abi(), pwszdescription.into_param().abi(), pwszuninstallstring.into_param().abi(), dwpriority, ::core::mem::transmute(guidplugintype), ::core::mem::transmute(clsid), cmediatypes, pmediatypes).ok()
    }
    pub unsafe fn WMPUnRegisterPlayerPlugin(&self, guidplugintype: ::windows_core::GUID, clsid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMPUnRegisterPlayerPlugin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidplugintype), ::core::mem::transmute(clsid)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPMediaPluginRegistrar, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPMediaPluginRegistrar {
    type Vtable = IWMPMediaPluginRegistrar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPMediaPluginRegistrar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68e27045_05bd_40b2_9720_23088c78e390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMediaPluginRegistrar_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub WMPRegisterPlayerPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows_core::PCWSTR, pwszdescription: ::windows_core::PCWSTR, pwszuninstallstring: ::windows_core::PCWSTR, dwpriority: u32, guidplugintype: ::windows_core::GUID, clsid: ::windows_core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WMPUnRegisterPlayerPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidplugintype: ::windows_core::GUID, clsid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPMetadataPicture(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMetadataPicture {
    pub unsafe fn mimeType(&self, pbstrmimetype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).mimeType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrmimetype)).ok()
    }
    pub unsafe fn pictureType(&self, pbstrpicturetype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).pictureType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrpicturetype)).ok()
    }
    pub unsafe fn description(&self, pbstrdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdescription)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).URL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPMetadataPicture, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPMetadataPicture {
    type Vtable = IWMPMetadataPicture_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPMetadataPicture {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c29bbe0_f87d_4c45_aa28_a70f0230ffa9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMetadataPicture_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub mimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub pictureType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpicturetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub URL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPMetadataText(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMetadataText {
    pub unsafe fn description(&self, pbstrdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdescription)).ok()
    }
    pub unsafe fn text(&self, pbstrtext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).text)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrtext)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPMetadataText, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPMetadataText {
    type Vtable = IWMPMetadataText_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPMetadataText {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x769a72db_13d2_45e2_9c48_53ca9d5b7450);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMetadataText_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPNetwork(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPNetwork {
    pub unsafe fn bandWidth(&self, plbandwidth: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).bandWidth)(::windows_core::Interface::as_raw(self), plbandwidth).ok()
    }
    pub unsafe fn recoveredPackets(&self, plrecoveredpackets: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).recoveredPackets)(::windows_core::Interface::as_raw(self), plrecoveredpackets).ok()
    }
    pub unsafe fn sourceProtocol(&self, pbstrsourceprotocol: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).sourceProtocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrsourceprotocol)).ok()
    }
    pub unsafe fn receivedPackets(&self, plreceivedpackets: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).receivedPackets)(::windows_core::Interface::as_raw(self), plreceivedpackets).ok()
    }
    pub unsafe fn lostPackets(&self, pllostpackets: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).lostPackets)(::windows_core::Interface::as_raw(self), pllostpackets).ok()
    }
    pub unsafe fn receptionQuality(&self, plreceptionquality: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).receptionQuality)(::windows_core::Interface::as_raw(self), plreceptionquality).ok()
    }
    pub unsafe fn bufferingCount(&self, plbufferingcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).bufferingCount)(::windows_core::Interface::as_raw(self), plbufferingcount).ok()
    }
    pub unsafe fn bufferingProgress(&self, plbufferingprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).bufferingProgress)(::windows_core::Interface::as_raw(self), plbufferingprogress).ok()
    }
    pub unsafe fn bufferingTime(&self, plbufferingtime: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).bufferingTime)(::windows_core::Interface::as_raw(self), plbufferingtime).ok()
    }
    pub unsafe fn SetbufferingTime(&self, lbufferingtime: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetbufferingTime)(::windows_core::Interface::as_raw(self), lbufferingtime).ok()
    }
    pub unsafe fn frameRate(&self, plframerate: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).frameRate)(::windows_core::Interface::as_raw(self), plframerate).ok()
    }
    pub unsafe fn maxBitRate(&self, plbitrate: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).maxBitRate)(::windows_core::Interface::as_raw(self), plbitrate).ok()
    }
    pub unsafe fn bitRate(&self, plbitrate: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).bitRate)(::windows_core::Interface::as_raw(self), plbitrate).ok()
    }
    pub unsafe fn getProxySettings<P0>(&self, bstrprotocol: P0, plproxysetting: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getProxySettings)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), plproxysetting).ok()
    }
    pub unsafe fn setProxySettings<P0>(&self, bstrprotocol: P0, lproxysetting: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setProxySettings)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), lproxysetting).ok()
    }
    pub unsafe fn getProxyName<P0>(&self, bstrprotocol: P0, pbstrproxyname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getProxyName)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), ::core::mem::transmute(pbstrproxyname)).ok()
    }
    pub unsafe fn setProxyName<P0, P1>(&self, bstrprotocol: P0, bstrproxyname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setProxyName)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), bstrproxyname.into_param().abi()).ok()
    }
    pub unsafe fn getProxyPort<P0>(&self, bstrprotocol: P0, lproxyport: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getProxyPort)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), lproxyport).ok()
    }
    pub unsafe fn setProxyPort<P0>(&self, bstrprotocol: P0, lproxyport: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setProxyPort)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), lproxyport).ok()
    }
    pub unsafe fn getProxyExceptionList<P0>(&self, bstrprotocol: P0, pbstrexceptionlist: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getProxyExceptionList)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), ::core::mem::transmute(pbstrexceptionlist)).ok()
    }
    pub unsafe fn setProxyExceptionList<P0, P1>(&self, bstrprotocol: P0, pbstrexceptionlist: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setProxyExceptionList)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), pbstrexceptionlist.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getProxyBypassForLocal<P0>(&self, bstrprotocol: P0, pfbypassforlocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getProxyBypassForLocal)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), pfbypassforlocal).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setProxyBypassForLocal<P0, P1>(&self, bstrprotocol: P0, fbypassforlocal: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).setProxyBypassForLocal)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    pub unsafe fn maxBandwidth(&self, lmaxbandwidth: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).maxBandwidth)(::windows_core::Interface::as_raw(self), lmaxbandwidth).ok()
    }
    pub unsafe fn SetmaxBandwidth(&self, lmaxbandwidth: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetmaxBandwidth)(::windows_core::Interface::as_raw(self), lmaxbandwidth).ok()
    }
    pub unsafe fn downloadProgress(&self, pldownloadprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).downloadProgress)(::windows_core::Interface::as_raw(self), pldownloadprogress).ok()
    }
    pub unsafe fn encodedFrameRate(&self, plframerate: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).encodedFrameRate)(::windows_core::Interface::as_raw(self), plframerate).ok()
    }
    pub unsafe fn framesSkipped(&self, plframes: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).framesSkipped)(::windows_core::Interface::as_raw(self), plframes).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPNetwork, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPNetwork {
    type Vtable = IWMPNetwork_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPNetwork {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec21b779_edef_462d_bba4_ad9dde2b29a7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNetwork_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub bandWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbandwidth: *mut i32) -> ::windows_core::HRESULT,
    pub recoveredPackets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plrecoveredpackets: *mut i32) -> ::windows_core::HRESULT,
    pub sourceProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsourceprotocol: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub receivedPackets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plreceivedpackets: *mut i32) -> ::windows_core::HRESULT,
    pub lostPackets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllostpackets: *mut i32) -> ::windows_core::HRESULT,
    pub receptionQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plreceptionquality: *mut i32) -> ::windows_core::HRESULT,
    pub bufferingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbufferingcount: *mut i32) -> ::windows_core::HRESULT,
    pub bufferingProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbufferingprogress: *mut i32) -> ::windows_core::HRESULT,
    pub bufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbufferingtime: *mut i32) -> ::windows_core::HRESULT,
    pub SetbufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbufferingtime: i32) -> ::windows_core::HRESULT,
    pub frameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows_core::HRESULT,
    pub maxBitRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows_core::HRESULT,
    pub bitRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows_core::HRESULT,
    pub getProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, plproxysetting: *mut i32) -> ::windows_core::HRESULT,
    pub setProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, lproxysetting: i32) -> ::windows_core::HRESULT,
    pub getProxyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrproxyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub setProxyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrproxyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub getProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, lproxyport: *mut i32) -> ::windows_core::HRESULT,
    pub setProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, lproxyport: i32) -> ::windows_core::HRESULT,
    pub getProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrexceptionlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub setProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrexceptionlist: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub getProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfbypassforlocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getProxyBypassForLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub setProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, fbypassforlocal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    setProxyBypassForLocal: usize,
    pub maxBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxbandwidth: *mut i32) -> ::windows_core::HRESULT,
    pub SetmaxBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxbandwidth: i32) -> ::windows_core::HRESULT,
    pub downloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldownloadprogress: *mut i32) -> ::windows_core::HRESULT,
    pub encodedFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows_core::HRESULT,
    pub framesSkipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plframes: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPNodeRealEstate(::windows_core::IUnknown);
impl IWMPNodeRealEstate {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesiredSize(&self, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesiredSize)(::windows_core::Interface::as_raw(self), psize).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRects(&self, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRects)(::windows_core::Interface::as_raw(self), psrc, pdest, pclip).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRects(&self, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRects)(::windows_core::Interface::as_raw(self), psrc, pdest, pclip).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWindowless<P0>(&self, fwindowless: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetWindowless)(::windows_core::Interface::as_raw(self), fwindowless.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowless(&self, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWindowless)(::windows_core::Interface::as_raw(self), pfwindowless).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullScreen<P0>(&self, ffullscreen: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetFullScreen)(::windows_core::Interface::as_raw(self), ffullscreen.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullScreen(&self, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFullScreen)(::windows_core::Interface::as_raw(self), pffullscreen).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPNodeRealEstate, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPNodeRealEstate {
    type Vtable = IWMPNodeRealEstate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPNodeRealEstate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42751198_5a50_4460_bcb4_709f8bdc8e59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeRealEstate_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesiredSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesiredSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWindowless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fwindowless: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWindowless: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindowless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindowless: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFullScreen: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPNodeRealEstateHost(::windows_core::IUnknown);
impl IWMPNodeRealEstateHost {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDesiredSizeChange(&self, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDesiredSizeChange)(::windows_core::Interface::as_raw(self), psize).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnFullScreenTransition<P0>(&self, ffullscreen: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).OnFullScreenTransition)(::windows_core::Interface::as_raw(self), ffullscreen.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPNodeRealEstateHost, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPNodeRealEstateHost {
    type Vtable = IWMPNodeRealEstateHost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPNodeRealEstateHost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1491087d_2c6b_44c8_b019_b3c929d2ada9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeRealEstateHost_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDesiredSizeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDesiredSizeChange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnFullScreenTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnFullScreenTransition: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPNodeWindowed(::windows_core::IUnknown);
impl IWMPNodeWindowed {
    pub unsafe fn SetOwnerWindow(&self, hwnd: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOwnerWindow)(::windows_core::Interface::as_raw(self), hwnd).ok()
    }
    pub unsafe fn GetOwnerWindow(&self, phwnd: *mut isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOwnerWindow)(::windows_core::Interface::as_raw(self), phwnd).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPNodeWindowed, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPNodeWindowed {
    type Vtable = IWMPNodeWindowed_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPNodeWindowed {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96740bfa_c56a_45d1_a3a4_762914d4ade9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeWindowed_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetOwnerWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: isize) -> ::windows_core::HRESULT,
    pub GetOwnerWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut isize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPNodeWindowedHost(::windows_core::IUnknown);
impl IWMPNodeWindowedHost {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessageFromRenderer<P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).OnWindowMessageFromRenderer)(::windows_core::Interface::as_raw(self), umsg, wparam.into_param().abi(), lparam.into_param().abi(), plret, pfhandled).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPNodeWindowedHost, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPNodeWindowedHost {
    type Vtable = IWMPNodeWindowedHost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPNodeWindowedHost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa300415a_54aa_4081_adbf_3b13610d8958);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeWindowedHost_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowMessageFromRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowMessageFromRenderer: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPNodeWindowless(::windows_core::IUnknown);
impl IWMPNodeWindowless {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessage<P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).base__.OnWindowMessage)(::windows_core::Interface::as_raw(self), umsg, wparam.into_param().abi(), lparam.into_param().abi(), plret, pfhandled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDraw(&self, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDraw)(::windows_core::Interface::as_raw(self), hdc, prcdraw).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPNodeWindowless, ::windows_core::IUnknown, IWMPWindowMessageSink);
unsafe impl ::windows_core::Interface for IWMPNodeWindowless {
    type Vtable = IWMPNodeWindowless_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPNodeWindowless {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b9199ad_780c_4eda_b816_261eba5d1575);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeWindowless_Vtbl {
    pub base__: IWMPWindowMessageSink_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDraw: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPNodeWindowlessHost(::windows_core::IUnknown);
impl IWMPNodeWindowlessHost {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvalidateRect<P0>(&self, prc: *const super::super::Foundation::RECT, ferase: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).InvalidateRect)(::windows_core::Interface::as_raw(self), prc, ferase.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPNodeWindowlessHost, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPNodeWindowlessHost {
    type Vtable = IWMPNodeWindowlessHost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPNodeWindowlessHost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe7017c6_ce34_4901_8106_770381aa6e3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeWindowlessHost_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InvalidateRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvalidateRect: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlayer(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer {
    pub unsafe fn close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.URL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.openState)(::windows_core::Interface::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.playState)(::windows_core::Interface::as_raw(self), pwmpps).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows_core::Result<IWMPControls> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.controls)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows_core::Result<IWMPSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.settings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.currentMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.SetcurrentMedia)(::windows_core::Interface::as_raw(self), pmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows_core::Result<IWMPMediaCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.mediaCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows_core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.playlistCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.versionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.launchURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows_core::Result<IWMPNetwork> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.network)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.currentPlaylist)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.SetcurrentPlaylist)(::windows_core::Interface::as_raw(self), ppl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows_core::Result<IWMPCdromCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.cdromCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows_core::Result<IWMPClosedCaption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.closedCaption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.isOnline)(::windows_core::Interface::as_raw(self), pfonline).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows_core::Result<IWMPError> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).enabled)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setenabled<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Setenabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).fullScreen)(::windows_core::Interface::as_raw(self), pbfullscreen).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetfullScreen<P0>(&self, bfullscreen: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetfullScreen)(::windows_core::Interface::as_raw(self), bfullscreen.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).enableContextMenu)(::windows_core::Interface::as_raw(self), pbenablecontextmenu).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetenableContextMenu<P0>(&self, benablecontextmenu: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetenableContextMenu)(::windows_core::Interface::as_raw(self), benablecontextmenu.into_param().abi()).ok()
    }
    pub unsafe fn SetuiMode<P0>(&self, bstrmode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetuiMode)(::windows_core::Interface::as_raw(self), bstrmode.into_param().abi()).ok()
    }
    pub unsafe fn uiMode(&self, pbstrmode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).uiMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrmode)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPPlayer, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPCore);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPPlayer {
    type Vtable = IWMPPlayer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPPlayer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bf52a4f_394a_11d3_b153_00c04f79faa6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer_Vtbl {
    pub base__: IWMPCore_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setenabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setenabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub fullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    fullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetfullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetfullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub enableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enableContextMenu: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetenableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetenableContextMenu: usize,
    pub SetuiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub uiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlayer2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer2 {
    pub unsafe fn close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.URL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.openState)(::windows_core::Interface::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.playState)(::windows_core::Interface::as_raw(self), pwmpps).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows_core::Result<IWMPControls> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.controls)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows_core::Result<IWMPSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.settings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.currentMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.SetcurrentMedia)(::windows_core::Interface::as_raw(self), pmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows_core::Result<IWMPMediaCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.mediaCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows_core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.playlistCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.versionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.launchURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows_core::Result<IWMPNetwork> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.network)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.currentPlaylist)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.SetcurrentPlaylist)(::windows_core::Interface::as_raw(self), ppl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows_core::Result<IWMPCdromCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.cdromCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows_core::Result<IWMPClosedCaption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.closedCaption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.isOnline)(::windows_core::Interface::as_raw(self), pfonline).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows_core::Result<IWMPError> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).enabled)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setenabled<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Setenabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).fullScreen)(::windows_core::Interface::as_raw(self), pbfullscreen).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetfullScreen<P0>(&self, bfullscreen: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetfullScreen)(::windows_core::Interface::as_raw(self), bfullscreen.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).enableContextMenu)(::windows_core::Interface::as_raw(self), pbenablecontextmenu).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetenableContextMenu<P0>(&self, benablecontextmenu: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetenableContextMenu)(::windows_core::Interface::as_raw(self), benablecontextmenu.into_param().abi()).ok()
    }
    pub unsafe fn SetuiMode<P0>(&self, bstrmode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetuiMode)(::windows_core::Interface::as_raw(self), bstrmode.into_param().abi()).ok()
    }
    pub unsafe fn uiMode(&self, pbstrmode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).uiMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrmode)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn stretchToFit(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).stretchToFit)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetstretchToFit<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetstretchToFit)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn windowlessVideo(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).windowlessVideo)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetwindowlessVideo<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetwindowlessVideo)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPPlayer2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPCore);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPPlayer2 {
    type Vtable = IWMPPlayer2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPPlayer2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e6b01d1_d407_4c85_bf5f_1c01f6150280);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer2_Vtbl {
    pub base__: IWMPCore_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setenabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setenabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub fullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    fullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetfullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetfullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub enableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enableContextMenu: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetenableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetenableContextMenu: usize,
    pub SetuiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub uiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub stretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    stretchToFit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetstretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetstretchToFit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub windowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    windowlessVideo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetwindowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetwindowlessVideo: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlayer3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer3 {
    pub unsafe fn close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.URL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.openState)(::windows_core::Interface::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.playState)(::windows_core::Interface::as_raw(self), pwmpps).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows_core::Result<IWMPControls> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.controls)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows_core::Result<IWMPSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.settings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.currentMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetcurrentMedia)(::windows_core::Interface::as_raw(self), pmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows_core::Result<IWMPMediaCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.mediaCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows_core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.playlistCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.versionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.launchURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows_core::Result<IWMPNetwork> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.network)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.currentPlaylist)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetcurrentPlaylist)(::windows_core::Interface::as_raw(self), ppl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows_core::Result<IWMPCdromCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.cdromCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows_core::Result<IWMPClosedCaption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.closedCaption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.isOnline)(::windows_core::Interface::as_raw(self), pfonline).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows_core::Result<IWMPError> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows_core::Result<IWMPDVD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.dvd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).enabled)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setenabled<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Setenabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).fullScreen)(::windows_core::Interface::as_raw(self), pbfullscreen).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetfullScreen<P0>(&self, bfullscreen: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetfullScreen)(::windows_core::Interface::as_raw(self), bfullscreen.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).enableContextMenu)(::windows_core::Interface::as_raw(self), pbenablecontextmenu).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetenableContextMenu<P0>(&self, benablecontextmenu: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetenableContextMenu)(::windows_core::Interface::as_raw(self), benablecontextmenu.into_param().abi()).ok()
    }
    pub unsafe fn SetuiMode<P0>(&self, bstrmode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetuiMode)(::windows_core::Interface::as_raw(self), bstrmode.into_param().abi()).ok()
    }
    pub unsafe fn uiMode(&self, pbstrmode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).uiMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrmode)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn stretchToFit(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).stretchToFit)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetstretchToFit<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetstretchToFit)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn windowlessVideo(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).windowlessVideo)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetwindowlessVideo<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetwindowlessVideo)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPPlayer3, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPCore, IWMPCore2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPPlayer3 {
    type Vtable = IWMPPlayer3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPPlayer3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54062b68_052a_4c25_a39f_8b63346511d4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer3_Vtbl {
    pub base__: IWMPCore2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setenabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setenabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub fullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    fullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetfullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetfullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub enableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enableContextMenu: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetenableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetenableContextMenu: usize,
    pub SetuiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub uiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub stretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    stretchToFit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetstretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetstretchToFit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub windowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    windowlessVideo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetwindowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetwindowlessVideo: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlayer4(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer4 {
    pub unsafe fn close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.URL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.openState)(::windows_core::Interface::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.playState)(::windows_core::Interface::as_raw(self), pwmpps).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows_core::Result<IWMPControls> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.controls)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows_core::Result<IWMPSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.settings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.currentMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetcurrentMedia)(::windows_core::Interface::as_raw(self), pmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows_core::Result<IWMPMediaCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.mediaCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows_core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.playlistCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.versionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.launchURL)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows_core::Result<IWMPNetwork> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.network)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.currentPlaylist)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetcurrentPlaylist)(::windows_core::Interface::as_raw(self), ppl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows_core::Result<IWMPCdromCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.cdromCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows_core::Result<IWMPClosedCaption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.closedCaption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.isOnline)(::windows_core::Interface::as_raw(self), pfonline).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows_core::Result<IWMPError> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows_core::Result<IWMPDVD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.dvd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newPlaylist<P0, P1>(&self, bstrname: P0, bstrurl: P1) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.newPlaylist)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), bstrurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newMedia<P0>(&self, bstrurl: P0) -> ::windows_core::Result<IWMPMedia>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.newMedia)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).enabled)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setenabled<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Setenabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).fullScreen)(::windows_core::Interface::as_raw(self), pbfullscreen).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetfullScreen<P0>(&self, bfullscreen: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetfullScreen)(::windows_core::Interface::as_raw(self), bfullscreen.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).enableContextMenu)(::windows_core::Interface::as_raw(self), pbenablecontextmenu).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetenableContextMenu<P0>(&self, benablecontextmenu: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetenableContextMenu)(::windows_core::Interface::as_raw(self), benablecontextmenu.into_param().abi()).ok()
    }
    pub unsafe fn SetuiMode<P0>(&self, bstrmode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetuiMode)(::windows_core::Interface::as_raw(self), bstrmode.into_param().abi()).ok()
    }
    pub unsafe fn uiMode(&self, pbstrmode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).uiMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrmode)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn stretchToFit(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).stretchToFit)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetstretchToFit<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetstretchToFit)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn windowlessVideo(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).windowlessVideo)(::windows_core::Interface::as_raw(self), pbenabled).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetwindowlessVideo<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetwindowlessVideo)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isRemote(&self, pvarfisremote: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).isRemote)(::windows_core::Interface::as_raw(self), pvarfisremote).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playerApplication(&self) -> ::windows_core::Result<IWMPPlayerApplication> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).playerApplication)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn openPlayer<P0>(&self, bstrurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).openPlayer)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPPlayer4, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPCore, IWMPCore2, IWMPCore3);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPPlayer4 {
    type Vtable = IWMPPlayer4_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPPlayer4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c497d62_8919_413c_82db_e935fb3ec584);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer4_Vtbl {
    pub base__: IWMPCore3_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setenabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setenabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub fullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    fullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetfullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetfullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub enableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enableContextMenu: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetenableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetenableContextMenu: usize,
    pub SetuiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub uiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub stretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    stretchToFit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetstretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetstretchToFit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub windowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    windowlessVideo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetwindowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetwindowlessVideo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub isRemote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarfisremote: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isRemote: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub playerApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwmpplayerapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    playerApplication: usize,
    pub openPlayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlayerApplication(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayerApplication {
    pub unsafe fn switchToPlayerApplication(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).switchToPlayerApplication)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn switchToControl(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).switchToControl)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn playerDocked(&self, pbplayerdocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).playerDocked)(::windows_core::Interface::as_raw(self), pbplayerdocked).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasDisplay(&self, pbhasdisplay: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).hasDisplay)(::windows_core::Interface::as_raw(self), pbhasdisplay).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPPlayerApplication, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPPlayerApplication {
    type Vtable = IWMPPlayerApplication_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPPlayerApplication {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40897764_ceab_47be_ad4a_8e28537f9bbf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayerApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub switchToPlayerApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub switchToControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub playerDocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbplayerdocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    playerDocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub hasDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbhasdisplay: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    hasDisplay: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlayerServices(::windows_core::IUnknown);
impl IWMPPlayerServices {
    pub unsafe fn activateUIPlugin<P0>(&self, bstrplugin: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).activateUIPlugin)(::windows_core::Interface::as_raw(self), bstrplugin.into_param().abi()).ok()
    }
    pub unsafe fn setTaskPane<P0>(&self, bstrtaskpane: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setTaskPane)(::windows_core::Interface::as_raw(self), bstrtaskpane.into_param().abi()).ok()
    }
    pub unsafe fn setTaskPaneURL<P0, P1, P2>(&self, bstrtaskpane: P0, bstrurl: P1, bstrfriendlyname: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setTaskPaneURL)(::windows_core::Interface::as_raw(self), bstrtaskpane.into_param().abi(), bstrurl.into_param().abi(), bstrfriendlyname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPPlayerServices, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPPlayerServices {
    type Vtable = IWMPPlayerServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPPlayerServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d01fbdb_ade2_4c8d_9842_c190b95c3306);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayerServices_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub activateUIPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrplugin: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub setTaskPane: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskpane: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub setTaskPaneURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskpane: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlayerServices2(::windows_core::IUnknown);
impl IWMPPlayerServices2 {
    pub unsafe fn activateUIPlugin<P0>(&self, bstrplugin: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.activateUIPlugin)(::windows_core::Interface::as_raw(self), bstrplugin.into_param().abi()).ok()
    }
    pub unsafe fn setTaskPane<P0>(&self, bstrtaskpane: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.setTaskPane)(::windows_core::Interface::as_raw(self), bstrtaskpane.into_param().abi()).ok()
    }
    pub unsafe fn setTaskPaneURL<P0, P1, P2>(&self, bstrtaskpane: P0, bstrurl: P1, bstrfriendlyname: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.setTaskPaneURL)(::windows_core::Interface::as_raw(self), bstrtaskpane.into_param().abi(), bstrurl.into_param().abi(), bstrfriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn setBackgroundProcessingPriority<P0>(&self, bstrpriority: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setBackgroundProcessingPriority)(::windows_core::Interface::as_raw(self), bstrpriority.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPPlayerServices2, ::windows_core::IUnknown, IWMPPlayerServices);
unsafe impl ::windows_core::Interface for IWMPPlayerServices2 {
    type Vtable = IWMPPlayerServices2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPPlayerServices2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bb1592f_f040_418a_9f71_17c7512b4d70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayerServices2_Vtbl {
    pub base__: IWMPPlayerServices_Vtbl,
    pub setBackgroundProcessingPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpriority: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlaylist(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylist {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).count)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Setname)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).attributeCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn get_attributeName(&self, lindex: i32, pbstrattributename: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_attributeName)(::windows_core::Interface::as_raw(self), lindex, ::core::mem::transmute(pbstrattributename)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_item(&self, lindex: i32) -> ::windows_core::Result<IWMPMedia> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn getItemInfo<P0>(&self, bstrname: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getItemInfo)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo<P0, P1>(&self, bstrname: P0, bstrvalue: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setItemInfo)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_isIdentical<P0>(&self, piwmpplaylist: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).get_isIdentical)(::windows_core::Interface::as_raw(self), piwmpplaylist.into_param().abi(), pvbool).ok()
    }
    pub unsafe fn clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn insertItem<P0>(&self, lindex: i32, piwmpmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).insertItem)(::windows_core::Interface::as_raw(self), lindex, piwmpmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendItem<P0>(&self, piwmpmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).appendItem)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeItem<P0>(&self, piwmpmedia: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).removeItem)(::windows_core::Interface::as_raw(self), piwmpmedia.into_param().abi()).ok()
    }
    pub unsafe fn moveItem(&self, lindexold: i32, lindexnew: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).moveItem)(::windows_core::Interface::as_raw(self), lindexold, lindexnew).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPPlaylist, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPPlaylist {
    type Vtable = IWMPPlaylist_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPPlaylist {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5f0f4f1_130c_11d3_b14e_00c04f79faa6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlaylist_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Setname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub attributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub get_attributeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstrattributename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_item: usize,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub setItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpplaylist: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_isIdentical: usize,
    pub clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub insertItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    insertItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub appendItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    appendItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub removeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    removeItem: usize,
    pub moveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindexold: i32, lindexnew: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlaylistArray(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylistArray {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).count)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn item(&self, lindex: i32) -> ::windows_core::Result<IWMPPlaylist> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPPlaylistArray, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPPlaylistArray {
    type Vtable = IWMPPlaylistArray_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPPlaylistArray {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x679409c0_99f7_11d3_9fb7_00105aa620bb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlaylistArray_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    item: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlaylistCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylistCollection {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newPlaylist<P0>(&self, bstrname: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).newPlaylist)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAll(&self) -> ::windows_core::Result<IWMPPlaylistArray> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getAll)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByName<P0>(&self, bstrname: P0) -> ::windows_core::Result<IWMPPlaylistArray>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getByName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn remove<P0>(&self, pitem: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).remove)(::windows_core::Interface::as_raw(self), pitem.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn setDeleted<P0, P1>(&self, pitem: P0, varfisdeleted: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).setDeleted)(::windows_core::Interface::as_raw(self), pitem.into_param().abi(), varfisdeleted.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isDeleted<P0>(&self, pitem: P0, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).isDeleted)(::windows_core::Interface::as_raw(self), pitem.into_param().abi(), pvarfisdeleted).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn importPlaylist<P0>(&self, pitem: P0) -> ::windows_core::Result<IWMPPlaylist>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).importPlaylist)(::windows_core::Interface::as_raw(self), pitem.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPPlaylistCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPPlaylistCollection {
    type Vtable = IWMPPlaylistCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPPlaylistCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10a13217_23a7_439b_b1c0_d847c79b7774);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlaylistCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub newPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    newPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplaylistarray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAll: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppplaylistarray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    remove: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub setDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    setDeleted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub isDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    isDeleted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub importPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, ppimporteditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    importPlaylist: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPlugin(::windows_core::IUnknown);
impl IWMPPlugin {
    pub unsafe fn Init(&self, dwplaybackcontext: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), dwplaybackcontext).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Shutdown)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetID(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetID)(::windows_core::Interface::as_raw(self), pguid).ok()
    }
    pub unsafe fn GetCaps(&self, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCaps)(::windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn AdviseWMPServices<P0>(&self, pwmpservices: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPServices>,
    {
        (::windows_core::Interface::vtable(self).AdviseWMPServices)(::windows_core::Interface::as_raw(self), pwmpservices.into_param().abi()).ok()
    }
    pub unsafe fn UnAdviseWMPServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnAdviseWMPServices)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPPlugin, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPPlugin {
    type Vtable = IWMPPlugin_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPPlugin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1392a70_024c_42bb_a998_73dfdfe7d5a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlugin_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwplaybackcontext: usize) -> ::windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub AdviseWMPServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpservices: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnAdviseWMPServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPluginEnable(::windows_core::IUnknown);
impl IWMPPluginEnable {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnable<P0>(&self, fenable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnable)(::windows_core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnable(&self, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEnable)(::windows_core::Interface::as_raw(self), pfenable).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPPluginEnable, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPPluginEnable {
    type Vtable = IWMPPluginEnable_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPPluginEnable {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5fca444c_7ad1_479d_a4ef_40566a5309d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPluginEnable_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnable: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPPluginUI(::windows_core::IUnknown);
impl IWMPPluginUI {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCore<P0>(&self, pcore: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPCore>,
    {
        (::windows_core::Interface::vtable(self).SetCore)(::windows_core::Interface::as_raw(self), pcore.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<P0>(&self, hwndparent: P0, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), phwndwindow).ok()
    }
    pub unsafe fn Destroy(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Destroy)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayPropertyPage<P0>(&self, hwndparent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).DisplayPropertyPage)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty<P0>(&self, pwszname: P0, pvarproperty: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi(), pvarproperty).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty<P0>(&self, pwszname: P0, pvarproperty: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi(), pvarproperty).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TranslateAccelerator)(::windows_core::Interface::as_raw(self), lpmsg).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPPluginUI, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPPluginUI {
    type Vtable = IWMPPluginUI_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPPluginUI {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c5e8f9f_ad3e_4bf9_9753_fcd30d6d38dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPluginUI_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayPropertyPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayPropertyPage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pvarproperty: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pvarproperty: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPQuery(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPQuery {
    pub unsafe fn addCondition<P0, P1, P2>(&self, bstrattribute: P0, bstroperator: P1, bstrvalue: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).addCondition)(::windows_core::Interface::as_raw(self), bstrattribute.into_param().abi(), bstroperator.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    pub unsafe fn beginNextGroup(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).beginNextGroup)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPQuery, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPQuery {
    type Vtable = IWMPQuery_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPQuery {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa00918f3_a6b0_4bfb_9189_fd834c7bc5a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPQuery_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub addCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstroperator: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub beginNextGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPRemoteMediaServices(::windows_core::IUnknown);
impl IWMPRemoteMediaServices {
    pub unsafe fn GetServiceType(&self, pbstrtype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetServiceType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrtype)).ok()
    }
    pub unsafe fn GetApplicationName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetApplicationName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetScriptableObject(&self, pbstrname: *mut ::windows_core::BSTR, ppdispatch: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetScriptableObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname), ::core::mem::transmute(ppdispatch)).ok()
    }
    pub unsafe fn GetCustomUIMode(&self, pbstrfile: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCustomUIMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrfile)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPRemoteMediaServices, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPRemoteMediaServices {
    type Vtable = IWMPRemoteMediaServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPRemoteMediaServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbb92747_741f_44fe_ab5b_f1a48f3b2a59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPRemoteMediaServices_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetServiceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetScriptableObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetScriptableObject: usize,
    pub GetCustomUIMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPRenderConfig(::windows_core::IUnknown);
impl IWMPRenderConfig {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetinProcOnly<P0>(&self, finproc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetinProcOnly)(::windows_core::Interface::as_raw(self), finproc.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn inProcOnly(&self, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).inProcOnly)(::windows_core::Interface::as_raw(self), pfinproc).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPRenderConfig, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPRenderConfig {
    type Vtable = IWMPRenderConfig_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPRenderConfig {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x959506c1_0314_4ec5_9e61_8528db5e5478);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPRenderConfig_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetinProcOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finproc: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetinProcOnly: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub inProcOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    inProcOnly: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPServices(::windows_core::IUnknown);
impl IWMPServices {
    pub unsafe fn GetStreamTime(&self, prt: *mut i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStreamTime)(::windows_core::Interface::as_raw(self), prt).ok()
    }
    pub unsafe fn GetStreamState(&self, pstate: *mut WMPServices_StreamState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStreamState)(::windows_core::Interface::as_raw(self), pstate).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPServices, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPServices {
    type Vtable = IWMPServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafb6b76b_1e20_4198_83b3_191db6e0b149);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPServices_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetStreamTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows_core::HRESULT,
    pub GetStreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut WMPServices_StreamState) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSettings(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPSettings {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_isAvailable<P0>(&self, bstritem: P0, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).get_isAvailable)(::windows_core::Interface::as_raw(self), bstritem.into_param().abi(), pisavailable).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn autoStart(&self, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).autoStart)(::windows_core::Interface::as_raw(self), pfautostart).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetautoStart<P0>(&self, fautostart: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetautoStart)(::windows_core::Interface::as_raw(self), fautostart.into_param().abi()).ok()
    }
    pub unsafe fn baseURL(&self, pbstrbaseurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).baseURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbaseurl)).ok()
    }
    pub unsafe fn SetbaseURL<P0>(&self, bstrbaseurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetbaseURL)(::windows_core::Interface::as_raw(self), bstrbaseurl.into_param().abi()).ok()
    }
    pub unsafe fn defaultFrame(&self, pbstrdefaultframe: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).defaultFrame)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdefaultframe)).ok()
    }
    pub unsafe fn SetdefaultFrame<P0>(&self, bstrdefaultframe: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetdefaultFrame)(::windows_core::Interface::as_raw(self), bstrdefaultframe.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn invokeURLs(&self, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).invokeURLs)(::windows_core::Interface::as_raw(self), pfinvokeurls).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetinvokeURLs<P0>(&self, finvokeurls: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetinvokeURLs)(::windows_core::Interface::as_raw(self), finvokeurls.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn mute(&self, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).mute)(::windows_core::Interface::as_raw(self), pfmute).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setmute<P0>(&self, fmute: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Setmute)(::windows_core::Interface::as_raw(self), fmute.into_param().abi()).ok()
    }
    pub unsafe fn playCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).playCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn SetplayCount(&self, lcount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetplayCount)(::windows_core::Interface::as_raw(self), lcount).ok()
    }
    pub unsafe fn rate(&self, pdrate: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).rate)(::windows_core::Interface::as_raw(self), pdrate).ok()
    }
    pub unsafe fn Setrate(&self, drate: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Setrate)(::windows_core::Interface::as_raw(self), drate).ok()
    }
    pub unsafe fn balance(&self, plbalance: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).balance)(::windows_core::Interface::as_raw(self), plbalance).ok()
    }
    pub unsafe fn Setbalance(&self, lbalance: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Setbalance)(::windows_core::Interface::as_raw(self), lbalance).ok()
    }
    pub unsafe fn volume(&self, plvolume: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).volume)(::windows_core::Interface::as_raw(self), plvolume).ok()
    }
    pub unsafe fn Setvolume(&self, lvolume: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Setvolume)(::windows_core::Interface::as_raw(self), lvolume).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getMode<P0>(&self, bstrmode: P0, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getMode)(::windows_core::Interface::as_raw(self), bstrmode.into_param().abi(), pvarfmode).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setMode<P0, P1>(&self, bstrmode: P0, varfmode: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).setMode)(::windows_core::Interface::as_raw(self), bstrmode.into_param().abi(), varfmode.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enableErrorDialogs(&self, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).enableErrorDialogs)(::windows_core::Interface::as_raw(self), pfenableerrordialogs).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetenableErrorDialogs<P0>(&self, fenableerrordialogs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetenableErrorDialogs)(::windows_core::Interface::as_raw(self), fenableerrordialogs.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPSettings, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPSettings {
    type Vtable = IWMPSettings_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9104d1ab_80c9_4fed_abf0_2e6417a6df14);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSettings_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_isAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_isAvailable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub autoStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    autoStart: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetautoStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fautostart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetautoStart: usize,
    pub baseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbaseurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetbaseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbaseurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub defaultFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdefaultframe: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetdefaultFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdefaultframe: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub invokeURLs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    invokeURLs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetinvokeURLs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finvokeurls: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetinvokeURLs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub mute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    mute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setmute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmute: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setmute: usize,
    pub playCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub SetplayCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcount: i32) -> ::windows_core::HRESULT,
    pub rate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdrate: *mut f64) -> ::windows_core::HRESULT,
    pub Setrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drate: f64) -> ::windows_core::HRESULT,
    pub balance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows_core::HRESULT,
    pub Setbalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows_core::HRESULT,
    pub volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows_core::HRESULT,
    pub Setvolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub getMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub setMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>, varfmode: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    setMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub enableErrorDialogs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enableErrorDialogs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetenableErrorDialogs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableerrordialogs: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetenableErrorDialogs: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSettings2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPSettings2 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_isAvailable<P0>(&self, bstritem: P0, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.get_isAvailable)(::windows_core::Interface::as_raw(self), bstritem.into_param().abi(), pisavailable).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn autoStart(&self, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.autoStart)(::windows_core::Interface::as_raw(self), pfautostart).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetautoStart<P0>(&self, fautostart: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetautoStart)(::windows_core::Interface::as_raw(self), fautostart.into_param().abi()).ok()
    }
    pub unsafe fn baseURL(&self, pbstrbaseurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.baseURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbaseurl)).ok()
    }
    pub unsafe fn SetbaseURL<P0>(&self, bstrbaseurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetbaseURL)(::windows_core::Interface::as_raw(self), bstrbaseurl.into_param().abi()).ok()
    }
    pub unsafe fn defaultFrame(&self, pbstrdefaultframe: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.defaultFrame)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdefaultframe)).ok()
    }
    pub unsafe fn SetdefaultFrame<P0>(&self, bstrdefaultframe: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetdefaultFrame)(::windows_core::Interface::as_raw(self), bstrdefaultframe.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn invokeURLs(&self, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.invokeURLs)(::windows_core::Interface::as_raw(self), pfinvokeurls).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetinvokeURLs<P0>(&self, finvokeurls: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetinvokeURLs)(::windows_core::Interface::as_raw(self), finvokeurls.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn mute(&self, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.mute)(::windows_core::Interface::as_raw(self), pfmute).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setmute<P0>(&self, fmute: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Setmute)(::windows_core::Interface::as_raw(self), fmute.into_param().abi()).ok()
    }
    pub unsafe fn playCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.playCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn SetplayCount(&self, lcount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetplayCount)(::windows_core::Interface::as_raw(self), lcount).ok()
    }
    pub unsafe fn rate(&self, pdrate: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.rate)(::windows_core::Interface::as_raw(self), pdrate).ok()
    }
    pub unsafe fn Setrate(&self, drate: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Setrate)(::windows_core::Interface::as_raw(self), drate).ok()
    }
    pub unsafe fn balance(&self, plbalance: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.balance)(::windows_core::Interface::as_raw(self), plbalance).ok()
    }
    pub unsafe fn Setbalance(&self, lbalance: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Setbalance)(::windows_core::Interface::as_raw(self), lbalance).ok()
    }
    pub unsafe fn volume(&self, plvolume: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.volume)(::windows_core::Interface::as_raw(self), plvolume).ok()
    }
    pub unsafe fn Setvolume(&self, lvolume: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Setvolume)(::windows_core::Interface::as_raw(self), lvolume).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getMode<P0>(&self, bstrmode: P0, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.getMode)(::windows_core::Interface::as_raw(self), bstrmode.into_param().abi(), pvarfmode).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setMode<P0, P1>(&self, bstrmode: P0, varfmode: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.setMode)(::windows_core::Interface::as_raw(self), bstrmode.into_param().abi(), varfmode.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enableErrorDialogs(&self, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.enableErrorDialogs)(::windows_core::Interface::as_raw(self), pfenableerrordialogs).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetenableErrorDialogs<P0>(&self, fenableerrordialogs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetenableErrorDialogs)(::windows_core::Interface::as_raw(self), fenableerrordialogs.into_param().abi()).ok()
    }
    pub unsafe fn defaultAudioLanguage(&self, pllangid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).defaultAudioLanguage)(::windows_core::Interface::as_raw(self), pllangid).ok()
    }
    pub unsafe fn mediaAccessRights(&self, pbstrrights: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).mediaAccessRights)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrrights)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn requestMediaAccessRights<P0>(&self, bstrdesiredaccess: P0, pvbaccepted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).requestMediaAccessRights)(::windows_core::Interface::as_raw(self), bstrdesiredaccess.into_param().abi(), pvbaccepted).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPSettings2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPSettings);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPSettings2 {
    type Vtable = IWMPSettings2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPSettings2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfda937a4_eece_4da5_a0b6_39bf89ade2c2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSettings2_Vtbl {
    pub base__: IWMPSettings_Vtbl,
    pub defaultAudioLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows_core::HRESULT,
    pub mediaAccessRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrights: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub requestMediaAccessRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdesiredaccess: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvbaccepted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    requestMediaAccessRights: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSkinManager(::windows_core::IUnknown);
impl IWMPSkinManager {
    pub unsafe fn SetVisualStyle<P0>(&self, bstrpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetVisualStyle)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPSkinManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPSkinManager {
    type Vtable = IWMPSkinManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPSkinManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x076f2fa6_ed30_448b_8cc5_3f3ef3529c7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSkinManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetVisualStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPStringCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPStringCollection {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).count)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn item(&self, lindex: i32, pbstrstring: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).item)(::windows_core::Interface::as_raw(self), lindex, ::core::mem::transmute(pbstrstring)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPStringCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPStringCollection {
    type Vtable = IWMPStringCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPStringCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a976298_8c0d_11d3_b389_00c04f68574b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPStringCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstrstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPStringCollection2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPStringCollection2 {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.count)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn item(&self, lindex: i32, pbstrstring: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.item)(::windows_core::Interface::as_raw(self), lindex, ::core::mem::transmute(pbstrstring)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isIdentical<P0>(&self, piwmpstringcollection2: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPStringCollection2>,
    {
        (::windows_core::Interface::vtable(self).isIdentical)(::windows_core::Interface::as_raw(self), piwmpstringcollection2.into_param().abi(), pvbool).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, lcollectionindex: i32, bstritemname: P0, pbstrvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getItemInfo)(::windows_core::Interface::as_raw(self), lcollectionindex, bstritemname.into_param().abi(), ::core::mem::transmute(pbstrvalue)).ok()
    }
    pub unsafe fn getAttributeCountByType<P0, P1>(&self, lcollectionindex: i32, bstrtype: P0, bstrlanguage: P1, plcount: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getAttributeCountByType)(::windows_core::Interface::as_raw(self), lcollectionindex, bstrtype.into_param().abi(), bstrlanguage.into_param().abi(), plcount).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn getItemInfoByType<P0, P1>(&self, lcollectionindex: i32, bstrtype: P0, bstrlanguage: P1, lattributeindex: i32, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getItemInfoByType)(::windows_core::Interface::as_raw(self), lcollectionindex, bstrtype.into_param().abi(), bstrlanguage.into_param().abi(), lattributeindex, pvarvalue).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWMPStringCollection2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWMPStringCollection);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWMPStringCollection2 {
    type Vtable = IWMPStringCollection2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWMPStringCollection2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46ad648d_53f1_4a74_92e2_2a1b68d63fd4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPStringCollection2_Vtbl {
    pub base__: IWMPStringCollection_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpstringcollection2: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    isIdentical: usize,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub getAttributeCountByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub getItemInfoByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, lattributeindex: i32, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    getItemInfoByType: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSubscriptionService(::windows_core::IUnknown);
impl IWMPSubscriptionService {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPlay<P0, P1>(&self, hwnd: P0, pmedia: P1, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).allowPlay)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), pmedia.into_param().abi(), pfallowplay).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowCDBurn<P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).allowCDBurn)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), pplaylist.into_param().abi(), pfallowburn).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPDATransfer<P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).allowPDATransfer)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), pplaylist.into_param().abi(), pfallowtransfer).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn startBackgroundProcessing<P0>(&self, hwnd: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).startBackgroundProcessing)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPSubscriptionService, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPSubscriptionService {
    type Vtable = IWMPSubscriptionService_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPSubscriptionService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x376055f8_2a59_4a73_9501_dca5273a7a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSubscriptionService_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub allowPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pmedia: *mut ::core::ffi::c_void, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    allowPlay: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub allowCDBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut ::core::ffi::c_void, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    allowCDBurn: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub allowPDATransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut ::core::ffi::c_void, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    allowPDATransfer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub startBackgroundProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    startBackgroundProcessing: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSubscriptionService2(::windows_core::IUnknown);
impl IWMPSubscriptionService2 {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPlay<P0, P1>(&self, hwnd: P0, pmedia: P1, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<IWMPMedia>,
    {
        (::windows_core::Interface::vtable(self).base__.allowPlay)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), pmedia.into_param().abi(), pfallowplay).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowCDBurn<P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.allowCDBurn)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), pplaylist.into_param().abi(), pfallowburn).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPDATransfer<P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).base__.allowPDATransfer)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), pplaylist.into_param().abi(), pfallowtransfer).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn startBackgroundProcessing<P0>(&self, hwnd: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.startBackgroundProcessing)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn stopBackgroundProcessing(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).stopBackgroundProcessing)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn serviceEvent(&self, event: WMPSubscriptionServiceEvent) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).serviceEvent)(::windows_core::Interface::as_raw(self), event).ok()
    }
    pub unsafe fn deviceAvailable<P0, P1>(&self, bstrdevicename: P0, pcb: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<IWMPSubscriptionServiceCallback>,
    {
        (::windows_core::Interface::vtable(self).deviceAvailable)(::windows_core::Interface::as_raw(self), bstrdevicename.into_param().abi(), pcb.into_param().abi()).ok()
    }
    pub unsafe fn prepareForSync<P0, P1, P2>(&self, bstrfilename: P0, bstrdevicename: P1, pcb: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<IWMPSubscriptionServiceCallback>,
    {
        (::windows_core::Interface::vtable(self).prepareForSync)(::windows_core::Interface::as_raw(self), bstrfilename.into_param().abi(), bstrdevicename.into_param().abi(), pcb.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPSubscriptionService2, ::windows_core::IUnknown, IWMPSubscriptionService);
unsafe impl ::windows_core::Interface for IWMPSubscriptionService2 {
    type Vtable = IWMPSubscriptionService2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPSubscriptionService2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa94c120e_d600_4ec6_b05e_ec9d56d84de0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSubscriptionService2_Vtbl {
    pub base__: IWMPSubscriptionService_Vtbl,
    pub stopBackgroundProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub serviceEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: WMPSubscriptionServiceEvent) -> ::windows_core::HRESULT,
    pub deviceAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub prepareForSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSubscriptionServiceCallback(::windows_core::IUnknown);
impl IWMPSubscriptionServiceCallback {
    pub unsafe fn onComplete(&self, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).onComplete)(::windows_core::Interface::as_raw(self), hrresult).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPSubscriptionServiceCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPSubscriptionServiceCallback {
    type Vtable = IWMPSubscriptionServiceCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPSubscriptionServiceCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd01d127_2dc2_4c3a_876e_63312079f9b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSubscriptionServiceCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub onComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSyncDevice(::windows_core::IUnknown);
impl IWMPSyncDevice {
    pub unsafe fn friendlyName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).friendlyName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn SetfriendlyName<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetfriendlyName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn deviceName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).deviceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn deviceId(&self, pbstrdeviceid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).deviceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn partnershipIndex(&self, plindex: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).partnershipIndex)(::windows_core::Interface::as_raw(self), plindex).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn connected(&self, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).connected)(::windows_core::Interface::as_raw(self), pvbconnected).ok()
    }
    pub unsafe fn status(&self, pwmpds: *mut WMPDeviceStatus) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).status)(::windows_core::Interface::as_raw(self), pwmpds).ok()
    }
    pub unsafe fn syncState(&self, pwmpss: *mut WMPSyncState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).syncState)(::windows_core::Interface::as_raw(self), pwmpss).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).progress)(::windows_core::Interface::as_raw(self), plprogress).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, bstritemname: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).getItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn createPartnership<P0>(&self, vbshowui: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).createPartnership)(::windows_core::Interface::as_raw(self), vbshowui.into_param().abi()).ok()
    }
    pub unsafe fn deletePartnership(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).deletePartnership)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn start(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).start)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn showSettings(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).showSettings)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isIdentical<P0>(&self, pdevice: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).isIdentical)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pvbool).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPSyncDevice, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPSyncDevice {
    type Vtable = IWMPSyncDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPSyncDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82a2986c_0293_4fd0_b279_b21b86c058be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncDevice_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub friendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetfriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub deviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub deviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub partnershipIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub connected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    connected: usize,
    pub status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> ::windows_core::HRESULT,
    pub syncState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpss: *mut WMPSyncState) -> ::windows_core::HRESULT,
    pub progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub createPartnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbshowui: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    createPartnership: usize,
    pub deletePartnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub showSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isIdentical: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSyncDevice2(::windows_core::IUnknown);
impl IWMPSyncDevice2 {
    pub unsafe fn friendlyName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.friendlyName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn SetfriendlyName<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetfriendlyName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn deviceName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.deviceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn deviceId(&self, pbstrdeviceid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.deviceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn partnershipIndex(&self, plindex: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.partnershipIndex)(::windows_core::Interface::as_raw(self), plindex).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn connected(&self, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.connected)(::windows_core::Interface::as_raw(self), pvbconnected).ok()
    }
    pub unsafe fn status(&self, pwmpds: *mut WMPDeviceStatus) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.status)(::windows_core::Interface::as_raw(self), pwmpds).ok()
    }
    pub unsafe fn syncState(&self, pwmpss: *mut WMPSyncState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.syncState)(::windows_core::Interface::as_raw(self), pwmpss).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.progress)(::windows_core::Interface::as_raw(self), plprogress).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, bstritemname: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.getItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn createPartnership<P0>(&self, vbshowui: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.createPartnership)(::windows_core::Interface::as_raw(self), vbshowui.into_param().abi()).ok()
    }
    pub unsafe fn deletePartnership(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.deletePartnership)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn start(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.start)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn showSettings(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.showSettings)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isIdentical<P0>(&self, pdevice: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.isIdentical)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pvbool).ok()
    }
    pub unsafe fn setItemInfo<P0, P1>(&self, bstritemname: P0, bstrval: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).setItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), bstrval.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPSyncDevice2, ::windows_core::IUnknown, IWMPSyncDevice);
unsafe impl ::windows_core::Interface for IWMPSyncDevice2 {
    type Vtable = IWMPSyncDevice2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPSyncDevice2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88afb4b2_140a_44d2_91e6_4543da467cd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncDevice2_Vtbl {
    pub base__: IWMPSyncDevice_Vtbl,
    pub setItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSyncDevice3(::windows_core::IUnknown);
impl IWMPSyncDevice3 {
    pub unsafe fn friendlyName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.friendlyName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn SetfriendlyName<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetfriendlyName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn deviceName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.deviceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn deviceId(&self, pbstrdeviceid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.deviceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn partnershipIndex(&self, plindex: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.partnershipIndex)(::windows_core::Interface::as_raw(self), plindex).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn connected(&self, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.connected)(::windows_core::Interface::as_raw(self), pvbconnected).ok()
    }
    pub unsafe fn status(&self, pwmpds: *mut WMPDeviceStatus) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.status)(::windows_core::Interface::as_raw(self), pwmpds).ok()
    }
    pub unsafe fn syncState(&self, pwmpss: *mut WMPSyncState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.syncState)(::windows_core::Interface::as_raw(self), pwmpss).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.progress)(::windows_core::Interface::as_raw(self), plprogress).ok()
    }
    pub unsafe fn getItemInfo<P0>(&self, bstritemname: P0, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.getItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn createPartnership<P0>(&self, vbshowui: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.createPartnership)(::windows_core::Interface::as_raw(self), vbshowui.into_param().abi()).ok()
    }
    pub unsafe fn deletePartnership(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.deletePartnership)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn start(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.start)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn showSettings(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.showSettings)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isIdentical<P0>(&self, pdevice: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPSyncDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.isIdentical)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pvbool).ok()
    }
    pub unsafe fn setItemInfo<P0, P1>(&self, bstritemname: P0, bstrval: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.setItemInfo)(::windows_core::Interface::as_raw(self), bstritemname.into_param().abi(), bstrval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn estimateSyncSize<P0, P1>(&self, pnonruleplaylist: P0, prulesplaylist: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMPPlaylist>,
        P1: ::windows_core::IntoParam<IWMPPlaylist>,
    {
        (::windows_core::Interface::vtable(self).estimateSyncSize)(::windows_core::Interface::as_raw(self), pnonruleplaylist.into_param().abi(), prulesplaylist.into_param().abi()).ok()
    }
    pub unsafe fn cancelEstimation(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).cancelEstimation)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPSyncDevice3, ::windows_core::IUnknown, IWMPSyncDevice, IWMPSyncDevice2);
unsafe impl ::windows_core::Interface for IWMPSyncDevice3 {
    type Vtable = IWMPSyncDevice3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPSyncDevice3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb22c85f9_263c_4372_a0da_b518db9b4098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncDevice3_Vtbl {
    pub base__: IWMPSyncDevice2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub estimateSyncSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnonruleplaylist: *mut ::core::ffi::c_void, prulesplaylist: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    estimateSyncSize: usize,
    pub cancelEstimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPSyncServices(::windows_core::IUnknown);
impl IWMPSyncServices {
    pub unsafe fn deviceCount(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).deviceCount)(::windows_core::Interface::as_raw(self), plcount).ok()
    }
    pub unsafe fn getDevice(&self, lindex: i32) -> ::windows_core::Result<IWMPSyncDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).getDevice)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMPSyncServices, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPSyncServices {
    type Vtable = IWMPSyncServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPSyncServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b5050ff_e0a4_4808_b3a8_893a9e1ed894);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncServices_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub deviceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub getDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPTranscodePolicy(::windows_core::IUnknown);
impl IWMPTranscodePolicy {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn allowTranscode(&self, pvballow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).allowTranscode)(::windows_core::Interface::as_raw(self), pvballow).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPTranscodePolicy, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPTranscodePolicy {
    type Vtable = IWMPTranscodePolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPTranscodePolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb64cbac3_401c_4327_a3e8_b9feb3a8c25c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPTranscodePolicy_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub allowTranscode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvballow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    allowTranscode: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPUserEventSink(::windows_core::IUnknown);
impl IWMPUserEventSink {
    pub unsafe fn NotifyUserEvent(&self, eventcode: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyUserEvent)(::windows_core::Interface::as_raw(self), eventcode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPUserEventSink, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPUserEventSink {
    type Vtable = IWMPUserEventSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPUserEventSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfccfa72_c343_48c3_a2de_b7a4402e39f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPUserEventSink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub NotifyUserEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcode: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPVideoRenderConfig(::windows_core::IUnknown);
impl IWMPVideoRenderConfig {
    #[doc = "Required features: `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn SetpresenterActivate<P0>(&self, pactivate: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::MediaFoundation::IMFActivate>,
    {
        (::windows_core::Interface::vtable(self).SetpresenterActivate)(::windows_core::Interface::as_raw(self), pactivate.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPVideoRenderConfig, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPVideoRenderConfig {
    type Vtable = IWMPVideoRenderConfig_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPVideoRenderConfig {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d6cf803_1ec0_4c8d_b3ca_f18e27282074);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPVideoRenderConfig_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub SetpresenterActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactivate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    SetpresenterActivate: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMPWindowMessageSink(::windows_core::IUnknown);
impl IWMPWindowMessageSink {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessage<P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).OnWindowMessage)(::windows_core::Interface::as_raw(self), umsg, wparam.into_param().abi(), lparam.into_param().abi(), plret, pfhandled).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMPWindowMessageSink, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMPWindowMessageSink {
    type Vtable = IWMPWindowMessageSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMPWindowMessageSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a0daa30_908d_4789_ba87_aed879b5c49b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPWindowMessageSink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowMessage: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeed(::windows_core::IUnknown);
impl IXFeed {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Xml)(::windows_core::Interface::as_raw(self), uiitemcount, sortproperty, sortorder, filterflags, includeflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Rename<P0>(&self, pszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Rename)(::windows_core::Interface::as_raw(self), pszname.into_param().abi()).ok()
    }
    pub unsafe fn Url(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Url)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUrl<P0>(&self, pszurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetUrl)(::windows_core::Interface::as_raw(self), pszurl.into_param().abi()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Move<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn Parent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastWriteTime(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastWriteTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Download)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows_core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SyncSetting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSyncSetting(&self, fss: FEEDS_SYNC_SETTING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSyncSetting)(::windows_core::Interface::as_raw(self), fss).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Interval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInterval(&self, uiinterval: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterval)(::windows_core::Interface::as_raw(self), uiinterval).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalEnclosurePath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Items(&self) -> ::windows_core::Result<IXFeedsEnum> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Items)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetItem<T>(&self, uiid: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetItem)(::windows_core::Interface::as_raw(self), uiid, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MarkAllItemsRead)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, uimaxitemcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxItemCount)(::windows_core::Interface::as_raw(self), uimaxitemcount).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadEnclosuresAutomatically)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDownloadEnclosuresAutomatically<P0>(&self, bdownloadenclosuresautomatically: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetDownloadEnclosuresAutomatically)(::windows_core::Interface::as_raw(self), bdownloadenclosuresautomatically.into_param().abi()).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastDownloadError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Merge<P0, P1>(&self, pstream: P0, pszurl: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Merge)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), pszurl.into_param().abi()).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Link)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Image(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Image)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastBuildDate(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastBuildDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PubDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Ttl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Language(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Language)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Copyright)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsList(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWatcher<T>(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetWatcher)(::windows_core::Interface::as_raw(self), scope, mask, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UnreadItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXFeed, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXFeed {
    type Vtable = IXFeed_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeed {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa44179a4_e0f6_403b_af8d_d080f425a451);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeed_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Xml: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastWriteTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastwritetime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastWriteTime: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CancelAsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SyncSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfss: *mut FEEDS_SYNC_SETTING) -> ::windows_core::HRESULT,
    pub SetSyncSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fss: FEEDS_SYNC_SETTING) -> ::windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastDownloadTime: usize,
    pub LocalEnclosurePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MarkAllItemsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puimaxitemcount: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uimaxitemcount: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DownloadEnclosuresAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdownloadenclosuresautomatically: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DownloadEnclosuresAutomatically: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDownloadEnclosuresAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDownloadEnclosuresAutomatically: usize,
    pub DownloadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows_core::HRESULT,
    pub LastDownloadError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Merge: usize,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsztitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszhomepage: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszimageurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastBuildDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastbuilddate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastBuildDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PubDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PubDate: usize,
    pub Ttl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puittl: *mut u32) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszlanguage: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Copyright: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcopyright: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbislist: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsList: usize,
    pub GetWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnreadItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiunreaditemcount: *mut u32) -> ::windows_core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiitemcount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeed2(::windows_core::IUnknown);
impl IXFeed2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Xml)(::windows_core::Interface::as_raw(self), uiitemcount, sortproperty, sortorder, filterflags, includeflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Rename<P0>(&self, pszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Rename)(::windows_core::Interface::as_raw(self), pszname.into_param().abi()).ok()
    }
    pub unsafe fn Url(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Url)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUrl<P0>(&self, pszurl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetUrl)(::windows_core::Interface::as_raw(self), pszurl.into_param().abi()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Move<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Move)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn Parent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastWriteTime(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastWriteTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Download)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CancelAsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows_core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SyncSetting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSyncSetting(&self, fss: FEEDS_SYNC_SETTING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSyncSetting)(::windows_core::Interface::as_raw(self), fss).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Interval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInterval(&self, uiinterval: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInterval)(::windows_core::Interface::as_raw(self), uiinterval).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalEnclosurePath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Items(&self) -> ::windows_core::Result<IXFeedsEnum> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Items)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetItem<T>(&self, uiid: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetItem)(::windows_core::Interface::as_raw(self), uiid, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MarkAllItemsRead)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MaxItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, uimaxitemcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMaxItemCount)(::windows_core::Interface::as_raw(self), uimaxitemcount).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadEnclosuresAutomatically)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDownloadEnclosuresAutomatically<P0>(&self, bdownloadenclosuresautomatically: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDownloadEnclosuresAutomatically)(::windows_core::Interface::as_raw(self), bdownloadenclosuresautomatically.into_param().abi()).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastDownloadError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Merge<P0, P1>(&self, pstream: P0, pszurl: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Merge)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), pszurl.into_param().abi()).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Link)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Image(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Image)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastBuildDate(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastBuildDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PubDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Ttl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Language(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Language)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Copyright)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsList(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWatcher<T>(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetWatcher)(::windows_core::Interface::as_raw(self), scope, mask, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UnreadItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetItemByEffectiveId<T>(&self, uieffectiveid: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetItemByEffectiveId)(::windows_core::Interface::as_raw(self), uieffectiveid, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastItemDownloadTime(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastItemDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Username(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Username)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Password(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Password)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCredentials<P0, P1>(&self, pszusername: P0, pszpassword: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCredentials)(::windows_core::Interface::as_raw(self), pszusername.into_param().abi(), pszpassword.into_param().abi()).ok()
    }
    pub unsafe fn ClearCredentials(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearCredentials)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXFeed2, ::windows_core::IUnknown, IXFeed);
unsafe impl ::windows_core::Interface for IXFeed2 {
    type Vtable = IXFeed2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeed2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce528e77_3716_4eb7_956d_f5e37502e12a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeed2_Vtbl {
    pub base__: IXFeed_Vtbl,
    pub GetItemByEffectiveId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uieffectiveid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastItemDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastitemdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastItemDownloadTime: usize,
    pub Username: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszusername: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpassword: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub ClearCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeedEnclosure(::windows_core::IUnknown);
impl IXFeedEnclosure {
    pub unsafe fn Url(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Url)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Length(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Length)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAsyncDownload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastDownloadError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LocalPath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DownloadMimeType(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadMimeType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveFile(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveFile)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetFile<P0, P1, P2, P3>(&self, pszdownloadurl: P0, pszdownloadfilepath: P1, pszdownloadmimetype: P2, pszenclosurefilename: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFile)(::windows_core::Interface::as_raw(self), pszdownloadurl.into_param().abi(), pszdownloadfilepath.into_param().abi(), pszdownloadmimetype.into_param().abi(), pszenclosurefilename.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXFeedEnclosure, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXFeedEnclosure {
    type Vtable = IXFeedEnclosure_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeedEnclosure {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfbfb953_644f_4792_b69c_dfaca4cbf89a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedEnclosure_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszmimetype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puilength: *mut u32) -> ::windows_core::HRESULT,
    pub AsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CancelAsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DownloadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows_core::HRESULT,
    pub LastDownloadError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT,
    pub LocalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub DownloadMimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszmimetype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub RemoveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdownloadurl: ::windows_core::PCWSTR, pszdownloadfilepath: ::windows_core::PCWSTR, pszdownloadmimetype: ::windows_core::PCWSTR, pszenclosurefilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeedEvents(::windows_core::IUnknown);
impl IXFeedEvents {
    pub unsafe fn Error(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FeedDeleted<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDeleted)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedRenamed<P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedRenamed)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pszoldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedUrlChanged<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedUrlChanged)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedMoved<P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedMoved)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pszoldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedDownloading<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDownloading)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedDownloadCompleted<P0>(&self, pszpath: P0, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDownloadCompleted)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), fde).ok()
    }
    pub unsafe fn FeedItemCountChanged<P0>(&self, pszpath: P0, feicfflags: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedItemCountChanged)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), feicfflags).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXFeedEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXFeedEvents {
    type Vtable = IXFeedEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeedEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1630852e_1263_465b_98e5_fe60ffec4ac2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FeedDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedUrlChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedDownloading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT,
    pub FeedItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, feicfflags: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeedFolder(::windows_core::IUnknown);
impl IXFeedFolder {
    pub unsafe fn Feeds(&self) -> ::windows_core::Result<IXFeedsEnum> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Feeds)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Subfolders(&self) -> ::windows_core::Result<IXFeedsEnum> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Subfolders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateFeed<P0, P1, T>(&self, pszname: P0, pszurl: P1) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateFeed)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), pszurl.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSubfolder<P0, T>(&self, pszname: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateSubfolder)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsFeed<P0>(&self, pszname: P0, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ExistsFeed)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), pbfeedexists).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsSubfolder<P0>(&self, pszname: P0, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ExistsSubfolder)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), pbsubfolderexists).ok()
    }
    pub unsafe fn GetFeed<P0, T>(&self, pszname: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetFeed)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSubfolder<P0, T>(&self, pszname: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetSubfolder)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Rename<P0>(&self, pszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Rename)(::windows_core::Interface::as_raw(self), pszname.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Move<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn Parent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRoot(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRoot)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWatcher<T>(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetWatcher)(::windows_core::Interface::as_raw(self), scope, mask, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalUnreadItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalUnreadItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXFeedFolder, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXFeedFolder {
    type Vtable = IXFeedFolder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeedFolder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c963678_3a51_4b88_8531_98b90b6508f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedFolder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Feeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Subfolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pszurl: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsFeed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsSubfolder: usize,
    pub GetFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisrootfeedfolder: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRoot: usize,
    pub GetWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TotalUnreadItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puitotalunreaditemcount: *mut u32) -> ::windows_core::HRESULT,
    pub TotalItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puitotalitemcount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeedFolderEvents(::windows_core::IUnknown);
impl IXFeedFolderEvents {
    pub unsafe fn Error(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FolderAdded<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderAdded)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn FolderDeleted<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderDeleted)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn FolderRenamed<P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderRenamed)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pszoldpath.into_param().abi()).ok()
    }
    pub unsafe fn FolderMovedFrom<P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderMovedFrom)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pszoldpath.into_param().abi()).ok()
    }
    pub unsafe fn FolderMovedTo<P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderMovedTo)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pszoldpath.into_param().abi()).ok()
    }
    pub unsafe fn FolderItemCountChanged<P0>(&self, pszpath: P0, feicfflags: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FolderItemCountChanged)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), feicfflags).ok()
    }
    pub unsafe fn FeedAdded<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedAdded)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedDeleted<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDeleted)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedRenamed<P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedRenamed)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pszoldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedUrlChanged<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedUrlChanged)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedMovedFrom<P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedMovedFrom)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pszoldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedMovedTo<P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedMovedTo)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pszoldpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedDownloading<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDownloading)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn FeedDownloadCompleted<P0>(&self, pszpath: P0, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedDownloadCompleted)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), fde).ok()
    }
    pub unsafe fn FeedItemCountChanged<P0>(&self, pszpath: P0, feicfflags: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FeedItemCountChanged)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), feicfflags).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXFeedFolderEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXFeedFolderEvents {
    type Vtable = IXFeedFolderEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeedFolderEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7964b769_234a_4bb1_a5f4_90454c8ad07e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedFolderEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FolderAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FolderDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FolderRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FolderMovedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FolderMovedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FolderItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, feicfflags: i32) -> ::windows_core::HRESULT,
    pub FeedAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedUrlChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedMovedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedMovedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedDownloading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub FeedDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT,
    pub FeedItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, feicfflags: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeedItem(::windows_core::IUnknown);
impl IXFeedItem {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, fxif: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Xml)(::windows_core::Interface::as_raw(self), fxif, &mut result__).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Link)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Guid)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PubDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Comments)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Author(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Author)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Enclosure<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).Enclosure)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRead(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRead)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRead<P0>(&self, bisread: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsRead)(::windows_core::Interface::as_raw(self), bisread.into_param().abi()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Modified(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Modified)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXFeedItem, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXFeedItem {
    type Vtable = IXFeedItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeedItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe757b2f5_e73e_434e_a1bf_2bd7c3e60fcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedItem_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fxif: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Xml: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsztitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszguid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PubDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PubDate: usize,
    pub Comments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszauthor: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Enclosure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisread: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRead: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisread: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsRead: usize,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiid: *mut u32) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastDownloadTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstmodifiedtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Modified: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeedItem2(::windows_core::IUnknown);
impl IXFeedItem2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, fxif: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Xml)(::windows_core::Interface::as_raw(self), fxif, &mut result__).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Link)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Guid)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PubDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Comments)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Author(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Author)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Enclosure<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.Enclosure)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRead(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRead)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRead<P0>(&self, bisread: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetIsRead)(::windows_core::Interface::as_raw(self), bisread.into_param().abi()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadUrl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastDownloadTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Modified(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Modified)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EffectiveId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EffectiveId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXFeedItem2, ::windows_core::IUnknown, IXFeedItem);
unsafe impl ::windows_core::Interface for IXFeedItem2 {
    type Vtable = IXFeedItem2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeedItem2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cda2dc7_9013_4522_9970_2a9dd9ead5a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedItem2_Vtbl {
    pub base__: IXFeedItem_Vtbl,
    pub EffectiveId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puieffectiveid: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeedsEnum(::windows_core::IUnknown);
impl IXFeedsEnum {
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Item<T>(&self, uiindex: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), uiindex, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXFeedsEnum, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXFeedsEnum {
    type Vtable = IXFeedsEnum_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeedsEnum {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc43a9d5_5015_4301_8c96_a47434b4d658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedsEnum_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXFeedsManager(::windows_core::IUnknown);
impl IXFeedsManager {
    pub unsafe fn RootFolder<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).RootFolder)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<P0>(&self, pszurl: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsSubscribed)(::windows_core::Interface::as_raw(self), pszurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsFeed<P0>(&self, pszpath: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExistsFeed)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFeed<P0, T>(&self, pszpath: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetFeed)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFeedByUrl<P0, T>(&self, pszurl: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetFeedByUrl)(::windows_core::Interface::as_raw(self), pszurl.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsFolder<P0>(&self, pszpath: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExistsFolder)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFolder<P0, T>(&self, pszpath: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetFolder)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteFeed<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteFeed)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn DeleteFolder<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteFolder)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn BackgroundSync(&self, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackgroundSync)(::windows_core::Interface::as_raw(self), fbsa).ok()
    }
    pub unsafe fn BackgroundSyncStatus(&self) -> ::windows_core::Result<FEEDS_BACKGROUNDSYNC_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BackgroundSyncStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DefaultInterval(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DefaultInterval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultInterval(&self, uiinterval: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultInterval)(::windows_core::Interface::as_raw(self), uiinterval).ok()
    }
    pub unsafe fn AsyncSyncAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncSyncAll)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Normalize<P0>(&self, pstreamin: P0) -> ::windows_core::Result<super::super::System::Com::IStream>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Normalize)(::windows_core::Interface::as_raw(self), pstreamin.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ItemCountLimit(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ItemCountLimit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXFeedsManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXFeedsManager {
    type Vtable = IXFeedsManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXFeedsManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5357e238_fb12_4aca_a930_cab7832b84bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedsManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RootFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSubscribed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, pbsubscribed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSubscribed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pbfeedexists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsFeed: usize,
    pub GetFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFeedByUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pbfolderexists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsFolder: usize,
    pub GetFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DeleteFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub DeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub BackgroundSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows_core::HRESULT,
    pub BackgroundSyncStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfbss: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows_core::HRESULT,
    pub DefaultInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows_core::HRESULT,
    pub SetDefaultInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows_core::HRESULT,
    pub AsyncSyncAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Normalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstreamin: *mut ::core::ffi::c_void, ppstreamout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Normalize: usize,
    pub ItemCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiitemcountlimit: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct _WMPOCXEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _WMPOCXEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(_WMPOCXEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for _WMPOCXEvents {
    type Vtable = _WMPOCXEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for _WMPOCXEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bf52a51_394a_11d3_b153_00c04f79faa6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _WMPOCXEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
pub const CLSID_WMPMediaPluginRegistrar: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5569e7f5_424b_4b93_89ca_79d17924689a);
pub const CLSID_WMPSkinManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2a7fd52_301f_4348_b93a_638c6de49229);
pub const CLSID_XFeedsManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe6b11c3_c72e_4061_86c6_9d163121f229);
pub const DISPID_DELTA: u32 = 50u32;
pub const DISPID_FEEDENCLOSURE_AsyncDownload: u32 = 24579u32;
pub const DISPID_FEEDENCLOSURE_CancelAsyncDownload: u32 = 24580u32;
pub const DISPID_FEEDENCLOSURE_DownloadMimeType: u32 = 24586u32;
pub const DISPID_FEEDENCLOSURE_DownloadStatus: u32 = 24581u32;
pub const DISPID_FEEDENCLOSURE_DownloadUrl: u32 = 24585u32;
pub const DISPID_FEEDENCLOSURE_LastDownloadError: u32 = 24582u32;
pub const DISPID_FEEDENCLOSURE_Length: u32 = 24578u32;
pub const DISPID_FEEDENCLOSURE_LocalPath: u32 = 24583u32;
pub const DISPID_FEEDENCLOSURE_Parent: u32 = 24584u32;
pub const DISPID_FEEDENCLOSURE_RemoveFile: u32 = 24587u32;
pub const DISPID_FEEDENCLOSURE_SetFile: u32 = 24588u32;
pub const DISPID_FEEDENCLOSURE_Type: u32 = 24577u32;
pub const DISPID_FEEDENCLOSURE_Url: u32 = 24576u32;
pub const DISPID_FEEDEVENTS_Error: u32 = 32768u32;
pub const DISPID_FEEDEVENTS_FeedDeleted: u32 = 32769u32;
pub const DISPID_FEEDEVENTS_FeedDownloadCompleted: u32 = 32774u32;
pub const DISPID_FEEDEVENTS_FeedDownloading: u32 = 32773u32;
pub const DISPID_FEEDEVENTS_FeedItemCountChanged: u32 = 32775u32;
pub const DISPID_FEEDEVENTS_FeedMoved: u32 = 32772u32;
pub const DISPID_FEEDEVENTS_FeedRenamed: u32 = 32770u32;
pub const DISPID_FEEDEVENTS_FeedUrlChanged: u32 = 32771u32;
pub const DISPID_FEEDFOLDEREVENTS_Error: u32 = 28672u32;
pub const DISPID_FEEDFOLDEREVENTS_FeedAdded: u32 = 28679u32;
pub const DISPID_FEEDFOLDEREVENTS_FeedDeleted: u32 = 28680u32;
pub const DISPID_FEEDFOLDEREVENTS_FeedDownloadCompleted: u32 = 28686u32;
pub const DISPID_FEEDFOLDEREVENTS_FeedDownloading: u32 = 28685u32;
pub const DISPID_FEEDFOLDEREVENTS_FeedItemCountChanged: u32 = 28687u32;
pub const DISPID_FEEDFOLDEREVENTS_FeedMovedFrom: u32 = 28683u32;
pub const DISPID_FEEDFOLDEREVENTS_FeedMovedTo: u32 = 28684u32;
pub const DISPID_FEEDFOLDEREVENTS_FeedRenamed: u32 = 28681u32;
pub const DISPID_FEEDFOLDEREVENTS_FeedUrlChanged: u32 = 28682u32;
pub const DISPID_FEEDFOLDEREVENTS_FolderAdded: u32 = 28673u32;
pub const DISPID_FEEDFOLDEREVENTS_FolderDeleted: u32 = 28674u32;
pub const DISPID_FEEDFOLDEREVENTS_FolderItemCountChanged: u32 = 28678u32;
pub const DISPID_FEEDFOLDEREVENTS_FolderMovedFrom: u32 = 28676u32;
pub const DISPID_FEEDFOLDEREVENTS_FolderMovedTo: u32 = 28677u32;
pub const DISPID_FEEDFOLDEREVENTS_FolderRenamed: u32 = 28675u32;
pub const DISPID_FEEDFOLDER_CreateFeed: u32 = 12290u32;
pub const DISPID_FEEDFOLDER_CreateSubfolder: u32 = 12291u32;
pub const DISPID_FEEDFOLDER_Delete: u32 = 12296u32;
pub const DISPID_FEEDFOLDER_ExistsFeed: u32 = 12292u32;
pub const DISPID_FEEDFOLDER_ExistsSubfolder: u32 = 12294u32;
pub const DISPID_FEEDFOLDER_Feeds: u32 = 12288u32;
pub const DISPID_FEEDFOLDER_GetFeed: u32 = 12293u32;
pub const DISPID_FEEDFOLDER_GetSubfolder: u32 = 12295u32;
pub const DISPID_FEEDFOLDER_GetWatcher: u32 = 12305u32;
pub const DISPID_FEEDFOLDER_IsRoot: u32 = 12302u32;
pub const DISPID_FEEDFOLDER_Move: u32 = 12300u32;
pub const DISPID_FEEDFOLDER_Name: u32 = 12297u32;
pub const DISPID_FEEDFOLDER_Parent: u32 = 12301u32;
pub const DISPID_FEEDFOLDER_Path: u32 = 12299u32;
pub const DISPID_FEEDFOLDER_Rename: u32 = 12298u32;
pub const DISPID_FEEDFOLDER_Subfolders: u32 = 12289u32;
pub const DISPID_FEEDFOLDER_TotalItemCount: u32 = 12304u32;
pub const DISPID_FEEDFOLDER_TotalUnreadItemCount: u32 = 12303u32;
pub const DISPID_FEEDITEM_Author: u32 = 20487u32;
pub const DISPID_FEEDITEM_Comments: u32 = 20486u32;
pub const DISPID_FEEDITEM_Delete: u32 = 20492u32;
pub const DISPID_FEEDITEM_Description: u32 = 20484u32;
pub const DISPID_FEEDITEM_DownloadUrl: u32 = 20493u32;
pub const DISPID_FEEDITEM_EffectiveId: u32 = 20496u32;
pub const DISPID_FEEDITEM_Enclosure: u32 = 20488u32;
pub const DISPID_FEEDITEM_Guid: u32 = 20483u32;
pub const DISPID_FEEDITEM_IsRead: u32 = 20489u32;
pub const DISPID_FEEDITEM_LastDownloadTime: u32 = 20494u32;
pub const DISPID_FEEDITEM_Link: u32 = 20482u32;
pub const DISPID_FEEDITEM_LocalId: u32 = 20490u32;
pub const DISPID_FEEDITEM_Modified: u32 = 20495u32;
pub const DISPID_FEEDITEM_Parent: u32 = 20491u32;
pub const DISPID_FEEDITEM_PubDate: u32 = 20485u32;
pub const DISPID_FEEDITEM_Title: u32 = 20481u32;
pub const DISPID_FEEDITEM_Xml: u32 = 20480u32;
pub const DISPID_FEEDSENUM_Count: u32 = 8192u32;
pub const DISPID_FEEDSENUM_Item: u32 = 8193u32;
pub const DISPID_FEEDS_AsyncSyncAll: u32 = 4108u32;
pub const DISPID_FEEDS_BackgroundSync: u32 = 4105u32;
pub const DISPID_FEEDS_BackgroundSyncStatus: u32 = 4106u32;
pub const DISPID_FEEDS_DefaultInterval: u32 = 4107u32;
pub const DISPID_FEEDS_DeleteFeed: u32 = 4102u32;
pub const DISPID_FEEDS_DeleteFolder: u32 = 4103u32;
pub const DISPID_FEEDS_ExistsFeed: u32 = 4098u32;
pub const DISPID_FEEDS_ExistsFolder: u32 = 4100u32;
pub const DISPID_FEEDS_GetFeed: u32 = 4099u32;
pub const DISPID_FEEDS_GetFeedByUrl: u32 = 4104u32;
pub const DISPID_FEEDS_GetFolder: u32 = 4101u32;
pub const DISPID_FEEDS_IsSubscribed: u32 = 4097u32;
pub const DISPID_FEEDS_ItemCountLimit: u32 = 4110u32;
pub const DISPID_FEEDS_Normalize: u32 = 4109u32;
pub const DISPID_FEEDS_RootFolder: u32 = 4096u32;
pub const DISPID_FEED_AsyncDownload: u32 = 16395u32;
pub const DISPID_FEED_CancelAsyncDownload: u32 = 16396u32;
pub const DISPID_FEED_ClearCredentials: u32 = 16428u32;
pub const DISPID_FEED_Copyright: u32 = 16411u32;
pub const DISPID_FEED_Delete: u32 = 16393u32;
pub const DISPID_FEED_Description: u32 = 16404u32;
pub const DISPID_FEED_Download: u32 = 16394u32;
pub const DISPID_FEED_DownloadEnclosuresAutomatically: u32 = 16412u32;
pub const DISPID_FEED_DownloadStatus: u32 = 16413u32;
pub const DISPID_FEED_DownloadUrl: u32 = 16416u32;
pub const DISPID_FEED_GetItem: u32 = 16402u32;
pub const DISPID_FEED_GetItemByEffectiveId: u32 = 16423u32;
pub const DISPID_FEED_GetWatcher: u32 = 16419u32;
pub const DISPID_FEED_Image: u32 = 16406u32;
pub const DISPID_FEED_Interval: u32 = 16397u32;
pub const DISPID_FEED_IsList: u32 = 16417u32;
pub const DISPID_FEED_ItemCount: u32 = 16421u32;
pub const DISPID_FEED_Items: u32 = 16401u32;
pub const DISPID_FEED_Language: u32 = 16410u32;
pub const DISPID_FEED_LastBuildDate: u32 = 16407u32;
pub const DISPID_FEED_LastDownloadError: u32 = 16414u32;
pub const DISPID_FEED_LastDownloadTime: u32 = 16399u32;
pub const DISPID_FEED_LastItemDownloadTime: u32 = 16424u32;
pub const DISPID_FEED_LastWriteTime: u32 = 16392u32;
pub const DISPID_FEED_Link: u32 = 16405u32;
pub const DISPID_FEED_LocalEnclosurePath: u32 = 16400u32;
pub const DISPID_FEED_LocalId: u32 = 16388u32;
pub const DISPID_FEED_MarkAllItemsRead: u32 = 16418u32;
pub const DISPID_FEED_MaxItemCount: u32 = 16422u32;
pub const DISPID_FEED_Merge: u32 = 16415u32;
pub const DISPID_FEED_Move: u32 = 16390u32;
pub const DISPID_FEED_Name: u32 = 16385u32;
pub const DISPID_FEED_Parent: u32 = 16391u32;
pub const DISPID_FEED_Password: u32 = 16426u32;
pub const DISPID_FEED_Path: u32 = 16389u32;
pub const DISPID_FEED_PubDate: u32 = 16408u32;
pub const DISPID_FEED_Rename: u32 = 16386u32;
pub const DISPID_FEED_SetCredentials: u32 = 16427u32;
pub const DISPID_FEED_SyncSetting: u32 = 16398u32;
pub const DISPID_FEED_Title: u32 = 16403u32;
pub const DISPID_FEED_Ttl: u32 = 16409u32;
pub const DISPID_FEED_UnreadItemCount: u32 = 16420u32;
pub const DISPID_FEED_Url: u32 = 16387u32;
pub const DISPID_FEED_Username: u32 = 16425u32;
pub const DISPID_FEED_Xml: u32 = 16384u32;
pub const DISPID_WMPCDROMCOLLECTION_BASE: u32 = 300u32;
pub const DISPID_WMPCDROMCOLLECTION_COUNT: u32 = 301u32;
pub const DISPID_WMPCDROMCOLLECTION_GETBYDRIVESPECIFIER: u32 = 303u32;
pub const DISPID_WMPCDROMCOLLECTION_ITEM: u32 = 302u32;
pub const DISPID_WMPCDROMCOLLECTION_STARTMONITORINGCDROMS: u32 = 304u32;
pub const DISPID_WMPCDROMCOLLECTION_STOPMONITORINGCDROMS: u32 = 305u32;
pub const DISPID_WMPCDROM_BASE: u32 = 250u32;
pub const DISPID_WMPCDROM_DRIVESPECIFIER: u32 = 251u32;
pub const DISPID_WMPCDROM_EJECT: u32 = 253u32;
pub const DISPID_WMPCDROM_PLAYLIST: u32 = 252u32;
pub const DISPID_WMPCLOSEDCAPTION2_GETLANGCOUNT: u32 = 955u32;
pub const DISPID_WMPCLOSEDCAPTION2_GETLANGID: u32 = 957u32;
pub const DISPID_WMPCLOSEDCAPTION2_GETLANGNAME: u32 = 956u32;
pub const DISPID_WMPCLOSEDCAPTION2_GETSTYLECOUNT: u32 = 958u32;
pub const DISPID_WMPCLOSEDCAPTION2_GETSTYLENAME: u32 = 959u32;
pub const DISPID_WMPCLOSEDCAPTION_BASE: u32 = 950u32;
pub const DISPID_WMPCLOSEDCAPTION_CAPTIONINGID: u32 = 954u32;
pub const DISPID_WMPCLOSEDCAPTION_SAMIFILENAME: u32 = 953u32;
pub const DISPID_WMPCLOSEDCAPTION_SAMILANG: u32 = 952u32;
pub const DISPID_WMPCLOSEDCAPTION_SAMISTYLE: u32 = 951u32;
pub const DISPID_WMPCONTROLS2_STEP: u32 = 64u32;
pub const DISPID_WMPCONTROLS3_AUDIOLANGUAGECOUNT: u32 = 65u32;
pub const DISPID_WMPCONTROLS3_CURRENTAUDIOLANGUAGE: u32 = 68u32;
pub const DISPID_WMPCONTROLS3_CURRENTAUDIOLANGUAGEINDEX: u32 = 69u32;
pub const DISPID_WMPCONTROLS3_CURRENTPOSITIONTIMECODE: u32 = 71u32;
pub const DISPID_WMPCONTROLS3_GETAUDIOLANGUAGEDESC: u32 = 67u32;
pub const DISPID_WMPCONTROLS3_GETAUDIOLANGUAGEID: u32 = 66u32;
pub const DISPID_WMPCONTROLS3_GETLANGUAGENAME: u32 = 70u32;
pub const DISPID_WMPCONTROLSFAKE_TIMECOMPRESSION: u32 = 72u32;
pub const DISPID_WMPCONTROLS_BASE: u32 = 50u32;
pub const DISPID_WMPCONTROLS_CURRENTITEM: u32 = 60u32;
pub const DISPID_WMPCONTROLS_CURRENTMARKER: u32 = 61u32;
pub const DISPID_WMPCONTROLS_CURRENTPOSITION: u32 = 56u32;
pub const DISPID_WMPCONTROLS_CURRENTPOSITIONSTRING: u32 = 57u32;
pub const DISPID_WMPCONTROLS_FASTFORWARD: u32 = 54u32;
pub const DISPID_WMPCONTROLS_FASTREVERSE: u32 = 55u32;
pub const DISPID_WMPCONTROLS_ISAVAILABLE: u32 = 62u32;
pub const DISPID_WMPCONTROLS_NEXT: u32 = 58u32;
pub const DISPID_WMPCONTROLS_PAUSE: u32 = 53u32;
pub const DISPID_WMPCONTROLS_PLAY: u32 = 51u32;
pub const DISPID_WMPCONTROLS_PLAYITEM: u32 = 63u32;
pub const DISPID_WMPCONTROLS_PREVIOUS: u32 = 59u32;
pub const DISPID_WMPCONTROLS_STOP: u32 = 52u32;
pub const DISPID_WMPCORE2_BASE: u32 = 39u32;
pub const DISPID_WMPCORE2_DVD: u32 = 40u32;
pub const DISPID_WMPCORE3_NEWMEDIA: u32 = 42u32;
pub const DISPID_WMPCORE3_NEWPLAYLIST: u32 = 41u32;
pub const DISPID_WMPCOREEVENT_AUDIOLANGUAGECHANGE: u32 = 5102u32;
pub const DISPID_WMPCOREEVENT_BUFFERING: u32 = 5402u32;
pub const DISPID_WMPCOREEVENT_CDROMMEDIACHANGE: u32 = 5701u32;
pub const DISPID_WMPCOREEVENT_CURRENTITEMCHANGE: u32 = 5806u32;
pub const DISPID_WMPCOREEVENT_CURRENTMEDIAITEMAVAILABLE: u32 = 5803u32;
pub const DISPID_WMPCOREEVENT_CURRENTPLAYLISTCHANGE: u32 = 5804u32;
pub const DISPID_WMPCOREEVENT_CURRENTPLAYLISTITEMAVAILABLE: u32 = 5805u32;
pub const DISPID_WMPCOREEVENT_DISCONNECT: u32 = 5401u32;
pub const DISPID_WMPCOREEVENT_DOMAINCHANGE: u32 = 5822u32;
pub const DISPID_WMPCOREEVENT_DURATIONUNITCHANGE: u32 = 5204u32;
pub const DISPID_WMPCOREEVENT_ENDOFSTREAM: u32 = 5201u32;
pub const DISPID_WMPCOREEVENT_ERROR: u32 = 5501u32;
pub const DISPID_WMPCOREEVENT_MARKERHIT: u32 = 5203u32;
pub const DISPID_WMPCOREEVENT_MEDIACHANGE: u32 = 5802u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONATTRIBUTESTRINGADDED: u32 = 5808u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONATTRIBUTESTRINGCHANGED: u32 = 5820u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONATTRIBUTESTRINGREMOVED: u32 = 5809u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONCHANGE: u32 = 5807u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONCONTENTSCANADDEDITEM: u32 = 5813u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONCONTENTSCANPROGRESS: u32 = 5814u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONMEDIAADDED: u32 = 5825u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONMEDIAREMOVED: u32 = 5826u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONSEARCHCOMPLETE: u32 = 5817u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONSEARCHFOUNDITEM: u32 = 5815u32;
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONSEARCHPROGRESS: u32 = 5816u32;
pub const DISPID_WMPCOREEVENT_MEDIAERROR: u32 = 5821u32;
pub const DISPID_WMPCOREEVENT_MODECHANGE: u32 = 5819u32;
pub const DISPID_WMPCOREEVENT_NEWSTREAM: u32 = 5403u32;
pub const DISPID_WMPCOREEVENT_OPENPLAYLISTSWITCH: u32 = 5823u32;
pub const DISPID_WMPCOREEVENT_OPENSTATECHANGE: u32 = 5001u32;
pub const DISPID_WMPCOREEVENT_PLAYLISTCHANGE: u32 = 5801u32;
pub const DISPID_WMPCOREEVENT_PLAYLISTCOLLECTIONCHANGE: u32 = 5810u32;
pub const DISPID_WMPCOREEVENT_PLAYLISTCOLLECTIONPLAYLISTADDED: u32 = 5811u32;
pub const DISPID_WMPCOREEVENT_PLAYLISTCOLLECTIONPLAYLISTREMOVED: u32 = 5812u32;
pub const DISPID_WMPCOREEVENT_PLAYLISTCOLLECTIONPLAYLISTSETASDELETED: u32 = 5818u32;
pub const DISPID_WMPCOREEVENT_PLAYSTATECHANGE: u32 = 5101u32;
pub const DISPID_WMPCOREEVENT_POSITIONCHANGE: u32 = 5202u32;
pub const DISPID_WMPCOREEVENT_SCRIPTCOMMAND: u32 = 5301u32;
pub const DISPID_WMPCOREEVENT_STATUSCHANGE: u32 = 5002u32;
pub const DISPID_WMPCOREEVENT_STRINGCOLLECTIONCHANGE: u32 = 5824u32;
pub const DISPID_WMPCOREEVENT_WARNING: u32 = 5601u32;
pub const DISPID_WMPCORE_BASE: u32 = 0u32;
pub const DISPID_WMPCORE_CDROMCOLLECTION: u32 = 14u32;
pub const DISPID_WMPCORE_CLOSE: u32 = 3u32;
pub const DISPID_WMPCORE_CLOSEDCAPTION: u32 = 15u32;
pub const DISPID_WMPCORE_CONTROLS: u32 = 4u32;
pub const DISPID_WMPCORE_CURRENTMEDIA: u32 = 6u32;
pub const DISPID_WMPCORE_CURRENTPLAYLIST: u32 = 13u32;
pub const DISPID_WMPCORE_ERROR: u32 = 17u32;
pub const DISPID_WMPCORE_ISONLINE: u32 = 16u32;
pub const DISPID_WMPCORE_LAST: u32 = 18u32;
pub const DISPID_WMPCORE_LAUNCHURL: u32 = 12u32;
pub const DISPID_WMPCORE_MAX: u32 = 1454u32;
pub const DISPID_WMPCORE_MEDIACOLLECTION: u32 = 8u32;
pub const DISPID_WMPCORE_MIN: u32 = 1u32;
pub const DISPID_WMPCORE_NETWORK: u32 = 7u32;
pub const DISPID_WMPCORE_OPENSTATE: u32 = 2u32;
pub const DISPID_WMPCORE_PLAYLISTCOLLECTION: u32 = 9u32;
pub const DISPID_WMPCORE_PLAYSTATE: u32 = 10u32;
pub const DISPID_WMPCORE_SETTINGS: u32 = 5u32;
pub const DISPID_WMPCORE_STATUS: u32 = 18u32;
pub const DISPID_WMPCORE_URL: u32 = 1u32;
pub const DISPID_WMPCORE_VERSIONINFO: u32 = 11u32;
pub const DISPID_WMPDOWNLOADCOLLECTION_BASE: u32 = 1200u32;
pub const DISPID_WMPDOWNLOADCOLLECTION_CLEAR: u32 = 1206u32;
pub const DISPID_WMPDOWNLOADCOLLECTION_COUNT: u32 = 1202u32;
pub const DISPID_WMPDOWNLOADCOLLECTION_ID: u32 = 1201u32;
pub const DISPID_WMPDOWNLOADCOLLECTION_ITEM: u32 = 1203u32;
pub const DISPID_WMPDOWNLOADCOLLECTION_REMOVEITEM: u32 = 1205u32;
pub const DISPID_WMPDOWNLOADCOLLECTION_STARTDOWNLOAD: u32 = 1204u32;
pub const DISPID_WMPDOWNLOADITEM2_BASE: u32 = 1300u32;
pub const DISPID_WMPDOWNLOADITEM2_GETITEMINFO: u32 = 1301u32;
pub const DISPID_WMPDOWNLOADITEM_BASE: u32 = 1250u32;
pub const DISPID_WMPDOWNLOADITEM_CANCEL: u32 = 1258u32;
pub const DISPID_WMPDOWNLOADITEM_DOWNLOADSTATE: u32 = 1255u32;
pub const DISPID_WMPDOWNLOADITEM_PAUSE: u32 = 1256u32;
pub const DISPID_WMPDOWNLOADITEM_PROGRESS: u32 = 1254u32;
pub const DISPID_WMPDOWNLOADITEM_RESUME: u32 = 1257u32;
pub const DISPID_WMPDOWNLOADITEM_SIZE: u32 = 1252u32;
pub const DISPID_WMPDOWNLOADITEM_SOURCEURL: u32 = 1251u32;
pub const DISPID_WMPDOWNLOADITEM_TYPE: u32 = 1253u32;
pub const DISPID_WMPDOWNLOADMANAGER_BASE: u32 = 1150u32;
pub const DISPID_WMPDOWNLOADMANAGER_CREATEDOWNLOADCOLLECTION: u32 = 1152u32;
pub const DISPID_WMPDOWNLOADMANAGER_GETDOWNLOADCOLLECTION: u32 = 1151u32;
pub const DISPID_WMPDVD_BACK: u32 = 1005u32;
pub const DISPID_WMPDVD_BASE: u32 = 1000u32;
pub const DISPID_WMPDVD_DOMAIN: u32 = 1002u32;
pub const DISPID_WMPDVD_ISAVAILABLE: u32 = 1001u32;
pub const DISPID_WMPDVD_RESUME: u32 = 1006u32;
pub const DISPID_WMPDVD_TITLEMENU: u32 = 1004u32;
pub const DISPID_WMPDVD_TOPMENU: u32 = 1003u32;
pub const DISPID_WMPERRORITEM2_CONDITION: u32 = 906u32;
pub const DISPID_WMPERRORITEM_BASE: u32 = 900u32;
pub const DISPID_WMPERRORITEM_CUSTOMURL: u32 = 905u32;
pub const DISPID_WMPERRORITEM_ERRORCODE: u32 = 901u32;
pub const DISPID_WMPERRORITEM_ERRORCONTEXT: u32 = 903u32;
pub const DISPID_WMPERRORITEM_ERRORDESCRIPTION: u32 = 902u32;
pub const DISPID_WMPERRORITEM_REMEDY: u32 = 904u32;
pub const DISPID_WMPERROR_BASE: u32 = 850u32;
pub const DISPID_WMPERROR_CLEARERRORQUEUE: u32 = 851u32;
pub const DISPID_WMPERROR_ERRORCOUNT: u32 = 852u32;
pub const DISPID_WMPERROR_ITEM: u32 = 853u32;
pub const DISPID_WMPERROR_WEBHELP: u32 = 854u32;
pub const DISPID_WMPMEDIA2_ERROR: u32 = 768u32;
pub const DISPID_WMPMEDIA3_GETATTRIBUTECOUNTBYTYPE: u32 = 769u32;
pub const DISPID_WMPMEDIA3_GETITEMINFOBYTYPE: u32 = 770u32;
pub const DISPID_WMPMEDIACOLLECTION2_BASE: u32 = 1400u32;
pub const DISPID_WMPMEDIACOLLECTION2_CREATEQUERY: u32 = 1401u32;
pub const DISPID_WMPMEDIACOLLECTION2_GETBYATTRANDMEDIATYPE: u32 = 1404u32;
pub const DISPID_WMPMEDIACOLLECTION2_GETPLAYLISTBYQUERY: u32 = 1402u32;
pub const DISPID_WMPMEDIACOLLECTION2_GETSTRINGCOLLBYQUERY: u32 = 1403u32;
pub const DISPID_WMPMEDIACOLLECTION_ADD: u32 = 452u32;
pub const DISPID_WMPMEDIACOLLECTION_BASE: u32 = 450u32;
pub const DISPID_WMPMEDIACOLLECTION_FREEZECOLLECTIONCHANGE: u32 = 474u32;
pub const DISPID_WMPMEDIACOLLECTION_GETALL: u32 = 453u32;
pub const DISPID_WMPMEDIACOLLECTION_GETATTRIBUTESTRINGCOLLECTION: u32 = 461u32;
pub const DISPID_WMPMEDIACOLLECTION_GETBYALBUM: u32 = 457u32;
pub const DISPID_WMPMEDIACOLLECTION_GETBYATTRIBUTE: u32 = 458u32;
pub const DISPID_WMPMEDIACOLLECTION_GETBYAUTHOR: u32 = 456u32;
pub const DISPID_WMPMEDIACOLLECTION_GETBYGENRE: u32 = 455u32;
pub const DISPID_WMPMEDIACOLLECTION_GETBYNAME: u32 = 454u32;
pub const DISPID_WMPMEDIACOLLECTION_GETBYQUERYDESCRIPTION: u32 = 473u32;
pub const DISPID_WMPMEDIACOLLECTION_GETMEDIAATOM: u32 = 470u32;
pub const DISPID_WMPMEDIACOLLECTION_ISDELETED: u32 = 472u32;
pub const DISPID_WMPMEDIACOLLECTION_NEWQUERY: u32 = 462u32;
pub const DISPID_WMPMEDIACOLLECTION_POSTCOLLECTIONCHANGE: u32 = 476u32;
pub const DISPID_WMPMEDIACOLLECTION_REMOVE: u32 = 459u32;
pub const DISPID_WMPMEDIACOLLECTION_SETDELETED: u32 = 471u32;
pub const DISPID_WMPMEDIACOLLECTION_STARTCONTENTSCAN: u32 = 465u32;
pub const DISPID_WMPMEDIACOLLECTION_STARTMONITORING: u32 = 463u32;
pub const DISPID_WMPMEDIACOLLECTION_STARTSEARCH: u32 = 467u32;
pub const DISPID_WMPMEDIACOLLECTION_STOPCONTENTSCAN: u32 = 466u32;
pub const DISPID_WMPMEDIACOLLECTION_STOPMONITORING: u32 = 464u32;
pub const DISPID_WMPMEDIACOLLECTION_STOPSEARCH: u32 = 468u32;
pub const DISPID_WMPMEDIACOLLECTION_UNFREEZECOLLECTIONCHANGE: u32 = 475u32;
pub const DISPID_WMPMEDIACOLLECTION_UPDATEMETADATA: u32 = 469u32;
pub const DISPID_WMPMEDIA_ATTRIBUTECOUNT: u32 = 759u32;
pub const DISPID_WMPMEDIA_BASE: u32 = 750u32;
pub const DISPID_WMPMEDIA_DURATION: u32 = 757u32;
pub const DISPID_WMPMEDIA_DURATIONSTRING: u32 = 758u32;
pub const DISPID_WMPMEDIA_GETATTRIBUTENAME: u32 = 760u32;
pub const DISPID_WMPMEDIA_GETITEMINFO: u32 = 761u32;
pub const DISPID_WMPMEDIA_GETITEMINFOBYATOM: u32 = 765u32;
pub const DISPID_WMPMEDIA_GETMARKERNAME: u32 = 756u32;
pub const DISPID_WMPMEDIA_GETMARKERTIME: u32 = 755u32;
pub const DISPID_WMPMEDIA_IMAGESOURCEHEIGHT: u32 = 753u32;
pub const DISPID_WMPMEDIA_IMAGESOURCEWIDTH: u32 = 752u32;
pub const DISPID_WMPMEDIA_ISIDENTICAL: u32 = 763u32;
pub const DISPID_WMPMEDIA_ISMEMBEROF: u32 = 766u32;
pub const DISPID_WMPMEDIA_ISREADONLYITEM: u32 = 767u32;
pub const DISPID_WMPMEDIA_MARKERCOUNT: u32 = 754u32;
pub const DISPID_WMPMEDIA_NAME: u32 = 764u32;
pub const DISPID_WMPMEDIA_SETITEMINFO: u32 = 762u32;
pub const DISPID_WMPMEDIA_SOURCEURL: u32 = 751u32;
pub const DISPID_WMPMETADATA_BASE: u32 = 1050u32;
pub const DISPID_WMPMETADATA_PICTURE_DESCRIPTION: u32 = 1053u32;
pub const DISPID_WMPMETADATA_PICTURE_MIMETYPE: u32 = 1051u32;
pub const DISPID_WMPMETADATA_PICTURE_PICTURETYPE: u32 = 1052u32;
pub const DISPID_WMPMETADATA_PICTURE_URL: u32 = 1054u32;
pub const DISPID_WMPMETADATA_TEXT_DESCRIPTION: u32 = 1056u32;
pub const DISPID_WMPMETADATA_TEXT_TEXT: u32 = 1055u32;
pub const DISPID_WMPNETWORK_BANDWIDTH: u32 = 801u32;
pub const DISPID_WMPNETWORK_BASE: u32 = 800u32;
pub const DISPID_WMPNETWORK_BITRATE: u32 = 812u32;
pub const DISPID_WMPNETWORK_BUFFERINGCOUNT: u32 = 807u32;
pub const DISPID_WMPNETWORK_BUFFERINGPROGRESS: u32 = 808u32;
pub const DISPID_WMPNETWORK_BUFFERINGTIME: u32 = 809u32;
pub const DISPID_WMPNETWORK_DOWNLOADPROGRESS: u32 = 824u32;
pub const DISPID_WMPNETWORK_ENCODEDFRAMERATE: u32 = 825u32;
pub const DISPID_WMPNETWORK_FRAMERATE: u32 = 810u32;
pub const DISPID_WMPNETWORK_FRAMESSKIPPED: u32 = 826u32;
pub const DISPID_WMPNETWORK_GETPROXYBYPASSFORLOCAL: u32 = 821u32;
pub const DISPID_WMPNETWORK_GETPROXYEXCEPTIONLIST: u32 = 819u32;
pub const DISPID_WMPNETWORK_GETPROXYNAME: u32 = 815u32;
pub const DISPID_WMPNETWORK_GETPROXYPORT: u32 = 817u32;
pub const DISPID_WMPNETWORK_GETPROXYSETTINGS: u32 = 813u32;
pub const DISPID_WMPNETWORK_LOSTPACKETS: u32 = 805u32;
pub const DISPID_WMPNETWORK_MAXBANDWIDTH: u32 = 823u32;
pub const DISPID_WMPNETWORK_MAXBITRATE: u32 = 811u32;
pub const DISPID_WMPNETWORK_RECEIVEDPACKETS: u32 = 804u32;
pub const DISPID_WMPNETWORK_RECEPTIONQUALITY: u32 = 806u32;
pub const DISPID_WMPNETWORK_RECOVEREDPACKETS: u32 = 802u32;
pub const DISPID_WMPNETWORK_SETPROXYBYPASSFORLOCAL: u32 = 822u32;
pub const DISPID_WMPNETWORK_SETPROXYEXCEPTIONLIST: u32 = 820u32;
pub const DISPID_WMPNETWORK_SETPROXYNAME: u32 = 816u32;
pub const DISPID_WMPNETWORK_SETPROXYPORT: u32 = 818u32;
pub const DISPID_WMPNETWORK_SETPROXYSETTINGS: u32 = 814u32;
pub const DISPID_WMPNETWORK_SOURCEPROTOCOL: u32 = 803u32;
pub const DISPID_WMPOCX2_BASE: u32 = 23u32;
pub const DISPID_WMPOCX2_STRETCHTOFIT: u32 = 24u32;
pub const DISPID_WMPOCX2_WINDOWLESSVIDEO: u32 = 25u32;
pub const DISPID_WMPOCX4_ISREMOTE: u32 = 26u32;
pub const DISPID_WMPOCX4_OPENPLAYER: u32 = 28u32;
pub const DISPID_WMPOCX4_PLAYERAPPLICATION: u32 = 27u32;
pub const DISPID_WMPOCXEVENT_CDROMBURNERROR: u32 = 6523u32;
pub const DISPID_WMPOCXEVENT_CDROMBURNMEDIAERROR: u32 = 6522u32;
pub const DISPID_WMPOCXEVENT_CDROMBURNSTATECHANGE: u32 = 6521u32;
pub const DISPID_WMPOCXEVENT_CDROMRIPMEDIAERROR: u32 = 6520u32;
pub const DISPID_WMPOCXEVENT_CDROMRIPSTATECHANGE: u32 = 6519u32;
pub const DISPID_WMPOCXEVENT_CLICK: u32 = 6505u32;
pub const DISPID_WMPOCXEVENT_CREATEPARTNERSHIPCOMPLETE: u32 = 6518u32;
pub const DISPID_WMPOCXEVENT_DEVICECONNECT: u32 = 6513u32;
pub const DISPID_WMPOCXEVENT_DEVICEDISCONNECT: u32 = 6514u32;
pub const DISPID_WMPOCXEVENT_DEVICEESTIMATION: u32 = 6527u32;
pub const DISPID_WMPOCXEVENT_DEVICESTATUSCHANGE: u32 = 6515u32;
pub const DISPID_WMPOCXEVENT_DEVICESYNCERROR: u32 = 6517u32;
pub const DISPID_WMPOCXEVENT_DEVICESYNCSTATECHANGE: u32 = 6516u32;
pub const DISPID_WMPOCXEVENT_DOUBLECLICK: u32 = 6506u32;
pub const DISPID_WMPOCXEVENT_FOLDERSCANSTATECHANGE: u32 = 6526u32;
pub const DISPID_WMPOCXEVENT_KEYDOWN: u32 = 6507u32;
pub const DISPID_WMPOCXEVENT_KEYPRESS: u32 = 6508u32;
pub const DISPID_WMPOCXEVENT_KEYUP: u32 = 6509u32;
pub const DISPID_WMPOCXEVENT_LIBRARYCONNECT: u32 = 6524u32;
pub const DISPID_WMPOCXEVENT_LIBRARYDISCONNECT: u32 = 6525u32;
pub const DISPID_WMPOCXEVENT_MOUSEDOWN: u32 = 6510u32;
pub const DISPID_WMPOCXEVENT_MOUSEMOVE: u32 = 6511u32;
pub const DISPID_WMPOCXEVENT_MOUSEUP: u32 = 6512u32;
pub const DISPID_WMPOCXEVENT_PLAYERDOCKEDSTATECHANGE: u32 = 6503u32;
pub const DISPID_WMPOCXEVENT_PLAYERRECONNECT: u32 = 6504u32;
pub const DISPID_WMPOCXEVENT_SWITCHEDTOCONTROL: u32 = 6502u32;
pub const DISPID_WMPOCXEVENT_SWITCHEDTOPLAYERAPPLICATION: u32 = 6501u32;
pub const DISPID_WMPOCX_BASE: u32 = 18u32;
pub const DISPID_WMPOCX_ENABLECONTEXTMENU: u32 = 22u32;
pub const DISPID_WMPOCX_ENABLED: u32 = 19u32;
pub const DISPID_WMPOCX_FULLSCREEN: u32 = 21u32;
pub const DISPID_WMPOCX_LAST: u32 = 23u32;
pub const DISPID_WMPOCX_TRANSPARENTATSTART: u32 = 20u32;
pub const DISPID_WMPOCX_UIMODE: u32 = 23u32;
pub const DISPID_WMPPLAYERAPP_BASE: u32 = 1100u32;
pub const DISPID_WMPPLAYERAPP_HASDISPLAY: u32 = 1104u32;
pub const DISPID_WMPPLAYERAPP_PLAYERDOCKED: u32 = 1103u32;
pub const DISPID_WMPPLAYERAPP_REMOTESTATUS: u32 = 1105u32;
pub const DISPID_WMPPLAYERAPP_SWITCHTOCONTROL: u32 = 1102u32;
pub const DISPID_WMPPLAYERAPP_SWITCHTOPLAYERAPPLICATION: u32 = 1101u32;
pub const DISPID_WMPPLAYLISTARRAY_BASE: u32 = 500u32;
pub const DISPID_WMPPLAYLISTARRAY_COUNT: u32 = 501u32;
pub const DISPID_WMPPLAYLISTARRAY_ITEM: u32 = 502u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_BASE: u32 = 550u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_GETALL: u32 = 553u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_GETBYNAME: u32 = 554u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_GETBYQUERYDESCRIPTION: u32 = 555u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_IMPORTPLAYLIST: u32 = 562u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_ISDELETED: u32 = 561u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_NEWPLAYLIST: u32 = 552u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_NEWQUERY: u32 = 557u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_REMOVE: u32 = 556u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_SETDELETED: u32 = 560u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_STARTMONITORING: u32 = 558u32;
pub const DISPID_WMPPLAYLISTCOLLECTION_STOPMONITORING: u32 = 559u32;
pub const DISPID_WMPPLAYLIST_APPENDITEM: u32 = 207u32;
pub const DISPID_WMPPLAYLIST_ATTRIBUTECOUNT: u32 = 210u32;
pub const DISPID_WMPPLAYLIST_ATTRIBUTENAME: u32 = 211u32;
pub const DISPID_WMPPLAYLIST_BASE: u32 = 200u32;
pub const DISPID_WMPPLAYLIST_CLEAR: u32 = 205u32;
pub const DISPID_WMPPLAYLIST_COUNT: u32 = 201u32;
pub const DISPID_WMPPLAYLIST_GETITEMINFO: u32 = 203u32;
pub const DISPID_WMPPLAYLIST_INSERTITEM: u32 = 206u32;
pub const DISPID_WMPPLAYLIST_ISIDENTICAL: u32 = 213u32;
pub const DISPID_WMPPLAYLIST_ITEM: u32 = 212u32;
pub const DISPID_WMPPLAYLIST_MOVEITEM: u32 = 209u32;
pub const DISPID_WMPPLAYLIST_NAME: u32 = 202u32;
pub const DISPID_WMPPLAYLIST_REMOVEITEM: u32 = 208u32;
pub const DISPID_WMPPLAYLIST_SETITEMINFO: u32 = 204u32;
pub const DISPID_WMPQUERY_ADDCONDITION: u32 = 1351u32;
pub const DISPID_WMPQUERY_BASE: u32 = 1350u32;
pub const DISPID_WMPQUERY_BEGINNEXTGROUP: u32 = 1352u32;
pub const DISPID_WMPSETTINGS2_DEFAULTAUDIOLANGUAGE: u32 = 114u32;
pub const DISPID_WMPSETTINGS2_LIBRARYACCESSRIGHTS: u32 = 115u32;
pub const DISPID_WMPSETTINGS2_REQUESTLIBRARYACCESSRIGHTS: u32 = 116u32;
pub const DISPID_WMPSETTINGS_AUTOSTART: u32 = 101u32;
pub const DISPID_WMPSETTINGS_BALANCE: u32 = 102u32;
pub const DISPID_WMPSETTINGS_BASE: u32 = 100u32;
pub const DISPID_WMPSETTINGS_BASEURL: u32 = 108u32;
pub const DISPID_WMPSETTINGS_DEFAULTFRAME: u32 = 109u32;
pub const DISPID_WMPSETTINGS_ENABLEERRORDIALOGS: u32 = 112u32;
pub const DISPID_WMPSETTINGS_GETMODE: u32 = 110u32;
pub const DISPID_WMPSETTINGS_INVOKEURLS: u32 = 103u32;
pub const DISPID_WMPSETTINGS_ISAVAILABLE: u32 = 113u32;
pub const DISPID_WMPSETTINGS_MUTE: u32 = 104u32;
pub const DISPID_WMPSETTINGS_PLAYCOUNT: u32 = 105u32;
pub const DISPID_WMPSETTINGS_RATE: u32 = 106u32;
pub const DISPID_WMPSETTINGS_SETMODE: u32 = 111u32;
pub const DISPID_WMPSETTINGS_VOLUME: u32 = 107u32;
pub const DISPID_WMPSTRINGCOLLECTION2_BASE: u32 = 1450u32;
pub const DISPID_WMPSTRINGCOLLECTION2_GETATTRCOUNTBYTYPE: u32 = 1453u32;
pub const DISPID_WMPSTRINGCOLLECTION2_GETITEMINFO: u32 = 1452u32;
pub const DISPID_WMPSTRINGCOLLECTION2_GETITEMINFOBYTYPE: u32 = 1454u32;
pub const DISPID_WMPSTRINGCOLLECTION2_ISIDENTICAL: u32 = 1451u32;
pub const DISPID_WMPSTRINGCOLLECTION_BASE: u32 = 400u32;
pub const DISPID_WMPSTRINGCOLLECTION_COUNT: u32 = 401u32;
pub const DISPID_WMPSTRINGCOLLECTION_ITEM: u32 = 402u32;
pub const EFFECT2_FULLSCREENEXCLUSIVE: u32 = 16u32;
pub const EFFECT_CANGOFULLSCREEN: u32 = 1u32;
pub const EFFECT_HASPROPERTYPAGE: u32 = 2u32;
pub const EFFECT_VARIABLEFREQSTEP: u32 = 4u32;
pub const EFFECT_WINDOWEDONLY: u32 = 8u32;
pub const FBSA_DISABLE: FEEDS_BACKGROUNDSYNC_ACTION = FEEDS_BACKGROUNDSYNC_ACTION(0i32);
pub const FBSA_ENABLE: FEEDS_BACKGROUNDSYNC_ACTION = FEEDS_BACKGROUNDSYNC_ACTION(1i32);
pub const FBSA_RUNNOW: FEEDS_BACKGROUNDSYNC_ACTION = FEEDS_BACKGROUNDSYNC_ACTION(2i32);
pub const FBSS_DISABLED: FEEDS_BACKGROUNDSYNC_STATUS = FEEDS_BACKGROUNDSYNC_STATUS(0i32);
pub const FBSS_ENABLED: FEEDS_BACKGROUNDSYNC_STATUS = FEEDS_BACKGROUNDSYNC_STATUS(1i32);
pub const FDE_ACCESS_DENIED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(13i32);
pub const FDE_AUTH_FAILED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(14i32);
pub const FDE_BACKGROUND_DOWNLOAD_DISABLED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(8i32);
pub const FDE_CANCELED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(6i32);
pub const FDE_DOWNLOAD_BLOCKED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(5i32);
pub const FDE_DOWNLOAD_FAILED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(1i32);
pub const FDE_DOWNLOAD_SIZE_LIMIT_EXCEEDED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(12i32);
pub const FDE_INVALID_AUTH: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(15i32);
pub const FDE_INVALID_FEED_FORMAT: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(2i32);
pub const FDE_NONE: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(0i32);
pub const FDE_NORMALIZATION_FAILED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(3i32);
pub const FDE_NOT_EXIST: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(9i32);
pub const FDE_PERSISTENCE_FAILED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(4i32);
pub const FDE_UNSUPPORTED_AUTH: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(7i32);
pub const FDE_UNSUPPORTED_DTD: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(11i32);
pub const FDE_UNSUPPORTED_MSXML: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(10i32);
pub const FDS_DOWNLOADED: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(3i32);
pub const FDS_DOWNLOADING: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(2i32);
pub const FDS_DOWNLOAD_FAILED: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(4i32);
pub const FDS_NONE: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(0i32);
pub const FDS_PENDING: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(1i32);
pub const FEC_E_DOWNLOADSIZELIMITEXCEEDED: FEEDS_ERROR_CODE = FEEDS_ERROR_CODE(-1073479167i32);
pub const FEC_E_ERRORBASE: FEEDS_ERROR_CODE = FEEDS_ERROR_CODE(-1073479168i32);
pub const FEC_E_INVALIDMSXMLPROPERTY: FEEDS_ERROR_CODE = FEEDS_ERROR_CODE(-1073479168i32);
pub const FEICF_READ_ITEM_COUNT_CHANGED: FEEDS_EVENTS_ITEM_COUNT_FLAGS = FEEDS_EVENTS_ITEM_COUNT_FLAGS(1i32);
pub const FEICF_UNREAD_ITEM_COUNT_CHANGED: FEEDS_EVENTS_ITEM_COUNT_FLAGS = FEEDS_EVENTS_ITEM_COUNT_FLAGS(2i32);
pub const FEM_FEEDEVENTS: FEEDS_EVENTS_MASK = FEEDS_EVENTS_MASK(2i32);
pub const FEM_FOLDEREVENTS: FEEDS_EVENTS_MASK = FEEDS_EVENTS_MASK(1i32);
pub const FES_ALL: FEEDS_EVENTS_SCOPE = FEEDS_EVENTS_SCOPE(0i32);
pub const FES_SELF_AND_CHILDREN_ONLY: FEEDS_EVENTS_SCOPE = FEEDS_EVENTS_SCOPE(2i32);
pub const FES_SELF_ONLY: FEEDS_EVENTS_SCOPE = FEEDS_EVENTS_SCOPE(1i32);
pub const FSS_DEFAULT: FEEDS_SYNC_SETTING = FEEDS_SYNC_SETTING(0i32);
pub const FSS_INTERVAL: FEEDS_SYNC_SETTING = FEEDS_SYNC_SETTING(1i32);
pub const FSS_MANUAL: FEEDS_SYNC_SETTING = FEEDS_SYNC_SETTING(2i32);
pub const FSS_SUGGESTED: FEEDS_SYNC_SETTING = FEEDS_SYNC_SETTING(3i32);
pub const FXFF_ALL: FEEDS_XML_FILTER_FLAGS = FEEDS_XML_FILTER_FLAGS(0i32);
pub const FXFF_READ: FEEDS_XML_FILTER_FLAGS = FEEDS_XML_FILTER_FLAGS(2i32);
pub const FXFF_UNREAD: FEEDS_XML_FILTER_FLAGS = FEEDS_XML_FILTER_FLAGS(1i32);
pub const FXIF_CF_EXTENSIONS: FEEDS_XML_INCLUDE_FLAGS = FEEDS_XML_INCLUDE_FLAGS(1i32);
pub const FXIF_NONE: FEEDS_XML_INCLUDE_FLAGS = FEEDS_XML_INCLUDE_FLAGS(0i32);
pub const FXSO_ASCENDING: FEEDS_XML_SORT_ORDER = FEEDS_XML_SORT_ORDER(1i32);
pub const FXSO_DESCENDING: FEEDS_XML_SORT_ORDER = FEEDS_XML_SORT_ORDER(2i32);
pub const FXSO_NONE: FEEDS_XML_SORT_ORDER = FEEDS_XML_SORT_ORDER(0i32);
pub const FXSP_DOWNLOADTIME: FEEDS_XML_SORT_PROPERTY = FEEDS_XML_SORT_PROPERTY(2i32);
pub const FXSP_NONE: FEEDS_XML_SORT_PROPERTY = FEEDS_XML_SORT_PROPERTY(0i32);
pub const FXSP_PUBDATE: FEEDS_XML_SORT_PROPERTY = FEEDS_XML_SORT_PROPERTY(1i32);
pub const FeedFolderWatcher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x281001ed_7765_4cb0_84af_e9b387af01ff);
pub const FeedWatcher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18a6737b_f433_4687_89bc_a1b4dfb9f123);
pub const FeedsManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfaeb54c4_f66f_4806_83a0_805299f5e3ad);
pub const IOCTL_WMP_DEVICE_CAN_SYNC: u32 = 844123479u32;
pub const IOCTL_WMP_METADATA_ROUND_TRIP: u32 = 827346263u32;
pub const PLUGIN_ALL_MEDIASENDTO: ::windows_core::PCWSTR = ::windows_core::w!("MediaSendTo");
pub const PLUGIN_ALL_PLAYLISTSENDTO: ::windows_core::PCWSTR = ::windows_core::w!("PlaylistSendTo");
pub const PLUGIN_FLAGS_ACCEPTSMEDIA: u32 = 268435456u32;
pub const PLUGIN_FLAGS_ACCEPTSPLAYLISTS: u32 = 134217728u32;
pub const PLUGIN_FLAGS_HASPRESETS: u32 = 67108864u32;
pub const PLUGIN_FLAGS_HASPROPERTYPAGE: u32 = 2147483648u32;
pub const PLUGIN_FLAGS_HIDDEN: u32 = 33554432u32;
pub const PLUGIN_FLAGS_INSTALLAUTORUN: u32 = 1073741824u32;
pub const PLUGIN_FLAGS_LAUNCHPROPERTYPAGE: u32 = 536870912u32;
pub const PLUGIN_INSTALLREGKEY: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\MediaPlayer\\UIPlugins");
pub const PLUGIN_INSTALLREGKEY_CAPABILITIES: ::windows_core::PCWSTR = ::windows_core::w!("Capabilities");
pub const PLUGIN_INSTALLREGKEY_DESCRIPTION: ::windows_core::PCWSTR = ::windows_core::w!("Description");
pub const PLUGIN_INSTALLREGKEY_FRIENDLYNAME: ::windows_core::PCWSTR = ::windows_core::w!("FriendlyName");
pub const PLUGIN_INSTALLREGKEY_UNINSTALL: ::windows_core::PCWSTR = ::windows_core::w!("UninstallPath");
pub const PLUGIN_MISC_CURRENTPRESET: ::windows_core::PCWSTR = ::windows_core::w!("CurrentPreset");
pub const PLUGIN_MISC_PRESETCOUNT: ::windows_core::PCWSTR = ::windows_core::w!("PresetCount");
pub const PLUGIN_MISC_PRESETNAMES: ::windows_core::PCWSTR = ::windows_core::w!("PresetNames");
pub const PLUGIN_MISC_QUERYDESTROY: ::windows_core::PCWSTR = ::windows_core::w!("QueryDestroy");
pub const PLUGIN_SEPARATEWINDOW_DEFAULTHEIGHT: ::windows_core::PCWSTR = ::windows_core::w!("DefaultHeight");
pub const PLUGIN_SEPARATEWINDOW_DEFAULTWIDTH: ::windows_core::PCWSTR = ::windows_core::w!("DefaultWidth");
pub const PLUGIN_SEPARATEWINDOW_MAXHEIGHT: ::windows_core::PCWSTR = ::windows_core::w!("MaxHeight");
pub const PLUGIN_SEPARATEWINDOW_MAXWIDTH: ::windows_core::PCWSTR = ::windows_core::w!("MaxWidth");
pub const PLUGIN_SEPARATEWINDOW_MINHEIGHT: ::windows_core::PCWSTR = ::windows_core::w!("MinHeight");
pub const PLUGIN_SEPARATEWINDOW_MINWIDTH: ::windows_core::PCWSTR = ::windows_core::w!("MinWidth");
pub const PLUGIN_SEPARATEWINDOW_RESIZABLE: ::windows_core::PCWSTR = ::windows_core::w!("Resizable");
pub const PLUGIN_TYPE_BACKGROUND: u32 = 1u32;
pub const PLUGIN_TYPE_DISPLAYAREA: u32 = 3u32;
pub const PLUGIN_TYPE_METADATAAREA: u32 = 5u32;
pub const PLUGIN_TYPE_SEPARATEWINDOW: u32 = 2u32;
pub const PLUGIN_TYPE_SETTINGSAREA: u32 = 4u32;
pub const SA_BUFFER_SIZE: u32 = 1024u32;
pub const SUBSCRIPTION_CAP_ALLOWCDBURN: u32 = 2u32;
pub const SUBSCRIPTION_CAP_ALLOWPDATRANSFER: u32 = 4u32;
pub const SUBSCRIPTION_CAP_ALLOWPLAY: u32 = 1u32;
pub const SUBSCRIPTION_CAP_ALTLOGIN: u32 = 128u32;
pub const SUBSCRIPTION_CAP_BACKGROUNDPROCESSING: u32 = 8u32;
pub const SUBSCRIPTION_CAP_DEVICEAVAILABLE: u32 = 16u32;
pub const SUBSCRIPTION_CAP_IS_CONTENTPARTNER: u32 = 64u32;
pub const SUBSCRIPTION_CAP_PREPAREFORSYNC: u32 = 32u32;
pub const SUBSCRIPTION_CAP_UILESSMODE_ALLOWPLAY: u32 = 256u32;
pub const SUBSCRIPTION_V1_CAPS: u32 = 15u32;
pub const WMPCOREEVENT_BASE: u32 = 5000u32;
pub const WMPCOREEVENT_CDROM_BASE: u32 = 5700u32;
pub const WMPCOREEVENT_CONTENT_BASE: u32 = 5300u32;
pub const WMPCOREEVENT_CONTROL_BASE: u32 = 5100u32;
pub const WMPCOREEVENT_ERROR_BASE: u32 = 5500u32;
pub const WMPCOREEVENT_NETWORK_BASE: u32 = 5400u32;
pub const WMPCOREEVENT_PLAYLIST_BASE: u32 = 5800u32;
pub const WMPCOREEVENT_SEEK_BASE: u32 = 5200u32;
pub const WMPCOREEVENT_WARNING_BASE: u32 = 5600u32;
pub const WMPGC_FLAGS_ALLOW_PREROLL: u32 = 1u32;
pub const WMPGC_FLAGS_DISABLE_PLUGINS: u32 = 8u32;
pub const WMPGC_FLAGS_IGNORE_AV_SYNC: u32 = 4u32;
pub const WMPGC_FLAGS_SUPPRESS_DIALOGS: u32 = 2u32;
pub const WMPGC_FLAGS_USE_CUSTOM_GRAPH: u32 = 16u32;
pub const WMPLib: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bf52a50_394a_11d3_b153_00c04f79faa6);
pub const WMPOCXEVENT_BASE: u32 = 6500u32;
pub const WMPPlugin_Caps_CannotConvertFormats: WMPPlugin_Caps = WMPPlugin_Caps(1i32);
pub const WMPRemoteMediaServices: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf333473_2cf7_4be2_907f_9aad5661364f);
pub const WMPServices_StreamState_Pause: WMPServices_StreamState = WMPServices_StreamState(1i32);
pub const WMPServices_StreamState_Play: WMPServices_StreamState = WMPServices_StreamState(2i32);
pub const WMPServices_StreamState_Stop: WMPServices_StreamState = WMPServices_StreamState(0i32);
pub const WMPUE_EC_USER: u32 = 33024u32;
pub const WMP_MDRT_FLAGS_UNREPORTED_ADDED_ITEMS: u32 = 2u32;
pub const WMP_MDRT_FLAGS_UNREPORTED_DELETED_ITEMS: u32 = 1u32;
pub const WMP_PLUGINTYPE_DSP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6434baea_4954_498d_abd5_2b07123e1f04);
pub const WMP_PLUGINTYPE_DSP_OUTOFPROC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef29b174_c347_44cc_9a4f_2399118ff38c);
pub const WMP_PLUGINTYPE_RENDERING: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8554541_115d_406a_a4c7_51111c330183);
pub const WMP_SUBSCR_DL_TYPE_BACKGROUND: ::windows_core::PCWSTR = ::windows_core::w!("background");
pub const WMP_SUBSCR_DL_TYPE_REALTIME: ::windows_core::PCWSTR = ::windows_core::w!("real time");
pub const WMProfile_V40_100Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f99ddd8_6684_456b_a0a3_33e1316895f0);
pub const WMProfile_V40_128Audio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93ddbe12_13dc_4e32_a35e_40378e34279a);
pub const WMProfile_V40_16AMRadio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f4be81f_d57d_41e1_b2e3_2fad986bfec2);
pub const WMProfile_V40_1MBVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4482a4c_cc17_4b07_a94e_9818d5e0f13f);
pub const WMProfile_V40_250Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x541841c3_9339_4f7b_9a22_b11540894e42);
pub const WMProfile_V40_2856100MBR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a1c2206_dc5e_4186_beb2_4c5a994b132e);
pub const WMProfile_V40_288FMRadioMono: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fa57fc8_6ea4_4645_8abf_b6e5a8f814a1);
pub const WMProfile_V40_288FMRadioStereo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22fcf466_aa40_431f_a289_06d0ea1a1e40);
pub const WMProfile_V40_288VideoAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac617f2d_6cbe_4e84_8e9a_ce151a12a354);
pub const WMProfile_V40_288VideoVoice: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb2bc274_0eb6_4da9_b550_ecf7f2b9948f);
pub const WMProfile_V40_288VideoWebServer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xabf2f00d_d555_4815_94ce_8275f3a70bfe);
pub const WMProfile_V40_3MBVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55374ac0_309b_4396_b88f_e6e292113f28);
pub const WMProfile_V40_512Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70440e6d_c4ef_4f84_8cd0_d5c28686e784);
pub const WMProfile_V40_56DialUpStereo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8026f87_e905_4594_a3c7_00d00041d1d9);
pub const WMProfile_V40_56DialUpVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe21713bb_652f_4dab_99de_71e04400270f);
pub const WMProfile_V40_56DialUpVideoWebServer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb756ff10_520f_4749_a399_b780e2fc9250);
pub const WMProfile_V40_64Audio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4820b3f7_cbec_41dc_9391_78598714c8e5);
pub const WMProfile_V40_6VoiceAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd508978a_11a0_4d15_b0da_acdc99d4f890);
pub const WMProfile_V40_96Audio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0efa0ee3_9e64_41e2_837f_3c0038f327ba);
pub const WMProfile_V40_DialUpMBR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd7f47f1_72a6_45a4_80f0_3aecefc32c07);
pub const WMProfile_V40_IntranetMBR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82cd3321_a94a_4ffc_9c2b_092c10ca16e7);
pub const WMProfile_V70_100Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9f3c932_5ea9_4c6d_89b4_2686e515426e);
pub const WMProfile_V70_128Audio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc64cf5da_df45_40d3_8027_de698d68dc66);
pub const WMProfile_V70_1500FilmContentVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6a5f6df_ee3f_434c_a433_523ce55f516b);
pub const WMProfile_V70_1500Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b89164a_5490_4686_9e37_5a80884e5146);
pub const WMProfile_V70_150VideoPDA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f472967_e3c6_4797_9694_f0304c5e2f17);
pub const WMProfile_V70_2000Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa980124_bf10_4e4f_9afd_4329a7395cff);
pub const WMProfile_V70_225VideoPDA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf55ea573_4c02_42b5_9026_a8260c438a9f);
pub const WMProfile_V70_256Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafe69b3a_403f_4a1b_8007_0e21cfb3df84);
pub const WMProfile_V70_2856100MBR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07df7a25_3fe2_4a5b_8b1e_348b0721ca70);
pub const WMProfile_V70_288FMRadioMono: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc012a833_a03b_44a5_96dc_ed95cc65582d);
pub const WMProfile_V70_288FMRadioStereo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe96d67c9_1a39_4dc4_b900_b1184dc83620);
pub const WMProfile_V70_288VideoAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58bba0ee_896a_4948_9953_85b736f83947);
pub const WMProfile_V70_288VideoVoice: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb952f38e_7dbc_4533_a9ca_b00b1c6e9800);
pub const WMProfile_V70_288VideoWebServer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70a32e2b_e2df_4ebd_9105_d9ca194a2d50);
pub const WMProfile_V70_384Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3d45fbb_8782_44df_97c6_8678e2f9b13d);
pub const WMProfile_V70_56DialUpStereo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x674ee767_0949_4fac_875e_f4c9c292013b);
pub const WMProfile_V70_56VideoWebServer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdef99e40_57bc_4ab3_b2d1_b6e3caf64257);
pub const WMProfile_V70_64Audio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb29cffc6_f131_41db_b5e8_99d8b0b945f4);
pub const WMProfile_V70_64AudioISDN: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91dea458_9d60_4212_9c59_d40919c939e4);
pub const WMProfile_V70_64VideoISDN: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2b7a7e9_7b8e_4992_a1a1_068217a3b311);
pub const WMProfile_V70_6VoiceAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeaba9fbf_b64f_49b3_aa0c_73fbdd150ad0);
pub const WMProfile_V70_700FilmContentVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a747920_2449_4d76_99cb_fdb0c90484d4);
pub const WMProfile_V70_768Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0326ebb6_f76e_4964_b0db_e729978d35ee);
pub const WMProfile_V70_96Audio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9d4b819_16cc_4a59_9f37_693dbb0302d6);
pub const WMProfile_V70_DialUpMBR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b16e74b_4068_45b5_b80e_7bf8c80d2c2f);
pub const WMProfile_V70_IntranetMBR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x045880dc_34b6_4ca9_a326_73557ed143f3);
pub const WMProfile_V80_100768VideoMBR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bdb5a0e_979e_47d3_9596_73b386392a55);
pub const WMProfile_V80_100Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2e300b4_c2d4_4fc0_b5dd_ecbd948dc0df);
pub const WMProfile_V80_128StereoAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x407b9450_8bdc_4ee5_88b8_6f527bd941f2);
pub const WMProfile_V80_1400NTSCVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x931d1bee_617a_4bcd_9905_ccd0786683ee);
pub const WMProfile_V80_150VideoPDA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaee16dfa_2c14_4a2f_ad3f_a3034031784f);
pub const WMProfile_V80_255VideoPDA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfeedbcdf_3fac_4c93_ac0d_47941ec72c0b);
pub const WMProfile_V80_256Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbc75500_33d2_4466_b86b_122b201cc9ae);
pub const WMProfile_V80_288100VideoMBR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8722c69_2419_4b36_b4e0_6e17b60564e5);
pub const WMProfile_V80_28856VideoMBR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd66920c4_c21f_4ec8_a0b4_95cf2bd57fc4);
pub const WMProfile_V80_288MonoAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ea3126d_e1ba_4716_89af_f65cee0c0c67);
pub const WMProfile_V80_288StereoAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e4cab5c_35dc_45bb_a7c0_19b28070d0cc);
pub const WMProfile_V80_288Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3df678d9_1352_4186_bbf8_74f0c19b6ae2);
pub const WMProfile_V80_288VideoOnly: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c45b4c7_4aeb_4f78_a5ec_88420b9dadef);
pub const WMProfile_V80_32StereoAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60907f9f_b352_47e5_b210_0ef1f47e9f9d);
pub const WMProfile_V80_384PALVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9227c692_ae62_4f72_a7ea_736062d0e21e);
pub const WMProfile_V80_384Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29b00c2b_09a9_48bd_ad09_cdae117d1da7);
pub const WMProfile_V80_48StereoAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ee06be5_492b_480a_8a8f_12f373ecf9d4);
pub const WMProfile_V80_56Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x254e8a96_2612_405c_8039_f0bf725ced7d);
pub const WMProfile_V80_56VideoOnly: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e2a6955_81df_4943_ba50_68a986a708f6);
pub const WMProfile_V80_64StereoAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09bb5bc4_3176_457f_8dd6_3cd919123e2d);
pub const WMProfile_V80_700NTSCVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8c2985f_e5d9_4538_9e23_9b21bf78f745);
pub const WMProfile_V80_700PALVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec298949_639b_45e2_96fd_4ab32d5919c2);
pub const WMProfile_V80_768Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74d01102_e71a_4820_8f0d_13d2ec1e4872);
pub const WMProfile_V80_96StereoAudio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1fc81930_61f2_436f_9d33_349f2a1c0f10);
pub const WMProfile_V80_BESTVBRVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x048439ba_309c_440e_9cb4_3dcca3756423);
pub const WMProfile_V80_FAIRVBRVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3510a862_5850_4886_835f_d78ec6a64042);
pub const WMProfile_V80_HIGHVBRVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f10d9d3_3b04_4fb0_a3d3_88d4ac854acc);
pub const WindowsMediaPlayer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bf52a52_394a_11d3_b153_00c04f79faa6);
pub const g_szAllAuthors: ::windows_core::PCWSTR = ::windows_core::w!("AllAuthors");
pub const g_szAllCPAlbumIDs: ::windows_core::PCWSTR = ::windows_core::w!("AllCPAlbumIDs");
pub const g_szAllCPAlbumSubGenreIDs: ::windows_core::PCWSTR = ::windows_core::w!("AllCPAlbumSubGenreIDs");
pub const g_szAllCPArtistIDs: ::windows_core::PCWSTR = ::windows_core::w!("AllCPArtistIDs");
pub const g_szAllCPGenreIDs: ::windows_core::PCWSTR = ::windows_core::w!("AllCPGenreIDs");
pub const g_szAllCPListIDs: ::windows_core::PCWSTR = ::windows_core::w!("AllCPListIDs");
pub const g_szAllCPRadioIDs: ::windows_core::PCWSTR = ::windows_core::w!("AllCPRadioIDs");
pub const g_szAllCPTrackIDs: ::windows_core::PCWSTR = ::windows_core::w!("AllCPTrackIDs");
pub const g_szAllReleaseDateYears: ::windows_core::PCWSTR = ::windows_core::w!("AllReleaseDateYears");
pub const g_szAllUserEffectiveRatingStarss: ::windows_core::PCWSTR = ::windows_core::w!("AllUserEffectiveRatingStarss");
pub const g_szAllWMParentalRatings: ::windows_core::PCWSTR = ::windows_core::w!("AllWMParentalRatings");
pub const g_szAuthor: ::windows_core::PCWSTR = ::windows_core::w!("Author");
pub const g_szCPAlbumID: ::windows_core::PCWSTR = ::windows_core::w!("CPAlbumID");
pub const g_szCPAlbumSubGenreID: ::windows_core::PCWSTR = ::windows_core::w!("CPAlbumSubGenreID");
pub const g_szCPArtistID: ::windows_core::PCWSTR = ::windows_core::w!("CPArtistID");
pub const g_szCPGenreID: ::windows_core::PCWSTR = ::windows_core::w!("CPGenreID");
pub const g_szCPListID: ::windows_core::PCWSTR = ::windows_core::w!("CPListID");
pub const g_szCPRadioID: ::windows_core::PCWSTR = ::windows_core::w!("CPRadioID");
pub const g_szCPTrackID: ::windows_core::PCWSTR = ::windows_core::w!("CPTrackID");
pub const g_szContentPartnerInfo_AccountBalance: ::windows_core::PCWSTR = ::windows_core::w!("AccountBalance");
pub const g_szContentPartnerInfo_AccountType: ::windows_core::PCWSTR = ::windows_core::w!("AccountType");
pub const g_szContentPartnerInfo_HasCachedCredentials: ::windows_core::PCWSTR = ::windows_core::w!("HasCachedCredentials");
pub const g_szContentPartnerInfo_LicenseRefreshAdvanceWarning: ::windows_core::PCWSTR = ::windows_core::w!("LicenseRefreshAdvanceWarning");
pub const g_szContentPartnerInfo_LoginState: ::windows_core::PCWSTR = ::windows_core::w!("LoginState");
pub const g_szContentPartnerInfo_MaximumTrackPurchasePerPurchase: ::windows_core::PCWSTR = ::windows_core::w!("MaximumNumberOfTracksPerPurchase");
pub const g_szContentPartnerInfo_MediaPlayerAccountType: ::windows_core::PCWSTR = ::windows_core::w!("MediaPlayerAccountType");
pub const g_szContentPartnerInfo_PurchasedTrackRequiresReDownload: ::windows_core::PCWSTR = ::windows_core::w!("PurchasedTrackRequiresReDownload");
pub const g_szContentPartnerInfo_UserName: ::windows_core::PCWSTR = ::windows_core::w!("UserName");
pub const g_szContentPrice_CannotBuy: ::windows_core::PCWSTR = ::windows_core::w!("PriceCannotBuy");
pub const g_szContentPrice_Free: ::windows_core::PCWSTR = ::windows_core::w!("PriceFree");
pub const g_szContentPrice_Unknown: ::windows_core::PCWSTR = ::windows_core::w!("PriceUnknown");
pub const g_szFlyoutMenu: ::windows_core::PCWSTR = ::windows_core::w!("FlyoutMenu");
pub const g_szItemInfo_ALTLoginCaption: ::windows_core::PCWSTR = ::windows_core::w!("ALTLoginCaption");
pub const g_szItemInfo_ALTLoginURL: ::windows_core::PCWSTR = ::windows_core::w!("ALTLoginURL");
pub const g_szItemInfo_AlbumArtURL: ::windows_core::PCWSTR = ::windows_core::w!("AlbumArt");
pub const g_szItemInfo_ArtistArtURL: ::windows_core::PCWSTR = ::windows_core::w!("ArtistArt");
pub const g_szItemInfo_AuthenticationSuccessURL: ::windows_core::PCWSTR = ::windows_core::w!("AuthenticationSuccessURL");
pub const g_szItemInfo_CreateAccountURL: ::windows_core::PCWSTR = ::windows_core::w!("CreateAccount");
pub const g_szItemInfo_ErrorDescription: ::windows_core::PCWSTR = ::windows_core::w!("CPErrorDescription");
pub const g_szItemInfo_ErrorURL: ::windows_core::PCWSTR = ::windows_core::w!("CPErrorURL");
pub const g_szItemInfo_ErrorURLLinkText: ::windows_core::PCWSTR = ::windows_core::w!("CPErrorURLLinkText");
pub const g_szItemInfo_ForgetPasswordURL: ::windows_core::PCWSTR = ::windows_core::w!("ForgotPassword");
pub const g_szItemInfo_GenreArtURL: ::windows_core::PCWSTR = ::windows_core::w!("GenreArt");
pub const g_szItemInfo_HTMLViewURL: ::windows_core::PCWSTR = ::windows_core::w!("HTMLViewURL");
pub const g_szItemInfo_ListArtURL: ::windows_core::PCWSTR = ::windows_core::w!("ListArt");
pub const g_szItemInfo_LoginFailureURL: ::windows_core::PCWSTR = ::windows_core::w!("LoginFailureURL");
pub const g_szItemInfo_PopupCaption: ::windows_core::PCWSTR = ::windows_core::w!("PopupCaption");
pub const g_szItemInfo_PopupURL: ::windows_core::PCWSTR = ::windows_core::w!("Popup");
pub const g_szItemInfo_RadioArtURL: ::windows_core::PCWSTR = ::windows_core::w!("RadioArt");
pub const g_szItemInfo_SubGenreArtURL: ::windows_core::PCWSTR = ::windows_core::w!("SubGenreArt");
pub const g_szItemInfo_TreeListIconURL: ::windows_core::PCWSTR = ::windows_core::w!("CPListIDIcon");
pub const g_szMediaPlayerTask_Browse: ::windows_core::PCWSTR = ::windows_core::w!("Browse");
pub const g_szMediaPlayerTask_Burn: ::windows_core::PCWSTR = ::windows_core::w!("Burn");
pub const g_szMediaPlayerTask_Sync: ::windows_core::PCWSTR = ::windows_core::w!("Sync");
pub const g_szOnlineStore: ::windows_core::PCWSTR = ::windows_core::w!("OnlineStore");
pub const g_szRefreshLicenseBurn: ::windows_core::PCWSTR = ::windows_core::w!("RefreshForBurn");
pub const g_szRefreshLicensePlay: ::windows_core::PCWSTR = ::windows_core::w!("RefreshForPlay");
pub const g_szRefreshLicenseSync: ::windows_core::PCWSTR = ::windows_core::w!("RefreshForSync");
pub const g_szReleaseDateYear: ::windows_core::PCWSTR = ::windows_core::w!("ReleaseDateYear");
pub const g_szRootLocation: ::windows_core::PCWSTR = ::windows_core::w!("RootLocation");
pub const g_szStationEvent_Complete: ::windows_core::PCWSTR = ::windows_core::w!("TrackComplete");
pub const g_szStationEvent_Skipped: ::windows_core::PCWSTR = ::windows_core::w!("TrackSkipped");
pub const g_szStationEvent_Started: ::windows_core::PCWSTR = ::windows_core::w!("TrackStarted");
pub const g_szUnknownLocation: ::windows_core::PCWSTR = ::windows_core::w!("UnknownLocation");
pub const g_szUserEffectiveRatingStars: ::windows_core::PCWSTR = ::windows_core::w!("UserEffectiveRatingStars");
pub const g_szUserPlaylist: ::windows_core::PCWSTR = ::windows_core::w!("UserPlaylist");
pub const g_szVerifyPermissionSync: ::windows_core::PCWSTR = ::windows_core::w!("VerifyPermissionSync");
pub const g_szVideoRecent: ::windows_core::PCWSTR = ::windows_core::w!("VideoRecent");
pub const g_szVideoRoot: ::windows_core::PCWSTR = ::windows_core::w!("VideoRoot");
pub const g_szViewMode_Details: ::windows_core::PCWSTR = ::windows_core::w!("ViewModeDetails");
pub const g_szViewMode_Icon: ::windows_core::PCWSTR = ::windows_core::w!("ViewModeIcon");
pub const g_szViewMode_OrderedList: ::windows_core::PCWSTR = ::windows_core::w!("ViewModeOrderedList");
pub const g_szViewMode_Report: ::windows_core::PCWSTR = ::windows_core::w!("ViewModeReport");
pub const g_szViewMode_Tile: ::windows_core::PCWSTR = ::windows_core::w!("ViewModeTile");
pub const g_szWMParentalRating: ::windows_core::PCWSTR = ::windows_core::w!("WMParentalRating");
pub const kfltTimedLevelMaximumFrequency: f32 = 22050f32;
pub const kfltTimedLevelMinimumFrequency: f32 = 20f32;
pub const pause_state: PlayerState = PlayerState(1i32);
pub const play_state: PlayerState = PlayerState(2i32);
pub const stop_state: PlayerState = PlayerState(0i32);
pub const wmpatBuyOnly: WMPAccountType = WMPAccountType(1i32);
pub const wmpatJanus: WMPAccountType = WMPAccountType(3i32);
pub const wmpatSubscription: WMPAccountType = WMPAccountType(2i32);
pub const wmpbfAudioCD: WMPBurnFormat = WMPBurnFormat(0i32);
pub const wmpbfDataCD: WMPBurnFormat = WMPBurnFormat(1i32);
pub const wmpbsBurning: WMPBurnState = WMPBurnState(6i32);
pub const wmpbsBusy: WMPBurnState = WMPBurnState(1i32);
pub const wmpbsDownloading: WMPBurnState = WMPBurnState(9i32);
pub const wmpbsErasing: WMPBurnState = WMPBurnState(8i32);
pub const wmpbsPreparingToBurn: WMPBurnState = WMPBurnState(5i32);
pub const wmpbsReady: WMPBurnState = WMPBurnState(2i32);
pub const wmpbsRefreshStatusPending: WMPBurnState = WMPBurnState(4i32);
pub const wmpbsStopped: WMPBurnState = WMPBurnState(7i32);
pub const wmpbsUnknown: WMPBurnState = WMPBurnState(0i32);
pub const wmpbsWaitingForDisc: WMPBurnState = WMPBurnState(3i32);
pub const wmpcnAuthResult: WMPCallbackNotification = WMPCallbackNotification(2i32);
pub const wmpcnDisableRadioSkipping: WMPCallbackNotification = WMPCallbackNotification(6i32);
pub const wmpcnLicenseUpdated: WMPCallbackNotification = WMPCallbackNotification(3i32);
pub const wmpcnLoginStateChange: WMPCallbackNotification = WMPCallbackNotification(1i32);
pub const wmpcnNewCatalogAvailable: WMPCallbackNotification = WMPCallbackNotification(4i32);
pub const wmpcnNewPluginAvailable: WMPCallbackNotification = WMPCallbackNotification(5i32);
pub const wmpdsLast: WMPDeviceStatus = WMPDeviceStatus(6i32);
pub const wmpdsManualDevice: WMPDeviceStatus = WMPDeviceStatus(4i32);
pub const wmpdsNewDevice: WMPDeviceStatus = WMPDeviceStatus(5i32);
pub const wmpdsPartnershipAnother: WMPDeviceStatus = WMPDeviceStatus(3i32);
pub const wmpdsPartnershipDeclined: WMPDeviceStatus = WMPDeviceStatus(2i32);
pub const wmpdsPartnershipExists: WMPDeviceStatus = WMPDeviceStatus(1i32);
pub const wmpdsUnknown: WMPDeviceStatus = WMPDeviceStatus(0i32);
pub const wmpfssScanning: WMPFolderScanState = WMPFolderScanState(1i32);
pub const wmpfssStopped: WMPFolderScanState = WMPFolderScanState(3i32);
pub const wmpfssUnknown: WMPFolderScanState = WMPFolderScanState(0i32);
pub const wmpfssUpdating: WMPFolderScanState = WMPFolderScanState(2i32);
pub const wmplcAppend: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(6i32);
pub const wmplcClear: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(1i32);
pub const wmplcDelete: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(4i32);
pub const wmplcInfoChange: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(2i32);
pub const wmplcInsert: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(5i32);
pub const wmplcLast: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(11i32);
pub const wmplcMorph: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(9i32);
pub const wmplcMove: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(3i32);
pub const wmplcNameChange: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(8i32);
pub const wmplcPrivate: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(7i32);
pub const wmplcSort: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(10i32);
pub const wmplcUnknown: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(0i32);
pub const wmpltAll: WMPLibraryType = WMPLibraryType(1i32);
pub const wmpltDisc: WMPLibraryType = WMPLibraryType(4i32);
pub const wmpltLocal: WMPLibraryType = WMPLibraryType(2i32);
pub const wmpltPortableDevice: WMPLibraryType = WMPLibraryType(5i32);
pub const wmpltRemote: WMPLibraryType = WMPLibraryType(3i32);
pub const wmpltUnknown: WMPLibraryType = WMPLibraryType(0i32);
pub const wmposBeginCodecAcquisition: WMPOpenState = WMPOpenState(14i32);
pub const wmposBeginIndividualization: WMPOpenState = WMPOpenState(18i32);
pub const wmposBeginLicenseAcquisition: WMPOpenState = WMPOpenState(16i32);
pub const wmposEndCodecAcquisition: WMPOpenState = WMPOpenState(15i32);
pub const wmposEndIndividualization: WMPOpenState = WMPOpenState(19i32);
pub const wmposEndLicenseAcquisition: WMPOpenState = WMPOpenState(17i32);
pub const wmposMediaChanging: WMPOpenState = WMPOpenState(8i32);
pub const wmposMediaConnecting: WMPOpenState = WMPOpenState(10i32);
pub const wmposMediaLoading: WMPOpenState = WMPOpenState(11i32);
pub const wmposMediaLocating: WMPOpenState = WMPOpenState(9i32);
pub const wmposMediaOpen: WMPOpenState = WMPOpenState(13i32);
pub const wmposMediaOpening: WMPOpenState = WMPOpenState(12i32);
pub const wmposMediaWaiting: WMPOpenState = WMPOpenState(20i32);
pub const wmposOpeningUnknownURL: WMPOpenState = WMPOpenState(21i32);
pub const wmposPlaylistChanged: WMPOpenState = WMPOpenState(7i32);
pub const wmposPlaylistChanging: WMPOpenState = WMPOpenState(1i32);
pub const wmposPlaylistConnecting: WMPOpenState = WMPOpenState(3i32);
pub const wmposPlaylistLoading: WMPOpenState = WMPOpenState(4i32);
pub const wmposPlaylistLocating: WMPOpenState = WMPOpenState(2i32);
pub const wmposPlaylistOpenNoMedia: WMPOpenState = WMPOpenState(6i32);
pub const wmposPlaylistOpening: WMPOpenState = WMPOpenState(5i32);
pub const wmposUndefined: WMPOpenState = WMPOpenState(0i32);
pub const wmppsBuffering: WMPPlayState = WMPPlayState(6i32);
pub const wmppsLast: WMPPlayState = WMPPlayState(12i32);
pub const wmppsMediaEnded: WMPPlayState = WMPPlayState(8i32);
pub const wmppsPaused: WMPPlayState = WMPPlayState(2i32);
pub const wmppsPlaying: WMPPlayState = WMPPlayState(3i32);
pub const wmppsReady: WMPPlayState = WMPPlayState(10i32);
pub const wmppsReconnecting: WMPPlayState = WMPPlayState(11i32);
pub const wmppsScanForward: WMPPlayState = WMPPlayState(4i32);
pub const wmppsScanReverse: WMPPlayState = WMPPlayState(5i32);
pub const wmppsStopped: WMPPlayState = WMPPlayState(1i32);
pub const wmppsTransitioning: WMPPlayState = WMPPlayState(9i32);
pub const wmppsUndefined: WMPPlayState = WMPPlayState(0i32);
pub const wmppsWaiting: WMPPlayState = WMPPlayState(7i32);
pub const wmprsRipping: WMPRipState = WMPRipState(1i32);
pub const wmprsStopped: WMPRipState = WMPRipState(2i32);
pub const wmprsUnknown: WMPRipState = WMPRipState(0i32);
pub const wmpsccetBeginUpdates: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(5i32);
pub const wmpsccetChange: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(2i32);
pub const wmpsccetClear: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(4i32);
pub const wmpsccetDelete: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(3i32);
pub const wmpsccetEndUpdates: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(6i32);
pub const wmpsccetInsert: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(1i32);
pub const wmpsccetUnknown: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(0i32);
pub const wmpsdlsCancelled: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(4i32);
pub const wmpsdlsCompleted: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(3i32);
pub const wmpsdlsDownloading: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(0i32);
pub const wmpsdlsPaused: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(1i32);
pub const wmpsdlsProcessing: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(2i32);
pub const wmpsnBackgroundProcessingBegin: WMPPartnerNotification = WMPPartnerNotification(1i32);
pub const wmpsnBackgroundProcessingEnd: WMPPartnerNotification = WMPPartnerNotification(2i32);
pub const wmpsnCatalogDownloadComplete: WMPPartnerNotification = WMPPartnerNotification(4i32);
pub const wmpsnCatalogDownloadFailure: WMPPartnerNotification = WMPPartnerNotification(3i32);
pub const wmpssEstimating: WMPSyncState = WMPSyncState(3i32);
pub const wmpssLast: WMPSyncState = WMPSyncState(4i32);
pub const wmpssStopped: WMPSyncState = WMPSyncState(2i32);
pub const wmpssSynchronizing: WMPSyncState = WMPSyncState(1i32);
pub const wmpssUnknown: WMPSyncState = WMPSyncState(0i32);
pub const wmpsseCurrentBegin: WMPSubscriptionServiceEvent = WMPSubscriptionServiceEvent(1i32);
pub const wmpsseCurrentEnd: WMPSubscriptionServiceEvent = WMPSubscriptionServiceEvent(2i32);
pub const wmpsseFullBegin: WMPSubscriptionServiceEvent = WMPSubscriptionServiceEvent(3i32);
pub const wmpsseFullEnd: WMPSubscriptionServiceEvent = WMPSubscriptionServiceEvent(4i32);
pub const wmpstMusic: WMPStreamingType = WMPStreamingType(1i32);
pub const wmpstRadio: WMPStreamingType = WMPStreamingType(3i32);
pub const wmpstUnknown: WMPStreamingType = WMPStreamingType(0i32);
pub const wmpstVideo: WMPStreamingType = WMPStreamingType(2i32);
pub const wmptsLarge: WMPTemplateSize = WMPTemplateSize(2i32);
pub const wmptsMedium: WMPTemplateSize = WMPTemplateSize(1i32);
pub const wmptsSmall: WMPTemplateSize = WMPTemplateSize(0i32);
pub const wmpttBrowse: WMPTaskType = WMPTaskType(1i32);
pub const wmpttBurn: WMPTaskType = WMPTaskType(3i32);
pub const wmpttBuy: WMPTransactionType = WMPTransactionType(2i32);
pub const wmpttCurrent: WMPTaskType = WMPTaskType(4i32);
pub const wmpttDownload: WMPTransactionType = WMPTransactionType(1i32);
pub const wmpttNoTransaction: WMPTransactionType = WMPTransactionType(0i32);
pub const wmpttSync: WMPTaskType = WMPTaskType(2i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_BACKGROUNDSYNC_ACTION(pub i32);
impl ::core::marker::Copy for FEEDS_BACKGROUNDSYNC_ACTION {}
impl ::core::clone::Clone for FEEDS_BACKGROUNDSYNC_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_BACKGROUNDSYNC_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_BACKGROUNDSYNC_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_BACKGROUNDSYNC_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_BACKGROUNDSYNC_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_BACKGROUNDSYNC_STATUS(pub i32);
impl ::core::marker::Copy for FEEDS_BACKGROUNDSYNC_STATUS {}
impl ::core::clone::Clone for FEEDS_BACKGROUNDSYNC_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_BACKGROUNDSYNC_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_BACKGROUNDSYNC_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_BACKGROUNDSYNC_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_BACKGROUNDSYNC_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_DOWNLOAD_ERROR(pub i32);
impl ::core::marker::Copy for FEEDS_DOWNLOAD_ERROR {}
impl ::core::clone::Clone for FEEDS_DOWNLOAD_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_DOWNLOAD_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_DOWNLOAD_ERROR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_DOWNLOAD_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_DOWNLOAD_ERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_DOWNLOAD_STATUS(pub i32);
impl ::core::marker::Copy for FEEDS_DOWNLOAD_STATUS {}
impl ::core::clone::Clone for FEEDS_DOWNLOAD_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_DOWNLOAD_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_DOWNLOAD_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_DOWNLOAD_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_DOWNLOAD_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_ERROR_CODE(pub i32);
impl ::core::marker::Copy for FEEDS_ERROR_CODE {}
impl ::core::clone::Clone for FEEDS_ERROR_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_ERROR_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_ERROR_CODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_ERROR_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_ERROR_CODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_EVENTS_ITEM_COUNT_FLAGS(pub i32);
impl ::core::marker::Copy for FEEDS_EVENTS_ITEM_COUNT_FLAGS {}
impl ::core::clone::Clone for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_EVENTS_ITEM_COUNT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_EVENTS_MASK(pub i32);
impl ::core::marker::Copy for FEEDS_EVENTS_MASK {}
impl ::core::clone::Clone for FEEDS_EVENTS_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_EVENTS_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_EVENTS_MASK {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_EVENTS_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_EVENTS_MASK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_EVENTS_SCOPE(pub i32);
impl ::core::marker::Copy for FEEDS_EVENTS_SCOPE {}
impl ::core::clone::Clone for FEEDS_EVENTS_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_EVENTS_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_EVENTS_SCOPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_EVENTS_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_EVENTS_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_SYNC_SETTING(pub i32);
impl ::core::marker::Copy for FEEDS_SYNC_SETTING {}
impl ::core::clone::Clone for FEEDS_SYNC_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_SYNC_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_SYNC_SETTING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_SYNC_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_SYNC_SETTING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_XML_FILTER_FLAGS(pub i32);
impl ::core::marker::Copy for FEEDS_XML_FILTER_FLAGS {}
impl ::core::clone::Clone for FEEDS_XML_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_XML_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_XML_FILTER_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_XML_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_FILTER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_XML_INCLUDE_FLAGS(pub i32);
impl ::core::marker::Copy for FEEDS_XML_INCLUDE_FLAGS {}
impl ::core::clone::Clone for FEEDS_XML_INCLUDE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_XML_INCLUDE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_XML_INCLUDE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_XML_INCLUDE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_INCLUDE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_XML_SORT_ORDER(pub i32);
impl ::core::marker::Copy for FEEDS_XML_SORT_ORDER {}
impl ::core::clone::Clone for FEEDS_XML_SORT_ORDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_XML_SORT_ORDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_XML_SORT_ORDER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_XML_SORT_ORDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_SORT_ORDER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_XML_SORT_PROPERTY(pub i32);
impl ::core::marker::Copy for FEEDS_XML_SORT_PROPERTY {}
impl ::core::clone::Clone for FEEDS_XML_SORT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_XML_SORT_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FEEDS_XML_SORT_PROPERTY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FEEDS_XML_SORT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_SORT_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayerState(pub i32);
impl ::core::marker::Copy for PlayerState {}
impl ::core::clone::Clone for PlayerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayerState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PlayerState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlayerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayerState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPAccountType(pub i32);
impl ::core::marker::Copy for WMPAccountType {}
impl ::core::clone::Clone for WMPAccountType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPAccountType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPAccountType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPAccountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPAccountType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPBurnFormat(pub i32);
impl ::core::marker::Copy for WMPBurnFormat {}
impl ::core::clone::Clone for WMPBurnFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPBurnFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPBurnFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPBurnFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPBurnFormat").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPBurnState(pub i32);
impl ::core::marker::Copy for WMPBurnState {}
impl ::core::clone::Clone for WMPBurnState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPBurnState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPBurnState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPBurnState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPBurnState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPCallbackNotification(pub i32);
impl ::core::marker::Copy for WMPCallbackNotification {}
impl ::core::clone::Clone for WMPCallbackNotification {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPCallbackNotification {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPCallbackNotification {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPCallbackNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPCallbackNotification").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPDeviceStatus(pub i32);
impl ::core::marker::Copy for WMPDeviceStatus {}
impl ::core::clone::Clone for WMPDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPDeviceStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPDeviceStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPDeviceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPDeviceStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPFolderScanState(pub i32);
impl ::core::marker::Copy for WMPFolderScanState {}
impl ::core::clone::Clone for WMPFolderScanState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPFolderScanState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPFolderScanState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPFolderScanState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPFolderScanState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPLibraryType(pub i32);
impl ::core::marker::Copy for WMPLibraryType {}
impl ::core::clone::Clone for WMPLibraryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPLibraryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPLibraryType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPLibraryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPLibraryType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPOpenState(pub i32);
impl ::core::marker::Copy for WMPOpenState {}
impl ::core::clone::Clone for WMPOpenState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPOpenState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPOpenState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPOpenState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPOpenState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPPartnerNotification(pub i32);
impl ::core::marker::Copy for WMPPartnerNotification {}
impl ::core::clone::Clone for WMPPartnerNotification {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPPartnerNotification {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPPartnerNotification {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPPartnerNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPartnerNotification").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPPlayState(pub i32);
impl ::core::marker::Copy for WMPPlayState {}
impl ::core::clone::Clone for WMPPlayState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPPlayState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPPlayState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPPlayState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPlayState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPPlaylistChangeEventType(pub i32);
impl ::core::marker::Copy for WMPPlaylistChangeEventType {}
impl ::core::clone::Clone for WMPPlaylistChangeEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPPlaylistChangeEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPPlaylistChangeEventType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPPlaylistChangeEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPlaylistChangeEventType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPPlugin_Caps(pub i32);
impl ::core::marker::Copy for WMPPlugin_Caps {}
impl ::core::clone::Clone for WMPPlugin_Caps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPPlugin_Caps {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPPlugin_Caps {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPPlugin_Caps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPlugin_Caps").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPRipState(pub i32);
impl ::core::marker::Copy for WMPRipState {}
impl ::core::clone::Clone for WMPRipState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPRipState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPRipState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPRipState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPRipState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPServices_StreamState(pub i32);
impl ::core::marker::Copy for WMPServices_StreamState {}
impl ::core::clone::Clone for WMPServices_StreamState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPServices_StreamState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPServices_StreamState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPServices_StreamState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPServices_StreamState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPStreamingType(pub i32);
impl ::core::marker::Copy for WMPStreamingType {}
impl ::core::clone::Clone for WMPStreamingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPStreamingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPStreamingType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPStreamingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPStreamingType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPStringCollectionChangeEventType(pub i32);
impl ::core::marker::Copy for WMPStringCollectionChangeEventType {}
impl ::core::clone::Clone for WMPStringCollectionChangeEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPStringCollectionChangeEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPStringCollectionChangeEventType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPStringCollectionChangeEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPStringCollectionChangeEventType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPSubscriptionDownloadState(pub i32);
impl ::core::marker::Copy for WMPSubscriptionDownloadState {}
impl ::core::clone::Clone for WMPSubscriptionDownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPSubscriptionDownloadState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPSubscriptionDownloadState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPSubscriptionDownloadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPSubscriptionDownloadState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPSubscriptionServiceEvent(pub i32);
impl ::core::marker::Copy for WMPSubscriptionServiceEvent {}
impl ::core::clone::Clone for WMPSubscriptionServiceEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPSubscriptionServiceEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPSubscriptionServiceEvent {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPSubscriptionServiceEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPSubscriptionServiceEvent").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPSyncState(pub i32);
impl ::core::marker::Copy for WMPSyncState {}
impl ::core::clone::Clone for WMPSyncState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPSyncState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPSyncState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPSyncState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPSyncState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPTaskType(pub i32);
impl ::core::marker::Copy for WMPTaskType {}
impl ::core::clone::Clone for WMPTaskType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPTaskType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPTaskType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPTaskType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPTaskType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPTemplateSize(pub i32);
impl ::core::marker::Copy for WMPTemplateSize {}
impl ::core::clone::Clone for WMPTemplateSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPTemplateSize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPTemplateSize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPTemplateSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPTemplateSize").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPTransactionType(pub i32);
impl ::core::marker::Copy for WMPTransactionType {}
impl ::core::clone::Clone for WMPTransactionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPTransactionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMPTransactionType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMPTransactionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPTransactionType").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TimedLevel {
    pub frequency: [u8; 2048],
    pub waveform: [u8; 2048],
    pub state: i32,
    pub timeStamp: i64,
}
impl ::core::marker::Copy for TimedLevel {}
impl ::core::clone::Clone for TimedLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedLevel").field("frequency", &self.frequency).field("waveform", &self.waveform).field("state", &self.state).field("timeStamp", &self.timeStamp).finish()
    }
}
impl ::windows_core::TypeKind for TimedLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for TimedLevel {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency && self.waveform == other.waveform && self.state == other.state && self.timeStamp == other.timeStamp
    }
}
impl ::core::cmp::Eq for TimedLevel {}
impl ::core::default::Default for TimedLevel {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMPContextMenuInfo {
    pub dwID: u32,
    pub bstrMenuText: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub bstrHelpText: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
}
impl ::core::clone::Clone for WMPContextMenuInfo {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for WMPContextMenuInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMPContextMenuInfo").field("dwID", &self.dwID).field("bstrMenuText", &self.bstrMenuText).field("bstrHelpText", &self.bstrHelpText).finish()
    }
}
impl ::windows_core::TypeKind for WMPContextMenuInfo {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WMPContextMenuInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwID == other.dwID && self.bstrMenuText == other.bstrMenuText && self.bstrHelpText == other.bstrHelpText
    }
}
impl ::core::cmp::Eq for WMPContextMenuInfo {}
impl ::core::default::Default for WMPContextMenuInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    pub dwCurrentTransactionID: u32,
    pub dwReturnedObjectCount: u32,
    pub dwUnretrievedObjectCount: u32,
    pub dwDeletedObjectStartingOffset: u32,
    pub dwFlags: u32,
    pub wsObjectPathnameList: [u16; 1],
}
impl ::core::marker::Copy for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {}
impl ::core::clone::Clone for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    pub dwChangesSinceTransactionID: u32,
    pub dwResultSetStartingIndex: u32,
}
impl ::core::marker::Copy for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {}
impl ::core::clone::Clone for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
