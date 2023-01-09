impl ::core::default::Default for FEEDS_BACKGROUNDSYNC_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_BACKGROUNDSYNC_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_BACKGROUNDSYNC_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_BACKGROUNDSYNC_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_BACKGROUNDSYNC_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_BACKGROUNDSYNC_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_DOWNLOAD_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_DOWNLOAD_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_DOWNLOAD_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_DOWNLOAD_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_DOWNLOAD_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_DOWNLOAD_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_ERROR_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_ERROR_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_ERROR_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_EVENTS_ITEM_COUNT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_EVENTS_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_EVENTS_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_EVENTS_MASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_EVENTS_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_EVENTS_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_EVENTS_SCOPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_SYNC_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_SYNC_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_SYNC_SETTING").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_XML_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_XML_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_FILTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_XML_INCLUDE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_XML_INCLUDE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_INCLUDE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_XML_SORT_ORDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_XML_SORT_ORDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_SORT_ORDER").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDS_XML_SORT_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDS_XML_SORT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_SORT_PROPERTY").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeed {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeed").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeed2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeed2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeed2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeed2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IFeed2 {
    pub unsafe fn Xml(&self, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Xml)(::windows::core::Vtable::as_raw(self), count, sortproperty, sortorder, filterflags, includeflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Rename(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Rename)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Url(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Url)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUrl(&self, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedurl)).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move(&self, newparentpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(newparentpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastWriteTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastWriteTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Download)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelAsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows::core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SyncSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSyncSetting(&self, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSyncSetting)(::windows::core::Vtable::as_raw(self), syncsetting).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Interval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalEnclosurePath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Items(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Items)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetItem(&self, itemid: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), itemid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Title)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Link)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Image(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Image)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastBuildDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastBuildDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PubDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Ttl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Language(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Language)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Copyright)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MaxItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, count: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaxItemCount)(::windows::core::Vtable::as_raw(self), count).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDownloadEnclosuresAutomatically<P0>(&self, downloadenclosuresautomatically: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), downloadenclosuresautomatically.into()).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadError)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Merge(&self, feedxml: &::windows::core::BSTR, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Merge)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedxml), ::core::mem::transmute_copy(feedurl)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsList(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MarkAllItemsRead)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWatcher)(::windows::core::Vtable::as_raw(self), scope, mask, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UnreadItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedEnclosure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedEnclosure {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedEnclosure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedEnclosure").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedFolder {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedFolder").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedFolderEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedFolderEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedFolderEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedFolderEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedItem2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedItem2 {
    pub unsafe fn Xml(&self, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Xml)(::windows::core::Vtable::as_raw(self), includeflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Title)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Link)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Guid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PubDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Comments)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Author(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Author)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enclosure(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Enclosure)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRead(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsRead)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRead<P0>(&self, isread: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIsRead)(::windows::core::Vtable::as_raw(self), isread.into()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Modified(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Modified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedsEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedsEnum {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedsEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedsEnum").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedsManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedsManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPAudioRenderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPAudioRenderConfig {}
impl ::core::fmt::Debug for IWMPAudioRenderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPAudioRenderConfig").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCdrom {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCdrom {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCdrom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCdrom").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPCdromBurn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPCdromBurn {}
impl ::core::fmt::Debug for IWMPCdromBurn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCdromBurn").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCdromCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCdromCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCdromCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCdromCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPCdromRip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPCdromRip {}
impl ::core::fmt::Debug for IWMPCdromRip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCdromRip").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPClosedCaption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPClosedCaption {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPClosedCaption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPClosedCaption").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPClosedCaption2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPClosedCaption2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPClosedCaption2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPClosedCaption2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPClosedCaption2 {
    pub unsafe fn SAMIStyle(&self, pbstrsamistyle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SAMIStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsamistyle)).ok()
    }
    pub unsafe fn SetSAMIStyle(&self, bstrsamistyle: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSAMIStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsamistyle)).ok()
    }
    pub unsafe fn SAMILang(&self, pbstrsamilang: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SAMILang)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsamilang)).ok()
    }
    pub unsafe fn SetSAMILang(&self, bstrsamilang: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSAMILang)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsamilang)).ok()
    }
    pub unsafe fn SAMIFileName(&self, pbstrsamifilename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SAMIFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsamifilename)).ok()
    }
    pub unsafe fn SetSAMIFileName(&self, bstrsamifilename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSAMIFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsamifilename)).ok()
    }
    pub unsafe fn captioningId(&self, pbstrcaptioningid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.captioningId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcaptioningid)).ok()
    }
    pub unsafe fn SetcaptioningId(&self, bstrcaptioningid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetcaptioningId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcaptioningid)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPContentContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPContentContainer {}
impl ::core::fmt::Debug for IWMPContentContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPContentContainer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPContentContainerList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPContentContainerList {}
impl ::core::fmt::Debug for IWMPContentContainerList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPContentContainerList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPContentPartner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPContentPartner {}
impl ::core::fmt::Debug for IWMPContentPartner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPContentPartner").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPContentPartnerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPContentPartnerCallback {}
impl ::core::fmt::Debug for IWMPContentPartnerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPContentPartnerCallback").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPControls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPControls {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPControls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPControls").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPControls2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPControls2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPControls2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPControls2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.get_isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), pisavailable).ok()
    }
    pub unsafe fn play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.play)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastForward(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.fastForward)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastReverse(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.fastReverse)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn currentPosition(&self, pdcurrentposition: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.currentPosition)(::windows::core::Vtable::as_raw(self), pdcurrentposition).ok()
    }
    pub unsafe fn SetcurrentPosition(&self, dcurrentposition: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentPosition)(::windows::core::Vtable::as_raw(self), dcurrentposition).ok()
    }
    pub unsafe fn currentPositionString(&self, pbstrcurrentposition: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.currentPositionString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcurrentposition)).ok()
    }
    pub unsafe fn next(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.next)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn previous(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.previous)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentItem(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentItem<P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    pub unsafe fn currentMarker(&self, plmarker: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.currentMarker)(::windows::core::Vtable::as_raw(self), plmarker).ok()
    }
    pub unsafe fn SetcurrentMarker(&self, lmarker: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentMarker)(::windows::core::Vtable::as_raw(self), lmarker).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playItem<P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.playItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPControls3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPControls3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPControls3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPControls3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.get_isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), pisavailable).ok()
    }
    pub unsafe fn play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.play)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastForward(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.fastForward)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastReverse(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.fastReverse)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn currentPosition(&self, pdcurrentposition: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.currentPosition)(::windows::core::Vtable::as_raw(self), pdcurrentposition).ok()
    }
    pub unsafe fn SetcurrentPosition(&self, dcurrentposition: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentPosition)(::windows::core::Vtable::as_raw(self), dcurrentposition).ok()
    }
    pub unsafe fn currentPositionString(&self, pbstrcurrentposition: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.currentPositionString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcurrentposition)).ok()
    }
    pub unsafe fn next(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.next)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn previous(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.previous)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentItem(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentItem<P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    pub unsafe fn currentMarker(&self, plmarker: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.currentMarker)(::windows::core::Vtable::as_raw(self), plmarker).ok()
    }
    pub unsafe fn SetcurrentMarker(&self, lmarker: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentMarker)(::windows::core::Vtable::as_raw(self), lmarker).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playItem<P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.playItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    pub unsafe fn step(&self, lstep: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.step)(::windows::core::Vtable::as_raw(self), lstep).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPConvert {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPConvert {}
impl ::core::fmt::Debug for IWMPConvert {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPConvert").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCore {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCore").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCore2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCore2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCore2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCore2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore2 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.openState)(::windows::core::Vtable::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.playState)(::windows::core::Vtable::as_raw(self), pwmpps).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.controls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.settings)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentMedia)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.mediaCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.playlistCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.network)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.cdromCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.closedCaption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.isOnline)(::windows::core::Vtable::as_raw(self), pfonline).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCore3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCore3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCore3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCore3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore3 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.openState)(::windows::core::Vtable::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.playState)(::windows::core::Vtable::as_raw(self), pwmpps).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.controls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.settings)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentMedia)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.mediaCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.playlistCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.network)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.cdromCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.closedCaption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.isOnline)(::windows::core::Vtable::as_raw(self), pfonline).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows::core::Result<IWMPDVD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.dvd)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDVD {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDVD {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDVD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDVD").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDownloadCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDownloadCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDownloadCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDownloadCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDownloadItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDownloadItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDownloadItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDownloadItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDownloadItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDownloadItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDownloadItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDownloadItem2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadItem2 {
    pub unsafe fn sourceURL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.sourceURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn size(&self, plsize: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.size)(::windows::core::Vtable::as_raw(self), plsize).ok()
    }
    pub unsafe fn r#type(&self, pbstrtype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.r#type)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrtype)).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.progress)(::windows::core::Vtable::as_raw(self), plprogress).ok()
    }
    pub unsafe fn downloadState(&self, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.downloadState)(::windows::core::Vtable::as_raw(self), pwmpsdls).ok()
    }
    pub unsafe fn pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDownloadManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDownloadManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDownloadManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDownloadManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPEffects {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEffects {}
impl ::core::fmt::Debug for IWMPEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEffects").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPEffects2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEffects2 {}
impl ::core::fmt::Debug for IWMPEffects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEffects2").field(&self.0).finish()
    }
}
impl IWMPEffects2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Render<P0>(&self, plevels: *mut TimedLevel, hdc: P0, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.Render)(::windows::core::Vtable::as_raw(self), plevels, hdc.into(), prc).ok()
    }
    pub unsafe fn MediaInfo(&self, lchannelcount: i32, lsamplerate: i32, bstrtitle: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MediaInfo)(::windows::core::Vtable::as_raw(self), lchannelcount, lsamplerate, ::core::mem::transmute_copy(bstrtitle)).ok()
    }
    pub unsafe fn GetCapabilities(&self, pdwcapabilities: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), pdwcapabilities).ok()
    }
    pub unsafe fn GetTitle(&self, bstrtitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTitle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(bstrtitle)).ok()
    }
    pub unsafe fn GetPresetTitle(&self, npreset: i32, bstrpresettitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPresetTitle)(::windows::core::Vtable::as_raw(self), npreset, ::core::mem::transmute(bstrpresettitle)).ok()
    }
    pub unsafe fn GetPresetCount(&self, pnpresetcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPresetCount)(::windows::core::Vtable::as_raw(self), pnpresetcount).ok()
    }
    pub unsafe fn SetCurrentPreset(&self, npreset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentPreset)(::windows::core::Vtable::as_raw(self), npreset).ok()
    }
    pub unsafe fn GetCurrentPreset(&self, pnpreset: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCurrentPreset)(::windows::core::Vtable::as_raw(self), pnpreset).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayPropertyPage<P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.DisplayPropertyPage)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GoFullscreen<P0>(&self, ffullscreen: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GoFullscreen)(::windows::core::Vtable::as_raw(self), ffullscreen.into()).ok()
    }
    pub unsafe fn RenderFullScreen(&self, plevels: *mut TimedLevel) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RenderFullScreen)(::windows::core::Vtable::as_raw(self), plevels).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPError {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPError").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPErrorItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPErrorItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPErrorItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPErrorItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPErrorItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPErrorItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPErrorItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPErrorItem2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorItem2 {
    pub unsafe fn errorCode(&self, phr: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.errorCode)(::windows::core::Vtable::as_raw(self), phr).ok()
    }
    pub unsafe fn errorDescription(&self, pbstrdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.errorDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn errorContext(&self, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.errorContext)(::windows::core::Vtable::as_raw(self), pvarcontext).ok()
    }
    pub unsafe fn remedy(&self, plremedy: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.remedy)(::windows::core::Vtable::as_raw(self), plremedy).ok()
    }
    pub unsafe fn customUrl(&self, pbstrcustomurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.customUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcustomurl)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEvents {}
impl ::core::fmt::Debug for IWMPEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPEvents2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEvents2 {}
impl ::core::fmt::Debug for IWMPEvents2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEvents2").field(&self.0).finish()
    }
}
impl IWMPEvents2 {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.OpenStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.PlayStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows::core::Vtable::vtable(self).base__.AudioLanguageChange)(::windows::core::Vtable::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.StatusChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ScriptCommand(&self, sctype: &::windows::core::BSTR, param: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.ScriptCommand)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(sctype), ::core::mem::transmute_copy(param))
    }
    pub unsafe fn NewStream(&self) {
        (::windows::core::Vtable::vtable(self).base__.NewStream)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.Disconnect)(::windows::core::Vtable::as_raw(self), result)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Buffering<P0>(&self, start: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Buffering)(::windows::core::Vtable::as_raw(self), start.into())
    }
    pub unsafe fn Error(&self) {
        (::windows::core::Vtable::vtable(self).base__.Error)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Warning(&self, warningtype: i32, param: i32, description: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.Warning)(::windows::core::Vtable::as_raw(self), warningtype, param, ::core::mem::transmute_copy(description))
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.EndOfStream)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows::core::Vtable::vtable(self).base__.PositionChange)(::windows::core::Vtable::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows::core::Vtable::vtable(self).base__.MarkerHit)(::windows::core::Vtable::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows::core::Vtable::vtable(self).base__.DurationUnitChange)(::windows::core::Vtable::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows::core::Vtable::vtable(self).base__.CdromMediaChange)(::windows::core::Vtable::as_raw(self), cdromnum)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PlaylistChange)(::windows::core::Vtable::as_raw(self), playlist.into().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows::core::Vtable::vtable(self).base__.CurrentPlaylistChange)(::windows::core::Vtable::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.CurrentPlaylistItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<P0>(&self, item: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MediaChange)(::windows::core::Vtable::as_raw(self), item.into().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.CurrentMediaItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CurrentItemChange)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionAttributeStringAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionAttributeStringRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &::windows::core::BSTR, bstroldattribval: &::windows::core::BSTR, bstrnewattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionAttributeStringChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstroldattribval), ::core::mem::transmute_copy(bstrnewattribval))
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.PlaylistCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.PlaylistCollectionPlaylistAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.PlaylistCollectionPlaylistRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted<P0>(&self, bstrplaylistname: &::windows::core::BSTR, varfisdeleted: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.PlaylistCollectionPlaylistSetAsDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname), varfisdeleted.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModeChange<P0>(&self, modename: &::windows::core::BSTR, newvalue: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ModeChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(modename), newvalue.into())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<P0>(&self, pmediaobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MediaError)(::windows::core::Vtable::as_raw(self), pmediaobject.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<P0>(&self, pitem: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OpenPlaylistSwitch)(::windows::core::Vtable::as_raw(self), pitem.into().abi())
    }
    pub unsafe fn DomainChange(&self, strdomain: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.DomainChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strdomain))
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows::core::Vtable::vtable(self).base__.SwitchedToPlayerApplication)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows::core::Vtable::vtable(self).base__.SwitchedToControl)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.PlayerDockedStateChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows::core::Vtable::vtable(self).base__.PlayerReconnect)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.Click)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.DoubleClick)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.KeyDown)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows::core::Vtable::vtable(self).base__.KeyPress)(::windows::core::Vtable::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.KeyUp)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.MouseDown)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.MouseMove)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.MouseUp)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
}
impl ::core::cmp::PartialEq for IWMPEvents3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEvents3 {}
impl ::core::fmt::Debug for IWMPEvents3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEvents3").field(&self.0).finish()
    }
}
impl IWMPEvents3 {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlayStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.AudioLanguageChange)(::windows::core::Vtable::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.StatusChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ScriptCommand(&self, sctype: &::windows::core::BSTR, param: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.ScriptCommand)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(sctype), ::core::mem::transmute_copy(param))
    }
    pub unsafe fn NewStream(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.NewStream)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.Disconnect)(::windows::core::Vtable::as_raw(self), result)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Buffering<P0>(&self, start: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Buffering)(::windows::core::Vtable::as_raw(self), start.into())
    }
    pub unsafe fn Error(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.Error)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Warning(&self, warningtype: i32, param: i32, description: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.Warning)(::windows::core::Vtable::as_raw(self), warningtype, param, ::core::mem::transmute_copy(description))
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.EndOfStream)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows::core::Vtable::vtable(self).base__.base__.PositionChange)(::windows::core::Vtable::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.MarkerHit)(::windows::core::Vtable::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.DurationUnitChange)(::windows::core::Vtable::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.CdromMediaChange)(::windows::core::Vtable::as_raw(self), cdromnum)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistChange)(::windows::core::Vtable::as_raw(self), playlist.into().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentPlaylistChange)(::windows::core::Vtable::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentPlaylistItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<P0>(&self, item: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaChange)(::windows::core::Vtable::as_raw(self), item.into().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentMediaItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentItemChange)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaCollectionAttributeStringAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaCollectionAttributeStringRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &::windows::core::BSTR, bstroldattribval: &::windows::core::BSTR, bstrnewattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaCollectionAttributeStringChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstroldattribval), ::core::mem::transmute_copy(bstrnewattribval))
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistCollectionPlaylistAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistCollectionPlaylistRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted<P0>(&self, bstrplaylistname: &::windows::core::BSTR, varfisdeleted: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistCollectionPlaylistSetAsDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname), varfisdeleted.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModeChange<P0>(&self, modename: &::windows::core::BSTR, newvalue: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ModeChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(modename), newvalue.into())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<P0>(&self, pmediaobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaError)(::windows::core::Vtable::as_raw(self), pmediaobject.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<P0>(&self, pitem: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenPlaylistSwitch)(::windows::core::Vtable::as_raw(self), pitem.into().abi())
    }
    pub unsafe fn DomainChange(&self, strdomain: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.DomainChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strdomain))
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.SwitchedToPlayerApplication)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.SwitchedToControl)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlayerDockedStateChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlayerReconnect)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.Click)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.DoubleClick)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.KeyDown)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.KeyPress)(::windows::core::Vtable::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.KeyUp)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.MouseDown)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.MouseMove)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.MouseUp)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DeviceConnect<P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceConnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceDisconnect<P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceDisconnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceStatusChange<P0>(&self, pdevice: P0, newstatus: WMPDeviceStatus)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceStatusChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstatus)
    }
    pub unsafe fn DeviceSyncStateChange<P0>(&self, pdevice: P0, newstate: WMPSyncState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceSyncStateChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstate)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSyncError<P0, P1>(&self, pdevice: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceSyncError)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CreatePartnershipComplete<P0>(&self, pdevice: P0, hrresult: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreatePartnershipComplete)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hrresult)
    }
}
impl ::core::cmp::PartialEq for IWMPEvents4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEvents4 {}
impl ::core::fmt::Debug for IWMPEvents4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEvents4").field(&self.0).finish()
    }
}
impl IWMPEvents4 {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlayStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AudioLanguageChange)(::windows::core::Vtable::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StatusChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ScriptCommand(&self, sctype: &::windows::core::BSTR, param: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ScriptCommand)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(sctype), ::core::mem::transmute_copy(param))
    }
    pub unsafe fn NewStream(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NewStream)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Disconnect)(::windows::core::Vtable::as_raw(self), result)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Buffering<P0>(&self, start: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Buffering)(::windows::core::Vtable::as_raw(self), start.into())
    }
    pub unsafe fn Error(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Error)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Warning(&self, warningtype: i32, param: i32, description: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Warning)(::windows::core::Vtable::as_raw(self), warningtype, param, ::core::mem::transmute_copy(description))
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EndOfStream)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PositionChange)(::windows::core::Vtable::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MarkerHit)(::windows::core::Vtable::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DurationUnitChange)(::windows::core::Vtable::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CdromMediaChange)(::windows::core::Vtable::as_raw(self), cdromnum)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistChange)(::windows::core::Vtable::as_raw(self), playlist.into().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentPlaylistChange)(::windows::core::Vtable::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentPlaylistItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<P0>(&self, item: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaChange)(::windows::core::Vtable::as_raw(self), item.into().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentMediaItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentItemChange)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaCollectionAttributeStringAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaCollectionAttributeStringRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &::windows::core::BSTR, bstroldattribval: &::windows::core::BSTR, bstrnewattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaCollectionAttributeStringChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstroldattribval), ::core::mem::transmute_copy(bstrnewattribval))
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistCollectionPlaylistAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistCollectionPlaylistRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted<P0>(&self, bstrplaylistname: &::windows::core::BSTR, varfisdeleted: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistCollectionPlaylistSetAsDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname), varfisdeleted.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModeChange<P0>(&self, modename: &::windows::core::BSTR, newvalue: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ModeChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(modename), newvalue.into())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<P0>(&self, pmediaobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaError)(::windows::core::Vtable::as_raw(self), pmediaobject.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<P0>(&self, pitem: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenPlaylistSwitch)(::windows::core::Vtable::as_raw(self), pitem.into().abi())
    }
    pub unsafe fn DomainChange(&self, strdomain: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DomainChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strdomain))
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SwitchedToPlayerApplication)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SwitchedToControl)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlayerDockedStateChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlayerReconnect)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Click)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DoubleClick)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.KeyDown)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.KeyPress)(::windows::core::Vtable::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.KeyUp)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MouseDown)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MouseMove)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MouseUp)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DeviceConnect<P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceConnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceDisconnect<P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceDisconnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceStatusChange<P0>(&self, pdevice: P0, newstatus: WMPDeviceStatus)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceStatusChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstatus)
    }
    pub unsafe fn DeviceSyncStateChange<P0>(&self, pdevice: P0, newstate: WMPSyncState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceSyncStateChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstate)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSyncError<P0, P1>(&self, pdevice: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceSyncError)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CreatePartnershipComplete<P0>(&self, pdevice: P0, hrresult: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePartnershipComplete)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hrresult)
    }
    pub unsafe fn CdromRipStateChange<P0>(&self, pcdromrip: P0, wmprs: WMPRipState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPCdromRip>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromRipStateChange)(::windows::core::Vtable::as_raw(self), pcdromrip.into().abi(), wmprs)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromRipMediaError<P0, P1>(&self, pcdromrip: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPCdromRip>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromRipMediaError)(::windows::core::Vtable::as_raw(self), pcdromrip.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CdromBurnStateChange<P0>(&self, pcdromburn: P0, wmpbs: WMPBurnState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPCdromBurn>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromBurnStateChange)(::windows::core::Vtable::as_raw(self), pcdromburn.into().abi(), wmpbs)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromBurnMediaError<P0, P1>(&self, pcdromburn: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPCdromBurn>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromBurnMediaError)(::windows::core::Vtable::as_raw(self), pcdromburn.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CdromBurnError<P0>(&self, pcdromburn: P0, hrerror: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPCdromBurn>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromBurnError)(::windows::core::Vtable::as_raw(self), pcdromburn.into().abi(), hrerror)
    }
    pub unsafe fn LibraryConnect<P0>(&self, plibrary: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPLibrary>>,
    {
        (::windows::core::Vtable::vtable(self).base__.LibraryConnect)(::windows::core::Vtable::as_raw(self), plibrary.into().abi())
    }
    pub unsafe fn LibraryDisconnect<P0>(&self, plibrary: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPLibrary>>,
    {
        (::windows::core::Vtable::vtable(self).base__.LibraryDisconnect)(::windows::core::Vtable::as_raw(self), plibrary.into().abi())
    }
    pub unsafe fn FolderScanStateChange(&self, wmpfss: WMPFolderScanState) {
        (::windows::core::Vtable::vtable(self).base__.FolderScanStateChange)(::windows::core::Vtable::as_raw(self), wmpfss)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StringCollectionChange<P0>(&self, pdispstringcollection: P0, change: WMPStringCollectionChangeEventType, lcollectionindex: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StringCollectionChange)(::windows::core::Vtable::as_raw(self), pdispstringcollection.into().abi(), change, lcollectionindex)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaAdded<P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionMediaAdded)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaRemoved<P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionMediaRemoved)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
}
impl ::core::cmp::PartialEq for IWMPFolderMonitorServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPFolderMonitorServices {}
impl ::core::fmt::Debug for IWMPFolderMonitorServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPFolderMonitorServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPGraphCreation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPGraphCreation {}
impl ::core::fmt::Debug for IWMPGraphCreation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPGraphCreation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPLibrary {}
impl ::core::fmt::Debug for IWMPLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPLibrary").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPLibrary2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPLibrary2 {}
impl ::core::fmt::Debug for IWMPLibrary2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPLibrary2").field(&self.0).finish()
    }
}
impl IWMPLibrary2 {
    pub unsafe fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn r#type(&self, pwmplt: *mut WMPLibraryType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.r#type)(::windows::core::Vtable::as_raw(self), pwmplt).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.mediaCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isIdentical<P0>(&self, piwmplibrary: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPLibrary>>,
    {
        (::windows::core::Vtable::vtable(self).base__.isIdentical)(::windows::core::Vtable::as_raw(self), piwmplibrary.into().abi(), pvbool).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPLibraryServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPLibraryServices {}
impl ::core::fmt::Debug for IWMPLibraryServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPLibraryServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPLibrarySharingServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPLibrarySharingServices {}
impl ::core::fmt::Debug for IWMPLibrarySharingServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPLibrarySharingServices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMedia {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMedia {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMedia").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMedia2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMedia2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMedia2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMedia2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_isIdentical<P0>(&self, piwmpmedia: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.get_isIdentical)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi(), pvbool).ok()
    }
    pub unsafe fn sourceURL(&self, pbstrsourceurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.sourceURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsourceurl)).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Setname)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn imageSourceWidth(&self, pwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.imageSourceWidth)(::windows::core::Vtable::as_raw(self), pwidth).ok()
    }
    pub unsafe fn imageSourceHeight(&self, pheight: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.imageSourceHeight)(::windows::core::Vtable::as_raw(self), pheight).ok()
    }
    pub unsafe fn markerCount(&self, pmarkercount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.markerCount)(::windows::core::Vtable::as_raw(self), pmarkercount).ok()
    }
    pub unsafe fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getMarkerTime)(::windows::core::Vtable::as_raw(self), markernum, pmarkertime).ok()
    }
    pub unsafe fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getMarkerName)(::windows::core::Vtable::as_raw(self), markernum, ::core::mem::transmute(pbstrmarkername)).ok()
    }
    pub unsafe fn duration(&self, pduration: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.duration)(::windows::core::Vtable::as_raw(self), pduration).ok()
    }
    pub unsafe fn durationString(&self, pbstrduration: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.durationString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrduration)).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.attributeCount)(::windows::core::Vtable::as_raw(self), plcount).ok()
    }
    pub unsafe fn getAttributeName(&self, lindex: i32, pbstritemname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getAttributeName)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstritemname)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.setItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute_copy(bstrval)).ok()
    }
    pub unsafe fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getItemInfoByAtom)(::windows::core::Vtable::as_raw(self), latom, ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isMemberOf<P0>(&self, pplaylist: P0, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.isMemberOf)(::windows::core::Vtable::as_raw(self), pplaylist.into().abi(), pvarfismemberof).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isReadOnlyItem(&self, bstritemname: &::windows::core::BSTR, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.isReadOnlyItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), pvarfisreadonly).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMedia3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMedia3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMedia3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMedia3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_isIdentical<P0>(&self, piwmpmedia: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.get_isIdentical)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi(), pvbool).ok()
    }
    pub unsafe fn sourceURL(&self, pbstrsourceurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.sourceURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsourceurl)).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Setname)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn imageSourceWidth(&self, pwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.imageSourceWidth)(::windows::core::Vtable::as_raw(self), pwidth).ok()
    }
    pub unsafe fn imageSourceHeight(&self, pheight: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.imageSourceHeight)(::windows::core::Vtable::as_raw(self), pheight).ok()
    }
    pub unsafe fn markerCount(&self, pmarkercount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.markerCount)(::windows::core::Vtable::as_raw(self), pmarkercount).ok()
    }
    pub unsafe fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getMarkerTime)(::windows::core::Vtable::as_raw(self), markernum, pmarkertime).ok()
    }
    pub unsafe fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getMarkerName)(::windows::core::Vtable::as_raw(self), markernum, ::core::mem::transmute(pbstrmarkername)).ok()
    }
    pub unsafe fn duration(&self, pduration: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.duration)(::windows::core::Vtable::as_raw(self), pduration).ok()
    }
    pub unsafe fn durationString(&self, pbstrduration: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.durationString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrduration)).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.attributeCount)(::windows::core::Vtable::as_raw(self), plcount).ok()
    }
    pub unsafe fn getAttributeName(&self, lindex: i32, pbstritemname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getAttributeName)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstritemname)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.setItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute_copy(bstrval)).ok()
    }
    pub unsafe fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getItemInfoByAtom)(::windows::core::Vtable::as_raw(self), latom, ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isMemberOf<P0>(&self, pplaylist: P0, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.isMemberOf)(::windows::core::Vtable::as_raw(self), pplaylist.into().abi(), pvarfismemberof).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isReadOnlyItem(&self, bstritemname: &::windows::core::BSTR, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.isReadOnlyItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), pvarfisreadonly).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPErrorItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMediaCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMediaCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMediaCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMediaCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMediaCollection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMediaCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMediaCollection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMediaCollection2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaCollection2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn add(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.add)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAll(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getAll)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByGenre(&self, bstrgenre: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByGenre)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrgenre), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAuthor(&self, bstrauthor: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByAuthor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrauthor), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAlbum(&self, bstralbum: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByAlbum)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstralbum), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAttribute(&self, bstrattribute: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByAttribute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribute), ::core::mem::transmute_copy(bstrvalue), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn remove<P0, P1>(&self, pitem: P0, varfdeletefile: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.remove)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), varfdeletefile.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAttributeStringCollection(&self, bstrattribute: &::windows::core::BSTR, bstrmediatype: &::windows::core::BSTR) -> ::windows::core::Result<IWMPStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getAttributeStringCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribute), ::core::mem::transmute_copy(bstrmediatype), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn getMediaAtom(&self, bstritemname: &::windows::core::BSTR, platom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getMediaAtom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), platom).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn setDeleted<P0, P1>(&self, pitem: P0, varfisdeleted: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.setDeleted)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), varfisdeleted.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn isDeleted<P0>(&self, pitem: P0, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.isDeleted)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), pvarfisdeleted).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPMediaPluginRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPMediaPluginRegistrar {}
impl ::core::fmt::Debug for IWMPMediaPluginRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMediaPluginRegistrar").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMetadataPicture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMetadataPicture {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMetadataPicture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMetadataPicture").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMetadataText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMetadataText {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMetadataText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMetadataText").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPNetwork {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNetwork").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPNodeRealEstate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeRealEstate {}
impl ::core::fmt::Debug for IWMPNodeRealEstate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeRealEstate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPNodeRealEstateHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeRealEstateHost {}
impl ::core::fmt::Debug for IWMPNodeRealEstateHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeRealEstateHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPNodeWindowed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeWindowed {}
impl ::core::fmt::Debug for IWMPNodeWindowed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeWindowed").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPNodeWindowedHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeWindowedHost {}
impl ::core::fmt::Debug for IWMPNodeWindowedHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeWindowedHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPNodeWindowless {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeWindowless {}
impl ::core::fmt::Debug for IWMPNodeWindowless {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeWindowless").field(&self.0).finish()
    }
}
impl IWMPNodeWindowless {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessage<P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnWindowMessage)(::windows::core::Vtable::as_raw(self), umsg, wparam.into(), lparam.into(), plret, pfhandled).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPNodeWindowlessHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeWindowlessHost {}
impl ::core::fmt::Debug for IWMPNodeWindowlessHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeWindowlessHost").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.openState)(::windows::core::Vtable::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.playState)(::windows::core::Vtable::as_raw(self), pwmpps).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.controls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.settings)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentMedia)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.mediaCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.playlistCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.network)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.cdromCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.closedCaption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.isOnline)(::windows::core::Vtable::as_raw(self), pfonline).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayer2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayer2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer2 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.openState)(::windows::core::Vtable::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.playState)(::windows::core::Vtable::as_raw(self), pwmpps).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.controls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.settings)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentMedia)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.mediaCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.playlistCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.network)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.cdromCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.closedCaption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.isOnline)(::windows::core::Vtable::as_raw(self), pfonline).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayer3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayer3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer3 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.openState)(::windows::core::Vtable::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.playState)(::windows::core::Vtable::as_raw(self), pwmpps).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.controls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.settings)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentMedia)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.mediaCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.playlistCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.network)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.cdromCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.closedCaption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.isOnline)(::windows::core::Vtable::as_raw(self), pfonline).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows::core::Result<IWMPDVD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.dvd)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayer4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayer4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayer4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayer4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer4 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.openState)(::windows::core::Vtable::as_raw(self), pwmpos).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.playState)(::windows::core::Vtable::as_raw(self), pwmpps).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.controls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.settings)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.currentMedia)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.mediaCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.playlistCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.network)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.cdromCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.closedCaption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.isOnline)(::windows::core::Vtable::as_raw(self), pfonline).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows::core::Result<IWMPDVD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.dvd)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newPlaylist(&self, bstrname: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.newPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrurl), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newMedia(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.newMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayerApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayerApplication {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayerApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayerApplication").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPPlayerServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPlayerServices {}
impl ::core::fmt::Debug for IWMPPlayerServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayerServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPPlayerServices2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPlayerServices2 {}
impl ::core::fmt::Debug for IWMPPlayerServices2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayerServices2").field(&self.0).finish()
    }
}
impl IWMPPlayerServices2 {
    pub unsafe fn activateUIPlugin(&self, bstrplugin: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.activateUIPlugin)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplugin)).ok()
    }
    pub unsafe fn setTaskPane(&self, bstrtaskpane: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.setTaskPane)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtaskpane)).ok()
    }
    pub unsafe fn setTaskPaneURL(&self, bstrtaskpane: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, bstrfriendlyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.setTaskPaneURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtaskpane), ::core::mem::transmute_copy(bstrurl), ::core::mem::transmute_copy(bstrfriendlyname)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlaylist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlaylist {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlaylist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlaylist").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlaylistArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlaylistArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlaylistArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlaylistArray").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlaylistCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlaylistCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlaylistCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlaylistCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPlugin {}
impl ::core::fmt::Debug for IWMPPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlugin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPPluginEnable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPluginEnable {}
impl ::core::fmt::Debug for IWMPPluginEnable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPluginEnable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPPluginUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPluginUI {}
impl ::core::fmt::Debug for IWMPPluginUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPluginUI").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPQuery {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPQuery").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPRemoteMediaServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPRemoteMediaServices {}
impl ::core::fmt::Debug for IWMPRemoteMediaServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPRemoteMediaServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPRenderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPRenderConfig {}
impl ::core::fmt::Debug for IWMPRenderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPRenderConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPServices {}
impl ::core::fmt::Debug for IWMPServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPServices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPSettings2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPSettings2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPSettings2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSettings2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPSettings2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.get_isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), pisavailable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn autoStart(&self, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.autoStart)(::windows::core::Vtable::as_raw(self), pfautostart).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetautoStart<P0>(&self, fautostart: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetautoStart)(::windows::core::Vtable::as_raw(self), fautostart.into()).ok()
    }
    pub unsafe fn baseURL(&self, pbstrbaseurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.baseURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrbaseurl)).ok()
    }
    pub unsafe fn SetbaseURL(&self, bstrbaseurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetbaseURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrbaseurl)).ok()
    }
    pub unsafe fn defaultFrame(&self, pbstrdefaultframe: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.defaultFrame)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdefaultframe)).ok()
    }
    pub unsafe fn SetdefaultFrame(&self, bstrdefaultframe: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetdefaultFrame)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdefaultframe)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn invokeURLs(&self, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.invokeURLs)(::windows::core::Vtable::as_raw(self), pfinvokeurls).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetinvokeURLs<P0>(&self, finvokeurls: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetinvokeURLs)(::windows::core::Vtable::as_raw(self), finvokeurls.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn mute(&self, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.mute)(::windows::core::Vtable::as_raw(self), pfmute).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setmute<P0>(&self, fmute: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Setmute)(::windows::core::Vtable::as_raw(self), fmute.into()).ok()
    }
    pub unsafe fn playCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.playCount)(::windows::core::Vtable::as_raw(self), plcount).ok()
    }
    pub unsafe fn SetplayCount(&self, lcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetplayCount)(::windows::core::Vtable::as_raw(self), lcount).ok()
    }
    pub unsafe fn rate(&self, pdrate: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.rate)(::windows::core::Vtable::as_raw(self), pdrate).ok()
    }
    pub unsafe fn Setrate(&self, drate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Setrate)(::windows::core::Vtable::as_raw(self), drate).ok()
    }
    pub unsafe fn balance(&self, plbalance: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.balance)(::windows::core::Vtable::as_raw(self), plbalance).ok()
    }
    pub unsafe fn Setbalance(&self, lbalance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Setbalance)(::windows::core::Vtable::as_raw(self), lbalance).ok()
    }
    pub unsafe fn volume(&self, plvolume: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.volume)(::windows::core::Vtable::as_raw(self), plvolume).ok()
    }
    pub unsafe fn Setvolume(&self, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Setvolume)(::windows::core::Vtable::as_raw(self), lvolume).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getMode(&self, bstrmode: &::windows::core::BSTR, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode), pvarfmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setMode<P0>(&self, bstrmode: &::windows::core::BSTR, varfmode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.setMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode), varfmode.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enableErrorDialogs(&self, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.enableErrorDialogs)(::windows::core::Vtable::as_raw(self), pfenableerrordialogs).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetenableErrorDialogs<P0>(&self, fenableerrordialogs: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetenableErrorDialogs)(::windows::core::Vtable::as_raw(self), fenableerrordialogs.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPSkinManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSkinManager {}
impl ::core::fmt::Debug for IWMPSkinManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSkinManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPStringCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPStringCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPStringCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPStringCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPStringCollection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPStringCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPStringCollection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPStringCollection2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPStringCollection2 {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.count)(::windows::core::Vtable::as_raw(self), plcount).ok()
    }
    pub unsafe fn item(&self, lindex: i32, pbstrstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.item)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstrstring)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPSubscriptionService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSubscriptionService {}
impl ::core::fmt::Debug for IWMPSubscriptionService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSubscriptionService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPSubscriptionService2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSubscriptionService2 {}
impl ::core::fmt::Debug for IWMPSubscriptionService2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSubscriptionService2").field(&self.0).finish()
    }
}
impl IWMPSubscriptionService2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPlay<P0, P1>(&self, hwnd: P0, pmedia: P1, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.allowPlay)(::windows::core::Vtable::as_raw(self), hwnd.into(), pmedia.into().abi(), pfallowplay).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowCDBurn<P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.allowCDBurn)(::windows::core::Vtable::as_raw(self), hwnd.into(), pplaylist.into().abi(), pfallowburn).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPDATransfer<P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.allowPDATransfer)(::windows::core::Vtable::as_raw(self), hwnd.into(), pplaylist.into().abi(), pfallowtransfer).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn startBackgroundProcessing<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.startBackgroundProcessing)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPSubscriptionServiceCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSubscriptionServiceCallback {}
impl ::core::fmt::Debug for IWMPSubscriptionServiceCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSubscriptionServiceCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPSyncDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSyncDevice {}
impl ::core::fmt::Debug for IWMPSyncDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSyncDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPSyncDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSyncDevice2 {}
impl ::core::fmt::Debug for IWMPSyncDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSyncDevice2").field(&self.0).finish()
    }
}
impl IWMPSyncDevice2 {
    pub unsafe fn friendlyName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.friendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn SetfriendlyName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetfriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn deviceName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.deviceName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn deviceId(&self, pbstrdeviceid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.deviceId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn partnershipIndex(&self, plindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.partnershipIndex)(::windows::core::Vtable::as_raw(self), plindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn connected(&self, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.connected)(::windows::core::Vtable::as_raw(self), pvbconnected).ok()
    }
    pub unsafe fn status(&self, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.status)(::windows::core::Vtable::as_raw(self), pwmpds).ok()
    }
    pub unsafe fn syncState(&self, pwmpss: *mut WMPSyncState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.syncState)(::windows::core::Vtable::as_raw(self), pwmpss).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.progress)(::windows::core::Vtable::as_raw(self), plprogress).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn createPartnership<P0>(&self, vbshowui: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.createPartnership)(::windows::core::Vtable::as_raw(self), vbshowui.into()).ok()
    }
    pub unsafe fn deletePartnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.deletePartnership)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn showSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.showSettings)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isIdentical<P0>(&self, pdevice: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.isIdentical)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pvbool).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPSyncDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSyncDevice3 {}
impl ::core::fmt::Debug for IWMPSyncDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSyncDevice3").field(&self.0).finish()
    }
}
impl IWMPSyncDevice3 {
    pub unsafe fn friendlyName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.friendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn SetfriendlyName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetfriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn deviceName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.deviceName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn deviceId(&self, pbstrdeviceid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.deviceId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn partnershipIndex(&self, plindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.partnershipIndex)(::windows::core::Vtable::as_raw(self), plindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn connected(&self, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.connected)(::windows::core::Vtable::as_raw(self), pvbconnected).ok()
    }
    pub unsafe fn status(&self, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.status)(::windows::core::Vtable::as_raw(self), pwmpds).ok()
    }
    pub unsafe fn syncState(&self, pwmpss: *mut WMPSyncState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.syncState)(::windows::core::Vtable::as_raw(self), pwmpss).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.progress)(::windows::core::Vtable::as_raw(self), plprogress).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn createPartnership<P0>(&self, vbshowui: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.createPartnership)(::windows::core::Vtable::as_raw(self), vbshowui.into()).ok()
    }
    pub unsafe fn deletePartnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.deletePartnership)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn showSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.showSettings)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isIdentical<P0>(&self, pdevice: P0, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.isIdentical)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pvbool).ok()
    }
    pub unsafe fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.setItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute_copy(bstrval)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPSyncServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSyncServices {}
impl ::core::fmt::Debug for IWMPSyncServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSyncServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPTranscodePolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPTranscodePolicy {}
impl ::core::fmt::Debug for IWMPTranscodePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPTranscodePolicy").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPUserEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPUserEventSink {}
impl ::core::fmt::Debug for IWMPUserEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPUserEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPVideoRenderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPVideoRenderConfig {}
impl ::core::fmt::Debug for IWMPVideoRenderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPVideoRenderConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPWindowMessageSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPWindowMessageSink {}
impl ::core::fmt::Debug for IWMPWindowMessageSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPWindowMessageSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXFeed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeed {}
impl ::core::fmt::Debug for IXFeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeed").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXFeed2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeed2 {}
impl ::core::fmt::Debug for IXFeed2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeed2").field(&self.0).finish()
    }
}
impl IXFeed2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Xml)(::windows::core::Vtable::as_raw(self), uiitemcount, sortproperty, sortorder, filterflags, includeflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Rename<P0>(&self, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Rename)(::windows::core::Vtable::as_raw(self), pszname.into().abi()).ok()
    }
    pub unsafe fn Url(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Url)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUrl<P0>(&self, pszurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUrl)(::windows::core::Vtable::as_raw(self), pszurl.into().abi()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move<P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), pszpath.into().abi()).ok()
    }
    pub unsafe fn Parent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastWriteTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastWriteTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Download)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelAsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows::core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SyncSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSyncSetting(&self, fss: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSyncSetting)(::windows::core::Vtable::as_raw(self), fss).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Interval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInterval(&self, uiinterval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInterval)(::windows::core::Vtable::as_raw(self), uiinterval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalEnclosurePath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Items(&self) -> ::windows::core::Result<IXFeedsEnum> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Items)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItem<T>(&self, uiid: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), uiid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MarkAllItemsRead)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MaxItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, uimaxitemcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaxItemCount)(::windows::core::Vtable::as_raw(self), uimaxitemcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDownloadEnclosuresAutomatically<P0>(&self, bdownloadenclosuresautomatically: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), bdownloadenclosuresautomatically.into()).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadError)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Merge<P0, P1>(&self, pstream: P0, pszurl: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Merge)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), pszurl.into().abi()).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Title)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Link)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Image(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Image)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastBuildDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastBuildDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PubDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Ttl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Language(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Language)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Copyright)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsList(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWatcher<T>(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWatcher)(::windows::core::Vtable::as_raw(self), scope, mask, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UnreadItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXFeedEnclosure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedEnclosure {}
impl ::core::fmt::Debug for IXFeedEnclosure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedEnclosure").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXFeedEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedEvents {}
impl ::core::fmt::Debug for IXFeedEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXFeedFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedFolder {}
impl ::core::fmt::Debug for IXFeedFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedFolder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXFeedFolderEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedFolderEvents {}
impl ::core::fmt::Debug for IXFeedFolderEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedFolderEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXFeedItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedItem {}
impl ::core::fmt::Debug for IXFeedItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXFeedItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedItem2 {}
impl ::core::fmt::Debug for IXFeedItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedItem2").field(&self.0).finish()
    }
}
impl IXFeedItem2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, fxif: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Xml)(::windows::core::Vtable::as_raw(self), fxif, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Title)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Link)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Guid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PubDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Comments)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Author(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Author)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Enclosure<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Enclosure)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRead(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsRead)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRead<P0>(&self, bisread: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIsRead)(::windows::core::Vtable::as_raw(self), bisread.into()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Modified(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Modified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXFeedsEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedsEnum {}
impl ::core::fmt::Debug for IXFeedsEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedsEnum").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXFeedsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedsManager {}
impl ::core::fmt::Debug for IXFeedsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedsManager").field(&self.0).finish()
    }
}
impl ::core::default::Default for PlayerState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PlayerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayerState").field(&self.0).finish()
    }
}
impl ::core::default::Default for TimedLevel {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TimedLevel {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency && self.waveform == other.waveform && self.state == other.state && self.timeStamp == other.timeStamp
    }
}
impl ::core::cmp::Eq for TimedLevel {}
impl ::core::fmt::Debug for TimedLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedLevel").field("frequency", &self.frequency).field("waveform", &self.waveform).field("state", &self.state).field("timeStamp", &self.timeStamp).finish()
    }
}
impl ::core::default::Default for WMPAccountType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPAccountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPAccountType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPBurnFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPBurnFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPBurnFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPBurnState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPBurnState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPBurnState").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPCallbackNotification {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPCallbackNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPCallbackNotification").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPContextMenuInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMPContextMenuInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwID == other.dwID && self.bstrMenuText == other.bstrMenuText && self.bstrHelpText == other.bstrHelpText
    }
}
impl ::core::cmp::Eq for WMPContextMenuInfo {}
impl ::core::fmt::Debug for WMPContextMenuInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMPContextMenuInfo").field("dwID", &self.dwID).field("bstrMenuText", &self.bstrMenuText).field("bstrHelpText", &self.bstrHelpText).finish()
    }
}
impl ::core::default::Default for WMPDeviceStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPDeviceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPDeviceStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPFolderScanState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPFolderScanState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPFolderScanState").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPLibraryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPLibraryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPLibraryType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPOpenState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPOpenState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPOpenState").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPPartnerNotification {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPPartnerNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPartnerNotification").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPPlayState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPPlayState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPlayState").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPPlaylistChangeEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPPlaylistChangeEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPlaylistChangeEventType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPPlugin_Caps {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPPlugin_Caps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPlugin_Caps").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPRipState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPRipState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPRipState").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPServices_StreamState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPServices_StreamState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPServices_StreamState").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPStreamingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPStreamingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPStreamingType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPStringCollectionChangeEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPStringCollectionChangeEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPStringCollectionChangeEventType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPSubscriptionDownloadState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPSubscriptionDownloadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPSubscriptionDownloadState").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPSubscriptionServiceEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPSubscriptionServiceEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPSubscriptionServiceEvent").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPSyncState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPSyncState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPSyncState").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPTaskType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPTaskType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPTaskType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPTemplateSize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPTemplateSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPTemplateSize").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMPTransactionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMPTransactionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPTransactionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _WMPOCXEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _WMPOCXEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _WMPOCXEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_WMPOCXEvents").field(&self.0).finish()
    }
}
