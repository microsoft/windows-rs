#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedImpl: Sized + IDispatchImpl {
    fn Xml();
    fn Name();
    fn Rename();
    fn Url();
    fn SetUrl();
    fn LocalId();
    fn Path();
    fn Move();
    fn Parent();
    fn LastWriteTime();
    fn Delete();
    fn Download();
    fn AsyncDownload();
    fn CancelAsyncDownload();
    fn SyncSetting();
    fn SetSyncSetting();
    fn Interval();
    fn SetInterval();
    fn LastDownloadTime();
    fn LocalEnclosurePath();
    fn Items();
    fn GetItem();
    fn Title();
    fn Description();
    fn Link();
    fn Image();
    fn LastBuildDate();
    fn PubDate();
    fn Ttl();
    fn Language();
    fn Copyright();
    fn MaxItemCount();
    fn SetMaxItemCount();
    fn DownloadEnclosuresAutomatically();
    fn SetDownloadEnclosuresAutomatically();
    fn DownloadStatus();
    fn LastDownloadError();
    fn Merge();
    fn DownloadUrl();
    fn IsList();
    fn MarkAllItemsRead();
    fn GetWatcher();
    fn UnreadItemCount();
    fn ItemCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedVtbl {
        unsafe extern "system" fn Xml<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rename<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Url<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUrl<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalId<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastWriteTime<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwrite: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Download<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncDownload<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncSetting<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSyncSetting<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Interval<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterval<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalEnclosurePath<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Items<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItem<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Title<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Link<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homepage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Image<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastBuildDate<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastbuilddate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PubDate<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastpopulatedate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ttl<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Language<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Copyright<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copyright: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxItemCount<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxItemCount<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadStatus<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastDownloadError<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Merge<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsList<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarkAllItemsRead<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWatcher<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnreadItemCount<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemCount<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Xml: Xml::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Rename: Rename::<Impl, IMPL_OFFSET>,
            Url: Url::<Impl, IMPL_OFFSET>,
            SetUrl: SetUrl::<Impl, IMPL_OFFSET>,
            LocalId: LocalId::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            LastWriteTime: LastWriteTime::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Download: Download::<Impl, IMPL_OFFSET>,
            AsyncDownload: AsyncDownload::<Impl, IMPL_OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Impl, IMPL_OFFSET>,
            SyncSetting: SyncSetting::<Impl, IMPL_OFFSET>,
            SetSyncSetting: SetSyncSetting::<Impl, IMPL_OFFSET>,
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
            LastDownloadTime: LastDownloadTime::<Impl, IMPL_OFFSET>,
            LocalEnclosurePath: LocalEnclosurePath::<Impl, IMPL_OFFSET>,
            Items: Items::<Impl, IMPL_OFFSET>,
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Link: Link::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            LastBuildDate: LastBuildDate::<Impl, IMPL_OFFSET>,
            PubDate: PubDate::<Impl, IMPL_OFFSET>,
            Ttl: Ttl::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            Copyright: Copyright::<Impl, IMPL_OFFSET>,
            MaxItemCount: MaxItemCount::<Impl, IMPL_OFFSET>,
            SetMaxItemCount: SetMaxItemCount::<Impl, IMPL_OFFSET>,
            DownloadEnclosuresAutomatically: DownloadEnclosuresAutomatically::<Impl, IMPL_OFFSET>,
            SetDownloadEnclosuresAutomatically: SetDownloadEnclosuresAutomatically::<Impl, IMPL_OFFSET>,
            DownloadStatus: DownloadStatus::<Impl, IMPL_OFFSET>,
            LastDownloadError: LastDownloadError::<Impl, IMPL_OFFSET>,
            Merge: Merge::<Impl, IMPL_OFFSET>,
            DownloadUrl: DownloadUrl::<Impl, IMPL_OFFSET>,
            IsList: IsList::<Impl, IMPL_OFFSET>,
            MarkAllItemsRead: MarkAllItemsRead::<Impl, IMPL_OFFSET>,
            GetWatcher: GetWatcher::<Impl, IMPL_OFFSET>,
            UnreadItemCount: UnreadItemCount::<Impl, IMPL_OFFSET>,
            ItemCount: ItemCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeed as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeed2Impl: Sized + IDispatchImpl + IFeedImpl {
    fn GetItemByEffectiveId();
    fn LastItemDownloadTime();
    fn Username();
    fn Password();
    fn SetCredentials();
    fn ClearCredentials();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeed2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeed2Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemeffectiveid: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastItemDownloadTime<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastitemdownloadtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Username<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Password<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCredentials<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearCredentials<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IFeedVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetItemByEffectiveId: GetItemByEffectiveId::<Impl, IMPL_OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Impl, IMPL_OFFSET>,
            Username: Username::<Impl, IMPL_OFFSET>,
            Password: Password::<Impl, IMPL_OFFSET>,
            SetCredentials: SetCredentials::<Impl, IMPL_OFFSET>,
            ClearCredentials: ClearCredentials::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeed2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedEnclosureImpl: Sized + IDispatchImpl {
    fn Url();
    fn Type();
    fn Length();
    fn AsyncDownload();
    fn CancelAsyncDownload();
    fn DownloadStatus();
    fn LastDownloadError();
    fn LocalPath();
    fn Parent();
    fn DownloadUrl();
    fn DownloadMimeType();
    fn RemoveFile();
    fn SetFile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedEnclosureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedEnclosureVtbl {
        unsafe extern "system" fn Url<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Length<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncDownload<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadStatus<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastDownloadError<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalPath<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadMimeType<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFile<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFile<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, downloadfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, downloadmimetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enclosurefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Url: Url::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            AsyncDownload: AsyncDownload::<Impl, IMPL_OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Impl, IMPL_OFFSET>,
            DownloadStatus: DownloadStatus::<Impl, IMPL_OFFSET>,
            LastDownloadError: LastDownloadError::<Impl, IMPL_OFFSET>,
            LocalPath: LocalPath::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            DownloadUrl: DownloadUrl::<Impl, IMPL_OFFSET>,
            DownloadMimeType: DownloadMimeType::<Impl, IMPL_OFFSET>,
            RemoveFile: RemoveFile::<Impl, IMPL_OFFSET>,
            SetFile: SetFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedEnclosure as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedEventsImpl: Sized + IDispatchImpl {
    fn Error();
    fn FeedDeleted();
    fn FeedRenamed();
    fn FeedUrlChanged();
    fn FeedMoved();
    fn FeedDownloading();
    fn FeedDownloadCompleted();
    fn FeedItemCountChanged();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedEventsVtbl {
        unsafe extern "system" fn Error<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDeleted<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedRenamed<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedMoved<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDownloading<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            FeedDeleted: FeedDeleted::<Impl, IMPL_OFFSET>,
            FeedRenamed: FeedRenamed::<Impl, IMPL_OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Impl, IMPL_OFFSET>,
            FeedMoved: FeedMoved::<Impl, IMPL_OFFSET>,
            FeedDownloading: FeedDownloading::<Impl, IMPL_OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Impl, IMPL_OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedFolderImpl: Sized + IDispatchImpl {
    fn Feeds();
    fn Subfolders();
    fn CreateFeed();
    fn CreateSubfolder();
    fn ExistsFeed();
    fn GetFeed();
    fn ExistsSubfolder();
    fn GetSubfolder();
    fn Delete();
    fn Name();
    fn Rename();
    fn Path();
    fn Move();
    fn Parent();
    fn IsRoot();
    fn TotalUnreadItemCount();
    fn TotalItemCount();
    fn GetWatcher();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedFolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedFolderVtbl {
        unsafe extern "system" fn Feeds<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Subfolders<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFeed<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSubfolder<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistsFeed<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeed<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistsSubfolder<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubfolder<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rename<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRoot<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isroot: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalUnreadItemCount<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalItemCount<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWatcher<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Feeds: Feeds::<Impl, IMPL_OFFSET>,
            Subfolders: Subfolders::<Impl, IMPL_OFFSET>,
            CreateFeed: CreateFeed::<Impl, IMPL_OFFSET>,
            CreateSubfolder: CreateSubfolder::<Impl, IMPL_OFFSET>,
            ExistsFeed: ExistsFeed::<Impl, IMPL_OFFSET>,
            GetFeed: GetFeed::<Impl, IMPL_OFFSET>,
            ExistsSubfolder: ExistsSubfolder::<Impl, IMPL_OFFSET>,
            GetSubfolder: GetSubfolder::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Rename: Rename::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            IsRoot: IsRoot::<Impl, IMPL_OFFSET>,
            TotalUnreadItemCount: TotalUnreadItemCount::<Impl, IMPL_OFFSET>,
            TotalItemCount: TotalItemCount::<Impl, IMPL_OFFSET>,
            GetWatcher: GetWatcher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedFolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedFolderEventsImpl: Sized + IDispatchImpl {
    fn Error();
    fn FolderAdded();
    fn FolderDeleted();
    fn FolderRenamed();
    fn FolderMovedFrom();
    fn FolderMovedTo();
    fn FolderItemCountChanged();
    fn FeedAdded();
    fn FeedDeleted();
    fn FeedRenamed();
    fn FeedUrlChanged();
    fn FeedMovedFrom();
    fn FeedMovedTo();
    fn FeedDownloading();
    fn FeedDownloadCompleted();
    fn FeedItemCountChanged();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedFolderEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedFolderEventsVtbl {
        unsafe extern "system" fn Error<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderAdded<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderDeleted<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderRenamed<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderMovedFrom<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderMovedTo<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderItemCountChanged<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedAdded<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDeleted<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedRenamed<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedMovedFrom<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedMovedTo<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDownloading<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            FolderAdded: FolderAdded::<Impl, IMPL_OFFSET>,
            FolderDeleted: FolderDeleted::<Impl, IMPL_OFFSET>,
            FolderRenamed: FolderRenamed::<Impl, IMPL_OFFSET>,
            FolderMovedFrom: FolderMovedFrom::<Impl, IMPL_OFFSET>,
            FolderMovedTo: FolderMovedTo::<Impl, IMPL_OFFSET>,
            FolderItemCountChanged: FolderItemCountChanged::<Impl, IMPL_OFFSET>,
            FeedAdded: FeedAdded::<Impl, IMPL_OFFSET>,
            FeedDeleted: FeedDeleted::<Impl, IMPL_OFFSET>,
            FeedRenamed: FeedRenamed::<Impl, IMPL_OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Impl, IMPL_OFFSET>,
            FeedMovedFrom: FeedMovedFrom::<Impl, IMPL_OFFSET>,
            FeedMovedTo: FeedMovedTo::<Impl, IMPL_OFFSET>,
            FeedDownloading: FeedDownloading::<Impl, IMPL_OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Impl, IMPL_OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedFolderEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedItemImpl: Sized + IDispatchImpl {
    fn Xml();
    fn Title();
    fn Link();
    fn Guid();
    fn Description();
    fn PubDate();
    fn Comments();
    fn Author();
    fn Enclosure();
    fn IsRead();
    fn SetIsRead();
    fn LocalId();
    fn Parent();
    fn Delete();
    fn DownloadUrl();
    fn LastDownloadTime();
    fn Modified();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedItemVtbl {
        unsafe extern "system" fn Xml<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Title<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Link<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linkurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Guid<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PubDate<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pubdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Comments<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comments: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Author<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, author: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enclosure<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRead<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsRead<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalId<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Modified<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Xml: Xml::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Link: Link::<Impl, IMPL_OFFSET>,
            Guid: Guid::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            PubDate: PubDate::<Impl, IMPL_OFFSET>,
            Comments: Comments::<Impl, IMPL_OFFSET>,
            Author: Author::<Impl, IMPL_OFFSET>,
            Enclosure: Enclosure::<Impl, IMPL_OFFSET>,
            IsRead: IsRead::<Impl, IMPL_OFFSET>,
            SetIsRead: SetIsRead::<Impl, IMPL_OFFSET>,
            LocalId: LocalId::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            DownloadUrl: DownloadUrl::<Impl, IMPL_OFFSET>,
            LastDownloadTime: LastDownloadTime::<Impl, IMPL_OFFSET>,
            Modified: Modified::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedItem2Impl: Sized + IDispatchImpl + IFeedItemImpl {
    fn EffectiveId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedItem2Vtbl {
        unsafe extern "system" fn EffectiveId<Impl: IFeedItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectiveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IFeedItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EffectiveId: EffectiveId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedsEnumImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedsEnumVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsEnumImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedsEnumVtbl {
        unsafe extern "system" fn Count<Impl: IFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedsEnum as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedsManagerImpl: Sized + IDispatchImpl {
    fn RootFolder();
    fn IsSubscribed();
    fn ExistsFeed();
    fn GetFeed();
    fn GetFeedByUrl();
    fn ExistsFolder();
    fn GetFolder();
    fn DeleteFeed();
    fn DeleteFolder();
    fn BackgroundSync();
    fn BackgroundSyncStatus();
    fn DefaultInterval();
    fn SetDefaultInterval();
    fn AsyncSyncAll();
    fn Normalize();
    fn ItemCountLimit();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedsManagerVtbl {
        unsafe extern "system" fn RootFolder<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSubscribed<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subscribed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistsFeed<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeed<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeedByUrl<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistsFolder<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFolder<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteFeed<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteFolder<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackgroundSync<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultInterval<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultInterval<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncSyncAll<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Normalize<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxmlin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedxmlout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemCountLimit<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemcountlimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RootFolder: RootFolder::<Impl, IMPL_OFFSET>,
            IsSubscribed: IsSubscribed::<Impl, IMPL_OFFSET>,
            ExistsFeed: ExistsFeed::<Impl, IMPL_OFFSET>,
            GetFeed: GetFeed::<Impl, IMPL_OFFSET>,
            GetFeedByUrl: GetFeedByUrl::<Impl, IMPL_OFFSET>,
            ExistsFolder: ExistsFolder::<Impl, IMPL_OFFSET>,
            GetFolder: GetFolder::<Impl, IMPL_OFFSET>,
            DeleteFeed: DeleteFeed::<Impl, IMPL_OFFSET>,
            DeleteFolder: DeleteFolder::<Impl, IMPL_OFFSET>,
            BackgroundSync: BackgroundSync::<Impl, IMPL_OFFSET>,
            BackgroundSyncStatus: BackgroundSyncStatus::<Impl, IMPL_OFFSET>,
            DefaultInterval: DefaultInterval::<Impl, IMPL_OFFSET>,
            SetDefaultInterval: SetDefaultInterval::<Impl, IMPL_OFFSET>,
            AsyncSyncAll: AsyncSyncAll::<Impl, IMPL_OFFSET>,
            Normalize: Normalize::<Impl, IMPL_OFFSET>,
            ItemCountLimit: ItemCountLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedsManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPAudioRenderConfigImpl: Sized {
    fn audioOutputDevice();
    fn SetaudioOutputDevice();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPAudioRenderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPAudioRenderConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPAudioRenderConfigVtbl {
        unsafe extern "system" fn audioOutputDevice<Impl: IWMPAudioRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstroutputdevice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetaudioOutputDevice<Impl: IWMPAudioRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroutputdevice: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            audioOutputDevice: audioOutputDevice::<Impl, IMPL_OFFSET>,
            SetaudioOutputDevice: SetaudioOutputDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPAudioRenderConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCdromImpl: Sized + IDispatchImpl {
    fn driveSpecifier();
    fn playlist();
    fn eject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCdromVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCdromVtbl {
        unsafe extern "system" fn driveSpecifier<Impl: IWMPCdromImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdrive: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn playlist<Impl: IWMPCdromImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn eject<Impl: IWMPCdromImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            driveSpecifier: driveSpecifier::<Impl, IMPL_OFFSET>,
            playlist: playlist::<Impl, IMPL_OFFSET>,
            eject: eject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdrom as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPCdromBurnImpl: Sized {
    fn isAvailable();
    fn getItemInfo();
    fn label();
    fn Setlabel();
    fn burnFormat();
    fn SetburnFormat();
    fn burnPlaylist();
    fn SetburnPlaylist();
    fn refreshStatus();
    fn burnState();
    fn burnProgress();
    fn startBurn();
    fn stopBurn();
    fn erase();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPCdromBurnVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurnImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCdromBurnVtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn label<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setlabel<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn burnFormat<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetburnFormat<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpbf: WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn burnPlaylist<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetburnPlaylist<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn refreshStatus<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn burnState<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn burnProgress<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startBurn<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stopBurn<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn erase<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            isAvailable: isAvailable::<Impl, IMPL_OFFSET>,
            getItemInfo: getItemInfo::<Impl, IMPL_OFFSET>,
            label: label::<Impl, IMPL_OFFSET>,
            Setlabel: Setlabel::<Impl, IMPL_OFFSET>,
            burnFormat: burnFormat::<Impl, IMPL_OFFSET>,
            SetburnFormat: SetburnFormat::<Impl, IMPL_OFFSET>,
            burnPlaylist: burnPlaylist::<Impl, IMPL_OFFSET>,
            SetburnPlaylist: SetburnPlaylist::<Impl, IMPL_OFFSET>,
            refreshStatus: refreshStatus::<Impl, IMPL_OFFSET>,
            burnState: burnState::<Impl, IMPL_OFFSET>,
            burnProgress: burnProgress::<Impl, IMPL_OFFSET>,
            startBurn: startBurn::<Impl, IMPL_OFFSET>,
            stopBurn: stopBurn::<Impl, IMPL_OFFSET>,
            erase: erase::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdromBurn as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCdromCollectionImpl: Sized + IDispatchImpl {
    fn count();
    fn item();
    fn getByDriveSpecifier();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCdromCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCdromCollectionVtbl {
        unsafe extern "system" fn count<Impl: IWMPCdromCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn item<Impl: IWMPCdromCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getByDriveSpecifier<Impl: IWMPCdromCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdrivespecifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcdrom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            count: count::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
            getByDriveSpecifier: getByDriveSpecifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdromCollection as ::windows::core::Interface>::IID
    }
}
pub trait IWMPCdromRipImpl: Sized {
    fn ripState();
    fn ripProgress();
    fn startRip();
    fn stopRip();
}
impl IWMPCdromRipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromRipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCdromRipVtbl {
        unsafe extern "system" fn ripState<Impl: IWMPCdromRipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmprs: *mut WMPRipState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ripProgress<Impl: IWMPCdromRipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startRip<Impl: IWMPCdromRipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stopRip<Impl: IWMPCdromRipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ripState: ripState::<Impl, IMPL_OFFSET>,
            ripProgress: ripProgress::<Impl, IMPL_OFFSET>,
            startRip: startRip::<Impl, IMPL_OFFSET>,
            stopRip: stopRip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdromRip as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPClosedCaptionImpl: Sized + IDispatchImpl {
    fn SAMIStyle();
    fn SetSAMIStyle();
    fn SAMILang();
    fn SetSAMILang();
    fn SAMIFileName();
    fn SetSAMIFileName();
    fn captioningId();
    fn SetcaptioningId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPClosedCaptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPClosedCaptionVtbl {
        unsafe extern "system" fn SAMIStyle<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamistyle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSAMIStyle<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamistyle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SAMILang<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamilang: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSAMILang<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamilang: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SAMIFileName<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamifilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSAMIFileName<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamifilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn captioningId<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcaptioningid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetcaptioningId<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaptioningid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SAMIStyle: SAMIStyle::<Impl, IMPL_OFFSET>,
            SetSAMIStyle: SetSAMIStyle::<Impl, IMPL_OFFSET>,
            SAMILang: SAMILang::<Impl, IMPL_OFFSET>,
            SetSAMILang: SetSAMILang::<Impl, IMPL_OFFSET>,
            SAMIFileName: SAMIFileName::<Impl, IMPL_OFFSET>,
            SetSAMIFileName: SetSAMIFileName::<Impl, IMPL_OFFSET>,
            captioningId: captioningId::<Impl, IMPL_OFFSET>,
            SetcaptioningId: SetcaptioningId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPClosedCaption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPClosedCaption2Impl: Sized + IDispatchImpl + IWMPClosedCaptionImpl {
    fn SAMILangCount();
    fn getSAMILangName();
    fn getSAMILangID();
    fn SAMIStyleCount();
    fn getSAMIStyleName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPClosedCaption2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPClosedCaption2Vtbl {
        unsafe extern "system" fn SAMILangCount<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getSAMILangName<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getSAMILangID<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SAMIStyleCount<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getSAMIStyleName<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPClosedCaptionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SAMILangCount: SAMILangCount::<Impl, IMPL_OFFSET>,
            getSAMILangName: getSAMILangName::<Impl, IMPL_OFFSET>,
            getSAMILangID: getSAMILangID::<Impl, IMPL_OFFSET>,
            SAMIStyleCount: SAMIStyleCount::<Impl, IMPL_OFFSET>,
            getSAMIStyleName: getSAMIStyleName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPClosedCaption2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPContentContainerImpl: Sized {
    fn GetID();
    fn GetPrice();
    fn GetType();
    fn GetContentCount();
    fn GetContentPrice();
    fn GetContentID();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPContentContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPContentContainerVtbl {
        unsafe extern "system" fn GetID<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrice<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentCount<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentPrice<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pbstrprice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentID<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetID: GetID::<Impl, IMPL_OFFSET>,
            GetPrice: GetPrice::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetContentCount: GetContentCount::<Impl, IMPL_OFFSET>,
            GetContentPrice: GetContentPrice::<Impl, IMPL_OFFSET>,
            GetContentID: GetContentID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPContentContainer as ::windows::core::Interface>::IID
    }
}
pub trait IWMPContentContainerListImpl: Sized {
    fn GetTransactionType();
    fn GetContainerCount();
    fn GetContainer();
}
impl IWMPContentContainerListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainerListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPContentContainerListVtbl {
        unsafe extern "system" fn GetTransactionType<Impl: IWMPContentContainerListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmptt: *mut WMPTransactionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainerCount<Impl: IWMPContentContainerListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontainer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainer<Impl: IWMPContentContainerListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontainer: u32, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTransactionType: GetTransactionType::<Impl, IMPL_OFFSET>,
            GetContainerCount: GetContainerCount::<Impl, IMPL_OFFSET>,
            GetContainer: GetContainer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPContentContainerList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPContentPartnerImpl: Sized {
    fn SetCallback();
    fn Notify();
    fn GetItemInfo();
    fn GetContentPartnerInfo();
    fn GetCommands();
    fn InvokeCommand();
    fn CanBuySilent();
    fn Buy();
    fn GetStreamingURL();
    fn Download();
    fn DownloadTrackComplete();
    fn RefreshLicense();
    fn GetCatalogURL();
    fn GetTemplate();
    fn UpdateDevice();
    fn GetListContents();
    fn Login();
    fn Authenticate();
    fn Logout();
    fn SendMessage();
    fn StationEvent();
    fn CompareContainerListPrices();
    fn VerifyPermission();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPContentPartnerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPContentPartnerVtbl {
        unsafe extern "system" fn SetCallback<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemInfo<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentPartnerInfo<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCommands<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeCommand<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcommandid: u32, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, citemids: u32, rgitemids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanBuySilent<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, pbstrtotalprice: *mut super::super::Foundation::BSTR, psilentok: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Buy<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamingURL<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, st: WMPStreamingType, pstreamcontext: *const super::super::System::Com::VARIANT, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Download<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadTrackComplete<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshLicense<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, flocal: i16, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, preasoncontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCatalogURL<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut super::super::Foundation::BSTR, pexpirationdate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTemplate<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: WMPTaskType, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, clicklocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pclickcontext: *const super::super::System::Com::VARIANT, bstrfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrviewparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrtemplateurl: *mut super::super::Foundation::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateDevice<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetListContents<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, bstrlisttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwlistcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Login<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: i16, foktocache: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Authenticate<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Logout<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendMessage<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StationEvent<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstationeventtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, stationid: u32, playlistindex: u32, trackid: u32, trackdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsecondsplayed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareContainerListPrices<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plistbase: ::windows::core::RawPtr, plistcompare: ::windows::core::RawPtr, presult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VerifyPermission<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetCallback: SetCallback::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
            GetItemInfo: GetItemInfo::<Impl, IMPL_OFFSET>,
            GetContentPartnerInfo: GetContentPartnerInfo::<Impl, IMPL_OFFSET>,
            GetCommands: GetCommands::<Impl, IMPL_OFFSET>,
            InvokeCommand: InvokeCommand::<Impl, IMPL_OFFSET>,
            CanBuySilent: CanBuySilent::<Impl, IMPL_OFFSET>,
            Buy: Buy::<Impl, IMPL_OFFSET>,
            GetStreamingURL: GetStreamingURL::<Impl, IMPL_OFFSET>,
            Download: Download::<Impl, IMPL_OFFSET>,
            DownloadTrackComplete: DownloadTrackComplete::<Impl, IMPL_OFFSET>,
            RefreshLicense: RefreshLicense::<Impl, IMPL_OFFSET>,
            GetCatalogURL: GetCatalogURL::<Impl, IMPL_OFFSET>,
            GetTemplate: GetTemplate::<Impl, IMPL_OFFSET>,
            UpdateDevice: UpdateDevice::<Impl, IMPL_OFFSET>,
            GetListContents: GetListContents::<Impl, IMPL_OFFSET>,
            Login: Login::<Impl, IMPL_OFFSET>,
            Authenticate: Authenticate::<Impl, IMPL_OFFSET>,
            Logout: Logout::<Impl, IMPL_OFFSET>,
            SendMessage: SendMessage::<Impl, IMPL_OFFSET>,
            StationEvent: StationEvent::<Impl, IMPL_OFFSET>,
            CompareContainerListPrices: CompareContainerListPrices::<Impl, IMPL_OFFSET>,
            VerifyPermission: VerifyPermission::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPContentPartner as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPContentPartnerCallbackImpl: Sized {
    fn Notify();
    fn BuyComplete();
    fn DownloadTrack();
    fn GetCatalogVersion();
    fn UpdateDeviceComplete();
    fn ChangeView();
    fn AddListContents();
    fn ListContentsComplete();
    fn SendMessageComplete();
    fn GetContentIDsInLibrary();
    fn RefreshLicenseComplete();
    fn ShowPopup();
    fn VerifyPermissionComplete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPContentPartnerCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPContentPartnerCallbackVtbl {
        unsafe extern "system" fn Notify<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BuyComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwbuycookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadTrack<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32, bstrtrackurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwservicetrackid: u32, bstrdownloadparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrdownload: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCatalogVersion<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateDeviceComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangeView<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddListContents<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ListContentsComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendMessageComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentIDsInLibrary<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshLicenseComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowPopup<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VerifyPermissionComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, hrpermission: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Notify: Notify::<Impl, IMPL_OFFSET>,
            BuyComplete: BuyComplete::<Impl, IMPL_OFFSET>,
            DownloadTrack: DownloadTrack::<Impl, IMPL_OFFSET>,
            GetCatalogVersion: GetCatalogVersion::<Impl, IMPL_OFFSET>,
            UpdateDeviceComplete: UpdateDeviceComplete::<Impl, IMPL_OFFSET>,
            ChangeView: ChangeView::<Impl, IMPL_OFFSET>,
            AddListContents: AddListContents::<Impl, IMPL_OFFSET>,
            ListContentsComplete: ListContentsComplete::<Impl, IMPL_OFFSET>,
            SendMessageComplete: SendMessageComplete::<Impl, IMPL_OFFSET>,
            GetContentIDsInLibrary: GetContentIDsInLibrary::<Impl, IMPL_OFFSET>,
            RefreshLicenseComplete: RefreshLicenseComplete::<Impl, IMPL_OFFSET>,
            ShowPopup: ShowPopup::<Impl, IMPL_OFFSET>,
            VerifyPermissionComplete: VerifyPermissionComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPContentPartnerCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPControlsImpl: Sized + IDispatchImpl {
    fn isAvailable();
    fn play();
    fn stop();
    fn pause();
    fn fastForward();
    fn fastReverse();
    fn currentPosition();
    fn SetcurrentPosition();
    fn currentPositionString();
    fn next();
    fn previous();
    fn currentItem();
    fn SetcurrentItem();
    fn currentMarker();
    fn SetcurrentMarker();
    fn playItem();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControlsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControlsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPControlsVtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn play<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stop<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn pause<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fastForward<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fastReverse<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentPosition<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcurrentposition: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetcurrentPosition<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dcurrentposition: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentPositionString<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurrentposition: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn next<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn previous<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentItem<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetcurrentItem<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentMarker<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmarker: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetcurrentMarker<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmarker: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn playItem<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            isAvailable: isAvailable::<Impl, IMPL_OFFSET>,
            play: play::<Impl, IMPL_OFFSET>,
            stop: stop::<Impl, IMPL_OFFSET>,
            pause: pause::<Impl, IMPL_OFFSET>,
            fastForward: fastForward::<Impl, IMPL_OFFSET>,
            fastReverse: fastReverse::<Impl, IMPL_OFFSET>,
            currentPosition: currentPosition::<Impl, IMPL_OFFSET>,
            SetcurrentPosition: SetcurrentPosition::<Impl, IMPL_OFFSET>,
            currentPositionString: currentPositionString::<Impl, IMPL_OFFSET>,
            next: next::<Impl, IMPL_OFFSET>,
            previous: previous::<Impl, IMPL_OFFSET>,
            currentItem: currentItem::<Impl, IMPL_OFFSET>,
            SetcurrentItem: SetcurrentItem::<Impl, IMPL_OFFSET>,
            currentMarker: currentMarker::<Impl, IMPL_OFFSET>,
            SetcurrentMarker: SetcurrentMarker::<Impl, IMPL_OFFSET>,
            playItem: playItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPControls as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPControls2Impl: Sized + IDispatchImpl + IWMPControlsImpl {
    fn step();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPControls2Vtbl {
        unsafe extern "system" fn step<Impl: IWMPControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstep: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMPControlsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), step: step::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPControls2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPControls3Impl: Sized + IDispatchImpl + IWMPControlsImpl + IWMPControls2Impl {
    fn audioLanguageCount();
    fn getAudioLanguageID();
    fn getAudioLanguageDescription();
    fn currentAudioLanguage();
    fn SetcurrentAudioLanguage();
    fn currentAudioLanguageIndex();
    fn SetcurrentAudioLanguageIndex();
    fn getLanguageName();
    fn currentPositionTimecode();
    fn SetcurrentPositionTimecode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPControls3Vtbl {
        unsafe extern "system" fn audioLanguageCount<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAudioLanguageID<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAudioLanguageDescription<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentAudioLanguage<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetcurrentAudioLanguage<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentAudioLanguageIndex<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetcurrentAudioLanguageIndex<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getLanguageName<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32, pbstrlangname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentPositionTimecode<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetcurrentPositionTimecode<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPControls2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            audioLanguageCount: audioLanguageCount::<Impl, IMPL_OFFSET>,
            getAudioLanguageID: getAudioLanguageID::<Impl, IMPL_OFFSET>,
            getAudioLanguageDescription: getAudioLanguageDescription::<Impl, IMPL_OFFSET>,
            currentAudioLanguage: currentAudioLanguage::<Impl, IMPL_OFFSET>,
            SetcurrentAudioLanguage: SetcurrentAudioLanguage::<Impl, IMPL_OFFSET>,
            currentAudioLanguageIndex: currentAudioLanguageIndex::<Impl, IMPL_OFFSET>,
            SetcurrentAudioLanguageIndex: SetcurrentAudioLanguageIndex::<Impl, IMPL_OFFSET>,
            getLanguageName: getLanguageName::<Impl, IMPL_OFFSET>,
            currentPositionTimecode: currentPositionTimecode::<Impl, IMPL_OFFSET>,
            SetcurrentPositionTimecode: SetcurrentPositionTimecode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPControls3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPConvertImpl: Sized {
    fn ConvertFile();
    fn GetErrorURL();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPConvertVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPConvertImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPConvertVtbl {
        unsafe extern "system" fn ConvertFile<Impl: IWMPConvertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinputfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstroutputfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorURL<Impl: IWMPConvertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConvertFile: ConvertFile::<Impl, IMPL_OFFSET>,
            GetErrorURL: GetErrorURL::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPConvert as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCoreImpl: Sized + IDispatchImpl {
    fn close();
    fn URL();
    fn SetURL();
    fn openState();
    fn playState();
    fn controls();
    fn settings();
    fn currentMedia();
    fn SetcurrentMedia();
    fn mediaCollection();
    fn playlistCollection();
    fn versionInfo();
    fn launchURL();
    fn network();
    fn currentPlaylist();
    fn SetcurrentPlaylist();
    fn cdromCollection();
    fn closedCaption();
    fn isOnline();
    fn error();
    fn status();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCoreVtbl {
        unsafe extern "system" fn close<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn URL<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetURL<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn openState<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpos: *mut WMPOpenState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn playState<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpps: *mut WMPPlayState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn controls<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn settings<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentMedia<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetcurrentMedia<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn mediaCollection<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediacollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn playlistCollection<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn versionInfo<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrversioninfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn launchURL<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn network<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqni: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentPlaylist<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetcurrentPlaylist<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn cdromCollection<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcdromcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn closedCaption<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclosedcaption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isOnline<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfonline: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn error<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn status<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            close: close::<Impl, IMPL_OFFSET>,
            URL: URL::<Impl, IMPL_OFFSET>,
            SetURL: SetURL::<Impl, IMPL_OFFSET>,
            openState: openState::<Impl, IMPL_OFFSET>,
            playState: playState::<Impl, IMPL_OFFSET>,
            controls: controls::<Impl, IMPL_OFFSET>,
            settings: settings::<Impl, IMPL_OFFSET>,
            currentMedia: currentMedia::<Impl, IMPL_OFFSET>,
            SetcurrentMedia: SetcurrentMedia::<Impl, IMPL_OFFSET>,
            mediaCollection: mediaCollection::<Impl, IMPL_OFFSET>,
            playlistCollection: playlistCollection::<Impl, IMPL_OFFSET>,
            versionInfo: versionInfo::<Impl, IMPL_OFFSET>,
            launchURL: launchURL::<Impl, IMPL_OFFSET>,
            network: network::<Impl, IMPL_OFFSET>,
            currentPlaylist: currentPlaylist::<Impl, IMPL_OFFSET>,
            SetcurrentPlaylist: SetcurrentPlaylist::<Impl, IMPL_OFFSET>,
            cdromCollection: cdromCollection::<Impl, IMPL_OFFSET>,
            closedCaption: closedCaption::<Impl, IMPL_OFFSET>,
            isOnline: isOnline::<Impl, IMPL_OFFSET>,
            error: error::<Impl, IMPL_OFFSET>,
            status: status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCore2Impl: Sized + IDispatchImpl + IWMPCoreImpl {
    fn dvd();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCore2Vtbl {
        unsafe extern "system" fn dvd<Impl: IWMPCore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdvd: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMPCoreVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), dvd: dvd::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCore3Impl: Sized + IDispatchImpl + IWMPCoreImpl + IWMPCore2Impl {
    fn newPlaylist();
    fn newMedia();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCore3Vtbl {
        unsafe extern "system" fn newPlaylist<Impl: IWMPCore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn newMedia<Impl: IWMPCore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPCore2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            newPlaylist: newPlaylist::<Impl, IMPL_OFFSET>,
            newMedia: newMedia::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDVDImpl: Sized + IDispatchImpl {
    fn isAvailable();
    fn domain();
    fn topMenu();
    fn titleMenu();
    fn back();
    fn resume();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDVDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVDImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDVDVtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn domain<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn topMenu<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn titleMenu<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn back<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn resume<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            isAvailable: isAvailable::<Impl, IMPL_OFFSET>,
            domain: domain::<Impl, IMPL_OFFSET>,
            topMenu: topMenu::<Impl, IMPL_OFFSET>,
            titleMenu: titleMenu::<Impl, IMPL_OFFSET>,
            back: back::<Impl, IMPL_OFFSET>,
            resume: resume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDVD as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadCollectionImpl: Sized + IDispatchImpl {
    fn id();
    fn count();
    fn item();
    fn startDownload();
    fn removeItem();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDownloadCollectionVtbl {
        unsafe extern "system" fn id<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn count<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn item<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32, ppdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startDownload<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeItem<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            id: id::<Impl, IMPL_OFFSET>,
            count: count::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
            startDownload: startDownload::<Impl, IMPL_OFFSET>,
            removeItem: removeItem::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadItemImpl: Sized + IDispatchImpl {
    fn sourceURL();
    fn size();
    fn r#type();
    fn progress();
    fn downloadState();
    fn pause();
    fn resume();
    fn cancel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDownloadItemVtbl {
        unsafe extern "system" fn sourceURL<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn size<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn r#type<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn progress<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn downloadState<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn pause<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn resume<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn cancel<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            sourceURL: sourceURL::<Impl, IMPL_OFFSET>,
            size: size::<Impl, IMPL_OFFSET>,
            r#type: r#type::<Impl, IMPL_OFFSET>,
            progress: progress::<Impl, IMPL_OFFSET>,
            downloadState: downloadState::<Impl, IMPL_OFFSET>,
            pause: pause::<Impl, IMPL_OFFSET>,
            resume: resume::<Impl, IMPL_OFFSET>,
            cancel: cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadItem2Impl: Sized + IDispatchImpl + IWMPDownloadItemImpl {
    fn getItemInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDownloadItem2Vtbl {
        unsafe extern "system" fn getItemInfo<Impl: IWMPDownloadItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMPDownloadItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), getItemInfo: getItemInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadManagerImpl: Sized + IDispatchImpl {
    fn getDownloadCollection();
    fn createDownloadCollection();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDownloadManagerVtbl {
        unsafe extern "system" fn getDownloadCollection<Impl: IWMPDownloadManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionid: i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createDownloadCollection<Impl: IWMPDownloadManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            getDownloadCollection: getDownloadCollection::<Impl, IMPL_OFFSET>,
            createDownloadCollection: createDownloadCollection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IWMPEffectsImpl: Sized {
    fn Render();
    fn MediaInfo();
    fn GetCapabilities();
    fn GetTitle();
    fn GetPresetTitle();
    fn GetPresetCount();
    fn SetCurrentPreset();
    fn GetCurrentPreset();
    fn DisplayPropertyPage();
    fn GoFullscreen();
    fn RenderFullScreen();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IWMPEffectsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffectsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEffectsVtbl {
        unsafe extern "system" fn Render<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaInfo<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lchannelcount: i32, lsamplerate: i32, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCapabilities<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTitle<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresetTitle<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32, bstrpresettitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresetCount<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpresetcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentPreset<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPreset<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpreset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayPropertyPage<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GoFullscreen<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenderFullScreen<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Render: Render::<Impl, IMPL_OFFSET>,
            MediaInfo: MediaInfo::<Impl, IMPL_OFFSET>,
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            GetTitle: GetTitle::<Impl, IMPL_OFFSET>,
            GetPresetTitle: GetPresetTitle::<Impl, IMPL_OFFSET>,
            GetPresetCount: GetPresetCount::<Impl, IMPL_OFFSET>,
            SetCurrentPreset: SetCurrentPreset::<Impl, IMPL_OFFSET>,
            GetCurrentPreset: GetCurrentPreset::<Impl, IMPL_OFFSET>,
            DisplayPropertyPage: DisplayPropertyPage::<Impl, IMPL_OFFSET>,
            GoFullscreen: GoFullscreen::<Impl, IMPL_OFFSET>,
            RenderFullScreen: RenderFullScreen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEffects as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IWMPEffects2Impl: Sized + IWMPEffectsImpl {
    fn SetCore();
    fn Create();
    fn Destroy();
    fn NotifyNewMedia();
    fn OnWindowMessage();
    fn RenderWindowed();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IWMPEffects2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEffects2Vtbl {
        unsafe extern "system" fn SetCore<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destroy<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyNewMedia<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnWindowMessage<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenderWindowed<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPEffectsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetCore: SetCore::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Destroy: Destroy::<Impl, IMPL_OFFSET>,
            NotifyNewMedia: NotifyNewMedia::<Impl, IMPL_OFFSET>,
            OnWindowMessage: OnWindowMessage::<Impl, IMPL_OFFSET>,
            RenderWindowed: RenderWindowed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEffects2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPErrorImpl: Sized + IDispatchImpl {
    fn clearErrorQueue();
    fn errorCount();
    fn item();
    fn webHelp();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPErrorVtbl {
        unsafe extern "system" fn clearErrorQueue<Impl: IWMPErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn errorCount<Impl: IWMPErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnumerrors: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn item<Impl: IWMPErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, pperroritem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn webHelp<Impl: IWMPErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            clearErrorQueue: clearErrorQueue::<Impl, IMPL_OFFSET>,
            errorCount: errorCount::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
            webHelp: webHelp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPErrorItemImpl: Sized + IDispatchImpl {
    fn errorCode();
    fn errorDescription();
    fn errorContext();
    fn remedy();
    fn customUrl();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPErrorItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPErrorItemVtbl {
        unsafe extern "system" fn errorCode<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn errorDescription<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn errorContext<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn remedy<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plremedy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn customUrl<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcustomurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            errorCode: errorCode::<Impl, IMPL_OFFSET>,
            errorDescription: errorDescription::<Impl, IMPL_OFFSET>,
            errorContext: errorContext::<Impl, IMPL_OFFSET>,
            remedy: remedy::<Impl, IMPL_OFFSET>,
            customUrl: customUrl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPErrorItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPErrorItem2Impl: Sized + IDispatchImpl + IWMPErrorItemImpl {
    fn condition();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPErrorItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPErrorItem2Vtbl {
        unsafe extern "system" fn condition<Impl: IWMPErrorItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcondition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMPErrorItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), condition: condition::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPErrorItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEventsImpl: Sized {
    fn OpenStateChange();
    fn PlayStateChange();
    fn AudioLanguageChange();
    fn StatusChange();
    fn ScriptCommand();
    fn NewStream();
    fn Disconnect();
    fn Buffering();
    fn Error();
    fn Warning();
    fn EndOfStream();
    fn PositionChange();
    fn MarkerHit();
    fn DurationUnitChange();
    fn CdromMediaChange();
    fn PlaylistChange();
    fn CurrentPlaylistChange();
    fn CurrentPlaylistItemAvailable();
    fn MediaChange();
    fn CurrentMediaItemAvailable();
    fn CurrentItemChange();
    fn MediaCollectionChange();
    fn MediaCollectionAttributeStringAdded();
    fn MediaCollectionAttributeStringRemoved();
    fn MediaCollectionAttributeStringChanged();
    fn PlaylistCollectionChange();
    fn PlaylistCollectionPlaylistAdded();
    fn PlaylistCollectionPlaylistRemoved();
    fn PlaylistCollectionPlaylistSetAsDeleted();
    fn ModeChange();
    fn MediaError();
    fn OpenPlaylistSwitch();
    fn DomainChange();
    fn SwitchedToPlayerApplication();
    fn SwitchedToControl();
    fn PlayerDockedStateChange();
    fn PlayerReconnect();
    fn Click();
    fn DoubleClick();
    fn KeyDown();
    fn KeyPress();
    fn KeyUp();
    fn MouseDown();
    fn MouseMove();
    fn MouseUp();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEventsVtbl {
        unsafe extern "system" fn OpenStateChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlayStateChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AudioLanguageChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScriptCommand<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sctype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, param: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NewStream<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Buffering<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Warning<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, warningtype: i32, param: i32, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndOfStream<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PositionChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldposition: f64, newposition: f64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarkerHit<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DurationUnitChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newdurationunit: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CdromMediaChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdromnum: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlaylistChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, playlist: ::windows::core::RawPtr, change: WMPPlaylistChangeEventType) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentPlaylistChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentPlaylistItemAvailable<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentMediaItemAvailable<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentItemChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaCollectionChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaCollectionAttributeStringAdded<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaCollectionAttributeStringRemoved<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaCollectionAttributeStringChanged<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstroldattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlaylistCollectionChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistAdded<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistRemoved<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistSetAsDeleted<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varfisdeleted: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModeChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newvalue: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaError<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediaobject: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenPlaylistSwitch<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DomainChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SwitchedToPlayerApplication<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SwitchedToControl<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlayerDockedStateChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlayerReconnect<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Click<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoubleClick<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyDown<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyPress<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeyascii: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyUp<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MouseDown<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MouseMove<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MouseUp<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OpenStateChange: OpenStateChange::<Impl, IMPL_OFFSET>,
            PlayStateChange: PlayStateChange::<Impl, IMPL_OFFSET>,
            AudioLanguageChange: AudioLanguageChange::<Impl, IMPL_OFFSET>,
            StatusChange: StatusChange::<Impl, IMPL_OFFSET>,
            ScriptCommand: ScriptCommand::<Impl, IMPL_OFFSET>,
            NewStream: NewStream::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            Buffering: Buffering::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
            Warning: Warning::<Impl, IMPL_OFFSET>,
            EndOfStream: EndOfStream::<Impl, IMPL_OFFSET>,
            PositionChange: PositionChange::<Impl, IMPL_OFFSET>,
            MarkerHit: MarkerHit::<Impl, IMPL_OFFSET>,
            DurationUnitChange: DurationUnitChange::<Impl, IMPL_OFFSET>,
            CdromMediaChange: CdromMediaChange::<Impl, IMPL_OFFSET>,
            PlaylistChange: PlaylistChange::<Impl, IMPL_OFFSET>,
            CurrentPlaylistChange: CurrentPlaylistChange::<Impl, IMPL_OFFSET>,
            CurrentPlaylistItemAvailable: CurrentPlaylistItemAvailable::<Impl, IMPL_OFFSET>,
            MediaChange: MediaChange::<Impl, IMPL_OFFSET>,
            CurrentMediaItemAvailable: CurrentMediaItemAvailable::<Impl, IMPL_OFFSET>,
            CurrentItemChange: CurrentItemChange::<Impl, IMPL_OFFSET>,
            MediaCollectionChange: MediaCollectionChange::<Impl, IMPL_OFFSET>,
            MediaCollectionAttributeStringAdded: MediaCollectionAttributeStringAdded::<Impl, IMPL_OFFSET>,
            MediaCollectionAttributeStringRemoved: MediaCollectionAttributeStringRemoved::<Impl, IMPL_OFFSET>,
            MediaCollectionAttributeStringChanged: MediaCollectionAttributeStringChanged::<Impl, IMPL_OFFSET>,
            PlaylistCollectionChange: PlaylistCollectionChange::<Impl, IMPL_OFFSET>,
            PlaylistCollectionPlaylistAdded: PlaylistCollectionPlaylistAdded::<Impl, IMPL_OFFSET>,
            PlaylistCollectionPlaylistRemoved: PlaylistCollectionPlaylistRemoved::<Impl, IMPL_OFFSET>,
            PlaylistCollectionPlaylistSetAsDeleted: PlaylistCollectionPlaylistSetAsDeleted::<Impl, IMPL_OFFSET>,
            ModeChange: ModeChange::<Impl, IMPL_OFFSET>,
            MediaError: MediaError::<Impl, IMPL_OFFSET>,
            OpenPlaylistSwitch: OpenPlaylistSwitch::<Impl, IMPL_OFFSET>,
            DomainChange: DomainChange::<Impl, IMPL_OFFSET>,
            SwitchedToPlayerApplication: SwitchedToPlayerApplication::<Impl, IMPL_OFFSET>,
            SwitchedToControl: SwitchedToControl::<Impl, IMPL_OFFSET>,
            PlayerDockedStateChange: PlayerDockedStateChange::<Impl, IMPL_OFFSET>,
            PlayerReconnect: PlayerReconnect::<Impl, IMPL_OFFSET>,
            Click: Click::<Impl, IMPL_OFFSET>,
            DoubleClick: DoubleClick::<Impl, IMPL_OFFSET>,
            KeyDown: KeyDown::<Impl, IMPL_OFFSET>,
            KeyPress: KeyPress::<Impl, IMPL_OFFSET>,
            KeyUp: KeyUp::<Impl, IMPL_OFFSET>,
            MouseDown: MouseDown::<Impl, IMPL_OFFSET>,
            MouseMove: MouseMove::<Impl, IMPL_OFFSET>,
            MouseUp: MouseUp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents2Impl: Sized + IWMPEventsImpl {
    fn DeviceConnect();
    fn DeviceDisconnect();
    fn DeviceStatusChange();
    fn DeviceSyncStateChange();
    fn DeviceSyncError();
    fn CreatePartnershipComplete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEvents2Vtbl {
        unsafe extern "system" fn DeviceConnect<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceDisconnect<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceStatusChange<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, newstatus: WMPDeviceStatus) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceSyncStateChange<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, newstate: WMPSyncState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceSyncError<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePartnershipComplete<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, hrresult: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPEventsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DeviceConnect: DeviceConnect::<Impl, IMPL_OFFSET>,
            DeviceDisconnect: DeviceDisconnect::<Impl, IMPL_OFFSET>,
            DeviceStatusChange: DeviceStatusChange::<Impl, IMPL_OFFSET>,
            DeviceSyncStateChange: DeviceSyncStateChange::<Impl, IMPL_OFFSET>,
            DeviceSyncError: DeviceSyncError::<Impl, IMPL_OFFSET>,
            CreatePartnershipComplete: CreatePartnershipComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents3Impl: Sized + IWMPEventsImpl + IWMPEvents2Impl {
    fn CdromRipStateChange();
    fn CdromRipMediaError();
    fn CdromBurnStateChange();
    fn CdromBurnMediaError();
    fn CdromBurnError();
    fn LibraryConnect();
    fn LibraryDisconnect();
    fn FolderScanStateChange();
    fn StringCollectionChange();
    fn MediaCollectionMediaAdded();
    fn MediaCollectionMediaRemoved();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEvents3Vtbl {
        unsafe extern "system" fn CdromRipStateChange<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: ::windows::core::RawPtr, wmprs: WMPRipState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CdromRipMediaError<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CdromBurnStateChange<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, wmpbs: WMPBurnState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CdromBurnMediaError<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CdromBurnError<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, hrerror: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LibraryConnect<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LibraryDisconnect<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderScanStateChange<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpfss: WMPFolderScanState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StringCollectionChange<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispstringcollection: ::windows::core::RawPtr, change: WMPStringCollectionChangeEventType, lcollectionindex: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaCollectionMediaAdded<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaCollectionMediaRemoved<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPEvents2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CdromRipStateChange: CdromRipStateChange::<Impl, IMPL_OFFSET>,
            CdromRipMediaError: CdromRipMediaError::<Impl, IMPL_OFFSET>,
            CdromBurnStateChange: CdromBurnStateChange::<Impl, IMPL_OFFSET>,
            CdromBurnMediaError: CdromBurnMediaError::<Impl, IMPL_OFFSET>,
            CdromBurnError: CdromBurnError::<Impl, IMPL_OFFSET>,
            LibraryConnect: LibraryConnect::<Impl, IMPL_OFFSET>,
            LibraryDisconnect: LibraryDisconnect::<Impl, IMPL_OFFSET>,
            FolderScanStateChange: FolderScanStateChange::<Impl, IMPL_OFFSET>,
            StringCollectionChange: StringCollectionChange::<Impl, IMPL_OFFSET>,
            MediaCollectionMediaAdded: MediaCollectionMediaAdded::<Impl, IMPL_OFFSET>,
            MediaCollectionMediaRemoved: MediaCollectionMediaRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents4Impl: Sized + IWMPEventsImpl + IWMPEvents2Impl + IWMPEvents3Impl {
    fn DeviceEstimation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEvents4Vtbl {
        unsafe extern "system" fn DeviceEstimation<Impl: IWMPEvents4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMPEvents3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DeviceEstimation: DeviceEstimation::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPFolderMonitorServicesImpl: Sized {
    fn count();
    fn item();
    fn add();
    fn remove();
    fn scanState();
    fn currentFolder();
    fn scannedFilesCount();
    fn addedFilesCount();
    fn updateProgress();
    fn startScan();
    fn stopScan();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPFolderMonitorServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPFolderMonitorServicesVtbl {
        unsafe extern "system" fn count<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn item<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn add<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn remove<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn scanState<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn currentFolder<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn scannedFilesCount<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addedFilesCount<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn updateProgress<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startScan<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stopScan<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            count: count::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
            add: add::<Impl, IMPL_OFFSET>,
            remove: remove::<Impl, IMPL_OFFSET>,
            scanState: scanState::<Impl, IMPL_OFFSET>,
            currentFolder: currentFolder::<Impl, IMPL_OFFSET>,
            scannedFilesCount: scannedFilesCount::<Impl, IMPL_OFFSET>,
            addedFilesCount: addedFilesCount::<Impl, IMPL_OFFSET>,
            updateProgress: updateProgress::<Impl, IMPL_OFFSET>,
            startScan: startScan::<Impl, IMPL_OFFSET>,
            stopScan: stopScan::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPFolderMonitorServices as ::windows::core::Interface>::IID
    }
}
pub trait IWMPGraphCreationImpl: Sized {
    fn GraphCreationPreRender();
    fn GraphCreationPostRender();
    fn GetGraphCreationFlags();
}
impl IWMPGraphCreationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPGraphCreationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPGraphCreationVtbl {
        unsafe extern "system" fn GraphCreationPreRender<Impl: IWMPGraphCreationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GraphCreationPostRender<Impl: IWMPGraphCreationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGraphCreationFlags<Impl: IWMPGraphCreationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GraphCreationPreRender: GraphCreationPreRender::<Impl, IMPL_OFFSET>,
            GraphCreationPostRender: GraphCreationPostRender::<Impl, IMPL_OFFSET>,
            GetGraphCreationFlags: GetGraphCreationFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPGraphCreation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPLibraryImpl: Sized {
    fn name();
    fn r#type();
    fn mediaCollection();
    fn isIdentical();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibraryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPLibraryVtbl {
        unsafe extern "system" fn name<Impl: IWMPLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn r#type<Impl: IWMPLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn mediaCollection<Impl: IWMPLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmediacollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isIdentical<Impl: IWMPLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmplibrary: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            name: name::<Impl, IMPL_OFFSET>,
            r#type: r#type::<Impl, IMPL_OFFSET>,
            mediaCollection: mediaCollection::<Impl, IMPL_OFFSET>,
            isIdentical: isIdentical::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPLibrary2Impl: Sized + IWMPLibraryImpl {
    fn getItemInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPLibrary2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPLibrary2Vtbl {
        unsafe extern "system" fn getItemInfo<Impl: IWMPLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMPLibraryVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), getItemInfo: getItemInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrary2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMPLibraryServicesImpl: Sized {
    fn getCountByType();
    fn getLibraryByType();
}
impl IWMPLibraryServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibraryServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPLibraryServicesVtbl {
        unsafe extern "system" fn getCountByType<Impl: IWMPLibraryServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getLibraryByType<Impl: IWMPLibraryServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            getCountByType: getCountByType::<Impl, IMPL_OFFSET>,
            getLibraryByType: getLibraryByType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibraryServices as ::windows::core::Interface>::IID
    }
}
pub trait IWMPLibrarySharingServicesImpl: Sized {
    fn isLibraryShared();
    fn isLibrarySharingEnabled();
    fn showLibrarySharing();
}
impl IWMPLibrarySharingServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrarySharingServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPLibrarySharingServicesVtbl {
        unsafe extern "system" fn isLibraryShared<Impl: IWMPLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbshared: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isLibrarySharingEnabled<Impl: IWMPLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn showLibrarySharing<Impl: IWMPLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            isLibraryShared: isLibraryShared::<Impl, IMPL_OFFSET>,
            isLibrarySharingEnabled: isLibrarySharingEnabled::<Impl, IMPL_OFFSET>,
            showLibrarySharing: showLibrarySharing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrarySharingServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMediaImpl: Sized + IDispatchImpl {
    fn isIdentical();
    fn sourceURL();
    fn name();
    fn Setname();
    fn imageSourceWidth();
    fn imageSourceHeight();
    fn markerCount();
    fn getMarkerTime();
    fn getMarkerName();
    fn duration();
    fn durationString();
    fn attributeCount();
    fn getAttributeName();
    fn getItemInfo();
    fn setItemInfo();
    fn getItemInfoByAtom();
    fn isMemberOf();
    fn isReadOnlyItem();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMediaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMediaVtbl {
        unsafe extern "system" fn isIdentical<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn sourceURL<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn name<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setname<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn imageSourceWidth<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn imageSourceHeight<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn markerCount<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmarkercount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getMarkerTime<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getMarkerName<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pbstrmarkername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn duration<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn durationString<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrduration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributeCount<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAttributeName<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setItemInfo<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getItemInfoByAtom<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, latom: i32, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isMemberOf<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: ::windows::core::RawPtr, pvarfismemberof: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isReadOnlyItem<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarfisreadonly: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            isIdentical: isIdentical::<Impl, IMPL_OFFSET>,
            sourceURL: sourceURL::<Impl, IMPL_OFFSET>,
            name: name::<Impl, IMPL_OFFSET>,
            Setname: Setname::<Impl, IMPL_OFFSET>,
            imageSourceWidth: imageSourceWidth::<Impl, IMPL_OFFSET>,
            imageSourceHeight: imageSourceHeight::<Impl, IMPL_OFFSET>,
            markerCount: markerCount::<Impl, IMPL_OFFSET>,
            getMarkerTime: getMarkerTime::<Impl, IMPL_OFFSET>,
            getMarkerName: getMarkerName::<Impl, IMPL_OFFSET>,
            duration: duration::<Impl, IMPL_OFFSET>,
            durationString: durationString::<Impl, IMPL_OFFSET>,
            attributeCount: attributeCount::<Impl, IMPL_OFFSET>,
            getAttributeName: getAttributeName::<Impl, IMPL_OFFSET>,
            getItemInfo: getItemInfo::<Impl, IMPL_OFFSET>,
            setItemInfo: setItemInfo::<Impl, IMPL_OFFSET>,
            getItemInfoByAtom: getItemInfoByAtom::<Impl, IMPL_OFFSET>,
            isMemberOf: isMemberOf::<Impl, IMPL_OFFSET>,
            isReadOnlyItem: isReadOnlyItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMedia2Impl: Sized + IDispatchImpl + IWMPMediaImpl {
    fn error();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMedia2Vtbl {
        unsafe extern "system" fn error<Impl: IWMPMedia2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmperroritem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMPMediaVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), error: error::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMedia3Impl: Sized + IDispatchImpl + IWMPMediaImpl + IWMPMedia2Impl {
    fn getAttributeCountByType();
    fn getItemInfoByType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMedia3Vtbl {
        unsafe extern "system" fn getAttributeCountByType<Impl: IWMPMedia3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getItemInfoByType<Impl: IWMPMedia3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPMedia2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            getAttributeCountByType: getAttributeCountByType::<Impl, IMPL_OFFSET>,
            getItemInfoByType: getItemInfoByType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMediaCollectionImpl: Sized + IDispatchImpl {
    fn add();
    fn getAll();
    fn getByName();
    fn getByGenre();
    fn getByAuthor();
    fn getByAlbum();
    fn getByAttribute();
    fn remove();
    fn getAttributeStringCollection();
    fn getMediaAtom();
    fn setDeleted();
    fn isDeleted();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMediaCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMediaCollectionVtbl {
        unsafe extern "system" fn add<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAll<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getByName<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getByGenre<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgenre: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getByAuthor<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getByAlbum<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstralbum: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getByAttribute<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn remove<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfdeletefile: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAttributeStringCollection<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstringcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getMediaAtom<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, platom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setDeleted<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfisdeleted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isDeleted<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            add: add::<Impl, IMPL_OFFSET>,
            getAll: getAll::<Impl, IMPL_OFFSET>,
            getByName: getByName::<Impl, IMPL_OFFSET>,
            getByGenre: getByGenre::<Impl, IMPL_OFFSET>,
            getByAuthor: getByAuthor::<Impl, IMPL_OFFSET>,
            getByAlbum: getByAlbum::<Impl, IMPL_OFFSET>,
            getByAttribute: getByAttribute::<Impl, IMPL_OFFSET>,
            remove: remove::<Impl, IMPL_OFFSET>,
            getAttributeStringCollection: getAttributeStringCollection::<Impl, IMPL_OFFSET>,
            getMediaAtom: getMediaAtom::<Impl, IMPL_OFFSET>,
            setDeleted: setDeleted::<Impl, IMPL_OFFSET>,
            isDeleted: isDeleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMediaCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMediaCollection2Impl: Sized + IDispatchImpl + IWMPMediaCollectionImpl {
    fn createQuery();
    fn getPlaylistByQuery();
    fn getStringCollectionByQuery();
    fn getByAttributeAndMediaType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMediaCollection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMediaCollection2Vtbl {
        unsafe extern "system" fn createQuery<Impl: IWMPMediaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getPlaylistByQuery<Impl: IWMPMediaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: ::windows::core::RawPtr, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fsortascending: i16, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getStringCollectionByQuery<Impl: IWMPMediaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pquery: ::windows::core::RawPtr, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fsortascending: i16, ppstringcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getByAttributeAndMediaType<Impl: IWMPMediaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPMediaCollectionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            createQuery: createQuery::<Impl, IMPL_OFFSET>,
            getPlaylistByQuery: getPlaylistByQuery::<Impl, IMPL_OFFSET>,
            getStringCollectionByQuery: getStringCollectionByQuery::<Impl, IMPL_OFFSET>,
            getByAttributeAndMediaType: getByAttributeAndMediaType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMediaCollection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPMediaPluginRegistrarImpl: Sized {
    fn WMPRegisterPlayerPlugin();
    fn WMPUnRegisterPlayerPlugin();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPMediaPluginRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaPluginRegistrarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMediaPluginRegistrarVtbl {
        unsafe extern "system" fn WMPRegisterPlayerPlugin<Impl: IWMPMediaPluginRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, pwszuninstallstring: super::super::Foundation::PWSTR, dwpriority: u32, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMPUnRegisterPlayerPlugin<Impl: IWMPMediaPluginRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            WMPRegisterPlayerPlugin: WMPRegisterPlayerPlugin::<Impl, IMPL_OFFSET>,
            WMPUnRegisterPlayerPlugin: WMPUnRegisterPlayerPlugin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMediaPluginRegistrar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMetadataPictureImpl: Sized + IDispatchImpl {
    fn mimeType();
    fn pictureType();
    fn description();
    fn URL();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMetadataPictureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataPictureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMetadataPictureVtbl {
        unsafe extern "system" fn mimeType<Impl: IWMPMetadataPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn pictureType<Impl: IWMPMetadataPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpicturetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn description<Impl: IWMPMetadataPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn URL<Impl: IWMPMetadataPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            mimeType: mimeType::<Impl, IMPL_OFFSET>,
            pictureType: pictureType::<Impl, IMPL_OFFSET>,
            description: description::<Impl, IMPL_OFFSET>,
            URL: URL::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMetadataPicture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMetadataTextImpl: Sized + IDispatchImpl {
    fn description();
    fn text();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMetadataTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMetadataTextVtbl {
        unsafe extern "system" fn description<Impl: IWMPMetadataTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn text<Impl: IWMPMetadataTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            description: description::<Impl, IMPL_OFFSET>,
            text: text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMetadataText as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPNetworkImpl: Sized + IDispatchImpl {
    fn bandWidth();
    fn recoveredPackets();
    fn sourceProtocol();
    fn receivedPackets();
    fn lostPackets();
    fn receptionQuality();
    fn bufferingCount();
    fn bufferingProgress();
    fn bufferingTime();
    fn SetbufferingTime();
    fn frameRate();
    fn maxBitRate();
    fn bitRate();
    fn getProxySettings();
    fn setProxySettings();
    fn getProxyName();
    fn setProxyName();
    fn getProxyPort();
    fn setProxyPort();
    fn getProxyExceptionList();
    fn setProxyExceptionList();
    fn getProxyBypassForLocal();
    fn setProxyBypassForLocal();
    fn maxBandwidth();
    fn SetmaxBandwidth();
    fn downloadProgress();
    fn encodedFrameRate();
    fn framesSkipped();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPNetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetworkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNetworkVtbl {
        unsafe extern "system" fn bandWidth<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn recoveredPackets<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrecoveredpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn sourceProtocol<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn receivedPackets<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceivedpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn lostPackets<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllostpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn receptionQuality<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceptionquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn bufferingCount<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn bufferingProgress<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn bufferingTime<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingtime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetbufferingTime<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbufferingtime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn frameRate<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn maxBitRate<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn bitRate<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProxySettings<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plproxysetting: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setProxySettings<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxysetting: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProxyName<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproxyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setProxyName<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProxyPort<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxyport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setProxyPort<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxyport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProxyExceptionList<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrexceptionlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setProxyExceptionList<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrexceptionlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProxyBypassForLocal<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfbypassforlocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setProxyBypassForLocal<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fbypassforlocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn maxBandwidth<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetmaxBandwidth<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn downloadProgress<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldownloadprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn encodedFrameRate<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn framesSkipped<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            bandWidth: bandWidth::<Impl, IMPL_OFFSET>,
            recoveredPackets: recoveredPackets::<Impl, IMPL_OFFSET>,
            sourceProtocol: sourceProtocol::<Impl, IMPL_OFFSET>,
            receivedPackets: receivedPackets::<Impl, IMPL_OFFSET>,
            lostPackets: lostPackets::<Impl, IMPL_OFFSET>,
            receptionQuality: receptionQuality::<Impl, IMPL_OFFSET>,
            bufferingCount: bufferingCount::<Impl, IMPL_OFFSET>,
            bufferingProgress: bufferingProgress::<Impl, IMPL_OFFSET>,
            bufferingTime: bufferingTime::<Impl, IMPL_OFFSET>,
            SetbufferingTime: SetbufferingTime::<Impl, IMPL_OFFSET>,
            frameRate: frameRate::<Impl, IMPL_OFFSET>,
            maxBitRate: maxBitRate::<Impl, IMPL_OFFSET>,
            bitRate: bitRate::<Impl, IMPL_OFFSET>,
            getProxySettings: getProxySettings::<Impl, IMPL_OFFSET>,
            setProxySettings: setProxySettings::<Impl, IMPL_OFFSET>,
            getProxyName: getProxyName::<Impl, IMPL_OFFSET>,
            setProxyName: setProxyName::<Impl, IMPL_OFFSET>,
            getProxyPort: getProxyPort::<Impl, IMPL_OFFSET>,
            setProxyPort: setProxyPort::<Impl, IMPL_OFFSET>,
            getProxyExceptionList: getProxyExceptionList::<Impl, IMPL_OFFSET>,
            setProxyExceptionList: setProxyExceptionList::<Impl, IMPL_OFFSET>,
            getProxyBypassForLocal: getProxyBypassForLocal::<Impl, IMPL_OFFSET>,
            setProxyBypassForLocal: setProxyBypassForLocal::<Impl, IMPL_OFFSET>,
            maxBandwidth: maxBandwidth::<Impl, IMPL_OFFSET>,
            SetmaxBandwidth: SetmaxBandwidth::<Impl, IMPL_OFFSET>,
            downloadProgress: downloadProgress::<Impl, IMPL_OFFSET>,
            encodedFrameRate: encodedFrameRate::<Impl, IMPL_OFFSET>,
            framesSkipped: framesSkipped::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNetwork as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeRealEstateImpl: Sized {
    fn GetDesiredSize();
    fn SetRects();
    fn GetRects();
    fn SetWindowless();
    fn GetWindowless();
    fn SetFullScreen();
    fn GetFullScreen();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeRealEstateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeRealEstateVtbl {
        unsafe extern "system" fn GetDesiredSize<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRects<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRects<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWindowless<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwindowless: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWindowless<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFullScreen<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFullScreen<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDesiredSize: GetDesiredSize::<Impl, IMPL_OFFSET>,
            SetRects: SetRects::<Impl, IMPL_OFFSET>,
            GetRects: GetRects::<Impl, IMPL_OFFSET>,
            SetWindowless: SetWindowless::<Impl, IMPL_OFFSET>,
            GetWindowless: GetWindowless::<Impl, IMPL_OFFSET>,
            SetFullScreen: SetFullScreen::<Impl, IMPL_OFFSET>,
            GetFullScreen: GetFullScreen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeRealEstate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeRealEstateHostImpl: Sized {
    fn OnDesiredSizeChange();
    fn OnFullScreenTransition();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeRealEstateHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstateHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeRealEstateHostVtbl {
        unsafe extern "system" fn OnDesiredSizeChange<Impl: IWMPNodeRealEstateHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnFullScreenTransition<Impl: IWMPNodeRealEstateHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnDesiredSizeChange: OnDesiredSizeChange::<Impl, IMPL_OFFSET>,
            OnFullScreenTransition: OnFullScreenTransition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeRealEstateHost as ::windows::core::Interface>::IID
    }
}
pub trait IWMPNodeWindowedImpl: Sized {
    fn SetOwnerWindow();
    fn GetOwnerWindow();
}
impl IWMPNodeWindowedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeWindowedVtbl {
        unsafe extern "system" fn SetOwnerWindow<Impl: IWMPNodeWindowedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOwnerWindow<Impl: IWMPNodeWindowedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetOwnerWindow: SetOwnerWindow::<Impl, IMPL_OFFSET>,
            GetOwnerWindow: GetOwnerWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowed as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowedHostImpl: Sized {
    fn OnWindowMessageFromRenderer();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowedHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowedHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeWindowedHostVtbl {
        unsafe extern "system" fn OnWindowMessageFromRenderer<Impl: IWMPNodeWindowedHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnWindowMessageFromRenderer: OnWindowMessageFromRenderer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowedHost as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowlessImpl: Sized + IWMPWindowMessageSinkImpl {
    fn OnDraw();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowlessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowlessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeWindowlessVtbl {
        unsafe extern "system" fn OnDraw<Impl: IWMPNodeWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMPWindowMessageSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnDraw: OnDraw::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowless as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowlessHostImpl: Sized {
    fn InvalidateRect();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowlessHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowlessHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeWindowlessHostVtbl {
        unsafe extern "system" fn InvalidateRect<Impl: IWMPNodeWindowlessHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), InvalidateRect: InvalidateRect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowlessHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayerImpl: Sized + IDispatchImpl + IWMPCoreImpl {
    fn enabled();
    fn Setenabled();
    fn fullScreen();
    fn SetfullScreen();
    fn enableContextMenu();
    fn SetenableContextMenu();
    fn SetuiMode();
    fn uiMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayerVtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPCoreVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            enabled: enabled::<Impl, IMPL_OFFSET>,
            Setenabled: Setenabled::<Impl, IMPL_OFFSET>,
            fullScreen: fullScreen::<Impl, IMPL_OFFSET>,
            SetfullScreen: SetfullScreen::<Impl, IMPL_OFFSET>,
            enableContextMenu: enableContextMenu::<Impl, IMPL_OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Impl, IMPL_OFFSET>,
            SetuiMode: SetuiMode::<Impl, IMPL_OFFSET>,
            uiMode: uiMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayer2Impl: Sized + IDispatchImpl + IWMPCoreImpl {
    fn enabled();
    fn Setenabled();
    fn fullScreen();
    fn SetfullScreen();
    fn enableContextMenu();
    fn SetenableContextMenu();
    fn SetuiMode();
    fn uiMode();
    fn stretchToFit();
    fn SetstretchToFit();
    fn windowlessVideo();
    fn SetwindowlessVideo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayer2Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stretchToFit<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetstretchToFit<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn windowlessVideo<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetwindowlessVideo<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPCoreVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            enabled: enabled::<Impl, IMPL_OFFSET>,
            Setenabled: Setenabled::<Impl, IMPL_OFFSET>,
            fullScreen: fullScreen::<Impl, IMPL_OFFSET>,
            SetfullScreen: SetfullScreen::<Impl, IMPL_OFFSET>,
            enableContextMenu: enableContextMenu::<Impl, IMPL_OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Impl, IMPL_OFFSET>,
            SetuiMode: SetuiMode::<Impl, IMPL_OFFSET>,
            uiMode: uiMode::<Impl, IMPL_OFFSET>,
            stretchToFit: stretchToFit::<Impl, IMPL_OFFSET>,
            SetstretchToFit: SetstretchToFit::<Impl, IMPL_OFFSET>,
            windowlessVideo: windowlessVideo::<Impl, IMPL_OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayer3Impl: Sized + IDispatchImpl + IWMPCoreImpl + IWMPCore2Impl {
    fn enabled();
    fn Setenabled();
    fn fullScreen();
    fn SetfullScreen();
    fn enableContextMenu();
    fn SetenableContextMenu();
    fn SetuiMode();
    fn uiMode();
    fn stretchToFit();
    fn SetstretchToFit();
    fn windowlessVideo();
    fn SetwindowlessVideo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayer3Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stretchToFit<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetstretchToFit<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn windowlessVideo<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetwindowlessVideo<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPCore2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            enabled: enabled::<Impl, IMPL_OFFSET>,
            Setenabled: Setenabled::<Impl, IMPL_OFFSET>,
            fullScreen: fullScreen::<Impl, IMPL_OFFSET>,
            SetfullScreen: SetfullScreen::<Impl, IMPL_OFFSET>,
            enableContextMenu: enableContextMenu::<Impl, IMPL_OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Impl, IMPL_OFFSET>,
            SetuiMode: SetuiMode::<Impl, IMPL_OFFSET>,
            uiMode: uiMode::<Impl, IMPL_OFFSET>,
            stretchToFit: stretchToFit::<Impl, IMPL_OFFSET>,
            SetstretchToFit: SetstretchToFit::<Impl, IMPL_OFFSET>,
            windowlessVideo: windowlessVideo::<Impl, IMPL_OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayer4Impl: Sized + IDispatchImpl + IWMPCoreImpl + IWMPCore2Impl + IWMPCore3Impl {
    fn enabled();
    fn Setenabled();
    fn fullScreen();
    fn SetfullScreen();
    fn enableContextMenu();
    fn SetenableContextMenu();
    fn SetuiMode();
    fn uiMode();
    fn stretchToFit();
    fn SetstretchToFit();
    fn windowlessVideo();
    fn SetwindowlessVideo();
    fn isRemote();
    fn playerApplication();
    fn openPlayer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayer4Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stretchToFit<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetstretchToFit<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn windowlessVideo<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetwindowlessVideo<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isRemote<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarfisremote: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn playerApplication<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpplayerapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn openPlayer<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPCore3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            enabled: enabled::<Impl, IMPL_OFFSET>,
            Setenabled: Setenabled::<Impl, IMPL_OFFSET>,
            fullScreen: fullScreen::<Impl, IMPL_OFFSET>,
            SetfullScreen: SetfullScreen::<Impl, IMPL_OFFSET>,
            enableContextMenu: enableContextMenu::<Impl, IMPL_OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Impl, IMPL_OFFSET>,
            SetuiMode: SetuiMode::<Impl, IMPL_OFFSET>,
            uiMode: uiMode::<Impl, IMPL_OFFSET>,
            stretchToFit: stretchToFit::<Impl, IMPL_OFFSET>,
            SetstretchToFit: SetstretchToFit::<Impl, IMPL_OFFSET>,
            windowlessVideo: windowlessVideo::<Impl, IMPL_OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Impl, IMPL_OFFSET>,
            isRemote: isRemote::<Impl, IMPL_OFFSET>,
            playerApplication: playerApplication::<Impl, IMPL_OFFSET>,
            openPlayer: openPlayer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayerApplicationImpl: Sized + IDispatchImpl {
    fn switchToPlayerApplication();
    fn switchToControl();
    fn playerDocked();
    fn hasDisplay();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayerApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayerApplicationVtbl {
        unsafe extern "system" fn switchToPlayerApplication<Impl: IWMPPlayerApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn switchToControl<Impl: IWMPPlayerApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn playerDocked<Impl: IWMPPlayerApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbplayerdocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn hasDisplay<Impl: IWMPPlayerApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhasdisplay: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            switchToPlayerApplication: switchToPlayerApplication::<Impl, IMPL_OFFSET>,
            switchToControl: switchToControl::<Impl, IMPL_OFFSET>,
            playerDocked: playerDocked::<Impl, IMPL_OFFSET>,
            hasDisplay: hasDisplay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPPlayerServicesImpl: Sized {
    fn activateUIPlugin();
    fn setTaskPane();
    fn setTaskPaneURL();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPPlayerServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayerServicesVtbl {
        unsafe extern "system" fn activateUIPlugin<Impl: IWMPPlayerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setTaskPane<Impl: IWMPPlayerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setTaskPaneURL<Impl: IWMPPlayerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            activateUIPlugin: activateUIPlugin::<Impl, IMPL_OFFSET>,
            setTaskPane: setTaskPane::<Impl, IMPL_OFFSET>,
            setTaskPaneURL: setTaskPaneURL::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPPlayerServices2Impl: Sized + IWMPPlayerServicesImpl {
    fn setBackgroundProcessingPriority();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPPlayerServices2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayerServices2Vtbl {
        unsafe extern "system" fn setBackgroundProcessingPriority<Impl: IWMPPlayerServices2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpriority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPPlayerServicesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            setBackgroundProcessingPriority: setBackgroundProcessingPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerServices2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylistImpl: Sized + IDispatchImpl {
    fn count();
    fn name();
    fn Setname();
    fn attributeCount();
    fn attributeName();
    fn item();
    fn getItemInfo();
    fn setItemInfo();
    fn isIdentical();
    fn clear();
    fn insertItem();
    fn appendItem();
    fn removeItem();
    fn moveItem();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylistVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlaylistVtbl {
        unsafe extern "system" fn count<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn name<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setname<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributeCount<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributeName<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrattributename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn item<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setItemInfo<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isIdentical<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpplaylist: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn clear<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn insertItem<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn appendItem<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeItem<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn moveItem<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindexold: i32, lindexnew: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            count: count::<Impl, IMPL_OFFSET>,
            name: name::<Impl, IMPL_OFFSET>,
            Setname: Setname::<Impl, IMPL_OFFSET>,
            attributeCount: attributeCount::<Impl, IMPL_OFFSET>,
            attributeName: attributeName::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
            getItemInfo: getItemInfo::<Impl, IMPL_OFFSET>,
            setItemInfo: setItemInfo::<Impl, IMPL_OFFSET>,
            isIdentical: isIdentical::<Impl, IMPL_OFFSET>,
            clear: clear::<Impl, IMPL_OFFSET>,
            insertItem: insertItem::<Impl, IMPL_OFFSET>,
            appendItem: appendItem::<Impl, IMPL_OFFSET>,
            removeItem: removeItem::<Impl, IMPL_OFFSET>,
            moveItem: moveItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlaylist as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylistArrayImpl: Sized + IDispatchImpl {
    fn count();
    fn item();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylistArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlaylistArrayVtbl {
        unsafe extern "system" fn count<Impl: IWMPPlaylistArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn item<Impl: IWMPPlaylistArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), count: count::<Impl, IMPL_OFFSET>, item: item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlaylistArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylistCollectionImpl: Sized + IDispatchImpl {
    fn newPlaylist();
    fn getAll();
    fn getByName();
    fn remove();
    fn setDeleted();
    fn isDeleted();
    fn importPlaylist();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylistCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlaylistCollectionVtbl {
        unsafe extern "system" fn newPlaylist<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAll<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistarray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getByName<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppplaylistarray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn remove<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setDeleted<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfisdeleted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isDeleted<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn importPlaylist<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, ppimporteditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            newPlaylist: newPlaylist::<Impl, IMPL_OFFSET>,
            getAll: getAll::<Impl, IMPL_OFFSET>,
            getByName: getByName::<Impl, IMPL_OFFSET>,
            remove: remove::<Impl, IMPL_OFFSET>,
            setDeleted: setDeleted::<Impl, IMPL_OFFSET>,
            isDeleted: isDeleted::<Impl, IMPL_OFFSET>,
            importPlaylist: importPlaylist::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlaylistCollection as ::windows::core::Interface>::IID
    }
}
pub trait IWMPPluginImpl: Sized {
    fn Init();
    fn Shutdown();
    fn GetID();
    fn GetCaps();
    fn AdviseWMPServices();
    fn UnAdviseWMPServices();
}
impl IWMPPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPluginVtbl {
        unsafe extern "system" fn Init<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplaybackcontext: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetID<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdviseWMPServices<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpservices: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnAdviseWMPServices<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
            GetID: GetID::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            AdviseWMPServices: AdviseWMPServices::<Impl, IMPL_OFFSET>,
            UnAdviseWMPServices: UnAdviseWMPServices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPPluginEnableImpl: Sized {
    fn SetEnable();
    fn GetEnable();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPPluginEnableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginEnableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPluginEnableVtbl {
        unsafe extern "system" fn SetEnable<Impl: IWMPPluginEnableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnable<Impl: IWMPPluginEnableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetEnable: SetEnable::<Impl, IMPL_OFFSET>,
            GetEnable: GetEnable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPluginEnable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWMPPluginUIImpl: Sized {
    fn SetCore();
    fn Create();
    fn Destroy();
    fn DisplayPropertyPage();
    fn GetProperty();
    fn SetProperty();
    fn TranslateAccelerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWMPPluginUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPluginUIVtbl {
        unsafe extern "system" fn SetCore<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destroy<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayPropertyPage<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pvarproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pvarproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetCore: SetCore::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Destroy: Destroy::<Impl, IMPL_OFFSET>,
            DisplayPropertyPage: DisplayPropertyPage::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPluginUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPQueryImpl: Sized + IDispatchImpl {
    fn addCondition();
    fn beginNextGroup();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPQueryVtbl {
        unsafe extern "system" fn addCondition<Impl: IWMPQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstroperator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn beginNextGroup<Impl: IWMPQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            addCondition: addCondition::<Impl, IMPL_OFFSET>,
            beginNextGroup: beginNextGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPRemoteMediaServicesImpl: Sized {
    fn GetServiceType();
    fn GetApplicationName();
    fn GetScriptableObject();
    fn GetCustomUIMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPRemoteMediaServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRemoteMediaServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPRemoteMediaServicesVtbl {
        unsafe extern "system" fn GetServiceType<Impl: IWMPRemoteMediaServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplicationName<Impl: IWMPRemoteMediaServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScriptableObject<Impl: IWMPRemoteMediaServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomUIMode<Impl: IWMPRemoteMediaServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetServiceType: GetServiceType::<Impl, IMPL_OFFSET>,
            GetApplicationName: GetApplicationName::<Impl, IMPL_OFFSET>,
            GetScriptableObject: GetScriptableObject::<Impl, IMPL_OFFSET>,
            GetCustomUIMode: GetCustomUIMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPRemoteMediaServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPRenderConfigImpl: Sized {
    fn SetinProcOnly();
    fn inProcOnly();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPRenderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRenderConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPRenderConfigVtbl {
        unsafe extern "system" fn SetinProcOnly<Impl: IWMPRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn inProcOnly<Impl: IWMPRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetinProcOnly: SetinProcOnly::<Impl, IMPL_OFFSET>,
            inProcOnly: inProcOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPRenderConfig as ::windows::core::Interface>::IID
    }
}
pub trait IWMPServicesImpl: Sized {
    fn GetStreamTime();
    fn GetStreamState();
}
impl IWMPServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPServicesVtbl {
        unsafe extern "system" fn GetStreamTime<Impl: IWMPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamState<Impl: IWMPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut WMPServices_StreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamTime: GetStreamTime::<Impl, IMPL_OFFSET>,
            GetStreamState: GetStreamState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPSettingsImpl: Sized + IDispatchImpl {
    fn isAvailable();
    fn autoStart();
    fn SetautoStart();
    fn baseURL();
    fn SetbaseURL();
    fn defaultFrame();
    fn SetdefaultFrame();
    fn invokeURLs();
    fn SetinvokeURLs();
    fn mute();
    fn Setmute();
    fn playCount();
    fn SetplayCount();
    fn rate();
    fn Setrate();
    fn balance();
    fn Setbalance();
    fn volume();
    fn Setvolume();
    fn getMode();
    fn setMode();
    fn enableErrorDialogs();
    fn SetenableErrorDialogs();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSettingsVtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn autoStart<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautostart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetautoStart<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fautostart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn baseURL<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbaseurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetbaseURL<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbaseurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn defaultFrame<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdefaultframe: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetdefaultFrame<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdefaultframe: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn invokeURLs<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinvokeurls: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetinvokeURLs<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finvokeurls: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn mute<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmute: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setmute<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmute: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn playCount<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetplayCount<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn rate<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdrate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setrate<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn balance<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setbalance<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn volume<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setvolume<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getMode<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarfmode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setMode<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varfmode: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enableErrorDialogs<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableerrordialogs: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetenableErrorDialogs<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableerrordialogs: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            isAvailable: isAvailable::<Impl, IMPL_OFFSET>,
            autoStart: autoStart::<Impl, IMPL_OFFSET>,
            SetautoStart: SetautoStart::<Impl, IMPL_OFFSET>,
            baseURL: baseURL::<Impl, IMPL_OFFSET>,
            SetbaseURL: SetbaseURL::<Impl, IMPL_OFFSET>,
            defaultFrame: defaultFrame::<Impl, IMPL_OFFSET>,
            SetdefaultFrame: SetdefaultFrame::<Impl, IMPL_OFFSET>,
            invokeURLs: invokeURLs::<Impl, IMPL_OFFSET>,
            SetinvokeURLs: SetinvokeURLs::<Impl, IMPL_OFFSET>,
            mute: mute::<Impl, IMPL_OFFSET>,
            Setmute: Setmute::<Impl, IMPL_OFFSET>,
            playCount: playCount::<Impl, IMPL_OFFSET>,
            SetplayCount: SetplayCount::<Impl, IMPL_OFFSET>,
            rate: rate::<Impl, IMPL_OFFSET>,
            Setrate: Setrate::<Impl, IMPL_OFFSET>,
            balance: balance::<Impl, IMPL_OFFSET>,
            Setbalance: Setbalance::<Impl, IMPL_OFFSET>,
            volume: volume::<Impl, IMPL_OFFSET>,
            Setvolume: Setvolume::<Impl, IMPL_OFFSET>,
            getMode: getMode::<Impl, IMPL_OFFSET>,
            setMode: setMode::<Impl, IMPL_OFFSET>,
            enableErrorDialogs: enableErrorDialogs::<Impl, IMPL_OFFSET>,
            SetenableErrorDialogs: SetenableErrorDialogs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPSettings2Impl: Sized + IDispatchImpl + IWMPSettingsImpl {
    fn defaultAudioLanguage();
    fn mediaAccessRights();
    fn requestMediaAccessRights();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPSettings2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSettings2Vtbl {
        unsafe extern "system" fn defaultAudioLanguage<Impl: IWMPSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn mediaAccessRights<Impl: IWMPSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrights: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn requestMediaAccessRights<Impl: IWMPSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdesiredaccess: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvbaccepted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPSettingsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            defaultAudioLanguage: defaultAudioLanguage::<Impl, IMPL_OFFSET>,
            mediaAccessRights: mediaAccessRights::<Impl, IMPL_OFFSET>,
            requestMediaAccessRights: requestMediaAccessRights::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSkinManagerImpl: Sized {
    fn SetVisualStyle();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSkinManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSkinManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSkinManagerVtbl {
        unsafe extern "system" fn SetVisualStyle<Impl: IWMPSkinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetVisualStyle: SetVisualStyle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSkinManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPStringCollectionImpl: Sized + IDispatchImpl {
    fn count();
    fn item();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPStringCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPStringCollectionVtbl {
        unsafe extern "system" fn count<Impl: IWMPStringCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn item<Impl: IWMPStringCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), count: count::<Impl, IMPL_OFFSET>, item: item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPStringCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPStringCollection2Impl: Sized + IDispatchImpl + IWMPStringCollectionImpl {
    fn isIdentical();
    fn getItemInfo();
    fn getAttributeCountByType();
    fn getItemInfoByType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPStringCollection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPStringCollection2Vtbl {
        unsafe extern "system" fn isIdentical<Impl: IWMPStringCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpstringcollection2: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPStringCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAttributeCountByType<Impl: IWMPStringCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getItemInfoByType<Impl: IWMPStringCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattributeindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPStringCollectionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            isIdentical: isIdentical::<Impl, IMPL_OFFSET>,
            getItemInfo: getItemInfo::<Impl, IMPL_OFFSET>,
            getAttributeCountByType: getAttributeCountByType::<Impl, IMPL_OFFSET>,
            getItemInfoByType: getItemInfoByType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPStringCollection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSubscriptionServiceImpl: Sized {
    fn allowPlay();
    fn allowCDBurn();
    fn allowPDATransfer();
    fn startBackgroundProcessing();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSubscriptionServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSubscriptionServiceVtbl {
        unsafe extern "system" fn allowPlay<Impl: IWMPSubscriptionServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pmedia: ::windows::core::RawPtr, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn allowCDBurn<Impl: IWMPSubscriptionServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: ::windows::core::RawPtr, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn allowPDATransfer<Impl: IWMPSubscriptionServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: ::windows::core::RawPtr, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startBackgroundProcessing<Impl: IWMPSubscriptionServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            allowPlay: allowPlay::<Impl, IMPL_OFFSET>,
            allowCDBurn: allowCDBurn::<Impl, IMPL_OFFSET>,
            allowPDATransfer: allowPDATransfer::<Impl, IMPL_OFFSET>,
            startBackgroundProcessing: startBackgroundProcessing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSubscriptionService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSubscriptionService2Impl: Sized + IWMPSubscriptionServiceImpl {
    fn stopBackgroundProcessing();
    fn serviceEvent();
    fn deviceAvailable();
    fn prepareForSync();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSubscriptionService2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSubscriptionService2Vtbl {
        unsafe extern "system" fn stopBackgroundProcessing<Impl: IWMPSubscriptionService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn serviceEvent<Impl: IWMPSubscriptionService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: WMPSubscriptionServiceEvent) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn deviceAvailable<Impl: IWMPSubscriptionService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn prepareForSync<Impl: IWMPSubscriptionService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPSubscriptionServiceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            stopBackgroundProcessing: stopBackgroundProcessing::<Impl, IMPL_OFFSET>,
            serviceEvent: serviceEvent::<Impl, IMPL_OFFSET>,
            deviceAvailable: deviceAvailable::<Impl, IMPL_OFFSET>,
            prepareForSync: prepareForSync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSubscriptionService2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMPSubscriptionServiceCallbackImpl: Sized {
    fn onComplete();
}
impl IWMPSubscriptionServiceCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionServiceCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSubscriptionServiceCallbackVtbl {
        unsafe extern "system" fn onComplete<Impl: IWMPSubscriptionServiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), onComplete: onComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSubscriptionServiceCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSyncDeviceImpl: Sized {
    fn friendlyName();
    fn SetfriendlyName();
    fn deviceName();
    fn deviceId();
    fn partnershipIndex();
    fn connected();
    fn status();
    fn syncState();
    fn progress();
    fn getItemInfo();
    fn createPartnership();
    fn deletePartnership();
    fn start();
    fn stop();
    fn showSettings();
    fn isIdentical();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSyncDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSyncDeviceVtbl {
        unsafe extern "system" fn friendlyName<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetfriendlyName<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn deviceName<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn deviceId<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn partnershipIndex<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn connected<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn status<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn syncState<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpss: *mut WMPSyncState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn progress<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createPartnership<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbshowui: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn deletePartnership<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn start<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stop<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn showSettings<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isIdentical<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            friendlyName: friendlyName::<Impl, IMPL_OFFSET>,
            SetfriendlyName: SetfriendlyName::<Impl, IMPL_OFFSET>,
            deviceName: deviceName::<Impl, IMPL_OFFSET>,
            deviceId: deviceId::<Impl, IMPL_OFFSET>,
            partnershipIndex: partnershipIndex::<Impl, IMPL_OFFSET>,
            connected: connected::<Impl, IMPL_OFFSET>,
            status: status::<Impl, IMPL_OFFSET>,
            syncState: syncState::<Impl, IMPL_OFFSET>,
            progress: progress::<Impl, IMPL_OFFSET>,
            getItemInfo: getItemInfo::<Impl, IMPL_OFFSET>,
            createPartnership: createPartnership::<Impl, IMPL_OFFSET>,
            deletePartnership: deletePartnership::<Impl, IMPL_OFFSET>,
            start: start::<Impl, IMPL_OFFSET>,
            stop: stop::<Impl, IMPL_OFFSET>,
            showSettings: showSettings::<Impl, IMPL_OFFSET>,
            isIdentical: isIdentical::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSyncDevice2Impl: Sized + IWMPSyncDeviceImpl {
    fn setItemInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSyncDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSyncDevice2Vtbl {
        unsafe extern "system" fn setItemInfo<Impl: IWMPSyncDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMPSyncDeviceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), setItemInfo: setItemInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSyncDevice3Impl: Sized + IWMPSyncDeviceImpl + IWMPSyncDevice2Impl {
    fn estimateSyncSize();
    fn cancelEstimation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSyncDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSyncDevice3Vtbl {
        unsafe extern "system" fn estimateSyncSize<Impl: IWMPSyncDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnonruleplaylist: ::windows::core::RawPtr, prulesplaylist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn cancelEstimation<Impl: IWMPSyncDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWMPSyncDevice2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            estimateSyncSize: estimateSyncSize::<Impl, IMPL_OFFSET>,
            cancelEstimation: cancelEstimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMPSyncServicesImpl: Sized {
    fn deviceCount();
    fn getDevice();
}
impl IWMPSyncServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSyncServicesVtbl {
        unsafe extern "system" fn deviceCount<Impl: IWMPSyncServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getDevice<Impl: IWMPSyncServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            deviceCount: deviceCount::<Impl, IMPL_OFFSET>,
            getDevice: getDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncServices as ::windows::core::Interface>::IID
    }
}
pub trait IWMPTranscodePolicyImpl: Sized {
    fn allowTranscode();
}
impl IWMPTranscodePolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPTranscodePolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPTranscodePolicyVtbl {
        unsafe extern "system" fn allowTranscode<Impl: IWMPTranscodePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvballow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), allowTranscode: allowTranscode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPTranscodePolicy as ::windows::core::Interface>::IID
    }
}
pub trait IWMPUserEventSinkImpl: Sized {
    fn NotifyUserEvent();
}
impl IWMPUserEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPUserEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPUserEventSinkVtbl {
        unsafe extern "system" fn NotifyUserEvent<Impl: IWMPUserEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), NotifyUserEvent: NotifyUserEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPUserEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IWMPVideoRenderConfigImpl: Sized {
    fn SetpresenterActivate();
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IWMPVideoRenderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPVideoRenderConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPVideoRenderConfigVtbl {
        unsafe extern "system" fn SetpresenterActivate<Impl: IWMPVideoRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetpresenterActivate: SetpresenterActivate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPVideoRenderConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPWindowMessageSinkImpl: Sized {
    fn OnWindowMessage();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPWindowMessageSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPWindowMessageSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPWindowMessageSinkVtbl {
        unsafe extern "system" fn OnWindowMessage<Impl: IWMPWindowMessageSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnWindowMessage: OnWindowMessage::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPWindowMessageSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedImpl: Sized {
    fn Xml();
    fn Name();
    fn Rename();
    fn Url();
    fn SetUrl();
    fn LocalId();
    fn Path();
    fn Move();
    fn Parent();
    fn LastWriteTime();
    fn Delete();
    fn Download();
    fn AsyncDownload();
    fn CancelAsyncDownload();
    fn SyncSetting();
    fn SetSyncSetting();
    fn Interval();
    fn SetInterval();
    fn LastDownloadTime();
    fn LocalEnclosurePath();
    fn Items();
    fn GetItem();
    fn MarkAllItemsRead();
    fn MaxItemCount();
    fn SetMaxItemCount();
    fn DownloadEnclosuresAutomatically();
    fn SetDownloadEnclosuresAutomatically();
    fn DownloadStatus();
    fn LastDownloadError();
    fn Merge();
    fn DownloadUrl();
    fn Title();
    fn Description();
    fn Link();
    fn Image();
    fn LastBuildDate();
    fn PubDate();
    fn Ttl();
    fn Language();
    fn Copyright();
    fn IsList();
    fn GetWatcher();
    fn UnreadItemCount();
    fn ItemCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedVtbl {
        unsafe extern "system" fn Xml<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, pps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rename<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Url<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUrl<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalId<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastWriteTime<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastwritetime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Download<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncDownload<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncSetting<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfss: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSyncSetting<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fss: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Interval<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterval<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalEnclosurePath<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Items<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItem<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarkAllItemsRead<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxItemCount<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puimaxitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxItemCount<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uimaxitemcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdownloadenclosuresautomatically: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadStatus<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastDownloadError<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Merge<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Title<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Link<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszhomepage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Image<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszimageurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastBuildDate<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastbuilddate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PubDate<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ttl<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puittl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Language<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlanguage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Copyright<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcopyright: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsList<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbislist: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWatcher<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnreadItemCount<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemCount<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Xml: Xml::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Rename: Rename::<Impl, IMPL_OFFSET>,
            Url: Url::<Impl, IMPL_OFFSET>,
            SetUrl: SetUrl::<Impl, IMPL_OFFSET>,
            LocalId: LocalId::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            LastWriteTime: LastWriteTime::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Download: Download::<Impl, IMPL_OFFSET>,
            AsyncDownload: AsyncDownload::<Impl, IMPL_OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Impl, IMPL_OFFSET>,
            SyncSetting: SyncSetting::<Impl, IMPL_OFFSET>,
            SetSyncSetting: SetSyncSetting::<Impl, IMPL_OFFSET>,
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
            LastDownloadTime: LastDownloadTime::<Impl, IMPL_OFFSET>,
            LocalEnclosurePath: LocalEnclosurePath::<Impl, IMPL_OFFSET>,
            Items: Items::<Impl, IMPL_OFFSET>,
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            MarkAllItemsRead: MarkAllItemsRead::<Impl, IMPL_OFFSET>,
            MaxItemCount: MaxItemCount::<Impl, IMPL_OFFSET>,
            SetMaxItemCount: SetMaxItemCount::<Impl, IMPL_OFFSET>,
            DownloadEnclosuresAutomatically: DownloadEnclosuresAutomatically::<Impl, IMPL_OFFSET>,
            SetDownloadEnclosuresAutomatically: SetDownloadEnclosuresAutomatically::<Impl, IMPL_OFFSET>,
            DownloadStatus: DownloadStatus::<Impl, IMPL_OFFSET>,
            LastDownloadError: LastDownloadError::<Impl, IMPL_OFFSET>,
            Merge: Merge::<Impl, IMPL_OFFSET>,
            DownloadUrl: DownloadUrl::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Link: Link::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            LastBuildDate: LastBuildDate::<Impl, IMPL_OFFSET>,
            PubDate: PubDate::<Impl, IMPL_OFFSET>,
            Ttl: Ttl::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            Copyright: Copyright::<Impl, IMPL_OFFSET>,
            IsList: IsList::<Impl, IMPL_OFFSET>,
            GetWatcher: GetWatcher::<Impl, IMPL_OFFSET>,
            UnreadItemCount: UnreadItemCount::<Impl, IMPL_OFFSET>,
            ItemCount: ItemCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeed as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeed2Impl: Sized + IXFeedImpl {
    fn GetItemByEffectiveId();
    fn LastItemDownloadTime();
    fn Username();
    fn Password();
    fn SetCredentials();
    fn ClearCredentials();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeed2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeed2Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uieffectiveid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastItemDownloadTime<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastitemdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Username<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Password<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCredentials<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearCredentials<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IXFeedVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetItemByEffectiveId: GetItemByEffectiveId::<Impl, IMPL_OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Impl, IMPL_OFFSET>,
            Username: Username::<Impl, IMPL_OFFSET>,
            Password: Password::<Impl, IMPL_OFFSET>,
            SetCredentials: SetCredentials::<Impl, IMPL_OFFSET>,
            ClearCredentials: ClearCredentials::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeed2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXFeedEnclosureImpl: Sized {
    fn Url();
    fn Type();
    fn Length();
    fn AsyncDownload();
    fn CancelAsyncDownload();
    fn DownloadStatus();
    fn LastDownloadError();
    fn LocalPath();
    fn Parent();
    fn DownloadUrl();
    fn DownloadMimeType();
    fn RemoveFile();
    fn SetFile();
}
#[cfg(feature = "Win32_Foundation")]
impl IXFeedEnclosureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedEnclosureVtbl {
        unsafe extern "system" fn Url<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Length<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puilength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncDownload<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadStatus<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastDownloadError<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalPath<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadMimeType<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFile<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFile<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdownloadurl: super::super::Foundation::PWSTR, pszdownloadfilepath: super::super::Foundation::PWSTR, pszdownloadmimetype: super::super::Foundation::PWSTR, pszenclosurefilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Url: Url::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            AsyncDownload: AsyncDownload::<Impl, IMPL_OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Impl, IMPL_OFFSET>,
            DownloadStatus: DownloadStatus::<Impl, IMPL_OFFSET>,
            LastDownloadError: LastDownloadError::<Impl, IMPL_OFFSET>,
            LocalPath: LocalPath::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            DownloadUrl: DownloadUrl::<Impl, IMPL_OFFSET>,
            DownloadMimeType: DownloadMimeType::<Impl, IMPL_OFFSET>,
            RemoveFile: RemoveFile::<Impl, IMPL_OFFSET>,
            SetFile: SetFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedEnclosure as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXFeedEventsImpl: Sized {
    fn Error();
    fn FeedDeleted();
    fn FeedRenamed();
    fn FeedUrlChanged();
    fn FeedMoved();
    fn FeedDownloading();
    fn FeedDownloadCompleted();
    fn FeedItemCountChanged();
}
#[cfg(feature = "Win32_Foundation")]
impl IXFeedEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedEventsVtbl {
        unsafe extern "system" fn Error<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDeleted<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedRenamed<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedMoved<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDownloading<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            FeedDeleted: FeedDeleted::<Impl, IMPL_OFFSET>,
            FeedRenamed: FeedRenamed::<Impl, IMPL_OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Impl, IMPL_OFFSET>,
            FeedMoved: FeedMoved::<Impl, IMPL_OFFSET>,
            FeedDownloading: FeedDownloading::<Impl, IMPL_OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Impl, IMPL_OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXFeedFolderImpl: Sized {
    fn Feeds();
    fn Subfolders();
    fn CreateFeed();
    fn CreateSubfolder();
    fn ExistsFeed();
    fn ExistsSubfolder();
    fn GetFeed();
    fn GetSubfolder();
    fn Delete();
    fn Name();
    fn Rename();
    fn Path();
    fn Move();
    fn Parent();
    fn IsRoot();
    fn GetWatcher();
    fn TotalUnreadItemCount();
    fn TotalItemCount();
}
#[cfg(feature = "Win32_Foundation")]
impl IXFeedFolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedFolderVtbl {
        unsafe extern "system" fn Feeds<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Subfolders<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFeed<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSubfolder<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistsFeed<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistsSubfolder<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeed<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubfolder<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rename<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRoot<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrootfeedfolder: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWatcher<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalUnreadItemCount<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalItemCount<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Feeds: Feeds::<Impl, IMPL_OFFSET>,
            Subfolders: Subfolders::<Impl, IMPL_OFFSET>,
            CreateFeed: CreateFeed::<Impl, IMPL_OFFSET>,
            CreateSubfolder: CreateSubfolder::<Impl, IMPL_OFFSET>,
            ExistsFeed: ExistsFeed::<Impl, IMPL_OFFSET>,
            ExistsSubfolder: ExistsSubfolder::<Impl, IMPL_OFFSET>,
            GetFeed: GetFeed::<Impl, IMPL_OFFSET>,
            GetSubfolder: GetSubfolder::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Rename: Rename::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            IsRoot: IsRoot::<Impl, IMPL_OFFSET>,
            GetWatcher: GetWatcher::<Impl, IMPL_OFFSET>,
            TotalUnreadItemCount: TotalUnreadItemCount::<Impl, IMPL_OFFSET>,
            TotalItemCount: TotalItemCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedFolder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXFeedFolderEventsImpl: Sized {
    fn Error();
    fn FolderAdded();
    fn FolderDeleted();
    fn FolderRenamed();
    fn FolderMovedFrom();
    fn FolderMovedTo();
    fn FolderItemCountChanged();
    fn FeedAdded();
    fn FeedDeleted();
    fn FeedRenamed();
    fn FeedUrlChanged();
    fn FeedMovedFrom();
    fn FeedMovedTo();
    fn FeedDownloading();
    fn FeedDownloadCompleted();
    fn FeedItemCountChanged();
}
#[cfg(feature = "Win32_Foundation")]
impl IXFeedFolderEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedFolderEventsVtbl {
        unsafe extern "system" fn Error<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderAdded<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderDeleted<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderRenamed<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderMovedFrom<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderMovedTo<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FolderItemCountChanged<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedAdded<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDeleted<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedRenamed<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedMovedFrom<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedMovedTo<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDownloading<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            FolderAdded: FolderAdded::<Impl, IMPL_OFFSET>,
            FolderDeleted: FolderDeleted::<Impl, IMPL_OFFSET>,
            FolderRenamed: FolderRenamed::<Impl, IMPL_OFFSET>,
            FolderMovedFrom: FolderMovedFrom::<Impl, IMPL_OFFSET>,
            FolderMovedTo: FolderMovedTo::<Impl, IMPL_OFFSET>,
            FolderItemCountChanged: FolderItemCountChanged::<Impl, IMPL_OFFSET>,
            FeedAdded: FeedAdded::<Impl, IMPL_OFFSET>,
            FeedDeleted: FeedDeleted::<Impl, IMPL_OFFSET>,
            FeedRenamed: FeedRenamed::<Impl, IMPL_OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Impl, IMPL_OFFSET>,
            FeedMovedFrom: FeedMovedFrom::<Impl, IMPL_OFFSET>,
            FeedMovedTo: FeedMovedTo::<Impl, IMPL_OFFSET>,
            FeedDownloading: FeedDownloading::<Impl, IMPL_OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Impl, IMPL_OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedFolderEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedItemImpl: Sized {
    fn Xml();
    fn Title();
    fn Link();
    fn Guid();
    fn Description();
    fn PubDate();
    fn Comments();
    fn Author();
    fn Enclosure();
    fn IsRead();
    fn SetIsRead();
    fn LocalId();
    fn Parent();
    fn Delete();
    fn DownloadUrl();
    fn LastDownloadTime();
    fn Modified();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedItemVtbl {
        unsafe extern "system" fn Xml<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fxif: FEEDS_XML_INCLUDE_FLAGS, pps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Title<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Link<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Guid<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszguid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PubDate<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Comments<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Author<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszauthor: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enclosure<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRead<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisread: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsRead<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisread: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalId<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Modified<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstmodifiedtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Xml: Xml::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Link: Link::<Impl, IMPL_OFFSET>,
            Guid: Guid::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            PubDate: PubDate::<Impl, IMPL_OFFSET>,
            Comments: Comments::<Impl, IMPL_OFFSET>,
            Author: Author::<Impl, IMPL_OFFSET>,
            Enclosure: Enclosure::<Impl, IMPL_OFFSET>,
            IsRead: IsRead::<Impl, IMPL_OFFSET>,
            SetIsRead: SetIsRead::<Impl, IMPL_OFFSET>,
            LocalId: LocalId::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            DownloadUrl: DownloadUrl::<Impl, IMPL_OFFSET>,
            LastDownloadTime: LastDownloadTime::<Impl, IMPL_OFFSET>,
            Modified: Modified::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedItem2Impl: Sized + IXFeedItemImpl {
    fn EffectiveId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedItem2Vtbl {
        unsafe extern "system" fn EffectiveId<Impl: IXFeedItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puieffectiveid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IXFeedItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EffectiveId: EffectiveId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedItem2 as ::windows::core::Interface>::IID
    }
}
pub trait IXFeedsEnumImpl: Sized {
    fn Count();
    fn Item();
}
impl IXFeedsEnumVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsEnumImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedsEnumVtbl {
        unsafe extern "system" fn Count<Impl: IXFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IXFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Count: Count::<Impl, IMPL_OFFSET>, Item: Item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedsEnum as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedsManagerImpl: Sized {
    fn RootFolder();
    fn IsSubscribed();
    fn ExistsFeed();
    fn GetFeed();
    fn GetFeedByUrl();
    fn ExistsFolder();
    fn GetFolder();
    fn DeleteFeed();
    fn DeleteFolder();
    fn BackgroundSync();
    fn BackgroundSyncStatus();
    fn DefaultInterval();
    fn SetDefaultInterval();
    fn AsyncSyncAll();
    fn Normalize();
    fn ItemCountLimit();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedsManagerVtbl {
        unsafe extern "system" fn RootFolder<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSubscribed<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pbsubscribed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistsFeed<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbfeedexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeed<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeedByUrl<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistsFolder<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbfolderexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFolder<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteFeed<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteFolder<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackgroundSync<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbss: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultInterval<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultInterval<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncSyncAll<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Normalize<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstreamin: ::windows::core::RawPtr, ppstreamout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemCountLimit<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcountlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RootFolder: RootFolder::<Impl, IMPL_OFFSET>,
            IsSubscribed: IsSubscribed::<Impl, IMPL_OFFSET>,
            ExistsFeed: ExistsFeed::<Impl, IMPL_OFFSET>,
            GetFeed: GetFeed::<Impl, IMPL_OFFSET>,
            GetFeedByUrl: GetFeedByUrl::<Impl, IMPL_OFFSET>,
            ExistsFolder: ExistsFolder::<Impl, IMPL_OFFSET>,
            GetFolder: GetFolder::<Impl, IMPL_OFFSET>,
            DeleteFeed: DeleteFeed::<Impl, IMPL_OFFSET>,
            DeleteFolder: DeleteFolder::<Impl, IMPL_OFFSET>,
            BackgroundSync: BackgroundSync::<Impl, IMPL_OFFSET>,
            BackgroundSyncStatus: BackgroundSyncStatus::<Impl, IMPL_OFFSET>,
            DefaultInterval: DefaultInterval::<Impl, IMPL_OFFSET>,
            SetDefaultInterval: SetDefaultInterval::<Impl, IMPL_OFFSET>,
            AsyncSyncAll: AsyncSyncAll::<Impl, IMPL_OFFSET>,
            Normalize: Normalize::<Impl, IMPL_OFFSET>,
            ItemCountLimit: ItemCountLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedsManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _WMPOCXEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _WMPOCXEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _WMPOCXEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _WMPOCXEventsVtbl {
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_WMPOCXEvents as ::windows::core::Interface>::IID
    }
}
